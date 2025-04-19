use crate::error::DeviceAccessError;
use crate::room::Room;
use crate::{Reporter, SmartDevice};
use std::collections::HashMap;

/// Represents a smart house with multiple rooms
#[derive(Debug)]
pub struct SmartHouse {
    name: String,
    rooms: HashMap<String, Room>,
}

impl SmartHouse {
    /// Creates a new smart house with the specified name and rooms
    pub fn new(name: String, rooms: HashMap<String, Room>) -> Self {
        Self { name, rooms }
    }

    /// Creates a new smart house with the specified name and an empty list of rooms
    pub fn new_empty(name: String) -> Self {
        Self {
            name,
            rooms: HashMap::new(),
        }
    }

    /// Returns the name of the house
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns all rooms in the house
    pub fn all_rooms(&self) -> &HashMap<String, Room> {
        &self.rooms
    }

    /// Returns a reference to a specific room by name
    pub fn room(&self, name: &String) -> Option<&Room> {
        self.rooms.get(name)
    }

    /// Returns a mutable reference to a specific room by name
    pub fn room_mut(&mut self, name: &String) -> Option<&mut Room> {
        self.rooms.get_mut(name)
    }

    /// Adds a new room to the house
    pub fn add_room(&mut self, name: String, room: Room) -> Option<Room> {
        self.rooms.insert(name, room)
    }

    /// Removes a room from the house by name
    pub fn remove_room(&mut self, name: &String) -> Option<Room> {
        self.rooms.remove(name)
    }

    /// Returns a reference to a specific device in a room by room and device name
    pub fn device(
        &self,
        room_name: &String,
        device_name: &String,
    ) -> Result<&SmartDevice, DeviceAccessError> {
        match self.rooms.get(room_name) {
            Some(room) => room.device(device_name).ok_or_else(|| {
                DeviceAccessError::DeviceNotFound(device_name.clone(), room_name.clone())
            }),
            None => Err(DeviceAccessError::RoomNotFound(room_name.clone())),
        }
    }

    /// Returns a mutable reference to a specific device in a room by room and device name
    pub fn device_mut(
        &mut self,
        room_name: &String,
        device_name: &String,
    ) -> Result<&mut SmartDevice, DeviceAccessError> {
        match self.rooms.get_mut(room_name) {
            Some(room) => room.device_mut(device_name).ok_or_else(|| {
                DeviceAccessError::DeviceNotFound(device_name.clone(), room_name.clone())
            }),
            None => Err(DeviceAccessError::RoomNotFound(room_name.clone())),
        }
    }
}

impl Reporter for SmartHouse {
    fn report(&self) -> String {
        let header_format = format!("=== Smart House: {} ===\n", self.name);
        let newlines = self.rooms.len();

        let estimated_capacity = header_format.len()
            + self.rooms.values().map(|r| r.report().len()).sum::<usize>()
            + newlines;

        let mut report = String::with_capacity(estimated_capacity);

        report.push_str(&header_format);

        for room in self.rooms.values() {
            report.push_str(&room.report());
            report.push('\n');
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::{SmartDevice, SmartSocket, SmartThermometer};
    use mockall::mock;
    use std::collections::HashMap;

    mock! {
        pub Room {}
        impl Clone for Room {
            fn clone(&self) -> Self;
        }
    }

    fn create_empty_house() -> SmartHouse {
        SmartHouse::new(String::from("Test House"), HashMap::new())
    }

    fn create_house_with_rooms() -> SmartHouse {
        let mut rooms = HashMap::new();
        rooms.insert(
            "Room 1".to_string(),
            Room::new("Room 1".to_string(), HashMap::new()),
        );
        rooms.insert(
            "Room 2".to_string(),
            Room::new("Room 2".to_string(), HashMap::new()),
        );
        SmartHouse::new("Test House".to_string(), rooms)
    }

    fn create_house_with_devices() -> SmartHouse {
        let mut living_room_devices = HashMap::new();
        living_room_devices.insert(
            "Living Room Thermometer".to_string(),
            SmartDevice::Thermometer(SmartThermometer::new(
                "Living Room Thermometer".to_string(),
                21.0,
            )),
        );
        living_room_devices.insert(
            "Living Room Socket".to_string(),
            SmartDevice::Socket(SmartSocket::new(
                "Living Room Socket".to_string(),
                true,
                80.0,
            )),
        );

        let mut bedroom_devices = HashMap::new();
        bedroom_devices.insert(
            "Bedroom Thermometer".to_string(),
            SmartDevice::Thermometer(SmartThermometer::new(
                "Bedroom Thermometer".to_string(),
                19.5,
            )),
        );

        let living_room = Room::new("Living Room".to_string(), living_room_devices);
        let bedroom = Room::new("Bedroom".to_string(), bedroom_devices);

        let mut hash_rooms = HashMap::new();
        hash_rooms.insert("Living Room".to_string(), living_room);
        hash_rooms.insert("Bedroom".to_string(), bedroom);
        SmartHouse::new("Smart Home".to_string(), hash_rooms)
    }

    #[test]
    fn test_house_creation() {
        let house = SmartHouse::new(String::from("Test House"), HashMap::new());
        assert_eq!(house.name(), "Test House");
        assert!(house.all_rooms().is_empty());
    }

    #[test]
    fn test_add_and_remove_room() {
        struct AddRemoveRoomTestCase {
            name: &'static str,
            initial_house: SmartHouse,
            operation: fn(&mut SmartHouse) -> (usize, bool),
            expected_rooms_count: usize,
            expected_success: bool,
        }

        let test_cases = vec![
            AddRemoveRoomTestCase {
                name: "Add a room to empty house",
                initial_house: create_empty_house(),
                operation: |house| {
                    let devices = HashMap::new();
                    let room = Room::new("Test Room".to_string(), devices);
                    let room_name = room.name().to_string();

                    house.add_room(room_name, room);
                    (house.all_rooms().len(), true)
                },
                expected_rooms_count: 1,
                expected_success: true,
            },
            AddRemoveRoomTestCase {
                name: "Remove a room from house with rooms",
                initial_house: create_house_with_rooms(),
                operation: |house| match house.remove_room(&"Room 1".to_string()) {
                    Some(_) => (house.all_rooms().len(), true),
                    None => (house.all_rooms().len(), false),
                },
                expected_rooms_count: 1,
                expected_success: true,
            },
            AddRemoveRoomTestCase {
                name: "Try to remove non-existent room",
                initial_house: create_house_with_rooms(),
                operation: |house| match house.remove_room(&"Room 3".to_string()) {
                    Some(_) => (house.all_rooms().len(), true),
                    None => (house.all_rooms().len(), false),
                },
                expected_rooms_count: 2,
                expected_success: false,
            },
        ];

        for tc in test_cases {
            let mut house = tc.initial_house;
            let (actual_count, success) = (tc.operation)(&mut house);

            assert_eq!(
                actual_count, tc.expected_rooms_count,
                "Failed test case '{}': Expected room count {} but got {}",
                tc.name, tc.expected_rooms_count, actual_count
            );

            assert_eq!(
                success, tc.expected_success,
                "Failed test case '{}': Expected success {} but got {}",
                tc.name, tc.expected_success, success
            );
        }
    }

    #[test]
    fn test_room_access() {
        struct RoomAccessTestCase {
            name: &'static str,
            house: SmartHouse,
            room_name: &'static str,
            expected_result: Option<&'static str>,
        }

        let test_cases = vec![
            RoomAccessTestCase {
                name: "Access first room",
                house: create_house_with_rooms(),
                room_name: "Room 1",
                expected_result: Some("Room 1"),
            },
            RoomAccessTestCase {
                name: "Access second room",
                house: create_house_with_rooms(),
                room_name: "Room 2",
                expected_result: Some("Room 2"),
            },
            RoomAccessTestCase {
                name: "Access non-existent room",
                house: create_house_with_rooms(),
                room_name: "Room 3",
                expected_result: None,
            },
        ];

        for tc in test_cases {
            let result = tc.house.room(&tc.room_name.to_string());

            match (result, tc.expected_result) {
                (Some(room), Some(expected_name)) => {
                    assert_eq!(
                        room.name(),
                        expected_name,
                        "Failed test case '{}': Expected room name '{}' but got '{}'",
                        tc.name,
                        expected_name,
                        room.name()
                    );
                }
                (None, None) => {
                    // No room was found, as expected
                }
                (Some(room), None) => {
                    panic!(
                        "Failed test case '{}': Expected no room but got room '{}'",
                        tc.name,
                        room.name()
                    );
                }
                (None, Some(expected_name)) => {
                    panic!(
                        "Failed test case '{}': Expected room '{}' but got none",
                        tc.name, expected_name
                    );
                }
            }
        }
    }

    #[test]
    fn test_mutable_room_access() {
        struct MutableRoomAccessTestCase {
            name: &'static str,
            room_name: &'static str,
            expected_success: bool,
        }

        let test_cases = vec![
            MutableRoomAccessTestCase {
                name: "Mutable access to first room",
                room_name: "Room 1",
                expected_success: true,
            },
            MutableRoomAccessTestCase {
                name: "Mutable access to non-existent room",
                room_name: "Room 10",
                expected_success: false,
            },
        ];

        for tc in test_cases {
            let mut house = create_house_with_rooms();
            let result = house.room_mut(&tc.room_name.to_string());

            assert_eq!(
                result.is_some(),
                tc.expected_success,
                "Failed test case '{}': Expected success {} but got {}",
                tc.name,
                tc.expected_success,
                result.is_some()
            );
        }
    }

    #[test]
    fn test_report_generation() {
        struct ReportTestCase {
            name: &'static str,
            house: SmartHouse,
            expected_content: Vec<&'static str>,
        }

        let test_cases = vec![
            ReportTestCase {
                name: "Report for house with a thermometer",
                house: {
                    let mut devices = HashMap::new();
                    devices.insert(
                        "Living Room Thermometer".to_string(),
                        SmartDevice::Thermometer(SmartThermometer::new(
                            "Living Room Thermometer".to_string(),
                            22.5,
                        )),
                    );
                    let room = Room::new("Living Room".to_string(), devices);
                    let mut rooms = HashMap::new();
                    rooms.insert("Living Room".to_string(), room);
                    SmartHouse::new("Test House".to_string(), rooms)
                },
                expected_content: vec![
                    "=== Smart House: Test House ===",
                    "=== Room: Living Room ===",
                    "Device: Living Room Thermometer",
                    "Temperature: 22.5°C",
                ],
            },
            ReportTestCase {
                name: "Report for house with multiple rooms and devices",
                house: create_house_with_devices(),
                expected_content: vec![
                    "=== Smart House: Smart Home ===",
                    "=== Room: Living Room ===",
                    "Device: Living Room Thermometer",
                    "Temperature: 21°C",
                    "Device: Living Room Socket",
                    "Status: ON",
                    "Power consumption: 80W",
                    "=== Room: Bedroom ===",
                    "Device: Bedroom Thermometer",
                    "Temperature: 19.5°C",
                ],
            },
        ];

        for tc in test_cases {
            let report = tc.house.report();

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
}
