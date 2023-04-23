use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
extern crate termdex;
use serde_json::Value;
use termdex::models::*;
use termdex::schema::pokemon::dsl::pokemon;
use termdex::schema::pokemon::large;
use termdex::schema::pokemon::pokemon_id;
use termdex::schema::pokemon::small;

fn main() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    let notfound_large = format!("sprites/notfound_large");
    let notfound_small = format!("sprites/notfound_small");
    let notfound_large_data = fs::read_to_string(notfound_large).expect("Unable to read large sprite");
    let notfound_small_data = fs::read_to_string(notfound_small).expect("Unable to read small sprite");

    let notfound = NewPokemon {
        id: 0, 
        pokemon_id: 0,
        name: "Not Found".to_string(),
        large: notfound_large_data,
        small: notfound_small_data,
        base_experience: -1,
        height: -1,
        weight: -1,
    };

    diesel::insert_into(pokemon)
        .values(&notfound)
        .execute(&mut connection)
        .expect("Error saving notfound");
}
