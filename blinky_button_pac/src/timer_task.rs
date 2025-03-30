
use cortex_m::interrupt::CriticalSection;
use crate::gpio_handler::PushPullOutput;

// === [ Traits ] ===
pub trait TimerTask {
    /// Execute the task's operation
    fn execute(&self, cs: &CriticalSection);
}

// === [ Task Implementation ] ===
pub struct BlinkyTask {
    output_pin: PushPullOutput
}

impl TimerTask for BlinkyTask {
    fn execute(&self, cs: &CriticalSection) {
        self.output_pin.toggle(cs);
    }
}

impl BlinkyTask {
    pub fn new(output_pin: PushPullOutput) -> BlinkyTask {
        BlinkyTask { output_pin }
    }
}