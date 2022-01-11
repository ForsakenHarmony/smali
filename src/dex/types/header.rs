use std::convert::{TryFrom, TryInto};

use color_eyre::{
	eyre::{bail, WrapErr},
	Result,
};

use crate::dex::parser::{Parse, ParseError, Parser};

const ENDIAN_CONSTANT: u32 = 0x12345678;
const REVERSE_ENDIAN_CONSTANT: u32 = 0x78563412;

#[derive(Debug, Copy, Clone)]
pub enum EndianConstant {
	EndianConstant,
	ReverseEndianConstant,
}

impl TryFrom<u32> for EndianConstant {
	type Error = ParseError;

	fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
		match value {
			ENDIAN_CONSTANT => Ok(EndianConstant::EndianConstant),
			REVERSE_ENDIAN_CONSTANT => Ok(EndianConstant::ReverseEndianConstant),
			_ => Err(ParseError::GenericError("Not a valid endian constant".to_string()).into()),
		}
	}
}

#[derive(Debug, Copy, Clone)]
pub struct Header {
	pub format_version:  u32,
	pub checksum:        u32,
	pub signature:       [u8; 20],
	pub file_size:       u32,
	pub header_size:     u32,
	pub endian_tag:      EndianConstant,
	pub link_size:       u32,
	pub link_off:        u32,
	pub map_off:         u32,
	pub string_ids_size: u32,
	pub string_ids_off:  u32,
	pub type_ids_size:   u32,
	pub type_ids_off:    u32,
	pub proto_ids_size:  u32,
	pub proto_ids_off:   u32,
	pub field_ids_size:  u32,
	pub field_ids_off:   u32,
	pub method_ids_size: u32,
	pub method_ids_off:  u32,
	pub class_defs_size: u32,
	pub class_defs_off:  u32,
	pub data_size:       u32,
	pub data_off:        u32,
}

const DEX_FILE_MAGIC: [u8; 8] = [0x64, 0x65, 0x78, 0x0a, 0x00, 0x00, 0x00, 0x00];
//                                                       ^^^^^version^^^^^

impl Header {
	fn verify_header<P: Parser>(parser: &mut P) -> Result<u32> {
		let mut magic = [0; 8];
		parser.read_exact(&mut magic)?;
		if magic[..4] != DEX_FILE_MAGIC[..4] || magic[7] != DEX_FILE_MAGIC[7] {
			bail!(ParseError::generic("Magic doesn't match"));
		}

		let version_str =
			core::str::from_utf8(&magic[4..7]).wrap_err("converting version bytes to string")?;
		let version = version_str
			.parse::<u32>()
			.wrap_err("parsing version string")?;
		Ok(version)
	}

	#[cfg_attr(feature = "trace", instrument(skip(parser)))]
	pub fn parse<P: Parser>(parser: &mut P) -> Result<Self> {
		parser.align(4)?;

		let version = Self::verify_header(parser)?;

		Ok(Header {
			format_version:  version,
			checksum:        parser.u32()?,
			signature:       {
				let mut signature = [0; 20];
				parser.read_exact(&mut signature)?;
				signature
			},
			file_size:       parser.u32()?,
			header_size:     parser.u32()?,
			endian_tag:      parser.u32()?.try_into()?,
			link_size:       parser.u32()?,
			link_off:        parser.u32()?,
			map_off:         parser.u32()?,
			string_ids_size: parser.u32()?,
			string_ids_off:  parser.u32()?,
			type_ids_size:   parser.u32()?,
			type_ids_off:    parser.u32()?,
			proto_ids_size:  parser.u32()?,
			proto_ids_off:   parser.u32()?,
			field_ids_size:  parser.u32()?,
			field_ids_off:   parser.u32()?,
			method_ids_size: parser.u32()?,
			method_ids_off:  parser.u32()?,
			class_defs_size: parser.u32()?,
			class_defs_off:  parser.u32()?,
			data_size:       parser.u32()?,
			data_off:        parser.u32()?,
		})
	}
}

impl Parse for Header {
	fn parse<P: Parser>(parser: &mut P) -> Result<Self> {
		Header::parse(parser)
	}
}
