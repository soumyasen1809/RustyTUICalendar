use std::rc::Rc;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Stylize},
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};
use tui_textarea::TextArea;

use crate::to_do_data::ToDoList;

fn get_todo_title_block() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .fg(Color::Magenta)
        .add_modifier(Modifier::BOLD)
        .title(format!(" To-Do "))
}

fn get_todo_user_input_block() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .fg(Color::DarkGray)
        // .add_modifier(Modifier::BOLD)
        .title(format!(" User Input "))
}

fn get_todo_list_text(todo_list_text: String) -> Paragraph<'static> {
    Paragraph::new(todo_list_text)
        .fg(Color::Magenta)
        .block(Block::new().padding(Padding::new(5, 1, 2, 1)))
        .alignment(Alignment::Left)
}

pub fn main_todo_layout(
    frame: &mut Frame,
    main_layout: &Rc<[Rect]>,
    input_todo_textarea: &TextArea,
) {
    let mut todolist = ToDoList::new();
    let todo_list_text = todolist.generate_todo_text();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].to_vec())
        .split(main_layout[1]);

    let todo_block = get_todo_title_block();
    frame.render_widget(todo_block.clone(), layout[0]);
    frame.render_widget(get_todo_list_text(todo_list_text), layout[0]);

    let user_input_block = get_todo_user_input_block();
    frame.render_widget(user_input_block, layout[1]);
    frame.render_widget(input_todo_textarea, layout[1]);
}
