#[cfg(feature = "rego-app")]
pub use rego_app as app;

#[cfg(feature = "rego-infra")]
pub use rego_infra as infra;

#[cfg(feature = "rego-domain")]
pub use rego_domain as domain;
