use std::rc::Rc;

use chrono::{Datelike, NaiveDateTime};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Stylize},
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
    Frame,
};

use crate::calendar_data::Calendar;

fn get_calendar_title_block(month: u32, year: i32) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .fg(Color::Red)
        .add_modifier(Modifier::BOLD)
        .title(format!(" Calendar - {:?} / {:?} ", month, year))
}

fn get_calendar_text(calendar_text: String) -> Paragraph<'static> {
    Paragraph::new(calendar_text)
        .fg(Color::DarkGray)
        .add_modifier(Modifier::BOLD)
        .block(Block::new().padding(Padding::new(5, 5, 2, 2)))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
}

fn get_calendar_month_block() -> Block<'static> {
    Block::default().borders(Borders::ALL).fg(Color::DarkGray)
}

fn get_appointment_text(appointment_text: String) -> Paragraph<'static> {
    Paragraph::new(appointment_text)
        .fg(Color::Green)
        .block(Block::new().padding(Padding::new(5, 2, 2, 2)))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
}

fn get_appointment_block(day: u32, month: u32, year: i32) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .fg(Color::Green)
        .add_modifier(Modifier::BOLD)
        .title(format!(
            " Appointments - {:?} / {:?} / {:?} ",
            day,
            chrono::Month::try_from(month as u8).unwrap(),
            year
        ))
}

fn get_weather_block(city_name: &str) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .fg(Color::LightMagenta)
        .title(format!("Weather for {:?}", city_name))
}

fn get_weather_text(weather_text: String) -> Paragraph<'static> {
    Paragraph::new(weather_text)
        .fg(Color::LightMagenta)
        .block(Block::new().padding(Padding::new(5, 2, 2, 2)))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
}

pub fn main_calendar_layout(
    frame: &mut Frame<'_>,
    main_layout: &Rc<[Rect]>,
    calendar_date: &mut NaiveDateTime,
    weather_text: &String,
    city_name: &str,
) {
    let mut calendar = Calendar::new();
    let day = calendar_date.day();
    let year = calendar_date.year();
    let month = calendar_date.month();

    let calendar_text = calendar.generate_calendar_text(calendar_date);
    let appointment_text = calendar.generate_appointment_text(*calendar_date);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(5),
                Constraint::Percentage(45),
                Constraint::Percentage(50),
            ]
            .to_vec(),
        )
        .split(main_layout[0]);

    let month_weather_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].to_vec())
        .split(layout[1]);

    let calendar_block = get_calendar_title_block(month, year);
    let month_days_block = get_calendar_month_block();
    let appointment_block = get_appointment_block(day, month, year);
    let weather_block = get_weather_block(city_name);

    frame.render_widget(calendar_block.clone(), layout[0]);

    frame.render_widget(month_days_block.clone(), month_weather_layout[0]);
    frame.render_widget(get_calendar_text(calendar_text), month_weather_layout[0]);

    frame.render_widget(weather_block.clone(), month_weather_layout[1]);
    frame.render_widget(
        get_weather_text(weather_text.to_string()),
        month_weather_layout[1],
    );

    frame.render_widget(appointment_block.clone(), layout[2]);
    frame.render_widget(get_appointment_text(appointment_text), layout[2]);
}
