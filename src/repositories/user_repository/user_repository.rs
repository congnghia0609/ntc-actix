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

