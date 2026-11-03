struct Color(i32, i32, i32);
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        email: String::from("someone@example.com"),
        username: String::from("someone"),
        sign_in_count: 1,
    };

    user1.email = String::from("someone2@example.com");

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email: email,       //可以簡化寫 email,
        username: username, //可以簡化寫username
        sign_in_count: 1,
    }
}
