#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    i2c::I2c,
    gpio::Io, 
    delay::Delay, 
    prelude::*
};
use esp_println::println;
use bme280::i2c::BME280;

#[entry]
fn main() -> ! {
   //#[allow(unused)]
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut delay = Delay::new();

    let i2c = I2c::new(
        peripherals.I2C0, 
        io.pins.gpio6,
        io.pins.gpio7,
        100_000.Hz(),
    );

    let mut bme280 = BME280::new_primary(i2c);
    bme280.init(&mut delay).unwrap();

    loop {
        let measurements = bme280.measure(&mut delay).unwrap(); 
        println!("Temperature: {}°C\nHumidity: {}%\nPressure: {} Pa\n", measurements.temperature, measurements.humidity, measurements.pressure);
        delay.delay(500.millis());
    }
}
