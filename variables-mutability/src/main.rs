fn main() {
    // imutable variable
    let num = 5;
    println!("Hello value {}", num);

    // if you can change value in variable you can add reserved word 'mut'
    let mut num_mutable = 21;
    println!("before change value {}", num_mutable);

    num_mutable = 2;
    println!("after change value {}", num_mutable);


    // you can declare constant too
    const TEST: i32 = 12;

    println!("my constant {}", TEST);
}
