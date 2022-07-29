use std::mem;

pub fn run(){
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    //Re-assign values
    numbers[2] = 18;

    println!("{:?}", numbers);

    //Get single value
    println!("{}", numbers[0]);

    // Get array length
    println!("{}", numbers.len());

    // Get array size
    println!("Array for size: {}", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] =  &numbers[1..3];
    println!("Slice: {:?}", slice);
}