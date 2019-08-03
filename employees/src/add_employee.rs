use std::collections::HashMap;
use crate::selection::*;

pub fn add_employee(mut data: &mut HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    loop {
        println!("Please enter the name of the employee to manage.");

        let employee = selection_text();
        let employee = employee.trim();

        println!("Please enter the name of the department you would like to add the employee to.");

        let department = selection_text();
        let department = department.trim();

        data.entry(department.to_string())
            .or_insert(vec![])
            .push(employee.to_string());

        match check_continue() {
            1 => continue,
            2 => break,
            _ => { println!("Too many errors. Assuming you wish to exit current function. ");
                break
            }
        }
    }
    data.to_owned()
}

pub fn select_continue() -> Result<u32, ::std::num::ParseIntError> {
    println!("Enter '1' to add more employees, '2' to finish adding employees");
    selection()
}

pub fn check_continue() -> u32 {
    let selection = select_continue();
    match selection {
        Ok(i) => i,
        Err(_) => try_again()
    }
}

pub fn try_again() -> u32 {
    println!("Please enter '1' or '2'.");
    check_continue()
}