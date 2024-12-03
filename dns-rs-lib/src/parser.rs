use std::convert::TryInto;

pub fn print_bits(byte: &u8) -> () {
    let mut s = "".to_string();
    let length = byte.count_ones() + byte.count_zeros();
    for n in (0..length).rev() {
        s.push_str(&(byte >> n & 1).to_string());
    }
    println!("{}", s);
}

pub fn bit_accessor(byte: &u8, n: u8) -> u8 {
    // access a byte's nth bit from the left
    let length: u8 = (byte.count_ones() + byte.count_zeros()).try_into().unwrap();
    byte >> (length - n - 1) & 1
}

pub fn bits_to_u8(bits: Vec<u8>) -> u8 {
    let mut result = 0u8;
    for (i, &bit) in bits.iter().enumerate() {
        if bit != 0 {
            result |= 1 << (7 - i - 4); // Set 4 bits, starting at the 4th bit from the left
        }
    }
    result
}
