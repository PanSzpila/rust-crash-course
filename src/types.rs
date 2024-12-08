pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    //Add explicit type
    let z: i64 = 45454545454;

    //find max size
    println!("max i32: {}", std::i32::MAX);
    println!("max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    //Get boolean from expression

    let is_greater = 10 > 20;

    let a1 = 'a'; //character in singe quote only!!!!!
    let face = '\u{1F600}'; //its allso a character - smiling face

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face))
}
