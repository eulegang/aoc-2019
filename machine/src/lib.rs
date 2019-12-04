#[derive(Debug)]
pub struct IntCode<'a> {
    space: &'a mut [u32],
    on: bool,
    ip: usize,
}

enum OpCode {
    Add,
    Mult,
    Quit,
}

impl OpCode {
    fn decode(code: u32) -> OpCode {
        match code {
            1 => OpCode::Add,
            2 => OpCode::Mult,
            99 => OpCode::Quit,
            unrecognized => panic!("unrecognized opcode: {}", unrecognized),
        }
    }

    fn effect(&self, vm: &mut IntCode<'_>) {
        use OpCode::*;
        match self {
            Add => {
                let addr = vm.deref_arg(2);
                vm[addr] = vm[vm.deref_arg(0)] + vm[vm.deref_arg(1)];
            }
            Mult => {
                let addr = vm.deref_arg(2);
                vm[addr] = vm[vm.deref_arg(0)] * vm[vm.deref_arg(1)];
            }
            Quit => vm.on = false,
        }
    }

    fn stride(&self) -> usize {
        use OpCode::*;

        match self {
            Add | Mult => 4,
            Quit => 1,
        }
    }
}

impl<'a> IntCode<'a> {
    pub fn new(space: &'a mut [u32]) -> IntCode<'a> {
        let on = true;
        let ip = 0;

        IntCode { space, on, ip }
    }

    pub fn run(&mut self) {
        while self.on {
            let opcode = OpCode::decode(self[self.ip]);

            opcode.effect(self);

            self.ip += opcode.stride();
        }
    }
    fn deref_arg(&self, offset: usize) -> usize {
        self[self.ip + 1 + offset] as usize
    }
}

impl<'a> std::ops::Index<usize> for IntCode<'a> {
    type Output = u32;

    fn index(&self, pos: usize) -> &u32 {
        &self.space[pos]
    }
}

impl<'a> std::ops::IndexMut<usize> for IntCode<'a> {
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
        let mut machine = IntCode::new(&mut buf);

        machine.run();

        assert_eq!(machine.space[0], 3500);
    }
}
