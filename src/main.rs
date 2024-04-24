use dotenv::dotenv;

mod common;

use common::ldap::Ldap;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load the .env file

    let secret = "ximkog-8civvu-pynpYf";

    let mut ldap = Ldap::new().await.unwrap();

    ldap.test_connection().await.unwrap();

    ldap.update_groups().await.unwrap();
    ldap.update_users().await.unwrap();

    let user = ldap
        .get_user_by_id("gaetan.florio")
        .await
        .unwrap();

    let is_verified = user.verify_password(secret);

    println!("Verification result: {}", is_verified);

    let users = ldap.get_users().await;
    for user in users {
        if !user.password.starts_with("{SSHA}") {
            println!("{:?}", user);
        }
    }
}
