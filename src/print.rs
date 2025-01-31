pub fn run(){
    println!("Hello from print.rs file");

    // Basic formatting
    println!("{} tycker om {}", "Ali", "Rust");

    // Positional arguments
    println!("{0} tycker om {1} och {1} tycker om {2}", "Ali", "Rust", "TS");

    // Named arguments
    println!("{name} tycker om {language}", name = "Ali", language = "Rust");
}