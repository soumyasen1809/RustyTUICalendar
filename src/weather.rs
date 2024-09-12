#[derive(Default, Debug, Clone)]
struct Temperature {
    temp_c: String,
    feels_like_c: String,
}

#[derive(Default, Debug, Clone)]
struct WeatherConditions {
    uv_index: String,
    humidity: String,
    pressure: String,
    visibility: String,
}
#[derive(Default, Debug, Clone)]
struct Wind {
    winddir_degree: String,
    winddir_point: String,
    wind_speed: String,
}

#[derive(Default, Debug, Clone)]
pub struct Weather {
    temp: Temperature,
    conditions: WeatherConditions,
    wind: Wind,
    local_obs_date_time: String,
    weather_description: String,
}

impl Weather {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        temp_c: String,
        feels_like_c: String,
        uv_index: String,
        humidity: String,
        pressure: String,
        visibility: String,
        winddir_degree: String,
        winddir_point: String,
        wind_speed: String,
        local_obs_date_time: String,
        weather_description: String,
    ) -> Self {
        let temp = Temperature {
            temp_c,
            feels_like_c,
        };
        let conditions = WeatherConditions {
            uv_index,
            humidity,
            pressure,
            visibility,
        };
        let wind = Wind {
            winddir_degree,
            winddir_point,
            wind_speed,
        };
        Self {
            temp,
            conditions,
            wind,
            local_obs_date_time,
            weather_description,
        }
    }

    pub fn temp_c(&self) -> &str {
        &self.temp.temp_c
    }

    pub fn feels_like_c(&self) -> &str {
        &self.temp.feels_like_c
    }

    pub fn uv_index(&self) -> &str {
        &self.conditions.uv_index
    }

    pub fn humidity(&self) -> &str {
        &self.conditions.humidity
    }

    pub fn pressure(&self) -> &str {
        &self.conditions.pressure
    }

    pub fn visibility(&self) -> &str {
        &self.conditions.visibility
    }

    pub fn winddir_degree(&self) -> &str {
        &self.wind.winddir_degree
    }

    pub fn wind_speed(&self) -> &str {
        &self.wind.wind_speed
    }

    pub fn winddir_point(&self) -> &str {
        &self.wind.winddir_point
    }

    pub fn local_obs_date_time(&self) -> &str {
        &self.local_obs_date_time
    }

    pub fn weather_description(&self) -> &str {
        &self.weather_description
    }

    pub async fn generate_weather_text(
        &self,
        city: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let current_weather = get_weather(city).await?;
        let mut city_weather_str = String::new();
        for wtr in current_weather {
            city_weather_str.push_str(&format!("Temp: {:>7} °C\n", wtr.temp.temp_c));
            city_weather_str.push_str(&format!("Feels: {:>6} °C\n", wtr.temp.feels_like_c));
            city_weather_str.push_str(&format!("Condition: {:>8}\n", wtr.weather_description));
            city_weather_str.push_str(&format!("UV Index: {:>3} \n", wtr.conditions.uv_index));
            city_weather_str.push_str(&format!("Pressure: {:>6} Pa\n", wtr.conditions.pressure));
            city_weather_str.push_str(&format!("Humidity: {:>4} °C\n", wtr.conditions.humidity));
            city_weather_str.push_str(&format!(
                "Visibility: {:>2} °C\n",
                wtr.conditions.visibility
            ));
            city_weather_str.push_str(&format!(
                "Wind dir: {:>5} {:>2} \n",
                wtr.wind.winddir_degree, wtr.wind.winddir_point
            ));
            city_weather_str.push_str(&format!("Wind speed: {:>2} kmph\n", wtr.wind.wind_speed));
            city_weather_str.push_str(&format!("Observed: {:>12} \n", wtr.local_obs_date_time));
        }
        Ok(city_weather_str)
    }
}

/// Use curl wttr.in in JSON format: https://wttr.in/London?format=j1
pub async fn get_weather(city: &str) -> Result<Vec<Weather>, Box<dyn std::error::Error>> {
    let url = String::from("https://wttr.in/") + city + &String::from("?format=j1");
    let weather_response = reqwest::get(url).await?;

    let mut weather_vec: Vec<Weather> = Vec::new();
    if weather_response.status().is_success() {
        let weather_body = weather_response.text().await?;
        weather_vec = get_weather_from_json(&weather_body);
    } else {
        println!("Failed to get the weather!");
    }

    Ok(weather_vec)
}

fn get_weather_from_json(weather_bod: &str) -> Vec<Weather> {
    let serde_weather_json: serde_json::Value =
        serde_json::from_str(weather_bod).expect("Serde error in reading data from JSON");

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

            let temp = Temperature {
                temp_c,
                feels_like_c,
            };
            let conditions = WeatherConditions {
                uv_index,
                humidity,
                pressure,
                visibility,
            };
            let wind = Wind {
                winddir_degree,
                winddir_point,
                wind_speed,
            };
            Weather {
                temp,
                conditions,
                wind,
                local_obs_date_time,
                weather_description,
            }
        })
        .collect::<Vec<Weather>>();

    weather
}
