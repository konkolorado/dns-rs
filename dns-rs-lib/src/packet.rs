use crate::answer::Answer;
use crate::header::Header;
use crate::question::Question;

#[derive(Debug)]

pub struct Packet {
    pub header: Header,
    pub question: Vec<Question>,
    pub answer: Vec<Answer>,
    pub authority: Vec<Answer>,
    pub additional: Vec<Answer>,
}

impl Packet {
    pub fn from_buf(buf: &[u8]) -> Self {
        let header = Header::from_buf(&buf);
        let question = Question::from_buf(&buf[header._length..]);

        let mut answers = Vec::new();
        for _ in 0..header.answer_count {
            let answer = Answer::from_buf(&buf, header._length + question._length);
            answers.push(answer);
        }

        let mut authorities = Vec::new();
        for _ in 0..header.authority_count {
            let auth = Answer::from_buf(&buf, header._length + question._length);
            authorities.push(auth);
        }

        let mut additionals = Vec::new();
        for _ in 0..header.additional_count {
            let add = Answer::from_buf(&buf, header._length + question._length);
            additionals.push(add);
        }

        Self {
            header,
            question: vec![question],
            answer: answers,
            authority: authorities,
            additional: additionals,
        }
    }
}
