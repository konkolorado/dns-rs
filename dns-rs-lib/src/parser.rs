use std::convert::TryInto;
use std::{fs::File, io::Read};

pub struct Parser {
    raw_data: Vec<u8>,
}

impl Parser {
    pub fn print_bits(byte: &u8) -> () {
        let mut s = "".to_string();
        let length = byte.count_ones() + byte.count_zeros();
        for n in (0..length).rev() {
            s.push_str(&(byte >> n & 1).to_string());
        }
        println!("{}", s);
    }

    pub fn from_file(filename: &str) -> Self {
        let mut buf: Vec<u8> = Vec::new();
        let mut inf = File::open(filename).expect("Error opening file");
        inf.read_to_end(&mut buf).expect("Error reading into vec");
        Self { raw_data: buf }
    }

    pub fn from_buf(buf: Vec<u8>) -> Self {
        Self { raw_data: buf }
    }

    fn bit_accessor(&self, byte: &u8, n: u8) -> u8 {
        // access a byte's nth bit from the left
        let length: u8 = (byte.count_ones() + byte.count_zeros()).try_into().unwrap();
        byte >> (length - n - 1) & 1
    }

    fn bits_to_u8(&self, bits: Vec<u8>) -> u8 {
        let mut result = 0u8;
        for (i, &bit) in bits.iter().enumerate() {
            if bit != 0 {
                result |= 1 << (7 - i - 4); // Set 4 bits, starting at the 4th bit from the left
            }
        }
        result
    }

    pub fn header(&self) -> &[u8] {
        &self.raw_data[0..12]
    }

    pub fn identifier(&self) -> u16 {
        let bytes = &self.header()[0..2];
        u16::from_be_bytes(bytes.try_into().unwrap())
    }

    pub fn is_query(&self) -> bool {
        let byte = &self.header()[2];
        let bit = self.bit_accessor(byte, 0);
        bit == 0
    }

    pub fn is_response(&self) -> bool {
        let byte = &self.header()[2];
        let bit: u8 = self.bit_accessor(byte, 0);
        bit == 1
    }

    pub fn op_code(&self) -> u8 {
        let mut my_bits = vec![];
        let byte = &self.header()[2];
        for n in 1..5 {
            let bit = self.bit_accessor(byte, n);
            my_bits.push(bit);
        }
        self.bits_to_u8(my_bits)
    }

    pub fn is_authoritative(&self) -> bool {
        let byte = &self.header()[2];
        let bit: u8 = self.bit_accessor(byte, 5);
        bit == 1
    }

    pub fn is_truncated(&self) -> bool {
        let byte = &self.header()[2];
        let bit: u8 = self.bit_accessor(byte, 6);
        bit == 1
    }

    pub fn should_recurse(&self) -> bool {
        let byte = &self.header()[2];
        let bit: u8 = self.bit_accessor(byte, 7);
        bit == 1
    }

    pub fn can_recurse(&self) -> bool {
        let byte = &self.header()[3];
        let bit: u8 = self.bit_accessor(byte, 0);
        bit == 1
    }

    pub fn reserved(&self) -> u8 {
        let mut my_bits = vec![];
        let byte = &self.header()[3];
        for n in 1..4 {
            let bit = self.bit_accessor(byte, n);
            my_bits.push(bit);
        }
        self.bits_to_u8(my_bits)
    }

    pub fn resp_code(&self) -> u8 {
        let mut my_bits = vec![];
        let byte = &self.header()[3];
        for n in 0..4 {
            let bit = self.bit_accessor(byte, 4 + n);
            my_bits.push(bit);
        }
        self.bits_to_u8(my_bits)
    }

    pub fn question_count(&self) -> u16 {
        let bytes = &self.header()[4..6];
        u16::from_be_bytes(bytes.try_into().unwrap())
    }

    pub fn answer_count(&self) -> u16 {
        let bytes = &self.header()[6..8];
        u16::from_be_bytes(bytes.try_into().unwrap())
    }

    pub fn authority_count(&self) -> u16 {
        let bytes = &self.header()[8..10];
        u16::from_be_bytes(bytes.try_into().unwrap())
    }

    pub fn additional_count(&self) -> u16 {
        let bytes = &self.header()[10..12];
        u16::from_be_bytes(bytes.try_into().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_first_two_bytes() {
        let packet = vec![
            0b11000111, 0b01010111, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        ];
        let resp = Parser::from_buf(packet);
        assert_eq!(resp.identifier(), 51031);

        let packet = vec![
            0b01000111, 0b01010111, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        ];
        let resp = Parser::from_buf(packet);
        assert_eq!(resp.identifier(), 18263);
    }

    #[test]
    fn test_parsing_third_byte() {
        let packet = vec![
            0b00000000, 0b00000000, 0b00001111, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        ];

        let resp = Parser::from_buf(packet);
        assert_eq!(resp.is_query(), true);
        assert_eq!(resp.is_response(), false);
        assert_eq!(resp.op_code(), 1);
        assert_eq!(resp.is_authoritative(), true);
        assert_eq!(resp.is_truncated(), true);
        assert_eq!(resp.should_recurse(), true);

        let packet = vec![
            0b00000000, 0b00000000, 0b10000001, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        ];

        let resp = Parser::from_buf(packet);
        assert_eq!(resp.is_query(), false);
        assert_eq!(resp.is_response(), true);
        assert_eq!(resp.op_code(), 0);
        assert_eq!(resp.is_authoritative(), false);
        assert_eq!(resp.is_truncated(), false);
        assert_eq!(resp.should_recurse(), true);
    }

    #[test]
    fn test_parsing_fourth_byte() {
        let packet = vec![
            0b00000000, 0b00000000, 0b00000000, 0b10000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        ];
        let resp = Parser::from_buf(packet);
        assert_eq!(resp.can_recurse(), true);
        assert_eq!(resp.reserved(), 0);
        assert_eq!(resp.resp_code(), 0);
    }

    #[test]
    fn test_parsing_question_counts() {
        let packet = vec![
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000001, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        ];
        let resp = Parser::from_buf(packet);
        assert_eq!(resp.question_count(), 1);

        let packet = [
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00000001, 0b00000000,
        ];
        let resp = Parser::from_buf(packet.to_vec());
        assert_eq!(resp.additional_count(), 256);
    }

    #[test]
    fn test_parsing_answer_counts() {
        let packet = vec![
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000001, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        ];
        let resp = Parser::from_buf(packet);
        assert_eq!(resp.answer_count(), 1);

        let packet = [
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00000001, 0b00000000,
        ];
        let resp = Parser::from_buf(packet.to_vec());
        assert_eq!(resp.additional_count(), 256);
    }

    #[test]
    fn test_parsing_authority_counts() {
        let packet = vec![
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000, 0b00000001, 0b00000000, 0b00000000,
        ];
        let resp = Parser::from_buf(packet);
        assert_eq!(resp.authority_count(), 1);

        let packet = [
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00000001, 0b00000000,
        ];
        let resp = Parser::from_buf(packet.to_vec());
        assert_eq!(resp.additional_count(), 256);
    }

    #[test]
    fn test_parsing_additional_counts() {
        let packet = vec![
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000001,
        ];
        let resp = Parser::from_buf(packet);
        assert_eq!(resp.additional_count(), 1);

        let packet = [
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00000001, 0b00000000,
        ];
        let resp = Parser::from_buf(packet.to_vec());
        assert_eq!(resp.additional_count(), 256);
    }
}
