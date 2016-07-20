use std::io::{Read};
use std::str;
use std::fmt;
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};
mod types;

pub const CHUNK_META_SIZE: usize = 12; 
pub struct ChunkType {
	data: Vec<u8>,
}

impl ChunkType {
	pub fn from_reader(r: &mut Read) -> Result<ChunkType, String> {
		let mut data: Vec<u8> = Vec::new();

		match r.take(4).read_to_end(&mut data) {
			Ok(bytes_read) => {
				if bytes_read != 4 {
					return Err(format!("Invalid chunk type: Not enough bytes remaining. {} bytes read.", bytes_read));
				}
			},
			Err(err) => return Err(format!("Invalid chunk type: Read error '{}'", err)),
		};

		Ok(ChunkType{data:data})
	}
	pub fn is_critical(&self) -> bool {
		(self.data[0] as char).is_uppercase()
	}
	pub fn is_public(&self) -> bool {
		(self.data[1] as char).is_uppercase()
	}
	pub fn is_safe_to_copy(&self) -> bool {
		(self.data[3] as char).is_uppercase()
	}
	pub fn as_string(&self) -> &str {
		let string = match str::from_utf8(&self.data[0..4]) {
			Ok(string) => string,
			Err(err) => "Invalid chunk type string, not valid UTF8",
		};

		string
	}
}
impl fmt::Debug for ChunkType {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(fmt, "{}", self.as_string())
	}
}
pub struct Chunk {
	data_length: u32,
	t: ChunkType,
	data: ChunkData,
	crc: u32,
}

impl Chunk {
	pub fn from_reader(r: &mut Read) -> Result<Chunk, String> {
		let data_length = match r.read_u32::<BigEndian>() {
			Ok(data_length) => data_length,
			Err(err) => return Err(format!("Invalid chunk length: Read error '{}'", err)),
		};


		let tv: Vec<u8> = Vec::new();
		let t = ChunkType{data:tv};
		let t = match ChunkType::from_reader(r) {
			Ok(t) => t,
			Err(err) => return Err(err),
		};


		let data: ChunkData = match t.as_string() {
			"IHDR" => {
				let ihdr = match types::Ihdr::from_reader(r, data_length as usize) {
					Ok(ihdr) => ihdr,
					Err(err) => return Err(err),
				};
				ChunkData::Ihdr(ihdr)
			},
			"PLTE" => {
				let plte = match types::Plte::from_reader(r, data_length as usize) {
					Ok(plte) => plte,
					Err(err) => return Err(err),
				};
				ChunkData::Plte(plte)
			},
			"sRGB" => {
				let srgb = match types::sRGB::from_reader(r, data_length as usize) {
					Ok(srgb) => srgb,
					Err(err) => return Err(err),
				};
				println!("{:?}", srgb.as_string());
				ChunkData::Srgb(srgb)
			},
			"IDAT" => {
				let idat = match types::Idat::from_reader(r, data_length as usize) {
					Ok(idat) => idat,
					Err(err) => return Err(err),
				};
				ChunkData::Idat(idat)
			},

			"IEND" => ChunkData::Iend,
			_ => {
				let chunk_name = t.as_string();
				if t.is_critical() {
					return Err(format!("Unhandled critical chunk '{}'", chunk_name));
				}

				if !t.is_safe_to_copy() {
					println!("[WARN] Ignoring unhandled non-critical chunk {} because it is not safely copyable", chunk_name);
					let mut byte_bin: Vec<u8> = Vec::new();
					let _ = r.take(data_length as u64).read_to_end(&mut byte_bin);
					ChunkData::Ignored
				}
				else {
					println!("[INFO] Copying unhandled non-critical chunk {}", chunk_name);

					let unknown = match types::Unknown::from_reader(r, data_length as usize) {
						Ok(unknown) => unknown,
						Err(err) => return Err(format!("[Chunk {}] {}", chunk_name, err)),
					};
					ChunkData::Unknown(unknown)
				}
			},
		};

		let crc = match r.read_u32::<BigEndian>() {
			Ok(crc) => crc,
			Err(err) => return Err(format!("Invalid chunk: Read error '{}'", err)),
		};

		Ok(Chunk{data_length:data_length, t:t, data:data, crc:crc})
	}
	pub fn data(&self) -> &ChunkData {
		&self.data
	}
	pub fn ctype(&self) -> &ChunkType {
		&self.t
	}
	pub fn data_len(&self) -> u32 {
		self.data_length
	}
}

#[derive(Debug)]
pub enum ChunkData {
	Ihdr(types::Ihdr),
	Plte(types::Plte),
	Idat(types::Idat),
	Srgb(types::sRGB),
	Unknown(types::Unknown),
	Ignored,
	Iend,
}
