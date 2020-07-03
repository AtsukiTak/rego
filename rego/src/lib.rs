#[cfg(feature = "rego-app")]
pub use rego_app as app;

#[cfg(feature = "rego-infra")]
pub use rego_infra as infra;

#[cfg(feature = "rego-domain")]
pub use rego_domain as domain;

#[cfg(feature = "rego-err")]
pub use rego_err as error;

pub use error::Error;
