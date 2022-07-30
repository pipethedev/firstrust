use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();

    let name: &str = "Ileri";
    let status: &str = "100%";

    // println!("Command: {:?}", command);
    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Hi status is {}", status);
    } else {
        println!("You are running cargo [Invalid command]");
    }
}