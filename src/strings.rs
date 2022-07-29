pub fn run() {
    let mut name = String::from("Ileri");
    println!("Length: {}", name.len());

    //Push character
    name.push('A');

    name.push_str("oluwa");

    //Capacity in bytes
    println!("Capactiy in bytes: {}", name.capacity());

    //Check if empty
    println!("Is Empty: {}", name.is_empty());

    //Contains
    println!("Contains 'oluwa': {}", name.contains("oluwa"));

    //Replace
    println!("Replace: {}", name.replace("oluwa", "yomi"));

    //loop through string
    for word in name.split_whitespace() {
        println!("{}", word);
    }

    //Create string with certain capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    //Assertion testing
    assert_eq!(2, s.len());

    //assert_eq!(10, s.len());
    
    println!("{}", name);
}