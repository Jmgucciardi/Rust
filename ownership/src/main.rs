fn main() {
    // ownership
    // memory is automatically returned once the variable that owns it goes out of scope
    let _s = "hello"; // stored on the stack
    let _string = String::from("hello"); // sored on the heap, create string literal using from function
    let mut string_mut = String::from("hello");
    let _value = String::new(); // stored on the heap, just create a new string

    string_mut.push_str(", world"); // push_str() will push literal to a String
    println!("{}", string_mut);

    // this will throw an error with the way memory is allocated in Rust
    let name = String::from("Jon"); // name is a new string literal stored on the heap
    let _copy_of_name = name; // copy name to the new variable copy_of-name

//    println!("name: {}", name) // if you try and run this line, rust says original instance of name is now invalid. Don't reference it, instead, reference copy

    // if you do want to do a deep copy of the data onto the heap and not just the stack data, use the clone method.

    let _deep_copy_original = String::from("copy this");
    let _deep_copy_clone = _deep_copy_original.clone(); // deep copies are expensive in any languages.

    println!("original: {}, clone: {}", _deep_copy_original, _deep_copy_clone);

    // integer copies. types like integers that have a known size at compile time are stored entirely on the stack.
    {
        let x = 5;
        let y = x;
        println!("x: {}, y: {}", x ,y);
    }

    // ownership and functions
    {
        let s = String::from("Json Jon"); // string comes into scope

        takes_ownership_of_string(s); // s's value moves into the function
                                                // ... and so is no longer valid here

        let x = 5;                          // x comes into scope
        makes_copy_of_integer(x);   // x would move into the function
                                                // but its i32 is copy so its ok and still usable
    }
    {
        let _s1 = gives_ownership(); // give_ownership moves its return value to s1
        let s2 = String::from("Hello");
        let _s3 = takes_and_gives_back(s2);
    } // here s3 goes out of scope and is dropped, s2 out of scope but was moved, so nothing happens.
      // s1 goes out of scope and is dropped.

    // return multiple values as a tuple
    {
        let s1 = String::from("hello");
        let (s2, len) = calculate_length_tuple(s1);
        println!("The length of '{}' is {}.", s2, len);
    }

    {
        let s1 = String::from("hello World");
        let len = calculate_length(&s1); // & means reference, now actual value. We are referencing s1 here, rather than owning it. This is called borrowing.
                                                    // you can not modify something we're borrowing.
        println!("The length of '{}' is {}. ", s1, len);
    }

    // making references mutable!
    // you can only have one mutable reference to a particular piece of data in a particular scope at a time.
    // create new brackets to make new scopes, allowing for multiple mutable references. just not simultaneous ones
    {
        let mut s = String::from("hello there"); // this is a mutable variable on the heap
        change(&mut s);
        let r1 = &s; // no problem here
        let r2 = &s; // no problem here either
        let r3 = &mut s; // error!
        println!("s is now: {}.", s); // the value of s is now mutated and will show the new mutated value
//        println!("{}, {}, and {}", r1, r2, r3); // this will throw an error!
    }

    {
        let mut s = String::from("testing string");
        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{}, and {}", r1, r2);

        // we reference the mutable s after we use the original r1 and r2. the scope of r1 and r2 end after println!
        // these scopes won't overlap so go ahead and create r3 and reference and mutate s.
        let r3 = &mut s; // no problem
        println!("{}", r3);
    }

    // dangling reference error
    {
        let reference_to_nothing = dangle();
    }
    // find the first word to a string
    {
        let mut s = String::from("Hello World ");
        let word = first_word(&s); // word will get the value 5
        s.clear() // empties the string, making it equal to ""
    }
    // string slices
    {
        let s = String::from("Hello World");
        let hello = &s[0..5];
        let world = &s[6..11];
        let word = first_word(&s);
        println!("sliced string: {}", word);
    }
    // second word check
    {
        let s = String::from("Hello there Charlie!");
        let second_word = second_word(&s);
        println!("Second Word is: {}", second_word);
    }
    // slicing an array
    {
        let a = [1,2,3,4,5];
        let slice = &a[1..3];
    }
}

fn takes_ownership_of_string(some_string: String) {  // some_string comes into scope
    println!("{}", some_string);
}   // here some_string goes out of scope and drop is called. The backing memory is freed

fn makes_copy_of_integer(some_integer: i32) {
    let copy = some_integer;
    println!("some copy {} ", copy);
} // here , some_integer goes out of scope, but nothing special happens.

fn gives_ownership() -> String { // arrow like this indicates the type to be returned from function
    let some_string = String::from("hello");
    some_string    // no semicolon here, because we are returning this value.  not declaring a statement
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length_tuple(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, nothing happens.

fn change(some_string: &mut String) { // function takes in a String that is mutable
    some_string.push_str(", oh, I don't think so"); // mutate the string given to the function
}

fn dangle() -> String { // dangle returns reference to a string
    let s = String::from("hi there"); // s is the new string

    // &s // we return a reference to the String, s
    s // the solution here is to return string s directly
} // Here, s goes out of scope and it's dropped. Its memory goes away. DANGER!

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; // slice the string from the start of the string, to the index
        }
    }
    &s[..] // return the string found after the slice
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; // slice the string from the start of the string, to the index
        }
    }
    &s[..]
}