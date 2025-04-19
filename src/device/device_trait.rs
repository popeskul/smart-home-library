/// Base trait for all smart devices
pub trait SmartDeviceTrait {
    /// Returns the name of the device
    fn name(&self) -> &str;
}

/// Trait for devices that can be turned on/off
pub trait PowerControl: SmartDeviceTrait {
    /// Checks if the device is on
    fn is_on(&self) -> bool;

    /// Turns the device on
    fn turn_on(&mut self);

    /// Turns the device off
    fn turn_off(&mut self);
}

/// Trait for devices that measure temperature
pub trait TemperatureSensor: SmartDeviceTrait {
    /// Returns the current temperature reading
    fn temperature(&self) -> f32;
}

/// Trait for devices that consume power
pub trait PowerConsumption: SmartDeviceTrait {
    /// Returns the current power consumption in watts
    fn power_consumption(&self) -> f32;
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use mockall::*;

    mock! {
        pub SmartDevice {}
        impl SmartDeviceTrait for SmartDevice {
            fn name(&self) -> &str;
        }
    }

    mock! {
        pub PowerDevice {}
        impl SmartDeviceTrait for PowerDevice {
            fn name(&self) -> &str;
        }
        impl PowerControl for PowerDevice {
            fn is_on(&self) -> bool;
            fn turn_on(&mut self);
            fn turn_off(&mut self);
        }
    }

    mock! {
        pub TempSensorDevice {}
        impl SmartDeviceTrait for TempSensorDevice {
            fn name(&self) -> &str;
        }
        impl TemperatureSensor for TempSensorDevice {
            fn temperature(&self) -> f32;
        }
    }

    mock! {
        pub PowerConsumptionDevice {}
        impl SmartDeviceTrait for PowerConsumptionDevice {
            fn name(&self) -> &str;
        }
        impl PowerConsumption for PowerConsumptionDevice {
            fn power_consumption(&self) -> f32;
        }
    }
}
