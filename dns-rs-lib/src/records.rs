#[derive(Debug)]
pub struct A {
    pub ip: String,
}

impl A {
    pub fn from_buf(buf: &[u8], start_at: usize) -> Self {
        Self {
            ip: Self::read_ip(&buf[start_at..]),
        }
    }

    fn read_ip(buf: &[u8]) -> String {
        let mut fragments = Vec::new();
        for i in 0..4 {
            // str::from_utf8(buf[i]).expect("Unable to convert bytes to str")
            fragments.push(buf[i].to_string());
        }
        fragments.join(".")
    }
}
