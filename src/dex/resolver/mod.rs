use std::{
	cell::RefCell,
	collections::HashMap,
	io::{Read, Seek},
};

use eyre::{Result, WrapErr};

use crate::dex::{
	parser::{FileParser, Parser},
	types::{file::DexFile, Class, MethodId},
};

pub trait ResolveFrom<T>
where
	Self: Sized,
{
	// type From;
	// fn resolve_from(item: &Self::From, resolver: &impl Resolve) -> Result<Self>;
	fn resolve_from(item: &T, resolver: &impl Resolve) -> Result<Self>;
}

pub trait ResolveInto<T> {
	fn resolve_into(&self, resolver: &impl Resolve) -> Result<T>;
}

impl<T, U> ResolveInto<U> for T
where
	U: ResolveFrom<T>,
{
	fn resolve_into(&self, resolver: &impl Resolve) -> Result<U> {
		U::resolve_from(self, resolver)
	}
}

pub trait Resolve
where
	Self: Sized,
{
	fn dex_file(&self) -> &DexFile;

	fn resolve<F, T: ResolveFrom<F>>(&self, from: &F) -> Result<T> {
		T::resolve_from(from, self)
	}

	fn string(&self, idx: usize) -> String {
		self.dex_file().string_data[idx].string.clone()
	}
}

pub struct Resolver<P: Parser> {
	parser:       RefCell<P>,
	pub dex_file: DexFile,

	string_cache: RefCell<HashMap<usize, String>>,
}

impl<R: Read + Seek> Resolver<FileParser<R>> {
	#[cfg_attr(feature = "trace", instrument(skip(parser)))]
	pub fn new(mut parser: FileParser<R>) -> Result<Self> {
		let dex_file = parser.parse_file().wrap_err("parsing file")?;
		Ok(Self {
			parser: RefCell::new(parser),
			dex_file,

			string_cache: RefCell::new(HashMap::new()),
		})
	}
}

impl<P: Parser> Resolver<P> {
	// #[cfg_attr(feature = "trace", instrument(skip(parser)))]
	// pub fn get_string(&self, idx: usize) -> String {
	// 	let off = self.dex_file.string_ids[idx].string_data_off;
	//
	// 	let map: &mut HashMap<usize, String> = &mut *self.string_cache.borrow_mut();
	//
	// 	map.entry(idx)
	// 		.or_insert_with(|| self.parser.borrow_mut().parse_string(*off).unwrap())
	// 		.clone()
	// }

	#[cfg_attr(feature = "trace", instrument(skip(self)))]
	pub fn class_names(&mut self) -> Result<Vec<String>> {
		for class_def in self.dex_file.class_defs.iter() {
			let type_id = class_def.class_type(self)?;
			info!("Name {:?}", type_id.descriptor(self));
		}
		Ok(vec![])
	}

	#[cfg_attr(feature = "trace", instrument(skip(self)))]
	pub fn methods(&mut self) -> Result<Vec<MethodId>> {
		let mut methods = Vec::with_capacity(self.dex_file.method_ids.len());
		for method_def in self.dex_file.method_ids.iter() {
			methods.push(MethodId::resolve_from(method_def, self)?);
		}
		Ok(methods)
	}

	#[cfg_attr(feature = "trace", instrument(skip(self)))]
	pub fn classes(&mut self) -> Result<Vec<Class>> {
		let mut methods = Vec::with_capacity(self.dex_file.method_ids.len());
		for class_def in self.dex_file.class_defs[0..5].iter() {
			methods.push(Class::resolve_from(class_def, self)?);
		}
		Ok(methods)
	}
}

impl<P: Parser> Resolve for Resolver<P> {
	fn dex_file(&self) -> &DexFile {
		&self.dex_file
	}
}
