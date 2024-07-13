#[derive(Debug, Clone)]
pub struct Config {
    // pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
    pub ldap_user: String,
    pub ldap_password: String,
    pub ldap_host: String,
    pub ldap_port: u16,
    pub ldap_base_dn: String,
    pub ldap_users_base_dn: String,
    pub ldap_groups_base_dn: String,
}

impl Config {
    pub fn init() -> Config {
        // let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expires_in = std::env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");
        let jwt_maxage = std::env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");

        let ldap_user = std::env::var("BIND_USER").expect("BIND_USER must be set");
        let ldap_password = std::env::var("BIND_PASSWORD").expect("BIND_PASSWORD must be set");
        let ldap_host = std::env::var("LDAP_SERVER").expect("LDAP_SERVER must be set");
        let ldap_port = std::env::var("LDAP_PORT").expect("LDAP_PORT must be set");
        let ldap_base_dn = std::env::var("LDAP_BASE").expect("LDAP_BASE must be set");
        let ldap_users_base_dn = std::env::var("LDAP_USERS_BASE").expect("LDAP_USERS_BASE must be set");
        let ldap_groups_base_dn = std::env::var("LDAP_GROUPS_BASE").expect("LDAP_GROUPS_BASE must be set");

        Config {
            // database_url,
            jwt_secret,
            jwt_expires_in,
            jwt_maxage: jwt_maxage.parse::<i32>().unwrap(),
            ldap_user,
            ldap_password,
            ldap_host,
            ldap_port: ldap_port.parse::<u16>().unwrap(),
            ldap_base_dn,
            ldap_users_base_dn,
            ldap_groups_base_dn,
        }
    }
}