
use cortex_m::interrupt::CriticalSection;
use rtt_target::rprintln;
use crate::gpio_handler::GpioInput;

// TODO: find a way to make this dynamic or at least use traits in some way

pub struct ButtonATask {
    pub input_button: GpioInput,
}

impl ButtonATask {
    pub fn new(button_input: GpioInput) -> ButtonATask {
        ButtonATask { input_button: button_input }
    }

    pub fn execute(&self, cs: &CriticalSection) {
            rprintln!("Button A has been pressed!");
    }
}


pub struct ButtonBTask {
    pub input_button: GpioInput,
}

impl ButtonBTask {
    pub fn new(button_input: GpioInput) -> ButtonBTask {
        ButtonBTask { input_button: button_input }
    }

    pub fn execute(&self, cs: &CriticalSection) {
            rprintln!("Button B has been pressed!");
    }
}


