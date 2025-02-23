use crate::{Priority, Status, TaskFilter};
pub fn get_priority(priority: &str) -> Priority {
    match priority.to_lowercase().as_str() {
        "high" => Priority::HIGH,
        "medium" => Priority::MEDIUM,
        "low" => Priority::LOW,
        _ => {
            println!("Unknown option, defaulting to low priority");
            Priority::LOW
        }
    }
}

pub fn get_status(status: &str) -> Status {
    match status.to_lowercase().as_str() {
        "not started" => Status::NotStarted,
        "in progress" => Status::InProgress,
        "on hold" => Status::OnHold,
        "completed" => Status::Completed,
        "cancelled" => Status::Cancelled,
        _ => {
            println!("Unknown status, defaulting to Not Started");
            Status::NotStarted
        }
    }
}

pub fn get_filter(filter_by: &str, filter: &str) -> TaskFilter {
    match filter_by.to_lowercase().as_str() {
        "status" => TaskFilter::ByStatus(get_status(filter)),
        "priority" =>  TaskFilter::ByPriority(get_priority(filter)),
        _ => TaskFilter::ByStatus(Status::NotStarted)
    }
}