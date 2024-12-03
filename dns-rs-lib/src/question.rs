use std::convert::TryInto;

#[derive(Debug)]
pub struct Question {}

impl Question {
    pub fn from_buf(buf: &Vec<u8>) -> Self {
        let question = &buf[12..];
        Self {}
    }

    pub fn read_label(buf: Vec<u8>) -> u8 {
        let bytes = [buf[0]];
        u8::from_be_bytes(bytes.try_into().unwrap())
    }
}
