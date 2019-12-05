use std::io;
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let mut first = String::new();
    let mut second = String::new();

    stdin.read_line(&mut first).unwrap();
    stdin.read_line(&mut second).unwrap();

    let min = hop(
        extract(first.trim_end_matches("\n")),
        extract(second.trim_end_matches("\n")),
    );

    println!("Distance: {}", min);
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, PartialEq, Copy, Clone)]
struct Movement(i32, Direction);

#[derive(Debug, PartialEq, Copy, Clone)]
struct Position(i32, i32);

#[derive(Debug, PartialEq, Copy, Clone)]
enum Stretch {
    Vertical((i32, i32), i32),
    Horizontal(i32, (i32, i32)),
}

impl FromStr for Movement {
    type Err = String;
    fn from_str(buf: &str) -> Result<Movement, String> {
        let mut chars = buf.chars();

        let direction = match chars.nth(0) {
            Some('R') => Ok(Direction::Right),
            Some('U') => Ok(Direction::Up),
            Some('L') => Ok(Direction::Left),
            Some('D') => Ok(Direction::Down),
            Some(otherwise) => Err(format!("Invalid Direction: {:?}", otherwise)),
            None => Err(format!("Expected a nonempty string")),
        }?;

        let rest = chars.as_str();

        let mag = rest
            .parse::<i32>()
            .map_err(|_| format!("Unable to parse {}", rest))?;

        Ok(Movement(mag, direction))
    }
}

impl Direction {
    fn unit(&self) -> (i32, i32) {
        match self {
            Direction::Up => (1, 0),
            Direction::Down => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Left => (0, -1),
        }
    }
}

impl Position {
    fn origin() -> Position {
        Position(0, 0)
    }

    fn next(&self, mv: &Movement) -> (Position, Stretch) {
        let unit = mv.1.unit();
        let Position(y, x) = *self;
        let (dy, dx) = (mv.0 * unit.0, mv.0 * unit.1);

        let pos = Position(y + dy, x + dx);

        let stretch = match mv.1 {
            Direction::Right | Direction::Left => {
                Stretch::Horizontal(y, (i32::min(x, x + dx), i32::max(x, x + dx)))
            }

            Direction::Up | Direction::Down => {
                Stretch::Vertical((i32::min(y, y + dy), i32::max(y, y + dy)), x)
            }
        };

        (pos, stretch)
    }
}

impl Stretch {
    fn intersection(&self, other: &Stretch) -> Option<Position> {
        match (self, other) {
            (Stretch::Horizontal(y, (xa, xe)), Stretch::Vertical((ya, ye), x))
                if ya <= y && y <= ye && xa <= x && x <= xe =>
            {
                Some(Position(*y, *x))
            }

            (Stretch::Vertical((ya, ye), x), Stretch::Horizontal(y, (xa, xe)))
                if ya <= y && y <= ye && xa <= x && x <= xe =>
            {
                Some(Position(*y, *x))
            }
            _ => None,
        }
    }

    fn len(&self) -> u32 {
        match self {
            Stretch::Vertical((a, b), _) => (*b - *a) as u32,
            Stretch::Horizontal(_, (a, b)) => (*b - *a) as u32,
        }
    }
}

fn extract(string: &str) -> Vec<Movement> {
    string.split(",").map(|s| s.parse().unwrap()).collect()
}

#[allow(dead_code)]
fn dist(first: Vec<Movement>, second: Vec<Movement>) -> i32 {
    let fs = stretches(first);
    let ss = stretches(second);

    let mut min_dist = i32::max_value();

    for f in &fs {
        for s in &ss {
            if let Some(Position(y, x)) = f.intersection(&s) {
                if y == 0 && x == 0 {
                    continue;
                }

                let dist = i32::abs(y) + i32::abs(x);
                if dist < min_dist {
                    min_dist = dist;
                }
            }
        }
    }

    min_dist
}

#[allow(dead_code)]
fn hop(first: Vec<Movement>, second: Vec<Movement>) -> i32 {
    let fs = dir_stretches(first);
    let ss = dir_stretches(second);

    let mut min_dist = i32::max_value();

    let mut f_len = 0;
    for (fdir, f) in &fs {
        f_len += f.len() as i32;
        let mut s_len = 0;

        for (sdir, s) in &ss {
            s_len += s.len() as i32;

            if let Some(Position(y, x)) = f.intersection(&s) {
                if y == 0 && x == 0 {
                    continue;
                }

                let f_dist = f_len
                    - match (fdir, f) {
                        (Direction::Up, Stretch::Vertical((_, b), _)) => b - y,
                        (Direction::Down, Stretch::Vertical((a, _), _)) => y - a,
                        (Direction::Right, Stretch::Horizontal(_, (_, b))) => b - x,
                        (Direction::Left, Stretch::Horizontal(_, (a, _))) => x - a,
                        (dir, stretch) => unreachable!("dir: {:?}, stretch: {:?}", dir, stretch),
                    };

                let s_dist = s_len
                    - match (sdir, s) {
                        (Direction::Up, Stretch::Vertical((_, b), _)) => b - y,
                        (Direction::Down, Stretch::Vertical((a, _), _)) => y - a,
                        (Direction::Right, Stretch::Horizontal(_, (_, b))) => b - x,
                        (Direction::Left, Stretch::Horizontal(_, (a, _))) => x - a,
                        (dir, stretch) => unreachable!("dir: {:?}, stretch: {:?}", dir, stretch),
                    };

                let dist = f_dist + s_dist;

                if dist < min_dist {
                    min_dist = dist
                }
            }
        }
    }

    min_dist
}

fn stretches(movements: Vec<Movement>) -> Vec<Stretch> {
    let mut pos = Position::origin();
    let mut result = Vec::new();

    for mv in movements {
        let (end, stretch) = pos.next(&mv);

        pos = end;
        result.push(stretch);
    }

    result
}

fn dir_stretches(movements: Vec<Movement>) -> Vec<(Direction, Stretch)> {
    let mut pos = Position::origin();
    let mut result = Vec::new();

    for mv in movements {
        let (end, stretch) = pos.next(&mv);

        pos = end;
        result.push((mv.1, stretch));
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple() {
        let first = extract("R8,U5,L5,D3");
        let second = extract("U7,R6,D4,L4");

        assert_eq!(dist(first, second), 6);
    }

    #[test]
    fn test_1() {
        let first = extract("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let second = extract("U62,R66,U55,R34,D71,R55,D58,R83");

        assert_eq!(dist(first, second), 159);
    }

    #[test]
    fn test_2() {
        let first = extract("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let second = extract("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");

        assert_eq!(dist(first, second), 135);
    }

    #[test]
    fn test_hop_simple() {
        let first = extract("R8,U5,L5,D3");
        let second = extract("U7,R6,D4,L4");

        assert_eq!(hop(first, second), 30);
    }

    #[test]
    fn test_hop_1() {
        let first = extract("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let second = extract("U62,R66,U55,R34,D71,R55,D58,R83");

        assert_eq!(hop(first, second), 610);
    }

    #[test]
    fn test_hop2() {
        let first = extract("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let second = extract("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");

        assert_eq!(hop(first, second), 410);
    }
}
