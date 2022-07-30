pub fn run() {
    let age: i32 = 18;
    let check_id: bool = true;
    let knows_person = true;

    if age >= 21 && check_id || knows_person {
        println!("Omo OG you are {} and old", age);
    } else if age < 21 && check_id {
        println!("Young lad");
    } else {
        println!("I will need your ID !");
    }

    // Shorthand IF
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);
}