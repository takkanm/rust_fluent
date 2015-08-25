use std::io::prelude::*;
use std::net;
use std::io;
use std::convert::From;

extern crate rustc_serialize;
use rustc_serialize::json;
use rustc_serialize::Encodable;

extern crate time;

pub struct Fluentd<'a, A: net::ToSocketAddrs> {
    pub address: A,
    pub tag: &'a str,
}

#[derive(Debug)]
pub enum FluentError {
    DecodeError(json::EncoderError),
    IoError(io::Error),
}

impl From<io::Error> for FluentError {
    fn from(err: io::Error) -> FluentError {
        FluentError::IoError(err)
    }
}

impl From<json::EncoderError> for FluentError {
    fn from(err: json::EncoderError) -> FluentError {
        FluentError::DecodeError(err)
    }
}


impl <'a, A: net::ToSocketAddrs> Fluentd<'a, A> {
    pub fn write<B: Encodable> (&self, object: &B) -> Result<(), FluentError> {
        let tag = try!(json::encode(&self.tag));
        let now = time::now();
        let record = try!(json::encode(object));
        let message = format!("[{},{},{}]", tag, now.to_timespec().sec, record);

        let mut stream = try!(net::TcpStream::connect(&self.address));
        let _ = stream.write(&message.into_bytes());
        drop(stream);

        Ok(())
    }
}
