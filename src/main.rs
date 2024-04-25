use dotenv::dotenv;

use crate::common::{Ldap, user::ModifyUser, user::UserBuilder};

mod common;


#[tokio::main]
async fn main() {
    dotenv().ok(); // Load the .env file


    let ldap = Ldap::new().await;
    if ldap.is_err() {
        panic!("{:?}", ldap.err().unwrap());
    }
    let mut ldap = ldap.unwrap();

    test_user(&mut ldap).await;    
}

async fn test_user(ldap: &mut Ldap) {
    let value = ldap.users.user("user_test").await.is_some();
    let res = ldap.users.delete_user("user_test").await;
    if res.is_err() {
        panic!("{:?}", res.err().unwrap());
    }
    let res = res.unwrap();
    println!("Delete user result: {} expected: {}", res, value);
    
    let new_user = UserBuilder::new().uid("user_test".to_string()).password("test".to_string()).mail("test".to_string()).last_name("test".to_string()).first_name("test".to_string()).name("test".to_string()).build();
    if new_user.is_err() {
        panic!("{:?}", new_user.err().unwrap());
    }
    let new_user = new_user.unwrap();

    println!("new_user: {:?}", new_user);

    let res = ldap.users.new_user(new_user).await;
    if res.is_err() {
        panic!("{:?}", res.err().unwrap());
    }
    let res = res.unwrap();

    if res {
        let user = ldap.users.user("user_test").await.unwrap();
        println!("user: {:?}", user);
    } else {
        panic!("Failed to create user");
    }

    let modify = ModifyUser::new().password("password".to_string());
    let res = ldap.users.modify_user("user_test", modify).await.unwrap();
    println!("Modify user result: {}", res);

    let user = ldap.users.user("user_test").await.unwrap();
    println!("user: {:?}", user);
    println!("verif : {}", user.verify_password("password"));
}