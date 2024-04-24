use base64::prelude::*;
use regex::Regex;
use sha1::{Sha1, Digest};
use std::str;


pub struct LdapSaltedSha1;

impl LdapSaltedSha1 {
    fn generate_salt() -> Vec<u8> {
        let size = 4;
        
        let mut salt = vec![0; size];
        salt.iter_mut().for_each(|x| *x = rand::random::<u8>());
        salt
    }

    fn calc_checksum(secret: &[u8], salt: &[u8]) -> Vec<u8> {
        let mut hasher = Sha1::new();
        hasher.update(secret);
        hasher.update(salt);
        let hash_result: Vec<u8> = hasher.finalize().to_vec();
        hash_result
    }

    pub fn hash(secret: &str) -> String {
        let salt = LdapSaltedSha1::generate_salt();
        let checksum = LdapSaltedSha1::calc_checksum(secret.as_bytes(), &salt);
        let data = [checksum, salt].concat();

        let encoded_data = BASE64_STANDARD.encode(data);
        format!("{}{}", "{SSHA}", encoded_data)
    }

    pub fn verify(secret: &str, hash: &str) -> bool {
        let hash_regex = Regex::new(r"^\{SSHA\}(?P<tmp>[+/a-zA-Z0-9]{32,}={0,2})$").unwrap();
        let captures = hash_regex.captures(hash).ok_or("Invalid hash format").unwrap();
        let encoded_salt = captures.name("tmp").unwrap().as_str();
        let data = BASE64_STANDARD.decode(encoded_salt).map_err(|_| "Invalid base64 encoding").unwrap();
        let checksum = &data[..20];
        let salt = &data[20..];

        let new_checksum = LdapSaltedSha1::calc_checksum(secret.as_bytes(), salt);
        new_checksum == checksum
    }
}