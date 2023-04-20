mod downloader;
mod models;
mod schema;
mod scraper;
use crate::pokemon::dsl::pokemon;
use crate::schema::pokemon::name;
use crate::schema::pokemon::pokemon_id;
use ansi_to_tui::IntoText;
use chrono::prelude::*;
use colored::Colorize;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use models::*;
use rand::seq::SliceRandom;
use schema::*;
use scraper::Scraper;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

enum InputMode {
    Normal,
    Editing,
}

fn show_pokemon(pokemon_term: String) -> Result<Vec<Pokemon>, Box<dyn Error>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
    if pokemon_term.chars().all(char::is_numeric) {
        let pid = pokemon_term.parse::<i32>().unwrap();
        let pokemon_result = pokemon
            .filter(pokemon_id.eq(pid))
            .limit(1)
            .load::<Pokemon>(&mut connection)
            .expect("Error loading posts");
        Ok(pokemon_result)
    } else {
        let pokemon_result = pokemon
            .filter(name.eq(pokemon_term))
            .limit(1)
            .load::<Pokemon>(&mut connection)
            .expect("Error loading posts");
        Ok(pokemon_result)
    }
}

/// App holds the state of the application
struct App {
    /// Current value of the input box
    input: Input,
    /// Current input mode
    input_mode: InputMode,
    /// Current search value for pokemon
    pokemon_search: String,
}

impl Default for App {
    fn default() -> App {
        App {
            input: Input::default(),
            input_mode: InputMode::Normal,
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        let p_input = app.input.value();
                        app.pokemon_search = p_input.to_string();
                        app.input.reset();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {
                        app.input.handle_event(&Event::Key(key));
                    }
                },
            }
        }
    }
}

fn show_border<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.size());
    let large_border = "[38;2;220;20;60m█"
        .to_string()
        .repeat(chunks[0].width as usize);
    let border_text = large_border.into_text();
    let border_enc = border_text.expect("can't parse border");
    let border_tui = Paragraph::new(border_enc.clone());
    let area = Rect::new(0, 0, chunks[0].width, 1);
    f.render_widget(border_tui.clone(), area);
    for y in 0..chunks[0].height {
        let border = "[38;2;220;20;60m█".to_string().repeat(6 as usize);
        let b = border.into_text();
        let b2 = b.expect("can't parse border");
        let b3 = Paragraph::new(b2.clone());
        let m = y as u16;
        let area = Rect::new(0, m, 6, 1);
        f.render_widget(b3.clone(), area);
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    // show_border(f, app);
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.size());

    let pokemon_db_result = show_pokemon(app.pokemon_search.clone()).expect("can't fetch pokmeon");
    if pokemon_db_result.len() > 0 {
        let large_sprite = pokemon_db_result[0].large.clone();
        let tui_sprite = large_sprite.into_text();
        let text_sprite = tui_sprite.expect("can't parse sprite");
        let paragraph_sprite = Paragraph::new(text_sprite.clone())
            .style(Style::default().fg(Color::Blue))
            .block(Block::default().borders(Borders::ALL));

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
        f.render_widget(paragraph_sprite, area);
    } else {
        let paragraph_sprite = Paragraph::new("Pokemon not found.");
        f.render_widget(paragraph_sprite, chunks[0]);
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .split(chunks[1]);

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled(
                    "q",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::raw(" to exit, "),
                Span::styled(
                    "e",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled(
                    "Esc",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::raw(" to stop editing, "),
                Span::styled(
                    "Enter",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let width = chunks[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor

    let scroll = app.input.visual_scroll(width as usize);
    let input = Paragraph::new(app.input.value())
        .style(match app.input_mode {
            InputMode::Normal => Style::default().fg(Color::Red),
            InputMode::Editing => Style::default().fg(Color::Red),
        })
        .scroll((0, scroll as u16))
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[1]);
    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[1].x + ((app.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + 1,
            )
        }
    }
}
