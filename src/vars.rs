pub fn run() {
    let name = "Piotr"; // same as js const
    let mut age = 38; // same as js let
    age = 39;

    println!("My name is {} and I'm {}", name, age);

    // Define constant
    const ID: i32 = 001; // when you define the const, you have to explicitly define the type, usulay it's uppercase
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Stefan", 27);
    println!("My name is {} and I'm {}", my_name, my_age);
}
