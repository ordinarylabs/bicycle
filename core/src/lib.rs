/*
Bicycle is a database, used for things databases are used for.

Copyright (C) 2023  sean watters

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

#[macro_use]
extern crate lazy_static;

use rocksdb::{Options, DB};

#[allow(non_snake_case)]
pub mod proto {
    tonic::include_proto!("bicycle");

    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("bicycle_descriptor");
}

pub mod models;

// TODO: evaluate using a OnceCell here instead
lazy_static! {
    pub(crate) static ref ROCKSDB_CONNECTION: DB = {
        let mut opts = Options::default();
        opts.create_if_missing(true);

        DB::open(&opts, "__bicycle__").unwrap()
    };
}
