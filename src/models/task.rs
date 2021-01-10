use std::fmt;

#[derive(PartialEq, Debug)]
pub enum TaskPriority {
    A,
    B,
    C,
}

impl fmt::Display for TaskPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TaskPriority::A => write!(f, "A"),
            TaskPriority::B => write!(f, "B"),
            TaskPriority::C => write!(f, "C"),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum TaskState {
    TODO,
    DONE,
}

impl fmt::Display for TaskState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TaskState::TODO => write!(f, "TODO"),
            TaskState::DONE => write!(f, "DONE"),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Task {
    pub title: String,
    pub description: String,
    pub priority: TaskPriority,
    pub state: TaskState,
}

impl Task {
    pub fn increment_priority(&mut self) {
        self.decrement_priority();
        self.decrement_priority();
    }

    pub fn decrement_priority(&mut self) {
        self.priority = match self.priority {
            TaskPriority::A => TaskPriority::B,
            TaskPriority::B => TaskPriority::C,
            TaskPriority::C => TaskPriority::A,
        }
    }

    pub fn toggle_state(&mut self) {
        self.state = match self.state {
            TaskState::TODO => TaskState::DONE,
            TaskState::DONE => TaskState::TODO,
        }
    }

    pub fn to_org(&self) -> String {
        format!(
            "* {} [#{}] {}\n{}",
            self.state, self.priority, self.title, self.description
        )
    }
}
