use std::{clone::Clone, marker::Copy, ops::Not};

use eyre::{bail, ensure, Result, WrapErr};

use crate::dex::{
	asm::instruction::Instruction,
	parser::{
		parse::{Sleb128, Uleb128},
		Parse,
		ParseError,
		Parser,
	},
	resolver::{Resolve, ResolveInto},
	types::{
		file::DexFile,
		refs::{IdItem, Idx, Ref},
	},
};

const NO_INDEX: usize = 0xffffffff;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct StringIdItem {
	pub string_data_off: Ref<StringDataItem, u32>,
}

parse_struct_default!(StringIdItem 4 { string_data_off });

impl IdItem for StringIdItem {
	type Output = StringDataItem;
	fn dex_section(dex_file: &DexFile) -> &[Self::Output] {
		&dex_file.string_data
	}
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct StringDataItem {
	pub size:   Uleb128,
	pub data:   Vec<u8>,
	pub string: String,
}

impl Parse for StringDataItem {
	#[cfg_attr(
		feature = "trace",
		instrument(skip(parser), name = "<StringDataItem as Parse>::parse")
	)]
	fn parse<P: Parser>(parser: &mut P) -> Result<Self> {
		let size = parser.uleb128()?;
		let (data, string) = parser.parse_string(*size)?;

		Ok(StringDataItem { size, data, string })
	}
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct TypeIdItem {
	pub descriptor_idx: Idx<StringIdItem, u32>,
}

parse_struct_default!(TypeIdItem 4 { descriptor_idx });

impl TypeIdItem {
	pub fn descriptor<R: Resolve>(&self, res: &R) -> Result<String> {
		let string_data: StringDataItem = self.descriptor_idx.resolve_into(res)?;
		Ok(string_data.string.clone())
		// self.descriptor_idx.resolve_into(res)?.string.clone()
		// let string_id: StringIdItem = res.resolve(&self.descriptor_idx)?;
		// res.dex_file().string_data[*self.descriptor_idx]
		// 	.string
		// 	.clone()
	}
}

impl IdItem for TypeIdItem {
	fn dex_section(dex_file: &DexFile) -> &[Self] {
		&dex_file.type_ids
	}
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ProtoIdItem {
	pub shorty_idx:      Idx<StringIdItem, u32>,
	pub return_type_idx: Idx<TypeIdItem, u32>,
	pub parameters:      Ref<Option<TypeList>, u32>,
}

parse_struct_default!(ProtoIdItem 4 {
	shorty_idx,
	return_type_idx,
	parameters
});

impl IdItem for ProtoIdItem {
	fn dex_section(dex_file: &DexFile) -> &[Self] {
		&dex_file.proto_ids
	}
}

impl ProtoIdItem {
	pub fn shorty<R: Resolve>(&self, res: &R) -> String {
		res.string(*self.shorty_idx)
	}

	pub fn return_type<R: Resolve>(&self, res: &R) -> TypeIdItem {
		res.dex_file().type_ids[*self.return_type_idx].clone()
	}

	// pub fn parameters<R: Read + Seek>(&self, __res: &Resolver<R>) -> Option<TypeList> {
	// 	self.parameters.clone()
	// }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct FieldIdItem {
	pub class_idx: Idx<TypeIdItem, u16>,
	pub type_idx:  Idx<TypeIdItem, u16>,
	pub name_idx:  Idx<StringIdItem, u32>,
}

parse_struct_default!(FieldIdItem 4 {
	class_idx,
	type_idx,
	name_idx
});

impl IdItem for FieldIdItem {
	fn dex_section(dex_file: &DexFile) -> &[Self] {
		&dex_file.field_ids
	}
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct MethodIdItem {
	pub class_idx: Idx<TypeIdItem, u16>,
	pub proto_idx: Idx<ProtoIdItem, u16>,
	pub name_idx:  Idx<StringIdItem, u32>,
}

parse_struct_default!(MethodIdItem 4 {
	class_idx,
	proto_idx,
	name_idx
});

impl IdItem for MethodIdItem {
	fn dex_section(dex_file: &DexFile) -> &[Self] {
		&dex_file.method_ids
	}
}

impl MethodIdItem {
	pub fn class(&self, res: &impl Resolve) -> Result<TypeIdItem> {
		self.class_idx.resolve_into(res)
	}

	pub fn proto(&self, res: &impl Resolve) -> Result<ProtoIdItem> {
		self.proto_idx.resolve_into(res)
	}

	pub fn name(&self, res: &impl Resolve) -> Result<String> {
		Ok(self.name_idx.resolve_into(res)?.string.clone())
	}
}

/// https://source.android.com/devices/tech/dalvik/dex-format#class-def-item
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct ClassDefItem {
	pub class_idx:         Idx<TypeIdItem, u32>,
	pub access_flags:      u32,
	pub superclass_idx:    Idx<TypeIdItem, u32>,
	pub interfaces_off:    Ref<Option<TypeList>, u32>,
	pub source_file_idx:   Idx<Option<StringIdItem>, u32>,
	pub annotations_off:   Ref<Option<AnnotationsDirectoryItem>, u32>,
	pub class_data_off:    Ref<Option<ClassDataItem>, u32>,
	pub static_values_off: Ref<Option<EncodedArrayItem>, u32>,
}

impl ClassDefItem {
	pub fn class_type(&self, res: &impl Resolve) -> Result<TypeIdItem> {
		self.class_idx.resolve_into(res)
		// res.dex_file().type_ids[*self.class_idx].clone()
	}

	pub fn access_flags(&self, _res: &impl Resolve) -> u32 {
		self.access_flags
	}

	pub fn superclass_type(&self, res: &impl Resolve) -> Result<TypeIdItem> {
		self.superclass_idx.resolve_into(res)
	}

	// pub fn interfaces<R: Read + Seek>(&self, _res: &Resolver<R>) -> Option<TypeList> {
	// 	self.interfaces.clone()
	// }

	pub fn source_file(&self, res: &impl Resolve) -> Result<Option<String>> {
		Ok(self.source_file_idx.resolve(res)?.map(|i| i.string.clone()))
		// if *self.source_file_idx == NO_INDEX {
		// 	return None;
		// }
		// Some(res.string(*self.source_file_idx))
	}
}

// impl Parse for ClassDefItem {
// 	#[cfg_attr(
// 		trace,
// 		instrument(skip(parser), name = "<ClassDefItem as Parse>::parse")
// 	)]
// 	fn parse<P: Parser>(parser: &mut P) -> Result<Self> {
// 		// let class_idx = parser.u32()?;
// 		// let access_flags = parser.u32()?;
// 		// let superclass_idx = parser.u32()?;
// 		// let interfaces_off = parser.u32()?;
// 		// let source_file_idx = parser.u32()?;
// 		// let annotations_off = parser.u32()?;
// 		// let class_data_off = parser.u32()?;
// 		// let static_values_off = parser.u32()?;
// 		// let interfaces = parser.parse_with_offset_in_data(interfaces_off)?;
// 		// let annotations = parser.parse_with_offset_in_data(annotations_off)?;
// 		// let class_data = parser.parse_with_offset_in_data(class_data_off)?;
// 		// let static_values = parser.parse_with_offset_in_data(static_values_off)?;
//
// 		Ok(ClassDefItem {
// 			class_idx:         parser.parse()?,
// 			access_flags:      parser.parse()?,
// 			superclass_idx:    parser.parse()?,
// 			interfaces_off:    parser.parse()?,
// 			source_file_idx:   parser.parse()?,
// 			annotations_off:   parser.parse()?,
// 			class_data_off:    parser.parse()?,
// 			static_values_off: parser.parse()?,
// 		})
// 	}
// }

parse_struct_default!(ClassDefItem 4 {
	class_idx,
	access_flags,
	superclass_idx,
	interfaces_off,
	source_file_idx,
	annotations_off,
	class_data_off,
	static_values_off,
});

/// https://source.android.com/devices/tech/dalvik/dex-format#call-site-id-item
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct CallSiteIdItem {
	pub call_site_off: Ref<CallSiteItem, u32>,
}

parse_struct_default!(CallSiteIdItem 4 { call_site_off });

impl IdItem for CallSiteIdItem {
	fn dex_section(dex_file: &DexFile) -> &[Self] {
		&dex_file.call_site_ids
	}
}

/// https://source.android.com/devices/tech/dalvik/dex-format#call-site-item
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct CallSiteItem {
	pub arr: EncodedArrayItem,
}

// TODO: check if we need to do any special handling/validation for the array
parse_struct_default!(CallSiteItem { arr });

/// https://source.android.com/devices/tech/dalvik/dex-format#method-handle-item
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct MethodHandleItem {
	pub method_handle_type: u16,
	pub field_or_method_id: u16,
}

impl Parse for MethodHandleItem {
	#[cfg_attr(
		feature = "trace",
		instrument(skip(parser), name = "<MethodHandleItem as Parse>::parse")
	)]
	fn parse<P: Parser>(parser: &mut P) -> Result<Self> {
		parser.align(4)?;

		let method_handle_type = parser.u16()?;
		parser.u16()?; // unused
		let field_or_method_id = parser.u16()?;
		parser.u16()?; // unused

		Ok(MethodHandleItem {
			method_handle_type,
			field_or_method_id,
		})
	}
}

/// https://source.android.com/devices/tech/dalvik/dex-format#method-handle-type-codes
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum MethodHandleType {
	// Method handle is a static field setter (accessor)
	MethodHandleTypeStaticPut = 0x00,
	/// Method handle is a static field getter (accessor)
	MethodHandleTypeStaticGet = 0x01,
	/// Method handle is an instance field setter (accessor)
	MethodHandleTypeInstancePut = 0x02,
	/// Method handle is an instance field getter (accessor)
	MethodHandleTypeInstanceGet = 0x03,
	/// Method handle is a static method invoker
	MethodHandleTypeInvokeStatic = 0x04,
	/// Method handle is an instance method invoker
	MethodHandleTypeInvokeInstance = 0x05,
	/// Method handle is a constructor method invoker
	MethodHandleTypeInvokeConstructor = 0x06,
	/// Method handle is a direct method invoker
	MethodHandleTypeInvokeDirect = 0x07,
	/// Method handle is an interface method invoker
	MethodHandleTypeInvokeInterface = 0x08,
}

impl Parse for MethodHandleType {
	#[cfg_attr(
		feature = "trace",
		instrument(skip(parser), name = "<MethodHandleType as Parse>::parse")
	)]
	fn parse<P: Parser>(parser: &mut P) -> Result<Self> {
		Ok(match parser.u16()? {
			0x00 => MethodHandleType::MethodHandleTypeStaticPut,
			0x01 => MethodHandleType::MethodHandleTypeStaticGet,
			0x02 => MethodHandleType::MethodHandleTypeInstancePut,
			0x03 => MethodHandleType::MethodHandleTypeInstanceGet,
			0x04 => MethodHandleType::MethodHandleTypeInvokeStatic,
			0x05 => MethodHandleType::MethodHandleTypeInvokeInstance,
			0x06 => MethodHandleType::MethodHandleTypeInvokeConstructor,
			0x07 => MethodHandleType::MethodHandleTypeInvokeDirect,
			0x08 => MethodHandleType::MethodHandleTypeInvokeInterface,
			v => {
				return Err(
					ParseError::generic(format!("{} is not a valid MethodHandleType", v)).into(),
				)
			}
		})
	}
}

/// https://source.android.com/devices/tech/dalvik/dex-format#class-data-item
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ClassDataItem {
	pub static_fields_size:   Uleb128,
	pub instance_fields_size: Uleb128,
	pub direct_methods_size:  Uleb128,
	pub virtual_methods_size: Uleb128,
	pub static_fields:        Vec<EncodedField>,
	pub instance_fields:      Vec<EncodedField>,
	pub direct_methods:       Vec<EncodedMethod>,
	pub virtual_methods:      Vec<EncodedMethod>,
}

impl Parse for ClassDataItem {
	#[cfg_attr(
		feature = "trace",
		instrument(skip(parser), name = "<ClassDataItem as Parse>::parse")
	)]
	fn parse<P: Parser>(parser: &mut P) -> Result<Self> {
		let static_fields_size = parser.uleb128()?;
		let instance_fields_size = parser.uleb128()?;
		let direct_methods_size = parser.uleb128()?;
		let virtual_methods_size = parser.uleb128()?;

		Ok(ClassDataItem {
			static_fields_size,
			instance_fields_size,
			direct_methods_size,
			virtual_methods_size,
			static_fields: parser.parse_list(*static_fields_size)?,
			instance_fields: parser.parse_list(*instance_fields_size)?,
			direct_methods: parser.parse_list(*direct_methods_size)?,
			virtual_methods: parser.parse_list(*virtual_methods_size)?,
		})
	}
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct EncodedField {
	pub field_idx_diff: Uleb128,
	pub access_flags:   Uleb128,
}

parse_struct_default!(EncodedField {
	field_idx_diff,
	access_flags,
});

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct EncodedMethod {
	pub method_idx_diff: Idx<MethodIdItem, Uleb128>,
	pub access_flags:    Uleb128,
	pub code_off:        Ref<CodeItem, Uleb128>,
}

parse_struct_default!(EncodedMethod {
	method_idx_diff,
	access_flags,
	code_off,
});

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct TypeList {
	pub size: u32,
	pub list: Vec<TypeItem>,
}

impl Parse for TypeList {
	#[cfg_attr(
		feature = "trace",
		instrument(skip(parser), name = "<TypeList as Parse>::parse")
	)]
	fn parse<P: Parser>(parser: &mut P) -> Result<Self> {
		parser.align(4)?;

		let size = parser.u32()?;
		let list = parser.parse_list(size)?;
		Ok(TypeList { size, list })
	}
}

/// https://source.android.com/devices/tech/dalvik/dex-format#type-item-format
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct TypeItem {
	pub type_idx: Idx<TypeIdItem, u16>,
}

parse_struct_default!(TypeItem { type_idx });

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct CodeItem {
	pub registers_size: u16,
	pub ins_size:       u16,
	pub outs_size:      u16,
	pub tries_size:     u16,
	pub debug_info_off: Ref<DebugInfoItem, u32>,
	pub insns:          Vec<Instruction>,
	pub padding:        Option<u16>,
	pub tries:          Option<Vec<TryItem>>,
	pub handlers:       Option<EncodedCatchHandlerList>,
}

impl Parse for CodeItem {
	#[cfg_attr(
		feature = "trace",
		instrument(skip(parser), name = "<CodeItem as Parse>::parse")
	)]
	fn parse<P: Parser>(parser: &mut P) -> Result<Self> {
		parser.align(4)?;

		let registers_size = parser.u16()?;
		let ins_size = parser.u16()?;
		let outs_size = parser.u16()?;
		let tries_size = parser.u16()?;
		let debug_info_off = parser.parse()?;
		let insns_size = parser.u32()?;

		// trace!(
		// 	registers_size = tracing::field::debug(&registers_size),
		// 	ins_size = tracing::field::debug(&ins_size),
		// 	outs_size = tracing::field::debug(&outs_size),
		// 	tries_size = tracing::field::debug(&tries_size),
		// 	debug_info_off = tracing::field::debug(&debug_info_off),
		// 	insns_size = tracing::field::debug(&insns_size),
		// 	"code item vals"
		// );

		let insns = {
			let start_offset = parser.get_offset();
			let mut vec = vec![0u8; (insns_size * 2) as usize];
			parser.read(&mut vec)?;
			// trace!(offset = start_pos, "raw instructions: {:#04x?}", vec);

			let mut instructions = Vec::new();
			parser.set_offset(start_offset)?;
			while parser.get_offset() < start_offset + insns_size * 2 {
				let i = match Instruction::parse(parser).wrap_err("parsing instruction") {
					Ok(i) => i,
					Err(e) => {
						// std::io::stdout();
						error!(
							// instructions = format!("{:?}", instructions).as_str(),
							// raw_instructions = format!("{:x?}", vec).as_str(),
							offset = parser.get_offset(),
							"failed to parse instruction: {:#}",
							e
						);
						return Err(e);
					}
				};
				// trace!(offset = parser.get_pos(), "parsed: {:x?}", i);
				instructions.push(i);
			}

			instructions
		};

		let (padding, tries, handlers) = if tries_size != 0 {
			let padding = if insns_size % 2 != 0 {
				Some(parser.u16()?)
			} else {
				None
			};

			let tries = parser.parse_list(tries_size as u32)?;
			let handlers = parser.parse()?;

			(padding, Some(tries), Some(handlers))
		} else {
			(None, None, None)
		};

		let offset = parser.get_offset() % 4;
		if offset > 0 {
			for _ in 0..(4 - offset) {
				parser.u8()?;
			}
		}

		let ret = Ok(CodeItem {
			registers_size,
			ins_size,
			outs_size,
			tries_size,
			debug_info_off,
			insns,
			padding,
			tries,
			handlers,
		});

		ret
	}
}

/// https://source.android.com/devices/tech/dalvik/dex-format#type-item
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct TryItem {
	pub start_addr:  u32,
	pub insn_count:  u16,
	pub handler_off: u16,
}

parse_struct_default!(TryItem {
	start_addr,
	insn_count,
	handler_off,
});

/// https://source.android.com/devices/tech/dalvik/dex-format#encoded-catch-handlerlist
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct EncodedCatchHandlerList {
	pub size: Uleb128,
	pub list: Vec<EncodedCatchHandler>,
}

impl Parse for EncodedCatchHandlerList {
	#[cfg_attr(
		feature = "trace",
		instrument(skip(parser), name = "<EncodedCatchHandlerList as Parse>::parse")
	)]
	fn parse<P: Parser>(parser: &mut P) -> Result<Self> {
		let size = parser.uleb128()?;
		let list = parser.parse_list(*size)?;
		Ok(EncodedCatchHandlerList { size, list })
	}
}

/// https://source.android.com/devices/tech/dalvik/dex-format#encoded-catch-handler
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct EncodedCatchHandler {
	pub size:           Sleb128,
	pub handlers:       Vec<EncodedTypeAddrPair>,
	pub catch_all_addr: Option<Uleb128>,
}

impl Parse for EncodedCatchHandler {
	#[cfg_attr(
		feature = "trace",
		instrument(skip(parser), name = "<EncodedCatchHandler as Parse>::parse")
	)]
	fn parse<P: Parser>(parser: &mut P) -> Result<Self> {
		let size = parser.sleb128()?;
		let handlers = parser.parse_list(size.abs() as u32)?;
		let catch_all_addr = if !size.is_positive() {
			Some(parser.uleb128()?)
		} else {
			None
		};

		Ok(EncodedCatchHandler {
			size,
			handlers,
			catch_all_addr,
		})
	}
}

/// https://source.android.com/devices/tech/dalvik/dex-format#encoded-type-addr-pair
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct EncodedTypeAddrPair {
	pub type_idx: Uleb128,
	pub addr:     Uleb128,
}

parse_struct_default!(EncodedTypeAddrPair { type_idx, addr });

/// https://source.android.com/devices/tech/dalvik/dex-format#debug-info-item
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct DebugInfoItem {
	pub line_start:      Uleb128,
	pub parameters_size: Uleb128,
	pub parameter_names: Vec<Uleb128>,
}

impl Parse for DebugInfoItem {
	#[cfg_attr(
		feature = "trace",
		instrument(skip(parser), name = "<DebugInfoItem as Parse>::parse")
	)]
	fn parse<P: Parser>(parser: &mut P) -> Result<Self> {
		let line_start = parser.uleb128()?;
		let parameters_size = parser.uleb128()?;
		let parameter_names = parser.parse_list(*parameters_size)?;

		Ok(DebugInfoItem {
			line_start,
			parameters_size,
			parameter_names,
		})
	}
}

/// https://source.android.com/devices/tech/dalvik/dex-format#annotations-directory
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct AnnotationsDirectoryItem {
	pub class_annotations_off:     Ref<AnnotationSetItem, u32>,
	pub fields_size:               u32,
	pub annotated_methods_size:    u32,
	pub annotated_parameters_size: u32,
	pub field_annotations:         Option<Vec<FieldAnnotation>>,
	pub method_annotations:        Option<Vec<MethodAnnotation>>,
	pub parameter_annotations:     Option<Vec<ParameterAnnotation>>,
}

impl Parse for AnnotationsDirectoryItem {
	#[cfg_attr(
		feature = "trace",
		instrument(skip(parser), name = "<AnnotationsDirectoryItem as Parse>::parse")
	)]
	fn parse<P: Parser>(parser: &mut P) -> Result<Self> {
		parser.align(4)?;

		let class_annotations_off = parser.parse()?;
		let fields_size = parser.u32()?;
		let annotated_methods_size = parser.u32()?;
		let annotated_parameters_size = parser.u32()?;
		let field_annotations = (fields_size != 0)
			.not()
			.then(|| parser.parse_list(fields_size))
			.transpose()?;
		let method_annotations = (annotated_methods_size != 0)
			.not()
			.then(|| parser.parse_list(annotated_methods_size))
			.transpose()?;
		let parameter_annotations = (annotated_parameters_size != 0)
			.not()
			.then(|| parser.parse_list(annotated_parameters_size))
			.transpose()?;

		Ok(AnnotationsDirectoryItem {
			class_annotations_off,
			fields_size,
			annotated_methods_size,
			annotated_parameters_size,
			field_annotations,
			method_annotations,
			parameter_annotations,
		})
	}
}

/// https://source.android.com/devices/tech/dalvik/dex-format#field-annotation
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct FieldAnnotation {
	pub field_idx:       Idx<FieldIdItem, u32>,
	pub annotations_off: Ref<AnnotationSetItem, u32>,
}

parse_struct_default!(FieldAnnotation {
	field_idx,
	annotations_off,
});

/// https://source.android.com/devices/tech/dalvik/dex-format#method-annotation
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct MethodAnnotation {
	pub method_idx:      Idx<MethodIdItem, u32>,
	pub annotations_off: Ref<AnnotationSetItem, u32>,
}
parse_struct_default!(MethodAnnotation {
	method_idx,
	annotations_off,
});

/// https://source.android.com/devices/tech/dalvik/dex-format#parameter-annotation
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct ParameterAnnotation {
	pub method_idx:      Idx<MethodIdItem, u32>,
	pub annotations_off: Ref<AnnotationSetItem, u32>,
}

parse_struct_default!(ParameterAnnotation {
	method_idx,
	annotations_off,
});

/// https://source.android.com/devices/tech/dalvik/dex-format#set-ref-list
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct AnnotationSetRefList {
	pub size: u32,
	pub list: Vec<AnnotationSetRefItem>,
}

impl Parse for AnnotationSetRefList {
	#[cfg_attr(
		feature = "trace",
		instrument(skip(parser), name = "<AnnotationSetRefList as Parse>::parse")
	)]
	fn parse<P: Parser>(parser: &mut P) -> Result<Self> {
		parser.align(4)?;

		let size = parser.u32()?;
		let list = parser.parse_list(size)?;
		Ok(AnnotationSetRefList { size, list })
	}
}

/// https://source.android.com/devices/tech/dalvik/dex-format#set-ref-item
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct AnnotationSetRefItem {
	pub annotations_off: Ref<AnnotationSetItem, u32>,
}

parse_struct_default!(AnnotationSetRefItem { annotations_off });

/// https://source.android.com/devices/tech/dalvik/dex-format#annotation-set-item
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct AnnotationSetItem {
	pub size:    u32,
	pub entries: Vec<AnnotationOffItem>,
}

impl Parse for AnnotationSetItem {
	#[cfg_attr(
		feature = "trace",
		instrument(skip(parser), name = "<AnnotationSetItem as Parse>::parse")
	)]
	fn parse<P: Parser>(parser: &mut P) -> Result<Self> {
		parser.align(4)?;

		let size = parser.u32()?;
		let entries = parser.parse_list(size)?;
		Ok(AnnotationSetItem { size, entries })
	}
}

/// https://source.android.com/devices/tech/dalvik/dex-format#off-item
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct AnnotationOffItem {
	pub annotations_off: Ref<AnnotationSetItem, u32>,
}

parse_struct_default!(AnnotationOffItem { annotations_off });

/// https://source.android.com/devices/tech/dalvik/dex-format#annotation-item
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct AnnotationItem {
	pub visibility: u8,
	pub annotation: EncodedAnnotation,
}

parse_struct_default!(AnnotationItem {
	visibility,
	annotation,
});

// macro_rules! parsed_struct {
//     (
//         [$($attrs_pub:tt)*]
//         struct $name:ident $($rest:tt)*
//     ) => {
//         $($attrs_pub)* struct $name $($rest)*
//
//         #[cfg(not(feature = "full"))]
//         $($attrs_pub)* struct $name {
//             _noconstruct: ::std::marker::PhantomData<::proc_macro2::Span>,
//         }
//
//         #[cfg(all(not(feature = "full"), feature = "printing"))]
//         impl ::quote::ToTokens for $name {
//             fn to_tokens(&self, _: &mut ::proc_macro2::TokenStream) {
//                 unreachable!()
//             }
//         }
//     };
//
//     (
//         [$($attrs_pub:tt)*]
//         struct $name:ident $($rest:tt)*
//     ) => {
//         $($attrs_pub)* struct $name $($rest)*
//     };
//
//     ($($t:tt)*) => {
//         strip_attrs_pub!(parsed_struct!($($t)*));
//     };
// }
//
// macro_rules! strip_attrs_pub {
//     ($mac:ident!($(#[$m:meta])* $pub:ident $($t:tt)*)) => {
//         check_keyword_matches!(pub $pub);
//
//         $mac!([$(#[$m])* $pub] $($t)*);
//     };
// }
//
// macro_rules! check_keyword_matches {
// 	(struct struct) => {};
// 	(enum enum) => {};
// 	(pub pub) => {};
// }
//
// parsed_struct! {
// 	/// https://source.android.com/devices/tech/dalvik/dex-format#annotation-item
// 	#[derive(Debug, Clone, PartialOrd, PartialEq)]
// 	struct AnnotationItem {
// 		pub visibility: u8,
// 		pub annotation: EncodedAnnotation,
// 	}
// }

/// https://source.android.com/devices/tech/dalvik/dex-format#encoded-array-item
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct EncodedArrayItem {
	pub value: EncodedArray,
}

parse_struct_default!(EncodedArrayItem { value });

/// https://source.android.com/devices/tech/dalvik/dex-format#hiddenapi-class-data-item
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct HiddenapiClassDataItem {
	pub size:    u32,
	pub offsets: Vec<u8>,
	pub flags:   Vec<Uleb128>,
}

impl Parse for HiddenapiClassDataItem {
	#[cfg_attr(
		feature = "trace",
		instrument(skip(parser), name = "<HiddenapiClassDataItem as Parse>::parse")
	)]
	fn parse<P: Parser>(parser: &mut P) -> Result<Self> {
		let size = parser.u32()?;
		// TODO
		let offsets = vec![];
		let flags = vec![];

		Ok(HiddenapiClassDataItem {
			size,
			offsets,
			flags,
		})
	}
}

// #[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
// pub enum EncodedValueType {
// 	Byte         = 0x00,
// 	Short        = 0x02,
// 	Char         = 0x03,
// 	Int          = 0x04,
// 	Long         = 0x06,
// 	Float        = 0x10,
// 	Double       = 0x11,
// 	MethodType   = 0x15,
// 	MethodHandle = 0x16,
// 	String       = 0x17,
// 	Type         = 0x18,
// 	Field        = 0x19,
// 	Method       = 0x1a,
// 	Enum         = 0x1b,
// 	Array        = 0x1c,
// 	Annotation   = 0x1d,
// 	Null         = 0x1e,
// 	Boolean      = 0x1f,
// }

/// https://source.android.com/devices/tech/dalvik/dex-format#encoding
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum EncodedValue {
	Byte(u8),
	Short(i16),
	Char(u16),
	Int(i32),
	Long(i64),
	Float(f32),
	Double(f64),
	/// index into the `proto_ids` section
	MethodType(u32),
	/// index into the `method_handles` section
	MethodHandle(u32),
	/// index into the `string_ids` section
	String(u32),
	/// index into the `type_ids` section
	Type(u32),
	/// index into the `field_ids` section
	Field(u32),
	/// index into the `method_ids` section
	Method(u32),
	/// index into the `field_ids` section
	Enum(u32),
	Array(EncodedArray),
	Annotation(EncodedAnnotation),
	Null,
	Boolean(bool),
}

impl Parse for EncodedValue {
	#[cfg_attr(
		feature = "trace",
		instrument(skip(parser), name = "<EncodedValue as Parse>::parse")
	)]
	fn parse<P: Parser>(parser: &mut P) -> Result<Self> {
		let val = parser.u8()?;
		let value_type = val & 0b0001_1111;
		let value_arg = ((val & 0b1110_0000) >> 5) as usize;

		Ok(match value_type {
			// byte
			0x00 => {
				ensure!(
					value_arg == 0,
					"byte -> value_arg must be 0, got {}",
					value_arg
				);
				EncodedValue::Byte(parser.u8()?)
			}
			// short
			0x02 => {
				ensure!(
					(0..=1).contains(&value_arg),
					"short -> value_arg must be 0..=1, got {}",
					value_arg
				);

				let mut bytes = [0; 2];
				parser.read_exact(&mut bytes[0..value_arg + 1])?;
				EncodedValue::Short(i16::from_le_bytes(bytes))
			}
			// char
			0x03 => {
				ensure!(
					(0..=1).contains(&value_arg),
					"char -> value_arg must be 0..=1, got {}",
					value_arg
				);

				let mut bytes = [0; 2];
				parser.read_exact(&mut bytes[0..value_arg + 1])?;
				EncodedValue::Char(u16::from_le_bytes(bytes))
			}
			// int
			0x04 => {
				ensure!(
					(0..=3).contains(&value_arg),
					"int -> value_arg must be 0..=3, got {}",
					value_arg
				);

				let mut bytes = [0; 4];
				parser.read_exact(&mut bytes[0..value_arg + 1])?;
				EncodedValue::Int(i32::from_le_bytes(bytes))
			}
			// long
			0x06 => {
				ensure!(
					(0..=7).contains(&value_arg),
					"long -> value_arg must be 0..=7, got {}",
					value_arg
				);

				let mut bytes = [0; 8];
				parser.read_exact(&mut bytes[0..value_arg + 1])?;
				EncodedValue::Long(i64::from_le_bytes(bytes))
			}
			// float
			0x10 => {
				ensure!(
					(0..=3).contains(&value_arg),
					"float -> value_arg must be 0..=3, got {}",
					value_arg
				);

				let mut bytes = [0; 4];
				parser.read_exact(&mut bytes[0..value_arg + 1])?;
				EncodedValue::Float(f32::from_le_bytes(bytes))
			}
			// double
			0x11 => {
				ensure!(
					(0..=7).contains(&value_arg),
					"double -> value_arg must be 0..=7, got {}",
					value_arg
				);

				let mut bytes = [0; 8];
				parser.read_exact(&mut bytes[0..value_arg + 1])?;
				EncodedValue::Double(f64::from_le_bytes(bytes))
			}
			// method type
			0x15 => {
				ensure!(
					(0..=3).contains(&value_arg),
					"method type -> value_arg must be 0..=3, got {}",
					value_arg
				);

				let mut bytes = [0; 4];
				parser.read_exact(&mut bytes[0..value_arg + 1])?;
				EncodedValue::MethodType(u32::from_le_bytes(bytes))
			}
			// method handle
			0x16 => {
				ensure!(
					(0..=3).contains(&value_arg),
					"method handle -> value_arg must be 0..=3, got {}",
					value_arg
				);

				let mut bytes = [0; 4];
				parser.read_exact(&mut bytes[0..value_arg + 1])?;
				EncodedValue::MethodHandle(u32::from_le_bytes(bytes))
			}
			// string
			0x17 => {
				ensure!(
					(0..=3).contains(&value_arg),
					"string -> value_arg must be 0..=3, got {}",
					value_arg
				);

				let mut bytes = [0; 4];
				parser.read_exact(&mut bytes[0..value_arg + 1])?;
				EncodedValue::String(u32::from_le_bytes(bytes))
			}
			// type
			0x18 => {
				ensure!(
					(0..=3).contains(&value_arg),
					"type -> value_arg must be 0..=3, got {}",
					value_arg
				);

				let mut bytes = [0; 4];
				parser.read_exact(&mut bytes[0..value_arg + 1])?;
				EncodedValue::Type(u32::from_le_bytes(bytes))
			}
			// field
			0x19 => {
				ensure!(
					(0..=3).contains(&value_arg),
					"field -> value_arg must be 0..=3, got {}",
					value_arg
				);

				let mut bytes = [0; 4];
				parser.read_exact(&mut bytes[0..value_arg + 1])?;
				EncodedValue::Field(u32::from_le_bytes(bytes))
			}
			// method
			0x1a => {
				ensure!(
					(0..=3).contains(&value_arg),
					"method -> value_arg must be 0..=3, got {}",
					value_arg
				);

				let mut bytes = [0; 4];
				parser.read_exact(&mut bytes[0..value_arg + 1])?;
				EncodedValue::Method(u32::from_le_bytes(bytes))
			}
			// enum
			0x1b => {
				ensure!(
					(0..=3).contains(&value_arg),
					"enum -> value_arg must be 0..=3, got {}",
					value_arg
				);

				let mut bytes = [0; 4];
				parser.read_exact(&mut bytes[0..value_arg + 1])?;
				EncodedValue::Enum(u32::from_le_bytes(bytes))
			}
			// array
			0x1c => {
				ensure!(
					value_arg == 0,
					"array -> value_arg must be 0, got {}",
					value_arg
				);
				EncodedValue::Array(parser.parse()?)
			}
			// annotation
			0x1d => {
				ensure!(
					value_arg == 0,
					"annotation -> value_arg must be 0, got {}",
					value_arg
				);
				EncodedValue::Annotation(parser.parse()?)
			}
			// null
			0x1e => {
				ensure!(
					value_arg == 0,
					"null -> value_arg must be 0, got {}",
					value_arg
				);
				EncodedValue::Null
			}
			// boolean
			0x1f => {
				ensure!(
					(0..=1).contains(&value_arg),
					"boolean -> value_arg must be 0..=1, got {}",
					value_arg
				);
				EncodedValue::Boolean(value_arg == 1)
			}
			_ => bail!("invalid value_type: {:x?}", value_type),
		})
	}
}

/// https://source.android.com/devices/tech/dalvik/dex-format#encoded-array
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct EncodedArray {
	pub size:   Uleb128,
	pub values: Vec<EncodedValue>,
}

impl Parse for EncodedArray {
	#[cfg_attr(
		feature = "trace",
		instrument(skip(parser), name = "<EncodedArray as Parse>::parse")
	)]
	fn parse<P: Parser>(parser: &mut P) -> Result<Self> {
		let size = parser.uleb128()?;
		let values = parser.parse_list(*size)?;

		Ok(EncodedArray { size, values })
	}
}

/// https://source.android.com/devices/tech/dalvik/dex-format#encoded-annotation
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct EncodedAnnotation {
	pub type_idx: Uleb128,
	pub size:     Uleb128,
	pub elements: Vec<AnnotationElement>,
}

impl Parse for EncodedAnnotation {
	#[cfg_attr(
		feature = "trace",
		instrument(skip(parser), name = "<EncodedAnnotation as Parse>::parse")
	)]
	fn parse<P: Parser>(parser: &mut P) -> Result<Self> {
		let type_idx = parser.uleb128()?;
		let size = parser.uleb128()?;
		let elements = parser.parse_list(*size)?;

		Ok(EncodedAnnotation {
			type_idx,
			size,
			elements,
		})
	}
}

/// https://source.android.com/devices/tech/dalvik/dex-format#annotation-element
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct AnnotationElement {
	pub name_idx: Idx<StringIdItem, Uleb128>,
	pub value:    EncodedValue,
}

parse_struct_default!(AnnotationElement { name_idx, value });
