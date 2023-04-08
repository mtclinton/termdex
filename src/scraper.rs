use diesel::pg::PgConnection;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::env;
use diesel::prelude::*;
use crossbeam::channel::{Receiver, Sender, TryRecvError};
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Mutex;
use serde::Deserialize;

#[derive(Deserialize)]
struct PokemonData {
    pokemon_id: u32,
    name:   String,
    spirte: String,
    types: Vec<String>,
}


/// Producer and Consumer data structure. Handles the incoming requests and
/// adds more as new URLs are found
pub struct Scraper {
    transmitter: Sender<(String, i32>,
    receiver: Receiver<(String, i32)>,
    downloader: downloader::Downloader,
    visited_urls: Mutex<HashSet<String>>,
    path_map: Mutex<HashMap<String, String>>,
    sprites: Mutex<serde_json::Value>,
    connection: Mutex<PgConnection>
}

impl Scraper {
    /// Create a new scraper with command line options
    pub fn new() -> Scraper {
        let (tx, rx) = crossbeam::channel::unbounded();
        let path = "pokemon.json";
	    let data = fs::read_to_string(path).expect("Unable to read file");
	    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	    let mut conn = PgConnection::establish(&database_url)
	        .expect(&format!("Error connecting to {}", database_url));

        Scraper {
            downloader: downloader::Downloader::new(
                3,
                format!("{}","termdex")
            ),
            transmitter: tx,
            receiver: rx,
            visited_urls: Mutex::new(HashSet::new()),
            path_map: Mutex::new(HashMap::new()),
            sprites: Mutex::new(serde_json::from_str(&data).expect("Unable to parse"))
            connection: Mutex::new(conn)
        }
    }

    /// Push a new URL into the channel
    fn push(transmitter: &Sender<(&str, i32, i32)>, url: String, id: i32) {
        if let Err(e) = transmitter.send((url, id)) {
            error!("Couldn't push to channel ! {}", e);
        }
    }


	fn get_sprite(scraper: &Scraper, pokemon_id: u32) -> String{
		let sprites = scraper.sprites.lock().unwrap();
		sprites[format!("{}", pokemon_id)].as_str().unwrap().to_string()

	}

    fn save_pokemon(scraper: &Scraper, data: PokemonAPIData, id: u64):
    	let sprite = scraper.get_sprite(id);
    	let new_pokemon = NewPokemon {
	        pokemon_id: id,
	        name: PokemonAPIData.name,
	        sprite: sprite,
	    };
	    conn = scraper.connection.lock().unwrap();

	    let inserted_row = diesel::insert_into(pokemon::table)
	        .values(&new_pokemon)
	        .get_result::<Pokemon>(conn);


    /// Process a single URL
    fn handle_url(
        scraper: &Scraper,
        transmitter: &Sender<(String, i32>,
        url: String,
    ) {
        match scraper.downloader.get(&url) {
            Ok(response) => {
                scraper.save_pokemon(response);
            }
            Err(e) => {
                if !scraper.args.continue_on_error {
                    error!("Couldn't download a page, {:?}", e);
                } else {
                    warn!("Couldn't download a page, {:?}", e);
                }
            }
        }

        scraper.visited_urls.lock().unwrap().insert(url);

        if scraper.args.verbose {
            info!("Visited: {}", url);
        }
    }



    /// Run through the channel and complete it
    pub fn run(&mut self) {
        /* Push the origin URL and depth (0) through the channel */
        (1..151).map(
            |p| Scraper::push(&self.transmitter, format!("https://pokeapi.co/api/v2/pokemon/{}", p), p)

        )
        

        thread::scope(|thread_scope| {
            for _ in 0..8 {
                let tx = self.transmitter.clone();
                let rx = self.receiver.clone();
                let self_clone = &self;

                thread_scope.spawn(move |_| {
                    let mut counter = 0;
                    // For a random delay
                    let mut rng = rand::thread_rng();

                    while counter < MAX_EMPTY_RECEIVES {
                        match rx.try_recv() {
                            Err(e) => match e {
                                TryRecvError::Empty => {
                                    counter += 1;
                                    std::thread::sleep(SLEEP_DURATION);
                                }
                                TryRecvError::Disconnected => panic!("{}", e),
                            },
                            Ok((url, id)) => {
                                counter = 0;
                                Scraper::handle_url(self_clone, &tx, url, id);
                                self_clone.sleep(&mut rng);
                            }
                        }
                    }
                });
            }
        })
        .unwrap();
    }


    /// Sleep the thread for a variable amount of seconds to avoid getting banned
    fn sleep(&self, rng: &mut rand::rngs::ThreadRng) {
        let base_delay = self.args.delay;
        let random_range = self.args.random_range;

        if base_delay == 0 && random_range == 0 {
            return;
        }

        // delay_range+1 because gen_range is exclusive on the upper limit
        let rand_delay_secs = rng.gen_range(0..random_range + 1);
        let delay_duration = time::Duration::from_secs(base_delay + rand_delay_secs);
        std::thread::sleep(delay_duration);
    }

}