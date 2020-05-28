use std::fmt;

const N_ITER: u32 = 1_000;

#[derive(Serialize, Deserialize, Clone)]
#[serde(transparent)]
pub struct Cred {
    cred: String,
}

impl fmt::Debug for Cred {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Cred {{ ... }}")
    }
}

impl Cred {
    pub fn derive(secret: &str) -> Result<Cred, std::io::Error> {
        let cred = pbkdf2::pbkdf2_simple(secret, N_ITER)?;
        Ok(Cred { cred })
    }

    pub fn verify(&self, attempt: &str) -> Result<(), pbkdf2::CheckError> {
        pbkdf2::pbkdf2_check(attempt, self.cred.as_str())
    }

    pub fn as_str(&self) -> &str {
        self.cred.as_str()
    }
}

impl From<String> for Cred {
    fn from(s: String) -> Self {
        Cred { cred: s }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_and_verify() {
        let pass = "hogehoge";

        let cred = Cred::derive(pass).unwrap();
        assert!(cred.verify(pass).is_ok());
        assert!(cred.verify("invalid").is_err());
    }
}
