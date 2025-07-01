use sysfs_gpio::{Direction, Pin as SysfsPin};

/// Trait abstraction for a GPIO pin (can be implemented by real or mock hardware)
pub trait GpioPin {
    fn export(&self) -> Result<(), String>;
    fn set_direction(&self, direction: Direction) -> Result<(), String>;
    fn get_value(&self) -> Result<u8, String>;
    fn set_value(&self, value: u8) -> Result<(), String>;
}

/// Real GPIO implementation using sysfs_gpio for Orange Pi Zero
pub struct RealPin {
    pin: SysfsPin,
}

impl RealPin {
    /// Create a new RealPin and immediately export it
    pub fn new(pin_num: u64) -> Self {
        let pin = SysfsPin::new(pin_num);
        let _ = pin.export();
        std::thread::sleep(std::time::Duration::from_millis(100));
        Self { pin }
    }
}

impl GpioPin for RealPin {
    fn export(&self) -> Result<(), String> {
        self.pin.export().map_err(|e| format!("{:?}", e))
    }

    fn set_direction(&self, direction: Direction) -> Result<(), String> {
        self.pin.set_direction(direction).map_err(|e| format!("{:?}", e))
    }

    fn get_value(&self) -> Result<u8, String> {
        self.pin.get_value().map(|v| v as u8).map_err(|e| format!("{:?}", e))
    }

    fn set_value(&self, value: u8) -> Result<(), String> {
        self.pin.set_value(value as u8).map_err(|e| format!("{:?}", e))
    }
}
