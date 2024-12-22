use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone(); //można wyciągać wyrazy wpisane np po cargo run
    let name = "Kocinka";
    let status = "100%";

    println!("Args: {:?}", args);
    println!("command: {:?}", command);

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("status is {}", status);
    } else {
        println!("That is not a valid command");
    }
}
