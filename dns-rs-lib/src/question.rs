use crate::buf_reader::BufReader;
use crate::class::RRClass;
use crate::r#type::RRType;
use std::convert::TryInto;
use std::str;

#[derive(Debug)]
pub struct Question {
    pub name: String,
    pub r#type: RRType,
    pub class: RRClass,
}

impl Question {
    pub fn from_buf(buf: &mut BufReader) -> Self {
        let name = Self::read_labels(buf);
        Self {
            name,
            r#type: Self::read_type(buf),
            class: Self::read_class(buf),
        }
    }

    pub fn read_labels(buf: &mut BufReader) -> String {
        let mut fragments = Vec::new();
        let mut curr_byte = buf.read(1)[0];

        while curr_byte != 0b00000000 {
            let bytes = [curr_byte];
            let label_len = u8::from_be_bytes(bytes.try_into().unwrap()) as usize;
            let label_bytes = buf.read(label_len);

            let label = str::from_utf8(label_bytes).expect("Unable to convert bytes to str");
            fragments.push(label.to_owned());

            // advance to the next label len
            curr_byte = buf.read(1)[0];
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_question() {
        let packet = vec![
            0b11000111, 0b01010111, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        ];
        let question = vec![
            0b00000110, 0b01100111, 0b01101111, 0b01101111, 0b01100111, 0b01101100, 0b01100101,
            0b00000011, 0b01100011, 0b01101111, 0b01101101, 0b00000000, 0b00000000, 0b00000001,
            0b00000000, 0b00000001,
        ];
        let packet = [packet, question].concat();
        let mut buf = BufReader::new(&packet);
        let question = Question::from_buf(&mut buf);
        assert_eq!(question.name, "google.com");
        assert_eq!(question.r#type, RRType::A);
        assert_eq!(question.class, RRClass::IN);
    }

    #[test]
    fn test_parsing_question_with_type() {
        let packet = vec![
            0b11000111, 0b01010111, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        ];
        let question = vec![
            0b00000110, 0b01100111, 0b01101111, 0b01101111, 0b01100111, 0b01101100, 0b01100101,
            0b00000011, 0b01100011, 0b01101111, 0b01101101, 0b00000000, 0b00000000, 0b00000010,
            0b00000000, 0b00000001,
        ];
        let packet = [packet, question].concat();
        let mut buf = BufReader::new(&packet);
        let question = Question::from_buf(&mut buf);
        assert_eq!(question.name, "google.com");
        assert_eq!(question.r#type, RRType::NS);
        assert_eq!(question.class, RRClass::IN);
    }

    #[test]
    fn test_parsing_question_with_class() {
        let packet = vec![
            0b11000111, 0b01010111, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        ];
        let question = vec![
            0b00000110, 0b01100111, 0b01101111, 0b01101111, 0b01100111, 0b01101100, 0b01100101,
            0b00000011, 0b01100011, 0b01101111, 0b01101101, 0b00000000, 0b00000000, 0b00000001,
            0b00000000, 0b00000010,
        ];
        let packet = [packet, question].concat();
        let mut buf = BufReader::new(&packet);
        let question = Question::from_buf(&mut buf);
        assert_eq!(question.name, "google.com");
        assert_eq!(question.r#type, RRType::A);
        assert_eq!(question.class, RRClass::UNKNOWN);
    }
}
