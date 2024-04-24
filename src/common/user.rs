use std::collections::HashSet;

use ldap3::{Mod, SearchEntry};

pub enum UserAttribute {
    Password,
    Mail,
    FirstName,
    LastName,
    Name,
    School,
    Genie,
    Matricule,
    Number,
    Picture,
    Uid,
    MemberOf,
    None,
}

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
        super::password::Password::verify(password, self.password.as_str())
    }
}

impl UserAttribute {
    pub fn from_str(s: &str) -> Self {
        match s {
            "userPassword" => Self::Password,
            "mail" => Self::Mail,
            "givenName" => Self::FirstName,
            "cn" => Self::Name,
            "sn" => Self::LastName,
            "departmentNumber" => Self::School,
            "roomNumber" => Self::Genie,
            "uid" => Self::Uid,
            "employeeNumber" => Self::Matricule,
            "telephoneNumber" => Self::Number,
            "memberOf" => Self::MemberOf,
            "jpegPhoto" => Self::Picture,
            _ => Self::None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Password => "userPassword",
            Self::Mail => "mail",
            Self::FirstName => "givenName",
            Self::Name => "cn",
            Self::LastName => "sn",
            Self::School => "departmentNumber",
            Self::Genie => "roomNumber",
            Self::Uid => "uid",
            Self::Matricule => "employeeNumber",
            Self::Number => "telephoneNumber",
            Self::MemberOf => "memberOf",
            Self::Picture => "jpegPhoto",
            Self::None => "",
        }
    }   
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
        self.password = Some(password);
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

    pub fn to_ldif(&self, user: &User) -> (Vec<Mod<&str>>) {
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

        // let ldif2: Vec<Mod<Vec<u8>>> = vec![];

        if let Some(_picture) = &self.picture {
            todo!("Add picture modification");
        }

        ldif
    }
}