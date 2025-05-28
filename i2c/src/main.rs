use std::{thread, time::Duration};

use bitvec::{field::BitField, prelude::*};
use i2cdev::{
    core::{I2CMessage, I2CTransfer},
    linux::{LinuxI2CDevice, LinuxI2CMessage},
};

fn main() {
    let mut dev = LinuxI2CDevice::new("/dev/i2c-0", 0x38).unwrap();

    let mut read_data = [0; 6];
    loop {
        let mut msgs = [
            LinuxI2CMessage::write(&[0xAC, 0x33, 0x00]),
            LinuxI2CMessage::read(&mut read_data),
        ];

        dev.transfer(&mut msgs).unwrap();

        let bits = read_data.view_bits_mut::<Msb0>();

        let hum = bits[8..28].load_be::<u32>();
        let hum = hum as f64 / 2_u32.pow(20) as f64 * 100.0;

        let temp = bits[28..48].load_be::<u32>();
        let temp = temp as f64 / 2_u32.pow(20) as f64 * 200.0 - 50.0;

        println!("Reading: {:x?}", read_data);
        println!("Влажность: {:.1}%; температура: {:.1}℃", hum, temp);

        thread::sleep(Duration::from_millis(500));
    }
}
