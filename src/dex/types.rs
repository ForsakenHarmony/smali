use eyre::Result;

use crate::dex::{
	resolver::{Resolve, ResolveFrom, ResolveInto},
	types::id::{
		ClassDataItem,
		ClassDefItem,
		CodeItem,
		EncodedField,
		EncodedMethod,
		FieldIdItem,
		MethodIdItem,
		ProtoIdItem,
	},
};

pub mod file;
pub mod header;

#[macro_use]
pub mod id;
pub mod map;
pub mod refs;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct MethodId {
	pub name:  String,
	pub class: String,
	pub proto: Proto,
}

impl ResolveFrom<MethodIdItem> for MethodId {
	fn resolve_from(item: &MethodIdItem, resolver: &impl Resolve) -> Result<Self> {
		Ok(MethodId {
			class: item.class(resolver)?.descriptor(resolver)?,
			name:  item.name(resolver)?,
			proto: item.proto(resolver)?.resolve_into(resolver)?,
		})
	}
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Method {
	pub id:           MethodId,
	pub access_flags: u32,
	pub code:         Option<CodeItem>,
}

impl ResolveFrom<EncodedMethod> for Method {
	fn resolve_from(item: &EncodedMethod, resolver: &impl Resolve) -> Result<Self> {
		Ok(Method {
			id:           item
				.method_idx_diff
				.resolve(resolver)?
				.resolve_into(resolver)?,
			access_flags: *item.access_flags,
			code:         None,
		})
	}
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct FieldId {
	pub class: usize,
	pub typ:   String,
	pub name:  String,
}

impl ResolveFrom<FieldIdItem> for FieldId {
	fn resolve_from(item: &FieldIdItem, resolver: &impl Resolve) -> Result<Self> {
		Ok(FieldId {
			class: *item.class_idx,
			typ:   item.type_idx.resolve(resolver)?.descriptor(resolver)?,
			name:  resolver.string(*item.name_idx),
		})
	}
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Field {
	pub id:           FieldId,
	pub access_flags: u32,
}

impl ResolveFrom<EncodedField> for Field {
	fn resolve_from(item: &EncodedField, resolver: &impl Resolve) -> Result<Self> {
		Ok(Field {
			id:           FieldId::resolve_from(
				&resolver.dex_file().field_ids[*item.field_idx_diff as usize],
				resolver,
			)?,
			access_flags: *item.access_flags,
		})
	}
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Class {
	pub name:         String,
	pub access_flags: u32,
	pub superclass:   String,
	pub interfaces:   Option<Vec<String>>,
	pub source_file:  Option<String>,
	pub class_data:   Option<ClassData>,
}

impl ResolveFrom<ClassDefItem> for Class {
	fn resolve_from(item: &ClassDefItem, resolver: &impl Resolve) -> Result<Self> {
		Ok(Class {
			name:         item.class_type(resolver)?.descriptor(resolver)?,
			access_flags: item.access_flags(resolver),
			superclass:   item.superclass_type(resolver)?.descriptor(resolver)?,
			interfaces:   None,
			// interfaces:   item.interfaces(resolver).map(|l| {
			// 	l.list
			// 		.iter()
			// 		.map(|i| resolver.dex_file.type_ids[i.type_idx as usize].descriptor(resolver))
			// 		.collect()
			// }),
			source_file:  item.source_file(resolver)?,
			class_data:   None,
			// class_data:   item
			// 	.class_data
			// 	.as_ref()
			// 	.map(|d| ClassData::resolve(d, resolver)),
		})
	}
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct ClassData {
	pub static_fields:   Vec<Field>,
	pub instance_fields: Vec<Field>,
	pub direct_methods:  Vec<Method>,
	pub virtual_methods: Vec<Method>,
}

impl ResolveFrom<ClassDataItem> for ClassData {
	fn resolve_from(item: &ClassDataItem, resolver: &impl Resolve) -> Result<Self> {
		Ok(ClassData {
			static_fields:   item
				.static_fields
				.iter()
				.map(|f| Field::resolve_from(f, resolver))
				.collect::<Result<Vec<_>>>()?,
			instance_fields: item
				.instance_fields
				.iter()
				.map(|f| Field::resolve_from(f, resolver))
				.collect::<Result<Vec<_>>>()?,
			direct_methods:  item
				.direct_methods
				.iter()
				.map(|m| Method::resolve_from(m, resolver))
				.collect::<Result<Vec<_>>>()?,
			virtual_methods: item
				.virtual_methods
				.iter()
				.map(|m| Method::resolve_from(m, resolver))
				.collect::<Result<Vec<_>>>()?,
		})
	}
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Proto {
	pub shorty_descriptor: String,
	pub return_type:       String,
	pub parameters:        Option<Vec<String>>,
}

impl ResolveFrom<ProtoIdItem> for Proto {
	fn resolve_from(item: &ProtoIdItem, resolver: &impl Resolve) -> Result<Self> {
		Ok(Proto {
			shorty_descriptor: item.shorty(resolver),
			return_type:       item.return_type(resolver).descriptor(resolver)?,
			parameters:        None,
			// parameters:        item.parameters(resolver).map(|l| {
			// 	l.list
			// 		.iter()
			// 		.map(|i| resolver.dex_file.type_ids[i.type_idx as usize].descriptor(resolver))
			// 		.collect()
			// }),
		})
	}
}
