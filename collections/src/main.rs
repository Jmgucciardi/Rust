use std::collections::HashMap;

fn main() {
    let v: Vec<i32> = Vec::new(); // empty vector ready to store i32 on the heap
    let v2 = vec![1,2,3]; // vector which holds 3 i32 values 1, 2 ,3 on the heap. vec! means rust will infer the type of vec to be the initial type
    // create vector and add elements to it

    {
        let mut v = Vec::new();

        v.push(5); // vector will now store i32 values by default once this integer is introduced. Rust infers the value
        v.push(6);
        v.push(7);
        v.push(8);
        println!("Vector: {:?}", v)
    } // just like before, this vector (v) goes out of scope here and is removed (dropped) from the heap

    // reading elements of a vector
    {
        let v = vec![1,2,3,4,5];

        let third: &i32 = &v[2]; // borrowing here with & syntax, transferring ownership to third
        println!("The third element is {}", third);

        match v.get(2) { // get method here will allow us to pass an index in and gives u an Option<&T>. Some or None
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element"),
        }
    }
    // access vector index that isn't there
    {
        let v = vec![1,2,3,4,5];
//        let does_not_exist = &v[100]; // this will throw an error , as index 100 does not exist
        let does_not_exist = v.get(100); // this will not error, as index 100 is not real, the Option<T> None will trigger and be returned
    }
    // can not have a mutable and immutable reference to a vector in the same scope
    {
        let mut v = vec![1,2,3,4,5]; //mutable vector
        let first = &v[0]; // first index of the mutable vector called here by reference, creating an immutable borrow
        v.push(6); // second call to the mutable reference, mutating original vector
//        println!("The first element is: {}", first); // ERROR: we're calling the immutable borrow after we mutated it the line above
    }

    // iterating over values in a vector
    {
        let v = vec![100, 32, 57];
        for i in &v { // for loop, i is index, &v is now borrowing hte value of v from above, v is dropped from memory for &v reference
            println!("{}", i)
        }

        // rust can also iterate over mutable references to each element , to change the elements
        let mut v1 = vec![100, 32, 57]; // mutable vector
        for i in &mut v1 { // borrowing the value of mutable vector with &mut v, mut v is dropped, &mut v has the memory
            *i += 50;
        } // &mut v1 goes out of scope from for loop
        println!("{:?}", v1) // call directly the value of v1 from above
    }

    // using enums to store multiple types
    // this sort of stores multiple types. It really stores type SpreadsheetCell. But as an enum, SpreadsheetCell keys can be of multiple types
    {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Float(10.12),
            SpreadsheetCell::Text(String::from("Baby Blue"))
        ];
    }

    // collection type: String, and all it's glory
    {
        let mut s = String::new(); // created memory on the heap for a string
        let data = "initial Contents"; // defaults to &str
        let string_thing = data.to_string();
        let other_string = "initial Content".to_string();
        let last_string = String::from("string literal"); // another way to create a string from a string literal
    }
    // updating a string
    {
        let mut s = String::from("foo");
        s.push_str("bar"); // uses push_string() to push "bar" to the end of "foo"

        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {}", s2);

        // push method will push a char to the end of a string, rather than the whole string
        let mut s2 = String::from("lo");
        s2.push('l');
        println!("s2: {}", s2);
    }
    // combining strings with the + operator, LIKE IN JAVASCRIPT? WHAT!?
    {
        let s1 = String::from("hello, ");
        let s2 = String::from("world");
        let s3 = s1 + &s2; // + operator becomes a concat type deal like from JS
        println!("s3: {}", s3)
    }
    // concat multiple strings the JS way and is lame
    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = s1 + "-" + &s2 + "-" + &s3;
        println!("s is: {}", &s);
    }
    // concat multiple strings the Rust way with the format! macro
    // easier to read and format! wont take ownership of any of its parameters
    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("s is: {}", s)
    }
    // slicing strings: Rust isn't trying to encourage you to find characters in strings. as it's expensive to determine exactly what is being returned
    // slicing strings in Rust should work like this
    {
        let hello = "Здравствуйте";
        let s = &hello[0..4]; // use ranges to create slices, with caution, doing so can still crash a program
    }
    // method for iterating over strings
    {
        // return chars
        for c in "नमस्ते".chars() {
//            println!("C: {}", c) // will print char by char
        }
        // return bytes
        for b in "नमस्ते".bytes() {
//            println!("B: {}", b); // prints the byte value
        }
    }

    // HASH MAPS! with insert
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        println!("scores: {:?}", scores); // object (or struct) with keys Yellow and Blue with values 50 and 10 respectively
    }
    // hash map with tuples
    {
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];

        // using HashMap<_, _> with underscores the collection will infer the types from the vector
        // zip method creates a vector of tuples, where "Blue" is paired with 10 and so forth
        // then we use the collect method to turn that vector of tuples into a hash map
        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    }
    // for types that implement the copy trait, like i32, the values are copied int0 the hash
    // for owned values like string, values are moved and the hash map will be the owner of the values
    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value); // creates map : { favorite color: Blue }
        println!("map: {:?}", map);
        // by this point field_name and field_value are invalid by this point
    }

    // accessing values in a hash map
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name); // score will = SOme(&10) because get is wrapper in Option<&V>, if no value is there, it returns None
        println!("Team: {:?}", &score);

        for (key, value) in &scores { // for loop, looking for the key and value of the object supplied after in
            println!("{}: {}", key, value);
        }
    }
    // over writing values in the hash map
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10); // blue: 10 here
        scores.insert(String::from("Blue"), 25); // blue is updated to 25 here
        println!("{:?}", scores);
    }
    // only insert a value if the key has no value
    // Rust has  an api for this. entry
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.entry(String::from("Yellow")).or_insert(50); // will create yellow key and give it 50 as a value
        scores.entry(String::from("Blue")).or_insert(50); // blue already exists so this won't execute anything

        println!("{:?}", scores);
    }
    // update the value based on an old value
    {
        let text = "Hello world wonderful world";

        let mut map = HashMap::new(); // create hash map

        for word in text.split_whitespace() { // for loop over the text string, finding whitespace
            let count = map.entry(word).or_insert(0); // take a count of the word, if it's new, add it to the hash, and give it a value of 0
            *count += 1 // take the value of count and at 1 to it, so if it's the first word its value goes from 0 to 1. If it already exists, it adds 1 to existing value
        }
        println!("{:?}", map);
    }
}
