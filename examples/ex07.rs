



struct Car {
    model: String,
    year: u16,
    seating_capacity: u8,
}

struct Truck {
    model: String,
    year: u16,
    cargo_capacity: u32,
}


enum Vehicle {
    Car(Car),
    Truck(Truck),
}

impl Vehicle {
    
    
    // Exemplo de função construtora
    fn new_car(model: String, year: u16, seating_capacity: u8) -> Self {
        let car: Vehicle = Vehicle::Car(Car { model, year, seating_capacity });
        car.print_info(); // Chama print_info ao criar a instância
        car
    }

    fn new_truck(model: String, year: u16, cargo_capacity: u32) -> Self {
        let truck: Vehicle = Vehicle::Truck(Truck { model, year, cargo_capacity });
        truck.print_info();
        truck
    }


    fn print_info(&self) {
        match self {
            Vehicle::Car(car) => {
                println!("model: {}\nyear: {}\nseating_capacity: {}", car.model, car.year, car.seating_capacity); print_line();
            }
            Vehicle::Truck(truck) => {
                println!("model: {}\nyear: {}\ncargo_capacity: {}", truck.model, truck.year, truck.cargo_capacity); print_line();
            }
        }
    }
}

fn print_line() {
    println!("-- -- --");
}

fn main() {

    let car: Vehicle = Vehicle::new_car(String::from("Honda"), 2015,    5);
    let truck: Vehicle = Vehicle::new_truck( String::from("Ford"), 2018, 15000 );

    let vehicles:Vec<Vehicle> = vec![car, truck];

    for vehicle in vehicles {
        match vehicle {
            Vehicle::Car(car) => println!("{} --> 🚗 Car", car.model),
            Vehicle::Truck(truck) => println!("{} --> 🚛 Truck", truck.model), 
        }
    }

}

