
#![feature(collections)]
#![feature(convert)]

mod png;

use std::fs::File;
use std::io::{Read, BufReader};

fn main() {
	let f = match File::open("valid.png") {
	// let f = match File::open("C:\\Users\\Dan\\Pictures\\KCvuoJV.png") {
		Ok(f) => f,
		Err(err) => {
			println!("[Error] Failed to open file: {:?}", err);
			return;
		},
	};


	let fmeta = match f.metadata() {
		Ok(fmeta) => fmeta,
		Err(err) => {
			println!("[Error] Failed to read file metadata: {:?}", err);
			return;
		},
	};

	let mut r = BufReader::new(&f);
	let image = match png::Image::from_reader(&mut r as &mut Read, fmeta.len() as usize) {
		Ok(image) => image,
		Err(e) => {
			println!("[Error] Failed to parse image: {:?}", e);
			return;	
		},
	};
}
