use std::fs;

#[derive(Default, Debug, Clone)]
pub struct ToDo {
    pub high_prio: bool,
    pub todo_name: String,
}

impl ToDo {
    pub fn new(high_prio: bool, todo_name: String) -> Self {
        Self {
            high_prio,
            todo_name,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct ToDoList {
    pub all_todos: Vec<ToDo>,
}

impl ToDoList {
    pub fn new() -> Self {
        Self {
            all_todos: Vec::new(),
        }
    }

    pub fn get_all_todo_items(&self) -> Vec<ToDo> {
        self.all_todos.clone()
    }

    pub fn add_todo_to_list(&mut self, todo_item: ToDo) {
        self.all_todos.push(todo_item);
    }

    pub fn get_todos_sorted_by_prio(&self) -> Vec<Vec<ToDo>> {
        let mut high_prio_todos = Vec::new();
        let mut low_prio_todos = Vec::new();

        for items in self.all_todos.iter() {
            if items.high_prio {
                high_prio_todos.push(items.clone());
            } else {
                low_prio_todos.push(items.clone())
            }
        }

        vec![high_prio_todos, low_prio_todos]
    }

    pub fn generate_todo_text(&mut self) -> String {
        let mut todo_text = String::new();

        self.add_todos_from_json();

        let sorted_todos = self.get_todos_sorted_by_prio();
        let high_prio_list = &sorted_todos[0];
        let low_prio_list = &sorted_todos[1];

        todo_text.push_str("High Priority");
        todo_text.push_str("\n");
        for items in high_prio_list.iter() {
            // todo_text.push_str(&convert_prio_to_string(items.high_prio));
            todo_text.push_str(&format!("{: <8}", items.todo_name));
            todo_text.push_str("\n");
        }
        todo_text.push_str("\n");

        todo_text.push_str("Low Priority");
        todo_text.push_str("\n");
        for items in low_prio_list.iter() {
            // todo_text.push_str(&convert_prio_to_string(items.high_prio));
            todo_text.push_str(&format!("{: <8}", items.todo_name));
            todo_text.push_str("\n");
        }

        todo_text
    }

    pub fn add_todos_from_json(&mut self) {
        let todo_path = "assets/todos.json";
        let data = fs::read_to_string(todo_path).expect("Could not open file");
        let todo_json: serde_json::Value =
            serde_json::from_str(&data).expect("Serde error in reading data from JSON");

        // Manually add each element to the struct
        let all_todos_json = todo_json["all_todos"]
            .as_array()
            .unwrap()
            .iter()
            .map(|todo_item| ToDo {
                high_prio: convert_string_to_bool(
                    todo_item["high_prio"].as_str().unwrap_or("false"),
                ),
                todo_name: todo_item["todo_name"].as_str().unwrap().to_string(),
            })
            .collect::<Vec<ToDo>>();

        self.all_todos.clear();
        self.all_todos = all_todos_json;
    }
}

fn convert_string_to_bool(prio: &str) -> bool {
    match prio {
        "true" => true,
        _ => false,
    }
}

fn convert_prio_to_string(prio: bool) -> String {
    if prio {
        "High Priority".to_string()
    } else {
        "Low Priority".to_string()
    }
}
