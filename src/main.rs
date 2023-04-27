use std::collections::HashMap;
use std::io;

enum Animal {
    Elephant,
    Giraffe,
    Monkey,
    Lion,
}

fn main() {
    let mut animal_map = HashMap::new();
    animal_map.insert("gray", Animal::Elephant);
    animal_map.insert("tall", Animal::Giraffe);
    animal_map.insert("funny", Animal::Monkey);
    animal_map.insert("fierce", Animal::Lion);
    animal_map.insert("elephant", Animal::Elephant);
    animal_map.insert("giraffe", Animal::Giraffe);
    animal_map.insert("monkey", Animal::Monkey);
    animal_map.insert("lion", Animal::Lion);

    println!("Welcome to 'Guess the Animal'!");
    println!("I'm thinking of an animal...");
    let mut guessed = false;
    while !guessed {
        println!("What animal am I thinking of? Enter a trait or the name of the animal: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        if let Some(animal) = animal_map.get(input) {
            println!(
                "Yes, I'm thinking of a {}!",
                match animal {
                    Animal::Elephant => "gray elephant",
                    Animal::Giraffe => "tall giraffe",
                    Animal::Monkey => "funny monkey",
                    Animal::Lion => "fierce lion",
                }
            );
            guessed = true;
        } else {
            let animal_option = animal_map.values().find(|&animal| match animal {
                Animal::Elephant if input == "gray" => true,
                Animal::Giraffe if input == "tall" => true,
                Animal::Monkey if input == "athletic" => true,
                Animal::Lion if input == "angry" => true,
                Animal::Elephant if input == "elephant" => true,
                Animal::Giraffe if input == "giraffe" => true,
                Animal::Monkey if input == "monkey" => true,
                Animal::Lion if input == "lion" => true,
                _ => false,
            });
            if let Some(animal) = animal_option {
                println!(
                    "Yes, I'm thinking of a {}!",
                    match animal {
                        Animal::Elephant => "gray elephant",
                        Animal::Giraffe => "tall giraffe",
                        Animal::Monkey => "athletic monkey",
                        Animal::Lion => "angry lion",
                    }
                );
                guessed = true;
            } else {
                println!(
                    "Sorry, I'm not thinking of an animal with that name or trait. Try again."
                );
            }
        }
    }
}
