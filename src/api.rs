#[derive(Deserialize)]
struct PokemonAPIData {
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

#[derive(Deserialize)]
struct PokemonData {
    pokemon_id: u32,
    name:   String,
    sprite: String,
    types: Vec<String>,
}


async pub fn search_pokemon(pokemon_id: u32) -> Result<PokemonData, Box<dyn std::error::Error>> {
    let res = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon_id)).await?;

    let body = res.json::<PokemonAPIData>().await?;
    let mut pokemon_types = Vec::new();
    for ptype in body.types {
        pokemon_types.push(ptype.poketype.name);
    }
    PokemonData{
    	pokemon_id: pokemon_id,
    	name: body.names,
    	spirte: '',
    	types: pokemon_types,
    }
}
