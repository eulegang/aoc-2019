use itertools::Itertools;
use machine::IntCode;
use utils::{input, split};

fn main() {
    let prog: Vec<i32> = split(input(), ",");

    let amp = part_two(&prog);
    println!("amplitude: {}", amp);
}

#[allow(dead_code)]
fn part_one(prog: &Vec<i32>) -> i32 {
    let mut max = 0;

    for combs in (0..5).permutations(5) {
        max = i32::max(max, run_amps(&prog, combs.clone()));
    }

    max
}

#[allow(dead_code)]
fn part_two(prog: &Vec<i32>) -> i32 {
    let mut max = 0;

    for combs in (5..10).permutations(5) {
        max = i32::max(max, run_amps_feedback(&prog, combs.clone()));
    }

    max
}

fn run_amps_feedback(code: &Vec<i32>, mut phases: Vec<i32>) -> i32 {
    phases.reverse();
    let mut amp_level = 0;

    'root: loop {
        for phase in &phases {
            dbg!(&phase, &amp_level);

            let mut machine = IntCode::new(code.clone(), vec![*phase, amp_level]);
            let out = machine.run();

            dbg!(&out);
            if let Some(amp) = out.get(0) {
                amp_level = *amp;
            } else {
                break 'root;
            }
        }
    }

    dbg!(&amp_level);

    amp_level
}

fn run_amps(code: &Vec<i32>, mut phases: Vec<i32>) -> i32 {
    phases.reverse();
    let mut amp_level = 0;
    while let Some(phase) = phases.pop() {
        dbg!(&phase, &amp_level);

        let mut machine = IntCode::new(code.clone(), vec![phase, amp_level]);
        let out = machine.run();

        dbg!(&out);
        amp_level = out[0];
    }

    amp_level
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x() {
        let prog = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let phases = vec![4, 3, 2, 1, 0];

        assert_eq!(run_amps(&prog, phases), 43210);
    }
}
