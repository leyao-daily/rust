struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


fn main() {
    let mut user1 = build_user("le.yao@intel.com".to_string(), "Yao Le".to_string());
    println!("{}", user1.email);
}


fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
