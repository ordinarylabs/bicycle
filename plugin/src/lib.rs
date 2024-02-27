/*
Bicycle is a database database framework.

Copyright (C) 2024  Ordinary Labs, LLC

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

// !! every plugin's `lib.rs` must start with the next 3 lines
#[allow(non_snake_case)]
mod proto;
use tonic::{Request, Response, Status};

// !! exported for use in the server
pub use proto::plugin_server::PluginServer;
use proto::{plugin_server::Plugin, Echo};

// !! must be named `XyzService` where `Xyz` is the PascalCase name of your plugin
pub struct PluginService {}

#[tonic::async_trait]
impl Plugin for PluginService {
    async fn plugin_echo(&self, req: Request<Echo>) -> Result<Response<Echo>, Status> {
        Ok(Response::new(req.into_inner()))
    }
}
