use nmp::models::task::{Task, TaskPriority, TaskState};
use nmp::models::task_stack::TaskStack;
mod test_data;

#[test]
fn tasks_list() {
    let mut stack = TaskStack::new();
    let num_tasks = 5;
    let list = vec![
        "task0 - Lorem ipsum dolor [A]",
        "task1 - Lorem ipsum dolor [A]",
        "task2 - Lorem ipsum dolor [A]",
        "task3 - Lorem ipsum dolor [A]",
        "task4 - Lorem ipsum dolor [A]",
    ]
    .join("\n");
    test_data::add_tasks(&mut stack, num_tasks, TaskState::TODO);

    assert_eq!(list, stack.list());
}

#[test]
fn get_task() {
    let mut stack = TaskStack::new();
    let num_tasks = 5;
    let third_task = Task {
        title: String::from("task3"),
        description: String::from("Lorem ipsum dolor"),
        priority: TaskPriority::A,
        state: TaskState::TODO,
    };
    test_data::add_tasks(&mut stack, num_tasks, TaskState::TODO);

    assert_eq!(&third_task, stack.get(3));
}

#[test]
fn stack_length_with_zero_tasks() {
    let mut stack = TaskStack::new();

    assert_eq!(stack.count_completed(), 0);
}

#[test]
fn stack_length_with_non_zero_tasks() {
    let mut stack = TaskStack::new();
    let num_completed_tasks = 5;
    let num_pending_tasks = 10;
    test_data::add_tasks(&mut stack, num_completed_tasks, TaskState::DONE);
    test_data::add_tasks(&mut stack, num_pending_tasks, TaskState::TODO);

    assert_eq!(stack.count_completed(), num_completed_tasks);
}

#[test]
fn stack_length_with_removed_task() {
    let mut stack = TaskStack::new();
    test_data::add_tasks(&mut stack, 10, TaskState::TODO);
    stack.remove(9);

    assert_eq!(stack.length(), 9);
}

#[test]
fn percent_completed_tasks() {
    let mut stack = TaskStack::new();
    let num_completed_tasks = 5;
    let num_pending_tasks = 10;
    test_data::add_tasks(&mut stack, num_completed_tasks, TaskState::DONE);
    test_data::add_tasks(&mut stack, num_pending_tasks, TaskState::TODO);

    assert_eq!((num_completed_tasks * 100 / stack.length()), 33);
}
