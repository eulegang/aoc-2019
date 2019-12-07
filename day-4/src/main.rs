fn main() {
    let mut seed = [2u8, 7, 3, 0, 2, 5];
    let mut end_buf = [7u8, 6, 7, 2, 5, 3];

    let mut pass = Pass(&mut seed);
    let end = Pass(&mut end_buf);

    while pass < end {
        if pass.accept() {
            println!("{}", &pass);
        }

        pass.inc();
    }
}

struct Pass<'a>(&'a mut [u8]);

impl<'a> Pass<'a> {
    fn inc(&mut self) {
        let last = self.0.len() - 1;

        let mut idx = last;
        loop {
            self.0[idx] += 1;

            if self.0[idx] < 10 {
                break;
            }

            if idx == 0 {
                break;
            }

            idx -= 1;
        }

        idx += 1;
        while idx <= last {
            self.0[idx] = self.0[idx - 1];
            idx += 1;
        }
    }

    fn accept(&self) -> bool {
        let mut iter = self.0.iter().peekable();
        let mut seq = false;

        while let (Some(cur), Some(next)) = (iter.next(), iter.peek()) {
            if cur == *next {
                seq = true;
            }

            if *next < cur {
                return false;
            }
        }

        seq
    }
}

impl<'a> PartialEq for Pass<'a> {
    fn eq(&self, other: &Pass) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }

        for i in 0..self.0.len() {
            if self.0[i] != other.0[i] {
                return false;
            }
        }

        true
    }
}

impl<'a> std::fmt::Display for Pass<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in self.0.iter() {
            write!(fmt, "{}", i)?;
        }

        Ok(())
    }
}

impl<'a> std::fmt::Debug for Pass<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in self.0.iter() {
            write!(fmt, "{},", i)?;
        }

        Ok(())
    }
}

impl<'a> PartialOrd for Pass<'a> {
    fn partial_cmp(&self, other: &Pass) -> Option<std::cmp::Ordering> {
        if self.0.len() != other.0.len() {
            return None;
        }

        for i in 0..self.0.len() {
            if self.0[i] < other.0[i] {
                return Some(std::cmp::Ordering::Less);
            } else if self.0[i] > other.0[i] {
                return Some(std::cmp::Ordering::Greater);
            }
        }

        return Some(std::cmp::Ordering::Equal);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_inc() {
        let mut buf = [0, 0];
        let mut pass = Pass(&mut buf);

        pass.inc();

        assert_eq!(buf, [0, 1]);
    }

    #[test]
    fn overflow_seq() {
        let mut buf = [2, 1, 9];
        let mut pass = Pass(&mut buf);

        pass.inc();

        assert_eq!(buf, [2, 2, 2]);
    }

    #[test]
    fn basic_inc_large() {
        let mut buf = [0, 0, 0, 0, 0, 0];
        let mut pass = Pass(&mut buf);

        pass.inc();

        assert_eq!(buf, [0, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn bound_check() {
        let mut check = [7u8, 7, 7, 7, 7, 7];
        let mut end_buf = [7u8, 6, 7, 2, 5, 3];

        let pass_a = Pass(&mut check);
        let pass_b = Pass(&mut end_buf);

        assert!(pass_b < pass_a);
    }
}
