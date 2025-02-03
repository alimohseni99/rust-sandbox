pub fn run(){
    let age = 18;

    if age >= 21 {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && age >= 18 {
        println!("Bartender: Sorry, you need to be 21 to drink.");
    } else {
        println!("Bartender: Sorry, you need to be 18 to enter.");
    }

    // Shorthand if
    let is_of_age = if age >= 19 { true } else { false };
    println!("Is of age: {}", is_of_age);
}