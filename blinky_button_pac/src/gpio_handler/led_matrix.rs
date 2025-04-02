
use crate::gpio_handler::GpioPushPullOutput;

struct LedMatrix {
    led_columns: [GpioPushPullOutput; 3],
    led_rows: [GpioPushPullOutput; 3],
}

impl LedMatrix {

}