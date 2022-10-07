const INPUT_LINE_COUNT: i32 = 1000;
const INPUT_LINE_LENGTH: usize = 12;

fn main() {
    // using vec![0, INPUT_LINE_LENGTH] caused vec to have size 2 despite havinc
    // capacity INPUT_LINE_LENGTH
    let mut buffer: Vec<i32> = Vec::with_capacity(INPUT_LINE_LENGTH);
    for _ in 0..INPUT_LINE_LENGTH { buffer.push(0); }

    include_str!("../input.txt")
        .lines()
        .for_each(|line| {
            for (i, c) in line.chars().enumerate() {
                if c == '1' {
                    buffer[i] += 1;
                }
            }
        });

    // We notice here, that INPUT_LINE_COUNT is even => we do not need to add one.
    let halved_line_count = INPUT_LINE_COUNT / 2;
    let mut gamma_rate_bin: String = String::with_capacity(INPUT_LINE_LENGTH);
    let mut eps_rate_bin: String = String::with_capacity(INPUT_LINE_LENGTH);

    buffer
        .into_iter()
        .for_each(|one_count| {
            if one_count >= halved_line_count {
                gamma_rate_bin.push('1');
                eps_rate_bin.push('0');
            } else {
                gamma_rate_bin.push('0');
                eps_rate_bin.push('1');
            }
        });

    let gamma_rate = i32::from_str_radix(&gamma_rate_bin, 2)
        .expect("Not a binary number");

    let eps_rate = i32::from_str_radix(&eps_rate_bin, 2)
        .expect("Not a binary number");

    println!("{}", gamma_rate * eps_rate);
}
