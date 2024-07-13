use std::sync::Arc;
use tokio::sync::Mutex;

use crate::common::{Config, Ldap};

#[derive(Clone)]
pub struct AppState {
    pub ldap: Arc<Mutex<Ldap>>,
    pub env: Config,
}

impl AppState {
    pub fn new(ldap: Ldap, env: Config) -> Self {
        Self {
            ldap: Arc::new(Mutex::new(ldap)),
            env,
        }
    }
}