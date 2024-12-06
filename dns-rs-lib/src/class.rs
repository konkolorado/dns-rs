#[derive(Debug, PartialEq)]
pub enum RRClass {
    IN = 1,
    CH = 3,
    HS = 4,
    UNKNOWN,
}

impl RRClass {
    pub fn from_value(value: u16) -> Self {
        match value {
            1 => RRClass::IN,
            3 => RRClass::CH,
            4 => RRClass::HS,
            _ => RRClass::UNKNOWN,
        }
    }
}
