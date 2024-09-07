use std::{
    fs::{self, OpenOptions},
    io::Write,
    str::FromStr,
};

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

    pub fn get_all_events_from_calendar(&self) -> Vec<Events> {
        self.all_events.clone()
    }

    pub fn get_day_count(&self) -> u32 {
        match self.current_date.month() {
            // In match expressions, you can match multiple patterns using the | syntax, which is the pattern or operator.
            // https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            2 => {
                if self.current_date.leap_year() {
                    29
                } else {
                    28
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
            calendar_text.push('\n');
            calendar_text.push('\n');
        }

        calendar_text
    }

    pub fn generate_appointment_text(&mut self, date: NaiveDate) -> String {
        let mut appointment_text = String::new();

        self.add_appointments_from_json();

        let events_to_search = self.get_event_from_calendar(date);

        for ev in &events_to_search {
            // Note: Using \t will cause the bounding box to cut lines
            let event_name_str = String::from("Event: ") + &ev.event_name;
            let location_name_str = String::from("Location: ") + &ev.location;
            appointment_text.push_str(&format!("{: <8}", event_name_str));
            appointment_text.push('\n');
            appointment_text.push_str(&format!("{: <8}", location_name_str));
            appointment_text.push('\n');
            appointment_text.push('\n');
        }

        if appointment_text.is_empty() {
            appointment_text.push_str(&format!("You do not have any appointments for {:?}", date))
        }

        appointment_text
    }

    pub fn add_appointments_from_json(&mut self) {
        let appointment_path = "assets/appointments.json";
        let data = fs::read_to_string(appointment_path).expect("Could not open file");
        let app_json: serde_json::Value =
            serde_json::from_str(&data).expect("Serde error in reading data from JSON");

        // Manually add each element to the struct
        let current_date_json = app_json["current_date"].as_str().unwrap().to_string();
        let current_date_chrono = string_to_naive_date(&current_date_json);

        let events_json = app_json["all_events"]
            .as_array()
            .unwrap()
            .iter()
            .map(|event| Events {
                date: string_to_naive_date(event["date"].as_str().unwrap()),
                event_name: event["event_name"].as_str().unwrap().to_string(),
                location: event["location"].as_str().unwrap().to_string(),
            })
            .collect::<Vec<Events>>();

        self.current_date = current_date_chrono;
        self.all_events.clear();
        self.all_events = events_json;
    }

    pub fn add_back_events_to_json(&self) {
        // Manually construct the updated JSON string
        let mut updated_data = String::new();
        updated_data.push_str("{\n");
        updated_data.push_str(&format!("  \"current_date\": \"{}\",\n", self.current_date));
        updated_data.push_str("  \"all_events\": [\n");
        for (i, event) in self.all_events.iter().enumerate() {
            updated_data.push_str("    {\n");
            updated_data.push_str(&format!("      \"date\": \"{}\",\n", event.date));
            updated_data.push_str(&format!(
                "      \"event_name\": \"{}\",\n",
                event.event_name
            ));
            updated_data.push_str(&format!("      \"location\": \"{}\"\n", event.location));
            if i == self.all_events.len() - 1 {
                updated_data.push_str("    }\n");
            } else {
                updated_data.push_str("    },\n");
            }
        }
        updated_data.push_str("  ]\n");
        updated_data.push_str("}\n");

        // Write the updated JSON back to the file
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("assets/appointments.json")
            .unwrap();
        file.write_all(updated_data.as_bytes()).unwrap();
    }
}

pub fn string_to_naive_date(s: &str) -> NaiveDate {
    NaiveDate::from_str(s).unwrap()
}
