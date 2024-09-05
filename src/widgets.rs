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

// ----------------------------------
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::calendar_widget::main_calendar_layout;
use crate::to_do_widget::main_todo_layout;

pub fn app_layout(frame: &mut Frame) {
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].to_vec())
        .split(frame.area());

    main_calendar_layout(frame, &main_layout);
    main_todo_layout(frame, &main_layout);
}
