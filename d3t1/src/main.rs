fn main() {
    let lines_iter = include_str!("../input.txt")
        .lines();

    let first_number = lines_iter.clone().next().unwrap();

    // we assume here that lines are of fixed length.
    let number_length = first_number.len();

    // println!("Number length: {}", number_length);

    let mut buffer: Vec<i32> = Vec::with_capacity(number_length);
    for i in 0..number_length { buffer.push(0); }

    // println!("Buffer length: {}", buffer.len());
    let mut line_count = 0;
   
    for line in lines_iter {
        for (i, c) in line.chars().enumerate() {
            // println!("{}", c);
            if c == '1' {
                buffer[i] += 1;
            }
        }
        line_count += 1;
    } 

    let halved_line_count = line_count / 2 + line_count % 2;

    println!("halved_line_count: {}", halved_line_count);
    println!("line_count: {}", line_count / 2 + line_count & 1);

    let gamma_rate_bin = buffer.into_iter().map(|one_count| {
        if one_count >= halved_line_count {
            return '1';
        } else {
            return '0';
        }
    }).collect::<String>();

    let eps_rate_bin = gamma_rate_bin.chars().map(|c| {
        if c == '1' {
            return '0';
        } else {
            return '1';
        }
    }).collect::<String>();

    println!("gamma_rate_bin: {}", gamma_rate_bin);
    println!("eps_rate_bin: {}", eps_rate_bin);

    let gamma_rate = i32::from_str_radix(&gamma_rate_bin, 2)
        .expect("Not a binary number");

    let eps_rate = i32::from_str_radix(&eps_rate_bin, 2)
        .expect("Not a binary number");

    println!("gamma_rate: {}", gamma_rate);
    println!("eps_rate: {}", eps_rate);

    println!("{}", gamma_rate * eps_rate);
}
