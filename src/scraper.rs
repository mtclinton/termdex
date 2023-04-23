use crossbeam::channel::{Receiver, Sender, TryRecvError};
use crossbeam::thread;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
// use crate::{error, info, warn};
use super::downloader;
use super::models::*;
use super::schema::*;
use rand::Rng;

/// Maximum number of empty recv() from the channel
static MAX_EMPTY_RECEIVES: usize = 10;
/// Sleep duration on empty recv()
static SLEEP_MILLIS: u64 = 100;
static SLEEP_DURATION: Duration = Duration::from_millis(SLEEP_MILLIS);

/// Producer and Consumer data structure. Handles the incoming requests and
/// adds more as new URLs are found
pub struct Scraper {
    transmitter: Sender<(String, u64)>,
    receiver: Receiver<(String, u64)>,
    downloader: downloader::Downloader,
    visited_urls: Mutex<HashSet<String>>,
    pokemon_data: Mutex<Vec<NewPokemon>>,
}

impl Scraper {
    /// Create a new scraper with command line options
    pub fn new() -> Scraper {
        let (tx, rx) = crossbeam::channel::unbounded();

        Scraper {
            downloader: downloader::Downloader::new(3, "termdex"),
            transmitter: tx,
            receiver: rx,
            visited_urls: Mutex::new(HashSet::new()),
            pokemon_data: Mutex::new(Vec::<NewPokemon>::new()),
        }
    }

    /// Push a new URL into the channel
    fn push(transmitter: &Sender<(String, u64)>, url: String, id: u64) {
        if let Err(e) = transmitter.send((url, id)) {
            println!("Couldn't push to channel ! {}", e);
        }
    }

    fn save_pokemon(scraper: &Scraper, data: downloader::PokemonAPIData, id: u64) {
        let l_path = format!("sprites/large/{}", data.name);
        let s_path = format!("sprites/small/{}", data.name);
        let l_data = fs::read_to_string(l_path).expect("Unable to read large sprite");
        let s_data = fs::read_to_string(s_path).expect("Unable to read small sprite");
        let new_pokemon = NewPokemon {
            pokemon_id: id as i32,
            name: data.name,
            large: l_data,
            small: s_data,
            base_experience: data.base_experience as i32,
            height: data.height as i32,
            weight: data.weight as i32,
        };
        scraper.pokemon_data.lock().unwrap().push(new_pokemon);
    }

    /// Process a single URL
    fn handle_url(scraper: &Scraper, url: &str, id: u64) {
        match scraper.downloader.get(url) {
            Ok(response) => {
                Scraper::save_pokemon(scraper, response, id);
            }
            Err(e) => {
                println!("Couldn't download a page, {:?}", e);
            }
        }

        scraper.visited_urls.lock().unwrap().insert(url.to_string());

        println!("Visited: {}", url);
    }

    /// Run through the channel and complete into
    pub fn run(&mut self) {
        for p in Vec::from_iter(1..152).iter() {
            Scraper::push(
                &self.transmitter,
                format!("https://pokeapi.co/api/v2/pokemon/{}", p),
                *p,
            )
        }

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
                                Scraper::handle_url(self_clone, &url, id);
                                self_clone.sleep(&mut rng);
                            }
                        }
                    }
                });
            }
        })
        .unwrap();

        let pokemon = self.pokemon_data.lock().unwrap();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let mut conn = PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));

        diesel::insert_into(pokemon::table)
            .values(&*pokemon)
            .execute(&mut conn);
        let notfound_large = format!("sprites/notfound_large");
        let notfound_small = format!("sprites/notfound_small");
        let notfound_large_data =
            fs::read_to_string(notfound_large).expect("Unable to read large sprite");
        let notfound_small_data =
            fs::read_to_string(notfound_small).expect("Unable to read small sprite");

        let notfound = NewPokemon {
            pokemon_id: 0,
            name: "Not Found".to_string(),
            large: notfound_large_data,
            small: notfound_small_data,
            base_experience: -1,
            height: -1,
            weight: -1,
        };

        diesel::insert_into(pokemon::table)
            .values(&notfound)
            .execute(&mut conn);
    }

    /// Sleep the thread for a variable amount of seconds to avoid getting banned
    fn sleep(&self, rng: &mut rand::rngs::ThreadRng) {
        let base_delay = 1;
        let random_range = 2;

        if base_delay == 0 && random_range == 0 {
            return;
        }

        // delay_range+1 because gen_range is exclusive on the upper limit
        let rand_delay_secs = rng.gen_range(0..random_range + 1);
        let delay_duration = Duration::from_secs(base_delay + rand_delay_secs);
        std::thread::sleep(delay_duration);
    }
}
