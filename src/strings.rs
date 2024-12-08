// Primitive str = immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - use when you need to modify or own string data

pub fn run() {
    let hello_primitive = "hello";
    let mut hello_growable = String::from("Hello ");

    println!("{:?}", (hello_primitive, hello_growable.clone())); // clone because we are using this variable multiple times

    hello_growable.push('W'); //push method is only for characters
    hello_growable.push_str("orld!"); // for pushing strings

    //Capacity in bytes
    println!("Capacity: {}", hello_growable.capacity());

    // Chceck if empty
    println!("Is Empty: {}", hello_growable.is_empty());

    println!("Length: {}", hello_growable.len())
}
