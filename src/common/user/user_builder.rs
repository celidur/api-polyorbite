use crate::common::password::{Password, DEFAULT_HASH};

use super::User;

pub struct UserBuilder {
    user: User,
}

impl UserBuilder {
    pub fn new() -> Self {
        Self {
            user: {
                User {
                    uid: String::new(),
                    name: String::new(),
                    first_name: String::new(),
                    last_name: String::new(),
                    mail: String::new(),
                    school: String::new(),
                    genie: String::new(),
                    matricule: String::new(),
                    number: String::new(),
                    password: String::new(),
                    picture: None,
                    member: None,
                }
            }
        }
    }

    pub fn uid(mut self, uid: String) -> Self {
        self.user.uid = uid;
        self
    }

    pub fn password(mut self, password: String) -> Self {
        self.user.password = Password::hash(password.as_str(), DEFAULT_HASH);
        self
    }

    pub fn mail(mut self, mail: String) -> Self {
        self.user.mail = mail;
        self
    }

    pub fn first_name(mut self, first_name: String) -> Self {
        self.user.first_name = first_name;
        self
    }

    pub fn last_name(mut self, last_name: String) -> Self {
        self.user.last_name = last_name;
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.user.name = name;
        self
    }

    pub fn school(mut self, school: String) -> Self {
        self.user.school = school;
        self
    }

    pub fn genie(mut self, genie: String) -> Self {
        self.user.genie = genie;
        self
    }

    pub fn matricule(mut self, matricule: String) -> Self {
        self.user.matricule = matricule;
        self
    }

    pub fn number(mut self, number: String) -> Self {
        self.user.number = number;
        self
    }

    pub fn picture(mut self, picture: Vec<u8>) -> Self {
        self.user.picture = Some(picture);
        self
    }

    pub fn build(self) -> Result<User, &'static str> {
        if self.user.uid.is_empty() {
            return Err("uid is required");
        }
        if self.user.password.is_empty() {
            return Err("password is required");
        }
        if self.user.mail.is_empty() {
            return Err("mail is required");
        }
        if self.user.first_name.is_empty() {
            return Err("first_name is required");
        }
        if self.user.last_name.is_empty() {
            return Err("last_name is required");
        }
        if self.user.name.is_empty() {
            return Err("name is required");
        }
        Ok(self.user)
    }
}