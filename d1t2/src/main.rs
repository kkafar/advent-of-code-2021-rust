fn main() {
    let input_data: Vec<i32> = include_str!("../input.txt")
        .lines()
        .map(|num| num.parse::<i32>().unwrap())
        .collect();

    // S_i = a_i + a_{i + 1} + a_{i + 2}
    // S_{i + 1} = a_{i + 1} + a_{i + 2} + a_{i + 3}
    // S_{i + 1} - S_i = a_{i + 3} - a_i
    // for all i in 0..(n - 3)

    let mut count = 0;
    for i in 3..input_data.len() {
        if input_data[i] - input_data[i - 3] > 0 {
            count += 1
        }
    }
    println!("{}", count);
}
