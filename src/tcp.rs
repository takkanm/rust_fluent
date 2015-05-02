use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpStream;

extern crate rustc_serialize;
use self::rustc_serialize::json;

extern crate time;

pub struct Fluentd {
    pub address: String,
    pub port: u16,
    pub tag: String,
}

impl Fluentd {
    pub fn write(&self, object: HashMap<String, String>) {
        let tag = json::encode(&self.tag).unwrap();
        let now = time::now();
        let record = json::encode(&object).unwrap();
        let message = format!("[{},{},{}]", tag, now.to_timespec().sec, record);

        let endpoint: &str = self.address.as_ref();
        let mut stream = TcpStream::connect((endpoint, self.port)).unwrap();

        let _ = stream.write(&message.into_bytes());

        drop(stream);
    }
}
