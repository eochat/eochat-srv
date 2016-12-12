// Copyright 2016 eochat developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use tokio_core::io::EasyBuf;
use serde::{Deserialize, Serialize};
use rmp_serde::{Deserializer, Serializer};
use std::io::{self, Cursor};

pub fn decode<R: Deserialize>(buf: &mut EasyBuf) -> io::Result<Option<R>> {
    let bytes = buf.as_slice();
    let cur = Cursor::new(&bytes[..]);
    let mut de = Deserializer::new(cur);
    match Deserialize::deserialize(&mut de) {
        Ok(v) => Ok(Some(v)),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, format!("{}", e))),
    }
}

pub fn encode<R: Serialize>(msg: R, buf: &mut Vec<u8>) -> io::Result<()> {
    match msg.serialize(&mut Serializer::new(buf)) {
        Ok(_) => Ok(()),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, format!("{}", e))),
    }
}
