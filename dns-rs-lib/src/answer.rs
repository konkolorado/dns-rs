use crate::buf_reader::BufReader;
use crate::class::RRClass;
use crate::parser::bit_accessor;
use crate::r#type::RRType;
use std::convert::TryInto;
use std::str;

#[derive(Debug)]
pub struct Answer {
    pub name: String,
    pub r#type: RRType,
    pub class: RRClass,
    pub ttl: u32,
    pub len: u16,
    pub ip: String,
}

impl Answer {
    pub fn from_buf(buf: &mut BufReader) -> Self {
        let name = Self::read_labels(buf);
        Self {
            name,
            r#type: Self::read_type(buf),
            class: Self::read_class(buf),
            ttl: Self::read_ttl(buf),
            len: Self::read_len(buf),
            ip: "ip".to_string(),
        }
    }

    pub fn read_labels(buf: &mut BufReader) -> String {
        let mut fragments = Vec::new();
        let started_at = buf.pos;
        let mut curr_byte = buf.read(1)[0];
        let mut jumped = false;

        while curr_byte != 0b00000000 {
            // jump if the first two bytes are set
            if bit_accessor(&curr_byte, 0) == 1 && bit_accessor(&curr_byte, 1) == 1 {
                let bytes = [curr_byte, buf.read(1)[0]];
                let value = u16::from_be_bytes(bytes.try_into().unwrap());
                let goto = (value ^ 0b1100000000000000) as usize;
                buf.goto(goto);

                curr_byte = buf.read(1)[0];
                jumped = true;
            }

            let bytes = [curr_byte];
            let label_len = u8::from_be_bytes(bytes.try_into().unwrap()) as usize;
            let label_bytes = buf.read(label_len);
            let label = str::from_utf8(label_bytes).expect("Unable to convert bytes to str");
            fragments.push(label.to_owned());

            // advance to the next label len
            curr_byte = buf.read(1)[0];
        }

        if jumped {
            // reset our position to be where we started + 2
            buf.goto(started_at + 2);
        }
        fragments.join(".")
    }

    pub fn read_type(buf: &mut BufReader) -> RRType {
        let bytes: &[u8] = buf.read(2);
        let value = u16::from_be_bytes(bytes.try_into().unwrap());
        RRType::from_value(value)
    }

    pub fn read_class(buf: &mut BufReader) -> RRClass {
        let bytes: &[u8] = buf.read(2);
        let value = u16::from_be_bytes(bytes.try_into().unwrap());
        RRClass::from_value(value)
    }

    pub fn read_ttl(buf: &mut BufReader) -> u32 {
        let bytes: &[u8] = buf.read(4);
        u32::from_be_bytes(bytes.try_into().unwrap())
    }

    pub fn read_len(buf: &mut BufReader) -> u16 {
        let bytes: &[u8] = buf.read(2);
        u16::from_be_bytes(bytes.try_into().unwrap())
    }
}
