fn main() {
    loop_check(30);
}

fn loop_check(n: u32) {
    for n in 1..n + 1 {
        call_loop(n);
    }
}

fn call_loop(n: u32) {
    match (n % 3, n % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", n)
    }
    
    // if n % 15 == 0 {
    //     println!("POPROCK");
    // } else if n % 3 == 0 {
    //     println!("POP");
    // } else if n % 5 == 0 {
    //     println!("ROCK");
    // } else {
    //     println!("{}", n);
    // }
}