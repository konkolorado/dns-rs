use crate::parser::bit_accessor;
use std::convert::TryInto;
use std::str;

#[derive(Debug)]
pub struct Answer {
    pub name: String,
    pub r#type: u16,
    pub class: u16,
    pub ttl: u32,
    pub len: u16,
    pub ip: String,
    pub _length: usize,
}

impl Answer {
    pub fn from_buf(buf: &[u8], start_at: usize) -> Self {
        let (name, index) = Self::read_labels(buf, start_at);
        Self {
            name,
            r#type: Self::read_type(&buf[index..]),
            class: Self::read_class(&buf[index + 2..]),
            ttl: Self::read_ttl(&buf[index + 4..]),
            len: Self::read_len(&buf[index + 8..]),
            ip: "ip".to_string(),
            _length: index + 10 - start_at,
        }
    }

    pub fn read_labels(buf: &[u8], start_at: usize) -> (String, usize) {
        let mut fragments = Vec::new();
        let mut curr_i = start_at;
        let mut curr_byte = buf[curr_i];
        let mut jumped = false;

        while curr_byte != 0b00000000 {
            // jump if the first two bytes are set
            if bit_accessor(&curr_byte, 0) == 1 && bit_accessor(&curr_byte, 1) == 1 {
                let value: u16 = u16::from_be_bytes(buf[curr_i..curr_i + 2].try_into().unwrap());
                curr_i = (value ^ 0b1100000000000000) as usize;
                curr_byte = buf[curr_i];
                jumped = true;
            }

            let bytes = [curr_byte];
            let label_len = u8::from_be_bytes(bytes.try_into().unwrap()) as usize;

            // advance to start reading the label
            curr_i += 1;

            let label_bytes = &buf[curr_i..curr_i + label_len];
            let label = str::from_utf8(label_bytes).expect("Unable to convert bytes to str");
            fragments.push(label);

            // advance to the next label len
            curr_i += label_len;
            curr_byte = buf[curr_i];
        }
        if jumped {
            (fragments.join("."), start_at + 2)
        } else {
            (fragments.join("."), curr_i + 1)
        }
    }

    pub fn read_type(buf: &[u8]) -> u16 {
        let bytes = &buf[..2];
        u16::from_be_bytes(bytes.try_into().unwrap())
    }

    pub fn read_class(buf: &[u8]) -> u16 {
        let bytes = &buf[..2];
        u16::from_be_bytes(bytes.try_into().unwrap())
    }

    pub fn read_ttl(buf: &[u8]) -> u32 {
        let bytes = &buf[..4];
        u32::from_be_bytes(bytes.try_into().unwrap())
    }

    pub fn read_len(buf: &[u8]) -> u16 {
        let bytes = &buf[..2];
        u16::from_be_bytes(bytes.try_into().unwrap())
    }
}
