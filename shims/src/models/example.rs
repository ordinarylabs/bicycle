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

use std::error::Error;

use crate::proto::{Example, Examples, IndexQuery};
use prost::Message;

extern "C" {
    fn host_get_examples_by_pk(ptr: i32, len: i32) -> i64;
    fn host_delete_examples_by_pk(ptr: i32, len: i32) -> i32;
    fn host_put_example(ptr: i32, len: i32) -> i32;
    fn host_batch_put_examples(ptr: i32, len: i32) -> i32;
}

pub fn get_examples_by_pk(index_query: IndexQuery) -> Result<Examples, Box<dyn Error>> {
    let mut encoded_index_query = index_query.encode_to_vec();
    let index_query_len = encoded_index_query.len();
    let index_query_ptr = encoded_index_query.as_mut_ptr();

    std::mem::forget(encoded_index_query);

    let examples =
        unsafe { host_get_examples_by_pk(index_query_ptr as i32, index_query_len as i32) };

    let examples_ptr = (examples >> 32) as i32;
    let examples_len = examples as i32;

    let encoded_examples = unsafe {
        Vec::from_raw_parts(
            examples_ptr as *mut u8,
            examples_len as usize,
            examples_len as usize,
        )
    };

    let examples = Examples::decode(&encoded_examples[..])?;

    Ok(examples)
}

pub fn delete_examples_by_pk(index_query: IndexQuery) -> Result<(), Box<dyn Error>> {
    let mut encoded_index_query = index_query.encode_to_vec();
    let index_query_len = encoded_index_query.len();
    let index_query_ptr = encoded_index_query.as_mut_ptr();

    std::mem::forget(encoded_index_query);

    unsafe { host_delete_examples_by_pk(index_query_ptr as i32, index_query_len as i32) };

    Ok(())
}

pub fn put_example(example: Example) -> Result<(), Box<dyn Error>> {
    let mut encoded_example = example.encode_to_vec();
    let example_len = encoded_example.len();
    let example_ptr = encoded_example.as_mut_ptr();

    std::mem::forget(encoded_example);

    unsafe { host_put_example(example_ptr as i32, example_len as i32) };

    Ok(())
}

pub fn batch_put_examples(examples: Examples) -> Result<(), Box<dyn Error>> {
    let mut encoded_examples = examples.encode_to_vec();
    let examples_len = encoded_examples.len();
    let examples_ptr = encoded_examples.as_mut_ptr();

    std::mem::forget(encoded_examples);

    unsafe { host_batch_put_examples(examples_ptr as i32, examples_len as i32) };

    Ok(())
}
