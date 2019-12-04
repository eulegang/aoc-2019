use utils::{input, split};

fn main() {
    let mut buf = split(input(), ",");

    let mut machine = machine::IntCode::new(&mut buf);

    machine[1] = 12;
    machine[2] = 2;

    machine.run();

    println!("{:?}", machine[0]);
}
