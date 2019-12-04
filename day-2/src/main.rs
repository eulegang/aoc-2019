use utils::{input, split};

fn main() {
    let mut buf = split(input(), ",");

    let mut machine = machine::Machine::new(&mut buf);

    machine.run();

    println!("{:?}", machine);
}
