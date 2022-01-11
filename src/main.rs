// [`crate::dex::types::refs::Parse`]
#![feature(associated_type_defaults)]
#![feature(once_cell)]
// FIXME
#![allow(dead_code)]

#[macro_use]
extern crate tracing;

use std::{fs, io::Cursor};

use color_eyre::{eyre::WrapErr, Report, Result};
use dex::resolver::Resolver;

use crate::dex::parser::FileParser;

#[macro_use]
mod dex;

#[cfg_attr(feature = "trace", instrument)]
fn main() -> Result<(), Report> {
	color_eyre::install()?;
	install_tracing();

	let buf = fs::read("./classes2.dex").wrap_err("reading file")?;
	let reader = Cursor::new(buf);
	// let reader = fs::File::open("./classes2.dex").wrap_err("opening file")?;

	let parser = FileParser::new(reader).wrap_err("creating parser")?;
	let mut resolver = Resolver::new(parser).wrap_err("creating resolver")?;
	// info!("map: {:#?}", resolver.dex_file.map_list);

	info!("resolving classes");
	let classes = resolver.classes()?;
	info!("class 0: {:#x?}", &classes[0]);
	// dbg!(resolver.dex_file.string_ids.len());

	Ok(())
}

fn install_tracing() {
	use tracing_error::ErrorLayer;
	use tracing_subscriber::{prelude::*, EnvFilter};

	let filter = EnvFilter::try_from_default_env()
		.or_else(|_| EnvFilter::try_new("info"))
		.unwrap();

	let format = tracing_subscriber::fmt::format()
		.without_time()
		// .compact()
		.with_ansi(true);

	tracing_subscriber::fmt()
		.with_env_filter(filter)
		.event_format(format)
		.finish()
		.with(ErrorLayer::default())
		.init();
}
