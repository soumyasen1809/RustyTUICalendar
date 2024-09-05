use std::{collections::HashMap, fmt::Debug};

use chrono::Months;
// use cursive::{
//     view::Resizable,
//     views::{LinearLayout, Panel, ResizedView, TextView},
// };

use crate::data::Calendar;

#[derive(Default, Clone)]
pub struct CalendarRender {
    pub calendar: Calendar,
}

impl CalendarRender {
    pub fn new() -> Self {
        Self {
            calendar: Calendar {
                current_date: chrono::Local::now().date_naive(),
                event_map: HashMap::new(),
            },
        }
    }

    pub fn next_date(&self) -> Self {
        Self {
            calendar: Calendar {
                current_date: self
                    .calendar
                    .get_current_date()
                    .checked_add_months(Months::new(1))
                    .unwrap(),
                event_map: HashMap::new(),
            },
        }
    }

    pub fn prev_date(&self) -> Self {
        Self {
            calendar: Calendar {
                current_date: self
                    .calendar
                    .get_current_date()
                    .checked_sub_months(Months::new(1))
                    .unwrap(),
                event_map: HashMap::new(),
            },
        }
    }

    // pub fn create_grids(&self) -> LinearLayout {
    //     let mut linear_layout_top = LinearLayout::horizontal();
    //     let mut linear_layout_middle = LinearLayout::horizontal();
    //     let mut linear_layout_bottom = LinearLayout::horizontal();

    //     for day in 1..=10_u32 {
    //         let view = get_view(day);
    //         linear_layout_top.add_child(view);
    //     }
    //     for day in 11..=20_u32 {
    //         let view = get_view(day);
    //         linear_layout_middle.add_child(view);
    //     }
    //     for day in 21..=self.calendar.get_day_count() {
    //         let view = get_view(day);
    //         linear_layout_bottom.add_child(view);
    //     }

    //     let mut linear_layout = LinearLayout::vertical();
    //     linear_layout.add_child(linear_layout_top);
    //     linear_layout.add_child(linear_layout_middle);
    //     linear_layout.add_child(linear_layout_bottom);

    //     linear_layout
    // }
}

impl Debug for CalendarRender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.calendar.get_current_date())
    }
}

// fn get_view(day: u32) -> ResizedView<ResizedView<Panel<TextView>>> {
//     Panel::new(TextView::new(day.to_string()))
//         .fixed_width(6)
//         .fixed_height(3)
// }
