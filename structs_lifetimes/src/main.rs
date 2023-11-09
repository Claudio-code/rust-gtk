
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    sign_in_count: u32,
}

#[derive(Debug)]
struct Coordinates(i32, i32, i32);

#[derive(Debug)]
struct Square {
    width: u32,
    height: u32
}

impl Square {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn whats_my_with(&self) -> u32 {
        self.width
    }

    fn change_width(& mut self, new_width: u32) {
        self.width = new_width
    }
}

fn main() {
    let kraudio = build_user(String::from("kraudeo"));
    let coord = Coordinates(2, 33, 12);

    println!("user: {:?}", kraudio);
    println!("Coordinates {:#?}", coord);

    let mut square = Square{width: 21, height: 42};
    println!("area: {}, whats_my_with: {}", square.area(), square.whats_my_with());
    square.change_width(221);
    println!("area: {}, whats_my_with: {}", square.area(), square.whats_my_with());

    let (four, nine) = (4, 9);
    print_refs(&four, &nine);
    pass_x(&four, &nine);

}

fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }

fn build_user(username: String) -> User {
    User{
        active: false,
        username: username,
        sign_in_count: 2
    }
}
