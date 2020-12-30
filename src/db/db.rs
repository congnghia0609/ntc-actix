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

use lazy_static::lazy_static;
// https://github.com/mongodb/mongo-rust-driver#enabling-the-sync-api
// use mongodb::{sync::Client, options::ClientOptions};
use mongodb::sync::Client;

lazy_static! {
    static ref LAZY_CONNECTION: Client = Client::with_uri_str("mongodb://localhost:27017").unwrap();
}

pub struct Connection;

pub trait IConnection {
    fn init(&self) -> &'static Client;
}

impl IConnection for Connection {
    fn init(&self) -> &'static Client {
        lazy_static::initialize(&LAZY_CONNECTION);
        &*LAZY_CONNECTION

        // // Parse a connection string into an options struct.
        // let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
        // // Manually set an option.
        // client_options.app_name = Some("ntc-axtix".to_string());
        // // Get a handle to the deployment.
        // let client = Client::with_options(client_options)?;
        // &client
        // let client = Client::with_uri_str("mongodb://localhost:27017").unwrap();
        // client
    }
}
