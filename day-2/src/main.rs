use machine::IntCode;
use utils::{input, split};

fn main() {
    let buf = split(input(), ",");

    part2(&buf);
}

#[allow(dead_code)]
fn part1(bin: &Vec<i32>) {
    let mut buf = bin.clone();
    let mut machine = IntCode::new(&mut buf);

    machine[1] = 12;
    machine[2] = 2;

    machine.run();

    println!("{:?}", machine[0]);
}

#[allow(dead_code)]
fn part2(bin: &Vec<i32>) {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut buf = bin.clone();
            let mut machine = IntCode::new(&mut buf);

            machine[1] = noun;
            machine[2] = verb;

            machine.run();

            if machine[0] == 19690720 {
                println!("Answer found: {}", noun * 100 + verb);
            }
        }
    }
}
