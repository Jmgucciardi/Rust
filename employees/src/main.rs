extern crate employees;

use std::collections::HashMap;
use employees::selection::*;
use employees::add_employee::*;
use employees::select_activity::*;
use employees::department_list::*;
use employees::company_list::*;


fn main() {
    let mut data = HashMap::new();
    loop {
        let activity = select_activity();
        match activity {
            1 => { let data = add_employee(&mut data);
                continue
            },
            2 => { println!("List employees from which department?\n");
                let department = selection_text();
                department_list(&data, &department);
                continue
            }
            3 => { company_list(&data);
            continue
            }
            4 => break,
            _ => continue
        }
    }
}