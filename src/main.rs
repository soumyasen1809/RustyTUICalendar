// use cursive::{event::Key, views::Dialog};
use std::io::{self, stdout};

use calendar_data::Calendar;
use chrono::{Duration, NaiveDateTime};
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
use widgets::app_layout;

pub mod calendar_data;
pub mod calendar_widget;
pub mod logic;
pub mod to_do_data;
pub mod to_do_widget;
pub mod widgets;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut input_todo_textarea = TextArea::default();
    let style = Style::default()
        .fg(Color::DarkGray)
        .add_modifier(Modifier::RAPID_BLINK);
    input_todo_textarea.set_line_number_style(style);
    input_todo_textarea.set_placeholder_text("Enter your ToDos or Appointments ... \n");

    let calendar = Calendar::new();
    let mut calendar_date = calendar.current_date;
    let mut should_quit = false;
    while !should_quit {
        terminal.draw(|f| app_layout(f, &mut input_todo_textarea, &mut calendar_date))?;
        should_quit = handle_events(&mut input_todo_textarea, &mut calendar_date)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events(textarea: &mut TextArea, calendar_data: &mut NaiveDateTime) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => return Ok(true),
                KeyCode::F(2) => *calendar_data += Duration::days(30),
                KeyCode::F(1) => *calendar_data -= Duration::days(30),
                _ => {
                    textarea.input(Input::from(key));
                }
            };
        }
    }
    Ok(false)
}
