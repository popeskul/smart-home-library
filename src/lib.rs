//! Smart Home Management Library
//!
//! This library provides tools for managing a smart home system
//! with various device types and room configurations.

// Export all modules
pub mod device;
mod error;
pub mod house;
pub mod report;
pub mod room;

// Re-export main types for easier access
pub use device::{SmartDevice, SmartDeviceTrait, SmartSocket, SmartThermometer};
pub use house::SmartHouse;
pub use report::Reporter;
pub use room::Room;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::device_trait::{PowerConsumption, PowerControl};
    use std::collections::HashMap;

    #[test]
    fn smart_socket_power_consumption() {
        let mut socket = SmartSocket::new(String::from("Test Socket"), true, 100.0);
        assert_eq!(socket.power_consumption(), 100.0);

        socket.turn_off();
        assert_eq!(socket.power_consumption(), 0.0);
    }

    #[test]
    fn room_device_access() {
        let thermo = SmartThermometer::new(String::from("Test Thermo"), 22.5);
        let socket = SmartSocket::new(String::from("Test Socket"), true, 100.0);

        let mut devices = HashMap::new();
        devices.insert("Test Thermo".to_string(), SmartDevice::Thermometer(thermo));
        devices.insert("Test Socket".to_string(), SmartDevice::Socket(socket));

        let room = Room::new(String::from("Test Room"), devices);

        assert!(room.device(&"Test Thermo".to_string()).is_some());
        assert!(room.device(&"Test Socket".to_string()).is_some());
        assert!(room.device(&"Non-existent Device".to_string()).is_none());

        assert_eq!(
            room.get_temperature(&"Test Thermo".to_string()).unwrap(),
            Some(22.5)
        );
        assert_eq!(
            room.get_power_consumption(&"Test Socket".to_string())
                .unwrap(),
            Some(100.0)
        );
    }

    #[test]
    fn house_room_access() {
        let room1 = Room::new(String::from("Room 1"), HashMap::new());
        let room2 = Room::new(String::from("Room 2"), HashMap::new());

        let mut rooms = HashMap::new();
        rooms.insert("Room 1".to_string(), room1);
        rooms.insert("Room 2".to_string(), room2);

        let house = SmartHouse::new(String::from("Test House"), rooms);

        assert!(house.room(&"Room 1".to_string()).is_some());
        assert!(house.room(&"Room 2".to_string()).is_some());
        assert!(house.room(&"Non-existent Room".to_string()).is_none());
    }
}
