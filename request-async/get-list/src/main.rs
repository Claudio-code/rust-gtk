fn main() {
    let body = reqwest::get("https://pokeapi.co/api/v2/pokemon?limit=10&offset=0")
        .await?
        .text()
        .await?;

    println!("body = {:?}", body);
}
