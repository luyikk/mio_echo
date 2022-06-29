use std::io::{Read,  Write};
use std::net::SocketAddr;
use log::LevelFilter;
use mio::{Events, Interest, Poll, Token};
use mio::net::{TcpListener, TcpStream};
use slab::Slab;


const SERVER: Token = Token(0);

fn main()->anyhow::Result<()> {
    struct Client {
        socket: TcpStream,
        peer_addr:SocketAddr,
        buff: [u8;1024],
        len: usize,
    }

    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    let mut poll = Poll::new()?;

    let mut listener = TcpListener::bind("0.0.0.0:55555".parse()?)?;

    poll.registry()
        .register(&mut listener, SERVER, Interest::READABLE)?;

    let mut clients = Slab::with_capacity(1024);
    clients.insert(None);
    let mut events = Events::with_capacity(1024);

    loop {

        poll.poll(&mut events, None)?;

        for event in events.iter() {
            let token=event.token();
            if token==SERVER{
                //表示可accept
                let (mut socket, peer_addr) = listener.accept()?;
                log::info!("addr:{} connect", peer_addr);
                poll.registry()
                    .reregister(&mut listener, token, Interest::READABLE)?;

                poll.registry().register(
                    &mut socket,
                    Token(clients.vacant_key()),
                    Interest::READABLE.add(Interest::WRITABLE),
                )?;

                clients.insert(
                    Some(Client {
                        socket,
                        peer_addr,
                        buff: [0;1024],
                        len: 0,
                    })
                );

            }else if let Some(Some(client)) = clients.get_mut(token.0){
                let mut disconnect = false;
                if event.is_readable(){
                    let size = match client.socket.read(&mut client.buff[..]) {
                        Ok(n) => n,
                        Err(err) if err.kind() == std::io::ErrorKind::ConnectionReset => 0,
                        Err(err) => {
                            log::error!("addr:{} error:{}", client.peer_addr, err);
                            0
                        }
                    };
                    client.len = size;
                    disconnect = size == 0;

                }

                if event.is_writable() && client.len>0 {
                    if let Err(err) = client.socket.write(&client.buff[..client.len]) {
                        log::error!("addr:{} error:{}", client.peer_addr, err);
                        disconnect = true;
                    }
                    client.len = 0;
                }

                if disconnect {
                    let mut client = clients.remove(token.0);
                    poll.registry().deregister(&mut client.as_mut().unwrap().socket)?;
                    log::info!("addr:{} disconnect", client.unwrap().peer_addr);
                }else {
                    if !cfg!(unix) {
                        poll.registry().reregister(
                            &mut client.socket,
                            token,
                            Interest::READABLE.add(Interest::WRITABLE),
                        )?;
                    }
                }
            }
        }


    }
}
