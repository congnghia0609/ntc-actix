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

use envfile::EnvFile;
use std::path::Path;

pub trait IConfig {
    fn get_config_with_key(&self, key: &str) -> String;
}

pub struct Config{}

impl IConfig for Config {
    fn get_config_with_key(&self, key: &str) -> String {
        let env = EnvFile::new(&Path::new("src/config/config.env")).unwrap();
        env.get(key).unwrap().to_string()
    }
}
