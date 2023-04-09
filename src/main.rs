mod downloader;
mod models;
mod schema;
mod scraper;
use crate::pokemon::dsl::pokemon;
use crate::schema::pokemon::pokemon_id;
use colored::Colorize;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use models::*;
use rand::seq::SliceRandom;
use schema::*;
use scraper::Scraper;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
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
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    loop {
        println!("Input a pokemon ID");
        let mut input_id = String::new();

        io::stdin()
            .read_line(&mut input_id)
            .expect("Failed to read line");
        let pid: i32 = input_id
            .trim()
            .parse()
            .expect("Pokemon ID must be an integer");
        if pid <= 0 || pid > 151 {
            println!("Invalid ID");
            continue
        }
        let pokemon_result = pokemon
            .filter(pokemon_id.eq(pid))
            .limit(1)
            .load::<Pokemon>(&mut connection)
            .expect("Error loading posts");
        if pokemon_result.len() > 0 {
            println!("{}", pokemon_result[0].sprite);
            println!("{}", pokemon_result[0].name);
        } else{
            println!("Invalid ID");
        }
    }
}
