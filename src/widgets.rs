// use chrono::Local;
// use cursive::{
//     view::Resizable,
//     views::{Button, Dialog, DummyView, LinearLayout, TextView},
//     Cursive,
// };

// use crate::{data::Events, logic::CalendarRender};

// fn add_event_to_ui(s: &mut Cursive, current_date: &mut CalendarRender) {
//     let new_event = Events::new(
//         Local::now().date_naive(),
//         "New Demo".to_string(),
//         "Demo Place".to_string(),
//     );
//     current_date
//         .calendar
//         .add_event_to_calendar(new_event.clone());
//     s.add_layer(
//         Dialog::around(
//             LinearLayout::vertical()
//                 .child(TextView::new(format!(
//                     "Event: {:?}, Date: {:?}, Location: {:?}",
//                     new_event.event_name, new_event.date, new_event.location
//                 )))
//                 .child(
//                     Button::new("Close", |p| {
//                         p.pop_layer();
//                     })
//                     .fixed_width(12),
//                 ),
//         )
//         .title("New event added!"),
//     );
// }

// fn show_diff_month_to_ui(s: &mut Cursive, current_date: &CalendarRender, go_next: bool) {
//     let next_month_cal = if go_next {
//         current_date.next_date()
//     } else {
//         current_date.prev_date()
//     };
//     s.pop_layer();
//     s.add_layer(layout(&next_month_cal));
// }

// fn show_events(current_date: &CalendarRender) -> LinearLayout {
//     let mut events_layout: LinearLayout = LinearLayout::vertical();
//     let mut event_name_layout: LinearLayout = LinearLayout::horizontal();
//     let mut event_location_layout: LinearLayout = LinearLayout::horizontal();

//     let all_events: Vec<Events> = current_date
//         .calendar
//         .get_event_from_calendar(current_date.calendar.get_current_date());

//     for ev in all_events {
//         let view_event_name = TextView::new(ev.event_name).fixed_width(25).fixed_height(3);
//         let view_location = TextView::new(ev.location).fixed_width(25).fixed_height(3);

//         event_name_layout.add_child(view_event_name);
//         event_location_layout.add_child(view_location);
//     }

//     events_layout.add_child(event_name_layout);
//     events_layout.add_child(event_location_layout);

//     events_layout
// }

// fn add_control_layout(current_date: CalendarRender) -> LinearLayout {
//     let mut controls_layout: LinearLayout = LinearLayout::horizontal();

//     controls_layout.add_child(
//         Button::new("Prev", {
//             let current_date_for_show_diff = current_date.clone();
//             move |s| show_diff_month_to_ui(s, &current_date_for_show_diff, false)
//         })
//         .fixed_width(15),
//     );

//     controls_layout.add_child(
//         Button::new("Next", {
//             let current_date_for_show_diff = current_date.clone();
//             move |s| show_diff_month_to_ui(s, &current_date_for_show_diff, true)
//         })
//         .fixed_width(15),
//     );

//     controls_layout.add_child(
//         Button::new("Add", {
//             let current_date_for_add = current_date.clone();
//             move |s| {
//                 add_event_to_ui(s, &mut current_date_for_add.clone());
//             }
//         })
//         .fixed_width(15),
//     );

//     controls_layout.add_child(Button::new("Quit", Cursive::quit).fixed_width(15));

//     controls_layout
// }

// pub fn layout(current_date: &CalendarRender) -> LinearLayout {
//     let calendar_dates_layout = current_date.create_grids();

//     let controls_layout = add_control_layout(current_date.clone());

//     let events_planned_layout = show_events(&current_date.clone());

//     // App layout
//     let mut app_layout = LinearLayout::vertical();
//     app_layout.add_child(DummyView::new()); // This acts as a separator
//     app_layout.add_child(controls_layout);
//     app_layout.add_child(DummyView::new()); // This acts as a separator
//     app_layout.add_child(calendar_dates_layout);
//     app_layout.add_child(DummyView::new()); // This acts as a separator
//     app_layout.add_child(events_planned_layout);
//     app_layout.add_child(DummyView::new()); // This acts as a separator

//     app_layout
// }

use chrono::Datelike;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Stylize},
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};

use crate::data::Calendar;

fn get_calendar_text(calendar_text: String) -> Paragraph<'static> {
    Paragraph::new(calendar_text)
        .fg(Color::Black)
        .add_modifier(Modifier::BOLD)
        .block(Block::new().padding(Padding::new(5, 5, 5, 5)))
        .alignment(Alignment::Left)
}

fn get_calendar_block(month: u32, year: i32) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .fg(Color::Red)
        .add_modifier(Modifier::BOLD)
        .title(format!(" Calendar - {}/{} ", month, year))
}

fn get_month_block() -> Block<'static> {
    Block::default().borders(Borders::ALL).fg(Color::Black)
}

fn get_appointment_block(day: u32, month: u32, year: i32) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .fg(Color::Green)
        .add_modifier(Modifier::BOLD)
        .title(format!(" Appointments - {}/{}/{} ", day, month, year))
}

pub fn app_layout(frame: &mut Frame) {
    let calendar = Calendar::new();
    let day = calendar.current_date.day0();
    let year = calendar.current_date.year();
    let month = calendar.current_date.month();

    let calendar_text = calendar.generate_calendar_text();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(5),
                Constraint::Percentage(55),
                Constraint::Percentage(30),
            ]
            .to_vec(),
        )
        .split(frame.area());

    let calendar_block = get_calendar_block(month, year);
    let month_days_block = get_month_block();
    let appointment_block = get_appointment_block(day, month, year);

    frame.render_widget(calendar_block.clone(), layout[0]);

    frame.render_widget(month_days_block.clone(), layout[1]);
    frame.render_widget(get_calendar_text(calendar_text), layout[1]);

    frame.render_widget(appointment_block.clone(), layout[2]);
}
