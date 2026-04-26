use std::fmt::Display;
use std::io;
use std::num::ParseIntError;

#[derive(Copy, Clone)]
struct User<'a> {
    id: i32,
    name: &'a str,
}

impl Display for User<'static> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User {{id : {}, name:{} }}", self.id, self.name)
    }
}

const USER_LIST: [User; 2] = [
    User {
        id: 1,
        name: "fabio",
    },
    User {
        id: 2,
        name: "renzo",
    },
];

fn find_user(user_id: i32) -> Option<User<'static>> {
    USER_LIST.iter().find(|user| user.id == user_id).copied()
}

fn parse_user_id(input: &str) -> Result<i32, ParseIntError> {
    input.parse::<i32>()
}

fn get_user_from_input(input: &str) -> Result<User<'static>, String> {
    // step 1 : parse input
    let user_id = match parse_user_id(input) {
        Ok(id) => id,
        Err(_) => return Err(String::from("Invalid ID format")),
    };

    // step 2 : find user
    let user = match find_user(user_id) {
        Some(db_user) => db_user,
        None => return Err(String::from("User not found")),
    };

    Ok(user)
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    match get_user_from_input(input.trim_matches('\n')) {
        Ok(user) => println!("Found user : {}", user),
        Err(err) => println!("Error : {}", err),
    }
}
