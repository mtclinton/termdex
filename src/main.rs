use std::io;
use serde::Deserialize;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
mod pokeball;

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
}


async fn search_pokemon(pokemon_id: u32) -> Result<(), Box<dyn std::error::Error>> {
    let res = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{}",pokemon_id)).await?;

    let body = res.json::<Data>().await?;
    println!("Pokemon: {}", body.name);
    Ok(())
}
