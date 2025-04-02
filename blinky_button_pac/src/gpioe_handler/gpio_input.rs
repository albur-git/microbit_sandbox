use super::button_action::ButtonAction;
use super::button_task::ButtonTask;

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