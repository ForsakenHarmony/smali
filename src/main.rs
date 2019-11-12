// FIXME
#![allow(dead_code)]

#[macro_use] extern crate log;

use std::fs::File;
use std::io::BufReader;
use std::io;
use crate::parser::{Parser, DexFile, ParseResult};
use std::collections::HashMap;
use std::cell::{RefCell};
use crate::types::{Method, Resolve, Class};

mod parser;
mod types;

pub struct Resolver {
	parser: RefCell<Parser>,
	dex_file: DexFile,

	string_cache: RefCell<HashMap<usize, String>>,
}

impl Resolver {
	fn new(mut parser: Parser) -> ParseResult<Self> {
		let dex_file = parser.parse()?;
		Ok(Self {
			parser: RefCell::new(parser),
			dex_file,

			string_cache: RefCell::new(HashMap::new()),
		})
	}

	pub fn get_string(&self, idx: usize) -> String {
		let off = self.dex_file.string_ids[idx].string_data_off;

		self.string_cache.borrow_mut().entry(idx).or_insert_with(|| {
			self.parser.borrow_mut().parse_string(off as u32).unwrap()
		}).clone()
	}

	fn class_names(&mut self) -> Vec<String> {
		for class_def in self.dex_file.class_defs.iter() {
			let type_id = class_def.class_type(self);
			println!("Name {:?}", type_id.descriptor(self));
		}
		vec![]
	}

	fn methods(&mut self) -> Vec<Method> {
		let mut methods = Vec::with_capacity(self.dex_file.method_ids.len());
		for method_def in self.dex_file.method_ids.iter() {
			methods.push(Method::resolve(method_def, self));
		}
		methods
	}

	fn classes(&mut self) -> Vec<Class> {
		let mut methods = Vec::with_capacity(self.dex_file.method_ids.len());
		for class_def in self.dex_file.class_defs[0..5].iter() {
			methods.push(Class::resolve(class_def, self));
		}
		methods
	}
}

fn main() -> Result<(), io::Error> {
	pretty_env_logger::init();

	let file = File::open("./classes2.dex")?;

	let reader = BufReader::new(file);

	let parser = Parser::new(reader).unwrap();
	let mut resolver = match Resolver::new(parser) {
		Ok(res) => res,
		Err(err) => {
			println!("{:?}", err);
			return Ok(());
		}
	};
	let names = resolver.classes();
	dbg!(&names[0..5]);
	dbg!(resolver.dex_file.string_ids.len());

	Ok(())
}
