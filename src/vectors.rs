use std::mem;
// Resize-able arrays
pub fn run(){
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    //Re-assign values
    numbers[2] = 18;

    // Add new item<number>
    numbers.push(6);
    numbers.push(7);

    // Removes last value
    numbers.pop();

    println!("{:?}", numbers);

    //Get single value
    println!("{}", numbers[0]);

    // Get array length
    println!("Vector length is {}", numbers.len());

    // Get array size
    println!("Vector for size: {}", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] =  &numbers[1..3];
    println!("Slice: {:?}", slice);

    // looop through vector values
    for x in numbers.iter() {
        println!("Numbers are : {}", x);
    }

    // loop and mutate values
    for x in numbers.iter_mut() {
        *x = *x + 2;
    }
    println!("New vector Numbers are : {:?}", numbers);
}