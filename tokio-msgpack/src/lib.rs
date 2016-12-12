// Copyright 2016 eochat developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate rmp;
extern crate serde;
extern crate rmp_serde;
extern crate tokio_core;
extern crate tokio_proto;

mod msgpack;

use std::io;
use tokio_core::io::{Io, Codec, Framed, EasyBuf};
use tokio_proto::pipeline::ServerProto;

pub struct MsgPackServer<Req, Res>
    where Req: serde::Deserialize,
          Res: serde::Serialize {
    req: std::marker::PhantomData<Req>,
    res: std::marker::PhantomData<Res>,
}

impl<Req, Res> MsgPackServer<Req, Res>
    where Req: serde::Deserialize,
          Res: serde::Serialize {

    pub fn new() -> MsgPackServer<Req, Res> {
        MsgPackServer {
            req: std::marker::PhantomData,
            res: std::marker::PhantomData,
        }
    }
}

impl<T: Io + 'static, Req: 'static, Res: 'static> ServerProto<T> for MsgPackServer<Req, Res>
    where Req: serde::Deserialize,
          Res: serde::Serialize {

    type Request = Req;
    type Response = Res;
    type Error = io::Error;
    type Transport = Framed<T, MsgPackCodec<Req, Res>>;
    type BindTransport = io::Result<Framed<T, MsgPackCodec<Req, Res>>>;

    fn bind_transport(&self, io: T) -> io::Result<Framed<T, MsgPackCodec<Req, Res>>> {
        Ok(io.framed(MsgPackCodec::new()))
    }
}

pub struct MsgPackCodec<Req, Res>
    where Req: serde::Deserialize,
          Res: serde::Serialize {
    req: std::marker::PhantomData<Req>,
    res: std::marker::PhantomData<Res>,
}

impl<Req, Res> MsgPackCodec<Req, Res>
    where Req: serde::Deserialize,
          Res: serde::Serialize {

    pub fn new() -> MsgPackCodec<Req, Res> {
        MsgPackCodec {
            req: std::marker::PhantomData,
            res: std::marker::PhantomData,
        }
    }
}

impl<Req, Res> Codec for MsgPackCodec<Req, Res>
    where Req: serde::Deserialize,
          Res: serde::Serialize {

    type In = Req;
    type Out = Res;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Req>> {
        println!("DECODE");
        msgpack::decode(buf)
    }

    fn encode(&mut self, msg: Res, buf: &mut Vec<u8>) -> io::Result<()> {
        let _ = msgpack::encode(msg, buf);
        Ok(())
    }
}
