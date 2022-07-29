pub fn run() {
    //Print to console
    println!("Hello world from print");

    //Basic formatting
    println!("Number: {}", 1);
    println!("My name is {} and {} years old", "David", 18);

    //Positional Arguments 
    println!("{0} is from {1} and likes to {2}", "David", "Lagos", "code");

    //Named arguments
    println!("{name} likes to play {sport}", name = "David", sport = "Football");

    //Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octo: {:o}", 10, 10, 10);

    //Placeholder for debug traits
    println!("{:?}", (12, true, "hello"));

    //Basic math
    println!("14 + 7 = {}", 14 + 7);
}