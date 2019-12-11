// Basically a class from OOP
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Structs used as new types, kinda like tuples
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("user1"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@email.com"),
        username: String::from("user2"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    let mut user3 = User {
        email: String::from("yetanother@email.com"),
        username: String::from("user3"),
        ..user1
    };

    let user4 = build_user(String::from("email@email.com"), String::from("user4"));

    // Color != Point, black != origin
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}


fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
