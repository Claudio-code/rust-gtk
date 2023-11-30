fn main() {
    // it like List in java or Collection in php
    let mut nums: Vec<i32> = vec![];
    nums.push(1);
    nums.push(12);
    nums.push(32);

    let removed_item = nums.pop();

    println!("removed item {:?}", removed_item);

    let two = nums[1]; // copy value
                       // &nums[1], creates a reference if copy is not available

    println!("{}", two);

    // .last
    // .first_mut and .last_mut, so will borrow mutuable references

    println!("{}", nums.len()); // return a value of the length
    println!("{}", nums.is_empty()); // bool

    nums.insert(0, 42);
    nums.insert(3, 21);
    nums.insert(4, 92);

    nums.remove(3);

    println!("{:?}", nums);

    nums.reverse();

    println!("{:?}", nums);

    nums.sort();

    println!("{:?}", nums);
}
