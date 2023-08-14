use crate::schema::*;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Debug, Insertable, PartialEq)]
#[table_name = "pokemon"]
pub struct NewPokemon {
    pub pokemon_id: i32,
    pub name: String,
    pub large: String,
    pub small: String,
    pub base_experience: i32,
    pub height: i32,
    pub weight: i32,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub special_attack: i32,
    pub special_defense: i32,
    pub speed: i32,
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
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub special_attack: i32,
    pub special_defense: i32,
    pub speed: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "pokemon_type"]
pub struct NewPokemonType {
    pub pokemon_id: i32,
    pub type_id: i32,
}

#[derive(Debug, Queryable, Serialize, Clone)]
pub struct PokemonType {
    pub id: i32,
    pub pokemon_id: i32,
    pub type_id: i32,
}

#[derive(Debug, Insertable, Eq, Hash, PartialEq, Clone)]
#[table_name = "ptype"]
pub struct NewPType {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Queryable, Serialize, Clone)]
pub struct PType {
    pub id: i32,
    pub name: String,
    pub url: String,
}
