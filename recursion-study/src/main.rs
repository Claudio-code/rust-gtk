fn fact(num: i32) -> i32 {
    if num < 1 {
        return 1;
    }
    num * fact(num - 1)
}

fn fib(num: i32) -> i32 {
    if num == 0 || num == 1 {
        return num
    }

    let n1 = fib(num - 1);
    let n2 = fib(num - 2);
    n1 + n2
}

fn palindrom(array: &Vec<i32>, start: usize, end: usize) -> bool {
    if start >= end {
        return true;
    }

    if array[start] == array[end] { 
        return palindrom(array, start + 1, end + 1);
    }
    false
}

fn toh(num: i32) -> i32 {
    if num == 0 {
        return 0
    }

    toh(num - 1) + 1 + toh(num - 1)
}

fn main() {
    // let result = fact(5);
    // println!("factorial result {}", result);

    // let fib_result = fib(15);
    // println!("fib result {}", fib_result);

    // let palindrom_false = vec![1, 2, 3, 4];
    // println!("palindrom false {:?}", palindrom(&palindrom_false, 0, palindrom_false.len() - 1));

    // let palindrom_true = vec![1, 2, 3, 4, 3, 2, 1];
    // println!("palindrom true {:?}", palindrom(&palindrom_true, 0, palindrom_true.len() - 1));


    println!("{}", toh(2))
}
