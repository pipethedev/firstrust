pub fn run() {
    //Integer 32
    let x = 1;

    //Float 64
    let y = 2.5;

    //Explict type
    let y: i64 = 454545454545;

    //Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //Boolean
    let is_active: bool = true;

    //Get boolean from expression
    let is_greater: bool = 10 > 5;

    //Characters
    let single = 'a';
    let  emoji = '\u{1F601}';
    println!("{:?}", (x, y, is_active, is_greater, single, emoji));

}