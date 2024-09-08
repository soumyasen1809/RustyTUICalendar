use chrono::NaiveDateTime;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};
use tui_textarea::TextArea;

use crate::calendar_widget::main_calendar_layout;
use crate::to_do_widget::main_todo_layout;

pub fn app_layout(
    frame: &mut Frame,
    input_todo_textarea: &mut TextArea,
    calendar_date: &mut NaiveDateTime,
) {
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].to_vec())
        .split(frame.area());

    main_calendar_layout(frame, &main_layout, calendar_date);
    main_todo_layout(frame, &main_layout, input_todo_textarea);
}
