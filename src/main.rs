use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

#[derive(Serialize, Deserialize)]
enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    fn to_string(&self) -> String {
        match self {
            Priority::Low => "Low".to_owned(),
            Priority::Medium => "Medium".to_owned(),
            Priority::High => "High".to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Task {
    name: String,
    description: String,
    priority: Priority,
    add_time: DateTime<Local>,
}

impl Task {
    fn new(
        name: String,
        description: String,
        priority: Priority,
        add_time: DateTime<Local>,
    ) -> Self {
        Self {
            name,
            description,
            priority,
            add_time,
        }
    }

    fn print_task(&self) {
        println!(
            "> {} | {} | {}\n/ {} /",
            self.name,
            self.priority.to_string(),
            self.add_time.format("%d-%m-%Y %H:%M:%S").to_string(),
            self.description.to_string()
        );
    }

    fn new_from_console() -> Self {
        let name = ConsoleManager::input("Enter new task name: ").unwrap();
        let description = ConsoleManager::input("Enter new task description: ").unwrap();
        let priority = match ConsoleManager::input("Enter new task priority: ")
            .unwrap()
            .as_str()
        {
            "low" => Priority::Low,
            "medium" => Priority::Medium,
            "high" => Priority::High,
            _ => {
                println!("Invalid priority, setting to low.");
                Priority::Low
            }
        };

        Self::new(name, description, priority, Local::now())
    }
}

struct TasksManager {
    tasks: Vec<Task>,
}

impl TasksManager {
    fn new() -> Self {
        Self { tasks: vec![] }
    }

    fn print_tasks(&self) {
        for task in self.tasks.iter() {
            task.print_task();
        }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn remove_task(&mut self, name: &str) -> Result<String, String> {
        if let Some(index) = self.find_task_index(name) {
            self.tasks.remove(index);
            Ok(format!("Task {} is removed", name))
        } else {
            Err(format!("Task {} not found", name))
        }
    }

    fn find_task_index(&self, name: &str) -> Option<usize> {
        self.tasks.iter().position(|t| t.name == name)
    }

    fn find_task(&self, name: &str) -> Option<&Task> {
        self.tasks
            .iter()
            .find(|t| t.name.to_string() == name.to_string())
    }

    fn edit_task(&mut self, name: &str, new_task: Task) -> Result<String, String> {
        if let Some(index) = self.find_task_index(name) {
            match self.tasks.get_mut(index) {
                Some(task) => {
                    task.name = new_task.name;
                    task.description = new_task.description;
                    task.priority = new_task.priority;
                    task.add_time = new_task.add_time;
                    Ok(format!("Task {} is removed", name))
                }
                None => Err("Error borrowing task index".to_string()),
            }
        } else {
            Err(format!("Task {} not found", name))
        }
    }

    fn store_to_file(&self, file_name: &str) -> Result<String, String> {
        if !Path::new(file_name).exists() {
            let file = match File::create(file_name) {
                Ok(file) => file,
                Err(err) => return Err(format!("Error creating file: {}", err)),
            };

            match serde_json::to_writer(&file, &self.tasks) {
                Ok(_) => Ok("Success".to_owned()),
                Err(err) => Err(format!("Error saving data: {}", err)),
            }
        } else {
            Err(format!("File {} already exists", file_name).to_owned())
        }
    }

    fn read_from_file(&mut self, file_name: &str) -> Result<String, String> {
        if Path::new(file_name).exists() {
            let file = match File::open(file_name) {
                Ok(file) => file,
                Err(err) => return Err(format!("Error creating file: {}", err)),
            };
            let reader = BufReader::new(file);

            self.tasks = match serde_json::from_reader(reader) {
                Ok(data) => data,
                Err(err) => return Err(format!("Error reading data: {}", err)),
            };

            Ok("Data read successfully".to_owned())
        } else {
            Err(format!("File {} does not exists", file_name).to_owned())
        }
    }
}

struct ConsoleManager {
    tasks_manager: TasksManager,
    menu_options: Vec<String>,
}

impl ConsoleManager {
    fn new() -> Self {
        Self {
            tasks_manager: TasksManager::new(),
            menu_options: vec![
                "Add task".to_owned(),
                "Find task".to_owned(),
                "Edit task".to_owned(),
                "Remove task".to_owned(),
                "Print list tasks".to_owned(),
                "Store tasks to file".to_owned(),
                "Read tasks from file".to_owned(),
            ],
        }
    }

    fn print_menu(&self) {
        for (idx, menu) in self.menu_options.iter().enumerate() {
            println!("{}. {}", idx + 1, menu);
        }
    }

    fn input(query: &str) -> std::io::Result<String> {
        print!("{}", query);
        std::io::stdout().flush()?;
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;
        Ok(buffer.trim().to_owned())
    }

    fn process_command(&mut self) {
        match Self::input("Enter command index") {
            Ok(command) => match command.as_str() {
                "1" => {
                    self.tasks_manager.add_task(Task::new_from_console());
                }
                "2" => {
                    let name = match Self::input("Enter new task name: ") {
                        Ok(n) => n,
                        Err(e) => {
                            println!("Error getting user input: {}", e);
                            return;
                        }
                    };
                    match self.tasks_manager.find_task(&name) {
                        None => println!("Task {} not found", name),
                        Some(task) => {
                            println!("Task found.");
                            task.print_task();
                        }
                    };
                }
                "3" => {
                    let name = match Self::input("Enter new task name: ") {
                        Ok(name) => name,
                        Err(e) => {
                            println!("Error getting user input: {}", e);
                            return;
                        }
                    };
                    match self
                        .tasks_manager
                        .edit_task(&name, Task::new_from_console())
                    {
                        Ok(msg) => {
                            println!("{}", msg)
                        }
                        Err(msg) => {
                            println!("{}", msg)
                        }
                    };
                }
                "4" => {
                    let name = match Self::input("Enter new task name: ") {
                        Ok(name) => name,
                        Err(e) => {
                            println!("Error getting user input: {}", e);
                            return;
                        }
                    };
                    match self.tasks_manager.remove_task(&name) {
                        Ok(msg) => {
                            println!("{}", msg)
                        }
                        Err(msg) => {
                            println!("{}", msg)
                        }
                    };
                }
                "5" => {
                    self.tasks_manager.print_tasks();
                }
                "6" => {
                    let file_name = match Self::input("Enter file name to store data in: ") {
                        Ok(name) => name,
                        Err(e) => {
                            println!("Error getting user input: {}", e);
                            return;
                        }
                    };
                    match self.tasks_manager.store_to_file(&file_name) {
                        Ok(msg) => {
                            println!("{}", msg);
                        }
                        Err(msg) => {
                            println!("{}", msg);
                            return;
                        }
                    }
                }
                "7" => {
                    let file_name = match Self::input("Enter file name to read data from: ") {
                        Ok(name) => name,
                        Err(e) => {
                            println!("Error getting user input: {}", e);
                            return;
                        }
                    };
                    match self.tasks_manager.read_from_file(&file_name) {
                        Ok(msg) => {
                            println!("{}", msg);
                        }
                        Err(msg) => {
                            println!("{}", msg);
                            return;
                        }
                    }
                }
                _ => println!("Invalid command"),
            },
            Err(e) => println!("Error getting user input: {}", e),
        }
    }
}

fn main() {
    let mut manager = ConsoleManager::new();
    manager.print_menu();

    loop {
        manager.process_command();
    }
}
