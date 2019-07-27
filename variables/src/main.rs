fn main() {
//    const MAX_POINTS: u32 = 100_000;
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    let spaces = "     ";
    let spaces = spaces.len();

    println!("The value of x is {} ", x);

    let tup: (i32, f64, u8) = (500, 6.4, 1); // create a tuple type
    let ( a, b , c ) = tup; // deconstruct the values into individual parts
    println!("the value of b is : {} ", b); // print the b value from the tuple
    let new_tuple = (i64, f64, u32) = (200, 2.2, 100);
    let two_hundred = new_tuple.0;
    let two_point_two = new_tuple.1;
    let one_hundred = new_tuple.2;

    // array type
    let a: [i32; 10] = [1,2,3,4,5,6,7,8,9,10];
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    let array_b =[3; 5]; // [3, 3, 3, 3, 3]

    // access array with indexing
    let array_c = [1, 2 , 3, 4 , 5];
    let first = array_c[0];
    let second = array_c[1];
}
