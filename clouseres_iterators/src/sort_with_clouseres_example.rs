
#[derive(Debug)]
struct City {
    name: String,
    population: u64,
}

fn sort_pop_name(cities: &mut Vec<City>) {
    cities.sort_by_key(sort_pop_name_helper);
}

fn sort_pop_name_helper(city: &City) -> usize {
    let name = city.name.clone();
    name.len()
}

fn sort_pop_population(cities: &mut Vec<City>) {
    cities.sort_by_key(|city| city.population);
}

pub fn run() {
    let cianorte = City{name: String::from("cianorte"), population: 123};
    let viseu = City{name: String::from("viseu"), population: 31222};
    let maringa = City{name: String::from("maringa"), population: 921};
    let curitiba = City{name: String::from("curitiba"), population: 621};
    let porto = City{name: String::from("porto"), population: 412};

    let mut list_cities: Vec<City> = Vec::new();
    list_cities.push(cianorte);
    list_cities.push(viseu);
    list_cities.push(maringa);
    list_cities.push(curitiba);
    list_cities.push(porto);

    sort_pop_name(&mut list_cities);
    println!("\n list_cieties orded by name len: {:?} \n", list_cities);

    sort_pop_population(&mut list_cities);
    println!("\n list_cieties orded by population len: {:?} \n", list_cities);
}
