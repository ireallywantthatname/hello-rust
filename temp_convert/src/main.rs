use std::io;

fn main() {
    let temp = temp_convert();
    println!("Temp is: {}", temp);
}

fn temp_convert() -> f32 {
    println!("1. fah -> cel");
    println!("2. cel -> fah");

    println!("Enter a choice: ");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Enter a valid choice.");

    println!("Enter a temp: ");

    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Error getting the temp.");

    let temp: f32 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };

    if choice.trim() == "1" {
        (temp - 32.0) * 5.0 / 9.0
    } else if choice.trim() == "2" {
        temp * 9.0 / 5.0 + 32.0
    } else {
        temp
    }
}
