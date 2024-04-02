use hex::encode;
use sha2::{Digest, Sha256, Sha384, Sha512};
use sha3::{Sha3_256, Sha3_384, Sha3_512};
use std::fmt;

pub fn calculate_hash(buffer: &[u8], algorithm: &str) -> Result<String, HasherError> {
    match algorithm {
        "sha2-256" => Ok(encode(Sha256::digest(buffer))),
        "sha2-384" => Ok(encode(Sha384::digest(buffer))),
        "sha2-512" => Ok(encode(Sha512::digest(buffer))),
        "sha3-256" => Ok(encode(Sha3_256::digest(buffer))),
        "sha3-384" => Ok(encode(Sha3_384::digest(buffer))),
        "sha3-512" => Ok(encode(Sha3_512::digest(buffer))),
        "md5" => Ok(encode(md5::compute(buffer).as_slice())),
        _ => Err(HasherError::UnsupportedAlgorithm),
    }
}

#[derive(Debug)]
pub enum HasherError {
    IOError(std::io::Error),
    UnsupportedAlgorithm,
}

impl fmt::Display for HasherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HasherError::IOError(ref err) => write!(f, "IO error: {}", err),
            HasherError::UnsupportedAlgorithm => write!(f, "Unsupported hash algorithm"),
        }
    }
}

impl From<std::io::Error> for HasherError {
    fn from(err: std::io::Error) -> HasherError {
        HasherError::IOError(err)
    }
}
