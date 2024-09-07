use std::rc::Rc;

use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Stylize},
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};
use tui_textarea::TextArea;

use crate::{
    calendar_data::{string_to_naive_date, Calendar, Events},
    to_do_data::{ToDo, ToDoList},
};

fn get_todo_title_block() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .fg(Color::Magenta)
        .add_modifier(Modifier::BOLD)
        .title(" To-Do ".to_string())
}

fn get_todo_user_input_block() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .fg(Color::DarkGray)
        // .add_modifier(Modifier::BOLD)
        .title(" User Input ".to_string())
}

fn get_todo_list_text(todo_list_text: String) -> Paragraph<'static> {
    Paragraph::new(todo_list_text)
        .fg(Color::Magenta)
        .block(Block::new().padding(Padding::new(5, 1, 2, 1)))
        .alignment(Alignment::Left)
}

fn write_user_input_to_json(
    input_todo_content: String,
    todolist: &mut Option<ToDoList>,
    calendar_list: &mut Option<Calendar>,
) {
    let parts_input: Vec<String> = input_todo_content
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    if parts_input.len() >= 3 {
        if parts_input[0].trim().to_lowercase() == "todo" {
            let new_todo = ToDo {
                high_prio: parts_input[1].parse().unwrap(),
                todo_name: parts_input[2].clone(),
            };

            todolist.as_mut().unwrap().add_todos_from_json();
            todolist.as_mut().unwrap().all_todos.push(new_todo);

            // Manually contruct the json
            todolist.as_mut().unwrap().add_back_todos_to_json();
        }
        if parts_input[0].trim().to_lowercase().contains("app") {
            let new_event = Events {
                date: string_to_naive_date(&parts_input[1].clone()),
                event_name: parts_input[2].clone(),
                location: parts_input[3].clone(),
            };

            calendar_list.as_mut().unwrap().add_appointments_from_json();
            calendar_list.as_mut().unwrap().all_events.push(new_event);

            // Manually contruct the json
            calendar_list.as_mut().unwrap().add_back_events_to_json();
        }
    }
}

pub fn main_todo_layout(
    frame: &mut Frame,
    main_layout: &Rc<[Rect]>,
    input_todo_textarea: &mut TextArea,
) {
    let mut todolist = ToDoList::new();
    let todo_list_text = todolist.generate_todo_text();

    let calendar = Calendar::new();

    // Check for Enter key and process input
    if event::poll(std::time::Duration::from_millis(50)).unwrap() {
        if let Event::Key(key) = event::read().unwrap() {
            if key.code == KeyCode::Enter {
                let input_todo_content = input_todo_textarea.lines().join("\n");
                write_user_input_to_json(
                    input_todo_content,
                    &mut Some(todolist),
                    &mut Some(calendar),
                );
                // Clear the textarea after processing
                *input_todo_textarea = TextArea::default();
            } else {
                input_todo_textarea.input(tui_textarea::Input::from(key));
            }
        }
    }

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].to_vec())
        .split(main_layout[1]);

    let todo_block = get_todo_title_block();
    frame.render_widget(todo_block.clone(), layout[0]);
    frame.render_widget(get_todo_list_text(todo_list_text), layout[0]);

    let user_input_block = get_todo_user_input_block();
    frame.render_widget(user_input_block, layout[1]);
    frame.render_widget(&*input_todo_textarea, layout[1]);
    // The &* syntax is a way to dereference a pointer and then take a reference to the value it points to.
    // In the context of your code, &*input_todo_textarea.widget() is used to convert a mutable reference (&mut TextArea)
    // to an immutable reference (&TextArea), which is required by the render_widget method.
    // Here’s a breakdown of what &* does:
    // *input_todo_textarea.widget() dereferences the pointer, giving you the value that input_todo_textarea.widget() points to.
    // &*input_todo_textarea.widget() then takes a reference to that value, effectively converting it
    // from a mutable reference to an immutable reference.
}
