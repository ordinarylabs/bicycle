/*
Bicycle is a framework for managing data.

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

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use prost::Message;
use prost_types::Value;

extern "C" {
    fn get_input() -> i64;
    fn set_output(ptr: i32, len: i32) -> ();

    fn get_examples_by_pk(ptr: i32, len: i32) -> i64;
    fn delete_examples_by_pk(ptr: i32, len: i32) -> ();
    fn put_example(ptr: i32, len: i32) -> ();
    fn batch_put_examples(ptr: i32, len: i32) -> ();
}

#[no_mangle]
pub extern "C" fn alloc(len: usize) -> *mut u8 {
    let mut buf: Vec<u8> = Vec::with_capacity(len);

    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);

    ptr
}

fn main() {
    unsafe {
        let input = get_input();

        if input != 0 {
            let input_ptr = (input >> 32) as i32;
            let input_len = input as i32;

            let input =
                Vec::from_raw_parts(input_ptr as *mut u8, input_len as usize, input_len as usize);

            let decoded_input = Value::decode(&input[..]).expect("should decode successfully");

            println!("decoded in guest: {:#?}", decoded_input);

            let result = get_examples_by_pk(input_ptr, input_len);

            let example_ptr = (result >> 32) as i32;
            let example_len = result as i32;

            delete_examples_by_pk(example_ptr, example_len);
            put_example(example_ptr, example_len);
            batch_put_examples(example_ptr, example_len);

            let mut encoded_output = decoded_input.encode_to_vec();

            let out_len = encoded_output.len();
            let out_ptr = encoded_output.as_mut_ptr();

            std::mem::forget(encoded_output);

            set_output(out_ptr as i32, out_len as i32);
        } else {
            println!("input was 0");
        }
    };
}
