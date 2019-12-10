use machine::IntCode;
use utils::{input, split};

fn main() {
    let mut buf = split(input(), ",");
    let mut machine = IntCode::new(&mut buf, vec![1]);

    println!("Output: {:?}", machine.run());
}
