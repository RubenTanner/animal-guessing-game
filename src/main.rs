fn car_factory() {
    // Create car color array
    // let colors = todo!("Set the enum values: 0 = Blue, 1 = Green, 2 = Red, 3 = Silver");
    let colors = ["Blue", "Green", "Red", "Silver"];

    // Declare the car type and initial values
    let mut car: Car;  
    let mut engine = Transmission::Manual;

    #[derive(PartialEq, Debug)]
    // Declare Car struct to describe vehicle with four named fields
    struct Car {
        color: String,
        motor: Transmission,
        roof: bool,
        age: (Age, u32),
    }

    #[derive(PartialEq, Debug)]
    // Declare enum for Car transmission type
    enum Transmission {
        Manual,
        SemiAuto,
        Automatic,
    }
    #[derive(PartialEq, Debug)]
    enum Age {
        New,
        Used,
    }

    // Get the car quality by testing the value of the input argument
    // - miles (u32)
    // Create a tuple for the car quality with the Age ("New" or "Used") and mileage
    // Return a tuple with the arrow `->` syntax
    fn car_quality(miles: u32) -> (Age, u32) {
        // Declare and initialize the return tuple value
        // For a new car, set the miles to 0
        let quality = (Age::New, miles);
        quality
    }
    // Build a new "Car" using the values of four input arguments
    // - color (String)
    // - motor (Transmission enum)
    // - roof (boolean, true if the car has a hard top roof)
    // - miles (u32)
    // Call the car_quality(miles) function to get the car age
    // Return an instance of a "Car" struct with the arrow `->` syntax
    fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
        // Create a new "Car" instance as requested
        // - Bind first three fields to values of input arguments
        // - "age" field calls "car_quality" function with "miles" input argument
        Car {
            color: color,
            motor: motor,
            roof: roof,
            age: car_quality(miles),
        }
    }

    // Order 3 cars, one car for each type of transmission
   
    // Car order #1: New, Manual, Hard top
    car = car_factory(String::from(colors[0]), engine, true, 0);
    println!("Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);
    
    // Car order #2: New, Semi-automatic, Convertible
    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[1]), engine, false, 100);
    println!("Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);

    // Car order #3: New, Automatic, Hard top
    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[2]), engine, true, 200);
    println!("Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);
}



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
