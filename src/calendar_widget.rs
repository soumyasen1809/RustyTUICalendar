use std::rc::Rc;

use chrono::Datelike;
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

pub fn main_calendar_layout(frame: &mut Frame, main_layout: &Rc<[Rect]>) {
    let mut calendar = Calendar::new();
    let day = calendar.current_date.day();
    let year = calendar.current_date.year();
    let month = calendar.current_date.month();

    let calendar_text = calendar.generate_calendar_text();
    let appointment_text = calendar.generate_appointment_text(calendar.current_date);

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

    let calendar_block = get_calendar_title_block(month, year);
    let month_days_block = get_calendar_month_block();
    let appointment_block = get_appointment_block(day, month, year);

    frame.render_widget(calendar_block.clone(), layout[0]);

    frame.render_widget(month_days_block.clone(), layout[1]);
    frame.render_widget(get_calendar_text(calendar_text), layout[1]);

    frame.render_widget(appointment_block.clone(), layout[2]);
    frame.render_widget(get_appointment_text(appointment_text), layout[2]);
}
