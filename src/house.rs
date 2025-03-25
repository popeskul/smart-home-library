use crate::error::AccessError;
use crate::room::Room;

/// Represents a smart house with multiple rooms
#[derive(Debug)]
pub struct SmartHouse {
    name: String,
    rooms: Vec<Room>,
}

impl SmartHouse {
    /// Creates a new smart house with the specified name and rooms
    pub fn new(name: String, rooms: Vec<Room>) -> Self {
        Self { name, rooms }
    }

    /// Returns the name of the house
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns all rooms in the house
    pub fn all_rooms(&self) -> &[Room] {
        &self.rooms
    }

    /// Returns a reference to a specific room by index
    pub fn rooms(&self, index: usize) -> Result<&Room, AccessError> {
        self.rooms.get(index).ok_or_else(|| AccessError {
            resource_type: String::from("Room"),
            requested_index: index,
            total_count: self.rooms.len(),
        })
    }

    /// Returns a mutable reference to a specific room by index
    pub fn rooms_mut(&mut self, index: usize) -> Result<&mut Room, AccessError> {
        let total_rooms = self.rooms.len();

        self.rooms.get_mut(index).ok_or_else(|| AccessError {
            resource_type: String::from("Room"),
            requested_index: index,
            total_count: total_rooms,
        })
    }

    /// Generates a text report about the house and all its rooms and devices
    pub fn report(&self) -> String {
        // Estimate capacity for better string allocation performance
        let estimated_capacity =
            self.name.len() + self.rooms.iter().map(|r| r.report().len()).sum::<usize>() + 50;

        let mut report = String::with_capacity(estimated_capacity);

        report.push_str(&format!("=== Smart House: {} ===\n", self.name));

        for room in &self.rooms {
            report.push_str(&room.report());
            report.push('\n');
        }

        report
    }

    /// Adds a new room to the house
    pub fn add_room(&mut self, room: Room) {
        self.rooms.push(room);
    }

    /// Removes a room from the house by index
    pub fn remove_room(&mut self, index: usize) -> Result<Room, AccessError> {
        if index < self.rooms.len() {
            let room = self.rooms.remove(index);
            self.rooms.shrink_to_fit();
            Ok(room)
        } else {
            Err(AccessError {
                resource_type: String::from("Room"),
                requested_index: index,
                total_count: self.rooms.len(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::{SmartDevice, SmartSocket, SmartThermometer};
    use mockall::mock;

    // Create mock for Room
    mock! {
        pub Room {}
        impl Clone for Room {
            fn clone(&self) -> Self;
        }
    }

    fn create_empty_house() -> SmartHouse {
        SmartHouse::new(String::from("Test House"), vec![])
    }

    fn create_house_with_rooms() -> SmartHouse {
        let room1 = Room::new("Room 1".to_string(), vec![]);
        let room2 = Room::new("Room 2".to_string(), vec![]);
        SmartHouse::new("Test House".to_string(), vec![room1, room2])
    }

    fn create_house_with_devices() -> SmartHouse {
        // Create devices for rooms
        let living_room_devices = vec![
            SmartDevice::Thermometer(SmartThermometer::new(
                "Living Room Thermometer".to_string(),
                21.0,
            )),
            SmartDevice::Socket(SmartSocket::new(
                "Living Room Socket".to_string(),
                true,
                80.0,
            )),
        ];

        let bedroom_devices = vec![SmartDevice::Thermometer(SmartThermometer::new(
            "Bedroom Thermometer".to_string(),
            19.5,
        ))];

        // Create rooms
        let living_room = Room::new("Living Room".to_string(), living_room_devices);
        let bedroom = Room::new("Bedroom".to_string(), bedroom_devices);

        // Create house
        SmartHouse::new("Smart Home".to_string(), vec![living_room, bedroom])
    }

    #[test]
    fn test_house_creation() {
        let house = SmartHouse::new(String::from("Test House"), vec![]);
        assert_eq!(house.name(), "Test House");
        assert!(house.all_rooms().is_empty());
    }

    #[test]
    fn test_add_and_remove_room() {
        // Тест як табличний
        struct AddRemoveRoomTestCase {
            name: &'static str,
            initial_house: SmartHouse,
            operation: fn(&mut SmartHouse) -> (usize, bool),
            expected_rooms_count: usize,
            expected_success: bool,
        }

        let test_cases = vec![
            // Тест на додавання кімнати
            AddRemoveRoomTestCase {
                name: "Add a room to empty house",
                initial_house: create_empty_house(),
                operation: |house| {
                    let devices = vec![
                        SmartDevice::Thermometer(SmartThermometer::new(
                            "Thermo 1".to_string(),
                            22.0,
                        )),
                        SmartDevice::Socket(SmartSocket::new("Socket 1".to_string(), true, 100.0)),
                    ];
                    let room = Room::new("Test Room".to_string(), devices);

                    house.add_room(room);
                    (house.all_rooms().len(), true)
                },
                expected_rooms_count: 1,
                expected_success: true,
            },
            // Тест на видалення кімнати
            AddRemoveRoomTestCase {
                name: "Remove a room from house with rooms",
                initial_house: create_house_with_rooms(),
                operation: |house| match house.remove_room(0) {
                    Ok(_) => (house.all_rooms().len(), true),
                    Err(_) => (house.all_rooms().len(), false),
                },
                expected_rooms_count: 1,
                expected_success: true,
            },
            // Тест на видалення неіснуючої кімнати
            AddRemoveRoomTestCase {
                name: "Try to remove non-existent room",
                initial_house: create_house_with_rooms(),
                operation: |house| match house.remove_room(10) {
                    Ok(_) => (house.all_rooms().len(), true),
                    Err(_) => (house.all_rooms().len(), false),
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
        // Табличний тест для доступу до кімнат
        struct RoomAccessTestCase {
            name: &'static str,
            house: SmartHouse,
            room_index: usize,
            expected_result: Result<&'static str, ()>,
        }

        let test_cases = vec![
            RoomAccessTestCase {
                name: "Access first room",
                house: create_house_with_rooms(),
                room_index: 0,
                expected_result: Ok("Room 1"),
            },
            RoomAccessTestCase {
                name: "Access second room",
                house: create_house_with_rooms(),
                room_index: 1,
                expected_result: Ok("Room 2"),
            },
            RoomAccessTestCase {
                name: "Access non-existent room",
                house: create_house_with_rooms(),
                room_index: 2,
                expected_result: Err(()),
            },
        ];

        for tc in test_cases {
            let result = tc.house.rooms(tc.room_index);

            match (result, tc.expected_result) {
                (Ok(room), Ok(expected_name)) => {
                    assert_eq!(
                        room.name(),
                        expected_name,
                        "Failed test case '{}': Expected room name '{}' but got '{}'",
                        tc.name,
                        expected_name,
                        room.name()
                    );
                }
                (Err(_), Err(_)) => {
                    // Test passes - we expected an error and got one
                }
                (Ok(room), Err(_)) => {
                    panic!(
                        "Failed test case '{}': Expected error but got room '{}'",
                        tc.name,
                        room.name()
                    );
                }
                (Err(_), Ok(expected_name)) => {
                    panic!(
                        "Failed test case '{}': Expected room '{}' but got error",
                        tc.name, expected_name
                    );
                }
            }
        }
    }

    #[test]
    fn test_mutable_room_access() {
        // Табличний тест для мутабельного доступу до кімнат
        struct MutableRoomAccessTestCase {
            name: &'static str,
            room_index: usize,
            expected_success: bool,
        }

        let test_cases = vec![
            MutableRoomAccessTestCase {
                name: "Mutable access to first room",
                room_index: 0,
                expected_success: true,
            },
            MutableRoomAccessTestCase {
                name: "Mutable access to non-existent room",
                room_index: 10,
                expected_success: false,
            },
        ];

        for tc in test_cases {
            let mut house = create_house_with_rooms();
            let result = house.rooms_mut(tc.room_index);

            assert_eq!(
                result.is_ok(),
                tc.expected_success,
                "Failed test case '{}': Expected success {} but got {}",
                tc.name,
                tc.expected_success,
                result.is_ok()
            );
        }
    }

    #[test]
    fn test_report_generation() {
        // Тест генерації звіту
        struct ReportTestCase {
            name: &'static str,
            house: SmartHouse,
            expected_content: Vec<&'static str>,
        }

        let test_cases = vec![
            ReportTestCase {
                name: "Report for house with a thermometer",
                house: {
                    // Створюємо кімнату з термометром
                    let devices = vec![SmartDevice::Thermometer(SmartThermometer::new(
                        "Living Room Thermometer".to_string(),
                        22.5,
                    ))];
                    let room = Room::new("Living Room".to_string(), devices);
                    SmartHouse::new("Test House".to_string(), vec![room])
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
