#[derive(Clone, Copy)]
pub enum Algorithm {
    Md5,
    Sha2_256,
    Sha2_384,
    Sha2_512,
    Sha3_256,
    Sha3_384,
    Sha3_512,
}

impl Algorithm {
    pub fn as_str(&self) -> &'static str {
        match self {
            Algorithm::Md5 => "md5",
            Algorithm::Sha2_256 => "Sha2-256",
            Algorithm::Sha2_384 => "Sha2-384",
            Algorithm::Sha2_512 => "Sha2-512",
            Algorithm::Sha3_256 => "Sha3-256",
            Algorithm::Sha3_384 => "Sha3-384",
            Algorithm::Sha3_512 => "Sha3-512",
        }
    }
}
