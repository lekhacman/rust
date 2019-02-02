fn main() {
    println!("Hello, world!");
    let user = new_user(String::from("Man"), String::from("man@example.com"));

    println!("Created user '{}' - {}", user.username, user.email);

    let user2 = User {
        username: String::from("Andrew"),
        email: String::from("abc@example.com"),
        ..user
    };

    assert_eq!(user2.username, "Andrew");
    assert_eq!(user2.active, false);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn new_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 0,
        active: false
    }
}
