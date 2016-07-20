extern crate byteorder;
extern crate getopts;
pub mod png;

use std::fs::File;
use std::io::{Read, BufReader};
use std::env;
use getopts::Options;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    println!("input: {}", input);
    let f = File::open(input).expect("Failed to open file");
    let fmeta = f.metadata().expect("Failed to read file metadata");
	let mut r = BufReader::new(&f);
	let image = png::Image::from_reader(&mut r as &mut Read, fmeta.len() as usize)
        .expect("Failed to parse image");

}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::io::{Read, BufReader};
    use std::fs::File;

    #[test]
    fn can_parse_valid_png() {
        let f = File::open("valid.png").expect("Failed to open file");
        let fmeta = f.metadata().expect("Failed to read file metadata");
        let mut r = BufReader::new(&f);
        let image = png::Image::from_reader(&mut r as &mut Read, fmeta.len() as usize)
            .expect("Failed to parse image");
    }
    #[test]
    #[should_panic]
    fn can_not_parse_non_png() {
        let f = File::open("valid.svg").expect("Failed to open file");
        let fmeta = f.metadata().expect("Failed to read file metadata");
        let mut r = BufReader::new(&f);
        let image = png::Image::from_reader(&mut r as &mut Read, fmeta.len() as usize)
            .expect("Failed to parse image");
    }
}