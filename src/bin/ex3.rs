//Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company; 
// for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then, let the user retrieve a list of all people in a 
// department or all people in the company by department, sorted alphabetically.

use std::io;
use std::collections::HashMap;

fn add_to_map(words: &Vec<&str>, directory: &mut HashMap<String, Vec<String>>) {

    if words.len() != 4 {
        return println!("Add a valid statement like 'Add Sally to Engineering'");
    }
    let name = words[1];
    let department = words[3];
    println!("{:?}, {:?}", name, department);
    directory
        .entry(department.to_string().to_lowercase())
        .or_default()
        .push(name.to_string());
}

fn list_department(words: &Vec<&str>, directory: &mut HashMap<String, Vec<String>>) {

    let department = words[1];
    let people: &mut Vec<String> = directory.get_mut(department).unwrap();
    people.sort();
    println!("{:?}", people);

}

fn get_all_people(directory: &mut HashMap<String, Vec<String>>) {

      for (key, value) in directory {
        value.sort();
        println!("{key}: {:?}", value);
    }


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
        let user_input_lower = user_input.to_lowercase();
        let words: Vec<&str> = user_input_lower.trim().split_whitespace().collect();
        let search_term: &str = words[0];


        match search_term {
            "add" => {
                add_to_map(&words, &mut directory);
                println!("{:?}", directory);
            }
            "list" => {
                list_department(&words, &mut directory);
            }
            "get" => {
                get_all_people(&mut directory);
            }
            "quit" => {
                break;
            }
            _ => {
                println!("Command not recognized");
            }
        }
    }
}