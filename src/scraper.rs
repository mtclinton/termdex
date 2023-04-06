use diesel::pg::PgConnection;

/// Producer and Consumer data structure. Handles the incoming requests and
/// adds more as new URLs are found
pub struct Scraper {
    args: args::Args,
    transmitter: Sender<(String)>,
    receiver: Receiver<(String)>,
    downloader: downloader::Downloader,
    visited_urls: Mutex<HashSet<String>>,
    path_map: Mutex<HashMap<String, String>>,
    sprites: Mutex<serde_json::Value>,
    connection: Mutex<PgConnection>
}

impl Scraper {
    /// Create a new scraper with command line options
    pub fn new(args: args::Args) -> Scraper {
        let (tx, rx) = crossbeam::channel::unbounded();

        let mut args = args;
        let path = "pokemon.json";
	    let data = fs::read_to_string(path).expect("Unable to read file");
	    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	    let mut conn = PgConnection::establish(&database_url)
	        .expect(&format!("Error connecting to {}", database_url));

        Scraper {
            downloader: downloader::Downloader::new(
                args.tries,
                &args.user_agent,
                args.disable_certs_checks,
                &args.auth,
                &args.origin,
            ),
            args,
            transmitter: tx,
            receiver: rx,
            visited_urls: Mutex::new(HashSet::new()),
            path_map: Mutex::new(HashMap::new()),
            sprites: Mutex::new(serde_json::from_str(&data).expect("Unable to parse"));
            connection: Mutex::new(conn)
        }
    }


	fn get_sprite(scraper: &Scraper, pokemon_id: u32) -> String{
		let sprites = scraper.sprites.lock().unwrap();
		sprites[format!("{}", pokemon_id)].as_str().unwrap().to_string()

	}

    fn save_pokemon(scraper: &Scraper, data: PokemonAPIData, id: u64):
    	fmt!("{:#?}", data);


    /// Process a single URL
    fn handle_url(
        scraper: &Scraper,
        transmitter: &Sender<(String>,
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
        	|p| Scraper::push(&self.transmitter, format!("https://pokeapi.co/api/v2/pokemon/{}", p)

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
                            Ok((url)) => {
                                counter = 0;
                                Scraper::handle_url(self_clone, &tx, url);
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