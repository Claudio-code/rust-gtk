enum Pet {
    DOG,
    CAT,
    FISH,
}

impl Pet {
    fn what_am_i(self) -> &'static str {
        match self {
            Pet::DOG => "I am a dog",
            Pet::CAT => "I am a cat",
            Pet::FISH => "I am a fish",
        }
    }
}

#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let dog = Pet::DOG;
    what_pet(&dog);
    println!("Hello, world! who are me? {}", dog.what_am_i());

    let home = IpAddrKind::V4(String::from("0.0.0.0"));
    let loopack = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{:?}", home);
    println!("{:?}", loopack);

    // Option
    let x: i32 = 23;
    let y: Option<i32> = Some(7);
    let result = plus_one(y);
    let result_none = plus_one(None);

    println!("{:?} | {:?}", result, result_none);
    what_letter("A");
    what_letter("W");

    let value = 21;
    match value {
        10..=21 => println!("ok match"),
        _ => println!("Nop"),
    };

    let some_value = Some(132);
    match some_value {
        Some(132) => println!("here"),
        _ => println!("default"),
    };
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn what_pet(pet: &Pet) {
    match pet {
        Pet::CAT => println!("Bad cat"),
        Pet::FISH => println!("Bad fish"),
        Pet::DOG => println!("Bad dog"),
    }
}

fn what_letter(input: &str) {
    match input {
        "A" => println!("letter A"),
        "B" => println!("letter B"),
        _ => println!("letter not found"),
    }
}
