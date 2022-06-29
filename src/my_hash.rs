use std::hash::{BuildHasher, Hasher};


#[derive(Copy, Clone, Default)]
pub(crate) struct MyHasher(u64);

impl Hasher for MyHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.0 = bytes.iter().map(|x|*x as u64).sum();
    }
    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.0 = i as u64;
    }
    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.0 = i as u64;
    }
    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.0 = i as u64;
    }
    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.0 = i;
    }
    #[inline]
    fn write_u128(&mut self, i: u128) {
        self.0 = i as u64;
    }
    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.0 = i as u64;
    }
    #[inline]
    fn write_i8(&mut self, i: i8) {
        self.0 = i as u64;
    }
    #[inline]
    fn write_i16(&mut self, i: i16) {
        self.0 = i as u64;
    }
    #[inline]
    fn write_i32(&mut self, i: i32) {
        self.0 = i as u64;
    }
    #[inline]
    fn write_i64(&mut self, i: i64) {
        self.0 = i as u64;
    }
    #[inline]
    fn write_i128(&mut self, i: i128) {
        self.0 = i as u64;
    }
    #[inline]
    fn write_isize(&mut self, i: isize) {
        self.0 = i as u64;
    }
}

pub(crate) struct MyHashBuilder {
    hasher: MyHasher,
}

impl MyHashBuilder {
    pub fn new() -> Self {
        Self {
            hasher: MyHasher(0),
        }
    }
}
impl BuildHasher for MyHashBuilder {
    type Hasher = MyHasher;
    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        self.hasher.clone()
    }
}