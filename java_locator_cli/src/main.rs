use clap::Parser;
use std::fmt::Debug;
use std::fs::ReadDir;
use std::io::{Error, ErrorKind, Read};
use std::process::{Child, ExitStatus};
use std::str::FromStr;
use std::{fs, path::PathBuf, process::Command};
use strum_macros::EnumString;

const ASDF_JAVA_PATH: &str = "/.asdf/installs/java";
const JETBRAINS_JAVA_PATH: &str = "/.jdks";
const SDKMAN_JAVA_PATH: &str = "/.sdkman/candidates/java";
const KEYTOOL_PATH: &str =
    "{}/bin/keytool -list -v -keystore {}/lib/security/cacerts -storepass changeit | grep Alias";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    path: String,
}

#[derive(Default, Debug)]
pub struct Key {
    pub name: String,
}

impl Key {
    fn from_string(key: String) -> Self {
        Self { name: key }
    }
}

#[derive(Default, Debug)]
pub struct JDKCollection {
    pub title: String,
    pub jdk_path: String,
    pub keys: Vec<Key>,
}

fn main() {
    // let args = Args::parse();

    // let paths = fs::read_dir(args.path).unwrap();

    // for path_result in paths {
    //     let path = path_result.unwrap();
    //     let command_content = format!("$JAVA_HOME/bin/keytool -importcert -file {} -cacerts -storepass changeit -noprompt", path.path().display());
    //     println!("{:?}", command_content);

    //     let out = Command::new("sh")
    //         .arg("-c")
    //         .arg(command_content)
    //         .output()
    //         .expect("teste ");
    //     println!("teste {:?}", out);
    // }

    // load_jdks_sdkman(); // ok

    load_jdks_jetbrains(); // error

    // load_jdks_asdf(); // ok
}

fn load_jdks_asdf() {
    let binding = dirs::home_dir().unwrap();
    let mut home_dir = binding.display().to_string();
    home_dir.push_str(ASDF_JAVA_PATH.into());
    let paths = fs::read_dir(home_dir).unwrap();
    let mut jdk_collections = list_jdks(paths);

    println!("jdk_collections {:?}", jdk_collections);
}


fn load_jdks_jetbrains() {
    let binding = dirs::home_dir().unwrap();
    let mut home_dir = binding.display().to_string();
    home_dir.push_str(JETBRAINS_JAVA_PATH.into());
    let paths = fs::read_dir(home_dir).unwrap();
    let mut jdk_collections = list_jdks(paths);

    println!("jdk_collections {:?}", jdk_collections);
}


fn load_jdks_sdkman() {
    let binding = dirs::home_dir().unwrap();
    let mut home_dir = binding.display().to_string();
    home_dir.push_str(SDKMAN_JAVA_PATH.into());
    let paths = fs::read_dir(home_dir).unwrap();
    let mut jdk_collections = list_jdks(paths);

    println!("jdk_collections {:?}", jdk_collections);
}

fn list_jdks(paths: ReadDir) -> Vec<JDKCollection> {
    let mut jdk_collections: Vec<JDKCollection> = Vec::new();
    for path_result in paths {
        let path = path_result.unwrap();
        let path_name = path.path().to_str().unwrap().to_string();
        let is_dir = path.path().is_dir();
        
        if !path_name.contains("current") && is_dir {
            let jdk_name = path_name.split("/").last().unwrap();
            let keys: Vec<Key> = list_certs_jdk(path_name.clone())
                .iter()
                .map(|key_name| Key::from_string(key_name.to_string()))
                .collect();
            let jdk_collection = JDKCollection {
                title: "wd".to_owned(),
                jdk_path: path_name,
                keys,
            };
            jdk_collections.push(jdk_collection);
        }
    }
    jdk_collections
}

fn list_certs_jdk(jdk_home_dir: String) -> Vec<String> {
    let mut keys_list: Vec<String> = Vec::new();
    let command = format!("{}/bin/keytool -list -v -keystore {}/lib/security/cacerts -storepass changeit | grep Alias", jdk_home_dir, jdk_home_dir);
    let out = keytool_capture(command).unwrap();
    for line in out.lines() {
        let key_name = line.replace("Alias name: ", "").replace(" [jdk]", "");
        keys_list.push(key_name);
    }
    keys_list
}

fn spawn_child(command: String) -> std::io::Result<Child> {
    std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
}

fn keytool_capture(command: String) -> std::io::Result<String> {
    let mut child = spawn_child(command)?;
    match child.wait()?.code() {
        Some(code) if code == 0 => {} // success
        Some(code) if code == 1 => {} // success -> Ok(keys not found)
        Some(code) if code == 3 => {} // success -> Ok(keys not found or error)
        Some(code) if code == 4 => {
            return Err(Error::new(
                ErrorKind::PermissionDenied,
                "Missing Priviledges or Unit not found",
            ))
        }
        // unknown errorcodes
        Some(code) => {
            return Err(Error::new(
                // TODO: Maybe a better ErrorKind, none really seem to fit
                ErrorKind::Other,
                format!("Process exited with code: {code}"),
            ));
        }
        None => {
            return Err(Error::new(
                ErrorKind::Interrupted,
                "Process terminated by signal",
            ))
        }
    }

    let mut stdout: Vec<u8> = Vec::new();
    let size = child.stdout.unwrap().read_to_end(&mut stdout)?;

    if size > 0 {
        if let Ok(s) = String::from_utf8(stdout) {
            return Ok(s);
        } else {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Invalid utf8 data in stdout",
            ));
        }
    }

    // if this is reached all if's above did not work
    Err(Error::new(
        ErrorKind::UnexpectedEof,
        "keytool stdout empty",
    ))
}
