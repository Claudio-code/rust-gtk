use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Pokemon {
    name: String,
}

#[derive(Deserialize, Debug)]
struct ResultBody {
    count: i32,
    results: Vec<Pokemon>,
}

#[tokio::main]
async fn main()  -> Result<(), Box<dyn std::error::Error>> {
    let uri = "https://pokeapi.co/api/v2/pokemon";
    let params = [
        ("limit", "10"),
        ("offset", "0"),
    ];
    let url = reqwest::Url::parse_with_params(uri, params)?;
    let body = reqwest::get(url)
        .await?
        .json::<ResultBody>()
        .await?;

    println!("numers of pokemons {}", body.count);

    for pokemon in &body.results {
        println!("pokemon name {}", pokemon.name);
    }

    println!("body = {:?}", body.results);
    Ok(())
}
