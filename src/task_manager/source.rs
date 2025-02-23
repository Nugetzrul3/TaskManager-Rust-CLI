use crate::{Priority, Status, Task, TaskFilter, TaskUpdate};
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufReader, Write},
};

pub struct TaskManager {
    tasks: HashMap<i32, Task>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        TaskManager {
            tasks: HashMap::<i32, Task>::new(),
        }
    }

    pub fn load_tasks(&mut self) -> Result<(), Box<dyn Error>> {
        let file = File::open("tasks.json")?;
        let reader = BufReader::new(file);
        let tasks_json = serde_json::from_reader(reader)?;
        self.tasks = tasks_json;

        Ok(())
    }

    pub fn add_task(
        &mut self,
        title: &str,
        description: &str,
        estimated_duration: u32,
        due_date: &str,
        priority: Priority,
    ) {
        // let last_task = self.tasks.keys()
        // .last()
        // .and_then(|key| self.tasks.get(key));

        let last_task = self.tasks.values().last();

        let new_task: Task;
        match last_task {
            Some(task) => {
                new_task = Task::new(
                    task.get_next_id(),
                    title,
                    description,
                    estimated_duration,
                    due_date,
                    priority,
                );
            }
            None => {
                new_task = Task::new(
                    0,
                    title,
                    description,
                    estimated_duration,
                    due_date,
                    priority,
                );
            }
        };
        self.tasks.insert(new_task.get_id(), new_task);
    }

    pub fn view_task(&self, id: i32) {
        match self.tasks.get(&id) {
            Some(task) => {
                let priority = match *task.get_priority() {
                    Priority::HIGH => "High",
                    Priority::MEDIUM => "Medium",
                    Priority::LOW => "Low",
                };

                let status = match *task.get_status() {
                    Status::NotStarted => "Not Started",
                    Status::Cancelled => "Cancelled",
                    Status::Completed => "Completed",
                    Status::InProgress => "In Progress",
                    Status::OnHold => "On Hold",
                };
                println!(
                    "
                Title: {}
                Description: {}
                Estimated Time: {}
                Due Date: {}
                Priority: {}
                Status: {}",
                    task.get_title(),
                    task.get_description(),
                    task.get_est_duration(),
                    task.get_due_date(),
                    priority,
                    status
                );
            }
            None => {
                println!("Task {} could not be found", id.to_string());
                return;
            }
        }
    }

    pub fn update_task(&mut self, task_id: i32, task_update_filter: TaskUpdate) {
        let task = match self.tasks.get_mut(&task_id) {
            Some(task) => task,
            None => {
                println!("Task {} could not be found", task_id.to_string());
                return;
            }
        };

        match task_update_filter {
            TaskUpdate::ByStatus(status) => task.update_status(status),
            TaskUpdate::ByDescription(description) => task.update_description(description),
            TaskUpdate::ByDueDate(due_date) => task.update_due_date(due_date),
            TaskUpdate::ByEstDuration(estimated_duration) => {
                task.update_est_duration(estimated_duration)
            }
            TaskUpdate::ByPriority(priority) => task.update_priority(priority),
            TaskUpdate::ByTitle(title) => task.update_title(title),
        }
    }

    pub fn list_tasks(&self, filter: Option<TaskFilter>) {
        let mut tasks = Vec::<&Task>::new();

        match filter {
            Some(TaskFilter::ByPriority(priority)) => {
                for task in self.tasks.values() {
                    if *task.get_priority() == priority {
                        tasks.push(task);
                    }
                }
            }
            Some(TaskFilter::ByStatus(status)) => {
                for task in self.tasks.values() {
                    if *task.get_status() == status {
                        tasks.push(task);
                    }
                }
            }
            None => {
                for task in self.tasks.values() {
                    tasks.push(task)
                }
            }
        }

        println!("-------------TASKS--------------");
        for task in tasks {
            let priority = match *task.get_priority() {
                Priority::HIGH => "High",
                Priority::MEDIUM => "Medium",
                Priority::LOW => "Low",
            };

            let status = match *task.get_status() {
                Status::NotStarted => "Not Started",
                Status::Cancelled => "Cancelled",
                Status::Completed => "Completed",
                Status::InProgress => "In Progress",
                Status::OnHold => "On Hold",
            };
            println!(
                "
            Title: {}
            Description: {}
            Estimated Time: {}
            Due Date: {}
            Priority: {}
            Status: {}",
                task.get_title(),
                task.get_description(),
                task.get_est_duration(),
                task.get_due_date(),
                priority,
                status
            );
            println!("------------------------------------")
        }
    }

    pub fn save_tasks(&self) -> Result<(), Box<dyn Error>> {
        let mut file: File = File::create("tasks.json")?;
        let tasks_json = serde_json::to_vec_pretty(&self.tasks)?;

        file.write_all(&tasks_json)?;

        Ok(())
    }

    pub fn delete_task(&mut self, id: i32) {
        if self.tasks.contains_key(&id) {
            self.tasks.remove(&id);
            println!("Task removed successfully!");
        } else {
            println!("Task id {} does not exist", id);
        }
    }

    pub fn get_task(&self, id: i32) -> &Task {
        self.tasks.get(&id).unwrap()
    }
}
