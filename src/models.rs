use crate::schema::*;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Debug, Insertable)]
#[table_name = "pokemon"]
pub struct NewPokemon {
    pub pokemon_id: i32,
    pub name: String,
    pub large: String,
    pub small: String,
    pub base_experience: i32,
    pub height: i32,
    pub weight: i32,
}

#[derive(Debug, Queryable, Serialize, Clone)]
pub struct Pokemon {
    pub id: i32,
    pub pokemon_id: i32,
    pub name: String,
    pub large: String,
    pub small: String,
    pub base_experience: i32,
    pub height: i32,
    pub weight: i32,
}
