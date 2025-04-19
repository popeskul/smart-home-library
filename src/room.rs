use crate::Reporter;
use crate::device::SmartDevice;
use std::collections::HashMap;

/// Represents a room in a smart house with multiple devices
#[derive(Debug)]
pub struct Room {
    name: String,
    devices: HashMap<String, SmartDevice>,
}

impl Room {
    /// Creates a new room with the specified name and devices
    pub fn new(name: String, devices: HashMap<String, SmartDevice>) -> Self {
        Room { name, devices }
    }

    /// Creates an empty room with the specified name
    pub fn new_empty(name: String) -> Self {
        Room {
            name,
            devices: HashMap::new(),
        }
    }

    /// Returns the name of the room
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns all devices in the room
    pub fn all_devices(&self) -> &HashMap<String, SmartDevice> {
        &self.devices
    }

    /// Returns a reference to a specific device by name
    pub fn device(&self, name: &String) -> Option<&SmartDevice> {
        self.devices.get(name)
    }

    /// Returns a mutable reference to a specific device by name
    pub fn device_mut(&mut self, name: &String) -> Option<&mut SmartDevice> {
        self.devices.get_mut(name)
    }

    /// Adds a new device to the room
    pub fn add_device(&mut self, name: String, device: SmartDevice) -> Option<SmartDevice> {
        self.devices.insert(name, device)
    }

    /// Removes a device from the room by name
    pub fn remove_device(&mut self, name: &String) -> Option<SmartDevice> {
        self.devices.remove(name)
    }

    /// Turns on a device by name (if the device supports power control)
    pub fn turn_on_device(&mut self, name: &String) -> Option<bool> {
        self.device_mut(name).map(|device| device.turn_on())
    }

    /// Turns off a device by name (if the device supports power control)
    pub fn turn_off_device(&mut self, name: &String) -> Option<bool> {
        self.device_mut(name).map(|device| device.turn_off())
    }

    /// Gets temperature from a device (if it's a thermometer)
    pub fn get_temperature(&self, name: &String) -> Option<Option<f32>> {
        self.device(name).map(|device| device.temperature())
    }

    /// Gets power consumption from a device (if it's a socket)
    pub fn get_power_consumption(&self, name: &String) -> Option<Option<f32>> {
        self.device(name).map(|device| device.power_consumption())
    }
}

impl Reporter for Room {
    fn report(&self) -> String {
        let header_format = format!("=== Room: {} ===\n", self.name);
        let newlines = self.devices.len();

        let estimated_capacity = header_format.len()
            + self
                .devices
                .values()
                .map(|d| d.report().len())
                .sum::<usize>()
            + newlines;

        let mut report = String::with_capacity(estimated_capacity);

        report.push_str(&header_format);

        for device in self.devices.values() {
            report.push_str(&device.report());
            report.push('\n');
        }

        report
    }
}

/// Macro to create a room with a name and devices
/// Usage:
/// ```
/// use smart_home::{create_room, Room, SmartDevice, SmartSocket, SmartThermometer};
///
/// // Create an empty room
/// let empty_room = create_room!();
///
/// // Create a room with just a name
/// let named_room = create_room!("Living Room");
///
/// // Create a room with devices
/// let socket = SmartSocket::new("Test Socket".to_string(), true, 100.0);
/// let thermo = SmartThermometer::new("Test Thermo".to_string(), 22.5);
/// let room_with_devices = create_room!(
///     "Living Room",
///     ("Socket", socket),
///     ("Thermometer", thermo)
/// );
/// ```
#[macro_export]
macro_rules! create_room {
    () => {{
        Room::new_empty("Default Room".to_string())
    }};

    ($room_name:expr) => {{
        Room::new_empty($room_name.to_string())
    }};

    ($room_name:expr, $(($device_key:expr, $device:expr)),* $(,)?) => {{
        let mut devices = std::collections::HashMap::new();
        $(
            devices.insert($device_key.to_string(), $device.into());
        )*
        Room::new($room_name.to_string(), devices)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SmartDeviceTrait;
    use crate::device::{SmartDevice, SmartSocket, SmartThermometer};
    use mockall::mock;
    use mockall::predicate::*;
    use std::collections::HashMap;

    mock! {
        pub SmartDevice {}
        impl SmartDeviceTrait for SmartDevice {
            fn name(&self) -> &str;
        }
        impl Clone for SmartDevice {
            fn clone(&self) -> Self;
        }
    }

    fn create_empty_room() -> Room {
        Room::new(String::from("Test Room"), HashMap::new())
    }

    fn create_room_with_devices() -> Room {
        let mut devices = HashMap::new();
        devices.insert(
            "Test Thermometer".to_string(),
            SmartDevice::Thermometer(SmartThermometer::new("Test Thermometer".to_string(), 22.0)),
        );
        devices.insert(
            "Test Socket".to_string(),
            SmartDevice::Socket(SmartSocket::new("Test Socket".to_string(), true, 100.0)),
        );

        Room::new(String::from("Test Room"), devices)
    }

    #[test]
    fn test_room_creation() {
        struct RoomCreationTestCase {
            name: &'static str,
            room_name: String,
            devices: HashMap<String, SmartDevice>,
            expected_name: &'static str,
            expected_devices_count: usize,
        }

        let test_cases = vec![
            RoomCreationTestCase {
                name: "Empty room creation",
                room_name: "Living Room".to_string(),
                devices: HashMap::new(),
                expected_name: "Living Room",
                expected_devices_count: 0,
            },
            RoomCreationTestCase {
                name: "Room with devices",
                room_name: "Bedroom".to_string(),
                devices: {
                    let mut devices = HashMap::new();
                    devices.insert(
                        "Bedroom Thermometer".to_string(),
                        SmartDevice::Thermometer(SmartThermometer::new(
                            "Bedroom Thermometer".to_string(),
                            21.5,
                        )),
                    );
                    devices
                },
                expected_name: "Bedroom",
                expected_devices_count: 1,
            },
        ];

        for tc in test_cases {
            let room = Room::new(tc.room_name, tc.devices);

            assert_eq!(
                room.name(),
                tc.expected_name,
                "Test case '{}': Expected room name '{}' but got '{}'",
                tc.name,
                tc.expected_name,
                room.name()
            );

            assert_eq!(
                room.all_devices().len(),
                tc.expected_devices_count,
                "Test case '{}': Expected {} devices but got {}",
                tc.name,
                tc.expected_devices_count,
                room.all_devices().len()
            );
        }
    }

    #[test]
    fn test_add_and_remove_device() {
        struct DeviceOperationTestCase {
            name: &'static str,
            use_empty_room: bool,
            operation: fn(&mut Room) -> Result<usize, ()>,
            expected_devices_count: usize,
            expected_success: bool,
        }

        let test_cases = vec![
            DeviceOperationTestCase {
                name: "Add device to empty room",
                use_empty_room: true,
                operation: |room| {
                    let device = SmartDevice::Thermometer(SmartThermometer::new(
                        "Added Thermometer".to_string(),
                        23.0,
                    ));
                    room.add_device("Added Thermometer".to_string(), device);
                    Ok(room.all_devices().len())
                },
                expected_devices_count: 1,
                expected_success: true,
            },
            DeviceOperationTestCase {
                name: "Remove device from room with devices",
                use_empty_room: false,
                operation: |room| match room.remove_device(&"Test Thermometer".to_string()) {
                    Some(_) => Ok(room.all_devices().len()),
                    None => Err(()),
                },
                expected_devices_count: 1,
                expected_success: true,
            },
            DeviceOperationTestCase {
                name: "Try to remove non-existent device",
                use_empty_room: false,
                operation: |room| match room.remove_device(&"Non-existent Device".to_string()) {
                    Some(_) => Ok(room.all_devices().len()),
                    None => Err(()),
                },
                expected_devices_count: 2,
                expected_success: false,
            },
        ];

        for tc in test_cases {
            let mut room = if tc.use_empty_room {
                create_empty_room()
            } else {
                create_room_with_devices()
            };

            let result = (tc.operation)(&mut room);

            match result {
                Ok(count) => {
                    assert_eq!(
                        count, tc.expected_devices_count,
                        "Test case '{}': Expected {} devices but got {}",
                        tc.name, tc.expected_devices_count, count
                    );
                    assert!(
                        tc.expected_success,
                        "Test case '{}': Expected failure but got success",
                        tc.name
                    );
                }
                Err(_) => {
                    assert!(
                        !tc.expected_success,
                        "Test case '{}': Expected success but got failure",
                        tc.name
                    );
                }
            }
        }
    }

    #[test]
    fn test_device_operations() {
        struct DeviceOperationTestCase {
            name: &'static str,
            operation: fn(&mut Room) -> Option<bool>,
            expected_success: bool,
            expected_operation_result: Option<bool>,
        }

        let test_cases = vec![
            DeviceOperationTestCase {
                name: "Turn off socket",
                operation: |room| room.turn_off_device(&"Test Socket".to_string()),
                expected_success: true,
                expected_operation_result: Some(true),
            },
            DeviceOperationTestCase {
                name: "Turn on socket",
                operation: |room| room.turn_on_device(&"Test Socket".to_string()),
                expected_success: true,
                expected_operation_result: Some(true),
            },
            DeviceOperationTestCase {
                name: "Try to turn on thermometer (not supported)",
                operation: |room| room.turn_on_device(&"Test Thermometer".to_string()),
                expected_success: true,
                expected_operation_result: Some(false),
            },
            DeviceOperationTestCase {
                name: "Try to operate non-existent device",
                operation: |room| room.turn_on_device(&"Non-existent Device".to_string()),
                expected_success: false,
                expected_operation_result: None,
            },
        ];

        for tc in test_cases {
            let mut room = create_room_with_devices();
            let result = (tc.operation)(&mut room);

            match result {
                Some(operation_result) => {
                    assert!(
                        tc.expected_success,
                        "Test case '{}': Expected failure but got success",
                        tc.name
                    );
                    assert_eq!(
                        Some(operation_result),
                        tc.expected_operation_result,
                        "Test case '{}': Expected operation result {:?} but got {:?}",
                        tc.name,
                        tc.expected_operation_result,
                        Some(operation_result)
                    );
                }
                None => {
                    assert!(
                        !tc.expected_success,
                        "Test case '{}': Expected success but got failure",
                        tc.name
                    );
                }
            }
        }
    }

    #[test]
    fn test_specialized_accessors() {
        struct SpecializedAccessTestCase {
            name: &'static str,
            accessor: fn(&Room, &String) -> Option<Option<f32>>,
            device_name: &'static str,
            expected_success: bool,
            expected_value: Option<f32>,
        }

        let test_cases = vec![
            SpecializedAccessTestCase {
                name: "Get temperature from thermometer",
                accessor: |room, name| room.get_temperature(name),
                device_name: "Test Thermometer",
                expected_success: true,
                expected_value: Some(22.0),
            },
            SpecializedAccessTestCase {
                name: "Get temperature from socket (not supported)",
                accessor: |room, name| room.get_temperature(name),
                device_name: "Test Socket",
                expected_success: true,
                expected_value: None,
            },
            SpecializedAccessTestCase {
                name: "Get power consumption from socket",
                accessor: |room, name| room.get_power_consumption(name),
                device_name: "Test Socket",
                expected_success: true,
                expected_value: Some(100.0),
            },
            SpecializedAccessTestCase {
                name: "Get power consumption from thermometer (not supported)",
                accessor: |room, name| room.get_power_consumption(name),
                device_name: "Test Thermometer",
                expected_success: true,
                expected_value: None,
            },
            SpecializedAccessTestCase {
                name: "Try to get temperature from non-existent device",
                accessor: |room, name| room.get_temperature(name),
                device_name: "Non-existent Device",
                expected_success: false,
                expected_value: None,
            },
        ];

        for tc in test_cases {
            let room = create_room_with_devices();
            let result = (tc.accessor)(&room, &tc.device_name.to_string());

            match result {
                Some(value) => {
                    assert!(
                        tc.expected_success,
                        "Test case '{}': Expected failure but got success",
                        tc.name
                    );
                    assert_eq!(
                        value, tc.expected_value,
                        "Test case '{}': Expected value {:?} but got {:?}",
                        tc.name, tc.expected_value, value
                    );
                }
                None => {
                    assert!(
                        !tc.expected_success,
                        "Test case '{}': Expected success but got failure",
                        tc.name
                    );
                }
            }
        }
    }
}
