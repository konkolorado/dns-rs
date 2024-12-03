use dns_rs_lib::header::Header;
use dns_rs_lib::parser::print_bits;

use std::{fs::File, io::Read};

pub fn from_file(filename: &str) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    let mut inf = File::open(filename).expect("Error opening file");
    inf.read_to_end(&mut buf).expect("Error reading into vec");
    buf
}

pub fn main() -> () {
    let buf = from_file("response_packet.txt");

    print_bits(&buf[0]);
    print_bits(&buf[1]);
    print_bits(&buf[2]);
    print_bits(&buf[3]);
    print_bits(&buf[4]);
    print_bits(&buf[5]);
    print_bits(&buf[6]);
    print_bits(&buf[7]);
    print_bits(&buf[8]);
    print_bits(&buf[9]);
    print_bits(&buf[10]);
    print_bits(&buf[11]);

    let header = Header::from_buf(&buf);
    dbg!(header);
}
