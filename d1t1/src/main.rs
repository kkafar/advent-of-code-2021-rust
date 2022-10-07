use std::{path::Path, io, fs};

fn main() {    
    let input_path = Path::new("input.txt");
    let input_data: Vec<i32> = fs::read_to_string(input_path)
        .unwrap()
        .split('\n')
        .filter_map(|num| {
            num.parse::<i32>().ok()
        })
        .collect();

    let mut counter = 0;
    for i in 1..input_data.len() {
        if input_data[i] - input_data[i - 1] > 0 {
            counter += 1;
        }
    }

    println!("Result is: {}", counter);
}
