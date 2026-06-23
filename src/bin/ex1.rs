//Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::io;
use std::collections::HashMap;


fn median(int_array:&Vec<i32>) -> i32{

    let vec_length: usize = int_array.len();
    println!("Here's the list: {:?} of length {}", int_array, vec_length);        
    let index: usize = (vec_length - 1)/2;

    if vec_length % 2 == 1 {
        return int_array[index]
    } else {
        let index1: usize = (vec_length)/2;
        return (int_array[index] + int_array[index1])/2
    }

}

fn mode(int_array:&Vec<i32>) -> Vec<i32>{

    let mut counts: HashMap<i32, i32> = HashMap::new();
    println!("Here's the list: {:?}", int_array);

    for i in int_array {
        let count = counts.entry(*i).or_insert(0);
        *count += 1;
    }

    let max_counts = match counts.values().max() {
        Some(&count) => count, 
        None => return Vec::new(), 
    };

    counts
        .into_iter()
        .filter(|&(_, count)| count == max_counts)
        .map(|(val, _)| val)
        .collect()
         

}


fn main() {

    //Let's read a list of integers from std in
    let mut input = String::new();

    println!("Enter a list of numbers");
    io::stdin().read_line(&mut input).expect("Failed to real input");

    //Get vector from input, trim, remove whitespace, get only the numbers
    let mut int_array: Vec<i32> = input.trim().split_whitespace().filter_map(|s| s.parse().ok()).collect();
    
    //sort array
    int_array.sort();

    let median: i32 = median(&int_array);

    let mode: Vec<i32> = mode(&int_array);

    println!("The median of the array is {} and the mode is {:?}", median, mode);

}