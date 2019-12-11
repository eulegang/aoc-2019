use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct IntCode {
    space: Vec<i32>,
    input: Vec<i32>,
    on: bool,
    ip: i32,
}

macro_rules! param_arg {
    ($machine: expr, $code: expr, $offset: literal) => {
        Param::digit(
            $machine[$machine.ip + 1 + $offset],
            dec_digit($code, 2 + $offset),
        )
    };
}

#[inline]
fn dec_digit(base: i32, digit: u32) -> i32 {
    let mut r = base;
    r = r % 10i32.pow(digit + 1);
    r = r / 10i32.pow(digit);

    r
}

#[derive(Debug, PartialEq)]
enum Param {
    Pos(i32),
    Inter(i32),
}

impl Display for Param {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        match self {
            Param::Pos(addr) => write!(fmt, "%0x{:04x}", addr),
            Param::Inter(val) => write!(fmt, "0x{:04x}", val),
        }?;

        Ok(())
    }
}

impl Param {
    fn digit(code: i32, encode: i32) -> Param {
        match encode {
            0 => Param::Pos(code),
            1 => Param::Inter(code),
            _ => panic!("Invalid Argument mode: {}", encode),
        }
    }

    fn get(&self, vm: &IntCode) -> i32 {
        match self {
            Param::Pos(addr) => vm[*addr],
            Param::Inter(val) => *val,
        }
    }

    fn set(&self, vm: &mut IntCode, value: i32) {
        match self {
            Param::Pos(addr) => vm[*addr] = value,
            Param::Inter(_) => panic!("can not set in intermediate mode"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum OpCode {
    Add(Param, Param, Param),
    Mult(Param, Param, Param),
    Input(Param),
    Output(Param),
    JumpTrue(Param, Param),
    JumpFalse(Param, Param),
    LessThan(Param, Param, Param),
    Equals(Param, Param, Param),

    Quit,
}

impl Display for OpCode {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        use OpCode::*;
        match self {
            Add(a, b, o) => write!(fmt, "ADD  {}, {}, {}", a, b, o)?,
            Mult(a, b, o) => write!(fmt, "MULT {}, {}, {}", a, b, o)?,
            Input(a) => write!(fmt, "IN   {}", a)?,
            Output(a) => write!(fmt, "OUT  {}", a)?,
            JumpTrue(a, b) => write!(fmt, "JT   {}, {}", a, b)?,
            JumpFalse(a, b) => write!(fmt, "JF   {}, {}", a, b)?,
            LessThan(a, b, o) => write!(fmt, "LT   {}, {}, {}", a, b, o)?,
            Equals(a, b, o) => write!(fmt, "EQ   {}, {}, {}", a, b, o)?,
            Quit => write!(fmt, "QT")?,
        };

        Ok(())
    }
}

impl OpCode {
    fn effect(&self, vm: &mut IntCode) -> Option<i32> {
        use OpCode::*;
        match self {
            Add(a, b, o) => {
                o.set(vm, a.get(vm) + b.get(vm));
                None
            }
            Mult(a, b, o) => {
                o.set(vm, a.get(vm) * b.get(vm));
                None
            }
            Input(o) => {
                let v = vm.input.pop().unwrap();
                o.set(vm, v);
                None
            }
            Output(i) => {
                let v = i.get(vm);
                Some(v)
            }
            JumpTrue(val, addr) => {
                if val.get(vm) != 0 {
                    vm.ip = addr.get(vm)
                }

                None
            }

            JumpFalse(val, addr) => {
                if val.get(vm) == 0 {
                    vm.ip = addr.get(vm)
                }

                None
            }

            LessThan(a, b, o) => {
                if a.get(vm) < b.get(vm) {
                    o.set(vm, 1)
                } else {
                    o.set(vm, 0)
                }

                None
            }

            Equals(a, b, o) => {
                if a.get(vm) == b.get(vm) {
                    o.set(vm, 1)
                } else {
                    o.set(vm, 0)
                }

                None
            }

            Quit => {
                vm.on = false;
                None
            }
        }
    }

    fn real(&self, vm: &IntCode) -> String {
        use OpCode::*;
        match self {
            Add(a, b, _) => format!("{} + {}", a.get(vm), b.get(vm)),
            Mult(a, b, _) => format!("{} * {}", a.get(vm), b.get(vm)),
            Input(_) => format!(""),
            Output(a) => format!("{}", a.get(vm)),
            JumpTrue(a, b) => format!("{}, {}", a.get(vm), b.get(vm)),
            JumpFalse(a, b) => format!("{}, {}", a.get(vm), b.get(vm)),
            LessThan(a, b, _) => format!("{} < {}", a.get(vm), b.get(vm)),
            Equals(a, b, _) => format!("{} = {}", a.get(vm), b.get(vm)),
            Quit => format!(""),
        }
    }

    fn stride(&self, vm: &IntCode) -> Option<usize> {
        use OpCode::*;

        match self {
            Add(_, _, _) | Mult(_, _, _) => Some(4),
            LessThan(_, _, _) | Equals(_, _, _) => Some(4),
            Input(_) | Output(_) => Some(2),
            JumpTrue(val, _) if val.get(vm) == 0 => Some(3),
            JumpFalse(val, _) if val.get(vm) != 0 => Some(3),
            Quit => Some(1),

            // Auto Jumps
            JumpTrue(_, _) => None,
            JumpFalse(_, _) => None,
        }
    }
}

impl<'a> IntCode {
    pub fn new(space: Vec<i32>, input: Vec<i32>) -> IntCode {
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
            let opcode = self.decode_op();

            if cfg!(debug_assertions) {
                eprintln!("0x{:04x}: {} ({})", self.ip, opcode, opcode.real(self));
            }

            if let Some(out) = opcode.effect(self) {
                output.push(out);
            }

            if let Some(stride) = opcode.stride(self) {
                self.ip += stride as i32;
            }
        }

        output
    }

    fn decode_op(&self) -> OpCode {
        let code = self[self.ip];
        let op = code % 100;

        match op {
            1 => OpCode::Add(
                param_arg!(self, code, 0),
                param_arg!(self, code, 1),
                param_arg!(self, code, 2),
            ),
            2 => OpCode::Mult(
                param_arg!(self, code, 0),
                param_arg!(self, code, 1),
                param_arg!(self, code, 2),
            ),
            3 => OpCode::Input(param_arg!(self, code, 0)),
            4 => OpCode::Output(param_arg!(self, code, 0)),
            5 => OpCode::JumpTrue(param_arg!(self, code, 0), param_arg!(self, code, 1)),
            6 => OpCode::JumpFalse(param_arg!(self, code, 0), param_arg!(self, code, 1)),
            7 => OpCode::LessThan(
                param_arg!(self, code, 0),
                param_arg!(self, code, 1),
                param_arg!(self, code, 2),
            ),
            8 => OpCode::Equals(
                param_arg!(self, code, 0),
                param_arg!(self, code, 1),
                param_arg!(self, code, 2),
            ),
            99 => OpCode::Quit,
            unrecognized => panic!("unrecognized opcode: {}", unrecognized),
        }
    }
}

impl std::ops::Index<i32> for IntCode {
    type Output = i32;

    fn index(&self, pos: i32) -> &i32 {
        if pos < 0 {
            panic!("addresses may not be negative")
        }

        &self.space[pos as usize]
    }
}

impl std::ops::IndexMut<i32> for IntCode {
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
        let prog = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut machine = IntCode::new(prog, vec![]);

        machine.run();

        assert_eq!(machine.space[0], 3500);
    }

    #[test]
    fn basic_io() {
        let prog = vec![3, 0, 4, 0, 99];
        let mut machine = IntCode::new(prog, vec![42]);

        assert_eq!(machine.run(), vec![42]);
    }
}
