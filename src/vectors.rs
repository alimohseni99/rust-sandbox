

pub fn run (){
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6];

    numbers[2] = 25;

    println!("{:?}", numbers);
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));

    for x in numbers.iter_mut()  {
        println!("numbers {}", x);
    }

    for x in numbers.iter_mut() {
        *x *= 3;
    }

    println!("{:?}", numbers);
}