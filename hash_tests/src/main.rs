use std::collections::HashMap;
use std::io;
use std::iter::FromIterator;

// find the mean, median and mode of a vector using a hash map and other rust functions

fn average(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

fn median(numbers: &mut[i32]) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn mode(numbers: &[i32]) -> i32 {
    let mut number_map = HashMap::new();

    for &value in numbers {
        *number_map.entry(value).or_insert(0) += 1;
    }

    number_map.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Can not compute the mode of zero numbers!")
}

fn main() {
    let mut numbers = vec![42, 1, 36, 76, 378, 43, 1, 43, 54, 2, 3, 43];

    println!("Average: {}", average(&numbers));
    println!("Median: {}", median(&mut numbers));
    println!("Median: {}", mode(&numbers));

    // Pig latin portion of the code
    println!("Enter the string to be made into pig latin!: ");
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read from stdin");

    let mut chars = input.chars().peekable(); // gives you access to peek() and next() methods to view the chars in provided string
    let mut new_s = String::new(); // open new memory fore pig latin string to be returned
    while let Some(c) = chars.next() { // loop over the chars while Some(c) is valid , looking in with tne next() method from peekable()
        let suffix = match c { // check and see if the Some(c) is a vowel or a consonant, upper or lower case
            'a' | 'e' | 'i' | 'o' | 'u' => { // if vowel push that onto the new_s string and return the new string "-hay" into suffix
                new_s.push(c);
                String::from("-hay")
            }
            'a'...'z' | 'A'...'Z' => { // if not a vowel, format c to have -ay on the end of it
                format!("-{}ay", c) // return this value onto suffix
            }
            _ => {
                new_s.push(c); // no more Some(c) so None will be found. This executes on none and pushes what is now c onto new_s
                continue;
            }
        };

        while let Some(&c) = chars.peek() { // borrow the value of c with &c and loop while we receive values from peeking at chars
            match c {
                'a'...'z' | 'A'...'Z' => { //
                    chars.next();
                    new_s.push(c); // push what we find onto new_s to finish the pig latin routine
                }
                _ => break, // end this loop once None() is reached
            }
        }
        new_s += &suffix; // += here will take new_s and concat the suffix on the end
    }
    println!("Pig Latin: {}", new_s);
}
