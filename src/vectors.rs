//Arrays - fixed list where elements are the same data types

use std::mem;

pub fn run() {
    let mut numbers: Vec<i8> = vec![1, 2, 3, 4, 5];

    // Re-assign value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(6);
    numbers.push(7);

    // Popoff last value
    numbers.pop();

    println!("{:?}", numbers);

    //Get single val
    println!("Single Value: {}", numbers[0]);

    //Get vector length
    println!("vector Length: {}", numbers.len());

    //vectors are stack allocated
    println!("vector ocupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i8] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    //Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut() {
        //similar to .map in js
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);
}
