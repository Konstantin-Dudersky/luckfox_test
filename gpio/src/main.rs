use std::{thread, time::Duration};

use gpio_cdev::{Chip, LineRequestFlags};

fn main() {
    let mut chip = Chip::new("/dev/gpiochip2").unwrap();
    let handle = chip
        .get_line(15)
        .unwrap()
        .request(LineRequestFlags::OUTPUT, 0, "write-output")
        .unwrap();

    loop {
        handle.set_value(1).unwrap();
        thread::sleep(Duration::from_millis(100));
        handle.set_value(0).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
}
