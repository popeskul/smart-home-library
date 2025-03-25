// Export all device-related modules and types
pub(crate) mod device_trait;
mod smart_device;
mod socket;
mod thermometer;

// Re-export for easier access
pub use device_trait::SmartDeviceTrait;
pub use smart_device::SmartDevice;
pub use socket::SmartSocket;
pub use thermometer::SmartThermometer;
