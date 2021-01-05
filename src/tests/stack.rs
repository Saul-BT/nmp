#[cfg(test)]
pub mod test_stack {
    use crate::models::task::task::TaskState;
    use crate::models::task_stack::stack::TaskStack;
    use crate::tests::test_data::test_data;

    #[test]
    fn stack_with_zero_tasks() {
        let mut stack = TaskStack::new();

        assert_eq!(stack.count_completed(), 0);
    }

    #[test]
    fn stack_with_non_zero_tasks() {
        let mut stack = TaskStack::new();
        let num_completed_tasks = 5;
        let num_pending_tasks = 10;
        test_data::add_tasks(&mut stack, num_completed_tasks, TaskState::DONE);
        test_data::add_tasks(&mut stack, num_pending_tasks, TaskState::TODO);

        assert_eq!(stack.count_completed(), num_completed_tasks);
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
}
