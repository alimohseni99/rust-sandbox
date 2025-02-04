pub fn run(){
    let name = String::from("Ali");

    let another_new_name = new_name(name);

    println!("New Name {}",another_new_name);
}
fn new_name(name: String) -> String {
    println!("Name: {}", name);
    name 
}
