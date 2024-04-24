use dotenv::dotenv;

use crate::common::{Ldap, ModifyUser};

mod common;


#[tokio::main]
async fn main() {
    dotenv().ok(); // Load the .env file


    let mut ldap = Ldap::new().await.unwrap();

    ldap.test_connection().await.unwrap();

    ldap.update_groups().await.unwrap();
    ldap.update_users().await.unwrap();

    let modified_user = ModifyUser::new().number("1234".to_string());

    let res = ldap.modify_user("test.test2", modified_user).await.unwrap();

    let user = ldap.update_user_info_by_id("test.test2").await;
    println!("{:?}", user);

    println!("Modify user result: {}", res);


    let users = ldap.get_users().await;
    for user in users {
        if !user.password.starts_with("{SSHA}") {
            println!("{:?}", user);
        }
    }
}
