/*
 * Copyright 2020 nghiatc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
/**
 *
 * @author nghiatc
 * @since 30/12/2020
 */

#[macro_use]
extern crate bson;

use actix_web::{middleware, web, App, HttpServer};
use actix_web::http::ContentEncoding;

mod config;
mod db;
mod models;
mod repositories;
mod middlewares;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Server is runing on: 0.0.0.0:8080");
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::new(ContentEncoding::Br))
            .wrap(middleware::Logger::default())
            .service(web::scope("/user").configure(repositories::user_repository::init_routes))
    })
        .bind("0.0.0.0:8080")?
        .run().await
}
