fn main() {
    let unit = systemctl::Unit::from_systemctl("docker.service")
        .unwrap();
    
    println!(" {:?} \n {}", unit.utype, unit.description.unwrap());

    let list = systemctl::list_units(None, None, None).unwrap();
    
    for item in list {
        println!("print: {}", item);
    }
}
