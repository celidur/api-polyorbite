use std::{collections::{HashMap, HashSet}, sync::Arc};

use ldap3::{LdapConnAsync, Mod, Scope};
use tokio::sync::Mutex;

use super::Group;

#[derive(Debug)]
pub struct Groups {
    groups: Arc<Mutex<HashMap<String, Group>>>,
    ldap_url: String,
    ldap_user: String,
    ldap_password: String,
    groups_base_dn: String,
    base_dn: String,
}

impl Groups {
    pub fn new(ldap_url: String, ldap_user: String, ldap_password: String, groups_base_dn: String, base_dn: String) -> Self {
        Self {
            groups: Arc::new(Mutex::new(HashMap::new())),
            ldap_url,
            ldap_user,
            ldap_password,
            groups_base_dn,
            base_dn,
        }
    }

    pub async fn update(&mut self) -> ldap3::result::Result<()> {
        let (conn, mut ldap) = LdapConnAsync::new(self.ldap_url.as_str()).await?;
        ldap3::drive!(conn);

        ldap.simple_bind(self.ldap_user.as_str(), self.ldap_password.as_str())
            .await?
            .success()?;

        let filter = "(objectClass=groupOfNames)";

        let (rs, _res) = ldap
            .search(self.base_dn.as_str(), Scope::Subtree, filter, vec!["*"])
            .await?
            .success()?;

        ldap.unbind().await?;

        let mut groups = self.groups.lock().await;
        groups.clear();
        for entry in rs {
            let group = Group::new(ldap3::SearchEntry::construct(entry));
            groups.insert(group.cn.clone(), group);
        }
        Ok(())
    }

    pub async fn update_group(&mut self, id: &str) -> ldap3::result::Result<()> {
        let (conn, mut ldap) = LdapConnAsync::new(self.ldap_url.as_str()).await?;
        ldap3::drive!(conn);

        ldap.simple_bind(self.ldap_user.as_str(), self.ldap_password.as_str())
            .await?
            .success()?;

        let filter = format!("(&(objectClass=groupOfNames)(cn={}))", id);

        let (rs, _res) = ldap
            .search(self.base_dn.as_str(), Scope::Subtree, filter.as_str(), vec!["*"])
            .await?
            .success()?;

        ldap.unbind().await?;

        if rs.is_empty() {
            self.groups.lock().await.remove(id);
            return Ok(());
        }

        let entry = rs.first().unwrap();
        let group = Group::new(ldap3::SearchEntry::construct(entry.clone()));
        self.groups.lock().await.insert(group.cn.clone(), group);
        Ok(())
    }

    pub async fn group(&self, id: &str) -> Option<Group> {
        self.groups.lock().await.get(id).map(|u| u.clone())
    }

    pub async fn to_vec(&self) -> Vec<Group> {
        self.groups.lock().await.values().map(|u| u.clone()).collect()
    }

    pub async fn add_group_owner(&mut self, group: &str, owner: Vec<&str>) -> ldap3::result::Result<bool> {
        let (conn, mut ldap) = LdapConnAsync::new(self.ldap_url.as_str()).await?;
        ldap3::drive!(conn);

        ldap.simple_bind(self.ldap_user.as_str(), self.ldap_password.as_str())
            .await?
            .success()?;

        let filter = format!("(&(objectClass=groupOfNames)(cn={}))", group);

        let (rs, _res) = ldap
            .search(self.base_dn.as_str(), Scope::Subtree, filter.as_str(), vec!["owner"])
            .await?
            .success()?;

        if rs.is_empty() {
            return Ok(false);
        }

        let entry = ldap3::SearchEntry::construct(rs.first().unwrap().clone());
        let current_owner = entry.attrs.get("owner").unwrap().clone();
        let mut new_owner :HashSet<&str> = HashSet::new();
        new_owner.extend(owner);
        new_owner.extend(current_owner.iter().map(|o| o.as_str()));
        
        if entry.attrs.get("owner").unwrap().len() == new_owner.len() {
            return Ok(false);
        }

        let mut changes = vec![];
        changes.push(Mod::Replace("owner", new_owner));

        let res = ldap
            .modify(entry.dn.as_str(), changes)
            .await?
            .success();

        ldap.unbind().await?;

        if res.is_err() {
            return Ok(false);
        }

        self.update_group(group).await?;

        Ok(true)
    }


}