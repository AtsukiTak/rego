use crate::Error;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json::value::RawValue;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebToken {
    pub body: Box<RawValue>,
    pub body_type: String,
    pub exp: usize,
}

pub trait AccessTokenBody: Serialize + DeserializeOwned {
    fn type_name() -> &'static str {
        std::any::type_name::<Self>()
    }
}

impl WebToken {
    pub fn new<B>(body: &B, valid_dur: Duration) -> Result<Self, Error>
    where
        B: AccessTokenBody,
    {
        let exp = (Utc::now() + valid_dur).timestamp() as usize;

        let serialized_body = serde_json::to_string(body).map_err(Error::internal)?;
        let raw_val = RawValue::from_string(serialized_body).map_err(Error::internal)?;

        Ok(WebToken {
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
            return Err(Error::bad_input(format!(
                "body type is not {}",
                std::any::type_name::<B>()
            )));
        }

        Ok(serde_json::from_str::<B>(self.body.get()).map_err(Error::internal)?)
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

    pub fn encode(&self, token: &WebToken) -> Result<String, Error> {
        self.inner.encode(token)
    }

    pub fn decode(&self, token: &str) -> Result<WebToken, Error> {
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

    fn encode(&self, token: &WebToken) -> Result<String, Error> {
        jsonwebtoken::encode(&Header::default(), token, &self.encoding_key).map_err(Error::internal)
    }

    fn decode(&self, token: &str) -> Result<WebToken, Error> {
        let res =
            jsonwebtoken::decode::<WebToken>(token, &self.decoding_key, &Validation::default())
                .map_err(Error::internal)?;
        Ok(res.claims)
    }
}
