use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use ansi_to_tui::IntoText;
use termdex::models::MaxStats;
use termdex::models::Pokemon;

pub struct TUIPokemon {
    pub tui_pokemon: Pokemon,
    pub tui_types: Vec<String>,
}

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App, pokemon_db_result: TUIPokemon, ms: MaxStats) {
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
    let text_sprite = tui_sprite.expect("can't parse large sprite");
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

    // let render_large_sprite = || -> Result<(), io::Error> {
    if sprite_height < height.into() && sprite_width < width.into() {
        let sprite_x = (width as u16 - sprite_width as u16) / 2;
        let sprite_y = (height as u16 - sprite_height as u16) / 2;
        let area = Rect::new(
            sprite_x,
            sprite_y,
            sprite_width as u16,
            sprite_height as u16,
        );
        f.render_widget(sprite, area);
    } else {
        let small_sprite = pokemon_db_result.tui_pokemon.small.clone();
        let small_tui_sprite = small_sprite.into_text();
        let small_text_sprite = small_tui_sprite.expect("can't parse small sprite");
        let small_paragraph_sprite = Paragraph::new(small_text_sprite.clone());

        // add color to not found sprite
        let small_para_sprite = small_paragraph_sprite.style(Style::default().fg(Color::Blue));

        let small_sprite_height = small_text_sprite.clone().lines.len();
        let mut small_sprite_width = 0;
        for line in small_text_sprite.clone().lines {
            if line.width() > small_sprite_width {
                small_sprite_width = line.width();
            }
        }

        if small_sprite_width < height.into() && small_sprite_height < width.into() {
            let small_sprite_x = (width as u16 - small_sprite_width as u16) / 2;
            let small_sprite_y = (height as u16 - small_sprite_height as u16) / 2;
            let area = Rect::new(
                small_sprite_x,
                small_sprite_y,
                small_sprite_width as u16,
                small_sprite_height as u16,
            );
            f.render_widget(small_para_sprite, area);
        }
    }

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
                .title(capitalize(&pokemon_db_result.tui_pokemon.name)),
        );
    f.render_widget(input, chunks[1]);
    let data_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(40),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .split(chunks[1]);
    let h = vec![Span::styled(
        format!("{}", pokemon_db_result.tui_pokemon.entry),
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )];
    let text = Text::from(Spans::from(h));
    let input = Paragraph::new(text)
        .style(Style::default().fg(Color::Red))
        .block(Block::default().borders(Borders::NONE))
        .wrap(Wrap { trim: true });
    f.render_widget(input, data_chunks[0]);

    let info_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(50)].as_ref())
        .split(data_chunks[1]);
    let height_weight_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(info_chunks[0]);
    let h = vec![Span::styled(
        format!("{}", pokemon_db_result.tui_pokemon.height),
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )];
    let text = Text::from(Spans::from(h));
    let input = Paragraph::new(text)
        .style(Style::default().fg(Color::Red))
        .alignment(Alignment::Center)
        .block(Block::default().title("Height").borders(Borders::ALL));
    f.render_widget(input, height_weight_chunks[0]);
    let w = vec![Span::styled(
        format!("{}", pokemon_db_result.tui_pokemon.weight),
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )];
    let text = Text::from(Spans::from(w));
    let input = Paragraph::new(text)
        .style(Style::default().fg(Color::Red))
        .alignment(Alignment::Center)
        .block(Block::default().title("Weight").borders(Borders::ALL));
    f.render_widget(input, height_weight_chunks[1]);

    if pokemon_db_result.tui_types.len() == 1 {
        let type_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(2)
            .constraints(
                [
                    Constraint::Percentage(25),
                    Constraint::Percentage(50),
                    Constraint::Percentage(25),
                ]
                .as_ref(),
            )
            .split(info_chunks[1]);
        for (index, tui_type) in pokemon_db_result.tui_types.iter().enumerate() {
            let h = vec![Span::styled(
                format!("{}", tui_type),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )];
            let text = Text::from(Spans::from(h));
            let input = Paragraph::new(text)
                .style(Style::default().fg(Color::Red))
                .alignment(Alignment::Center)
                .block(Block::default().title("Type").borders(Borders::ALL));
            f.render_widget(input, type_chunks[1]);
        }
    } else {
        let type_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(info_chunks[1]);
        for (index, tui_type) in pokemon_db_result.tui_types.iter().enumerate() {
            let h = vec![Span::styled(
                format!("{}", tui_type),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )];
            let text = Text::from(Spans::from(h));
            let input = Paragraph::new(text)
                .style(Style::default().fg(Color::Red))
                .alignment(Alignment::Center)
                .block(Block::default().title("Type").borders(Borders::ALL));
            f.render_widget(input, type_chunks[index]);
        }
    }

    // let text = Text::from(Spans::from(h));
    // let input = Paragraph::new(text)
    //     .style(Style::default().fg(Color::Red))
    //     .block(Block::default().borders(Borders::ALL));
    // f.render_widget(input, data_chunks[2]);

    // for (index, tui_type) in pokemon_db_result.tui_types.iter().enumerate() {
    //     let h = vec![
    //         Span::styled(
    //             "Type:",
    //             Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
    //         ),
    //         Span::styled(
    //             format!("{}", tui_type),
    //             Style::default()
    //                 .fg(Color::Green)
    //                 .add_modifier(Modifier::BOLD),
    //         ),
    //     ];
    //     let text = Text::from(Spans::from(h));
    //     let input = Paragraph::new(text)
    //         .style(Style::default().fg(Color::Red))
    //         .block(Block::default().borders(Borders::ALL));
    //     f.render_widget(input, data_chunks[index + 3]);
    // }

    let hp_label = format!("{}", pokemon_db_result.tui_pokemon.hp);
    let hp_gauge = Gauge::default()
        .block(Block::default().title("HP").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Yellow))
        .percent(((pokemon_db_result.tui_pokemon.hp * 100) / ms.hp) as u16)
        .label(hp_label);

    let attack_label = format!("{}", pokemon_db_result.tui_pokemon.attack);
    let attack_gauge = Gauge::default()
        .block(Block::default().title("Attack").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Yellow))
        .percent(((pokemon_db_result.tui_pokemon.attack * 100) / ms.attack) as u16)
        .label(attack_label);

    let defense_label = format!("{}", pokemon_db_result.tui_pokemon.defense);
    let defense_gauge = Gauge::default()
        .block(Block::default().title("Defense").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Yellow))
        .percent(((pokemon_db_result.tui_pokemon.defense * 100) / ms.defense) as u16)
        .label(defense_label);

    let special_attack_label = format!("{}", pokemon_db_result.tui_pokemon.special_attack);
    let special_attack_gauge = Gauge::default()
        .block(
            Block::default()
                .title("Special Attack")
                .borders(Borders::ALL),
        )
        .gauge_style(Style::default().fg(Color::Yellow))
        .percent(((pokemon_db_result.tui_pokemon.special_attack * 100) / ms.special_attack) as u16)
        .label(special_attack_label);

    let special_defense_label = format!("{}", pokemon_db_result.tui_pokemon.special_defense);
    let special_defense_gauge = Gauge::default()
        .block(
            Block::default()
                .title("Special Defense")
                .borders(Borders::ALL),
        )
        .gauge_style(Style::default().fg(Color::Yellow))
        .percent(
            ((pokemon_db_result.tui_pokemon.special_defense * 100) / ms.special_defense) as u16,
        )
        .label(special_defense_label);

    let speed_label = format!("{}", pokemon_db_result.tui_pokemon.speed);
    let speed_gauge = Gauge::default()
        .block(Block::default().title("Speed").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Yellow))
        .percent(((pokemon_db_result.tui_pokemon.speed * 100) / ms.speed) as u16)
        .label(speed_label);

    let guage_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(16),
                Constraint::Percentage(16),
                Constraint::Percentage(16),
                Constraint::Percentage(16),
                Constraint::Percentage(16),
                Constraint::Percentage(16),
                Constraint::Percentage(4),
            ]
            .as_ref(),
        )
        .split(data_chunks[2]);
    f.render_widget(hp_gauge, guage_chunks[0]);
    f.render_widget(attack_gauge, guage_chunks[1]);
    f.render_widget(defense_gauge, guage_chunks[2]);
    f.render_widget(special_attack_gauge, guage_chunks[3]);
    f.render_widget(special_defense_gauge, guage_chunks[4]);
    f.render_widget(speed_gauge, guage_chunks[5]);
}
