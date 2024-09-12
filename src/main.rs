// use cursive::{event::Key, views::Dialog};
use std::io::{self, stdout};

use calendar_data::Calendar;
use chrono::{Months, NaiveDateTime};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    style::{Color, Modifier, Style},
    Terminal,
};

use tui_textarea::{Input, TextArea};
use weather::Weather;
use widgets::app_layout;

pub mod calendar_data;
pub mod calendar_widget;
pub mod logic;
pub mod to_do_data;
pub mod to_do_widget;
pub mod weather;
pub mod widgets;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut input_todo_textarea = TextArea::default();
    let style = Style::default()
        .fg(Color::DarkGray)
        .add_modifier(Modifier::RAPID_BLINK);
    input_todo_textarea.set_line_number_style(style);
    input_todo_textarea.set_placeholder_text("F3 to start entering events ... \n");

    let calendar = Calendar::new();
    let mut calendar_date = calendar.current_date;
    let mut is_writing_mode = false;
    let mut should_quit = false;

    let city_name = "London";
    let weather = Weather::default();
    let weather_text = weather.generate_weather_text(city_name).await?;

    while !should_quit {
        terminal.draw(|f| {
            app_layout(
                f,
                &mut input_todo_textarea,
                &mut calendar_date,
                is_writing_mode,
                &weather_text,
                city_name,
            );
        })?;

        should_quit = handle_events(
            &mut input_todo_textarea,
            &mut calendar_date,
            &mut is_writing_mode,
        )?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events(
    textarea: &mut TextArea,
    calendar_data: &mut NaiveDateTime,
    is_writing_mode: &mut bool,
) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => return Ok(true),
                KeyCode::F(1) => {
                    // Go to the prev month
                    *calendar_data = calendar_data.checked_sub_months(Months::new(1)).unwrap();
                }
                KeyCode::F(2) => {
                    // Go to the next month
                    *calendar_data = calendar_data.checked_add_months(Months::new(1)).unwrap();
                }
                KeyCode::F(3) => {
                    if *is_writing_mode {
                        // If writing mode is ON, F3 turns it OFF
                        *is_writing_mode = false
                    } else {
                        textarea.set_placeholder_text("");
                        // If writing mode is OFF, F3 turns it ON
                        *is_writing_mode = true
                    };
                }
                _ => {
                    if *is_writing_mode {
                        textarea.set_placeholder_text("");
                        // User can only write if the writing_mode is ON
                        textarea.input(Input::from(key));
                    }
                }
            };
        }
    }
    Ok(false)
}
