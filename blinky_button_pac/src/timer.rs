use core::cell::RefCell;

use cortex_m::interrupt::Mutex;
use nrf52833_pac::{self as pac, interrupt, NVIC, TIMER0};
use core::sync::atomic::{AtomicU32, Ordering};
use rtt_target::{rprintln};


static TICK_COUNT: AtomicU32 = AtomicU32::new(0);

pub struct Timer {
    timer_channel: usize,
    timner_peripheral: Mutex<RefCell<Option<TIMER0>>>,
}

static TIMER_0: Timer = Timer {
    timer_channel: 0,
    timner_peripheral: Mutex::new(RefCell::new(None))
};

impl Timer {
    pub fn init(timer_0: TIMER0) {

        timer_0.tasks_stop.write(|w| w.tasks_stop().set_bit());   // disable timer before configuration
        timer_0.bitmode.write(|w| w.bitmode()._32bit());          // Set 32-bit mode to make counter 32 bit wide
        timer_0.prescaler.write(|w| w.prescaler().variant(4));    // Prescaler = 2^4 = 16 (16 MHz / 16 = 1 MHz)
        timer_0.cc[TIMER_0.timer_channel].write(|w| w.cc().variant(1000_000));        // Set the compare value for 1-second intervals (1 Mhz clock => 1000_000_000 ticks = 1 second)
        timer_0.intenset.write(|w| w.compare0().set());           // Enable the compare event interrupt
        timer_0.shorts.write(|w| w.compare0_clear().enabled());   // Enable counter clear on compare match, writes to SHORTS register
        timer_0.tasks_clear.write(|w| w.tasks_clear().set_bit());
        timer_0.tasks_start.write(|w| w.tasks_start().set_bit());

        cortex_m::interrupt::free(|cs| {
            // needs borrow_mut because changes the content of the struct held by the timer_peripheral variable
            TIMER_0.timner_peripheral.borrow(cs).borrow_mut().replace(timer_0);
        });

        unsafe { NVIC::unmask(pac::Interrupt::TIMER0);}
        NVIC::unpend(pac::interrupt::TIMER0);
    
    }
}

// Define the TIMER0 interrupt handler
#[interrupt]
fn TIMER0() {
    // Access the peripherals
    cortex_m::interrupt::free(|cs| {
        // does not need borrow_mut because the struct held by timer_peripheral is not changed, only it's content is accessed (used to write to a memory location)
        // the ref keywoard is needed because "if let" is a pattern matching functionality and .borrow returns an immutable reference. Ref tells the "if let" that it is looking for a reference.
        if let Some(ref timer_peripheral) = *TIMER_0.timner_peripheral.borrow(cs).borrow() {
            timer_peripheral.events_compare[TIMER_0.timer_channel].write(|w| w.events_compare().clear_bit());
        }
    });

    // Increment the seconds count
    TICK_COUNT.fetch_add(1, Ordering::Relaxed);
    rprintln!("Timer interrupd fired {} times",TICK_COUNT.load(Ordering::Relaxed));

}