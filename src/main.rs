use std::{fs::File, io::Read};

use dns_rs_lib::packet::Packet;

pub fn from_file(filename: &str) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    let mut inf = File::open(filename).expect("Error opening file");
    inf.read_to_end(&mut buf).expect("Error reading into vec");
    buf
}

pub fn main() -> () {
    let buf = from_file("query_packet.txt");
    //let buf = from_file("response_packet.txt");

    let packet = Packet::from_buf(&buf);
    dbg!(packet);
}
