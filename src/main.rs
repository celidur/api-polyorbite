use dotenv::dotenv;

use crate::common::{Ldap, user::ModifyUser};

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

    let modified_user = ModifyUser::new().password("password".to_string());

    let res = ldap.users.modify_user("gs", modified_user).await.unwrap();

    println!("Modify user result: {}", res);

    let user = ldap.users.user("gs").await.unwrap();

    println!("verif : {}", user.verify_password("password"));

    println!("user: {:?}", user);


    let users = ldap.users.to_vec().await;
    for user in users {
        if !user.password.starts_with("{SSHA}") {
            println!("{:?}", user);
        }
    }
}
