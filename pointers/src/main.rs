use std::rc::Rc;

fn main() {
    box_deferency();
    rc_and_arc();
}

fn box_deferency() {
    let tuple = (12, "apps"); // create on the stack
    let tuple_box = Box::new(tuple); // create referece pointer on the heap, but was stored on the stack
    println!("{:?}", tuple_box);

    let x = 5; // has value on the stack
    let y = &x; // has referece of value on the stack
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("{:?}", y);
}

fn rc_and_arc() {
    let s1 = Rc::new(String::from("Pointer"));
    let s2 = s1.clone();
    let s3 = s2.clone();

    println!("{}, {}, {}", s1, s2, s3);

    // Compare String objects refereces like object.equals in Java
    // But trait net extend PartialEq
    assert!(s1 == s2 && s1 == s3);
}
