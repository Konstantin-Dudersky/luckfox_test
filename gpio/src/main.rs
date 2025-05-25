use futures::stream::StreamExt;
use gpio_cdev::{AsyncLineEventHandle, Chip, EventRequestFlags, LineRequestFlags};

#[tokio::main]
async fn main() {
    let button = Chip::new("/dev/gpiochip0").unwrap().get_line(16).unwrap();

    let mut events = AsyncLineEventHandle::new(
        button
            .events(
                LineRequestFlags::INPUT,
                EventRequestFlags::BOTH_EDGES,
                "gpioevents",
            )
            .unwrap(),
    )
    .unwrap();

    let led = Chip::new("/dev/gpiochip2")
        .unwrap()
        .get_line(14)
        .unwrap()
        .request(LineRequestFlags::OUTPUT, 0, "LED")
        .unwrap();

    while let Some(event) = events.next().await {
        println!("GPIO Event: {:?}", event);
        let event = event.unwrap();
        match event.event_type() {
            gpio_cdev::EventType::RisingEdge => led.set_value(1).unwrap(),
            gpio_cdev::EventType::FallingEdge => led.set_value(0).unwrap(),
        }
    }
}
