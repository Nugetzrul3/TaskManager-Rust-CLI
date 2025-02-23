use serde::{Deserialize, Serialize};
use crate::{Priority, Status};

#[derive(Serialize, Deserialize)]
pub struct Task {
    id: i32,
    title: String,
    description: String,
    estimated_duration: u32,
    due_date: String,
    priority: Priority,
    status: Status
}

impl Task {
    pub fn new(
        id: i32, title: &str, description: &str,
        estimated_duration: u32, due_date: &str,
        priority: Priority
    ) -> Task {
        Task {
            id,
            title: title.to_string(),
            description: description.to_string(),
            estimated_duration,
            due_date: due_date.to_string(),
            priority,
            status: Status::NotStarted,
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_next_id(&self) -> i32 {
        self.id + 1
    }

    pub fn get_status(&self) -> &Status {
        &self.status
    }

    pub fn get_priority(&self) -> &Priority {
        &self.priority
    }

    pub fn get_due_date(&self) -> &String {
        &self.due_date
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_est_duration(&self) -> u32 {
        self.estimated_duration
    }

    pub fn update_status(&mut self, status: Status) {
        self.status = status
    }

    pub fn update_priority(&mut self, priority: Priority) {
        self.priority = priority;
    }

    pub fn update_due_date(&mut self, due_date: String) {
        self.due_date = due_date
    }

    pub fn update_description(&mut self, description: String) {
        self.description = description
    }

    pub fn update_title(&mut self, title: String) {
        self.title = title
    }

    pub fn update_est_duration(&mut self, estimated_duration: u32) {
        self.estimated_duration = estimated_duration
    }
}