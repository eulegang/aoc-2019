use machine::IntCode;
use utils::{input, split};

fn main() {
    let buf = split(input(), ",");
    let mut machine = IntCode::new(buf, vec![5]);

    println!("Output: {:?}", machine.run());
}
