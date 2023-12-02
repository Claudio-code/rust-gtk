use std::collections::BinaryHeap;

fn main() {
    let mut bheap = BinaryHeap::<i32>::new();

    bheap.push(21);
    bheap.push(31);
    bheap.push(13);
    bheap.push(17);
    bheap.push(8);

    bheap.pop();

    let value_removed = bheap.peek();
    // let mut mutable_value_removed = bheap.peek_mut();

    println!("{:?}", bheap);
}
