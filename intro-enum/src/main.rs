#[derive(Debug)]
enum CarModel {
    Toyota,
    Honda,
    Ford,
}

#[derive(Debug)]
enum Vehicle {
    Bicycle,
    Mortorcycle,
    Car(CarModel, u16),
}

fn vehicle_description(vehicle: &Vehicle) -> String {
    match vehicle {
        Vehicle::Bicycle => String::from("This is a bicycle"),
        Vehicle::Mortorcycle => String::from("This is a mortorcycle"),
        Vehicle::Car(model, year) => {
            format!("This is a {} from {},", format!("{:?},", model), year)
        }
    }
}

fn main() {
    let bicycle = Vehicle::Bicycle;
    let mortorcycle = Vehicle::Mortorcycle;
    let car = Vehicle::Car(CarModel::Honda, 2024);

    println!("{}", vehicle_description(&bicycle));
    println!("{}", vehicle_description(&mortorcycle));
    println!("{}", vehicle_description(&car));
}
