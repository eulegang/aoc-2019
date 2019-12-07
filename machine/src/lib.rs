#[derive(Debug)]
pub struct IntCode<'a> {
    space: &'a mut [i32],
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
            Param::Pos => vm[vm.ip + 1 + arg_pos],
            Param::Inter => vm.ip + 1 + arg_pos,
        }
    }

    fn set(&self, vm: &mut IntCode<'_>, arg_pos: i32, value: i32) {
        match self {
            Param::Pos => {
                dbg!(value);
                let addr = vm.ip + 1 + arg_pos;
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
            99 => OpCode::Quit,
            unrecognized => panic!("unrecognized opcode: {}", unrecognized),
        }
    }

    fn effect(&self, vm: &mut IntCode<'_>) {
        use OpCode::*;
        match self {
            Add(a, b, o) => {
                o.set(vm, 2, a.get(vm, 0) + b.get(vm, 1));
            }
            Mult(a, b, o) => {
                o.set(vm, 2, a.get(vm, 0) * b.get(vm, 1));
            }
            Quit => vm.on = false,
        }
    }

    fn stride(&self) -> usize {
        use OpCode::*;

        match self {
            Add(_, _, _) | Mult(_, _, _) => 4,
            Quit => 1,
        }
    }
}

impl<'a> IntCode<'a> {
    pub fn new(space: &'a mut [i32]) -> IntCode<'a> {
        let on = true;
        let ip = 0;

        IntCode { space, on, ip }
    }

    pub fn run(&mut self) {
        while self.on {
            let opcode = OpCode::decode(self[self.ip]);
            dbg!(&opcode);

            opcode.effect(self);

            self.ip += opcode.stride() as i32;
        }
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
        let mut machine = IntCode::new(&mut buf);

        machine.run();

        assert_eq!(machine.space[0], 3500);
    }

    #[test]
    fn param_mode() {
        assert_eq!(Param::decode(10, 2), Param::Pos);
        assert_eq!(Param::decode(100, 2), Param::Inter);
    }
}
