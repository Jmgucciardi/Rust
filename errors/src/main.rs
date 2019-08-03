use std::io;
use std::io::Read;
use std::fs::File;
use std::fs;
use std::io::ErrorKind;

fn main() {
    let _f = File::open("hello.txt");

    let _f = match _f {
        Ok(file) => file,
        Err(error) => match error.kind() {
          ErrorKind::NotFound => match File::create("hello.txt") {
              Ok(fc) => fc,
              Err(e) => panic!("Problem creating the file {:?}", e),
          },
            other_error => panic!("Problem opening the file {:?}", other_error),
        },
    };

    // cleaner version of the above code of reading a file in, or creating it if it isn't found
    let _f1 = File::open("goodbye.txt").unwrap_or_else(|error| {
       if error.kind() == ErrorKind::NotFound {
           File::create("goodbye.txt").unwrap_or_else(|error| {
               panic!("Problem creating the file: {:?}", error);
           })
       } else {
           panic!("Problem opening the file: {:?}", error)
       }
    });

    // a simple example of the unwrap function and expect function
//    {
////        let _f = File::open("hola.txt").unwrap();
//        let _g = File::open("hola.txt").expect("Failed to open hola.txt"); // better practice for error handling
//    }

    // propagating the error, using match
    fn read_username_from_file() -> Result<String, io::Error> { // returning type Result, either the username, or the error
        let f = File::open("user_info.txt");
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
    // propagating the error, more of a rust solution
    {
        // the ? after a statement acts like a ternary in a way. Ok, return ok, Err, return err
        fn read_username_from_file() -> Result<String, io::Error> {
            let mut f = File::open("hello.txt")?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(s)
        }
    }
    // even easier for of the above functions
    {
        fn read_username_from_file() -> Result<String, io::Error> {
            let mut s = String::new();
            File::open("hello.txt")?.read_to_string(&mut s)?;
            Ok(s)
        }
    }
    // easiest way to do it
    // use rust fs from standard library
    {
        fn read_username_from_file() -> Result<String, io::Error> {
            fs::read_to_string("hello.txt")
        }
    }
}
