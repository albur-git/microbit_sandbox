
use rtt_target::rprintln;

// Define a Trait for Button Actions: Creates a trait that specifies the action to be performed when a button event is detected.
pub trait ButtonAction {
    fn execute(&self);
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

