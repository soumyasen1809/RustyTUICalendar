use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use chrono::{Datelike, Duration, NaiveDateTime};

#[derive(Default, Debug, Clone)]
pub struct Events {
    pub date: NaiveDateTime,
    pub event_name: String,
    pub location: String,
}

impl Events {
    pub fn new(date: NaiveDateTime, event_name: String, location: String) -> Self {
        Self {
            date,
            event_name,
            location,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Calendar {
    pub current_date: chrono::NaiveDateTime,
    pub all_events: Vec<Events>,
}

impl Calendar {
    pub fn new() -> Self {
        Self {
            current_date: chrono::Local::now().naive_local(),
            all_events: Vec::new(),
        }
    }

    pub fn get_current_date(&self) -> chrono::NaiveDateTime {
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
                if self.current_date.date().leap_year() {
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

    pub fn get_event_from_calendar(&self, date: NaiveDateTime) -> Vec<Events> {
        // Since the dates are in NaiveDateTime, they need to be searched against NaiveDate only.
        // I want to find if there are any appointments for today (NaiveDate) only.
        let indices: Vec<_> = self
            .all_events
            .iter()
            .enumerate()
            .filter_map(|(index, ev)| {
                if ev.date.date() == date.date() {
                    Some(index)
                } else {
                    None
                }
            })
            .collect();

        // Filter Map Logic: The filter_map closure should return the index if the condition is met, otherwise None (important)

        let mut event_vec = Vec::new();
        for &i in indices.iter() {
            event_vec.push(self.all_events[i].clone());
        }

        event_vec
    }

    // AI: Copilot generated function
    /// Get the month table for a given month
    pub fn get_month_table(&self, calendar_date: &NaiveDateTime) -> Vec<Vec<u32>> {
        // let first_date_of_month = calendar_date.with_day(1).unwrap();
        let first_date_of_month = *calendar_date - Duration::days(calendar_date.day0().into());
        let day_of_firstdate = first_date_of_month.weekday().num_days_from_sunday(); // Start week from Sunday

        let mut iter_date = first_date_of_month;

        let mut week_layer = vec![0; 7]; // Initialize with 0s
        let mut days_in_month = Vec::new();

        for _week in 0..6 {
            for day in 0..7 {
                if days_in_month.is_empty() && day < day_of_firstdate as usize {
                    week_layer[day] = 0; // Fill with 0 for days before the first day of the month
                } else if iter_date.month() == calendar_date.month() {
                    week_layer[day] = iter_date.day();
                    iter_date = iter_date
                        .date()
                        .succ_opt()
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap();
                } else {
                    week_layer[day] = 0; // Fill with 0 for days after the last day of the month
                }
            }
            days_in_month.push(week_layer.clone());
            week_layer = vec![0; 7];
        }

        days_in_month
    }

    pub fn generate_calendar_text(&self, calendar_date: &NaiveDateTime) -> String {
        let mut calendar_text = String::new();
        let weekdays_list = vec!["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"]; // Start week from Monday
        for weekday in weekdays_list {
            calendar_text.push_str(&format!("{: <3}", weekday));
        }
        calendar_text.push('\n');

        let month_table = self.get_month_table(calendar_date);

        for row in month_table {
            for day in row {
                if day == 0 {
                    calendar_text.push_str(&format!("{: <3}", "\u{00A0}")); // Empty space for days outside the current month
                                                                            // Note that the empty spaces are ignored in during rendering the frame
                                                                            // and so, we need to use the non-breaking space character (\u{00A0}) instead
                } else {
                    calendar_text.push_str(&format!("{: <3}", day));
                }
            }
            calendar_text.push('\n');
        }

        calendar_text
    }

    pub fn generate_appointment_text(&mut self, date: NaiveDateTime) -> String {
        let mut appointment_text = String::new();

        self.add_appointments_from_json();

        let events_to_search = self.get_event_from_calendar(date);

        for ev in &events_to_search {
            let event_name_str = String::from("Event: ") + &ev.event_name;
            let location_name_str = String::from("Location: ") + &ev.location;
            let event_time_str = String::from("Time: ") + &ev.date.time().to_string();
            appointment_text.push_str(&format!("{: <8}", event_name_str));
            appointment_text.push('\n');
            appointment_text.push_str(&format!("{: <8}", location_name_str));
            appointment_text.push('\n');
            appointment_text.push_str(&format!("{: <8}", event_time_str));
            appointment_text.push('\n');
            appointment_text.push('\n');
        }

        if appointment_text.is_empty() {
            appointment_text.push_str(&format!(
                "You do not have any appointments for {:?}",
                date.date()
            ))
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

/// Converts string to chrono::NaiveDateTime format
/// Both inputs of type %Y-%m-%d %H:%M:%S and %Y-%m-%dT%H:%M:%S
/// are supported
pub fn string_to_naive_date(s: &str) -> NaiveDateTime {
    if let Ok(date_time) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
        date_time
    } else {
        let new_date_time = s.replace("T", " ");
        NaiveDateTime::parse_from_str(&new_date_time, "%Y-%m-%d %H:%M:%S").unwrap()
    }
}
