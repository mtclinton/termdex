use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::Deserialize;
use std::env;
use termdex::models::*;
use termdex::schema::pokemon::dsl::pokemon;
use termdex::schema::pokemon::pokemon_id;
use termdex::schema::pokemon::base_experience;
use termdex::schema::pokemon::weight;
use termdex::schema::pokemon::height;

#[derive(Deserialize)]
pub struct PokemonAPIData {
    pub name: String,
    pub base_experience: u64,
    pub height: u64,
    pub weight: u64,
}


///A Downloader to download web content
pub struct Downloader {
    client: reqwest::blocking::Client,
    tries: usize,
}

impl Downloader {
    /// Create a new Downloader
    pub fn new(tries: usize, user_agent: &str) -> Downloader {
        Downloader {
            client: reqwest::blocking::ClientBuilder::new()
                .cookie_store(true)
                .user_agent(user_agent)
                .build()
                .unwrap(),
            tries,
        }
    }

    ///Download the content at this url
    fn make_request(&self, url: &str) -> Result<PokemonAPIData, reqwest::Error> {
        let req = self.client.get(url);
        match req.send() {
            Ok(mut response) => {
                let pokemon_resp: PokemonAPIData = response.json().unwrap();
                Ok(pokemon_resp)
            }

            Err(e) => {
                println!("Downloader.get() has encountered an error: {}", e);
                Err(e)
            }
        }
    }

    ///Download the content of an url and retries at most 'tries' times on failure
    pub fn get(&self, url: &str) -> Result<PokemonAPIData, reqwest::Error> {
        let mut error: Option<reqwest::Error> = None;
        for _ in 0..self.tries {
            match self.make_request(url) {
                Ok(response) => return Ok(response),
                Err(e) => error = Some(e),
            }
        }

        Err(error.unwrap())
    }
}

fn main() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    let pokemon_db_data = pokemon
        .load::<Pokemon>(&mut connection)
        .expect("Error loading pokemon");

    let downloader = Downloader::new(3, "termdex");

    for p in pokemon_db_data.iter() {
        let url = format!("https://pokeapi.co/api/v2/pokemon/{}", p.name);
        match downloader.get(&url) {
            Ok(response) => {
                diesel::update(pokemon)
                    .filter(pokemon_id.eq(p.pokemon_id))
                    .set((
                        base_experience.eq(response.base_experience as i32),
                        height.eq(response.height as i32),
                        weight.eq(response.weight as i32),
                    ))
                    .execute(&mut connection);
            }
            Err(e) => {
                println!("Couldn't pokemon {}: {:?}", p.name, e);
            }
        }
    }
}
