use ldap3::SearchEntry;

#[derive(Debug, Clone)]
pub struct Group {
    pub dn: String,
    pub cn: String,
    pub user_members: Vec<String>,
    pub group_members: Vec<String>,
    pub owner: Vec<String>
}

impl Group {
    pub fn new(entry: SearchEntry) -> Self {
        let mut dn = String::new();
        let mut cn = String::new();
        let mut member = vec![];
        let mut owner = vec![];

        dn = entry.dn.clone();

        for (key, value) in entry.attrs {
            match key.as_str() {
                "cn" => cn = value[0].clone(),
                "member" => member = value.clone(),
                "owner" => owner = value.clone(),
                _ => {}
            }
        }
        let owner: Vec<String> = owner.iter().map(|o| o.split(",").next().unwrap().split("=").last().unwrap().to_string()).collect();

        let mut user_members = vec![];
        let mut group_members = vec![];

        member.iter().for_each(|m| {
            if m.contains("uid") {
                let uid = m.split(",").next().unwrap().split("=").last().unwrap();
                user_members.push(uid.to_string());
            } else {
                let cn = m.split(",").next().unwrap().split("=").last().unwrap();
                group_members.push(cn.to_string());
            }
        });

        Self {
            dn,
            cn,
            user_members,
            group_members,
            owner
        }
    }
}
