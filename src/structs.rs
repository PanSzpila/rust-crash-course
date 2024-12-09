//Traditional Struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

//Tuple Struct
struct Color2(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    //Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
}

pub fn run() {
    // let mut c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0,
    // };

    // c.red = 200;

    // println!("color: {} {} {}", c.red, c.green, c.blue);

    let mut c2 = Color2(255, 0, 0);
    c2.0 = 200;

    println!("color: {} {} {}", c2.0, c2.1, c2.2);
}
