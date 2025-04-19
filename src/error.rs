// Original AccessError kept for backward compatibility
#[derive(Debug)]
pub struct AccessError {
    /// Type of resource being accessed (e.g., "Room", "Device")
    pub resource_type: String,
    /// Index that was requested
    pub requested_index: usize,
    /// Total number of available resources
    pub total_count: usize,
}

impl std::fmt::Display for AccessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} index {} is out of bounds. Total {}: {}",
            self.resource_type,
            self.requested_index,
            self.resource_type.to_lowercase(),
            self.total_count,
        )
    }
}

impl std::error::Error for AccessError {}

#[derive(Debug)]
pub enum DeviceAccessError {
    RoomNotFound(String),

    /// Device not found in a specific room
    /// Example: DeviceNotFound(device_name, room_name)
    DeviceNotFound(String, String),
}

impl std::fmt::Display for DeviceAccessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceAccessError::RoomNotFound(room_name) => {
                write!(f, "Room '{}' not found", room_name)
            }
            DeviceAccessError::DeviceNotFound(device_name, room_name) => {
                write!(
                    f,
                    "Device '{}' not found in room '{}'",
                    device_name, room_name
                )
            }
        }
    }
}

impl std::error::Error for DeviceAccessError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_access_error_creation() {
        let error = AccessError {
            resource_type: "Room".to_string(),
            requested_index: 5,
            total_count: 3,
        };

        assert_eq!(error.resource_type, "Room");
        assert_eq!(error.requested_index, 5);
        assert_eq!(error.total_count, 3);
    }

    #[test]
    fn test_error_trait_implementation() {
        let error = AccessError {
            resource_type: "Room".to_string(),
            requested_index: 5,
            total_count: 3,
        };

        // Check that our error type implements the Error trait
        let _: &dyn Error = &error;
    }

    #[test]
    fn test_debug_implementation() {
        let error = AccessError {
            resource_type: "Room".to_string(),
            requested_index: 5,
            total_count: 3,
        };

        let debug_output = format!("{:?}", error);

        assert!(debug_output.contains("resource_type"));
        assert!(debug_output.contains("Room"));
        assert!(debug_output.contains("requested_index"));
        assert!(debug_output.contains("5"));
        assert!(debug_output.contains("total_count"));
        assert!(debug_output.contains("3"));
    }

    #[test]
    fn test_with_unusual_resource_types() {
        let error = AccessError {
            resource_type: "SensorNode".to_string(),
            requested_index: 8,
            total_count: 4,
        };

        let error_message = format!("{}", error);
        assert_eq!(
            error_message,
            "SensorNode index 8 is out of bounds. Total sensornode: 4"
        );
    }

    #[test]
    fn test_with_empty_resource_type() {
        let error = AccessError {
            resource_type: "".to_string(),
            requested_index: 3,
            total_count: 2,
        };

        let error_message = format!("{}", error);
        assert_eq!(error_message, " index 3 is out of bounds. Total : 2");
    }

    #[test]
    fn test_with_zero_total_count() {
        let error = AccessError {
            resource_type: "Room".to_string(),
            requested_index: 0,
            total_count: 0,
        };

        let error_message = format!("{}", error);
        assert_eq!(
            error_message,
            "Room index 0 is out of bounds. Total room: 0"
        );
    }

    #[test]
    fn test_with_large_indices() {
        let error = AccessError {
            resource_type: "Room".to_string(),
            requested_index: usize::MAX,
            total_count: 1000000,
        };

        let error_message = format!("{}", error);
        assert_eq!(
            error_message,
            format!(
                "Room index {} is out of bounds. Total room: 1000000",
                usize::MAX
            )
        );
    }

    #[test]
    fn test_device_access_error() {
        let room_error = DeviceAccessError::RoomNotFound("Living Room".to_string());
        assert!(format!("{}", room_error).contains("Room 'Living Room' not found"));

        let device_error =
            DeviceAccessError::DeviceNotFound("Fridge".to_string(), "Kitchen".to_string());
        let error_msg = format!("{}", device_error);
        assert!(error_msg.contains("Device 'Fridge' not found in room 'Kitchen'"));

        // Check Error trait implementation
        let _: &dyn Error = &room_error;
        let _: &dyn Error = &device_error;

        let debug_output = format!("{:?}", room_error);
        assert!(debug_output.contains("RoomNotFound"));
        assert!(debug_output.contains("Living Room"));
    }
}
