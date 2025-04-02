
use cortex_m::interrupt::CriticalSection;
use crate::gpio_handler::GpioPushPullOutput;

pub struct BlinkyTask {
    index: u32,
    led_columns: [GpioPushPullOutput; 3],
    led_rows: [GpioPushPullOutput; 3],
}

impl BlinkyTask {
    const TOTAL_MAX_INDEX: u32 = 9; 
    const ROW_MAX_INDEX: u32 = 3;
    const COLUMN_MAX_INDEX: u32 = 3;

    pub fn new(led_columns: [GpioPushPullOutput; 3], led_rows: [GpioPushPullOutput; 3]) -> BlinkyTask {
        BlinkyTask { index: 0, led_columns, led_rows }
    }

    pub fn execute(&mut self, cs: &CriticalSection) {
        // Reset all LEDs
        for led in &mut self.led_columns {
            led.set_high(cs);
        }
        for led in &mut self.led_rows {
            led.set_low(cs);
        }

        // Update index
        self.index = self.index.wrapping_add(1) % Self::TOTAL_MAX_INDEX;

        // Calculate row and column indices
        let (row_index, column_index) = ((self.index % Self::ROW_MAX_INDEX) as usize, (self.index / Self::COLUMN_MAX_INDEX) as usize);

        // Set LED
        self.led_rows[row_index].set_high(cs);
        self.led_columns[column_index].set_low(cs);
    }
}
