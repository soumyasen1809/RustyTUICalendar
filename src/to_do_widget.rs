use std::rc::Rc;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Stylize},
    widgets::{Block, Borders},
    Frame,
};

fn get_todo_title_block() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .fg(Color::Magenta)
        .add_modifier(Modifier::BOLD)
        .title(format!(" To-Do "))
}

pub fn main_todo_layout(frame: &mut Frame, main_layout: &Rc<[Rect]>) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].to_vec())
        .split(main_layout[1]);

    let calendar_block = get_todo_title_block();
    frame.render_widget(calendar_block.clone(), layout[0]);
}
