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
            Algorithm::Sha2_256 => "sha2-256",
            Algorithm::Sha2_384 => "sha2-384",
            Algorithm::Sha2_512 => "sha2-512",
            Algorithm::Sha3_256 => "sha3-256",
            Algorithm::Sha3_384 => "sha3-384",
            Algorithm::Sha3_512 => "sha3-512",
        }
    }

    // pub fn from_str(s: &str) -> Option<Self> {
    //     match s {
    //         "md5" => Some(Algorithm::Md5),
    //         "sha2-256" => Some(Algorithm::Sha2_256),
    //         "sha2-384" => Some(Algorithm::Sha2_384),
    //         "sha2-512" => Some(Algorithm::Sha2_512),
    //         "sha3-256" => Some(Algorithm::Sha3_256),
    //         "sha3-384" => Some(Algorithm::Sha3_384),
    //         "sha3-512" => Some(Algorithm::Sha3_512),
    //         _ => None,
    //     }
    // }
}
