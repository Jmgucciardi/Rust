use std::collections::HashMap;
use crate::department_list::*;

pub fn company_list(data: &HashMap<String, Vec<String>>) {
    let departments = data.keys();
    println!("Company employees listed by department");
    for dept in departments {
        department_list(data, dept)
    }
}