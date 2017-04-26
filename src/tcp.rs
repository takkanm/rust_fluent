use std::io::prelude::*;
use std::net;
use std::io;
use std::convert::From;

extern crate serde;
extern crate serde_json;
extern crate time;

pub struct Fluentd<'a, A: net::ToSocketAddrs> {
    pub address: A,
    pub tag: &'a str,
}

#[derive(Debug)]
pub enum FluentError {
    DecodeError(serde_json::Error),
    IoError(io::Error),
}

impl From<io::Error> for FluentError {
    fn from(err: io::Error) -> FluentError {
        FluentError::IoError(err)
    }
}

impl From<serde_json::Error> for FluentError {
    fn from(err: serde_json::Error) -> FluentError {
        FluentError::DecodeError(err)
    }
}

impl <'a, A: net::ToSocketAddrs> Fluentd<'a, A> {
    pub fn new(address: A, tag: &'a str) -> Fluentd<'a, A> {
        Fluentd {
            address: address,
            tag: tag
        }
    }

    pub fn write<B: serde::ser::Serialize>(&self, object: &B) -> Result<(), FluentError> {
        let tag = try!(serde_json::to_string(&self.tag));
        let now = time::now();
        let record = try!(serde_json::to_string(object));
        let message = format!("[{},{},{}]", tag, now.to_timespec().sec, record);

        let mut stream = try!(net::TcpStream::connect(&self.address));
        let result = stream.write(&message.into_bytes());
        drop(stream);

        match result {
            Ok(_) => Ok(()),
            Err(v) => Err(From::from(v)),
        }
    }
}
