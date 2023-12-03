use std::{fs::File, fmt::Error, io::ErrorKind};

fn main() {
    // panic is like last throw in one java api
    // panic!("screem");

    // Result pattern contain value in OK and error in ERR
    let file = File::open("text.txt");

    // unwrap thorw panic if not file and you can not try
    // let file = File::open("text.txt").unwrap();

    // expect custom panic message
    // let file = File::open("text.txt").expect("panic message");

    // match file {
    //     Ok(file) => file,
    //     Err(err) => panic!("error message = {}", err),
    // };

    // when resolve error
    match file {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("text.txt") {
                Ok(file_created) => file_created,
                Err(err) => panic!("Cannot create the file = {}", err),
            },
            _ => panic!("Another generic error :)"),
        },
    };

}

