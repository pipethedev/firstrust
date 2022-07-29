// Group of values
// Max: 12 elements
pub fn run() {
    let person: (&str, &str, i32) = ("Ileriolwa", "Kwara", 19);

    println!("{} is from {} and he is {} years old", person.0, person.1, person.2);
}