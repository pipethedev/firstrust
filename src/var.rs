//You can't re-assign variables

pub fn run() {
    let name = "Ileri";
    let mut age = 18;

    age = 19;

    println!("My age is {}", age);

    println!("My name is {}", name);

    //Defining constants
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assing multiple variables
    let (my_name, my_age) = ("Ileri", 18);
    println!("My name is {} is {}", my_name, my_age);

}