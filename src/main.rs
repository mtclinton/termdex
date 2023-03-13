use std::io;
use serde::Deserialize;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
use std::fs;

mod pokeball;

fn show_pokemon(pokemon_id: u32) {
    let path = "pokemon.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");
    print!("{}", res[format!("{}",pokemon_id)].as_str().unwrap())
}


fn setup_db() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pokeball::show_pokeball();
    println!("Welcome to TermDex");
    println!("Input a pokemon ID");
    let mut pokemon_id = String::new();

    io::stdin()
        .read_line(&mut pokemon_id)
        .expect("Failed to read line");
    let pokemon_id: u32 = pokemon_id.trim().parse().expect("Pokemon ID must be an integer");
    search_pokemon(pokemon_id).await?;
    Ok(())
}

#[derive(Deserialize)]
struct Data {
    name: String,
    types: Vec<PokeType>,
}

#[derive(Deserialize)]
struct PokeType {
    #[serde(rename = "type")]
    poketype: TypeName,
}

#[derive(Deserialize)]
struct TypeName {
    name: String,
}

async fn search_pokemon(pokemon_id: u32) -> Result<(), Box<dyn std::error::Error>> {
    show_pokemon(pokemon_id);
    let res = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{}",pokemon_id)).await?;

    let body = res.json::<Data>().await?;
    let mut pokemon_types = Vec::new();
    for ptype in body.types {
        pokemon_types.push(ptype.poketype.name);
    }
    println!("Pokemon: {}", body.name);
    println!("Pokemon type: {:#?}", pokemon_types);    
    Ok(())
}
