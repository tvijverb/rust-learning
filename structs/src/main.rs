
// Create struct with bool, String, and unsigned 64-bit integer
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // create new User
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // change email value
    user1.email = String::from("anotheremail@example.net");

    // copy user1 values and modify some of the parameters
    // user1 is NOT valid anymore, ownership of variables
    // is now with user2... not exactly what we intended.
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}

fn build_user(email: String, username: String) -> User {
    // email and username are directly inserted into new User struct
    // without explicit assignment
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}


// tuple struct
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
// }
