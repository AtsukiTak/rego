use crate::res::{response, Response};
use futures::future;
use http::StatusCode;
use warp::reject::{Reject, Rejection};

#[derive(Debug, Clone)]
pub struct Error {
    pub status: StatusCode,
    pub msg: &'static str,
}

impl Error {
    pub fn new(status: StatusCode, msg: &'static str) -> Self {
        Error { status, msg }
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

    pub fn internal_server_error(msg: &'static str) -> Self {
        Error::new(StatusCode::INTERNAL_SERVER_ERROR, msg)
    }

    pub async fn recover(reject: Rejection) -> Result<(Response,), Rejection> {
        match reject.find::<Error>() {
            Some(e) => future::ok((response(e.status, &e.msg),)),
            None => future::err(reject),
        }
        .await
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
