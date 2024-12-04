use dns_rs_lib::header::Header;
use dns_rs_lib::parser::print_bits;
use dns_rs_lib::question::Question;

use std::{fs::File, io::Read};

pub fn from_file(filename: &str) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    let mut inf = File::open(filename).expect("Error opening file");
    inf.read_to_end(&mut buf).expect("Error reading into vec");
    buf
}

pub fn main() -> () {
    let buf = from_file("query_packet.txt");

    /*
    for i in 0..12 {
        print!("{}: ", i);
        print_bits(&buf[i]);
    }
    let header = Header::from_buf(&buf);
    dbg!(header);
    */

    for i in 12..28 {
        print!("{}: ", i);
        print_bits(&buf[i]);
    }
    let question = Question::from_buf(&buf);
    dbg!(question);
}
