use std::io::{Read, BufReader};
use std::str;
mod chunk;
mod metadata;

extern crate byteorder;
use self::byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};

extern crate flate2; 
use self::flate2::read::ZlibDecoder;

pub struct Image {
	pub sig: metadata::Signature,
	pub chunks: Vec<chunk::Chunk>,
}

impl Image {
	pub fn from_reader(r: &mut Read, file_len: usize) -> Result<Image, String>
	{
		let mut bytes_read = 0usize;
		let mut sig = match metadata::Signature::from_reader(r) {
			Ok(sig) => sig,
			Err(err) => return Err(err),
		};

		bytes_read += metadata::SIGNATURE_SIZE;

		let mut chunks: Vec<chunk::Chunk> = Vec::new();
		while bytes_read < file_len
		{
			let c = match chunk::Chunk::from_reader(r) {
				Ok(c) => c,
				Err(err) => return Err(err),
			};
			bytes_read += chunk::CHUNK_META_SIZE + c.data_length as usize;
			chunks.push(c);
		}

		if chunks.len() == 0
		{
			return Err(String::from_str("Invalid image. Doesn't contain any chunks."));
		}


		Ok(Image{sig:sig, chunks:chunks})
	}
	// pub fn condense_idat(&self) {
	// 	let idat_iter = self.chunks.iter().filter(|&x| x.data() == chunk::ChunkData::Idat);

	// 	if idat_iter.count() < 2 {
	// 		return;
	// 	}

	// 	let lead_idat = idat_iter.next();

 //    	self.chunks.retain(|&x| x == lead_idat || x.data() != chunk::ChunkData::Idat);
 //  //   	s.iter().filter(|&x| x == ChunkData::Idat).skip(1) {
	// 	// 	match chunk.data {
	// 	// 		chunk::ChunkData::Idat(idat) => compressed_data.push_all(idat.compressed_data()),
	// 	// 		_ => {}
	// 	// 	};
	// 	// }

	//     // let mut decoder = ZlibDecoder::new(compressed_data);// {
	//     // // 	Ok(decoder) => decoder,
	//     // // 	Err(err) => return format!("Failed to inflate data steam: {}", err),
	//     // // };
	//     // let mut decompressed_data: Vec<u8> = Vec::new();
	//     // let bytes_read = match decoder.read_to_end(&mut decompressed_data) {
	//     // 	Ok(bytes_read) => bytes_read,
	//     // 	Err(err) => return Err(format!("Failed to inflate data steam: {}", err)),
	//     // };

	// }
	// pub fn inflate_image_data(&self) -> Result<Vec<u8>, String> {
	// 	let mut compressed_data: Vec<u8> = Vec::new();
	// 	for chunk in self.chunks.iter() {
	// 		match chunk.data {
	// 			chunk::ChunkData::Idat(idat) => compressed_data.push_all(idat.compressed_data()),
	// 			_ => {}
	// 		};
	// 	}

	//     let mut decoder = ZlibDecoder::new(compressed_data);// {
	//     // 	Ok(decoder) => decoder,
	//     // 	Err(err) => return format!("Failed to inflate data steam: {}", err),
	//     // };
	//     let mut decompressed_data: Vec<u8> = Vec::new();
	//     let bytes_read = match decoder.read_to_end(&mut decompressed_data) {
	//     	Ok(bytes_read) => bytes_read,
	//     	Err(err) => return Err(format!("Failed to inflate data steam: {}", err)),
	//     };

	//     Ok(decompressed_data)
	// }
}

