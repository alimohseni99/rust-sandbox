use crate::print;

pub fn run(){ 
let name= "Ali";
let mut age: i32 = 25;

age = 26;
println!("My name is {} and I am {}", name, age);


const ID: i32 = 001;
println!("ID: {}", ID);


let (my_name, my_age) = ("Ali", 25);
println!("{} is {}", my_name, my_age);
} 