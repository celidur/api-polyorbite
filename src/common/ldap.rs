use super::user::Users;
use super::group::Groups;
use std::env;

use ldap3::LdapConnAsync;

#[derive(Debug)]
pub struct Ldap {
    pub groups: Groups,
    pub users: Users,
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

        let mut users = Users::new(
            url.clone(),
            user.clone(),
            password.clone(),
            users_base_dn.clone(),
            base_dn.clone(),
        );

        let _ = users.update().await;

        let mut groups = Groups::new(
            url.clone(),
            user.clone(),
            password.clone(),
            groups_base_dn.clone(),
            base_dn.clone(),
        );

        let _ = groups.update().await;

        let res = Ldap::test_connection(url.as_str(), user.as_str(), password.as_str()).await;
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
