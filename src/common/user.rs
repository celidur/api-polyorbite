use ldap3::SearchEntry;

#[derive(Debug, Clone)]
pub struct User {
    pub password: String,
    pub mail: String,
    pub first_name: String,
    pub last_name: String,
    pub school: String,
    pub genie: String,
    pub uid: String,
    pub matricule: String,
    pub number: String,
    pub picture: Vec<u8>,
    pub member: Vec<String>,
}

impl User {
    pub fn new(entry: SearchEntry) -> Self {
        let mut password = String::new();
        let mut mail = String::new();
        let mut first_name = String::new();
        let mut last_name = String::new();
        let mut school = String::new();
        let mut genie = String::new();
        let mut uid = String::new();
        let mut matricule = String::new();
        let mut number = String::new();
        let mut picture = vec![];
        let mut member = vec![];

        for (key, value) in entry.attrs {
            match key.as_str() {
                "userPassword" => password = value[0].clone(),
                "mail" => mail = value[0].clone(),
                "givenName" => first_name = value[0].clone(),
                "sn" => last_name = value[0].clone(),
                "school" => school = value[0].clone(),
                "genie" => genie = value[0].clone(),
                "uid" => uid = value[0].clone(),
                "matricule" => matricule = value[0].clone(),
                "telephoneNumber" => number = value[0].clone(),
                "memberOf" => member = value.clone(),
                _ => {}
            }
        }

        for (key, value) in entry.bin_attrs {
            if key.as_str() == "jpegPhoto" {
                picture = value[0].clone();
            }
        }

        Self {
            password,
            mail,
            first_name,
            last_name,
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
