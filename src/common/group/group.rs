use std::collections::HashSet;

use ldap3::SearchEntry;

#[derive(Debug, Clone)]
pub struct Group {
    pub dn: String,
    pub cn: String,
    pub parents: HashSet<String>,
    pub user_members: HashSet<String>,
    pub group_members: HashSet<String>,
    pub owner_user: HashSet<String>,
    pub owner_group: HashSet<String>,
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

        let mut user_members = HashSet::new();
        let mut group_members = HashSet::new();
        member.iter().for_each(|m| {
            if m.contains("uid") {
                let uid = m.split(",").next().unwrap().split("=").last().unwrap();
                user_members.insert(uid.to_string());
            } else {
                let cn = m.split(",").next().unwrap().split("=").last().unwrap();
                group_members.insert(cn.to_string());
            }
        });

        let owner_user: HashSet<String> = owner.iter().filter(|o| o.contains("uid")).map(|o| o.split(",").next().unwrap().split("=").last().unwrap().to_string()).collect();
        let owner_group: HashSet<String> = owner.iter().filter(|o| o.contains("cn")).map(|o| o.split(",").next().unwrap().split("=").last().unwrap().to_string()).collect();


        let reg = regex::Regex::new( r"cn=([^,]+)").unwrap();
        let mut parents: HashSet<String> = dn.split(",").map(|v|  reg.captures_iter(v).map(|c| c[1].to_string()).collect::<Vec<String>>()).flatten().collect();

        parents.remove(&cn);

        Self {
            dn,
            cn,
            user_members,
            group_members,
            owner_user,
            owner_group,
            parents,
        }
    }
}
