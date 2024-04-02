use crate::algorithms::Algorithm;
use hex::encode;
use sha2::{Digest, Sha256, Sha384, Sha512};
use sha3::{Sha3_256, Sha3_384, Sha3_512};
use std::fmt;

fn hash_with_algorithm<D: Digest>(data: &[u8], mut hasher: D) -> String {
    hasher.update(data);
    encode(hasher.finalize())
}

pub fn calculate_hash(buffer: &[u8], algorithm: Algorithm) -> Result<String, HasherError> {
    let hasher = match algorithm {
        Algorithm::Sha2_256 => hash_with_algorithm::<Sha256>(buffer, Sha256::new()),
        Algorithm::Sha2_384 => hash_with_algorithm::<Sha384>(buffer, Sha384::new()),
        Algorithm::Sha2_512 => hash_with_algorithm::<Sha512>(buffer, Sha512::new()),
        Algorithm::Sha3_256 => hash_with_algorithm::<Sha3_256>(buffer, Sha3_256::new()),
        Algorithm::Sha3_384 => hash_with_algorithm::<Sha3_384>(buffer, Sha3_384::new()),
        Algorithm::Sha3_512 => hash_with_algorithm::<Sha3_512>(buffer, Sha3_512::new()),
        Algorithm::Md5 => hash_with_algorithm::<md5::Md5>(buffer, md5::Md5::new()),
    };
    Ok(hasher)
}

#[derive(Debug)]
pub enum HasherError {
    IOError(std::io::Error),
}

impl fmt::Display for HasherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HasherError::IOError(ref err) => write!(f, "IO error: {}", err),
        }
    }
}

impl From<std::io::Error> for HasherError {
    fn from(err: std::io::Error) -> HasherError {
        HasherError::IOError(err)
    }
}
