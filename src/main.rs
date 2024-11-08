use std::{error::Error, io};

use chrono::prelude::*;
use reqwest::Client;
use serde_json::Value;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    DefaultTerminal,
};

use rand::Rng;

fn run(mut terminal: DefaultTerminal, event: &str, birth: &str, death: &str) -> io::Result<()> {
    let event_area = Rect::new(18, 0, 50, 10);
    let birth_area = Rect::new(71, 0, 50, 10);
    let death_area = Rect::new(124, 0, 50, 10);
    loop {
        terminal.draw(|frame| {
            let e = Paragraph::new(event)
                .style(Style::default().fg(Color::Yellow))
                .wrap(Wrap { trim: true })
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Event")
                        .border_type(BorderType::Rounded),
                );
            frame.render_widget(e, event_area);
            let b = Paragraph::new(birth)
                .style(Style::default().fg(Color::Yellow))
                .wrap(Wrap { trim: true })
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Birth")
                        .border_type(BorderType::Rounded),
                );
            frame.render_widget(b, birth_area);
            let d = Paragraph::new(death)
                .style(Style::default().fg(Color::Yellow))
                .wrap(Wrap { trim: true })
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Death")
                        .border_type(BorderType::Rounded),
                );
            frame.render_widget(d, death_area);
        })?;
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let dt = Utc::now();
    let url = format!(
        "http://history.muffinlabs.com/date/{}/{}",
        dt.month(),
        dt.day()
    );

    let response = client.get(&url).send().await?;
    if !response.status().is_success() {
        println!("Unable to fetch data");
        return Err("Unable to fetch data".into());
    }

    let json = response.json::<Value>().await?;
    // println!("{:?}", json["data"]["Events"][0]);
    let events = json["data"]["Events"].as_array().unwrap();
    let births = json["data"]["Births"].as_array().unwrap();
    let deaths = json["data"]["Deaths"].as_array().unwrap();
    let mut rng = rand::thread_rng();
    let n: usize = rng.gen_range(0..events.len());
    let event_message = events[n]["text"].as_str().unwrap();
    let mut rng = rand::thread_rng();
    let n: usize = rng.gen_range(0..births.len());
    let birth_message = births[n]["text"].as_str().unwrap();
    let mut rng = rand::thread_rng();
    let n: usize = rng.gen_range(0..deaths.len());
    let death_message = deaths[n]["text"].as_str().unwrap();

    let mut terminal = ratatui::init();
    terminal.clear()?;
    let _app_result = run(terminal, event_message, birth_message, death_message);
    ratatui::restore();
    Ok(())
}
