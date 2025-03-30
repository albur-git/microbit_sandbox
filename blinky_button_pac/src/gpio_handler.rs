use core::cell::RefCell;

use cortex_m::interrupt::{CriticalSection, Mutex};
use nrf52833_pac::{P0};

use crate::gpioe_task::GpioInput;

static PORT_0_REG: Mutex<RefCell<Option<P0>>> = Mutex::new(RefCell::new(None));

pub enum GpioPortId {
    Port0,
//    Port1 // Todo
}

pub struct GpioHandler {
    port: GpioPortId
}

pub struct GpioPushPullOutput {
    port: GpioPortId,
    pin: u32,
    pin_bit_mask: u32
}

impl GpioPushPullOutput {
    
    pub fn set_high(&self, cs: &CriticalSection) {
        if let Some(port_0) = PORT_0_REG.borrow(cs).borrow().as_ref() { 
            unsafe { port_0.outset.write(|w|w.bits(self.pin_bit_mask)); }
        }
    }

    pub fn set_low(&self, cs: &CriticalSection) {
        if let Some(port_0) = PORT_0_REG.borrow(cs).borrow().as_ref() { 
                unsafe { port_0.outclr.write(|w|w.bits(self.pin_bit_mask)); } 
        }
    }

    pub fn toggle(&self, cs: &CriticalSection) {
        if let Some(port_0) = PORT_0_REG.borrow(cs).borrow().as_ref() { 
                unsafe { 
                    if port_0.out.read().bits() & self.pin_bit_mask != 0 {
                        port_0.outclr.write(|w| w.bits(self.pin_bit_mask));
                    } else {
                        port_0.outset.write(|w| w.bits(self.pin_bit_mask));
                    }
                } 
        }
    }
}

impl GpioHandler {
    // Todo: make it a singleton? --> P0 and P1 are already singletons, and as they are consumed by the new function, it is not necessary to make GpioHandler a singleton
    // The init function basically just "transforms" a singleton
    pub fn new(port: GpioPortId, port_register: P0) -> GpioHandler {
        // cortex_m::interrupt::free critical section disables all interrupts momentarily
        // As the code cannot be interrupted anway it is not necessary to persist critical sections for selective mutex functionality
        cortex_m::interrupt::free(|cs| {
            PORT_0_REG.borrow(cs).borrow_mut().replace(port_register)
        });
        GpioHandler{ port }
    }

    // the &self in the signature makes the function only available on instances of GpioHandler, therefore forcing the user to execute the GpioHandler new function to be able to instanciate a push_pull_output
    // TODO: Add initial state (High / Low)
    pub fn get_push_pull_output(&self, port: GpioPortId, pin: u32) -> GpioPushPullOutput {
        let pin_bit_mask: u32 = 1u32 << pin;
        cortex_m::interrupt::free(|cs| {
            if let Some(port_0) = PORT_0_REG.borrow(cs).borrow().as_ref() {    // as ref to not take ownership
                unsafe { port_0.outclr.write(|w|w.bits(pin_bit_mask)); }    // Set output to low pre-emtpively
                port_0.pin_cnf[pin as usize].write(|w| {
                    w.dir().output();          // Set as output
                    w.input().disconnect();    // Disconnect input buffer
                    w.pull().disabled();       // Disable pull-up and pull-down resistors
                    w.drive().s0s1();          // Standard '0', standard '1' drive configuration
                    w.sense().disabled();      // No sensing mechanism
                    w
                });
            }
        });
        GpioPushPullOutput{port, pin, pin_bit_mask}
    }

    pub fn get_input(&self, event_channel: u32, pin: u32) -> GpioInput {
        let event_channel_bit_mask = 1u32 << event_channel;
        let pin_bit_mask = 1u32 << pin;
        cortex_m::interrupt::free(|cs| {
            if let Some(port_0) = PORT_0_REG.borrow(cs).borrow().as_ref() {
                unsafe { port_0.outclr.write(|w|w.bits(pin_bit_mask)); } 
                port_0.pin_cnf[pin as usize].write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
            }
        });
        GpioInput { event_channel, event_channel_bit_mask, pin, pin_bit_mask }
    }

}
