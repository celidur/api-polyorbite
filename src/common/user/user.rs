use ldap3::SearchEntry;
use crate::common::password::Password;

use super::UserAttribute;

#[derive(Debug, Clone)]
pub struct User {
    pub uid: String,
    pub password: String,
    pub mail: String,
    pub first_name: String,
    pub last_name: String,
    pub name: String,
    pub school: String,
    pub genie: String,
    pub matricule: String,
    pub number: String,
    pub picture: Option<Vec<u8>>,
    pub member: Option<Vec<String>>,
}

impl User {
    pub fn new(entry: SearchEntry) -> Self {
        let mut password = String::new();
        let mut uid = String::new();
        let mut mail = String::new();
        let mut first_name = String::new();
        let mut last_name = String::new();
        let mut school = String::new();
        let mut genie = String::new();
        let mut matricule = String::new();
        let mut number = String::new();
        let mut name = String::new();
        let mut picture = None;
        let mut member = None;

        for (key, value) in entry.attrs {
            match UserAttribute::from_str(key.as_str()) {
                UserAttribute::Password => password = value[0].clone(),
                UserAttribute::Mail => mail = value[0].clone(),
                UserAttribute::FirstName => first_name = value[0].clone(),
                UserAttribute::Name => name = value[0].clone(),
                UserAttribute::LastName => last_name = value[0].clone(),
                UserAttribute::School => school = value[0].clone(),
                UserAttribute::Genie => genie = value[0].clone(),
                UserAttribute::Uid => uid = value[0].clone(),
                UserAttribute::Matricule => matricule = value[0].clone(),
                UserAttribute::Number => number = value[0].clone(),
                UserAttribute::MemberOf => member = Some(value.clone()),
                _ => {}
            }
        }

        for (key, value) in entry.bin_attrs {
            match UserAttribute::from_str(key.as_str()) {
                UserAttribute::Picture => picture = Some(value[0].clone()),
                _ => {}
            }
        }

        Self {
            password,
            mail,
            first_name,
            last_name,
            name,
            school,
            genie,
            uid,
            matricule,
            number,
            picture,
            member,
        }
    }

    pub fn verify_password(&self, password: &str) -> bool {
        Password::verify(password, self.password.as_str())
    }
}