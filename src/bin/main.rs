use smart_home::{
    Reporter, Room, SmartDevice, SmartDeviceTrait, SmartHouse, SmartSocket, SmartThermometer,
    create_room,
};

fn demonstrate_dynamic_rooms_and_devices() {
    println!("\n=== Демонстрація динамічного управління кімнатами та пристроями ===");

    let mut house = SmartHouse::new_empty(String::from("Dynamic Smart Home"));

    let living_room = create_room!(
        "Living Room",
        (
            "TV Socket",
            SmartSocket::new(String::from("TV Socket"), true, 50.0)
        ),
        (
            "Room Thermometer",
            SmartThermometer::new(String::from("Living Room Thermo"), 22.5)
        )
    );

    let bedroom = create_room!(
        "Bedroom",
        (
            "Desk Lamp",
            SmartSocket::new(String::from("Desk Lamp"), false, 10.0)
        )
    );

    house.add_room("Living Room".to_string(), living_room);
    house.add_room("Bedroom".to_string(), bedroom);

    println!("Initial House State:");
    println!("{}", house.report());

    if let Some(removed_room) = house.remove_room(&"Bedroom".to_string()) {
        println!("\nВидалена кімната: {}", removed_room.name());
    }

    if let Some(living_room) = house.room_mut(&"Living Room".to_string()) {
        let new_socket = SmartSocket::new(String::from("Ceiling Light"), true, 20.0);
        living_room.add_device("Ceiling Light".to_string(), new_socket.into());

        if let Some(removed_device) = living_room.remove_device(&"TV Socket".to_string()) {
            println!("\nВидалений пристрій: {}", removed_device.name());
        }
    }

    println!("\nFinal House State:");
    println!("{}", house.report());
}

fn demonstrate_error_handling() {
    println!("\n=== Демонстрація обробки помилок ===");

    let house = SmartHouse::new_empty(String::from("Error Handling Demo"));

    match house.device(&"Non-existent Room".to_string(), &"Some Device".to_string()) {
        Ok(_) => println!("Несподіваний успіх"),
        Err(e) => println!("Помилка (очікувана): {}", e),
    }
}

fn print_report<T: Reporter>(reportable: &T) {
    println!("\n=== Звіт ===");
    println!("{}", reportable.report());
}

fn main() {
    demonstrate_dynamic_rooms_and_devices();
    demonstrate_error_handling();

    let thermometer = SmartThermometer::new(String::from("Main Thermometer"), 23.5);
    let socket = SmartSocket::new(String::from("Main Socket"), true, 100.0);
    let device = SmartDevice::Thermometer(thermometer);

    print_report(&device);
    print_report(&socket);
}
