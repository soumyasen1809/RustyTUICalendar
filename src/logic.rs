use std::{collections::HashMap, fmt::Debug};

use chrono::Months;

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
}

impl Debug for CalendarRender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.calendar.get_current_date())
    }
}
