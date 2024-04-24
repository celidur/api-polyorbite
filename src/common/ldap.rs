use crate::common::{group::Group, user::UserAttribute};

use super::user::{ModifyUser, User};
use std::{collections::HashMap, env, vec};

use ldap3::{LdapConnAsync, Scope};

#[derive(Debug)]
pub struct Ldap {
    url: String,
    user: String,
    password: String,
    base_dn: String,
    users_base_dn: String,
    groups_base_dn: String,
    users: HashMap<String, User>,
    groups: HashMap<String, Group>,
}

impl Ldap {
    pub async fn new() -> Result<Self, &'static str> {
        let user = env::var("BIND_USER").expect("BIND_USER must be set");
        let password = env::var("BIND_PASSWORD").expect("BIND_PASSWORD must be set");
        let host = env::var("LDAP_SERVER").expect("LDAP_SERVER must be set");
        let port = env::var("LDAP_PORT")
            .expect("LDAP_PORT must be set")
            .parse::<u16>();
        let base_dn = env::var("LDAP_BASE").expect("LDAP_BASE must be set");
        let users_base_dn = env::var("LDAP_USERS_BASE").expect("LDAP_USERS_BASE must be set");
        let groups_base_dn = env::var("LDAP_GROUPS_BASE").expect("LDAP_GROUPS_BASE must be set");

        if port.is_err() {
            return Err("LDAP_PORT must be a valid port number");
        }

        let port = port.unwrap();

        let url = format!("{}:{}", host, port);

        Ok(Self {
            url,
            user,
            password,
            base_dn,
            users_base_dn,
            groups_base_dn,
            users: HashMap::new(),
            groups: HashMap::new(),
        })
    }

    pub async fn test_connection(&self) -> ldap3::result::Result<bool> {

        let (conn, mut ldap) = LdapConnAsync::new(self.url.as_str()).await?;
        ldap3::drive!(conn);

        let result = ldap
            .simple_bind(self.user.as_str(), self.password.as_str())
            .await?
            .success();
        ldap.unbind().await?;

        if result.is_err() {
            return Ok(false);
        }
        Ok(true)
    }

    pub async fn update_user_info_by_id(&mut self, id: &str) -> ldap3::result::Result<()> {
        let (conn, mut ldap) = LdapConnAsync::new(self.url.as_str()).await?;
        ldap3::drive!(conn);

        ldap.simple_bind(self.user.as_str(), self.password.as_str())
            .await?
            .success()?;

        let filter = format!("(&(objectClass=inetOrgPerson)(uid={}))", id);

        let (rs, _res) = ldap
            .search(self.base_dn.as_str(), Scope::Subtree, filter.as_str(), vec!["*", "memberOf"])
            .await?
            .success()?;

        ldap.unbind().await?;

        if rs.is_empty() {
            self.users.remove(id);
            return Ok(());
        }

        let entry = rs.first().unwrap();
        let user = User::new(ldap3::SearchEntry::construct(entry.clone()));
        self.users.insert(user.uid.clone(), user);
        Ok(())
    }

    pub async fn update_users(&mut self) -> ldap3::result::Result<()> {
        let (conn, mut ldap) = LdapConnAsync::new(self.url.as_str()).await?;
        ldap3::drive!(conn);

        ldap.simple_bind(self.user.as_str(), self.password.as_str())
            .await?
            .success()?;

        let filter = "(objectClass=inetOrgPerson)";

        let (rs, _res) = ldap
            .search(self.base_dn.as_str(), Scope::Subtree, filter, vec!["*", "memberOf"])
            .await?
            .success()?;

        ldap.unbind().await?;

        for entry in rs {
            let user = User::new(ldap3::SearchEntry::construct(entry));
            self.users.insert(user.uid.clone(), user);
        }
        Ok(())
    }

    pub async fn get_user_by_id(&self, id: &str) -> Option<&User> {
        self.users.get(id)
    }

    pub async fn update_groups(&mut self) -> ldap3::result::Result<()> {
        let (conn, mut ldap) = LdapConnAsync::new(self.url.as_str()).await?;
        ldap3::drive!(conn);

        ldap.simple_bind(self.user.as_str(), self.password.as_str())
            .await?
            .success()?;

        let filter = "(objectClass=groupOfNames)";

        let (rs, _res) = ldap
            .search(self.base_dn.as_str(), Scope::Subtree, filter, vec!["*"])
            .await?
            .success()?;

        ldap.unbind().await?;

        for entry in rs {
            let group = Group::new(ldap3::SearchEntry::construct(entry));
            self.groups.insert(group.cn.clone(), group);
        }
        Ok(())
    }

    pub async fn update_group_info_by_id(&mut self, id: &str) -> ldap3::result::Result<()> {
        let (conn, mut ldap) = LdapConnAsync::new(self.url.as_str()).await?;
        ldap3::drive!(conn);

        ldap.simple_bind(self.user.as_str(), self.password.as_str())
            .await?
            .success()?;

        let filter = format!("(&(objectClass=groupOfNames)(cn={}))", id);

        let (rs, _res) = ldap
            .search(self.base_dn.as_str(), Scope::Subtree, filter.as_str(), vec!["*"])
            .await?
            .success()?;

        ldap.unbind().await?;

        if rs.is_empty() {
            self.groups.remove(id);
            return Ok(());
        }

        let entry = rs.first().unwrap();
        let group = Group::new(ldap3::SearchEntry::construct(entry.clone()));
        self.groups.insert(group.cn.clone(), group);
        Ok(())
    }

    pub async fn get_group_by_id(&self, id: &str) -> Option<&Group> {
        self.groups.get(id)
    }

    pub async fn get_groups(&self) -> Vec<&Group> {
        self.groups.values().collect()
    }

    pub async fn get_users(&self) -> Vec<&User> {
        self.users.values().collect()
    }

    pub async fn modify_user(&mut self, id: &str, modification: ModifyUser) -> ldap3::result::Result<bool>{
        self.update_user_info_by_id(id).await?;

        let (conn, mut ldap) = LdapConnAsync::new(self.url.as_str()).await?;
        ldap3::drive!(conn);

        ldap.simple_bind(self.user.as_str(), self.password.as_str())
            .await?
            .success()?;

        let user = self.get_user_by_id(id).await;

        if user.is_none() {
            return Ok(false);
        }
    
        let changes = modification.to_ldif(user.unwrap());

        if changes.is_empty() {
            return Ok(false);
        }

        let dn = format!("uid={},{}", id, self.users_base_dn);

        let result = ldap
            .modify(dn.as_str(), changes)
            .await?
            .success();

        ldap.unbind().await?;


        self.update_user_info_by_id(id).await?;

        if result.is_err() {
            return Ok(false);
        }

        Ok(true)
    }


    pub async fn delete_user(&mut self, id: &str) -> ldap3::result::Result<bool> {
        let (conn, mut ldap) = LdapConnAsync::new(self.url.as_str()).await?;
        ldap3::drive!(conn);

        ldap.simple_bind(self.user.as_str(), self.password.as_str())
            .await?
            .success()?;

        let dn = format!("uid={},{}", id, self.users_base_dn);

        let result = ldap
            .delete(dn.as_str())
            .await?
            .success();

        ldap.unbind().await?;

        self.update_user_info_by_id(id).await?;

        if result.is_err() {
            return Ok(false);
        }

        Ok(true)
    }

    pub async fn new_user(&mut self, user: User) -> ldap3::result::Result<bool> {
        if self.get_user_by_id(user.uid.as_str()).await.is_some() {
            return Ok(false);
        }

        Ok(true)
    }
}
