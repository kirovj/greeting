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
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = new_user(String::from("jack"), String::from("qwe@a.com"));

    let user2 = User {
        username: String::from("alice"),
        ..user1
    };
    user1.active = false; // user1 has to be mut

    println!(
        "user1: {}, user2: {}, user1.active: {}",
        user1.username, user2.username, user1.active
    );
}
