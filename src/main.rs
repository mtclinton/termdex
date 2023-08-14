mod app;
mod downloader;
mod models;
mod schema;
mod scraper;
mod ui;
use crate::pokemon::dsl::pokemon;
use crate::pokemon_type::dsl::pokemon_type;
use crate::ptype::dsl::ptype;
use crate::schema::pokemon::name;
use crate::schema::pokemon::pokemon_id;
use crate::schema::pokemon_type::pokemon_id as pokemon_type_id;
use crate::schema::ptype::id as ptype_id;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use models::*;
use termdex::models::Pokemon;

use crate::app::App;
use crate::ui::ui;
use schema::*;
use scraper::Scraper;
use std::env;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use tui_input::backend::crossterm::EventHandler;

fn show_pokemon(pokemon_term: String) -> Result<Option<Pokemon>, Box<dyn Error>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
    if pokemon_term.chars().all(char::is_numeric) {
        let pid = pokemon_term.parse::<i32>().unwrap();
        let pokemon_result = pokemon
            .filter(pokemon_id.eq(pid))
            .first(&mut connection)
            .optional()
            .expect("Error loading pokemon");
        Ok(pokemon_result)
    } else {
        let pokemon_result = pokemon
            .filter(name.eq(pokemon_term))
            .first(&mut connection)
            .optional()
            .expect("Error loading pokemon");
        Ok(pokemon_result)
    }
}

fn initialize_pokemon() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
    let results = pokemon
        .limit(1)
        .load::<Pokemon>(&mut connection)
        .expect("Error loading pokemon");
    if results.len() > 0 {
        println!("Finished initializing pokemon database");
    } else {
        println!("Initializing pokemon database");
        let mut scraper = Scraper::new();
        println!("Initializing scraper");
        scraper.run();
        println!("Finished running scraper");

        println!("Finished initializing pokemon database");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    initialize_pokemon();
    //setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::default();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn get_types(spokemon: Pokemon) -> Vec<String> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
    let type_relations = pokemon_type
        .filter(pokemon_type_id.eq(spokemon.pokemon_id))
        .load::<PokemonType>(&mut connection)
        .expect("Error loading type relations");
    let type_relations_ids: Vec<i32> = type_relations.into_iter().map(|x| x.type_id).collect();
    let searched_types = ptype
        .filter(ptype_id.eq_any(type_relations_ids))
        .load::<PType>(&mut connection)
        .expect("Error loading type pokemon types");
    let results: Vec<String> = searched_types.into_iter().map(|x| x.name).collect();
    results
}

fn get_pokemon(app: &App) -> ui::TUIPokemon {
    match show_pokemon(app.pokemon_search.clone()) {
        Ok(db_result) => match db_result {
            Some(foundpokemon) => {
                let t = get_types(foundpokemon.clone());
                ui::TUIPokemon {
                    tui_pokemon: foundpokemon,
                    tui_types: t,
                }
            }
            None => match show_pokemon("0".to_string()) {
                Ok(notfound) => match notfound {
                    Some(notfound) => ui::TUIPokemon {
                        tui_pokemon: notfound,
                        tui_types: vec![],
                    },
                    None => {
                        panic!("Something went wrong querying not found pokemon");
                    }
                },
                _ => {
                    panic!("Something went wrong querying not found pokemon");
                }
            },
        },
        Err(_e) => match show_pokemon("0".to_string()) {
            Ok(notfound) => match notfound {
                Some(notfound) => ui::TUIPokemon {
                    tui_pokemon: notfound,
                    tui_types: vec![],
                },
                None => {
                    panic!("Something went wrong querying not found pokemon");
                }
            },
            _ => {
                panic!("Something went wrong querying not found pokemon");
            }
        },
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        let current_pokemon = get_pokemon(&mut app);
        terminal.draw(|f| ui(f, &mut app, current_pokemon))?;

        if let Event::Key(key) = event::read()? {
            if key.modifiers.is_empty() {
                match key.code {
                    KeyCode::Enter => {
                        let p_input = app.input.value();
                        app.pokemon_search = p_input.to_string();
                        app.input.reset();
                    }
                    KeyCode::Esc => {
                        disable_raw_mode()?;
                        terminal.show_cursor()?;
                        return Ok(());
                    }
                    _ => {
                        app.input.handle_event(&Event::Key(key));
                    }
                }
            } else if key.modifiers == KeyModifiers::CONTROL {
                if let KeyCode::Char('c') = key.code {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    return Ok(());
                }
            }
        }
    }
}
