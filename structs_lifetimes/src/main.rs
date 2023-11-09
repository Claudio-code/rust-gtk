
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    sign_in_count: u32,
}

#[derive(Debug)]
struct Coordinates(i32, i32, i32);

fn main() {
    let kraudio = build_user(String::from("kraudeo"));
    let coord = Coordinates(2, 33, 12);

    println!("user: {:?}", kraudio);
    println!("Coordinates {:#?}", coord)
}

fn build_user(username: String) -> User {
    User{
        active: false,
        username: username,
        sign_in_count: 2
    }
}
