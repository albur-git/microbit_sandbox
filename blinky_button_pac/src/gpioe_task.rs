
use nrf52833_pac::GPIOTE;
use rtt_target::rprintln;

// Define a Trait for Button Actions: Creates a trait that specifies the action to be performed when a button event is detected.
pub trait ButtonAction {
    fn execute(&self);
}

// Implement the Generic ButtonTask Struct: This struct will hold the GpioInput and be parameterized by the action to perform.​
pub struct ButtonTask<A: ButtonAction> {
    pub input_button: GpioInput,
    pub action: A,
}

impl<A: ButtonAction> ButtonTask<A> {
    pub fn new(button_input: GpioInput, action: A) -> Self {
        ButtonTask {
            input_button: button_input,
            action,
        }
    }

    pub fn check_event_clear_and_execute_task(&self, gpio_tasks_and_events: &GPIOTE) {
        let event_index = self.input_button.event_channel as usize;
        if gpio_tasks_and_events.events_in[event_index].read().bits() != 0 {
            gpio_tasks_and_events.events_in[event_index].write(|w| w);
            self.action.execute();
        }
    }
}

// Implement Specific Actions for Each Button: Structs for each button's action and implement the ButtonAction trait for them.
pub struct ButtonAAction;

impl ButtonAction for ButtonAAction {
    fn execute(&self) {
        rprintln!("Button A has been pressed!");
    }
}

pub struct ButtonBAction;

impl ButtonAction for ButtonBAction {
    fn execute(&self) {
        rprintln!("Button B has been pressed!");
    }
}

// GPIO input definition
pub struct GpioInput {
    pub event_channel: u32,
    pub event_channel_bit_mask: u32,
    pub pin: u32,
    pub pin_bit_mask: u32,
}

impl GpioInput {
    pub fn into_button_event_task<A: ButtonAction>(self, action: A) -> ButtonTask<A> {
        ButtonTask::new(self, action)
    }
}

// // TODO: find a way to make this dynamic or at least use traits in some way

// trait GpioEventHandling {
    
// }

// pub struct ButtonATask {
//     pub input_button: GpioInput,
// }

// impl ButtonATask {
//     pub fn new(button_input: GpioInput) -> ButtonATask {
//         ButtonATask { input_button: button_input }
//     }

//     pub fn check_event_clear_and_execute_task(&self, gpio_tasks_and_events: &GPIOTE) {
//         if gpio_tasks_and_events.events_in[self.input_button.event_channel as usize].read().bits() != 0 {
//             gpio_tasks_and_events.events_in[self.input_button.event_channel as usize].write(|w| w);
//             self.execute();
//         }
//     }

//     pub fn execute(&self) {
//             rprintln!("Button A has been pressed!");
//     }
// }


// pub struct ButtonBTask {
//     pub input_button: GpioInput,
// }

// impl ButtonBTask {
//     pub fn new(button_input: GpioInput) -> ButtonBTask {
//         ButtonBTask { input_button: button_input }
//     }

//     pub fn check_event_clear_and_execute_task(&self, gpio_tasks_and_events: &GPIOTE) {
//         if gpio_tasks_and_events.events_in[self.input_button.event_channel as usize].read().bits() != 0 {
//             gpio_tasks_and_events.events_in[self.input_button.event_channel as usize].write(|w| w);
//             self.execute();
//         }
//     }

//     pub fn execute(&self) {
//             rprintln!("Button B has been pressed!");
//     }
// }


