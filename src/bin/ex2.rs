//Convert strings to Pig Latin. 
// The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. 
// Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). 
// Keep in mind the details about UTF-8 encoding!

use std::io;

fn pig_latin(word: &str) -> String {

    //Convert the word into a vector of characters
    let first_letter = match word.chars().next() {
        Some(c) => c.to_ascii_lowercase(),
        None => return String::new(),
    };
    let vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    if vowels.contains(&first_letter) {

        format!("{word}-hay")

    } else {
        
        let first_string: String = word.chars().skip(1).collect();
        format!("{first_string}-{first_letter}ay")
    }
}

fn main() {

    //Variable for new user input
    let mut user_input = String::new();
    println!("Enter a new word to convert to Pig Latin. Only enter 1 word");

    io::stdin().read_line(&mut user_input).expect("Failed to read user input");
    let word = user_input.trim().to_string();

    let pig_latin: String = pig_latin(&word);

    println!("The Pig-Latin word is {}", pig_latin)

}