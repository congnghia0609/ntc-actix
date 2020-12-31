/**
 *
 * @author nghiatc
 * @since 01/01/2021
 */

use crate::config::{Config, IConfig};
use crate::models::response::{Response, LoginResponse};
use crate::models::user::{Claims, Login, Register, User};
use mongodb::error::Error;
use mongodb::sync::Client;

pub trait IUserRepository {
    fn find_user_with_email(&self, email: String) -> Result<Option<User>, Error>;
    fn login(&self, login: Login) -> Result<LoginResponse, Response>;
    fn register(&self, user: Register) -> Response;
    fn user_informations(&self, token: &str) -> Result<Option<User>, Response>;
    fn protected_function(&self) -> bool;
}

pub struct UserRepository {
    pub connection: &'static Client,
}

impl IUserRepository for UserRepository {
    fn find_user_with_email(&self, email: String) -> Result<Option<User>, Error> {
        let _config: Config = Config{};
        let database_name = _config.get_config_with_key("DATABASE_NAME");
        let collection_name = _config.get_config_with_key("USER_COLLECTION_NAME");
        let db = self.connection.database(database_name.as_str());
        let cursor = db
            .collection(collection_name.as_str())
            .find_one(doc! {"email": email}, None)
            .unwrap();
        match cursor {
            Some(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
                Ok(model) => Ok(model),
                Err(e) => Err(Error::from(e)),
            },
            None => Ok(None),
        }
    }

    fn login(&self, login: Login) -> Result<LoginResponse, Response> {
        unimplemented!()
    }

    fn register(&self, user: Register) -> Response {
        unimplemented!()
    }

    fn user_informations(&self, token: &str) -> Result<Option<User>, Response> {
        unimplemented!()
    }

    fn protected_function(&self) -> bool {
        unimplemented!()
    }
}