
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
    spirte: String,
    types: Vec<String>,
}


///A Downloader to download web content
pub struct Downloader {
    client: reqwest::blocking::Client,
    tries: usize,
}

impl Downloader {
    /// Create a new Downloader
    pub fn new(
        tries: usize,
        user_agent: &str,
    ) -> Downloader {
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
            Ok(mut data) => {
                let pokemon: PokemonAPIData = response.json().unwrap();
                Ok(pokemon)
            }

            Err(e) => {
                warn!("Downloader.get() has encountered an error: {}", e);
                Err(e)
            }
        }
    }

    ///Download the content of an url and retries at most 'tries' times on failure
    pub fn get(&self, url: &Url) -> Result<PokemonAPIData, reqwest::Error> {
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

