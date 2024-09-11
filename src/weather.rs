#[derive(Debug, Clone)]
pub struct Weather {
    temp_c: String,
    feels_like_c: String,
    local_obs_date_time: String,
    uv_index: String,
    humidity: String,
    pressure: String,
    visibility: String,
    winddir_degree: String,
    winddir_point: String,
    wind_speed: String,
    weather_description: String,
}

impl Weather {
    pub fn new(
        temp_c: String,
        feels_like_c: String,
        local_obs_date_time: String,
        uv_index: String,
        humidity: String,
        pressure: String,
        visibility: String,
        winddir_degree: String,
        winddir_point: String,
        wind_speed: String,
        weather_description: String,
    ) -> Self {
        Self {
            temp_c,
            feels_like_c,
            local_obs_date_time,
            uv_index,
            humidity,
            pressure,
            visibility,
            winddir_degree,
            winddir_point,
            wind_speed,
            weather_description,
        }
    }

    pub fn temp_c(&self) -> &str {
        &self.temp_c
    }

    pub fn feels_like_c(&self) -> &str {
        &self.feels_like_c
    }

    pub fn local_obs_date_time(&self) -> &str {
        &self.local_obs_date_time
    }

    pub fn uv_index(&self) -> &str {
        &self.uv_index
    }

    pub fn humidity(&self) -> &str {
        &self.humidity
    }

    pub fn pressure(&self) -> &str {
        &self.pressure
    }

    pub fn visibility(&self) -> &str {
        &self.visibility
    }

    pub fn winddir_degree(&self) -> &str {
        &self.winddir_degree
    }

    pub fn wind_speed(&self) -> &str {
        &self.wind_speed
    }

    pub fn weather_description(&self) -> &str {
        &self.weather_description
    }

    pub fn winddir_point(&self) -> &str {
        &self.winddir_point
    }
}

/// Use curl wttr.in in JSON format: https://wttr.in/London?format=j1
pub async fn get_weather(city: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = String::from("https://wttr.in/") + city + &String::from("?format=j1");
    let weather_response = reqwest::get(url).await?;

    if weather_response.status().is_success() {
        let weather_body = weather_response.text().await?;
        let weather_vec = get_weather_from_json(&weather_body);
        println!("weather: {:?}", &weather_vec);
    } else {
        println!("Failed to get the weather!");
    }

    Ok(())
}

fn get_weather_from_json(weather_bod: &String) -> Vec<Weather> {
    let serde_weather_json: serde_json::Value =
        serde_json::from_str(&weather_bod).expect("Serde error in reading data from JSON");

    let weather = serde_weather_json["current_condition"]
        .as_array()
        .unwrap()
        .iter()
        .map(|val| {
            let temp_c = val["temp_C"].as_str().unwrap().to_string();
            let feels_like_c = val["FeelsLikeC"].as_str().unwrap().to_string();
            let local_obs_date_time = val["localObsDateTime"].as_str().unwrap().to_string();
            let uv_index = val["uvIndex"].as_str().unwrap().to_string();
            let humidity = val["humidity"].as_str().unwrap().to_string();
            let pressure = val["pressure"].as_str().unwrap().to_string();
            let visibility = val["visibility"].as_str().unwrap().to_string();
            let winddir_degree = val["winddirDegree"].as_str().unwrap().to_string();
            let winddir_point = val["winddir16Point"].as_str().unwrap().to_string();
            let wind_speed = val["windspeedKmph"].as_str().unwrap().to_string();
            let weather_description = val["weatherDesc"][0]["value"].as_str().unwrap().to_string(); // get the first description
            return Weather {
                temp_c,
                feels_like_c,
                local_obs_date_time,
                uv_index,
                humidity,
                pressure,
                visibility,
                winddir_degree,
                winddir_point,
                wind_speed,
                weather_description,
            };
        })
        .collect::<Vec<Weather>>();

    return weather;
}
