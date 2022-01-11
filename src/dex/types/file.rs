use eyre::{Result, WrapErr};

use crate::dex::{
	parser::{Parse, Parser},
	types::{header::Header, id::*, map::MapList},
};

#[derive(Debug)]
pub struct DexFile {
	pub header:   Header,
	pub map_list: MapList,

	pub string_ids:               Vec<StringIdItem>,
	pub type_ids:                 Vec<TypeIdItem>,
	pub proto_ids:                Vec<ProtoIdItem>,
	pub field_ids:                Vec<FieldIdItem>,
	pub method_ids:               Vec<MethodIdItem>,
	pub class_defs:               Vec<ClassDefItem>,
	pub code:                     Vec<CodeItem>, // in map, not in header
	pub debug_info:               Vec<DebugInfoItem>, // in map, not in header
	pub type_lists:               Vec<TypeList>, // in map, not in header
	pub string_data:              Vec<StringDataItem>, // in map, not in header
	pub annotations:              Vec<AnnotationItem>, // in map, not in header
	pub class_data:               Vec<ClassDataItem>, // in map, not in header
	pub encoded_arrays:           Vec<EncodedArrayItem>, // in map, not in header
	pub annotation_sets:          Vec<AnnotationSetItem>, // in map, not in header
	pub annotation_set_ref_lists: Vec<AnnotationSetRefList>, // in map, not in header
	pub annotation_directories:   Vec<AnnotationsDirectoryItem>, // in map, not in header
	pub call_site_ids:            Vec<CallSiteIdItem>,
	pub method_handles:           Vec<MethodHandleItem>,

	pub data:      Vec<u8>,
	pub link_data: Vec<u8>,
}

impl Parse for DexFile {
	#[cfg_attr(feature = "trace", instrument(skip(parser)))]
	fn parse<P: Parser>(parser: &mut P) -> Result<Self> {
		let header: Header = parser.offset(0)?.parse()?;
		debug!("Header: {:#?}", header);
		let map_list: MapList = parser.offset(header.map_off)?.parse()?;

		let map = map_list.map()?;
		debug!("Map: {:#?}", map);

		macro_rules! parse_section {
			($item_name:ident, $ty:ty) => {{
				tracing::debug!(concat!("parsing ", stringify!($item_name)));
				let $item_name: Vec<$ty> = parser
					.offset(map.$item_name.offset)?
					.parse_list(map.$item_name.size)
					.wrap_err(concat!("parsing ", stringify!($item_name)))?;
				$item_name
			}};
			($item_name:ident, $item:expr, $ty:ty) => {{
				tracing::debug!(concat!("parsing ", stringify!($item_name)));
				let $item_name: Vec<$ty> = parser
					.offset($item.offset)?
					.parse_list($item.size)
					.wrap_err(concat!("parsing ", stringify!($item_name)))?;
				$item_name
			}};
		}

		let string_ids = parse_section!(string_id_item, StringIdItem);
		let string_data = string_ids
			.iter()
			.map(|id| parser.offset(*id.string_data_off).and_then(|p| p.parse()))
			.collect::<Result<_>>()?;

		let type_ids = parse_section!(type_id_item, TypeIdItem);
		let proto_ids = parse_section!(proto_id_item, ProtoIdItem);
		let field_ids = parse_section!(field_id_item, FieldIdItem);
		let method_ids = parse_section!(method_id_item, MethodIdItem);
		let class_defs = parse_section!(class_def_item, ClassDefItem);
		let code = parse_section!(code_item, CodeItem);
		// let debug_info = parse_section!(debug_info_item, DebugInfoItem);
		let debug_info = vec![];
		let type_lists = parse_section!(type_list, TypeList);
		let annotations = parse_section!(annotation_item, AnnotationItem);
		let class_data = parse_section!(class_data_item, ClassDataItem);
		let encoded_arrays = parse_section!(encoded_array_item, EncodedArrayItem);
		let annotation_sets = parse_section!(annotation_set_item, AnnotationSetItem);
		let annotation_set_ref_lists =
			parse_section!(annotation_set_ref_list, AnnotationSetRefList);
		let annotation_directories =
			parse_section!(annotations_directory_item, AnnotationsDirectoryItem);

		let call_site_ids = if let Some(item) = map.call_site_id_item {
			parse_section!(call_site_id_item, item, CallSiteIdItem)
		} else {
			vec![]
		};

		let method_handles = if let Some(item) = map.method_handle_item {
			parse_section!(method_handle_item, item, MethodHandleItem)
		} else {
			vec![]
		};

		Ok(DexFile {
			header,
			map_list,
			string_ids,
			type_ids,
			proto_ids,
			field_ids,
			method_ids,
			class_defs,
			code,
			debug_info,
			type_lists,
			string_data,
			annotations,
			class_data,
			encoded_arrays,
			annotation_sets,
			annotation_set_ref_lists,
			annotation_directories,
			call_site_ids,
			method_handles,
			data: vec![],
			link_data: vec![],
		})
	}
}
