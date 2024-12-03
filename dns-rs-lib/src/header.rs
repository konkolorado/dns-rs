use crate::parser;
use std::convert::TryInto;

#[derive(Debug)]
pub struct Header {
    pub identifier: u16,
    pub query: bool,
    pub response: bool,
    pub op_code: u8,
    pub is_authoritative: bool,
    pub truncated: bool,
    pub should_recurse: bool,
    pub can_recurse: bool,
    pub reserved: u8,
    pub resp_code: u8,
    pub question_count: u16,
    pub answer_count: u16,
    pub authority_count: u16,
    pub additional_count: u16,
}

impl Header {
    pub fn from_buf(buf: &Vec<u8>) -> Self {
        let header = &buf[0..12];
        Self {
            identifier: Self::identifier(header),
            query: Self::is_query(header),
            response: Self::is_response(header),
            op_code: Self::op_code(header),
            is_authoritative: Self::is_authoritative(header),
            truncated: Self::is_truncated(header),
            should_recurse: Self::should_recurse(header),
            can_recurse: Self::can_recurse(header),
            reserved: Self::reserved(header),
            resp_code: Self::resp_code(header),
            question_count: Self::question_count(header),
            answer_count: Self::answer_count(header),
            authority_count: Self::authority_count(header),
            additional_count: Self::additional_count(header),
        }
    }

    fn identifier(buf: &[u8]) -> u16 {
        let bytes = &buf[0..2];
        u16::from_be_bytes(bytes.try_into().unwrap())
    }

    fn is_query(buf: &[u8]) -> bool {
        let byte = &buf[2];
        let bit = parser::bit_accessor(byte, 0);
        bit == 0
    }

    fn is_response(buf: &[u8]) -> bool {
        let byte = &buf[2];
        let bit = parser::bit_accessor(byte, 0);
        bit == 1
    }

    fn op_code(buf: &[u8]) -> u8 {
        let mut my_bits = vec![];
        let byte = &buf[2];
        for n in 1..5 {
            let bit = parser::bit_accessor(byte, n);
            my_bits.push(bit);
        }
        parser::bits_to_u8(my_bits)
    }

    fn is_authoritative(buf: &[u8]) -> bool {
        let byte = &buf[2];
        let bit: u8 = parser::bit_accessor(byte, 5);
        bit == 1
    }

    fn is_truncated(buf: &[u8]) -> bool {
        let byte = &buf[2];
        let bit: u8 = parser::bit_accessor(byte, 6);
        bit == 1
    }

    fn should_recurse(buf: &[u8]) -> bool {
        let byte = &buf[2];
        let bit: u8 = parser::bit_accessor(byte, 7);
        bit == 1
    }

    fn can_recurse(buf: &[u8]) -> bool {
        let byte = &buf[3];
        let bit: u8 = parser::bit_accessor(byte, 0);
        bit == 1
    }

    fn reserved(buf: &[u8]) -> u8 {
        let mut my_bits = vec![];
        let byte = &buf[3];
        for n in 1..4 {
            let bit = parser::bit_accessor(byte, n);
            my_bits.push(bit);
        }
        parser::bits_to_u8(my_bits)
    }

    fn resp_code(buf: &[u8]) -> u8 {
        let mut my_bits = vec![];
        let byte = &buf[3];
        for n in 0..4 {
            let bit = parser::bit_accessor(byte, 4 + n);
            my_bits.push(bit);
        }
        parser::bits_to_u8(my_bits)
    }

    fn question_count(buf: &[u8]) -> u16 {
        let bytes = &buf[4..6];
        u16::from_be_bytes(bytes.try_into().unwrap())
    }

    fn answer_count(buf: &[u8]) -> u16 {
        let bytes = &buf[6..8];
        u16::from_be_bytes(bytes.try_into().unwrap())
    }

    fn authority_count(buf: &[u8]) -> u16 {
        let bytes = &buf[8..10];
        u16::from_be_bytes(bytes.try_into().unwrap())
    }

    fn additional_count(buf: &[u8]) -> u16 {
        let bytes = &buf[10..12];
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
        let header = Header::from_buf(&packet);
        assert_eq!(header.identifier, 51031);

        let packet = vec![
            0b01000111, 0b01010111, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        ];
        let header = Header::from_buf(&packet);
        assert_eq!(header.identifier, 18263);
    }

    #[test]
    fn test_parsing_third_byte() {
        let packet = vec![
            0b00000000, 0b00000000, 0b00001111, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        ];

        let header = Header::from_buf(&packet);
        assert_eq!(header.query, true);
        assert_eq!(header.response, false);
        assert_eq!(header.op_code, 1);
        assert_eq!(header.is_authoritative, true);
        assert_eq!(header.truncated, true);
        assert_eq!(header.should_recurse, true);

        let packet = vec![
            0b00000000, 0b00000000, 0b10000001, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        ];

        let header = Header::from_buf(&packet);
        assert_eq!(header.query, false);
        assert_eq!(header.response, true);
        assert_eq!(header.op_code, 0);
        assert_eq!(header.is_authoritative, false);
        assert_eq!(header.truncated, false);
        assert_eq!(header.should_recurse, true);
    }

    #[test]
    fn test_parsing_fourth_byte() {
        let packet = vec![
            0b00000000, 0b00000000, 0b00000000, 0b10000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        ];
        let header = Header::from_buf(&packet);
        assert_eq!(header.can_recurse, true);
        assert_eq!(header.reserved, 0);
        assert_eq!(header.resp_code, 0);
    }

    #[test]
    fn test_parsing_question_counts() {
        let packet = vec![
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000001, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        ];
        let header = Header::from_buf(&packet);
        assert_eq!(header.question_count, 1);

        let packet = vec![
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00000001, 0b00000000,
        ];
        let header = Header::from_buf(&packet);
        assert_eq!(header.additional_count, 256);
    }

    #[test]
    fn test_parsing_answer_counts() {
        let packet = vec![
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000001, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        ];
        let header = Header::from_buf(&packet);
        assert_eq!(header.answer_count, 1);

        let packet = vec![
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00000001, 0b00000000,
        ];
        let header = Header::from_buf(&packet);
        assert_eq!(header.additional_count, 256);
    }

    #[test]
    fn test_parsing_authority_counts() {
        let packet = vec![
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000, 0b00000001, 0b00000000, 0b00000000,
        ];
        let header = Header::from_buf(&packet);
        assert_eq!(header.authority_count, 1);

        let packet = vec![
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00000001, 0b00000000,
        ];
        let header = Header::from_buf(&packet);
        assert_eq!(header.additional_count, 256);
    }

    #[test]
    fn test_parsing_additional_counts() {
        let packet = vec![
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000001,
        ];
        let header = Header::from_buf(&packet);
        assert_eq!(header.additional_count, 1);

        let packet = vec![
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00000001, 0b00000000,
        ];
        let header = Header::from_buf(&packet);
        assert_eq!(header.additional_count, 256);
    }
}
