use crate::arrays;

pub fn find_fast(mut u: u32) -> u32 {
    u += 0xe91aaa35;
    u ^= u >> 16;
    u += u << 8;
    u ^= u >> 4;
    let b: u32 = (u >> 8) & 0x1ff;
    let a: u32 = (u + (u << 2)) >> 19;
    let hash: u32 = arrays::HASH_ADJUST[b as usize];
    let c: u32 = a ^ hash;

    c
}
