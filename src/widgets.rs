use chrono::NaiveDateTime;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};
use tui_textarea::TextArea;

use crate::to_do_widget::main_todo_layout;
use crate::{calendar_widget::main_calendar_layout, weather::Weather};

pub fn app_layout(
    frame: &mut Frame<'_>,
    input_todo_textarea: &mut TextArea<'_>,
    calendar_date: &mut NaiveDateTime,
    is_writing_mode: bool,
    weather_text: &String,
) {
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].to_vec())
        .split(frame.area());

    main_calendar_layout(frame, &main_layout, calendar_date, &weather_text);
    main_todo_layout(
        frame,
        &main_layout,
        input_todo_textarea,
        is_writing_mode,
        calendar_date,
    );
}
