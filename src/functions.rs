pub fn run() {
    let greet = String::from("Hello");
    let name = String::from("Bitch");

    greeting(greet, name);

    let sum = add(4, 9);
    println!("{}", sum);

    //Closure
    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("Closure answer is {}", add_nums(3, 3));
}

fn greeting(greet: String, name: String) {
    println!("{} {} nice to meet you", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    return n1 + n2;
} 