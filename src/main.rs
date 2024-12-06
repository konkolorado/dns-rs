use std::{fs::File, io::Read};

use dns_rs_lib::answer::Answer;
use dns_rs_lib::header::Header;
use dns_rs_lib::parser::print_bits;
use dns_rs_lib::records::A;

use dns_rs_lib::question::Question;

pub fn from_file(filename: &str) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    let mut inf = File::open(filename).expect("Error opening file");
    inf.read_to_end(&mut buf).expect("Error reading into vec");
    buf
}

pub fn main() -> () {
    //let buf = from_file("query_packet.txt");
    let buf = from_file("response_packet.txt");

    /*
    for i in 0..12 {
        print!("{}: ", i);
        print_bits(&buf[i]);
    }
    */
    let ref header = Header::from_buf(&buf);
    dbg!(header);

    /*
    for i in header._length..28 {
        print!("{}: ", i);
        print_bits(&buf[i]);
    }
    */

    let ref question = Question::from_buf(&buf[header._length..]);
    dbg!(question);

    /*for i in header._length + question._length..header._length + question._length + 10 {
        print!("{}: ", i);
        print_bits(&buf[i]);
    }
    */
    let ref answer = Answer::from_buf(&buf, header._length + question._length);
    dbg!(answer);

    let record = A::from_buf(&buf, header._length + question._length + answer._length);
    dbg!(record);
}
