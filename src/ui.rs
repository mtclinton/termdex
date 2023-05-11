use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use ansi_to_tui::IntoText;
use termdex::models::Pokemon;

pub struct TUIPokemon {
    pub tui_pokemon: Pokemon,
    pub tui_types: Vec<String>,
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App, pokemon_db_result: TUIPokemon) {
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
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
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
                .title(pokemon_db_result.tui_pokemon.name),
        );
    f.render_widget(input, chunks[1]);
    let data_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
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
