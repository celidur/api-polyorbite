use super::ssha;
use super::hash_type::Hash;

pub struct Password;

impl Password {
    pub fn hash(password: &str, hash: Hash) -> String {
        match hash {
            Hash::SSHA => ssha::LdapSaltedSha1::hash(password),
        }
    }

    pub fn verify(password: &str, hash: &str) -> bool {
        let reg = regex::Regex::new(r"\{(?P<code>.*?)\}").unwrap();
        let captures = reg.captures(hash).ok_or("Invalid hash format");
        if captures.is_err() {
            return password == hash;
        }
        let captures = captures.unwrap();
        let hash_type = Hash::from_str(captures.name("code").unwrap().as_str());
        match hash_type {
            Hash::SSHA => ssha::LdapSaltedSha1::verify(password, hash),
        } 
    }
}



