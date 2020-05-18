use crate::app::err::Error;
use crate::domain::access_token::{AccessToken, JwtEncoder};
use futures::future;
use std::str::FromStr;
use warp::{filters::header::header, Filter, Rejection};

pub fn auth(
    jwt_encoder: JwtEncoder,
) -> impl Filter<Extract = (AccessToken,), Error = Rejection> + Clone {
    (header::<BearerToken>("Authorization")
        .or(header::<BearerToken>("authorization"))
        .unify())
    .and_then(
        move |BearerToken(token)| match jwt_encoder.decode(token.as_str()) {
            Ok(token) => future::ok(token),
            Err(_) => future::err(Into::<Rejection>::into(Error::unauthorized())),
        },
    )
    .or_else(|_| future::err(Into::<Rejection>::into(Error::unauthorized())))
}

pub struct BearerToken(String);

impl FromStr for BearerToken {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        if s.len() < 7 {
            return Err(anyhow::anyhow!("Not a BearerToken"));
        }
        let (bearer, token) = s.split_at(7);
        if bearer != "Bearer " {
            return Err(anyhow::anyhow!("Not a BearerToken"));
        }
        Ok(BearerToken(token.to_string()))
    }
}
