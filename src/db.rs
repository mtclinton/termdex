



fn setup_db(connection: &mut PgConnection) {
    let pokemon_data = pokemon
        .load::<Pokemon>(connection)
        .expect("Error loading pokemon");
    if pokemon_data.len() == 0 {
        initialize_pokemon(&mut connection);
    }
    
}

fn create_pokemon(connection: &mut PgConnection, pokemon_id: u32, name: &str, sprite: &str) {
    let new_pokemon = NewPokemon {
        pokemon_id: pokemon_id,
        name: name,
        sprite: sprite,
    };

    let inserted_row = diesel::insert_into(pokemon::table)
        .values(&new_pokemon)
        .get_result::<Pokemon>(connection);

    println!("{:?}", inserted_row);
}