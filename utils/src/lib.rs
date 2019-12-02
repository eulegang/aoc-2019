use std::io::{self, BufRead, Write};
use std::process::exit;
use std::str::FromStr;

pub fn prompt<T: FromStr>(msg: &str) -> T {
    print!("{}", msg);
    let sin = io::stdin();
    let mut buf = String::new();
    let _ = io::stdout().flush();

    if let Err(err) = sin.read_line(&mut buf) {
        eprintln!("{}", err);
        exit(1);
    }

    match T::from_str(buf.trim_end_matches("\n")) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("unable to parse {:?} into {}", buf, stringify!(T));
            exit(1);
        }
    }
}

pub fn lines<T: FromStr>() -> Vec<T> {
    let sin = io::stdin();

    sin.lock()
        .lines()
        .map(|event| match event {
            Ok(line) => line,
            Err(err) => {
                eprintln!("error reading line: {}", err);
                exit(1);
            }
        })
        .map(|line| match T::from_str(line.trim_end_matches("\n")) {
            Ok(t) => t,
            Err(_) => {
                eprintln!("unable to parse {:?} into {}", line, stringify!(T));
                exit(1);
            }
        })
        .collect()
}
