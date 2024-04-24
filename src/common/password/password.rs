use super::ssha;
use super::hash_type::Hash;

pub struct Password;

impl Password {
    pub fn hash(password: &str, hash: Hash) -> String {
        match hash {
            Hash::SSHA => ssha::LdapSaltedSha1::hash(password),
            _ => panic!("Unsupported hash type"),
        }
    }

    pub fn verify(password: &str, hash: &str) -> bool {
        let reg = regex::Regex::new(r"\{(?P<code>.*?)\}").unwrap();
        let captures = reg.captures(hash).ok_or("Invalid hash format").unwrap();
        let hash_type = Hash::from_str(captures.name("code").unwrap().as_str());
        match hash_type {
            Hash::SSHA => ssha::LdapSaltedSha1::verify(password, hash),
            _ => panic!("Unsupported hash type"),
        } 
    }
}



