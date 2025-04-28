struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

// struct UserType2 {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // example_struct()
    // example_build_and_spread()
    example_unit_like_structs()
}

fn example_struct() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
}

fn example_build_and_spread() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn example_tuple() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn example_unit_like_structs() {
    let subject = AlwaysEqual;
}

// fn example_life_time() {
//     let user1 = UserType2 {
//         active: true,
//         username: "someusername123",
//         email: "someone@example.com",
//         sign_in_count: 1,
//     };
// }
