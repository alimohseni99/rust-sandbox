pub fn run(){
    let mut count = 1;

     while count <= 100{
        if count % 15 == 0 {
            println!("FizzBuzz");
        }else if count % 3 == 0 {
            println!("fizz");
        }else if count % 5 == 0 {
            println!("buzz");
        }
        else {
            println!("{}", count);
        }
        count += 1;
     }
}