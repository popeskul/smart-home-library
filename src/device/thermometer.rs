use crate::device::device_trait::{SmartDeviceTrait, TemperatureSensor};

/// Smart thermometer device implementation
///
/// Provides temperature readings from a smart home thermometer
#[derive(Debug)]
pub struct SmartThermometer {
    name: String,
    temperature: f32,
}

impl SmartThermometer {
    /// Creates a new thermometer with the specified name and temperature
    pub fn new(name: String, temperature: f32) -> Self {
        Self { name, temperature }
    }
}

impl SmartDeviceTrait for SmartThermometer {
    fn name(&self) -> &str {
        &self.name
    }

    fn report(&self) -> String {
        format!(
            "Device: {name}, Temperature: {temperature}°C",
            name = self.name(),
            temperature = self.temperature()
        )
    }
}

impl TemperatureSensor for SmartThermometer {
    fn temperature(&self) -> f32 {
        self.temperature
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    fn float_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < f32::EPSILON
    }

    fn create_test_thermometer() -> SmartThermometer {
        SmartThermometer::new(String::from("Test Thermometer"), 22.5)
    }

    #[test]
    fn test_thermometer_creation() {
        struct ThermometerCreationTestCase {
            name: &'static str,
            thermometer_name: String,
            initial_temperature: f32,
            expected_name: &'static str,
            expected_temperature: f32,
        }

        let test_cases = vec![
            ThermometerCreationTestCase {
                name: "Create thermometer with positive temperature",
                thermometer_name: "Living Room Thermometer".to_string(),
                initial_temperature: 21.3,
                expected_name: "Living Room Thermometer",
                expected_temperature: 21.3,
            },
            ThermometerCreationTestCase {
                name: "Create thermometer with zero temperature",
                thermometer_name: "Freezer Thermometer".to_string(),
                initial_temperature: 0.0,
                expected_name: "Freezer Thermometer",
                expected_temperature: 0.0,
            },
            ThermometerCreationTestCase {
                name: "Create thermometer with negative temperature",
                thermometer_name: "Outdoor Thermometer".to_string(),
                initial_temperature: -15.7,
                expected_name: "Outdoor Thermometer",
                expected_temperature: -15.7,
            },
        ];

        for tc in test_cases {
            let thermometer =
                SmartThermometer::new(tc.thermometer_name.clone(), tc.initial_temperature);

            assert_eq!(
                thermometer.name(),
                tc.expected_name,
                "Test case '{}': Expected name '{}' but got '{}'",
                tc.name,
                tc.expected_name,
                thermometer.name()
            );

            assert!(
                float_eq(thermometer.temperature(), tc.expected_temperature),
                "Test case '{}': Expected temperature {} but got {}",
                tc.name,
                tc.expected_temperature,
                thermometer.temperature()
            );
        }
    }

    #[test]
    fn test_report_method() {
        struct ReportTestCase {
            name: &'static str,
            thermometer: SmartThermometer,
            expected_content: Vec<&'static str>,
        }

        let test_cases = vec![
            ReportTestCase {
                name: "Standard report format",
                thermometer: create_test_thermometer(),
                expected_content: vec!["Device: Test Thermometer", "Temperature: 22.5°C"],
            },
            ReportTestCase {
                name: "Report with zero temperature",
                thermometer: SmartThermometer::new("Zero Thermometer".to_string(), 0.0),
                expected_content: vec!["Device: Zero Thermometer", "Temperature: 0°C"],
            },
            ReportTestCase {
                name: "Report with negative temperature",
                thermometer: SmartThermometer::new("Freezer Thermometer".to_string(), -20.3),
                expected_content: vec!["Device: Freezer Thermometer", "Temperature: -20.3°C"],
            },
        ];

        for tc in test_cases {
            let report = tc.thermometer.report();

            for expected in tc.expected_content {
                assert!(
                    report.contains(expected),
                    "Test case '{}': Report should contain '{}', got: '{}'",
                    tc.name,
                    expected,
                    report
                );
            }
        }
    }

    #[test]
    fn test_temperature_sensor_functionality() {
        struct TemperatureSensorTestCase {
            name: &'static str,
            initial_temperature: f32,
            new_temperature: f32,
        }

        let test_cases = vec![
            TemperatureSensorTestCase {
                name: "Change temperature from positive to positive",
                initial_temperature: 22.0,
                new_temperature: 25.0,
            },
            TemperatureSensorTestCase {
                name: "Change temperature from positive to negative",
                initial_temperature: 5.0,
                new_temperature: -10.0,
            },
            TemperatureSensorTestCase {
                name: "Change temperature from negative to positive",
                initial_temperature: -15.0,
                new_temperature: 10.0,
            },
        ];

        for tc in test_cases {
            let thermometer =
                SmartThermometer::new("Test Sensor".to_string(), tc.initial_temperature);

            assert!(
                float_eq(thermometer.temperature(), tc.initial_temperature),
                "Test case '{}': Initial temperature should be {}",
                tc.name,
                tc.initial_temperature
            );

            assert!(
                !float_eq(thermometer.temperature(), tc.new_temperature),
                "Test case '{}': Initial temperature should not be equal to new temperature {}",
                tc.name,
                tc.new_temperature
            );
        }
    }
}
