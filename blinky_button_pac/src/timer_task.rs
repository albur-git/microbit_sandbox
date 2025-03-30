pub trait TimerTask {
    /// Execute the task's operation
    fn execute(&mut self);
}
