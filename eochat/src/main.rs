// Copyright 2016 eochat developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(proc_macro)]

extern crate env_logger;
extern crate futures;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate tokio_proto;
extern crate tokio_service;
extern crate tokio_msgpack;

use futures::future;
use std::io;
use tokio_service::Service;
use tokio_proto::TcpServer;
use tokio_msgpack::MsgPackServer;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Hello,
    Join(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Ok,
    Err(String),
}

struct WelcomeServer;

impl Service for WelcomeServer {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = future::Ok<Response, io::Error>;

    fn call(&self, _request: Request) -> Self::Future {
        println!("YOLO");
        future::ok(Response::Ok)
    }
}

fn main() {
    let _ = env_logger::init();
    let addr = "0.0.0.0:1492".parse().unwrap();
    TcpServer::new(MsgPackServer::new(), addr)
        .serve(|| Ok(WelcomeServer));
}
