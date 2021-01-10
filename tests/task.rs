use nmp::models::task::{Task, TaskPriority, TaskState};
mod test_data;

#[test]
fn to_org() {
    let org = "* DONE [#C] Test\nLorem ipsum dolor";
    let task = test_data::get_testable_task(TaskPriority::C, TaskState::DONE);

    assert_eq!(task.to_org(), org);
}

#[test]
fn toggle_state() {
    let mut task = test_data::get_testable_task(TaskPriority::B, TaskState::TODO);
    task.toggle_state();

    assert_eq!(task.state, TaskState::DONE);
}

#[test]
fn increment_priority() {
    let mut task = test_data::get_testable_task(TaskPriority::B, TaskState::TODO);
    task.increment_priority();

    assert_eq!(task.priority, TaskPriority::A);
}
