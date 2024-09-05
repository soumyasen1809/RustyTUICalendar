use std::collections::HashMap;

use chrono::{Datelike, NaiveDate};

#[derive(Default, Debug, Clone)]
pub struct Events {
    pub date: NaiveDate,
    pub event_name: String,
    pub location: String,
}

impl Events {
    pub fn new(date: NaiveDate, event_name: String, location: String) -> Self {
        Self {
            date,
            event_name,
            location,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Calendar {
    pub current_date: chrono::NaiveDate,
    pub event_map: HashMap<chrono::NaiveDate, Vec<Events>>,
}

impl Calendar {
    pub fn new() -> Self {
        Self {
            current_date: chrono::Local::now().date_naive(),
            event_map: HashMap::new(),
        }
    }

    pub fn get_current_date(&self) -> chrono::NaiveDate {
        self.current_date
    }

    pub fn get_day_count(&self) -> u32 {
        match self.current_date.month() {
            // In match expressions, you can match multiple patterns using the | syntax, which is the pattern or operator.
            // https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            2 => {
                if self.current_date.leap_year() {
                    return 29;
                } else {
                    return 28;
                }
            }
            4 | 6 | 9 | 11 => 30,
            _ => 1,
        }
    }

    pub fn add_event_to_calendar(&mut self, event: Events) {
        let key = event.date;
        self.event_map.entry(key).or_insert(Vec::new()).push(event);
    }

    pub fn get_event_from_calendar(&self, date: NaiveDate) -> Vec<Events> {
        self.event_map.get(&date).unwrap_or(&Vec::new()).clone()
    }

    pub fn get_month_table(&self) -> Vec<Vec<u32>> {
        let mut linear_top_layer = Vec::new();
        let mut linear_middle_layer = Vec::new();
        let mut linear_bottom_layer = Vec::new();
        for day in 1..=10_u32 {
            linear_top_layer.push(day);
        }
        for day in 11..=20_u32 {
            linear_middle_layer.push(day);
        }
        for day in 21..=self.get_day_count() {
            linear_bottom_layer.push(day);
        }

        let days_in_month = vec![linear_top_layer, linear_middle_layer, linear_bottom_layer];

        days_in_month
    }

    pub fn generate_calendar_text(&self) -> String {
        let mut calendar_text = String::new();

        let month_table = self.get_month_table();

        for row in month_table {
            for day in row {
                calendar_text.push_str(&format!("{: <3}", day));
            }
            calendar_text.push_str("\n");
        }

        calendar_text
    }
}
