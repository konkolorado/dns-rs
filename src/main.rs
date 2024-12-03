use dns_rs_lib::header::Header;
use dns_rs_lib::parser::Parser;

pub fn main() -> () {
    let response = Parser::from_file("response_packet.txt");
    Parser::print_bits(&response.header()[0]);
    Parser::print_bits(&response.header()[1]);
    dbg!(response.identifier());

    Parser::print_bits(&response.header()[2]);
    dbg!(response.is_query());
    dbg!(response.is_response());
    dbg!(response.op_code());
    dbg!(response.is_authoritative());
    dbg!(response.is_truncated());
    dbg!(response.should_recurse());

    Parser::print_bits(&response.header()[3]);
    dbg!(response.can_recurse());
    dbg!(response.reserved());
    dbg!(response.resp_code());

    Parser::print_bits(&response.header()[4]);
    Parser::print_bits(&response.header()[5]);
    dbg!(response.question_count());

    Parser::print_bits(&response.header()[6]);
    Parser::print_bits(&response.header()[7]);
    dbg!(response.answer_count());

    Parser::print_bits(&response.header()[8]);
    Parser::print_bits(&response.header()[9]);
    dbg!(response.authority_count());

    Parser::print_bits(&response.header()[10]);
    Parser::print_bits(&response.header()[11]);
    dbg!(response.additional_count());
}
