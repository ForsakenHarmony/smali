#[macro_use]
pub mod parse;

use std::{
	io,
	io::{Error, Read, Seek, SeekFrom},
};

use color_eyre::{
	eyre::{bail, WrapErr},
	Result,
};
pub use parse::{Parse, ReadThings};
use thiserror::Error;

use crate::dex::types::file::DexFile;

#[derive(Debug, Error)]
pub enum ParseError {
	#[error("parsing failed: {0}")]
	GenericError(String),
	#[error("parsing failed: bad UTF-8 byte {value:#04x} at offset {offset:#04x}")]
	BadUTF8 { value: u16, offset: usize },
	#[error("parsing failed with IO error")]
	Io(#[from] io::Error),
}

impl ParseError {
	pub fn generic<T: Into<String>>(msg: T) -> ParseError {
		ParseError::GenericError(msg.into())
	}

	pub fn bad_utf8(value: u16, offset: usize) -> ParseError {
		ParseError::BadUTF8 { value, offset }
	}
}

pub trait Parser: Seek + ReadThings + Sized {
	// fn header(&self) -> &Header;

	#[inline(always)]
	fn align(&mut self, alignment: u32) -> Result<()> {
		let offset = self.get_offset();
		let align = offset % alignment;
		if align != 0 {
			self.set_offset(offset - align + alignment)?;
		}
		Ok(())
	}

	#[inline(always)]
	// #[cfg_attr(feature = "trace", instrument(skip(self)))]
	fn parse<T: Parse>(&mut self) -> Result<T> {
		T::parse(self)
	}

	#[cfg_attr(feature = "trace", instrument(skip(self)))]
	fn get_offset(&mut self) -> u32 {
		self.stream_position()
			.expect("there should always be a current position") as u32
	}

	#[cfg_attr(feature = "trace", instrument(skip(self)))]
	fn set_offset(&mut self, offset: u32) -> Result<()> {
		self.seek(SeekFrom::Start(offset as u64))
			.map(|_| ())
			.wrap_err("seeking to new offset")
	}

	#[cfg_attr(feature = "trace", instrument(skip(self)))]
	fn offset(&mut self, offset: u32) -> Result<&mut Self> {
		self.set_offset(offset)?;
		Ok(self)
	}

	fn with_offset<T>(
		&mut self,
		offset: u32,
		f: impl FnOnce(&mut Self) -> Result<T>,
	) -> Result<Option<T>> {
		if offset == 0 {
			return Ok(None);
		}
		if offset < 112 {
			bail!("offset out of bounds");
		}

		let old_offset = self.get_offset();
		self.seek(SeekFrom::Start(offset as u64))?;
		let res = f(self)?;
		self.seek(SeekFrom::Start(old_offset as u64))?;

		Ok(Some(res))
	}

	#[cfg_attr(feature = "trace", instrument(skip(self)))]
	fn parse_with_offset<T: Parse>(&mut self, offset: u32) -> Result<Option<T>> {
		if offset == 0 {
			return Ok(None);
		}
		if offset < 112 {
			bail!("offset out of bounds");
		}

		let old_offset = self.get_offset();
		self.seek(SeekFrom::Start(offset as u64))?;
		let res = self.parse()?;
		self.seek(SeekFrom::Start(old_offset as u64))?;

		Ok(Some(res))
	}

	// #[cfg_attr(feature = "trace", instrument(skip(self)))]
	// fn parse_with_offset_in_data<T: Parse>(&mut self, offset: u32) -> Result<Option<T>> {
	// 	if offset == 0 {
	// 		return Ok(None);
	// 	}
	// 	if offset < self.header().data_off {
	// 		bail!("offset out of bounds");
	// 	}
	// 	self.parse_with_offset(offset)
	// }

	#[cfg_attr(feature = "trace", instrument(skip(self), fields(idx)))]
	fn parse_list<T: Parse>(&mut self, len: u32) -> Result<Vec<T>> {
		let mut res = Vec::with_capacity(len as usize);

		#[cfg(feature = "trace")]
		for i in 0..len {
			let span = trace_span!("parse_list_item", idx = i);
			let _ = span.enter();
			res.push(self.parse()?)
		}
		#[cfg(not(feature = "trace"))]
		for _ in 0..len {
			res.push(self.parse()?)
		}

		Ok(res)
	}

	#[cfg_attr(feature = "trace", instrument(skip(self)))]
	fn parse_list_with_offset<T: Parse>(&mut self, size: u32, offset: u32) -> Result<Vec<T>> {
		// if offset == 0 || offset < 112 {
		// 	return bail!("offset out of bounds");
		// }

		let old_offset = self.get_offset();
		self.seek(SeekFrom::Start(offset as u64))?;
		let res = self.parse_list(size)?;
		self.seek(SeekFrom::Start(old_offset as u64))?;

		Ok(res)
	}

	#[cfg_attr(feature = "trace", instrument(skip(self)))]
	fn parse_string(&mut self, len: u32) -> Result<(Vec<u8>, String)> {
		// self.set_offset(offset)?;
		// let size = self.uleb128()?;
		parse_utf8_bytes_utf16_len_string(self, len)
	}
}

#[cfg_attr(feature = "trace", instrument(skip(p)))]
fn parse_utf8_bytes_utf16_len_string<P: Parser>(p: &mut P, len: u32) -> Result<(Vec<u8>, String)> {
	let mut bytes: Vec<u8> = Vec::new();
	let mut chars: Vec<u16> = Vec::with_capacity(len as usize);

	let mut next_byte = || -> Result<u16> {
		let byte = p.u8()?;
		bytes.push(byte);

		Ok(byte as u16 & 0xFF)
	};

	let mut at = 0;
	for _ in 0..len {
		let v0 = next_byte()?;
		let out = match v0 >> 4 {
			0x00..=0x07 => {
				// 0XXXXXXX -- single-byte encoding
				if v0 == 0 {
					// A single zero byte is illegal.
					bail!(ParseError::bad_utf8(v0, at));
				}
				at += 1;
				v0
			}
			0x0c..=0x0d => {
				// 110XXXXX -- two-byte encoding
				let v1 = next_byte()?;
				if (v1 & 0xc0) != 0x80 {
					/*
					 * This should have been represented with
					 * one-byte encoding.
					 */
					bail!(ParseError::bad_utf8(v1, at + 1));
				}
				let value = ((v0 & 0x1f) << 6) | (v1 & 0x3f);
				if value != 0 && value < 0x80 {
					bail!(ParseError::bad_utf8(v1, at + 1));
				}
				at += 2;
				value
			}
			0x0e => {
				// 1110XXXX -- three-byte encoding
				let v1 = next_byte()?;
				if (v1 & 0xc0) != 0x80 {
					bail!(ParseError::bad_utf8(v1, at + 1));
				}
				let v2 = next_byte()?;
				if (v1 & 0xc0) != 0x80 {
					bail!(ParseError::bad_utf8(v2, at + 2));
				}
				let value = ((v0 & 0x0f) << 12) | ((v1 & 0x3f) << 6) | (v2 & 0x3f);
				if value < 0x800 {
					/*
					 * This should have been represented with one- or
					 * two-byte encoding.
					 */
					bail!(ParseError::bad_utf8(v2, at + 2));
				}
				at += 3;
				value
			}
			_ => bail!(ParseError::bad_utf8(v0, at)),
		};
		chars.push(out);
	}

	Ok((bytes, String::from_utf16_lossy(&chars).to_string()))
}

pub struct FileParser<R: Read + Seek> {
	reader: R,
	// header: Header,
}

impl<R: Read + Seek> FileParser<R> {
	#[cfg_attr(feature = "trace", instrument(skip(reader)))]
	pub fn new(reader: R) -> Result<Self> {
		// let header = Header::parse(&mut reader)?;

		Ok(FileParser {
			reader,
			// header
		})
	}

	// #[cfg_attr(feature = "trace", instrument(skip(self)))]
	// fn parse_array<T: Parse>(&mut self, size: u32, offset: Option<u32>) -> Result<Vec<T>> {
	// 	if let Some(offset) = offset {
	// 		assert_eq!(
	// 			self.get_offset(),
	// 			offset,
	// 			"current offset {} is not the expected offset {}",
	// 			self.get_offset(),
	// 			offset
	// 		);
	// 	}
	//
	// 	self.parse_list(size)
	// }

	#[cfg_attr(feature = "trace", instrument(skip(self)))]
	pub fn parse_file(&mut self) -> Result<DexFile> {
		self.parse()
	}

	// #[cfg_attr(feature = "trace", instrument(skip(self)))]
	// pub fn parse_file(&mut self) -> Result<DexFile> {
	// 	let header = self.header;
	// 	debug!("Header: {:#?}", header);
	//
	// 	debug!("parsing map list");
	// 	let map_list: MapList = self
	// 		// .with_offset(header.map_off, |p| p.parse())
	// 		// .offset(header.map_off)
	// 		.parse_with_offset(header.map_off)
	// 		.wrap_err("parsing map list")?
	// 		.unwrap();
	//
	// 	let map = map_list.map()?;
	// 	debug!("Map: {:#?}", map);
	//
	// 	debug!("parsing strings ids");
	// 	let string_ids: Vec<StringIdItem> = self
	// 		.parse_array(map.string_id_item.size, Some(map.string_id_item.offset))
	// 		.wrap_err("parsing strings ids")?;
	//
	// 	debug!("parsing type ids");
	// 	let type_ids = self
	// 		.parse_array(header.type_ids_size, Some(header.type_ids_off))
	// 		.wrap_err("parsing type ids")?;
	//
	// 	debug!("parsing proto ids");
	// 	let proto_ids = self
	// 		.parse_array(header.proto_ids_size, Some(header.proto_ids_off))
	// 		.wrap_err("parsing proto ids")?;
	//
	// 	debug!("parsing field ids");
	// 	let field_ids = self
	// 		.parse_array(header.field_ids_size, Some(header.field_ids_off))
	// 		.wrap_err("parsing field ids")?;
	//
	// 	debug!("parsing method ids");
	// 	let method_ids = self
	// 		.parse_array(header.method_ids_size, Some(header.method_ids_off))
	// 		.wrap_err("parsing method ids")?;
	//
	// 	debug!("parsing class defs");
	// 	let class_defs = self
	// 		.parse_array(header.class_defs_size, Some(header.class_defs_off))
	// 		.wrap_err("parsing class defs")?;
	//
	// 	// debug!("parsing call_site_ids");
	// 	// let call_site_ids = self.parse_array(header.call)?;
	// 	// let method_handles = self.read_array(None)?;
	//
	// 	// let mut data = Vec::with_capacity(header.data_size as usize);
	// 	// self.set_offset(header.data_off)?;
	// 	// self.read(&mut data)?;
	//
	// 	let mut strings = Vec::with_capacity(string_ids.len());
	// 	for id in string_ids.iter() {
	// 		let str = self.parse_string(*id.string_data_off)?;
	// 		strings.push(str);
	// 	}
	//
	// 	Ok(DexFile {
	// 		header,
	// 		map_list,
	//
	// 		string_ids,
	// 		type_ids,
	// 		proto_ids,
	// 		field_ids,
	// 		method_ids,
	// 		class_defs,
	// 		code: vec![],
	// 		debug_info: vec![],
	// 		type_lists: vec![],
	// 		string_data: vec![],
	// 		annotations: vec![],
	// 		class_data: vec![],
	// 		encoded_arrays: vec![],
	// 		annotation_sets: vec![],
	// 		annotation_set_ref_lists: vec![],
	// 		annotation_directories: vec![],
	// 		call_site_ids: vec![],
	// 		method_handles: vec![],
	//
	// 		strings,
	//
	// 		data: vec![],
	// 		link_data: vec![],
	// 	})
	// }
}

// impl<R: Read + Seek> Parser for FileParser<R> {
// 	fn header(&self) -> &Header {
// 		&self.header
// 	}
// }

impl<R: Read + Seek> Parser for R {}

impl<R: Read + Seek> Read for FileParser<R> {
	fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
		self.reader.read(buf)
	}
}

impl<R: Read + Seek> Seek for FileParser<R> {
	fn seek(&mut self, pos: SeekFrom) -> Result<u64, Error> {
		self.reader.seek(pos)
	}
}
