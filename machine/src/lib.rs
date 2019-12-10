#[derive(Debug)]
pub struct IntCode<'a> {
    space: &'a mut [i32],
    input: Vec<i32>,
    on: bool,
    ip: i32,
}

#[derive(Debug, PartialEq)]
enum Param {
    Pos,
    Inter,
}

impl Param {
    fn decode(code: i32, pos: u32) -> Param {
        match code % (10i32.pow(pos + 1)) / (1 * 10i32.pow(pos)) {
            0 => Param::Pos,
            1 => Param::Inter,
            otherwise => panic!("Undefined parameter mode: {}", otherwise),
        }
    }

    fn get(&self, vm: &IntCode<'_>, arg_pos: i32) -> i32 {
        match self {
            Param::Pos => vm[vm[vm.ip + 1 + arg_pos]],
            Param::Inter => vm[vm.ip + 1 + arg_pos],
        }
    }

    fn set(&self, vm: &mut IntCode<'_>, arg_pos: i32, value: i32) {
        match self {
            Param::Pos => {
                let addr = vm[vm.ip + 1 + arg_pos];
                vm[addr] = value
            }
            Param::Inter => panic!("can not set in intermediate mode"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum OpCode {
    Add(Param, Param, Param),
    Mult(Param, Param, Param),
    Input(Param),
    Output(Param),
    Quit,
}

impl OpCode {
    fn decode(code: i32) -> OpCode {
        let op = code % 100;

        match op {
            1 => OpCode::Add(
                Param::decode(code, 2),
                Param::decode(code, 3),
                Param::decode(code, 4),
            ),
            2 => OpCode::Mult(
                Param::decode(code, 2),
                Param::decode(code, 3),
                Param::decode(code, 4),
            ),
            3 => OpCode::Input(Param::decode(code, 2)),
            4 => OpCode::Output(Param::decode(code, 2)),
            99 => OpCode::Quit,
            unrecognized => panic!("unrecognized opcode: {}", unrecognized),
        }
    }

    fn effect(&self, vm: &mut IntCode<'_>) -> Option<i32> {
        use OpCode::*;
        match self {
            Add(a, b, o) => {
                o.set(vm, 2, a.get(vm, 0) + b.get(vm, 1));
                None
            }
            Mult(a, b, o) => {
                o.set(vm, 2, a.get(vm, 0) * b.get(vm, 1));
                None
            }
            Input(o) => {
                let v = vm.input.pop().unwrap();
                o.set(vm, 0, v);
                None
            }
            Output(i) => {
                let v = i.get(vm, 0);
                Some(v)
            }
            Quit => {
                vm.on = false;
                None
            }
        }
    }

    fn stride(&self) -> usize {
        use OpCode::*;

        match self {
            Add(_, _, _) | Mult(_, _, _) => 4,
            Input(_) | Output(_) => 2,
            Quit => 1,
        }
    }
}

impl<'a> IntCode<'a> {
    pub fn new(space: &'a mut [i32], input: Vec<i32>) -> IntCode<'a> {
        let on = true;
        let ip = 0;

        IntCode {
            space,
            on,
            ip,
            input,
        }
    }

    pub fn run(&mut self) -> Vec<i32> {
        let mut output = Vec::new();
        while self.on {
            let opcode = OpCode::decode(self[self.ip]);

            if let Some(out) = opcode.effect(self) {
                output.push(out);
            }

            self.ip += opcode.stride() as i32;
        }

        output
    }
}

impl<'a> std::ops::Index<i32> for IntCode<'a> {
    type Output = i32;

    fn index(&self, pos: i32) -> &i32 {
        if pos < 0 {
            panic!("addresses may not be negative")
        }

        &self.space[pos as usize]
    }
}

impl<'a> std::ops::IndexMut<i32> for IntCode<'a> {
    fn index_mut(&mut self, pos: i32) -> &mut i32 {
        if pos < 0 {
            panic!("addresses may not be negative")
        }

        &mut self.space[pos as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        let mut buf = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut machine = IntCode::new(&mut buf, vec![]);

        machine.run();

        assert_eq!(machine.space[0], 3500);
    }

    #[test]
    fn basic_io() {
        let mut buf = [3, 0, 4, 0, 99];
        let mut machine = IntCode::new(&mut buf, vec![42]);

        assert_eq!(machine.run(), vec![42]);
    }

    #[test]
    fn param_mode() {
        assert_eq!(Param::decode(10, 2), Param::Pos);
        assert_eq!(Param::decode(100, 2), Param::Inter);
    }
}
