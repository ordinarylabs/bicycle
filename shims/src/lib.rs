/*
Bicycle is a protobuf defined database framework.

Copyright (C) 2024 Ordinary Labs

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub use prost;
pub use prost_types;

mod models;
pub use models::*;

pub mod proto {
    tonic::include_proto!("bicycle");
}

use std::error::Error;

use prost::Message;
use prost_types::Value;

extern "C" {
    fn host_get_input() -> i64;
    fn host_set_output(ptr: i32, len: i32) -> i32;
}

/// exposes memory allocation functionality to host.
/// must be exported by your SPROC bin.
#[no_mangle]
pub extern "C" fn alloc(len: usize) -> *mut u8 {
    let mut buf: Vec<u8> = Vec::with_capacity(len);

    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);

    ptr
}

/// gets the SPROC input from the host.
/// must always be the first thing called.
pub fn recv_in() -> Result<Option<Value>, Box<dyn Error>> {
    let input = unsafe { host_get_input() };

    if input == 0 {
        return Ok(None);
    }

    let input_ptr = (input >> 32) as i32;
    let input_len = input as i32;

    let input = unsafe {
        Vec::from_raw_parts(input_ptr as *mut u8, input_len as usize, input_len as usize)
    };

    let decoded_input = Value::decode(&input[..])?;

    Ok(Some(decoded_input))
}

/// sets the output value for the SPROC.
/// must always be the last thing called.
pub fn send_out(output: Option<Value>) -> Result<(), Box<dyn Error>> {
    let res = match output {
        Some(out) => {
            let mut encoded_out = out.encode_to_vec();
            let out_len = encoded_out.len();
            let out_ptr = encoded_out.as_mut_ptr();

            std::mem::forget(encoded_out);
            unsafe { host_set_output(out_ptr as i32, out_len as i32) }
        }
        None => unsafe { host_set_output(1, 0) },
    };

    if res == 1 {
        Ok(())
    } else {
        Err("failed to write to output".into())
    }
}
