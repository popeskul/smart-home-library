# Smart Home Management System 🏠🔌🌡️

## Overview

Smart Home is a Rust-based library for managing and interacting with smart home devices. It provides a flexible and type-safe approach to modeling smart home environments, supporting various device types and operations.

## Features

- 🔌 **Smart Sockets**: Control and monitor power consumption
- 🌡️ **Smart Thermometers**: Track room temperatures
- 🏘️ **Room and House Management**: Organize devices into logical structures
- 🛡️ **Error Handling**: Robust error management with custom error types
- 🧪 **Comprehensive Testing**: Extensive unit tests covering all functionality

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
smart_home = { git = "https://github.com/popeskul/smart-home-library" }
```

## Quick Start

```rust
use smart_home::{SmartHouse, Room, SmartDevice, SmartSocket, SmartThermometer};

fn main() {
    // Create devices
    let living_room_socket = SmartSocket::new(
        String::from("Living Room Socket"), 
        true, 
        120.0
    );
    
    let bedroom_thermometer = SmartThermometer::new(
        String::from("Bedroom Thermometer"), 
        22.5
    );

    // Create rooms
    let living_room = Room::new(
        String::from("Living Room"),
        vec![SmartDevice::Socket(living_room_socket)]
    );

    let bedroom = Room::new(
        String::from("Bedroom"),
        vec![SmartDevice::Thermometer(bedroom_thermometer)]
    );

    // Create a smart house
    let mut house = SmartHouse::new(
        String::from("My Smart Home"), 
        vec![living_room, bedroom]
    );

    // Generate a report
    println!("{}", house.report());
}
```

## Key Concepts

### SmartDevice
An enum representing different types of smart devices:
- `Thermometer`: Measures temperature
- `Socket`: Controls power and measures consumption

### Room
A collection of smart devices with methods to:
- Add/remove devices
- Access devices
- Generate reports

### SmartHouse
Manages multiple rooms with capabilities to:
- Add/remove rooms
- Generate comprehensive reports
- Safely access rooms and devices

## Error Handling

The library uses a custom `AccessError` for safe device and room access:

```rust
match house.rooms(2) {
    Ok(room) => println!("Room accessed"),
    Err(e) => println!("Error: {}", e),
}
```

## Device Traits

- `SmartDeviceTrait`: Base functionality for all devices
- `PowerControl`: Turn devices on/off
- `TemperatureSensor`: Get temperature readings
- `PowerConsumption`: Measure power usage

## Testing

Extensive test coverage includes:
- Unit tests for each component
- Behavior testing
- Error case validation

Run tests with:
```bash
cargo test
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Performance Considerations

- Efficient memory management
- Minimal overhead
- Zero-cost abstractions typical of Rust

## Future Roadmap

- [ ] Add more device types
- [ ] Implement device communication protocols
- [ ] Create a web/mobile interface
- [ ] Add persistent storage support

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Makefile Commands

The project includes a Makefile to simplify common development tasks:

| Command        | Description                                     |
|---------------|-------------------------------------------------|
| `make help`   | Show all available commands                     |
| `make build`  | Compile the project                             |
| `make test`   | Run all unit tests                              |
| `make clean`  | Remove build artifacts                          |
| `make format` | Format code using rustfmt                       |
| `make lint`   | Run clippy for static code analysis             |
| `make doc`    | Generate and open project documentation         |
| `make run`    | Execute the main binary                         |
| `make all`    | Build, test, and lint the project               |
| `make update` | Update project dependencies                     |
| `make release`| Create an optimized release build               |

### Quick Start with Makefile

```bash
# Show available commands
make help

# Build the project
make build

# Run tests
make test

# Check code quality
make lint

# Generate documentation
make doc
```
