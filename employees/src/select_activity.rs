use crate::selection::*;

pub fn select_activity() -> u32 {
    println!("Enter '1' to add an employee to a department.\nEnter '2'\
     to list employees within a department.\nEnter '3' to list all company employees by department.\nEnter '4' \
     to exit.");
    check_activity()
}

pub fn check_activity() -> u32 {
    let selection = selection();
    match selection {
        Ok(i) => i,
        Err(_) => select_again()
    }
}

pub fn select_again() -> u32 {
    println!("Please enter a number 1 through 4.");
    select_activity()
}