use std::{thread, time::Duration};

use gpio_cdev::{Chip, LineRequestFlags};

fn main() {
    let button = Chip::new("/dev/gpiochip0")
        .unwrap()
        .get_line(16)
        .unwrap()
        .request(LineRequestFlags::INPUT, 0, "button")
        .unwrap();

    let led = Chip::new("/dev/gpiochip2")
        .unwrap()
        .get_line(14)
        .unwrap()
        .request(LineRequestFlags::OUTPUT, 0, "LED")
        .unwrap();

    loop {
        let button_value = button.get_value().unwrap();
        thread::sleep(Duration::from_millis(100));
        led.set_value(button_value).unwrap();
        println!("Button value: {button_value}");
    }
}
