fn main() {
    println!("Hello in the functions main function");
    another_function(5, 6);

    // blocked scope evaluation example
    let a = {
        let x = 5;
        x + 1
    };
    println!("The value of a is : {} ", a);
}

fn another_function(x: i32, y: i32) {
    println!("Hello from another function!");

    if x  > 3 {
        println!("The value of x is : {} ", x);
    } else {
        println!("x is less than or equal to 3: {} ", x);
    }

    println!("The value of y is : {} ", y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}