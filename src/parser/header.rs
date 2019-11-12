use crate::parser::{ParseError, ParseResult, ReadThings};
use std::convert::{TryFrom, TryInto};

const ENDIAN_CONSTANT: u32 = 0x12345678;
const REVERSE_ENDIAN_CONSTANT: u32 = 0x78563412;

#[derive(Debug, Clone)]
pub enum EndianConstant {
	EndianConstant,
	ReverseEndianConstant,
}

impl TryFrom<u32> for EndianConstant {
	type Error = ParseError;

	fn try_from(value: u32) -> Result<Self, Self::Error> {
		match value {
			ENDIAN_CONSTANT => Ok(EndianConstant::EndianConstant),
			REVERSE_ENDIAN_CONSTANT => Ok(EndianConstant::ReverseEndianConstant),
			_ => Err(ParseError::GenericError("Not a valid endian constant".to_string()).into()),
		}
	}
}

#[derive(Debug, Clone)]
pub struct Header {
	pub format_version: u32,
	pub checksum: u32,
	pub signature: Vec<u8>,
	pub file_size: u32,
	pub header_size: u32,
	pub endian_tag: EndianConstant,
	pub link_size: u32,
	pub link_off: u32,
	pub map_off: u32,
	pub string_ids_size: u32,
	pub string_ids_off: u32,
	pub type_ids_size: u32,
	pub type_ids_off: u32,
	pub proto_ids_size: u32,
	pub proto_ids_off: u32,
	pub field_ids_size: u32,
	pub field_ids_off: u32,
	pub method_ids_size: u32,
	pub method_ids_off: u32,
	pub class_defs_size: u32,
	pub class_defs_off: u32,
	pub data_size: u32,
	pub data_off: u32,
}

const DEX_FILE_MAGIC: [u8; 8] = [0x64, 0x65, 0x78, 0x0a, 0x00, 0x00, 0x00, 0x00];
//                                                       ^^^^^version^^^^^

impl Header {
	fn verify_header(parser: &mut impl ReadThings) -> ParseResult<u32> {
		let mut magic = vec![0; 8];
		parser.read(&mut magic)?;
		println!("magic {:?}", magic);
		println!("magic {:?}", DEX_FILE_MAGIC);
		if magic[..4] != DEX_FILE_MAGIC[..4] || magic[7] != DEX_FILE_MAGIC[7] {
			return Err(ParseError::GenericError("Magic doesn't match".to_string()));
		}

		let version_string = String::from_utf8(magic[4..7].to_vec()).map_err(|e|
			ParseError::GenericError(e.to_string())
		)?;
		let version = version_string.parse::<u32>().map_err(|e|
			ParseError::GenericError(e.to_string())
		)?;
		Ok(version)
	}

	pub fn parse(parser: &mut impl ReadThings) -> Result<Self, ParseError> {
		let version = Self::verify_header(parser)?;

		Ok(Header {
			format_version: version,
			checksum: parser.u32()?,
			signature: {
				let mut signature = vec![0; 20];
				parser.read(&mut signature)?;
				signature
			},
			file_size: parser.u32()?,
			header_size: parser.u32()?,
			endian_tag: parser.u32()?.try_into()?,
			link_size: parser.u32()?,
			link_off: parser.u32()?,
			map_off: parser.u32()?,
			string_ids_size: parser.u32()?,
			string_ids_off: parser.u32()?,
			type_ids_size: parser.u32()?,
			type_ids_off: parser.u32()?,
			proto_ids_size: parser.u32()?,
			proto_ids_off: parser.u32()?,
			field_ids_size: parser.u32()?,
			field_ids_off: parser.u32()?,
			method_ids_size: parser.u32()?,
			method_ids_off: parser.u32()?,
			class_defs_size: parser.u32()?,
			class_defs_off: parser.u32()?,
			data_size: parser.u32()?,
			data_off: parser.u32()?,
		})
	}
}
