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
    let mut nums = vec![1, 2, 3, 4, 5, 6];
    nums.push(7);
    println!("vector {:?}", nums);

    // A slice is a kind of reference, so it does not have ownership.
    let slic: &[i32] = &nums[1..5];
    println!("slice {:?}", slic);

    let hello = String::from("hello world");
    let word = first_word(&hello);
    println!("word {}", word);

    // differences between String and &str

    // String is like String object in java you have methods to help in work and you can modify value
    let full_name = String::from("full name");

    // &str is slice, is reference to vector it is fat pointer
    // &str you can't modify string slice
    let cource = "rust --0";
    println!("test {} explicit return {}", cal_plus(2, 2), cal_plus_with_explicit_return(3, 1))
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// in function with implict return
fn cal_plus(ab: i16, cd: i16) -> i16 {
    ab + cd
}

// same function with explicit return
fn cal_plus_with_explicit_return(ab: i16, cd: i16) -> i16 {
    return ab + cd
}
