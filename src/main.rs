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

    println!("Welcome to 'Guess the Animal'!");
    println!("I'm thinking of an animal...");
    let mut guessed = false;
    while !guessed {
        println!("What animal am I thinking of? Enter a trait of the animal: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if let Some(animal) = animal_map.get(input) {
            println!("Yes, I'm thinking of a {}!", match animal {
                Animal::Elephant => "gray elephant",
                Animal::Giraffe => "tall giraffe",
                Animal::Monkey => "funny monkey",
                Animal::Lion => "fierce lion",
            });
            guessed = true;
        } else {
            println!("Sorry, I'm not thinking of an animal with that trait. Try again.");
        }
    }
}
