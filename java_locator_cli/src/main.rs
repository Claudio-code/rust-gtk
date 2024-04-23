
use std::{fs, process::Command};
use clap::Parser;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    path: String
}

fn main() {
    let args = Args::parse();

    let paths = fs::read_dir(args.path).unwrap();

    for path_result in paths {
        let path = path_result.unwrap();
        let command_content = format!("$JAVA_HOME/bin/keytool -importcert -file {} -cacerts -storepass changeit -noprompt", path.path().display());
        println!("{:?}", command_content);
        
        let out = Command::new("sh")
            .arg("-c")
            .arg(command_content)
            .output()
            .expect("teste ");
        println!("teste {:?}", out);
    }
}
