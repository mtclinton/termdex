use crate::schema::*;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Insertable)]
#[table_name = "pokemon"]
pub struct NewPokemon {
    pub pokemon_id: i32,
    pub name: String,
    pub sprite: String,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Pokemon {
    pub id: i32,
    pub pokemon_id: i32,
    pub name: String,
    pub sprite: Option<String>,
}
