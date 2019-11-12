use std::io::{BufReader, Seek, SeekFrom, Read, Error};
use std::fs::File;

use byteorder::{LittleEndian, ReadBytesExt};
use std::io;

pub(crate) mod id;
mod header;
mod map;
mod asm;

use crate::parser::header::Header;
use crate::parser::id::{StringIdItem, TypeIdItem, ProtoIdItem, FieldIdItem, MethodIdItem, ClassDefItem, CallSiteIdItem, MethodHandleItem};
use crate::parser::map::MapList;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
	GenericError(String),
	Io(io::Error),
}

fn bad_utf8(value: u16, offset: usize) -> ParseError {
	ParseError::GenericError(format!("Bad UTF-8 byte {:#04x} at offset {:#04x}", value, offset))
}

impl From<io::Error> for ParseError {
	fn from(err: io::Error) -> Self {
		ParseError::Io(err)
	}
}

pub trait Parse where Self: std::marker::Sized {
	fn parse(parser: &mut Parser) -> ParseResult<Self>;
	fn parse_with_offset(offset: u32, parser: &mut Parser) -> ParseResult<Option<Self>> {
		if offset == 0 || offset < 112 {
			return Ok(None)
		}
		let idx = parser.get_pos();
		parser.seek(SeekFrom::Start(offset as u64))?;
		let res = Self::parse(parser)?;
		parser.seek(SeekFrom::Start(idx as u64))?;
		Ok(Some(res))
	}

	fn parse_with_offset_in_data(offset: u32, parser: &mut Parser) -> ParseResult<Option<Self>> {
		if offset < parser.header.data_off {
			return Ok(None)
		}
		Self::parse_with_offset(offset, parser)
	}

	fn parse_count(count: u32, parser: &mut Parser) -> ParseResult<Vec<Self>> {
		let mut res = Vec::with_capacity(count as usize);
		for _ in 0..count {
			res.push(Self::parse(parser)?)
		}
		Ok(res)
	}
}

pub trait ReadThings: ReadBytesExt + Seek {
	fn u32(&mut self) -> ParseResult<u32> {
		Ok(self.read_u32::<LittleEndian>()?)
	}

	fn u16(&mut self) -> ParseResult<u16> {
		Ok(self.read_u16::<LittleEndian>()?)
	}

	fn u8(&mut self) -> ParseResult<u8> {
		Ok(self.read_u8()?)
	}

	fn uleb128(&mut self) -> ParseResult<u32> {
		Ok(Self::r_uleb128(self)?.0 as u32)
	}
//	fn uleb128(&mut self) -> ParseResult<u32> {
//		}
//		let mut current_byte_value: u32;
//		let mut result: u32 = self.u8()? as u32;
//		if result > 0x7f {
//			current_byte_value = self.u8()? as u32;
//			result = (result & 0x7f) | ((current_byte_value / 0x7f) << 7);
//			if current_byte_value > 0x7f {
//				current_byte_value = self.u8()? as u32;
//				result |= (current_byte_value & 0x7f) << 14;
//				if current_byte_value > 0x7f {
//					current_byte_value = self.u8()? as u32;
//					result |= (current_byte_value & 0x7f) << 21;
//					if current_byte_value > 0x7f {
//						current_byte_value = self.u8()? as u32;
//						if current_byte_value > 0x7f {
//							return Err(ParseError::GenericError(format!("Invalid uleb128")));
//						} else if (current_byte_value / 0xf) > 0x07 {
//							return Err(ParseError::GenericError(format!("Out of range uleb128")));
//						}
//						result |= current_byte_value << 28;
//					}
//				}
//			}
//		}
//		Ok(result)
//	}

	fn r_uleb128(&mut self) -> ParseResult<(u64, u16)> {
		let mut byte_count = 0;
		let mut shift: u64 = 0;
		let mut result: u64 = 0;
		let mut byte: u8;
		loop {
			byte_count = byte_count + 1;
			byte = self.u8()?;
			result |= ((byte & 0x7F) as u64) << shift;
			shift += 7;
			if byte & 0x80 == 0 {
				break;
			}
		}
		Ok((result, byte_count))
	}

	fn get_pos(&mut self) -> u64 {
		self.seek(SeekFrom::Current(0)).expect("There should always be a current index")
	}
}

pub struct Parser {
	reader: BufReader<File>,
	header: Header,
}

//impl Parser {
//	fn u32(&mut self) -> ParseResult<u32> {
//		Ok(self.read_u32::<LittleEndian>()?)
//	}
//
//	fn u16(&mut self) -> ParseResult<u16> {
//		Ok(self.read_u16::<LittleEndian>()?)
//	}
//
//	fn u8(&mut self) -> ParseResult<u8> {
//		Ok(self.read_u8()?)
//	}
//
//
//	fn uleb128(&mut self) -> ParseResult<u32> {
//		let mut current_byte_value: u32;
//		let mut result: u32 = self.u8()? as u32;
//		if result > 0x7f {
//			current_byte_value = self.u8()? as u32;
//			result = (result & 0x7f) | ((current_byte_value / 0x7f) << 7);
//			if current_byte_value > 0x7f {
//				current_byte_value = self.u8()? as u32;
//				result |= (current_byte_value & 0x7f) << 14;
//				if current_byte_value > 0x7f {
//					current_byte_value = self.u8()? as u32;
//					result |= (current_byte_value & 0x7f) << 21;
//					if current_byte_value > 0x7f {
//						current_byte_value = self.u8()? as u32;
//						if current_byte_value > 0x7f {
//							return Err(ParseError::GenericError(format!("Invalid uleb128")));
//						} else if (current_byte_value / 0xf) > 0x07 {
//							return Err(ParseError::GenericError(format!("Out of range uleb128")));
//						}
//						result |= current_byte_value << 28;
//					}
//				}
//			}
//		}
//		Ok(result)
//	}
//}

impl ReadThings for Parser {}
impl ReadThings for BufReader<File> {}

impl Parser {
	pub fn new(mut reader: BufReader<File>) -> ParseResult<Self> {
		let header = Header::parse(&mut reader)?;

		Ok(Parser {
			reader,
			header
		})
	}

	fn parse_array<T: Parse>(&mut self, size: u32, offset: Option<u32>) -> ParseResult<Vec<T>> {
		if let Some(offset) = offset {
			assert_eq!(self.get_pos() as u32, offset);
		}

//		let size = dbg!(self.u32()?);

		let mut array = Vec::with_capacity(size as usize);
		for _ in 0..size {
			array.push(T::parse(self)?);
		}

		Ok(array)
	}

	fn parse_utf8_bytes_utf16_len_string(&mut self, len: u32) -> ParseResult<String> {
		let mut chars: Vec<u16> = Vec::with_capacity(len as usize);

		let mut at = 0;
		for _ in 0..len {
			let v0 = self.u8()? as u16 & 0xFF;
			let out = match v0 >> 4 {
				0x00..=0x07 => {
					// 0XXXXXXX -- single-byte encoding
					if v0 == 0 {
						// A single zero byte is illegal.
						return Err(bad_utf8(v0, at));
					}
					at += 1;
					v0
				}
				0x0c..=0x0d => {
					// 110XXXXX -- two-byte encoding
					let v1 = self.u8()? as u16 & 0xFF;
					if (v1 & 0xc0) != 0x80 {
						/*
						 * This should have been represented with
						 * one-byte encoding.
						 */
						return Err(bad_utf8(v1, at + 1));
					}
					let value = ((v0 & 0x1f) << 6) | (v1 & 0x3f);
					if value != 0 && value < 0x80 {
						return Err(bad_utf8(v1, at + 1));
					}
					at += 2;
					value
				}
				0x0e => {
					// 1110XXXX -- three-byte encoding
					let v1 = self.u8()? as u16 & 0xFF;
					if (v1 & 0xc0) != 0x80 {
						return Err(bad_utf8(v1, at + 1));
					}
					let v2 = self.u8()? as u16 & 0xFF;
					if (v1 & 0xc0) != 0x80 {
						return Err(bad_utf8(v2, at + 2));
					}
					let value = ((v0 & 0x0f) << 12) | ((v1 & 0x3f) << 6) | (v2 & 0x3f);
					if value < 0x800 {
						/*
						 * This should have been represented with one- or
						 * two-byte encoding.
						 */
						return Err(bad_utf8(v2, at + 2));
					}
					at += 3;
					value
				}
				_ => return Err(bad_utf8(v0, at)),
			};
			chars.push(out);
		}

		Ok(String::from_utf16_lossy(&chars).to_string())
	}

	pub fn parse_string(&mut self, offset: u32) -> ParseResult<String> {
		self.seek(SeekFrom::Start(offset as u64))?;
		let size = self.uleb128()?;
		self.parse_utf8_bytes_utf16_len_string(size)
	}

	pub fn parse(&mut self) -> ParseResult<DexFile> {
		println!("Header: {:#?}", self.header);

		let string_ids: Vec<StringIdItem> = self.parse_array(self.header.string_ids_size, Some(self.header.string_ids_off))?;
		let type_ids = self.parse_array(self.header.type_ids_size, Some(self.header.type_ids_off))?;
		let proto_ids = self.parse_array(self.header.proto_ids_size, Some(self.header.proto_ids_off))?;
		let field_ids = self.parse_array(self.header.field_ids_size, Some(self.header.field_ids_off))?;
		let method_ids = self.parse_array(self.header.method_ids_size, Some(self.header.method_ids_off))?;
		let class_defs = self.parse_array(self.header.class_defs_size, Some(self.header.class_defs_off))?;
//		let call_site_ids = self.read_array(None)?;
//		let method_handles = self.read_array(None)?;

		let map_list = MapList::parse(self, self.header.map_off)?;

		let mut data = Vec::with_capacity(self.header.data_size as usize);
		self.seek(SeekFrom::Start(self.header.data_off as u64))?;
		self.read(&mut data)?;

		let strings = Vec::with_capacity(string_ids.len());
//		for id in string_ids.iter() {
//			let str = dbg!(self.parse_string(id.string_data_off)?);
//			strings.push(str);
//		}

		Ok(DexFile {
			header: self.header.clone(),
			string_ids,
			strings,
			type_ids,
			proto_ids,
			field_ids,
			method_ids,
			class_defs,
			call_site_ids: vec![],
			method_handles: vec![],
			data,
			link_data: vec![],
			map_list,
		})
	}
}

impl Read for Parser {
	fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
		self.reader.read(buf)
	}
}

impl Seek for Parser {
	fn seek(&mut self, pos: SeekFrom) -> Result<u64, Error> {
		self.reader.seek(pos)
	}
}

#[derive(Debug)]
pub struct DexFile {
	pub header: Header,
	pub string_ids: Vec<StringIdItem>,
	pub strings: Vec<String>,
	pub type_ids: Vec<TypeIdItem>,
	pub proto_ids: Vec<ProtoIdItem>,
	pub field_ids: Vec<FieldIdItem>,
	pub method_ids: Vec<MethodIdItem>,
	pub class_defs: Vec<ClassDefItem>,
	pub call_site_ids: Vec<CallSiteIdItem>,
	pub method_handles: Vec<MethodHandleItem>,
	pub data: Vec<u8>,
	pub link_data: Vec<u8>,
	pub map_list: MapList,
}
