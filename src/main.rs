use std::env;
use futures::executor::block_on;
use sysfs_gpio::{Direction, Edge, Pin};

async fn monitor_pin(pin: Pin) -> Result<(), sysfs_gpio::Error> {    
    pin.export()?;
    pin.set_direction(Direction::In)?;
    pin.set_edge(Edge::BothEdges)?;
    let mut gpio_events = pin.get_poller()?;

    while let Some(val) = gpio_events.poll(1000)? {
        println!("Pin {} changed value to {:?}", pin.get_pin_num(),val);
    }
    println!("Timeout");
    Ok(())
}

fn main() {
    println!("Test For Async/Await Rust IO!");

    let pin_number = 5;
    let pin = Pin::new(pin_number);
    let future = monitor_pin(pin);
    match block_on(future){
        Ok(_) => println!("done"),
        Err(e) => println!("error = {:?}", e),
    };
}
