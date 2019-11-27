struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


fn main() {
    let mut user1 = build_user("le.yao@intel.com".to_string(), "Yao Le".to_string());
    println!("{}", user1.email);
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("another"),
        ..user1
    };
    println!("{}", user2.username);
}


fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
