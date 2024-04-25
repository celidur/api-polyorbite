use std::{collections::HashSet, vec};

use ldap3::Mod;

use crate::common::password::{DEFAULT_HASH, Password};

use super::{User, UserAttribute};


pub struct ModifyUser {
    password: Option<String>,
    mail: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    name: Option<String>,
    school: Option<String>,
    genie: Option<String>,
    matricule: Option<String>,
    number: Option<String>,
    picture: Option<Vec<u8>>,
}


impl ModifyUser {
    pub fn new() -> Self {
        Self {
            password: None,
            mail: None,
            first_name: None,
            last_name: None,
            name: None,
            school: None,
            genie: None,
            matricule: None,
            number: None,
            picture: None,
        }
    }

    pub fn password(mut self, password: String) -> Self {
        self.password = Some(Password::hash(password.as_str(), DEFAULT_HASH));
        self
    }

    pub fn mail(mut self, mail: String) -> Self {
        self.mail = Some(mail);
        self
    }

    pub fn first_name(mut self, first_name: String) -> Self {
        self.first_name = Some(first_name);
        self
    }

    pub fn last_name(mut self, last_name: String) -> Self {
        self.last_name = Some(last_name);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn school(mut self, school: String) -> Self {
        self.school = Some(school);
        self
    }

    pub fn genie(mut self, genie: String) -> Self {
        self.genie = Some(genie);
        self
    }

    pub fn matricule(mut self, matricule: String) -> Self {
        self.matricule = Some(matricule);
        self
    }

    pub fn number(mut self, number: String) -> Self {
        self.number = Some(number);
        self
    }

    pub fn picture(mut self, picture: Vec<u8>) -> Self {
        self.picture = Some(picture);
        self
    }

    pub fn set_all(mut self, user: User) -> Self {
        self.password = Some(user.password);
        self.mail = Some(user.mail);
        self.first_name = Some(user.first_name);
        self.last_name = Some(user.last_name);
        self.name = Some(user.name);
        self.school = Some(user.school);
        self.genie = Some(user.genie);
        self.matricule = Some(user.matricule);
        self.number = Some(user.number);
        self.picture = user.picture;
        self
    }

    pub fn to_ldif(&self, user: User) -> (Vec<Mod<&str>>, Vec<Mod<&[u8]>>) {
        let mut ldif = Vec::new();

        if let Some(password) = &self.password {
            ldif.push(Mod::Replace(UserAttribute::Password.as_str(), HashSet::from([password.as_str()])));
        }

        if let Some(mail) = &self.mail {
            if user.mail == "" && mail != ""{
                ldif.push(Mod::Add(UserAttribute::Mail.as_str(), HashSet::from([mail.as_str()])));
            } else if mail == "" && user.mail != ""{
                ldif.push(Mod::Delete(UserAttribute::Mail.as_str(), HashSet::new()));
            } else if user.mail != *mail {
                ldif.push(Mod::Replace(UserAttribute::Mail.as_str(), HashSet::from([mail.as_str()])));
            }
        }

        if let Some(first_name) = &self.first_name {
            if user.first_name == "" && first_name != "" {
                ldif.push(Mod::Add(UserAttribute::FirstName.as_str(), HashSet::from([first_name.as_str()])));
            } else if first_name == "" && user.first_name != "" {
                ldif.push(Mod::Delete(UserAttribute::FirstName.as_str(), HashSet::new()));
            } else if user.first_name != *first_name {
                ldif.push(Mod::Replace(UserAttribute::FirstName.as_str(), HashSet::from([first_name.as_str()])));
            }
        }

        if let Some(last_name) = &self.last_name {
            if user.last_name == "" && last_name != "" {
                ldif.push(Mod::Add(UserAttribute::LastName.as_str(), HashSet::from([last_name.as_str()])));
            } else if last_name == "" && user.last_name != "" {
                ldif.push(Mod::Delete(UserAttribute::LastName.as_str(), HashSet::new()));
            } else if user.last_name != *last_name {
                ldif.push(Mod::Replace(UserAttribute::LastName.as_str(), HashSet::from([last_name.as_str()])));
            }
        }

        if let Some(school) = &self.school {
            if user.school == "" && school != "" {
                ldif.push(Mod::Add(UserAttribute::School.as_str(), HashSet::from([school.as_str()])));
            } else if school == "" && user.school != "" {
                ldif.push(Mod::Delete(UserAttribute::School.as_str(), HashSet::new()));
            } else if user.school != *school {
                ldif.push(Mod::Replace(UserAttribute::School.as_str(), HashSet::from([school.as_str()])));
            }
        }

        if let Some(genie) = &self.genie {
            if user.genie == "" && genie != "" {
                ldif.push(Mod::Add(UserAttribute::Genie.as_str(), HashSet::from([genie.as_str()])));
            } else if genie == "" && user.genie != "" {
                ldif.push(Mod::Delete(UserAttribute::Genie.as_str(), HashSet::new()));
            } else if user.genie != *genie {
                ldif.push(Mod::Replace(UserAttribute::Genie.as_str(), HashSet::from([genie.as_str()])));
            }
        }

        if let Some(matricule) = &self.matricule {
            if user.matricule == "" && matricule != "" {
                ldif.push(Mod::Add(UserAttribute::Matricule.as_str(), HashSet::from([matricule.as_str()])));
            } else if matricule == "" && user.matricule != "" {
                ldif.push(Mod::Delete(UserAttribute::Matricule.as_str(), HashSet::new()));
            } else if user.matricule != *matricule {
                ldif.push(Mod::Replace(UserAttribute::Matricule.as_str(), HashSet::from([matricule.as_str()])));
            }
        }

        if let Some(number) = &self.number {
            if user.number == "" && number != "" {
                ldif.push(Mod::Add(UserAttribute::Number.as_str(), HashSet::from([number.as_str()])));
            } else if number == "" && user.number != "" {
                ldif.push(Mod::Delete(UserAttribute::Number.as_str(), HashSet::new()));
            } else if user.number != *number {
                ldif.push(Mod::Replace(UserAttribute::Number.as_str(), HashSet::from([number.as_str()])));
            }
        }

        if let Some(name) = &self.name {
            if user.name == "" && name != "" {
                ldif.push(Mod::Add(UserAttribute::Name.as_str(), HashSet::from([name.as_str()])));
            } else if name == "" && user.name != "" {
                ldif.push(Mod::Delete(UserAttribute::Name.as_str(), HashSet::new()));
            } else if user.name != *name {
                ldif.push(Mod::Replace(UserAttribute::Name.as_str(), HashSet::from([name.as_str()])));
            }
        }

        let mut ldif2:Vec<Mod<&[u8]>> = vec![];

        if let Some(picture) = &self.picture {
            ldif2.push(Mod::Replace(UserAttribute::Picture.as_bytes(), HashSet::from([picture.as_slice()])));
        }

        (ldif, ldif2)
    }
}