use std::io;

pub fn selection() -> Result<u32, ::std::num::ParseIntError> {
    let mut selection = String::new();
    io::stdin.read_line(&mut selection)
        .expect("failed to read line");
    selection.trim().parse::<u32>()
}

pub fn selection_text() -> String {
    let mut selection = String::new();
    io::stdin(&mut selection)
        .expect("Failed to read line");
    selection.trim().to_string().to_uppercase()
}