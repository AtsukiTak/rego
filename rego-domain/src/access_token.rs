use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json::value::RawValue;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessToken {
    pub body: Box<RawValue>,
    pub body_type: String,
    pub exp: usize,
}

pub trait AccessTokenBody: Serialize + DeserializeOwned {
    fn type_name() -> &'static str {
        std::any::type_name::<Self>()
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("body type is not {0}")]
    BodyTypeMismatch(&'static str),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::error::Error),
    #[error(transparent)]
    Encoder(#[from] jsonwebtoken::errors::Error),
}

impl AccessToken {
    pub fn new<B>(body: &B, valid_dur: Duration) -> Result<Self, Error>
    where
        B: AccessTokenBody,
    {
        let exp = (Utc::now() + valid_dur).timestamp() as usize;

        let serialized_body = serde_json::to_string(body)?;
        let raw_val = RawValue::from_string(serialized_body)?;

        Ok(AccessToken {
            body: raw_val,
            body_type: std::any::type_name::<B>().to_string(),
            exp,
        })
    }

    pub fn get_body<B>(&self) -> Result<B, Error>
    where
        B: AccessTokenBody,
    {
        if self.body_type != std::any::type_name::<B>() {
            return Err(Error::BodyTypeMismatch(std::any::type_name::<B>()));
        }

        Ok(serde_json::from_str::<B>(self.body.get())?)
    }
}

#[derive(Debug, Clone)]
pub struct JwtEncoder {
    inner: Arc<Inner>,
}

impl JwtEncoder {
    pub fn from_secret(secret: &[u8]) -> JwtEncoder {
        JwtEncoder {
            inner: Arc::new(Inner::from_secret(secret)),
        }
    }

    pub fn encode(&self, token: &AccessToken) -> Result<String, Error> {
        self.inner.encode(token)
    }

    pub fn decode(&self, token: &str) -> Result<AccessToken, Error> {
        self.inner.decode(token)
    }
}

#[derive(Debug)]
struct Inner {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey<'static>,
}

impl Inner {
    fn from_secret(secret: &[u8]) -> Inner {
        Inner {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret).into_static(),
        }
    }

    fn encode(&self, token: &AccessToken) -> Result<String, Error> {
        Ok(jsonwebtoken::encode(
            &Header::default(),
            token,
            &self.encoding_key,
        )?)
    }

    fn decode(&self, token: &str) -> Result<AccessToken, Error> {
        let res =
            jsonwebtoken::decode::<AccessToken>(token, &self.decoding_key, &Validation::default())?;
        Ok(res.claims)
    }
}
