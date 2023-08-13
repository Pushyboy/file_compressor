use std::{hash::{Hasher, BuildHasher}};

const FNV_PRIME: u32 = 0x01000193;
const FNV_OFFSET_BASIS: u32 = 0x811c9dc5;

pub struct FNV1a {
    hash: u32,
}

impl FNV1a {
    pub fn new() -> Self {
        FNV1a {hash: FNV_OFFSET_BASIS}
    }
}

impl Hasher for FNV1a {
    fn finish(&self) -> u64 {
        self.hash as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.hash ^= *b as u32;
            self.hash = self.hash.wrapping_mul(FNV_PRIME);
        }
    }
}

pub struct FNV1aBuilder;

impl BuildHasher for FNV1aBuilder {
    type Hasher = FNV1a;

    fn build_hasher(&self) -> FNV1a {
        FNV1a::new()
    }

}


