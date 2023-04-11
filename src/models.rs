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
}

#[derive(Debug, Queryable, Serialize)]
pub struct Pokemon {
    pub id: i32,
    pub pokemon_id: i32,
    pub name: String,
    pub large: Option<String>,
    pub small: Option<String>,

}
