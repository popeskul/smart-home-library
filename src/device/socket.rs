use crate::device::device_trait::{PowerConsumption, PowerControl, SmartDeviceTrait};

/// Smart socket device implementation
///
/// Controls a smart power socket that can be turned on/off
/// and provides power consumption metrics
#[derive(Debug)]
pub struct SmartSocket {
    name: String,
    is_on: bool,
    power_consumption: f32,
}

impl SmartSocket {
    /// Creates a new smart socket with the specified parameters
    pub fn new(name: String, is_on: bool, power_consumption: f32) -> Self {
        Self {
            name,
            is_on,
            power_consumption,
        }
    }

    /// Calculates the active power consumption based on the current state
    fn calculate_active_power(&self) -> f32 {
        if self.is_on {
            self.power_consumption
        } else {
            0.0
        }
    }
}

impl SmartDeviceTrait for SmartSocket {
    fn name(&self) -> &str {
        &self.name
    }

    fn report(&self) -> String {
        let status = if self.is_on() { "ON" } else { "OFF" };
        format!(
            "Device: {name}, Status: {status}, Power consumption: {consumption}W",
            name = self.name(),
            status = status,
            consumption = self.power_consumption()
        )
    }
}

impl PowerControl for SmartSocket {
    fn is_on(&self) -> bool {
        self.is_on
    }

    fn turn_on(&mut self) {
        self.is_on = true;
    }

    fn turn_off(&mut self) {
        self.is_on = false;
    }
}

impl PowerConsumption for SmartSocket {
    fn power_consumption(&self) -> f32 {
        self.calculate_active_power()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    enum Action {
        None,
        TurnOn,
        TurnOff,
    }

    fn float_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < f32::EPSILON
    }

    fn create_test_socket(is_on: bool, consumption: f32) -> SmartSocket {
        SmartSocket::new(String::from("Test Socket"), is_on, consumption)
    }

    #[test]
    fn test_socket_behavior_with_mocks() {
        struct SocketBehaviorTestCase {
            name: &'static str,
            initial_power_state: bool,
            action: Action,
            expected_power_state: bool,
            initial_consumption: f32,
            expected_consumption: f32,
            expected_report_contains: Vec<&'static str>,
        }

        let test_cases = vec![
            SocketBehaviorTestCase {
                name: "Socket ON, no action",
                initial_power_state: true,
                action: Action::None,
                expected_power_state: true,
                initial_consumption: 150.0,
                expected_consumption: 150.0,
                expected_report_contains: vec!["Status: ON", "Power consumption: 150W"],
            },
            SocketBehaviorTestCase {
                name: "Socket ON, turn OFF",
                initial_power_state: true,
                action: Action::TurnOff,
                expected_power_state: false,
                initial_consumption: 150.0,
                expected_consumption: 0.0,
                expected_report_contains: vec!["Status: OFF", "Power consumption: 0W"],
            },
            SocketBehaviorTestCase {
                name: "Socket OFF, turn ON",
                initial_power_state: false,
                action: Action::TurnOn,
                expected_power_state: true,
                initial_consumption: 150.0,
                expected_consumption: 150.0,
                expected_report_contains: vec!["Status: ON", "Power consumption: 150W"],
            },
        ];

        for (i, tc) in test_cases.iter().enumerate() {
            println!("Running test case #{}: {}", i, tc.name);

            let mut real_socket = SmartSocket::new(
                "Test Socket".to_string(),
                tc.initial_power_state,
                tc.initial_consumption,
            );

            match tc.action {
                Action::TurnOn => real_socket.turn_on(),
                Action::TurnOff => real_socket.turn_off(),
                Action::None => {}
            }

            assert_eq!(
                real_socket.is_on(),
                tc.expected_power_state,
                "Test case #{} ({}): expected power state to be {}",
                i,
                tc.name,
                tc.expected_power_state
            );

            assert!(
                float_eq(real_socket.power_consumption(), tc.expected_consumption),
                "Test case #{} ({}): expected consumption to be {}, got {}",
                i,
                tc.name,
                tc.expected_consumption,
                real_socket.power_consumption()
            );

            let report = real_socket.report();
            for expected_text in &tc.expected_report_contains {
                assert!(
                    report.contains(expected_text),
                    "Test case #{} ({}): report should contain '{}', got: '{}'",
                    i,
                    tc.name,
                    expected_text,
                    report
                );
            }
        }
    }

    #[test]
    fn test_socket_creation() {
        struct SocketCreationTestCase {
            name: &'static str,
            socket_name: String,
            initial_state: bool,
            initial_consumption: f32,
            expected_name: &'static str,
            expected_state: bool,
            expected_consumption: f32,
        }

        let test_cases = vec![
            SocketCreationTestCase {
                name: "Create socket in ON state",
                socket_name: "Living Room Socket".to_string(),
                initial_state: true,
                initial_consumption: 120.0,
                expected_name: "Living Room Socket",
                expected_state: true,
                expected_consumption: 120.0,
            },
            SocketCreationTestCase {
                name: "Create socket in OFF state",
                socket_name: "Kitchen Socket".to_string(),
                initial_state: false,
                initial_consumption: 80.0,
                expected_name: "Kitchen Socket",
                expected_state: false,
                expected_consumption: 0.0,
            },
            SocketCreationTestCase {
                name: "Create socket with zero consumption",
                socket_name: "Office Socket".to_string(),
                initial_state: true,
                initial_consumption: 0.0,
                expected_name: "Office Socket",
                expected_state: true,
                expected_consumption: 0.0,
            },
        ];

        for tc in test_cases {
            let socket = SmartSocket::new(
                tc.socket_name.clone(),
                tc.initial_state,
                tc.initial_consumption,
            );

            assert_eq!(
                socket.name(),
                tc.expected_name,
                "Test case '{}': Expected name '{}' but got '{}'",
                tc.name,
                tc.expected_name,
                socket.name()
            );

            assert_eq!(
                socket.is_on(),
                tc.expected_state,
                "Test case '{}': Expected state {} but got {}",
                tc.name,
                tc.expected_state,
                socket.is_on()
            );

            assert!(
                float_eq(socket.power_consumption(), tc.expected_consumption),
                "Test case '{}': Expected consumption {} but got {}",
                tc.name,
                tc.expected_consumption,
                socket.power_consumption()
            );
        }
    }

    #[test]
    fn test_power_control_operations() {
        struct PowerControlTestCase {
            name: &'static str,
            initial_state: bool,
            operation: fn(&mut SmartSocket),
            expected_state: bool,
            initial_consumption: f32,
            expected_consumption: f32,
        }

        let test_cases = vec![
            PowerControlTestCase {
                name: "Turn on an OFF socket",
                initial_state: false,
                operation: |s| s.turn_on(),
                expected_state: true,
                initial_consumption: 150.0,
                expected_consumption: 150.0,
            },
            PowerControlTestCase {
                name: "Turn off an ON socket",
                initial_state: true,
                operation: |s| s.turn_off(),
                expected_state: false,
                initial_consumption: 150.0,
                expected_consumption: 0.0,
            },
            PowerControlTestCase {
                name: "Turn on an already ON socket",
                initial_state: true,
                operation: |s| s.turn_on(),
                expected_state: true,
                initial_consumption: 150.0,
                expected_consumption: 150.0,
            },
            PowerControlTestCase {
                name: "Turn off an already OFF socket",
                initial_state: false,
                operation: |s| s.turn_off(),
                expected_state: false,
                initial_consumption: 150.0,
                expected_consumption: 0.0,
            },
        ];

        for tc in test_cases {
            let mut socket = create_test_socket(tc.initial_state, tc.initial_consumption);
            (tc.operation)(&mut socket);

            assert_eq!(
                socket.is_on(),
                tc.expected_state,
                "Test case '{}': Expected state {} but got {}",
                tc.name,
                tc.expected_state,
                socket.is_on()
            );

            assert!(
                float_eq(socket.power_consumption(), tc.expected_consumption),
                "Test case '{}': Expected consumption {} but got {}",
                tc.name,
                tc.expected_consumption,
                socket.power_consumption()
            );
        }
    }

    #[test]
    fn test_power_consumption_operations() {
        struct PowerConsumptionTestCase {
            name: &'static str,
            initial_state: bool,
            initial_consumption: f32,
            expected_effective_consumption: f32,
        }

        let test_cases = vec![
            PowerConsumptionTestCase {
                name: "Set consumption for ON socket",
                initial_state: true,
                initial_consumption: 150.0,
                expected_effective_consumption: 150.0,
            },
            PowerConsumptionTestCase {
                name: "Set consumption for OFF socket",
                initial_state: false,
                initial_consumption: 100.0,
                expected_effective_consumption: 0.0,
            },
            PowerConsumptionTestCase {
                name: "Set zero consumption for ON socket",
                initial_state: true,
                initial_consumption: 0.0,
                expected_effective_consumption: 0.0,
            },
            PowerConsumptionTestCase {
                name: "Set high consumption for ON socket",
                initial_state: true,
                initial_consumption: 1000.0,
                expected_effective_consumption: 1000.0,
            },
        ];

        for tc in test_cases {
            let socket = create_test_socket(tc.initial_state, tc.initial_consumption);

            assert!(
                float_eq(
                    socket.power_consumption(),
                    tc.expected_effective_consumption
                ),
                "Test case '{}': Expected effective consumption {} but got {}",
                tc.name,
                tc.expected_effective_consumption,
                socket.power_consumption()
            );
        }
    }
}
