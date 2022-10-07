use std::ops::AddAssign;

struct Position {
    x: i32,
    depth: i32,
    aim: i32,
}

struct PositionDelta<'a> {
    cmd: &'a str,
    value: i32,
}

impl<'a> AddAssign<PositionDelta<'a>> for Position {

    fn add_assign(&mut self, rhs: PositionDelta) {
        return match rhs.cmd {
            "forward" => {
                self.x += rhs.value;
                self.depth += self.aim * rhs.value;
            },
            "down" => {
                self.aim += rhs.value;
            },
            "up" => {
                self.aim -= rhs.value;
            },
            &_ => {
                panic!("Unknown direction type");
            }
        }
    }
}

impl<'a> Into<PositionDelta<'a>> for &'a str {
    fn into(self) -> PositionDelta<'a> {
        let splitted = self.split_once(' ').unwrap();
        return PositionDelta { cmd: splitted.0, value: splitted.1.parse::<i32>().unwrap() }
    }
}


fn main() {
    let mut initial_pos = Position { x: 0, depth: 0, aim: 0 };

    include_str!("../input.txt")
        .lines()
        .for_each(|line| {
            let pos_delta: PositionDelta = line.into();
            initial_pos += pos_delta
        });

    println!("{}", initial_pos.x * initial_pos.depth);
}
