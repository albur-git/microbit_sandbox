use super::gpio_input::GpioInput;
use super::button_action::ButtonAction;
use super::GPIOTE;

// Implement the Generic ButtonTask Struct: This struct will hold the GpioInput and be parameterized by the action to perform.​
// As A ius of type ButtonAction, this struct can either be populated by ButtonAAction or ButtonBAction and, by extension, it's execute function
// - ButtonTask<A> → Declares a generic type A inside the struct.
//   - A: ButtonAction → Trait bound, meaning A must implement the ButtonAction trait.
// - action: A → A field storing an action of type A (which must implement ButtonAction).
pub struct ButtonTask<A: ButtonAction> {
    pub input_button: GpioInput,
    pub action: A,
}

// ButtonTask<A> --> This implementation is aimed at any Generic ButtonTask
// impl<A: ButtonAction> --> restricts A so that it must implement ButtonAction
// --> This implementation applies to any ButtonTask<A> where A implements ButtonAction.
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
