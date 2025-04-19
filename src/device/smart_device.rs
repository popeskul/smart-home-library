use super::device_trait::{PowerConsumption, PowerControl, SmartDeviceTrait, TemperatureSensor};
use super::{SmartSocket, SmartThermometer};
use crate::Reporter;
use std::fmt::Debug;

#[derive(Debug)]
pub enum SmartDevice {
    Thermometer(SmartThermometer),
    Socket(SmartSocket),
}

// Basic device functionality implemented for all devices
impl SmartDeviceTrait for SmartDevice {
    fn name(&self) -> &str {
        match self {
            SmartDevice::Thermometer(thermometer) => thermometer.name(),
            SmartDevice::Socket(socket) => socket.name(),
        }
    }
}

impl From<SmartSocket> for SmartDevice {
    fn from(socket: SmartSocket) -> Self {
        SmartDevice::Socket(socket)
    }
}

impl From<SmartThermometer> for SmartDevice {
    fn from(thermometer: SmartThermometer) -> Self {
        SmartDevice::Thermometer(thermometer)
    }
}

impl Reporter for SmartDevice {
    fn report(&self) -> String {
        match self {
            SmartDevice::Thermometer(thermometer) => thermometer.report(),
            SmartDevice::Socket(socket) => socket.report(),
        }
    }
}

impl SmartDevice {
    /// Checks if the device supports power control functionality
    pub fn supports_power_control(&self) -> bool {
        match self {
            SmartDevice::Socket(_) => true,
            SmartDevice::Thermometer(_) => false,
        }
    }

    /// Checks if the device is on (if it supports power control)
    pub fn is_on(&self) -> Option<bool> {
        match self {
            SmartDevice::Socket(socket) => Some(socket.is_on()),
            SmartDevice::Thermometer(_) => None,
        }
    }

    /// Turns the device on (if it supports power control)
    /// Returns true if operation was successful
    pub fn turn_on(&mut self) -> bool {
        match self {
            SmartDevice::Socket(socket) => {
                socket.turn_on();
                true
            }
            SmartDevice::Thermometer(_) => false,
        }
    }

    /// Turns the device off (if it supports power control)
    /// Returns true if operation was successful
    pub fn turn_off(&mut self) -> bool {
        match self {
            SmartDevice::Socket(socket) => {
                socket.turn_off();
                true
            }
            SmartDevice::Thermometer(_) => false,
        }
    }

    /// Gets the temperature (if the device is a thermometer)
    pub fn temperature(&self) -> Option<f32> {
        match self {
            SmartDevice::Thermometer(thermometer) => Some(thermometer.temperature()),
            SmartDevice::Socket(_) => None,
        }
    }

    /// Gets power consumption (if the device is a socket)
    pub fn power_consumption(&self) -> Option<f32> {
        match self {
            SmartDevice::Socket(socket) => Some(socket.power_consumption()),
            SmartDevice::Thermometer(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_thermometer() -> SmartDevice {
        let thermometer = SmartThermometer::new(String::from("Test Thermometer"), 22.5);
        SmartDevice::Thermometer(thermometer)
    }

    fn create_test_socket_on() -> SmartDevice {
        let socket = SmartSocket::new(String::from("Test Socket"), true, 100.0);
        SmartDevice::Socket(socket)
    }

    fn create_test_socket_off() -> SmartDevice {
        let socket = SmartSocket::new(String::from("Test Socket"), false, 100.0);
        SmartDevice::Socket(socket)
    }

    #[test]
    fn test_supports_power_control() {
        struct TestCase {
            name: &'static str,
            device: SmartDevice,
            expected: bool,
        }

        let test_cases = vec![
            TestCase {
                name: "Thermometer should not support power control",
                device: create_test_thermometer(),
                expected: false,
            },
            TestCase {
                name: "Socket should support power control",
                device: create_test_socket_on(),
                expected: true,
            },
        ];

        for tc in test_cases {
            assert_eq!(
                tc.device.supports_power_control(),
                tc.expected,
                "Failed test: {}",
                tc.name
            );
        }
    }

    #[test]
    fn test_is_on() {
        struct TestCase {
            name: &'static str,
            device: SmartDevice,
            expected: Option<bool>,
        }

        let test_cases = vec![
            TestCase {
                name: "Thermometer should return None for is_on",
                device: create_test_thermometer(),
                expected: None,
            },
            TestCase {
                name: "Socket ON should return Some(true)",
                device: create_test_socket_on(),
                expected: Some(true),
            },
            TestCase {
                name: "Socket OFF should return Some(false)",
                device: create_test_socket_off(),
                expected: Some(false),
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.device.is_on(), tc.expected, "Failed test: {}", tc.name);
        }
    }

    #[test]
    fn test_temperature() {
        struct TestCase {
            name: &'static str,
            device: SmartDevice,
            expected: Option<f32>,
        }

        let test_cases = vec![
            TestCase {
                name: "Thermometer should return temperature",
                device: create_test_thermometer(),
                expected: Some(22.5),
            },
            TestCase {
                name: "Socket should return None for temperature",
                device: create_test_socket_on(),
                expected: None,
            },
        ];

        for tc in test_cases {
            assert_eq!(
                tc.device.temperature(),
                tc.expected,
                "Failed test: {}",
                tc.name
            );
        }
    }

    #[test]
    fn test_power_consumption() {
        struct TestCase {
            name: &'static str,
            device: SmartDevice,
            expected: Option<f32>,
        }

        let test_cases = vec![
            TestCase {
                name: "Thermometer should return None for power consumption",
                device: create_test_thermometer(),
                expected: None,
            },
            TestCase {
                name: "Socket ON should return Some(power_value)",
                device: create_test_socket_on(),
                expected: Some(100.0),
            },
            TestCase {
                name: "Socket OFF should return Some(0.0)",
                device: create_test_socket_off(),
                expected: Some(0.0),
            },
        ];

        for tc in test_cases {
            assert_eq!(
                tc.device.power_consumption(),
                tc.expected,
                "Failed test: {}",
                tc.name
            );
        }
    }

    #[test]
    fn test_turn_on_off() {
        struct TestCase {
            name: &'static str,
            device_factory: fn() -> SmartDevice,
            operation: fn(&mut SmartDevice) -> bool,
            expected_result: bool,
            expected_state: Option<bool>,
        }

        let test_cases = vec![
            TestCase {
                name: "Thermometer turn_on should return false and not change state",
                device_factory: create_test_thermometer,
                operation: |d| d.turn_on(),
                expected_result: false,
                expected_state: None,
            },
            TestCase {
                name: "Thermometer turn_off should return false and not change state",
                device_factory: create_test_thermometer,
                operation: |d| d.turn_off(),
                expected_result: false,
                expected_state: None,
            },
            TestCase {
                name: "Socket turn_on should return true and change state to ON",
                device_factory: create_test_socket_off,
                operation: |d| d.turn_on(),
                expected_result: true,
                expected_state: Some(true),
            },
            TestCase {
                name: "Socket turn_off should return true and change state to OFF",
                device_factory: create_test_socket_on,
                operation: |d| d.turn_off(),
                expected_result: true,
                expected_state: Some(false),
            },
        ];

        for tc in test_cases {
            let mut device = (tc.device_factory)();

            let result = (tc.operation)(&mut device);
            assert_eq!(
                result, tc.expected_result,
                "Operation result incorrect for test: {}",
                tc.name
            );

            assert_eq!(
                device.is_on(),
                tc.expected_state,
                "Device state incorrect after test: {}",
                tc.name
            );
        }
    }
}
