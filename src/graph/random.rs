// xorshift32 restricted to 8 bits
pub fn u8() -> u8 {
    unsafe {
        static mut x: u32 = 1;  // change the seed to get a different sequence
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        return x as u8;
    }
}
