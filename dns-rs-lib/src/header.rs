pub struct Header {
    identifier: u16,
    query: bool,
    respose: bool,
    op_code: u8,
    is_authoritative: bool,
    truncated: bool,
    should_recurse: bool,
    can_recurse: bool,
    reserved: u8,
    resp_code: u8,
    question_count: u16,
    answer_count: u16,
    authority_count: u16,
    additional_count: u16,
}
