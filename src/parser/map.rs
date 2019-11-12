// Shane Isbell licenses this file to you under the Apache License, Version 2.0
// (the "License"); you may not use this file except in compliance with the License.
//
// You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License. See the NOTICE file distributed with this work for
// additional information regarding copyright ownership.
use std::io::{Seek, SeekFrom, Read};

use crate::parser::{Parser, ParseResult, ParseError, ReadThings};
use std::convert::TryFrom;

#[derive(Debug)]
pub struct MapData {
	pub item_type: u16,
	pub size: u32,
	pub offset: u32,
	pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct MapItem {
	pub item_type: u16,
	pub unused: u16,
	pub size: u32,
	pub offset: u32,
}

#[derive(Debug)]
pub struct MapList {
	pub size: u32,
	pub list: Vec<MapItem>,
}

enum TypeCode {
	HeaderItem,
	StringIdItem,
	TypeIdItem,
	ProtoIdItem,
	FieldIdItem,
	MethodIdItem,
	ClassDefItem,
	CallSiteIdItem,
	MethodHandleItem,
	MapList,
	TypeList,
	AnnotationSetRefList,
	AnnotationSetItem,
	ClassDataItem,
	CodeItem,
	StringDataItem,
	DebugInfoItem,
	AnnotationItem,
	EncodedArrayItem,
	AnnotationsDirectoryItem,
	HiddenapiClassDataItem,
}

impl TryFrom<u16> for TypeCode {
	type Error = ParseError;

	fn try_from(value: u16) -> Result<Self, Self::Error> {
		Ok(match value {
			0x0000 => TypeCode::HeaderItem,
			0x0001 => TypeCode::StringIdItem,
			0x0002 => TypeCode::TypeIdItem,
			0x0003 => TypeCode::ProtoIdItem,
			0x0004 => TypeCode::FieldIdItem,
			0x0005 => TypeCode::MethodIdItem,
			0x0006 => TypeCode::ClassDefItem,
			0x0007 => TypeCode::CallSiteIdItem,
			0x0008 => TypeCode::MethodHandleItem,
			0x1000 => TypeCode::MapList,
			0x1001 => TypeCode::TypeList,
			0x1002 => TypeCode::AnnotationSetRefList,
			0x1003 => TypeCode::AnnotationSetItem,
			0x2000 => TypeCode::ClassDataItem,
			0x2001 => TypeCode::CodeItem,
			0x2002 => TypeCode::StringDataItem,
			0x2003 => TypeCode::DebugInfoItem,
			0x2004 => TypeCode::AnnotationItem,
			0x2005 => TypeCode::EncodedArrayItem,
			0x2006 => TypeCode::AnnotationsDirectoryItem,
			0xF000 => TypeCode::HiddenapiClassDataItem,
			_ => Err(ParseError::GenericError(format!("{:?} is not a valid Type Code", value)))?,
		})
	}
}

pub fn get_bytes_range(parser: &mut Parser, start: u32, len: usize) -> ParseResult<Vec<u8>> {
	parser.seek(SeekFrom::Start(start.into()))?;
	let mut buffer = vec![0; len];
	parser.read(&mut buffer)?;
	Ok(buffer)
}

impl MapData {
	pub fn parse(parser: &mut Parser, map_item: &MapItem, len: u32) -> ParseResult<MapData> {
		Ok(MapData {
			item_type: map_item.item_type,
			size: map_item.size,
			offset: map_item.offset,
			data: get_bytes_range(parser, map_item.offset, len as usize)?,
		})
	}
}

impl MapItem {
	pub fn parse(parser: &mut Parser) -> ParseResult<MapItem> {
		Ok(MapItem {
			item_type: parser.u16()?,
			unused: parser.u16()?,
			size: parser.u32()?,
			offset: parser.u32()?,
		})
	}
}

impl MapList {
	pub fn parse(parser: &mut Parser, offset: u32) -> ParseResult<MapList> {
		parser.seek(SeekFrom::Start(offset.into()))?;

		let size = parser.u32()?;
		let mut map_items = Vec::with_capacity(size as usize);
		for _x in 0..size {
			map_items.push(MapItem::parse(parser)?);
		}

		Ok(MapList {
			size,
			list: map_items,
		})
	}
}
