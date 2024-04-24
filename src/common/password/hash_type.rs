pub enum Hash {
    SSHA,
}

impl Hash {
    pub fn from_str(hash: &str) -> Self {
        match hash {
            "SSHA" => Hash::SSHA,
            _ => panic!("Unsupported hash type"),
        }
    }
}