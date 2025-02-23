use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Priority {
    HIGH,
    MEDIUM,
    LOW
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    NotStarted,
    InProgress,
    OnHold,
    Completed,
    Cancelled
}

pub enum TaskFilter {
    ByStatus(Status),
    ByPriority(Priority)
}

pub enum TaskUpdate {
    ByStatus(Status),
    ByPriority(Priority),
    ByDueDate(String),
    ByDescription(String),
    ByEstDuration(u32),
    ByTitle(String)
}