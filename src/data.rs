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
    pub all_events: Vec<Events>,
}

impl Calendar {
    pub fn new() -> Self {
        Self {
            current_date: chrono::Local::now().date_naive(),
            all_events: Vec::new(),
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
        self.all_events.push(event);
    }

    pub fn get_event_from_calendar(&self, date: NaiveDate) -> Vec<Events> {
        let indices: Vec<_> = self
            .all_events
            .iter()
            .enumerate()
            .filter_map(|(index, ev)| if ev.date == date { Some(index) } else { None })
            .collect();

        // Filter Map Logic: The filter_map closure should return the index if the condition is met, otherwise None (important)

        let mut event_vec = Vec::new();
        for &i in indices.iter() {
            event_vec.push(self.all_events[i].clone());
        }

        event_vec
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
                calendar_text.push_str(&format!("{: <8}", day));
            }
            calendar_text.push_str("\n");
            calendar_text.push_str("\n");
        }

        calendar_text
    }

    pub fn generate_appointment_text(&self, date: NaiveDate) -> String {
        let mut appointment_text = String::new();

        let mut events_to_search = self.get_event_from_calendar(date);

        // Remove DUMMY --------
        let dummy_event = Events::new(date, "Demo name".to_string(), "Demo Location".to_string());
        events_to_search.push(dummy_event);
        let dummy_event2 =
            Events::new(date, "Demo name2".to_string(), "Demo Location2".to_string());
        events_to_search.push(dummy_event2);
        // ---------------------

        for ev in &events_to_search {
            let event_name_str = String::from("Event: \t") + &ev.event_name;
            let location_name_str = String::from("Location: \t") + &ev.location;
            appointment_text.push_str(&event_name_str);
            appointment_text.push_str("\n");
            appointment_text.push_str(&location_name_str);
            appointment_text.push_str("\n");
        }

        appointment_text
    }
}
