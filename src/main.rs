mod gpio;
use gpio::controller::GpioController;
use std::{thread, time::Duration};

fn main() {
    println!("📥 Orange Pi Zero GPIO Control");
    let controller = GpioController::new();
    controller.init();

    loop {
        controller.read_inputs();
        controller.write_outputs();
        println!("⏱️ Waiting 10 seconds...\n");
        thread::sleep(Duration::from_secs(10));
    }
}
