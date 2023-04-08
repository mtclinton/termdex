mod models;
mod schema;
mod scraper;
mod downloader;
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
use crate::pokemon::dsl::pokemon;
use scraper::Scraper;
mod pokeball;

// fn show_sprite(sprite: &str, poke_type: &str) {
//     let poke_colors = HashMap::from([
//         ("normal", (168, 167, 122)),
//         ("fire", (238, 129, 48)),
//         ("water", (99, 144, 240)),
//         ("electric", (247, 208, 44)),
//         ("grass", (122, 199, 76)),
//         ("ice", (150, 217, 214)),
//         ("fighting", (194, 46, 40)),
//         ("poison", (163, 62, 161)),
//         ("ground", (226, 191, 101)),
//         ("flying", (169, 143, 243)),
//         ("psychic", (249, 85, 135)),
//         ("bug", (166, 185, 26)),
//         ("rock", (182, 161, 54)),
//         ("ghost", (115, 87, 151)),
//         ("dragon", (111, 53, 252)),
//         ("dark", (112, 87, 70)),
//         ("steel", (183, 183, 206)),
//         ("fairy", (214, 133, 173)),
//     ]);
//     let (r, g, b) = poke_colors.get(poke_type).unwrap();
//     println!("{}", sprite.truecolor(*r, *g, *b));
// }
fn initialize_pokemon() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
    let results = pokemon
        .limit(1)
        .load::<Pokemon>(&mut connection)
        .expect("Error loading pokemon");
    if results.len() > 0 {
        println!("Finished initializing pokemon database");
    } else {
        println!("Initializing pokemon database");
        let mut scraper = Scraper::new();
        println!("Initializing scraper");
        scraper.run();
        println!("Finished running scraper");

        println!("Finished initializing pokemon database");

    }
}





fn main() {

    pokeball::show_pokeball();
    println!("Welcome to TermDex");
    initialize_pokemon();
    // loop {
    //     println!("Input a pokemon ID");
    //     let mut pokemon_id = String::new();

    //     io::stdin()
    //         .read_line(&mut pokemon_id)
    //         .expect("Failed to read line");
    //     let pokemon_id: u32 = pokemon_id
    //         .trim()
    //         .parse()
    //         .expect("Pokemon ID must be an integer");
    //     search_pokemon(pokemon_id).await?;
    // }
}







