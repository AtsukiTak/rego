use http::StatusCode;
use warp::reply;

/*
 * ========
 * Response
 * ========
 */
pub type Response = reply::WithStatus<reply::Json>;

pub fn response_ok<T>(json: &T) -> Response
where
    T: serde::Serialize,
{
    response(StatusCode::OK, json)
}

pub fn response<T>(status: StatusCode, json: &T) -> Response
where
    T: serde::Serialize,
{
    reply::with_status(reply::json(json), status)
}
