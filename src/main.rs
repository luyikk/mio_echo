use std::collections::HashMap;
use std::io::{Read, Write};
use log::LevelFilter;
use mio::{Events, Interest, Poll, Token};
use mio::net::{TcpListener, TcpStream};

const SERVER: Token = Token(0);

fn main()->anyhow::Result<()> {
    struct Client {
        socket: TcpStream,
        buff: [u8; 1024],
        len: usize,
    }

    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(1024);
    let mut listener = TcpListener::bind("0.0.0.0:55555".parse()?)?;

    poll.registry()
        .register(&mut listener, SERVER, Interest::READABLE)?;

    let mut clients = HashMap::new();

    let mut unique_token = Token(SERVER.0 + 1);

    loop {
        poll.poll(&mut events, None)?;

        for event in events.iter() {
            let token=event.token();
            if token==SERVER{
                //表示可accept
                let (mut socket, addr) = listener.accept()?;
                log::info!("addr:{} connect", addr);
                let client_key =next(&mut unique_token);
                poll.registry().register(
                    &mut socket,
                    client_key,
                    Interest::READABLE,
                )?;

                clients.insert(
                    client_key,
                    Client {
                        socket,
                        buff: [0; 1024],
                        len: 0,
                    },
                );
            }else if let Some(client) = clients.get_mut(&token){
                let mut disconnect = false;
                if event.is_readable(){
                    let size = match client.socket.read(&mut client.buff[..]) {
                        Ok(n) => n,
                        Err(err) if err.kind() == std::io::ErrorKind::ConnectionReset => 0,
                        Err(err) => {
                            log::error!("addr:{} error:{}", client.socket.peer_addr()?, err);
                            0
                        }
                    };
                    client.len = size;
                    disconnect = size == 0;

                    poll.registry().reregister(
                        &mut client.socket,
                        token,
                        Interest::WRITABLE,
                    )?;

                }else if event.is_writable(){
                    if let Err(err) = client.socket.write(&client.buff[..client.len]) {
                        log::error!("addr:{} error:{}", client.socket.peer_addr()?, err);
                        disconnect = true;
                    }

                    poll.registry().reregister(
                        &mut client.socket,
                        token,
                        Interest::READABLE,
                    )?;
                }

                if disconnect {
                    let mut client = clients.remove(&token).unwrap();
                    poll.registry().deregister(&mut client.socket)?;
                    log::info!("addr:{} disconnect", client.socket.peer_addr()?);
                }
            }
        }
    }
}

#[inline]
fn next(current: &mut Token) -> Token {
    let next = current.0;
    current.0 += 1;
    Token(next)
}