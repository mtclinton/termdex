use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::Deserialize;
use std::env;
use termdex::models::*;
use termdex::schema::pokemon::attack;
use termdex::schema::pokemon::defense;
use termdex::schema::pokemon::dsl::pokemon;
use termdex::schema::pokemon::hp;
use termdex::schema::pokemon::pokemon_id;
use termdex::schema::pokemon::special_attack;
use termdex::schema::pokemon::special_defense;
use termdex::schema::pokemon::speed;

#[derive(Deserialize)]
pub struct PokemonAPIData {
    pub name: String,
    pub types: Vec<PokeType>,
    pub stats: Vec<Stat>,
    pub abilities: Vec<PokeAbility>,
    pub base_experience: u64,
    pub height: u64,
    pub moves: Vec<PokeMove>,
    pub weight: u64,
}
pub struct StatValues {
    pub hp: u64,
    pub attack: u64,
    pub defense: u64,
    pub special_attack: u64,
    pub special_defense: u64,
    pub speed: u64,
}

impl Default for StatValues {
    fn default() -> StatValues {
        StatValues {
            hp: 0,
            attack: 0,
            defense: 0,
            special_attack: 0,
            special_defense: 0,
            speed: 0,
        }
    }
}

#[derive(Deserialize)]
pub struct PokeAbility {
    pub ability: PokeAbilityName,
}

#[derive(Deserialize)]
pub struct PokeAbilityName {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize)]
pub struct PokeMove {
    pub r#move: PokeMoveDetails,
}

#[derive(Deserialize)]
pub struct PokeMoveDetails {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize)]
pub struct PokeType {
    #[serde(rename = "type")]
    pub poketype: TypeName,
}

#[derive(Deserialize)]
pub struct TypeName {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize)]
pub struct Stat {
    pub base_stat: u64,
    effort: u64,
    pub stat: StatName,
}

#[derive(Deserialize)]
pub struct StatName {
    pub name: String,
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
    fn make_request(&self, url: &str) -> Result<StatValues, reqwest::Error> {
        let req = self.client.get(url);
        println!("{}", url);
        match req.send() {
            Ok(mut response) => {
                let mut pokemon_resp: PokemonAPIData = response.json().unwrap();
                let mut statvalues = StatValues::default();
                for stat in pokemon_resp.stats.iter() {
                    match &*stat.stat.name {
                        "hp" => statvalues.hp = stat.base_stat,
                        "attack" => statvalues.attack = stat.base_stat,
                        "defense" => statvalues.defense = stat.base_stat,
                        "special-attack" => statvalues.special_attack = stat.base_stat,
                        "special-defense" => statvalues.special_defense = stat.base_stat,
                        "speed" => statvalues.speed = stat.base_stat,
                        _ => println!("Unknown stat: {}", stat.stat.name),
                    }
                }
                Ok(statvalues)
            }

            Err(e) => {
                println!("Stat scraper has encountered an error: {}", e);
                Err(e)
            }
        }
    }

    ///Download the content of an url and retries at most 'tries' times on failure
    pub fn get(&self, url: &str) -> Result<StatValues, reqwest::Error> {
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
        if p.name == "Not Found" {
            diesel::update(pokemon)
                .filter(pokemon_id.eq(p.pokemon_id))
                .set((
                    hp.eq(0),
                    attack.eq(0),
                    defense.eq(0),
                    special_attack.eq(0),
                    special_defense.eq(0),
                    speed.eq(0),
                ))
                .execute(&mut connection);
        } else {
            let url = format!("https://pokeapi.co/api/v2/pokemon/{}", p.name);
            match downloader.get(&url) {
                Ok(response) => {
                    diesel::update(pokemon)
                        .filter(pokemon_id.eq(p.pokemon_id))
                        .set((
                            hp.eq(response.hp as i32),
                            attack.eq(response.attack as i32),
                            defense.eq(response.defense as i32),
                            special_attack.eq(response.special_attack as i32),
                            special_defense.eq(response.special_defense as i32),
                            speed.eq(response.speed as i32),
                        ))
                        .execute(&mut connection);
                }
                Err(e) => {
                    println!("Couldn't pokemon {}: {:?}", p.name, e);
                }
            }
        }
    }
}
