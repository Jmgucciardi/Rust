fn main() {
    let v: Vec<i32> = Vec::new(); // empty vector ready to store i32 on the heap
    let w = vec![1,2,3]; // vector which holds 3 i32 values 1, 2 ,3 on the heap. vec! means rust will infer the type of vec to be the initial type
    // create vector and add elements to it

    {
        let mut v = Vec::new();

        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
        println!("Vector: {:?}", v)
    }

}
