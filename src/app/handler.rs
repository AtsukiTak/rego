use crate::app::{err::Error, res::Response};
use futures::future::{TryFuture, TryFutureExt};
use warp::reject::Rejection;

/*
 * ========
 * Handler
 * ========
 */
pub async fn handler_fn<F, Fut>(func: F) -> Result<Response, Rejection>
where
    F: FnOnce() -> Fut,
    Fut: TryFuture<Ok = Response, Error = Error>,
{
    func().err_into().await
}
