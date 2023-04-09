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
            let file_path = format!("sprites/large/{}", pokemon_result[0].name);
            let sprite = fs::read_to_string(file_path).expect("Should have been able to read the sprite file");
            println!("{}", sprite);
            println!("{}", pokemon_result[0].name);
        } else{
            println!("Invalid ID");
        }
    }
}
