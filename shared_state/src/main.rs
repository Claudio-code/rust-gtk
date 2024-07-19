
use std::{sync::{Arc, Mutex}, thread, time::Instant};
use rayon::prelude::*;
use num::{BigUint, One};

fn main() {
    // use_mutex();
    // how_recover_pointer_after_panic();
    test_factorial();
}

fn factorial(num: u32) -> BigUint {
    if num == 0 || num == 1 {
        return BigUint::one();
    }
    (1..= num)
        .map(BigUint::from)
        .reduce(|acc, x| acc * x).unwrap()
}

fn multi_factorial(num: u32) -> BigUint {
    if num == 0 || num == 1 {
        return BigUint::one();
    }
    (1..= num)
        .into_par_iter()
        .map(BigUint::from)
        .reduce(|| BigUint::one(), |acc, x| acc * x)
}

fn test_factorial() {
    let now = Instant::now();
    factorial(50000);
    println!("{:.2?}", now.elapsed());


    let now = Instant::now();
    multi_factorial(50000);
    println!("{:.2?}", now.elapsed());
}


fn how_recover_pointer_after_panic() {
    let lock = Arc::new(Mutex::new(1));
    let lock2 = Arc::clone(&lock);

    let _ = thread::spawn(move || -> () {
        // we acquire the lock here
        let _guard = lock2.lock().unwrap();
        
        //  mutex is now poisoned
        panic!();
    });

    let mut guard = match lock.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(), // return pointer
    };

    *guard += 1;
    println!("{:?}", guard);
}

fn use_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..8  {
        let internal_counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = internal_counter.lock().unwrap();
            // if lock several times in same thread cause dead lock
            // let mut num2 = internal_counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", counter.lock().unwrap());
}
