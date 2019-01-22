use std::io;


fn get_number_from_user_or_zero() -> u32 {
    let mut input = String::new();
    let input = match io::stdin().read_line(&mut input)
    {
        Ok(_x) => match input.trim().parse() {
            Ok(x) => x,
            Err(_e) => 0
        },
        Err(_e) => 0
    };
    input
}

fn fibonacci(x: u32) -> u32{
    if x == 0 {
        0
    }
    else if x == 1 {
        1
    }
    else{
        fibonacci(x -1) + fibonacci(x-2)
    }
}

fn main() {
    println!("Input which fibonacci number you want: ");
    let nth_fibonacci = get_number_from_user_or_zero();

    println!("The {}th fibonacci number is {}",nth_fibonacci, fibonacci(nth_fibonacci));
}
