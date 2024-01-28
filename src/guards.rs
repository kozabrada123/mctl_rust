//! Defines requests guards

use log::info;
use rocket::http::Status;
use rocket::request;
use rocket::request::FromRequest;
use rocket::request::Request;

use crate::errors;

#[derive(Clone, PartialEq, Eq)]
/// Request quard for a request to have a valid api key
pub struct AuthenticatedUser {}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = errors::RequestError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let header_result = req.headers().get("Authorization").next();

        if header_result.is_none() {
            return request::Outcome::Error((
                Status::Unauthorized,
                errors::RequestError::AuthenticationError {},
            ));
        }

        let authorization = header_result.unwrap().to_string().trim().to_string();

        let config = crate::config::Config::load().unwrap();

        let matches_keys = crate::hash::verify(authorization, config.auth_key_hash);

        if !matches_keys {
            return request::Outcome::Error((
                Status::Forbidden,
                errors::RequestError::AuthenticationError {},
            ));
        }

        return request::Outcome::Success(AuthenticatedUser {});
    }
}
