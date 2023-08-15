use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
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

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct PokeAbility {
    pub ability: PokeAbilityName,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct PokeAbilityName {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct PokeMove {
    pub r#move: PokeMoveDetails,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct PokeMoveDetails {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct PokeType {
    #[serde(rename = "type")]
    pub poketype: TypeName,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct TypeName {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Stat {
    pub base_stat: u64,
    pub effort: u64,
    pub stat: StatName,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
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
    fn make_request(&self, url: &str) -> Result<PokemonAPIData, reqwest::Error> {
        let req = self.client.get(url);
        match req.send() {
            Ok(response) => {
                let pokemon: PokemonAPIData = response.json().unwrap();
                Ok(pokemon)
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

#[cfg(test)]
mod tests {
    use super::*;

    use httpmock::prelude::*;
    use serde_json::json;

    fn test_expected_response_is_retrieved() {
        let pokemon_types = vec![PokeType {
            poketype: TypeName {
                name: String::from("grass"),
                url: String::from("https://pokeapi.co/api/v2/type/12/"),
            },
        }];
        let pokemon_stats = vec![Stat {
            base_stat: 80,
            effort: 0,
            stat: StatName {
                name: String::from("hp"),
            },
        }];
        let pokemon_abilities = vec![PokeAbility {
            ability: PokeAbilityName {
                name: String::from("overgrow"),
                url: String::from("https://pokeapi.co/api/v2/ability/65/"),
            },
        }];
        let pokemon_moves = vec![PokeMove {
            r#move: PokeMoveDetails {
                name: String::from("swords-dance"),
                url: String::from("https://pokeapi.co/api/v2/move/14/"),
            },
        }];
        let expected = PokemonAPIData {
            name: String::from("bulbasaur"),
            types: pokemon_types,
            stats: pokemon_stats,
            abilities: pokemon_abilities,
            base_experience: 64,
            height: 7,
            moves: pokemon_moves,
            weight: 69,
        };

        let server = MockServer::start();

        let downloader = Downloader::new(3, "test");

        let _hello_mock = server.mock(|when, then| {
            when.method(GET).path("/pokemon/1");
            then.status(200)
                .header("content-type", "text/json")
                .json_body(json!(expected));
        });

<<<<<<< HEAD
        let actual = downloader.get(&(server.base_url() + &String::from("pokemon/1")));
        assert_eq!(actual.unwrap(), expected);
=======
        let actual = downloader.get(&server.url("/pokemon/1")).unwrap();
        assert_eq!(actual.name, "bulbasaur");
        assert_eq!(actual.base_experience, 64);
        assert_eq!(actual, expected);
>>>>>>> Add scraper creation of max stats
    }
}
