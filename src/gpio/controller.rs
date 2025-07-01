use sysfs_gpio::Direction;
use super::pin::{GpioPin, RealPin};

/// Controller struct holding all digital input and output pins
pub struct GpioController {
    pub input_pins: Vec<Box<dyn GpioPin>>,
    pub output_pins: Vec<Box<dyn GpioPin>>,
}

impl GpioController {
    /// Initialize GPIO controller with fixed input/output pin mappings
    pub fn new() -> Self {
        // Digital Inputs
        let input_numbers = vec![18, 19, 13, 14, 15, 16, 6, 7];
        let input_pins: Vec<Box<dyn GpioPin>> = input_numbers
            .into_iter()
            .map(|num| Box::new(RealPin::new(num)) as Box<dyn GpioPin>)
            .collect();

        // Digital Outputs
        let output_numbers = vec![2, 3];
        let output_pins: Vec<Box<dyn GpioPin>> = output_numbers
            .into_iter()
            .map(|num| Box::new(RealPin::new(num)) as Box<dyn GpioPin>)
            .collect();

        Self {
            input_pins,
            output_pins,
        }
    }

    /// Export and set direction for all pins
    pub fn init(&self) {
        for pin in &self.input_pins {
            let _ = pin.export();
            let _ = pin.set_direction(Direction::In);
        }
        for pin in &self.output_pins {
            let _ = pin.export();
            let _ = pin.set_direction(Direction::Out);
        }
    }

    /// Read input pin values
    pub fn read_inputs(&self) {
        println!("📥 Reading Digital Inputs:");
        for (i, pin) in self.input_pins.iter().enumerate() {
            match pin.get_value() {
                Ok(value) => println!("DI{}: {}", i, value),
                Err(e) => println!("DI{}: Error - {}", i, e),
            }
        }
    }

    /// Write alternating values to output pins
    pub fn write_outputs(&self) {
        println!("📤 Writing to Digital Outputs:");
        for (i, pin) in self.output_pins.iter().enumerate() {
            let value = if i % 2 == 0 { 1 } else { 0 };
            match pin.set_value(value) {
                Ok(_) => println!("DO{}: Set to {}", i, value),
                Err(e) => println!("DO{}: Error - {}", i, e),
            }
        }
    }   
}


