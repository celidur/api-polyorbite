use super::user::Users;
use super::group::Groups;
use super::Config;
use std::env;

use ldap3::LdapConnAsync;

#[derive(Debug)]
pub struct Ldap {
    pub groups: Groups,
    pub users: Users,
}

impl Ldap {
    pub async fn new(config: Config) -> Result<Self, &'static str> {
        let url = format!("{}:{}", config.ldap_host, config.ldap_port);

        let mut users = Users::new(
            url.clone(),
            config.ldap_user.clone(),
            config.ldap_password.clone(),
            config.ldap_users_base_dn.clone(),
            config.ldap_base_dn.clone(),
        );

        let _ = users.update().await;

        let mut groups = Groups::new(
            url.clone(),
            config.ldap_user.clone(),
            config.ldap_password.clone(),
            config.ldap_groups_base_dn.clone(),
            config.ldap_base_dn.clone(),
        );

        let _ = groups.update().await;

        let res = Ldap::test_connection(url.as_str(), config.ldap_user.as_str(), config.ldap_password.as_str()).await;
        if res.is_err() || res.unwrap() == false {
            return Err("Failed to connect to LDAP server");
        }

        Ok(Self {
            users,
            groups
        })
    }

    async fn test_connection(url: &str,user: &str, password: &str) -> ldap3::result::Result<bool> {

        let (conn, mut ldap) = LdapConnAsync::new(url).await?;
        ldap3::drive!(conn);

        let result = ldap
            .simple_bind(user, password)
            .await?
            .success();
        ldap.unbind().await?;

        if result.is_err() {
            return Ok(false);
        }
        Ok(true)
    }

    pub async fn update(&mut self) -> ldap3::result::Result<()> {
        self.groups.update().await?;
        self.users.update().await?;
        Ok(())
    }
}
