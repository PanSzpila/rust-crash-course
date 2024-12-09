//Conditionals - Used to check the condition of something and act on the result

pub fn run() {
    let age: u8 = 19;
    let check_id: bool = false;

    if age >= 18 && check_id {
        println!("Bartender: What would you like to drink?");
    } else if age < 18 && check_id {
        println!("Bartender: Sorry, you have to leave");
    } else {
        println!("Bartender: I'll need to see your ID");
    }

    // Shorthand If
    let is_of_age = if age >= 18 { true } else { false };
    println!("Is Of Age: {}", is_of_age);
}
