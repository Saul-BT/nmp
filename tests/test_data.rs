use nmp::models::task::*;
use nmp::models::task_stack::*;

pub fn create_task(
    title: &str,
    description: &str,
    priority: TaskPriority,
    state: TaskState,
) -> Task {
    Task {
        title: String::from(title),
        description: String::from(description),
        priority,
        state,
    }
}

pub fn add_tasks(stack: &mut TaskStack, num_tasks: usize, state: TaskState) {
    for i in 0..num_tasks {
        stack.add(create_task(
            &format!("{}{}", "task", i)[..],
            "Lorem ipsum dolor",
            TaskPriority::A,
            state.clone(),
        ))
    }
}

pub fn get_testable_task(priority: TaskPriority, state: TaskState) -> Task {
    create_task("Test", "Lorem ipsum dolor", priority, state)
}
