use crate::answer::Answer;
use crate::buf_reader::BufReader;
use crate::header::Header;
use crate::question::Question;

#[derive(Debug)]

pub struct Packet {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<Answer>,
    pub authorities: Vec<Answer>,
    pub additionals: Vec<Answer>,
}

impl Packet {
    pub fn from_buf(buf: &[u8]) -> Self {
        let mut buf = BufReader::new(buf);
        let header = Header::from_buf(&mut buf);

        let mut questions = Vec::new();
        for _ in 0..header.question_count {
            let q = Question::from_buf(&mut buf);
            questions.push(q);
        }

        let mut answers = Vec::new();
        for _ in 0..header.answer_count {
            let a = Answer::from_buf(&mut buf);
            answers.push(a);
        }

        let mut authorities = Vec::new();
        for _ in 0..header.authority_count {
            let a = Answer::from_buf(&mut buf);
            authorities.push(a);
        }

        let mut additionals = Vec::new();
        for _ in 0..header.additional_count {
            let a = Answer::from_buf(&mut buf);
            additionals.push(a);
        }

        Self {
            header,
            questions,
            answers,
            authorities,
            additionals,
        }
    }
}
