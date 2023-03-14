mod models;
mod schema;

use colored::Colorize;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use models::*;
use schema::*;
mod pokeball;

fn show_sprite(sprite: &str, poke_type: &str) {
    let poke_colors = HashMap::from([
        ("normal", (168, 167, 122)),
        ("fire", (238, 129, 48)),
        ("water", (99, 144, 240)),
        ("electric", (247, 208, 44)),
        ("grass", (122, 199, 76)),
        ("ice", (150, 217, 214)),
        ("fighting", (194, 46, 40)),
        ("poison", (163, 62, 161)),
        ("ground", (226, 191, 101)),
        ("flying", (169, 143, 243)),
        ("psychic", (249, 85, 135)),
        ("bug", (166, 185, 26)),
        ("rock", (182, 161, 54)),
        ("ghost", (115, 87, 151)),
        ("dragon", (111, 53, 252)),
        ("dark", (112, 87, 70)),
        ("steel", (183, 183, 206)),
        ("fairy", (214, 133, 173)),
    ]);
    let (r, g, b) = poke_colors.get(poke_type).unwrap();
    println!("{}", sprite.truecolor(*r, *g, *b));
}

fn show_pokemon(pokemon_id: u32) -> String {
    let path = "pokemon.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");
    let sprite: String = res[format!("{}", pokemon_id)].as_str().unwrap().to_string();
    return sprite;
}

fn setup_db() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
    create(&mut connection);
}

fn create(connection: &mut PgConnection) {
    let new_pokemon = NewPokemon {
        pokemon_id: 1,
        name: "blah".to_string(),
        sprite: "".to_string(),
    };

    let inserted_row = diesel::insert_into(pokemon::table)
        .values(&new_pokemon)
        .get_result::<Pokemon>(connection);

    println!("{:?}", inserted_row);
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pokeball::show_pokeball();
    println!("Welcome to TermDex");
    setup_db();
    loop {
        println!("Input a pokemon ID");
        let mut pokemon_id = String::new();

        io::stdin()
            .read_line(&mut pokemon_id)
            .expect("Failed to read line");
        let pokemon_id: u32 = pokemon_id
            .trim()
            .parse()
            .expect("Pokemon ID must be an integer");
        search_pokemon(pokemon_id).await?;
    }
}

#[derive(Deserialize)]
struct PokemonData {
    pokemon_id: u32,
    name:   String,
    spirte: String,
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
    let res = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon_id)).await?;

    let body = res.json::<Data>().await?;
    let mut pokemon_types = Vec::new();
    for ptype in body.types {
        pokemon_types.push(ptype.poketype.name);
    }
    show_sprite(
        &show_pokemon(pokemon_id),
        pokemon_types
            .choose(&mut rand::thread_rng())
            .unwrap()
            .as_str(),
    );
    println!("Pokemon: {}", body.name);
    Ok(())
}
