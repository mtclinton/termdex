use ansi_to_tui::IntoText;
use chrono::prelude::*;
use crossterm::{
    event::{self, Event , read, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rand::{distributions::Alphanumeric, prelude::*};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use termdex::models::*;
use termdex::schema::pokemon::dsl::pokemon;
use termdex::schema::pokemon::pokemon_id;
use thiserror::Error;
use tui::text::Text;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Terminal,
};


#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

// enum Event<I> {
//     Input(I),
//     Tick,
// }

fn show_pokemon() -> Result<Vec<Pokemon>, Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
    let pokemon_result = pokemon
        .filter(pokemon_id.eq(4))
        .limit(1)
        .load::<Pokemon>(&mut connection)
        .expect("Error loading posts");
    Ok(pokemon_result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");

    // let (tx, rx) = mpsc::channel();
    // let tick_rate = Duration::from_millis(200);
    // thread::spawn(move || {
    //     let mut last_tick = Instant::now();
    //     loop {
    //         let timeout = tick_rate
    //             .checked_sub(last_tick.elapsed())
    //             .unwrap_or_else(|| Duration::from_secs(0));

    //         if event::poll(timeout).expect("poll works") {
    //             if let CEvent::Key(key) = event::read().expect("can read events") {
    //                 tx.send(Event::Input(key)).expect("can send events");
    //             }
    //         }

    //         if last_tick.elapsed() >= tick_rate {
    //             if let Ok(_) = tx.send(Event::Tick) {
    //                 last_tick = Instant::now();
    //             }
    //         }
    //     }
    // });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;



    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(size);
            let pokemon_db_result = show_pokemon().expect("can't fetch pokmeon");
            let large_sprite = pokemon_db_result[0].large.clone();
            let tui_sprite = large_sprite.into_text();
            let text_sprite = tui_sprite.expect("can't parse sprite");
            let paragraph_sprite = Paragraph::new(text_sprite);
            rect.render_widget(paragraph_sprite, chunks[0]);
        })?;

        // match rx.recv()? {
        //     Event::Input(event) => match event.code {
        //         KeyCode::Char('q') => {
        //             disable_raw_mode()?;
        //             terminal.show_cursor()?;
        //             break;
        //         }
        //         _ => {
        //             textarea.input(event);
        //         }
        //     },if let Event::Key(key) = read()? {
        if let Event::Key(key) = read()? {
            // Your own key mapping to break the event loop
            if key.code == KeyCode::Esc {
                disable_raw_mode()?;
                terminal.show_cursor()?;
                break;
            }
            // `TextArea::input` can directly handle key events from backends and update the editor state
            // textarea.input(key);
        }

    }

    Ok(())
}
