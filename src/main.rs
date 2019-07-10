// https://gist.github.com/ayosec/2ee0993247e003b42c5c

use std::{env, fs};
use std::io::{self, BufReader, BufRead};

enum Input {
    Standard(io::Stdin),
    File(fs::File),
}

impl Input {
    fn stdin() -> Input {
        Input::Standard(io::stdin())
    }

    fn file(path: String) -> io::Result<Input> {
        Ok(Input::File(fs::File::open(path)?))
    }

    fn from_arg(arg: Option<String>) -> io::Result<Input> {
        Ok(match arg {
            None => Input::stdin(),
            Some(path) => Input::file(path)?,
        })
    }
}

impl io::Read for Input {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            Input::Standard(ref mut s) => s.read(buf),
            Input::File(ref mut f) => f.read(buf),
        }
    }
}

fn load_inputs() -> io::Result<(Input)> {
    let mut args = env::args().skip(1);
    Ok(Input::from_arg(args.next())?)

}

fn main() {
    let input = match load_inputs() {
        Ok(input) => input,
        Err(error) => panic!("Failed: {}", error),
    };
    let reader = BufReader::new(input);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}
