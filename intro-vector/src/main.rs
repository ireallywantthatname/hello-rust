// this will be used in the end
#[derive(Debug)]
enum Metahumans {
    Heroes(String),
    Villians(String),
}

fn main() {
    // trying out vectors with Option<T>
    let mut superheroes: Vec<String> = Vec::new();
    superheroes.push(String::from("Batman"));
    superheroes.push(String::from("Superman"));
    superheroes.push(String::from("Wonder Woman"));

    println!("{:?}", &superheroes);

    let second_superhero: Option<&String> = superheroes.get(1);

    match second_superhero {
        Some(superhero) => println!("{} is here don't worry everybody.", superhero),
        None => println!("You will have to protect yourselves."),
    }

    // trying out the vec! macro
    let mut enemies = vec![
        String::from("Joker"),
        String::from("Lex Luthor"),
        String::from("Cheetah"),
    ];

    for enemy in &mut enemies {
        enemy.push_str(" is bad.");
    }

    println!("{:?}", enemies);

    // trying out vectors with enums
    let mut people: Vec<Metahumans> = Vec::new();

    people.push(Metahumans::Heroes(String::from("Mister Terrific")));
    people.push(Metahumans::Villians(String::from("Guy Gardner")));

    for person in &people {
        match person {
            Metahumans::Heroes(hero) => println!("Thank god! {} is here.", hero),
            Metahumans::Villians(villian) => println!("We are screwed, {} is here.", villian),
        }
    }

    println!("{:?}", people);
}
