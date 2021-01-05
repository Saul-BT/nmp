#[cfg(test)]
pub mod test_task {
    use crate::models::task::task::{Task, TaskPriority, TaskState};
    use crate::tests::test_data::test_data;

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
}
