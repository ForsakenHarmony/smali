use crate::Resolver;
use crate::parser::id::{MethodIdItem, ClassDefItem, ProtoIdItem, ClassDataItem};

pub trait Resolve {
	type Item;
	fn resolve(item: &Self::Item, resolver: &Resolver) -> Self;
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Method {
	pub name: String,
	pub class: String,
	pub proto: Proto,
}

impl Resolve for Method {
	type Item = MethodIdItem;

	fn resolve(method_def: &Self::Item, resolver: &Resolver) -> Self {
		Method {
			class: method_def.class(resolver).descriptor(resolver),
			name: method_def.name(resolver),
			proto: Proto::resolve(&method_def.proto(resolver), resolver)
		}
	}
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Class {
	pub name: String,
	pub access_flags: u32,
	pub superclass: String,
	pub interfaces: Option<Vec<String>>,
	pub source_file: Option<String>,
	pub class_data: Option<ClassDataItem>,
}

impl Resolve for Class {
	type Item = ClassDefItem;

	fn resolve(item: &Self::Item, resolver: &Resolver) -> Self {
		Class {
			name: item.class_type(resolver).descriptor(resolver),
			access_flags: item.access_flags(resolver),
			superclass: item.superclass_type(resolver).descriptor(resolver),
			interfaces: item.interfaces(resolver).map(|l| l.list.iter().map(|i| resolver.dex_file.type_ids[*i as usize].descriptor(resolver)).collect()),
			source_file: item.source_file(resolver),
			class_data: item.class_data.clone()
		}
	}
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Proto {
	pub shorty_descriptor: String,
	pub return_type: String,
	pub parameters: Option<Vec<String>>,
}

impl Resolve for Proto {
	type Item = ProtoIdItem;

	fn resolve(item: &Self::Item, resolver: &Resolver) -> Self {
		Proto {
			shorty_descriptor: item.shorty(resolver),
			return_type: item.return_type(resolver).descriptor(resolver),
			parameters: item.parameters(resolver).map(|l| l.list.iter().map(|i| resolver.dex_file.type_ids[*i as usize].descriptor(resolver)).collect()),
		}
	}
}
