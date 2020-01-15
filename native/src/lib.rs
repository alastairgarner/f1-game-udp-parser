#![allow(non_snake_case)]
extern crate neon;
#[macro_use]
extern crate nom;
extern crate neon_serde;
extern crate serde_derive;

pub mod parser;

use neon::prelude::*;
use neon_serde::to_value;

fn parseMessage(mut cx: FunctionContext) -> JsResult<JsValue> {
    let buf: Handle<JsBuffer> = cx.argument(0)?;
    let buf = cx.borrow(&buf, |data| data.as_slice().to_vec());

    // parse the packet
    let packet = parser::parse_packet(&buf);

    // return the message
    let result = to_value(&mut cx, &packet)?;

    Ok(result)
}

register_module!(mut cx, {
    cx.export_function("parseMessage", parseMessage)
});
