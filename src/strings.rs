use crate::print;


pub fn run(){

    let mut hello = String::from("Hello ");


    //get length
    println!("Length: {}", hello.len());
    hello.push('W');
    hello.push_str( "orld!");
    
    // check if empty
    println!("Is Empty: {}", hello.is_empty());

    // capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Contains 
    println!("Contains 'World' {}", hello.contains("World"));

    // replace
    println!("Replace: {}", hello.replace("World", "There"));

    // loop through string by whitespace
    for word in hello.split_whitespace(){
        println!("{}", word);
    }

    // create string with capacity
    let mut h = String::with_capacity(10);

    h.push('a');
    h.push('b');

    // Assertion testing
    assert_eq!(23, h.len());


    println!("{}", hello);
    println!("{}", h);

}