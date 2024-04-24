use dotenv::dotenv;

use crate::common::{Ldap, ModifyUser};

mod common;


#[tokio::main]
async fn main() {
    dotenv().ok(); // Load the .env file


    let ldap = Ldap::new().await;
    if ldap.is_err() {
        panic!("{:?}", ldap.err().unwrap());
    }
    let mut ldap = ldap.unwrap();

    ldap.update().await.unwrap();

    let modified_user = ModifyUser::new().number("12345".to_string());

    let res = ldap.users.modify_user("test.test", modified_user).await.unwrap();

    println!("Modify user result: {}", res);


    let users = ldap.users.to_vec().await;
    for user in users {
        if !user.password.starts_with("{SSHA}") {
            println!("{:?}", user);
        }
    }
}
