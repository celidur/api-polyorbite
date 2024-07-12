use std::{collections::HashMap, sync::Arc};

use ldap3::{LdapConnAsync, Scope};
use tokio::sync::Mutex;

use super::{ModifyUser, User};

#[derive(Debug)]
pub struct Users {
    users: Arc<Mutex<HashMap<String, User>>>,
    ldap_url: String,
    ldap_user: String,
    ldap_password: String,
    users_base_dn: String,
    base_dn: String,
}


impl Users {
    pub fn new(ldap_url: String, ldap_user: String, ldap_password: String, users_base_dn: String, base_dn: String) -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
            ldap_url,
            ldap_user,
            ldap_password,
            users_base_dn,
            base_dn,
        }
    }

    async fn update_user(&mut self, id: &str) -> ldap3::result::Result<()> {
        let (conn, mut ldap) = LdapConnAsync::new(self.ldap_url.as_str()).await?;
        ldap3::drive!(conn);

        ldap.simple_bind(self.ldap_user.as_str(), self.ldap_password.as_str())
            .await?
            .success()?;

        let filter = format!("(&(objectClass=inetOrgPerson)(uid={}))", id);

        let (rs, _res) = ldap
            .search(self.base_dn.as_str(), Scope::Subtree, filter.as_str(), vec!["*", "memberOf"])
            .await?
            .success()?;

        ldap.unbind().await?;

        if rs.is_empty() {
            self.users.lock().await.remove(id);
            return Ok(());
        }

        let entry = rs.first().unwrap();
        let user = User::new(ldap3::SearchEntry::construct(entry.clone()));
        self.users.lock().await.insert(user.uid.clone(), user);
        Ok(())
    }

    pub async fn update(&mut self) -> ldap3::result::Result<()> {
        let (conn, mut ldap) = LdapConnAsync::new(self.ldap_url.as_str()).await?;
        ldap3::drive!(conn);

        ldap.simple_bind(self.ldap_user.as_str(), self.ldap_password.as_str())
            .await?
            .success()?;

        let filter = "(objectClass=inetOrgPerson)";

        let (rs, _res) = ldap
            .search(self.base_dn.as_str(), Scope::Subtree, filter, vec!["*", "memberOf"])
            .await?
            .success()?;

        ldap.unbind().await?;

        let mut users = self.users.lock().await;
        users.clear();
        for entry in rs {
            let user = User::new(ldap3::SearchEntry::construct(entry));
            users.insert(user.uid.clone(), user);
        }
        Ok(())
    }

    pub async fn user(&self, id: &str) -> Option<User> {
        self.users.lock().await.get(id).map(|u| u.clone())
    }

    pub async fn to_vec(&self) -> Vec<User> {
        self.users.lock().await.values().map(|u| u.clone()).collect()
    }

    pub async fn modify_user(&mut self, id: &str, modification: ModifyUser) -> ldap3::result::Result<bool>{
        self.update_user(id).await?;

        let (conn, mut ldap) = LdapConnAsync::new(self.ldap_url.as_str()).await?;
        ldap3::drive!(conn);

        ldap.simple_bind(self.ldap_user.as_str(), self.ldap_password.as_str())
            .await?
            .success()?;

        let user = self.user(id).await;

        if user.is_none() {
            return Ok(false);
        }
    
        let (changes1, changes2) = modification.to_ldif(user.unwrap());

        let dn = format!("uid={},{}", id, self.users_base_dn);

        if changes1.is_empty() && changes2.is_empty() {
            return Ok(false);
        }


        if !changes1.is_empty(){
            let result = ldap
                .modify(dn.as_str(), changes1)
                .await?
                .success();

            if result.is_err() {
                ldap.unbind().await?;
                return Ok(false);
            }

        }

        if !changes2.is_empty(){
            let result = ldap
                .modify(dn.as_str(), changes2)
                .await?
                .success();

            if result.is_err() {
                ldap.unbind().await?;
                println!("{:?}", result.err().unwrap());
                return Ok(false);
            }

        }

        ldap.unbind().await?;
        self.update_user(id).await?;

        Ok(true)
    }


    pub async fn delete_user(&mut self, id: &str) -> ldap3::result::Result<bool> {
        let (conn, mut ldap) = LdapConnAsync::new(self.ldap_url.as_str()).await?;
        ldap3::drive!(conn);

        ldap.simple_bind(self.ldap_user.as_str(), self.ldap_password.as_str())
            .await?
            .success()?;

        let dn = format!("uid={},{}", id, self.users_base_dn);

        let result = ldap
            .delete(dn.as_str())
            .await?
            .success();

        ldap.unbind().await?;

        self.update_user(id).await?;

        if result.is_err() {
            return Ok(false);
        }

        Ok(true)
    }

    pub async fn new_user(&mut self, user: User) -> ldap3::result::Result<bool> {
        if self.user(user.uid.as_str()).await.is_some() {
            return Ok(false);
        }

        let (conn, mut ldap) = LdapConnAsync::new(self.ldap_url.as_str()).await?;
        ldap3::drive!(conn);

        ldap.simple_bind(self.ldap_user.as_str(), self.ldap_password.as_str())
            .await?
            .success()?;

        let dn = format!("uid={},{}", user.uid, self.users_base_dn);

        let result = ldap
            .add(dn.as_str(), user.to_ldif())
            .await?
            .success();

        ldap.unbind().await?;

        if result.is_err() {
            return Ok(false);
        }

        self.update_user(user.uid.as_str()).await?;

        if user.picture.is_some() {
            let modification = ModifyUser::new().picture(user.picture.unwrap());
            self.modify_user(user.uid.as_str(), modification).await?;
        }

        Ok(true)
    }

    pub async fn member_of(&self, cn: &str) -> Vec<User> {
        self.users.lock().await.values().filter(|u| u.member.is_some() && u.member.as_ref().unwrap().contains(cn)).map(|u| u.clone()).collect()
    }
}