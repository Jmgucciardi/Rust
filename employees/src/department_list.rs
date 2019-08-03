use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn department_list(data: &HashMap<String, Vec<String>>, dept: &str) {
    match data.clone().entry(dept.to_string()) {
        Entry::Occupied(mut entry) => {
            let mut values: Vec<_> = entry.get_mut().iter().collect();
            values.sort();
            println!("\nEmployees in department {}: ", dept);
            for x in values {
                println!("{}\n", x)
            }
        },
        Entry::Vacant(entry) => {
            println!("The department {} does not exist ot does not have any employees yet. You may add \
            departments through the 'add an employee to a department' activity. The current departments are: ", dept);

            let mut keys: Vec<_> = data.keys().collect();
            keys.sort();
            for x in keys {
                println!("{}", x)
            }
        }
    }
}