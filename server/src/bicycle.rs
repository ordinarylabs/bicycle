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

use tonic::{Request, Response, Status};

use bicycle_core::{models, proto};

use proto::bicycle_server::Bicycle;
use proto::{Empty, IndexQuery};

pub struct BicycleService {}

#[tonic::async_trait]
impl Bicycle for BicycleService {
    // ##START_HANDLERS##
    async fn get_examples_by_pk(
        &self,
        req: Request<IndexQuery>,
    ) -> Result<Response<proto::Examples>, Status> {
        match models::example::get_examples_by_pk(req.into_inner()) {
            Ok(items) => Ok(Response::new(items)),
            Err(err) => {
                let msg = format!("failed to GET 'Examples': {}", err.to_string());
                Err(Status::internal(msg))
            }
        }
    }

    async fn delete_examples_by_pk(
        &self,
        req: Request<IndexQuery>,
    ) -> Result<Response<Empty>, Status> {
        match models::example::delete_examples_by_pk(req.into_inner()) {
            Ok(_) => Ok(Response::new(Empty {})),
            Err(err) => {
                let msg = format!("failed to DELETE 'Examples': {}", err.to_string());
                Err(Status::internal(msg))
            }
        }
    }

    async fn put_example(&self, req: Request<proto::Example>) -> Result<Response<Empty>, Status> {
        if let Err(err) = models::example::put_example(req.into_inner()) {
            let msg = format!("failed to PUT 'Example': {}", err.to_string());

            return Err(Status::internal(msg));
        }

        Ok(Response::new(Empty {}))
    }

    async fn batch_put_examples(
        &self,
        req: Request<proto::Examples>,
    ) -> Result<Response<Empty>, Status> {
        if let Err(err) = models::example::batch_put_examples(req.into_inner()) {
            let msg = format!("failed to BATCH PUT 'Examples': {}", err.to_string());

            return Err(Status::internal(msg));
        }

        Ok(Response::new(Empty {}))
    }
    // ##END_HANDLERS##
}
