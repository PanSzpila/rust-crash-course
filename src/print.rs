pub fn run() {
    // print to console
    println!("Hello from the print.rs file!");

    // Basic formating
    println!("{} is from {}", "Piotr", "Mars");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Piotr", "Mars", "code");

    // Named arguments
    println!("{name} likes to play {activity}", name = "Piotr", activity = "chess");

    //placeholder traits
    println!("Binary: {:b} Hex {:x} Octal: {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    //basic math
    println!("10 + 10 = {}", 10 + 10);
}
