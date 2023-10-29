fn main() {
    let city = "cianorte"; // string
    let float_number = 2.3; // float
    let bool = true; // boolean
    let icon = '♥'; // icon char unicode
    // integers
    // singed can store postive and negative values
    // unsinged can't store negativo values
    let count: u8 = 2;
    let positive: i8 = 21;

    // isize and usize define integer size basead in arch machine
    let mark:isize = -21;
    let value:usize = 2;

    let decimal_separator = 22_02;
    println!("decimal {}", decimal_separator);

    // tupla is array with setted size
    let tu = (2, "name");
    println!("tupla {}", tu.0);

    let (first, second) = tu;
    println!("first {}, second {}", first, second);

    // array
    let names = ["claudio", "silva"];
    for value in names {
        println!("name {}", value);
    }

    // vector like list in java
    let mut nums = vec![1, 2, 3];
    nums.push(4);
    nums.reverse();
    println!("vector {:?}", nums);
}
