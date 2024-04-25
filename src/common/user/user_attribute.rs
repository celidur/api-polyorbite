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

    pub fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}