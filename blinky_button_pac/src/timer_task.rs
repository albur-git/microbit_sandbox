
use cortex_m::interrupt::CriticalSection;
use crate::gpio_handler::GpioPushPullOutput;

pub struct BlinkyTask {
    output_pin: GpioPushPullOutput
}

impl BlinkyTask {
    pub fn new(output_pin: GpioPushPullOutput) -> BlinkyTask {
        BlinkyTask { output_pin }
    }

    pub fn execute(&self, cs: &CriticalSection) {
        self.output_pin.toggle(cs);
    }
}