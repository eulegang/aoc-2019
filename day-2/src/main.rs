use utils::{input, split};

fn main() {
    let mut buf = split(input(), ",");

    let mut machine = Machine::new(&mut buf);

    machine.run();

    println!("{:?}", machine.space);
}

struct Machine<'a> {
    space: &'a mut [u32],
}

const PC_STRIDE: usize = 4;

impl<'a> Machine<'a> {
    fn new(buf: &'a mut [u32]) -> Machine<'a> {
        Machine { space: buf }
    }

    fn run(&mut self) {
        let mut pc = 0;

        while let Some(next) = self.step(pc) {
            pc = next;
        }
    }

    fn deref(&self, addr: usize) -> usize {
        self[addr] as usize
    }

    fn step(&mut self, pc: usize) -> Option<usize> {
        match self.space[pc] {
            1 => {
                let addr = self.deref(pc + 3);
                self[addr] = self[self.deref(pc + 1)] + self[self.deref(pc + 2)];
                Some(pc + PC_STRIDE)
            }
            2 => {
                let output_addr = self[pc + 3] as usize;
                self[output_addr] = self[self.deref(pc + 1)] * self[self.deref(pc + 2)];
                Some(pc + PC_STRIDE)
            }
            99 => None,
            otherwise => panic!("undefined opcode {}", otherwise),
        }
    }
}

impl<'a> std::ops::Index<usize> for Machine<'a> {
    type Output = u32;

    fn index(&self, pos: usize) -> &u32 {
        &self.space[pos]
    }
}

impl<'a> std::ops::IndexMut<usize> for Machine<'a> {
    fn index_mut(&mut self, pos: usize) -> &mut u32 {
        &mut self.space[pos]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        let mut buf = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut machine = Machine::new(&mut buf);

        machine.run();

        assert_eq!(machine.space[0], 3500);
    }
}
