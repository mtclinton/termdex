mod downloader;
mod models;
mod schema;
mod scraper;
use crate::pokemon::dsl::pokemon;
use crate::pokemon_type::dsl::pokemon_type;
use crate::ptype::dsl::ptype;
use crate::schema::pokemon::name;
use crate::schema::pokemon::pokemon_id;
use crate::schema::pokemon_type::pokemon_id as pokemon_type_id;
use crate::schema::ptype::id as ptype_id;
use ansi_to_tui::IntoText;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use models::*;
use schema::*;
use scraper::Scraper;
use std::env;
use std::fmt;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

#[derive(Debug)]
struct PokeError(String);

impl fmt::Display for PokeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for PokeError {}

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

/// App holds the state of the application
struct App {
    /// Current value of the input box
    input: Input,
    /// Current search value for pokemon
    pokemon_search: String,
}

impl Default for App {
    fn default() -> App {
        App {
            input: Input::default(),
            pokemon_search: "25".to_string(),
        }
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

struct TUIPokemon {
    tui_pokemon: Pokemon,
    tui_types: Vec<String>,
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

fn get_pokemon(app: &App) -> TUIPokemon {
    match show_pokemon(app.pokemon_search.clone()) {
        Ok(db_result) => match db_result {
            Some(foundpokemon) => {
                let t = get_types(foundpokemon.clone());
                TUIPokemon {
                    tui_pokemon: foundpokemon,
                    tui_types: t,
                }
            }
            None => match show_pokemon("0".to_string()) {
                Ok(notfound) => match notfound {
                    Some(notfound) => TUIPokemon {
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
                Some(notfound) => TUIPokemon {
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
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App, pokemon_db_result: TUIPokemon) {
    // show_border(f, app);
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.size());

    let input = Paragraph::new("")
        .style(Style::default().fg(Color::Red))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(input, chunks[0]);
    let large_sprite = pokemon_db_result.tui_pokemon.large.clone();
    let tui_sprite = large_sprite.into_text();
    let text_sprite = tui_sprite.expect("can't parse sprite");
    let paragraph_sprite = Paragraph::new(text_sprite.clone());

    // add color to not found sprite
    let sprite = paragraph_sprite.style(Style::default().fg(Color::Blue));

    // f.render_widget(paragraph_sprite, chunks[0]);
    let width = chunks[0].width;
    let height = chunks[0].height;
    let sprite_height = text_sprite.clone().lines.len();
    let mut sprite_width = 0;
    for line in text_sprite.clone().lines {
        if line.width() > sprite_width {
            sprite_width = line.width();
        }
    }
    let sprite_x = (width as u16 - sprite_width as u16) / 2;
    let sprite_y = (height as u16 - sprite_height as u16) / 2;
    let area = Rect::new(
        sprite_x,
        sprite_y,
        sprite_width as u16,
        sprite_height as u16,
    );
    f.render_widget(sprite, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(90),
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    let width = chunks[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor

    let scroll = app.input.visual_scroll(width as usize);
    let input = Paragraph::new(app.input.value())
        .style(Style::default().fg(Color::Red))
        // .scroll((0, scroll as u16))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Search Pokemon"),
        );
    f.render_widget(input, chunks[0]);
    // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
    f.set_cursor(
        // Put cursor past the end of the input text
        chunks[0].x + ((app.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
        // Move one line down, from the border to the input line
        chunks[0].y + 1,
    );
    let input = Paragraph::new("")
        .style(Style::default().fg(Color::Red))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(pokemon_db_result.tui_pokemon.name)
                );
    f.render_widget(input, chunks[1]);
    let data_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(chunks[1]);
    let h = vec![
        Span::styled(
            "Experience:",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("{}", pokemon_db_result.tui_pokemon.base_experience),
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        ),
    ];
    let text = Text::from(Spans::from(h));
    let input = Paragraph::new(text)
        .style(Style::default().fg(Color::Red))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(input, data_chunks[0]);
    let h = vec![
        Span::styled(
            "Height:",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("{}", pokemon_db_result.tui_pokemon.height),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
    ];
    let text = Text::from(Spans::from(h));
    let input = Paragraph::new(text)
        .style(Style::default().fg(Color::Red))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(input, data_chunks[1]);
    let h = vec![
        Span::styled(
            "Weight:",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("{}", pokemon_db_result.tui_pokemon.weight),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
    ];
    let text = Text::from(Spans::from(h));
    let input = Paragraph::new(text)
        .style(Style::default().fg(Color::Red))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(input, data_chunks[2]);

    for (index, tui_type) in pokemon_db_result.tui_types.iter().enumerate() {
        let h = vec![
            Span::styled(
                "Type:",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{}", tui_type),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ];
        let text = Text::from(Spans::from(h));
        let input = Paragraph::new(text)
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(input, data_chunks[index + 3]);
    }
}
