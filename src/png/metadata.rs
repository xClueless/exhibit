use std::io::{Read, BufReader};
use std::str;

pub const SIGNATURE_SIZE: usize = 16; 
pub struct Signature;
impl Signature {
	pub fn from_reader(r: &mut Read) -> Result<Signature, String>
	{
		let mut data: Vec<u8> = Vec::new();

		match r.take(8).read_to_end(&mut data) {
			Ok(bytes_read) => {
				if bytes_read != 8 {
					return Err(format!("Invalid signature: Not enough bytes remaining. {} bytes read.", bytes_read));
				}
			},
			Err(err) => return Err(format!("Invalid signature: Read error '{}'", err)),
		};

		if data[0] != 0x89 {
			return Err(format!("Invalid signature: Byte 1 was {}, should have been {}", data[0], 0x89));
		}

		let ascii_mag = &mut data[1..4];

		let ascii_mag = match str::from_utf8(&ascii_mag) {
			Ok(ascii) => ascii,
			Err(err) => return Err(format!("Invalid signature: ASCII bytes are not valid UTF8. {}", err)),
		};
		if ascii_mag != "PNG" {
			return Err(format!("Invalid signature: ASCII Signature was {}, should have been {}", ascii_mag, "PNG"));
		}

		Ok(Signature)
	}
}