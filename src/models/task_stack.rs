use crate::models::task::{Task, TaskState};

pub struct TaskStack {
    tasks: Vec<Task>,
}

impl TaskStack {
    pub fn new() -> TaskStack {
        TaskStack { tasks: Vec::new() }
    }

    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn remove(&mut self, task_index: usize) -> Task {
        self.tasks.remove(task_index)
    }

    pub fn get(&self, task_index: usize) -> &Task {
        &self.tasks[task_index]
    }

    pub fn list(&self) -> String {
        self.tasks
            .iter()
            .map(|task| -> String { task.to_org() })
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn get_completed(&mut self) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|task| task.state == TaskState::DONE)
            .collect::<Vec<&Task>>()
    }

    pub fn count_completed(&mut self) -> usize {
        self.get_completed().len()
    }

    pub fn length(&self) -> usize {
        self.tasks.len()
    }
}
