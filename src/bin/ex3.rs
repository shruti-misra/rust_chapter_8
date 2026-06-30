//Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company; 
// for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then, let the user retrieve a list of all people in a 
// department or all people in the company by department, sorted alphabetically.

use std::io;
use std::collections::HashMap;

fn add_to_map(name: &str, department: &str, directory: &mut HashMap<String, Vec<String>>) {
    directory
        .entry(department.to_string().to_uppercase())
        .or_default()
        .push(name.to_string());
}

fn main() {

    println!("--- Company Directory CLI ---");
    println!("Commands:");
    println!("  - Add <Name> to <Department>");
    println!("  - List <Department>");
    println!("  - Get all people");
    println!("  - Quit\n");

    let mut directory: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut user_input = String::new();

        //Read user input
        io::stdin().read_line(&mut user_input).expect("Failed to real input");

        let words: Vec<&str> = user_input.trim().split_whitespace().collect();
        if words.len() != 4 {
            return println!("Add a valid statement like 'Add Sally to Engineering'");
        }
        
        else {

            let name = words[1];
            let department = words[3];

            println!("{:?}, {:?}", name, department);
            add_to_map(&name, &department, &mut directory);
            println!("{:?}", directory)

            //Add name and department to hashmap
        }
    }
}