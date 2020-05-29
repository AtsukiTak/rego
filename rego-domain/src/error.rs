use std::borrow::Cow;

#[derive(Error, Debug)]
pub enum Error {
    /// Error representing an **input** is at fault.
    #[error("violate domain invariance rule - {msg}")]
    BadInput { msg: Cow<'static, str> },

    /// Authentication or Authorization is failed.
    #[error("auth failed")]
    AuthFailed,

    /// Some resources are not found.
    #[error("{resource} is not found")]
    NotFound { resource: &'static str },

    /// SOme resources are conflict.
    #[error("{resource} is conflict")]
    Conflict { resource: &'static str },

    /// Error representing an **internal** is at fault.
    #[error(transparent)]
    Internal(anyhow::Error),
}

impl Error {
    pub fn bad_input<S>(s: S) -> Self
    where
        Cow<'static, str>: From<S>,
    {
        Error::BadInput { msg: s.into() }
    }

    pub fn auth_failed() -> Self {
        Error::AuthFailed
    }

    pub fn not_found(resource: &'static str) -> Self {
        Error::NotFound { resource }
    }

    pub fn conflict(resource: &'static str) -> Self {
        Error::Conflict { resource }
    }

    pub fn internal<E>(e: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Error::Internal(anyhow::Error::from(e))
    }
}
