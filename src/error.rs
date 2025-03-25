// Error type for resource access (rooms or devices)
#[derive(Debug)]
pub struct AccessError {
    pub resource_type: String,
    pub requested_index: usize,
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

        // Debug output should contain all fields
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
}
