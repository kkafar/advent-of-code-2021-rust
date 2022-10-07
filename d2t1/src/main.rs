use std::ops::AddAssign;

struct Position {
    x: i32,
    depth: i32,
}

struct PositionDelta<'a> {
    direction: &'a str,
    magnitude: i32,
}

impl<'a> AddAssign<PositionDelta<'a>> for Position {

    fn add_assign(&mut self, rhs: PositionDelta) {
        return match rhs.direction {
            "forward" => {
                self.x += rhs.magnitude;
            },
            "down" => {
                self.depth += rhs.magnitude;
            },
            "up" => {
                self.depth -= rhs.magnitude;
            },
            &_ => {
                panic!("Unknown direction type");
            }
        }
    }
}

impl<'a> Into<PositionDelta<'a>> for &'a str {
    fn into(self) -> PositionDelta<'a> {
        let splitted = self.split(' ')
            .collect::<Vec<&str>>();
        
        return PositionDelta { direction: splitted[0], magnitude: splitted[1].parse::<i32>().unwrap() }
    }
}


fn main() {
    let mut initial_pos = Position { x: 0, depth: 0 };

    include_str!("../input.txt")
        .lines()
        .for_each(|line| {
            let pos_delta: PositionDelta = line.into();
            initial_pos += pos_delta
        });

    println!("{}", initial_pos.x * initial_pos.depth);
}
