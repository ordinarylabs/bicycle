/*
Bicycle is a database database framework.

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

use tonic::{Request, Response, Status};

mod proto {
    tonic::include_proto!("plugin");
}

pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("plugin_descriptor");

pub use proto::plugin_server::PluginServer as Server;
use proto::{plugin_server::Plugin, Echo};

pub struct Service {}

#[tonic::async_trait]
impl Plugin for Service {
    async fn plugin_echo(&self, req: Request<Echo>) -> Result<Response<Echo>, Status> {
        Ok(Response::new(req.into_inner()))
    }

    async fn plugin_echo_loud(&self, req: Request<Echo>) -> Result<Response<Echo>, Status> {
        use heck::ToShoutySnekCase;

        let resp = Echo {
            hiya_buddy: req.into_inner().hiya_buddy.TO_SHOUTY_SNEK_CASE(),
        };

        Ok(Response::new(resp))
    }
}
