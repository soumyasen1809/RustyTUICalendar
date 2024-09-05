use std::rc::Rc;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Stylize},
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};

use crate::to_do_data::ToDoList;

fn get_todo_title_block() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .fg(Color::Magenta)
        .add_modifier(Modifier::BOLD)
        .title(format!(" To-Do "))
}

fn get_todo_list_text(todo_list_text: String) -> Paragraph<'static> {
    Paragraph::new(todo_list_text)
        .fg(Color::Magenta)
        .block(Block::new().padding(Padding::new(5, 1, 2, 1)))
        .alignment(Alignment::Left)
}

pub fn main_todo_layout(frame: &mut Frame, main_layout: &Rc<[Rect]>) {
    let mut todolist = ToDoList::new();
    let todo_list_text = todolist.generate_todo_text();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].to_vec())
        .split(main_layout[1]);

    let calendar_block = get_todo_title_block();
    frame.render_widget(calendar_block.clone(), layout[0]);
    frame.render_widget(get_todo_list_text(todo_list_text), layout[0]);
}
