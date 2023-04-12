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

    let pokemon_db_data = pokemon
        .load::<Pokemon>(&mut connection)
        .expect("Error loading pokemon");

    for p in pokemon_db_data.iter() {
        let l_path = format!("sprites/large/{}", p.name);
        let s_path = format!("sprites/small/{}", p.name);
        let l_data = fs::read_to_string(l_path).expect("Unable to read large sprite");
        let s_data = fs::read_to_string(s_path).expect("Unable to read small sprite");
        diesel::update(pokemon)
            .filter(pokemon_id.eq(p.pokemon_id))
            .set((large.eq(l_data), small.eq(s_data)))
            .execute(&mut connection);
    }
}
