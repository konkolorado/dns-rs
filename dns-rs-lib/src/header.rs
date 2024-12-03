#[derive(Debug)]
pub struct Header {
    pub identifier: u16,
    pub query: bool,
    pub response: bool,
    pub op_code: u8,
    pub is_authoritative: bool,
    pub truncated: bool,
    pub should_recurse: bool,
    pub can_recurse: bool,
    pub reserved: u8,
    pub resp_code: u8,
    pub question_count: u16,
    pub answer_count: u16,
    pub authority_count: u16,
    pub additional_count: u16,
}
