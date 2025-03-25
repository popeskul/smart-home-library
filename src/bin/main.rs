use smart_home::{Room, SmartDevice, SmartHouse, SmartSocket, SmartThermometer};

fn main() {
    // Create devices for the first room
    let thermo1 = SmartThermometer::new(String::from("Living Room Thermometer"), 22.5);
    let socket1 = SmartSocket::new(String::from("Living Room Socket 1"), true, 120.0);
    let socket2 = SmartSocket::new(String::from("Living Room Socket 2"), false, 0.0);

    // Create devices for the second room
    let thermo2 = SmartThermometer::new(String::from("Bedroom Thermometer"), 21.0);
    let socket3 = SmartSocket::new(String::from("Bedroom Socket"), true, 80.0);

    let living_room = Room::new(
        String::from("Living Room"),
        vec![
            SmartDevice::Thermometer(thermo1),
            SmartDevice::Socket(socket1),
            SmartDevice::Socket(socket2),
        ],
    );

    let bedroom = Room::new(
        String::from("Bedroom"),
        vec![
            SmartDevice::Thermometer(thermo2),
            SmartDevice::Socket(socket3),
        ],
    );

    let mut house = SmartHouse::new(String::from("My Smart House"), vec![living_room, bedroom]);

    println!("Initial state:");
    println!("{}", house.report());

    println!("\nTurning off Living Room Socket 1...");
    if let Ok(room) = house.rooms_mut(0) {
        if let Ok(result) = room.turn_off_device(1) {
            if result {
                println!("Socket turned off successfully");
            } else {
                println!("Device does not support turning off");
            }
        } else {
            println!("Failed to access the device");
        }
    } else {
        println!("Failed to access the room");
    }

    println!("\nUpdated state:");
    println!("{}", house.report());

    println!("Checking temperature in bedroom...");
    if let Ok(room) = house.rooms(1) {
        if let Ok(temperature) = room.get_temperature(0) {
            if let Some(temp) = temperature {
                println!("Current temperature: {}°C", temp);
            } else {
                println!("Device does not support temperature readings");
            }
        } else {
            println!("Failed to access the device");
        }
    }

    println!("\nChecking power consumption in bedroom...");
    if let Ok(room) = house.rooms(1) {
        if let Ok(power) = room.get_power_consumption(1) {
            if let Some(watts) = power {
                println!("Current power consumption: {}W", watts);
            } else {
                println!("Device does not support power consumption readings");
            }
        } else {
            println!("Failed to access the device");
        }
    }

    println!("\nTrying to access a non-existent room...");
    match house.rooms(10) {
        Ok(_) => println!("Successfully accessed the room"),
        Err(e) => println!("Error: {}", e),
    }

    println!("\nTurning on Living Room Socket 1...");
    if let Err(e) = turn_on_device(&mut house, 0, 1) {
        println!("Failed to turn on device: {}", e);
    } else {
        println!("Socket turned on successfully");
    }

    println!("\nFinal state:");
    println!("{}", house.report());
}

// Helper function to demonstrate error handling
fn turn_on_device(
    house: &mut SmartHouse,
    room_idx: usize,
    device_idx: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let room = house.rooms_mut(room_idx)?;
    let success = room.turn_on_device(device_idx)?;

    if !success {
        return Err("Device does not support turning on".into());
    }

    Ok(())
}
