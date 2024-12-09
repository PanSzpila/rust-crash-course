//Reference pointers - Point to a resource in memory

pub fn run() {
    // Primitive Array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("values: {:?}", (arr1, arr2));

    //Vector (non primitive value)
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1; //there must be & as a reference

    println!("values: {:?}", (&vec1, vec2));
}
