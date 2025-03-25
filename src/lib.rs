//! Smart Home Management Library
//!
//! This library provides tools for managing a smart home system
//! with various device types and room configurations.

// Export all modules
pub mod device;
pub mod error;
pub mod house;
pub mod room;

// Re-export main types for easier access
pub use device::{SmartDevice, SmartDeviceTrait, SmartSocket, SmartThermometer};
pub use error::AccessError;
pub use house::SmartHouse;
pub use room::Room;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::device_trait::{PowerConsumption, PowerControl};

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

        let room = Room::new(
            String::from("Test Room"),
            vec![
                SmartDevice::Thermometer(thermo),
                SmartDevice::Socket(socket),
            ],
        );

        assert!(room.devices(0).is_ok());
        assert!(room.devices(1).is_ok());
        assert!(room.devices(2).is_err());

        assert_eq!(room.get_temperature(0).unwrap(), Some(22.5));
        assert_eq!(room.get_power_consumption(1).unwrap(), Some(100.0));
    }

    #[test]
    fn house_room_access() {
        let room1 = Room::new(String::from("Room 1"), vec![]);
        let room2 = Room::new(String::from("Room 2"), vec![]);

        let house = SmartHouse::new(String::from("Test House"), vec![room1, room2]);

        assert!(house.rooms(0).is_ok());
        assert!(house.rooms(1).is_ok());
        assert!(house.rooms(2).is_err());
    }
}
