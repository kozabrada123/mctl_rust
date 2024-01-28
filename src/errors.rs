//! Defines possible errors we send back to the requester

use custom_error::custom_error;
use rocket::{
    http::Status,
    response::{self, status, Responder},
    Request, Response,
};

custom_error! {
    #[derive(PartialEq)]
    pub RequestError
    AuthenticationError = "Authentication failed (missing / invalid api key)",
    GitError{error: git2::Error} = "A Git error ocurred: {error}",
}

impl<'r> Responder<'r, 'static> for RequestError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let string = format!("{}", self);

        let status_code = match self {
            Self::AuthenticationError => Status::Forbidden,
            Self::GitError { .. } => Status::InternalServerError,
            _ => Status::InternalServerError,
        };

        let status = status::Custom(status_code, string);

        Response::build_from(status.respond_to(req)?).ok()
    }
}
