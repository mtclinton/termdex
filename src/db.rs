fn setup_db() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
    create(&mut connection);
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