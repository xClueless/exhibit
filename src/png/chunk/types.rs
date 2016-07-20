extern crate byteorder;
use self::byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};
use std::io::Read;
use std::fmt;

const IHDR_SIZE: usize = 13; 
pub struct Ihdr {
	pub width: u32,
	pub height: u32,
	pub bpp: u8,
	pub colour_type: ColourType,
	pub compression_method: CompressionMethod,
	pub filter_method: FilterMethod,
	pub interlace_method: InterlaceMethod,
}

impl Ihdr {
	pub fn from_reader(r: &mut Read, data_length: usize) -> Result<Ihdr, String> {
		if data_length != IHDR_SIZE {
			return Err(format!("Invalid IHDR: data length is {} but it should be {}", data_length, IHDR_SIZE));
		}

		let mut r = r.take(data_length as u64);

		let width = match r.read_u32::<BigEndian>() {
			Ok(width) => width,
			Err(err) => return Err(format!("Invalid IHDR: Read error on width'{}'", err)),
		};
		let height = match r.read_u32::<BigEndian>() {
			Ok(height) => height,
			Err(err) => return Err(format!("Invalid IHDR: Read error on height'{}'", err)),
		};
		let bpp = match r.read_u8() {
			Ok(bpp) => bpp,
			Err(err) => return Err(format!("Invalid IHDR: Read error on BPP '{}'", err)),
		};
		let colour_type = match r.read_u8() {
			Ok(colour_type) => colour_type,
			Err(err) => return Err(format!("Invalid IHDR: Read error on Colour Type '{}'", err)),
		};
		let colour_type = match colour_type {
			0 => ColourType::Grayscale,
			2 => ColourType::RGB,
			3 => ColourType::Indexed,
			4 => ColourType::GrayscaleAlpha,
			6 => ColourType::RGBA,
			_ => {
				return Err(format!("Invalid IHDR: Invalid variant on Colour Type '{}'", colour_type));
			},
		};
		let compression_method = match r.read_u8() {
			Ok(compression_method) => compression_method,
			Err(err) => return Err(format!("Invalid IHDR: Read error on compression_method '{}'", err)),
		};
		let compression_method = match compression_method {
			0 => CompressionMethod::Deflate,
			_ => {
				return Err(format!("Invalid IHDR: Invalid variant on compression_method '{}'", compression_method));
			},
		};
		let filter_method = match r.read_u8() {
			Ok(filter_method) => filter_method,
			Err(err) => return Err(format!("Invalid IHDR: Read error on filter '{}'", err)),
		};
		let filter_method = match filter_method {
			0 => FilterMethod::NoFilter,
			1 => FilterMethod::Sub,
			2 => FilterMethod::Up,
			3 => FilterMethod::Average,
			4 => FilterMethod::Paeth,
			_ => {
				return Err(format!("Invalid IHDR: Invalid variant on filter '{}'", filter_method));
			},
		};
		let interlace_method = match r.read_u8() {
			Ok(interlace_method) => interlace_method,
			Err(err) => return Err(format!("Invalid IHDR: Read error on interlace_method '{}'", err)),
		};
		let interlace_method = match interlace_method {
			0 => InterlaceMethod::NoInterlace,
			1 => InterlaceMethod::Adam7,
			_ => {
				return Err(format!("Invalid IHDR: Invalid variant on interlace_method '{}'", interlace_method));
			},
		};
		Ok(Ihdr{width:width,height:height,bpp:bpp,colour_type:colour_type, compression_method:compression_method, filter_method:filter_method, interlace_method:interlace_method})
	}
}
impl fmt::Debug for Ihdr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}x{} {}BPP, {:?}, {:?}, {:?}, {:?}",
               self.width, self.height, self.bpp, self.colour_type,
               self.compression_method, self.filter_method, self.interlace_method)
    }
}

#[derive(Debug)]
pub enum ColourType {
	Grayscale = 0,
	RGB = 2,
	Indexed = 3,
	GrayscaleAlpha = 4,
	RGBA = 6,
}

#[derive(Debug)]
pub enum CompressionMethod {
	Deflate = 0,
}

#[derive(Debug)]
pub enum FilterMethod {
	NoFilter = 0,
	Sub = 1,
	Up = 2,
	Average = 3,
	Paeth = 4
}

#[derive(Debug)]
pub enum InterlaceMethod {
	NoInterlace = 0,
	Adam7 = 1,
}

#[derive(Debug)]
pub struct Rgb {	
	pub red: u8,
	pub green: u8,
	pub blue: u8,
}

const PLTE_SIZE: usize = 3; 
#[derive(Debug)]
pub struct Plte {
	palette: Vec<Rgb>,
}
impl Plte {
	pub fn from_reader(r: &mut Read, data_length: usize) -> Result<Plte, String> {
		if data_length != PLTE_SIZE {
			return Err(format!("Invalid PLTE: data length is {} but it should be {}", data_length, PLTE_SIZE));
		}
		if data_length % 3 != 0 {
			return Err(format!("Invalid PLTE: data length is {} which is not divisible by three (r,g,b)", data_length));
		}

		let mut r = r.take(data_length as u64);

		let mut v: Vec<Rgb> = Vec::new();
		let mut bytes_read = 0usize;
		while bytes_read != data_length {
			let red = match r.read_u8() {
				Ok(red) => red,
				Err(err) => return Err(format!("Invalid PLTE: Read error on red '{}'", err)),
			};
			let green = match r.read_u8() {
				Ok(green) => green,
				Err(err) => return Err(format!("Invalid PLTE: Read error on green '{}'", err)),
			};
			let blue = match r.read_u8() {
				Ok(blue) => blue,
				Err(err) => return Err(format!("Invalid PLTE: Read error on blue '{}'", err)),
			};
			bytes_read += 3;
			v.push(Rgb{red:red,green:green,blue:blue});
		}
		Ok(Plte{palette:v})
	}
}

pub struct Idat {
	compressed_data: Vec<u8>,
}
impl Idat {
	pub fn from_reader(r: &mut Read, data_length: usize) -> Result<Idat, String> {
		let mut d: Vec<u8> = Vec::new();
		let bytes_read = match r.take(data_length as u64).read_to_end(&mut d) {
			Ok(bytes_read) => {
				if bytes_read != data_length {
					return Err(format!("Invalid chunk: Not enough bytes remaining. {} bytes read out of a specified {}.", bytes_read, data_length));
				}
				bytes_read
			},
			Err(err) => return Err(format!("Invalid chunk: Read error '{}'", err)),
		};

		if bytes_read != data_length {
			return Err(format!("Invalid IDAT chunk: bytes available in buffer ({}) doesn't match expected buffer length ({})", bytes_read, data_length));
		}

		Ok(Idat{compressed_data:d})
	}
	pub fn compressed_data(&self) -> &Vec<u8> {
		&self.compressed_data
	}
}
impl fmt::Debug for Idat {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "IDAT - {} compressed bytes", self.compressed_data.len())
    }
}

const SRGB_SIZE: usize = 1;
#[derive(Debug)]
pub struct sRGB {
	rendering_intent: RenderingIntent,
}

#[derive(Debug)]
pub enum RenderingIntent {
	Perceptual = 0,
	RelativeColorimetric = 1,
	Saturation = 2,
	AbsoluteColorimetric = 3
}

impl sRGB {
	pub fn from_reader(r: &mut Read, data_length: usize) -> Result<sRGB, String> {
		if data_length != SRGB_SIZE {
			return Err(format!("Invalid sRGB: data length is {} but it should be {}", data_length, SRGB_SIZE));
		}

		let mut r = r.take(data_length as u64);

		let intent: RenderingIntent = match r.read_u8() {
			Ok(intent) => {
				let intent = match intent {
					0 => RenderingIntent::Perceptual,
					1 => RenderingIntent::RelativeColorimetric,
					2 => RenderingIntent::Saturation,
					3 => RenderingIntent::AbsoluteColorimetric,
					_ => {
						return Err(format!("Invalid sRGB: Invalid rendering intent variant '{}'", intent));
					},
				};
				intent
			},
			Err(err) => return Err(format!("Invalid sRGB: Read error on rendering intent '{}'", err)),
		};
		Ok(sRGB{rendering_intent:intent})
	}
	pub fn as_string(&self) -> String {
		format!("Rendering Intent: {:?}", self.rendering_intent)
	}
}

pub struct Unknown {
	data: Vec<u8>,
}
impl Unknown {
	pub fn from_reader(r: &mut Read, data_length: usize) -> Result<Unknown, String> {
		let mut d: Vec<u8> = Vec::new();
		let bytes_read = match r.take(data_length as u64).read_to_end(&mut d) {
			Ok(bytes_read) => {
				if bytes_read != data_length {
					return Err(format!("Invalid chunk: Not enough bytes remaining. {} bytes read out of a specified {}.", bytes_read, data_length));
				}
				bytes_read
			},
			Err(err) => return Err(format!("Invalid chunk: Read error '{}'", err)),
		};

		if bytes_read != data_length {
			return Err(format!("Invalid chunk: bytes available in buffer ({}) doesn't match expected buffer length ({})", bytes_read, data_length));
		}

		Ok(Unknown{data:d})
	}
	pub fn mut_data(&mut self) -> &mut Vec<u8> {
		&mut self.data
	}
	pub fn data(&self) -> &Vec<u8> {
		&self.data
	}
}
impl fmt::Debug for Unknown {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(fmt, "Unknown Chunk Type - {} bytes", self.data.len())
	}
}