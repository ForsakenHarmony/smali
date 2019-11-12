use crate::parser::{Parse, ParseError, Parser, ReadThings};
use crate::Resolver;
use std::io::{Read};

const NO_INDEX: u32 = 0xffffffff;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct StringIdItem {
	pub string_data_off: u32,
}

impl Parse for StringIdItem {
	fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
//		debug!("Parsing StringIdItem");
		Ok(StringIdItem {
			string_data_off: parser.u32()?,
		})
	}
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct TypeIdItem {
	pub descriptor_idx: u32,
}

impl TypeIdItem {
	pub fn descriptor(&self, res: &Resolver) -> String {
		res.get_string(self.descriptor_idx as usize)
	}
}

impl Parse for TypeIdItem {
	fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
//		debug!("Parsing TypeIdItem");
		Ok(TypeIdItem {
			descriptor_idx: parser.u32()?,
		})
	}
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ProtoIdItem {
	pub shorty_idx: u32,
	pub return_type_idx: u32,
	pub parameters: Option<TypeList>,
}

impl ProtoIdItem {
	pub fn shorty(&self, res: &Resolver) -> String {
		res.get_string(self.shorty_idx as usize)
	}

	pub fn return_type(&self, res: &Resolver) -> TypeIdItem {
		res.dex_file.type_ids[self.return_type_idx as usize].clone()
	}

	pub fn parameters(&self, __res: &Resolver) -> Option<TypeList> {
		self.parameters.clone()
	}
}

impl Parse for ProtoIdItem {
	fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
//		debug!("Parsing ProtoIdItem");
		Ok(ProtoIdItem {
			shorty_idx: parser.u32()?,
			return_type_idx: parser.u32()?,
			parameters: TypeList::parse_with_offset(parser.u32()?, parser)?,
		})
	}
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct FieldIdItem {
	pub class_idx: u16,
	pub type_idx: u16,
	pub name_idx: u32,
}

impl Parse for FieldIdItem {
	fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
//		debug!("Parsing FieldIdItem");
		Ok(FieldIdItem {
			class_idx: parser.u16()?,
			type_idx: parser.u16()?,
			name_idx: parser.u32()?,
		})
	}
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct MethodIdItem {
	pub class_idx: u16,
	pub proto_idx: u16,
	pub name_idx: u32,
}

impl MethodIdItem {
	pub fn class(&self, res: &Resolver) -> TypeIdItem {
		res.dex_file.type_ids[self.class_idx as usize].clone()
	}

	pub fn proto(&self, res: &Resolver) -> ProtoIdItem {
		res.dex_file.proto_ids[self.proto_idx as usize].clone()
	}

	pub fn name(&self, res: &Resolver) -> String {
		res.get_string(self.name_idx as usize)
	}
}

impl Parse for MethodIdItem {
	fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
//		debug!("Parsing MethodIdItem");
		Ok(MethodIdItem {
			class_idx: parser.u16()?,
			proto_idx: parser.u16()?,
			name_idx: parser.u32()?,
		})
	}
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct CallSiteIdItem {
	pub call_site_off: u32,
}

impl Parse for CallSiteIdItem {
	fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
//		debug!("Parsing CallSiteIdItem");
		Ok(CallSiteIdItem {
			call_site_off: parser.u32()?,
		})
	}
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ClassDefItem {
	pub class_idx: u32,
	pub access_flags: u32,
	pub superclass_idx: u32,
	pub interfaces: Option<TypeList>,
	pub source_file_idx: u32,
	pub annotations: Option<AnnotationsDirectoryItem>,
	pub class_data: Option<ClassDataItem>,
	pub static_values: Option<EncodedArrayItem>,
}

impl ClassDefItem {
	pub fn class_type(&self, res: &Resolver) -> TypeIdItem {
		res.dex_file.type_ids[self.class_idx as usize].clone()
	}

	pub fn access_flags(&self, _res: &Resolver) -> u32 {
		self.access_flags
	}

	pub fn superclass_type(&self, res: &Resolver) -> TypeIdItem {
		res.dex_file.type_ids[self.superclass_idx as usize].clone()
	}

	pub fn interfaces(&self, _res: &Resolver) -> Option<TypeList> {
		self.interfaces.clone()
	}

	pub fn source_file(&self, res: &Resolver) -> Option<String> {
		if self.source_file_idx == NO_INDEX {
			return None;
		}
		Some(res.get_string(self.source_file_idx as usize))
	}
}

impl Parse for ClassDefItem {
	fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
//		debug!("Parsing ClassDefItem");
		Ok(ClassDefItem {
			class_idx: parser.u32()?,
			access_flags: parser.u32()?,
			superclass_idx: parser.u32()?,
			interfaces: TypeList::parse_with_offset(parser.u32()?, parser)?,
			source_file_idx: parser.u32()?,
			annotations: AnnotationsDirectoryItem::parse_with_offset(parser.u32()?, parser)?,
			class_data: ClassDataItem::parse_with_offset(parser.u32()?, parser)?,
			static_values: EncodedArrayItem::parse_with_offset(parser.u32()?, parser)?,
		})
	}
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct AnnotationsDirectoryItem {
//	pub class_annotations_off: u32,
//	pub fields_size: u32,
//	pub annotated_methods_size: u32,
//	pub annotated_parameters_size: u32,
//	pub field_annotations: u32,
//	pub method_annotations: u32,
//	pub parameter_annotations: u32,
}

impl Parse for AnnotationsDirectoryItem {
	fn parse(__parser: &mut Parser) -> Result<Self, ParseError> {
//		debug!("Parsing AnnotationsDirectoryItem");
		Ok(AnnotationsDirectoryItem {

		})
	}
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ClassDataItem {
	pub static_fields_size: u32,
	pub instance_fields_size: u32,
	pub direct_methods_size: u32,
	pub virtual_methods_size: u32,
	pub static_fields: Vec<EncodedField>,
	pub instance_fields: Vec<EncodedField>,
	pub direct_methods: Vec<EncodedMethod>,
	pub virtual_methods: Vec<EncodedMethod>,
}

impl Parse for ClassDataItem {
	fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
//		debug!("Parsing ClassDataItem");
		let static_fields_size = parser.r_uleb128()?.0 as u32;
		let instance_fields_size = parser.r_uleb128()?.0 as u32;
		let direct_methods_size = parser.r_uleb128()?.0 as u32;
		let virtual_methods_size = parser.r_uleb128()?.0 as u32;

//		println!("static {}, instance {}, direct {}, virtual {}", static_fields_size, instance_fields_size, direct_methods_size, virtual_methods_size);

		Ok(ClassDataItem {
			static_fields_size,
			instance_fields_size,
			direct_methods_size,
			virtual_methods_size,
			static_fields: EncodedField::parse_count(static_fields_size as u32, parser)?,
			instance_fields: EncodedField::parse_count(instance_fields_size as u32, parser)?,
			direct_methods: EncodedMethod::parse_count(direct_methods_size as u32, parser)?,
			virtual_methods: EncodedMethod::parse_count(virtual_methods_size as u32, parser)?,
		})
	}
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct EncodedField {
	pub field_idx_diff: u32,
	pub access_flags: u32,
}

impl Parse for EncodedField {
	fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
//		debug!("Parsing EncodedField");
		Ok(dbg!(EncodedField {
			field_idx_diff: parser.r_uleb128()?.0 as u32,
			access_flags: parser.r_uleb128()?.0 as u32,
		}))
	}
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct EncodedMethod {
	pub method_idx_diff: u32,
	pub access_flags: u32,
	pub code_off: u32,
	pub code: Option<CodeItem>,
}

impl Parse for EncodedMethod {
	fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
//		debug!("Parsing EncodedMethod");

		let method_idx_diff = parser.r_uleb128()?.0 as u32;
		let access_flags = parser.r_uleb128()?.0 as u32;
		let code_off = parser.r_uleb128()?.0 as u32;

//		if code_off < 1000000 {
//			dbg!(code_off);
//		}

		Ok(EncodedMethod {
			method_idx_diff,
			access_flags,
			code_off,
			code: CodeItem::parse_with_offset_in_data(dbg!(code_off), parser)?,
		})
	}
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct CodeItem {
	pub registers_size: u16,
	pub ins_size: u16,
	pub outs_size: u16,
	pub tries_size: u16,
	pub debug_info_off: u32,
	pub insns_size: u32,
	pub insns: Vec<u8>,
	pub padding: Option<u16>,
	pub tries: Option<Vec<()>>,
	pub handlers: Option<()>,
}

impl Parse for CodeItem {
	fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
		debug!("Parsing CodeItem");

		let registers_size = parser.u16()?;
		let ins_size = parser.u16()?;
		let outs_size = parser.u16()?;
		let tries_size = parser.u16()?;
		let debug_info_off = parser.u32()?;
		let insns_size = parser.u32()?;

		println!("registers: {}, ins: {}, outs: {}, tries: {}, debug_info_off: {}, insns: {}", registers_size, ins_size, outs_size, tries_size, debug_info_off, insns_size);

		let insns = {
			let mut vec = vec![0u8; (insns_size * 2) as usize];
			trace!("insns_size {}", insns_size);
			parser.read(&mut vec)?;
			vec
		};
		let (padding, tries, handlers) = if tries_size == 0 {
			(None, None, None)
		} else {
			// TODO
			(Some(parser.u16()?), None, None)
		};

		let ret = Ok(CodeItem {
			registers_size,
			ins_size,
			outs_size,
			tries_size,
			debug_info_off,
			insns_size,
			insns,
			padding,
			tries,
			handlers,
		});

		debug!("Parsing CodeItem done");

		ret
	}
}

// TODO
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct EncodedArrayItem {
}

impl Parse for EncodedArrayItem {
	fn parse(__parser: &mut Parser) -> Result<Self, ParseError> {
//		debug!("Parsing EncodedArrayItem");
		Ok(EncodedArrayItem {

		})
	}
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct MethodHandleItem {
	pub method_handle_type: u16,
	pub field_or_method_id: u16,
}

impl Parse for MethodHandleItem {
	fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
//		debug!("Parsing MethodHandleItem");
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

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct TypeList {
	pub size: u32,
	pub list: Vec<u16>,
}

impl Parse for TypeList {
	fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
//		debug!("Parsing TypeList");
		let size = parser.u32()?;
		let mut list = Vec::with_capacity(size as usize);
		for _ in 0..size {
			list.push(parser.u16()?);
		}
		Ok(TypeList {
			size,
			list
		})
	}
}
