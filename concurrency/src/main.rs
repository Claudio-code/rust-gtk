use std::{sync::{mpsc::{channel, Receiver, Sender}, Arc}, thread, time::Duration};

fn main() {
    await_main_thread_print_message();
    join_handle_block_thread_to_await_finish();
    play_with_channels();
    using_value_across_threads();
}

fn join_handle_block_thread_to_await_finish() {
    let handle = thread::spawn(move || {
        println!("blabla 2");
    });
    println!("in the main thread");
    handle.join().unwrap();
}

fn await_main_thread_print_message() {
    thread::spawn(move || {
        println!("blabla");
    });
    thread::sleep(Duration::from_secs(1));
}

fn play_with_channels() {
    let (transmiter, receiver): (Sender<String>, Receiver<String>) = channel();
    let tx = transmiter.clone();
    let message = String::from(" val w Transmitting value");
    thread::spawn(move || {
        println!("Transmitting value {}", message);
        transmiter.send(message).unwrap();
    });

    thread::spawn(move || {
        let vec = vec![
            String::from("Clone"),
            String::from(" Is "),
            String::from("Transmitting"),
        ];
        for val in vec {
            println!("Transmitting value with clone {}", val);
            tx.send(val).unwrap();
        }
    });

    for rec in receiver {
        println!("{}", rec);
    }

}

fn using_value_across_threads() {
    let rc1 = Arc::new(String::from("value to test"));
    let rc2 = rc1.clone();

    thread::spawn(move || {
        println!("value to print {}", rc2);
    });

    thread::spawn(move || {
        println!("print another thread {}", rc1);
    });
}

