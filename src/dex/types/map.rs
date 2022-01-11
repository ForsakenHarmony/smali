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
use std::convert::{TryFrom, TryInto};

use eyre::{bail, eyre, Report, Result};

use crate::dex::parser::{Parse, Parser};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Map {
	pub header_item:                MapItem,
	pub string_id_item:             MapItem,
	pub type_id_item:               MapItem,
	pub proto_id_item:              MapItem,
	pub field_id_item:              MapItem,
	pub method_id_item:             MapItem,
	pub class_def_item:             MapItem,
	pub code_item:                  MapItem,
	pub debug_info_item:            MapItem,
	pub type_list:                  MapItem,
	pub string_data_item:           MapItem,
	pub annotation_item:            MapItem,
	pub class_data_item:            MapItem,
	pub encoded_array_item:         MapItem,
	pub annotation_set_item:        MapItem,
	pub annotation_set_ref_list:    MapItem,
	pub annotations_directory_item: MapItem,
	pub map_list:                   MapItem,
	pub call_site_id_item:          Option<MapItem>,
	pub method_handle_item:         Option<MapItem>,
	pub hiddenapi_class_data_item:  Option<MapItem>,
}

/// https://source.android.com/devices/tech/dalvik/dex-format#map-list
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MapList {
	pub size: u32,
	pub list: Vec<MapItem>,
}

impl MapList {
	pub fn map(&self) -> Result<Map> {
		Ok(Map {
			header_item:                self.item(TypeCode::HeaderItem)?,
			string_id_item:             self.item(TypeCode::StringIdItem)?,
			type_id_item:               self.item(TypeCode::TypeIdItem)?,
			proto_id_item:              self.item(TypeCode::ProtoIdItem)?,
			field_id_item:              self.item(TypeCode::FieldIdItem)?,
			method_id_item:             self.item(TypeCode::MethodIdItem)?,
			class_def_item:             self.item(TypeCode::ClassDefItem)?,
			code_item:                  self.item(TypeCode::CodeItem)?,
			debug_info_item:            self.item(TypeCode::DebugInfoItem)?,
			type_list:                  self.item(TypeCode::TypeList)?,
			string_data_item:           self.item(TypeCode::StringDataItem)?,
			annotation_item:            self.item(TypeCode::AnnotationItem)?,
			class_data_item:            self.item(TypeCode::ClassDataItem)?,
			encoded_array_item:         self.item(TypeCode::EncodedArrayItem)?,
			annotation_set_item:        self.item(TypeCode::AnnotationSetItem)?,
			annotation_set_ref_list:    self.item(TypeCode::AnnotationSetRefList)?,
			annotations_directory_item: self.item(TypeCode::AnnotationsDirectoryItem)?,
			map_list:                   self.item(TypeCode::MapList)?,
			call_site_id_item:          self.item(TypeCode::CallSiteIdItem).ok(),
			method_handle_item:         self.item(TypeCode::MethodHandleItem).ok(),
			hiddenapi_class_data_item:  self.item(TypeCode::HiddenapiClassDataItem).ok(),
		})
	}

	pub fn item(&self, typ: TypeCode) -> Result<MapItem> {
		self.list
			.iter()
			.find(|i| i.item_type == typ)
			.map(|i| *i)
			.ok_or_else(|| eyre!("could not find item with code: {:?}", typ))
	}
}

impl Parse for MapList {
	#[cfg_attr(feature = "trace", instrument(skip(parser)))]
	fn parse<P: Parser>(parser: &mut P) -> Result<Self> {
		parser.align(4)?;

		let size = parser.u32()?;
		let list = parser.parse_list(size)?;

		Ok(MapList { size, list })
	}
}

/// https://source.android.com/devices/tech/dalvik/dex-format#map-item
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MapItem {
	pub item_type: TypeCode,
	pub size:      u32,
	pub offset:    u32,
}

impl Parse for MapItem {
	#[cfg_attr(feature = "trace", instrument(skip(parser)))]
	fn parse<P: Parser>(parser: &mut P) -> Result<Self> {
		let item_type = parser.u16()?.try_into()?;
		let _unused = parser.u16()?;
		let size = parser.u32()?;
		let offset = parser.u32()?;

		Ok(MapItem {
			item_type,
			size,
			offset,
		})
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TypeCode {
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
	type Error = Report;

	fn try_from(value: u16) -> Result<Self> {
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
			_ => bail!("{:#x?} is not a valid Type Code", value),
		})
	}
}

// #[derive(Debug)]
// pub struct MapData {
// 	pub item_type: u16,
// 	pub size:      u32,
// 	pub offset:    u32,
// 	pub data:      Vec<u8>,
// }
// pub fn get_bytes_range<P: Parser>(parser: &mut P, start: u32, len: usize) -> Result<Vec<u8>> {
// 	parser.set_offset(start)?;
// 	let mut buffer = vec![0; len];
// 	parser.read_exact(&mut buffer)?;
// 	Ok(buffer)
// }

// impl Parse for MapData {
//    #[cfg_attr(feature = "trace", instrument(skip(parser)))]
// 	fn parse<P: Parser>(parser: &mut P) -> Result<Self> {
// 		Ok(MapData {
// 			item_type: map_item.item_type,
// 			size:      map_item.size,
// 			offset:    map_item.offset,
// 			data:      parser.parse_list_with_offset(len as usize, map_item.offset)?,
// 		})
// 	}
// }
