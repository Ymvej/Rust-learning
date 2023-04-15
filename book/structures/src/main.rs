struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;

}




















// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let black = Color(0,1,2);
//     let origin = Point(0,0,0);

//     println!("{0}", black.1)

// }












// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username: username,
//         email: email,
//         sign_in_count: 1,
//     }
// }




// fn main() {
//     let user1 = build_user(
//         String::from("someone@example.com"),
//         String::from("someusername123"),
//     );

//     let user2 = User {
//         active: user1.active.clone(),
//         username: user1.username.clone(),
//         email: String::from("another@example.com"),
//         sign_in_count: user1.sign_in_count.clone(),
//     };

//     let user3 = User {
//         email: String::from("athird@example.com"),
//         ..user1
//     };
    
//     let user4 = User {
//         email: String::from("athird@example.com"),
//         ..user1
//     };


//     println!("{}", user3.sign_in_count)
// }