/**
 *
 * @author nghiatc
 * @since 02/01/2021
 */

use actix_web::{FromRequest, Error, HttpRequest, dev};
use futures::future::{Ready, err, ok};
use actix_web::error::ErrorUnauthorized;
use crate::config::{Config, IConfig};
use crate::models::user::Claims;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

pub struct AuthorizationService;

impl FromRequest for AuthorizationService {
    type Error = Error;
    type Future = Ready<Result<AuthorizationService, Error>>;
    type Config = ();

    fn from_request(_req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let _auth = _req.headers().get("Authorization");
        match _auth {
            Some(_) => {
                let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
                let token = _split[1].trim();
                let _config: Config = Config {};
                let _var = _config.get_config_with_key("SECRET_KEY");
                let key = _var.as_bytes();
                match decode::<Claims> (
                    token,
                    &DecodingKey::from_secret(key),
                    &Validation::new(Algorithm::HS256),
                ) {
                    Ok(_token) => ok(AuthorizationService),
                    Err(_e) => err(ErrorUnauthorized("invalid tiken!")),
                }
            }
            None => err(ErrorUnauthorized("blocked!")),
        }
    }
}
