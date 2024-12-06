use crate::r#type::RRType;
use std::convert::TryInto;
use std::str;

#[derive(Debug)]
pub struct Question {
    pub name: String,
    pub r#type: RRType,
    pub class: u16,
    pub _length: usize,
}

impl Question {
    pub fn from_buf(buf: &[u8]) -> Self {
        //let question = &buf[12..];
        let question = buf;

        // TODO - idiomatically share these read_field methods
        let (name, index) = Self::read_labels(question);
        Self {
            name,
            r#type: Self::read_type(&question[index..]),
            class: Self::read_class(&question[index + 2..]),
            _length: index + 4,
        }
    }

    pub fn read_labels(buf: &[u8]) -> (String, usize) {
        let mut fragments = Vec::new();
        let mut curr_i = 0;
        let mut curr_byte = buf[curr_i];

        while curr_byte != 0b00000000 {
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
        (fragments.join("."), curr_i + 1)
    }

    pub fn read_type(buf: &[u8]) -> RRType {
        let bytes = &buf[..2];
        let value = u16::from_be_bytes(bytes.try_into().unwrap());
        RRType::from_value(value)
    }

    pub fn read_class(buf: &[u8]) -> u16 {
        let bytes = &buf[..2];
        u16::from_be_bytes(bytes.try_into().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_question() {
        let mut packet = vec![
            0b11000111, 0b01010111, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        ];
        let question = vec![
            0b00000110, 0b01100111, 0b01101111, 0b01101111, 0b01100111, 0b01101100, 0b01100101,
            0b00000011, 0b01100011, 0b01101111, 0b01101101, 0b00000000, 0b00000000, 0b00000001,
            0b00000000, 0b00000001,
        ];
        packet.extend(question);
        let question = Question::from_buf(&packet);
        assert_eq!(question.name, "google.com");
        assert_eq!(question.r#type, RRType::A);
        assert_eq!(question.class, 1);
    }

    #[test]
    fn test_parsing_question_with_type() {
        let mut packet = vec![
            0b11000111, 0b01010111, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        ];
        let question = vec![
            0b00000110, 0b01100111, 0b01101111, 0b01101111, 0b01100111, 0b01101100, 0b01100101,
            0b00000011, 0b01100011, 0b01101111, 0b01101101, 0b00000000, 0b00000000, 0b00000010,
            0b00000000, 0b00000001,
        ];
        packet.extend(question);
        let question = Question::from_buf(&packet);
        assert_eq!(question.name, "google.com");
        assert_eq!(question.r#type, RRType::NS);
        assert_eq!(question.class, 1);
    }

    #[test]
    fn test_parsing_question_with_class() {
        let mut packet = vec![
            0b11000111, 0b01010111, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        ];
        let question = vec![
            0b00000110, 0b01100111, 0b01101111, 0b01101111, 0b01100111, 0b01101100, 0b01100101,
            0b00000011, 0b01100011, 0b01101111, 0b01101101, 0b00000000, 0b00000000, 0b00000001,
            0b00000000, 0b00000010,
        ];
        packet.extend(question);
        let question = Question::from_buf(&packet);
        assert_eq!(question.name, "google.com");
        assert_eq!(question.r#type, RRType::A);
        assert_eq!(question.class, 2);
    }
}
