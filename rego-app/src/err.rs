use crate::res::{response, Response};
use futures::future;
use http::StatusCode;
use std::borrow::Cow;
use warp::reject::{Reject, Rejection};

#[derive(Debug, Clone)]
pub struct Error {
    pub status: StatusCode,
    pub msg: Cow<'static, str>,
}

impl Error {
    /*
     * ==============
     * Constructor methods
     * ==============
     */
    pub fn new<S>(status: StatusCode, msg: S) -> Self
    where
        Cow<'static, str>: From<S>,
    {
        Error {
            status,
            msg: msg.into(),
        }
    }

    pub fn not_found(msg: &'static str) -> Self {
        Error::new(StatusCode::NOT_FOUND, msg)
    }

    pub fn bad_request(msg: &'static str) -> Self {
        Error::new(StatusCode::BAD_REQUEST, msg)
    }

    pub fn unauthorized() -> Self {
        Error::new(StatusCode::UNAUTHORIZED, "")
    }

    pub fn conflict(msg: &'static str) -> Self {
        Error::new(StatusCode::CONFLICT, msg)
    }

    pub fn internal_server_error(msg: &'static str) -> Self {
        Error::new(StatusCode::INTERNAL_SERVER_ERROR, msg)
    }

    /*
     * =============
     * Modifier
     * =============
     */
    pub fn add_msg<S>(self, msg: S) -> Self
    where
        S: AsRef<str>,
    {
        Error::new(self.status, format!("{}\n{}", self.msg, msg.as_ref()))
    }

    /*
     * ==========
     * Others
     * ==========
     */
    pub async fn recover(reject: Rejection) -> Result<(Response,), Rejection> {
        match reject.find::<Error>() {
            Some(e) => future::ok((response(e.status, &e.msg),)),
            None => future::err(reject),
        }
        .await
    }
}

use rego_err::Error as RegoError;

impl From<RegoError> for Error {
    fn from(e: RegoError) -> Self {
        match e {
            RegoError::BadInput { msg } => Error::new(StatusCode::BAD_REQUEST, msg),
            RegoError::AuthFailed => Error::new(
                StatusCode::UNAUTHORIZED,
                "authentication or authorization is failed",
            ),
            RegoError::NotFound { resource } => {
                Error::new(StatusCode::NOT_FOUND, format!("{} is not found", resource))
            }
            RegoError::Conflict { resource } => {
                Error::new(StatusCode::CONFLICT, format!("{} is conflict", resource))
            }
            RegoError::Internal(_) => {
                Error::new(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
            }
        }
    }
}

impl Reject for Error {}

impl Into<Rejection> for Error {
    fn into(self) -> Rejection {
        warp::reject::custom(self)
    }
}

impl From<anyhow::Error> for Error {
    fn from(_e: anyhow::Error) -> Error {
        Error::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
    }
}
