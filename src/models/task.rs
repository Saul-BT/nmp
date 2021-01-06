#[derive(PartialEq, Debug)]
pub enum TaskPriority {
    A,
    B,
    C,
}

#[derive(PartialEq, Debug, Clone)]
pub enum TaskState {
    TODO,
    DONE,
}

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
}
