use async_std::{fs::File, io, prelude::*, task};

async fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

fn main() {
    let future_task = task::spawn(async {
        let result = read_file("./.gitignore").await;
        match result {
            Ok(k) => println!("Content {}", k),
            Err(err) => println!("Error {}", err)
        }
    });

    println!("Start task");
    task::block_on(future_task);
    println!("Finished task");
}
