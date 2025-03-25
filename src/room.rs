use crate::device::SmartDevice;
use crate::device::device_trait::SmartDeviceTrait;
use crate::error::AccessError;

/// Represents a room in a smart house with multiple devices
#[derive(Debug)]
pub struct Room {
    name: String,
    devices: Vec<SmartDevice>,
}

impl Room {
    /// Creates a new room with the specified name and devices
    pub fn new(name: String, devices: Vec<SmartDevice>) -> Self {
        Self { name, devices }
    }

    /// Returns the name of the room
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns all devices in the room
    pub fn all_devices(&self) -> &[SmartDevice] {
        &self.devices
    }

    /// Returns a reference to a specific device by index
    pub fn devices(&self, index: usize) -> Result<&SmartDevice, AccessError> {
        let total_devices = self.devices.len();
        self.devices.get(index).ok_or_else(|| AccessError {
            resource_type: String::from("Device"),
            requested_index: index,
            total_count: total_devices,
        })
    }

    /// Returns a mutable reference to a specific device by index
    pub fn devices_mut(&mut self, index: usize) -> Result<&mut SmartDevice, AccessError> {
        let total_devices = self.devices.len();

        self.devices.get_mut(index).ok_or_else(|| AccessError {
            resource_type: String::from("Device"),
            requested_index: index,
            total_count: total_devices,
        })
    }

    /// Generates a text report about the room and its devices
    pub fn report(&self) -> String {
        let estimated_capacity =
            self.name.len() + self.devices.iter().map(|d| d.report().len()).sum::<usize>() + 50;

        let mut report = String::with_capacity(estimated_capacity);

        report.push_str(&format!("=== Room: {} ===\n", self.name));

        for device in &self.devices {
            report.push_str(&device.report());
            report.push('\n');
        }

        report
    }

    /// Adds a new device to the room
    pub fn add_device(&mut self, device: SmartDevice) {
        self.devices.push(device);
    }

    /// Removes a device from the room by index
    pub fn remove_device(&mut self, index: usize) -> Result<SmartDevice, AccessError> {
        if index < self.devices.len() {
            Ok(self.devices.remove(index))
        } else {
            Err(AccessError {
                resource_type: String::from("Device"),
                requested_index: index,
                total_count: self.devices.len(),
            })
        }
    }

    /// Turns on a device by index (if the device supports power control)
    pub fn turn_on_device(&mut self, index: usize) -> Result<bool, AccessError> {
        let device = self.devices_mut(index)?;
        Ok(device.turn_on())
    }

    /// Turns off a device by index (if the device supports power control)
    pub fn turn_off_device(&mut self, index: usize) -> Result<bool, AccessError> {
        let device = self.devices_mut(index)?;
        Ok(device.turn_off())
    }

    /// Gets temperature from a device (if it's a thermometer)
    pub fn get_temperature(&self, index: usize) -> Result<Option<f32>, AccessError> {
        let device = self.devices(index)?;
        Ok(device.temperature())
    }

    /// Gets power consumption from a device (if it's a socket)
    pub fn get_power_consumption(&self, index: usize) -> Result<Option<f32>, AccessError> {
        let device = self.devices(index)?;
        Ok(device.power_consumption())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::{SmartDevice, SmartSocket, SmartThermometer};
    use mockall::mock;
    use mockall::predicate::*;

    mock! {
        pub SmartDevice {}
        impl SmartDeviceTrait for SmartDevice {
            fn name(&self) -> &str;
            fn report(&self) -> String;
        }
        impl Clone for SmartDevice {
            fn clone(&self) -> Self;
        }
    }

    fn create_empty_room() -> Room {
        Room::new(String::from("Test Room"), vec![])
    }

    fn create_room_with_devices() -> Room {
        let devices = vec![
            SmartDevice::Thermometer(SmartThermometer::new("Test Thermometer".to_string(), 22.0)),
            SmartDevice::Socket(SmartSocket::new("Test Socket".to_string(), true, 100.0)),
        ];

        Room::new(String::from("Test Room"), devices)
    }

    #[test]
    fn test_room_creation() {
        struct RoomCreationTestCase {
            name: &'static str,
            room_name: String,
            devices: Vec<SmartDevice>,
            expected_name: &'static str,
            expected_devices_count: usize,
        }

        let test_cases = vec![
            RoomCreationTestCase {
                name: "Empty room creation",
                room_name: "Living Room".to_string(),
                devices: vec![],
                expected_name: "Living Room",
                expected_devices_count: 0,
            },
            RoomCreationTestCase {
                name: "Room with devices",
                room_name: "Bedroom".to_string(),
                devices: vec![SmartDevice::Thermometer(SmartThermometer::new(
                    "Bedroom Thermometer".to_string(),
                    21.5,
                ))],
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
                    room.add_device(device);
                    Ok(room.all_devices().len())
                },
                expected_devices_count: 1,
                expected_success: true,
            },
            DeviceOperationTestCase {
                name: "Remove device from room with devices",
                use_empty_room: false,
                operation: |room| match room.remove_device(0) {
                    Ok(_) => Ok(room.all_devices().len()),
                    Err(_) => Err(()),
                },
                expected_devices_count: 1,
                expected_success: true,
            },
            DeviceOperationTestCase {
                name: "Try to remove non-existent device",
                use_empty_room: false,
                operation: |room| match room.remove_device(5) {
                    Ok(_) => Ok(room.all_devices().len()),
                    Err(_) => Err(()),
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
    fn test_device_access() {
        struct DeviceAccessTestCase {
            name: &'static str,
            index: usize,
            expected_success: bool,
            expected_device_name: Option<&'static str>,
        }

        let test_cases = vec![
            DeviceAccessTestCase {
                name: "Access first device",
                index: 0,
                expected_success: true,
                expected_device_name: Some("Test Thermometer"),
            },
            DeviceAccessTestCase {
                name: "Access second device",
                index: 1,
                expected_success: true,
                expected_device_name: Some("Test Socket"),
            },
            DeviceAccessTestCase {
                name: "Access non-existent device",
                index: 5,
                expected_success: false,
                expected_device_name: None,
            },
        ];

        for tc in test_cases {
            let room = create_room_with_devices();
            let result = room.devices(tc.index);

            match result {
                Ok(device) => {
                    assert!(
                        tc.expected_success,
                        "Test case '{}': Expected failure but got success",
                        tc.name
                    );
                    if let Some(expected_name) = tc.expected_device_name {
                        assert_eq!(
                            device.name(),
                            expected_name,
                            "Test case '{}': Expected device name '{}' but got '{}'",
                            tc.name,
                            expected_name,
                            device.name()
                        );
                    }
                }
                Err(_) => {
                    assert!(
                        !tc.expected_success,
                        "Test case '{}': Expected success but got failure",
                        tc.name
                    );
                    assert_eq!(
                        tc.expected_device_name, None,
                        "Test case '{}': Expected no device name for failure case",
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
            operation: fn(&mut Room) -> Result<bool, AccessError>,
            expected_success: bool,
            expected_operation_result: bool,
        }

        let test_cases = vec![
            DeviceOperationTestCase {
                name: "Turn off socket",
                operation: |room| room.turn_off_device(1),
                expected_success: true,
                expected_operation_result: true,
            },
            DeviceOperationTestCase {
                name: "Turn on socket",
                operation: |room| room.turn_on_device(1),
                expected_success: true,
                expected_operation_result: true,
            },
            DeviceOperationTestCase {
                name: "Try to turn on thermometer (not supported)",
                operation: |room| room.turn_on_device(0),
                expected_success: true,
                expected_operation_result: false,
            },
            DeviceOperationTestCase {
                name: "Try to operate non-existent device",
                operation: |room| room.turn_on_device(5),
                expected_success: false,
                expected_operation_result: false,
            },
        ];

        for tc in test_cases {
            let mut room = create_room_with_devices();
            let result = (tc.operation)(&mut room);

            match result {
                Ok(operation_result) => {
                    assert!(
                        tc.expected_success,
                        "Test case '{}': Expected failure but got success",
                        tc.name
                    );
                    assert_eq!(
                        operation_result, tc.expected_operation_result,
                        "Test case '{}': Expected operation result {} but got {}",
                        tc.name, tc.expected_operation_result, operation_result
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
    fn test_specialized_accessors() {
        struct SpecializedAccessTestCase {
            name: &'static str,
            accessor: fn(&Room, usize) -> Result<Option<f32>, AccessError>,
            device_index: usize,
            expected_success: bool,
            expected_value: Option<f32>,
        }

        let test_cases = vec![
            SpecializedAccessTestCase {
                name: "Get temperature from thermometer",
                accessor: |room, idx| room.get_temperature(idx),
                device_index: 0,
                expected_success: true,
                expected_value: Some(22.0),
            },
            SpecializedAccessTestCase {
                name: "Get temperature from socket (not supported)",
                accessor: |room, idx| room.get_temperature(idx),
                device_index: 1,
                expected_success: true,
                expected_value: None,
            },
            SpecializedAccessTestCase {
                name: "Get power consumption from socket",
                accessor: |room, idx| room.get_power_consumption(idx),
                device_index: 1,
                expected_success: true,
                expected_value: Some(100.0),
            },
            SpecializedAccessTestCase {
                name: "Get power consumption from thermometer (not supported)",
                accessor: |room, idx| room.get_power_consumption(idx),
                device_index: 0,
                expected_success: true,
                expected_value: None,
            },
            SpecializedAccessTestCase {
                name: "Try to get temperature from non-existent device",
                accessor: |room, idx| room.get_temperature(idx),
                device_index: 5,
                expected_success: false,
                expected_value: None,
            },
        ];

        for tc in test_cases {
            let room = create_room_with_devices();
            let result = (tc.accessor)(&room, tc.device_index);

            match result {
                Ok(value) => {
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
    fn test_room_with_mock_devices() {
        let mut mock_device1 = MockSmartDevice::new();
        let mut mock_device2 = MockSmartDevice::new();

        mock_device1
            .expect_name()
            .return_const(String::from("Mock Thermometer"));
        mock_device1
            .expect_report()
            .return_const("Mock Thermometer Report".to_string());
        mock_device1
            .expect_clone()
            .return_const(MockSmartDevice::new());

        mock_device2
            .expect_name()
            .return_const(String::from("Mock Socket"));
        mock_device2
            .expect_report()
            .return_const("Mock Socket Report".to_string());
        mock_device2
            .expect_clone()
            .return_const(MockSmartDevice::new());
    }
}
