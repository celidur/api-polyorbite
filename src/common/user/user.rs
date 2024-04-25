use std::collections::HashSet;

use ldap3::SearchEntry;
use crate::common::password::Password;

use super::UserAttribute;

#[derive(Debug, Clone)]
pub struct User {
    pub(super) uid: String,
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
                UserAttribute::Picture => picture = Some(value[0].clone().into_bytes()),
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

    pub fn to_ldif(&self) -> Vec<(&str, HashSet<&str>)> {
        let mut ldif = Vec::new();

        ldif.push((UserAttribute::Password.as_str(), HashSet::from([self.password.as_str()])));
        ldif.push((UserAttribute::Mail.as_str(), HashSet::from([self.mail.as_str()])));
        ldif.push((UserAttribute::FirstName.as_str(), HashSet::from([self.first_name.as_str()])));
        ldif.push((UserAttribute::LastName.as_str(), HashSet::from([self.last_name.as_str()])));
        ldif.push((UserAttribute::Name.as_str(), HashSet::from([self.name.as_str()])));
        ldif.push((UserAttribute::School.as_str(), HashSet::from([self.school.as_str()])));
        ldif.push((UserAttribute::Genie.as_str(), HashSet::from([self.genie.as_str()])));
        ldif.push((UserAttribute::Uid.as_str(), HashSet::from([self.uid.as_str()])));
        ldif.push((UserAttribute::Matricule.as_str(), HashSet::from([self.matricule.as_str()])));
        ldif.push((UserAttribute::Number.as_str(), HashSet::from([self.number.as_str()])));
        ldif.push(("objectClass", HashSet::from(["inetOrgPerson"])));
        let ldif = ldif.into_iter().filter(|(_, v)| !v.contains("")).collect();

        ldif
    }
}