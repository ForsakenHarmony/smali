use std::{convert::TryInto, num::TryFromIntError, ops::Deref};

use byteorder::{LittleEndian, ReadBytesExt};
use eyre::{Result, WrapErr};

use crate::dex::parser::Parser;

pub trait Parse
where
	Self: Sized,
{
	fn parse<P: Parser>(parser: &mut P) -> eyre::Result<Self>;
}

macro_rules! parse_simple {
	($($ty:tt),*) => {
		$(
			impl Parse for $ty {
				fn parse<P: Parser>(parser: &mut P) -> eyre::Result<Self> {
					parser.$ty()
				}
			}
		)*
	};
}

parse_simple!(u8, u16, i16, u32, i32);

macro_rules! parse_struct_default {
	($name:ident $align:literal { $($field:ident),* } $span:literal) => {
		impl Parse for $name {
			#[cfg_attr(feature = "trace", instrument(skip(parser), name = $span))]
			fn parse<P: Parser>(parser: &mut P) -> Result<Self> {
				if $align != 0 {
					parser.align($align)?;
				}
				Ok($name {
					$($field: parser.parse()?),*
				})
			}
		}
	};
	($name:ident $align:literal { $($field:ident),* }) => {
		::with_builtin_macros::with_builtin! {
			let $span = concat!("<", stringify!($name), " as Parse>::parse") in {
				parse_struct_default!($name $align { $($field),* } $span);
			}
		}
	};
	($name:ident $align:literal { $($field:ident),*, }) => {
		parse_struct_default!($name $align { $($field),* });
	};
	($name:ident { $($field:ident),* }) => {
		parse_struct_default!($name 0 { $($field),* });
	};
	($name:ident { $($field:ident),*, }) => {
		parse_struct_default!($name 0 { $($field),* });
	};
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Uleb128(u32);

impl Parse for Uleb128 {
	fn parse<P: Parser>(parser: &mut P) -> Result<Self> {
		parser.uleb128()
	}
}

impl Deref for Uleb128 {
	type Target = u32;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl Into<u32> for Uleb128 {
	fn into(self) -> u32 {
		self.0
	}
}

impl TryInto<usize> for Uleb128 {
	type Error = TryFromIntError;

	fn try_into(self) -> core::result::Result<usize, Self::Error> {
		self.0.try_into()
	}
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Sleb128(i32);

impl Parse for Sleb128 {
	fn parse<P: Parser>(parser: &mut P) -> Result<Self> {
		parser.sleb128()
	}
}

impl Deref for Sleb128 {
	type Target = i32;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl Into<i32> for Sleb128 {
	fn into(self) -> i32 {
		self.0
	}
}

pub trait ReadThings: ReadBytesExt {
	#[cfg_attr(feature = "trace", instrument(skip(self)))]
	fn u8(&mut self) -> Result<u8> {
		Ok(self.read_u8().wrap_err("reading u8")?)
	}

	#[cfg_attr(feature = "trace", instrument(skip(self)))]
	fn split_u8(&mut self) -> Result<(u8, u8)> {
		let val = self.u8()?;
		Ok((val & 0xf, val >> 4))
	}

	#[cfg_attr(feature = "trace", instrument(skip(self)))]
	fn u16(&mut self) -> Result<u16> {
		Ok(self.read_u16::<LittleEndian>().wrap_err("reading u16")?)
	}

	#[cfg_attr(feature = "trace", instrument(skip(self)))]
	fn i16(&mut self) -> Result<i16> {
		Ok(self.read_i16::<LittleEndian>().wrap_err("reading i16")?)
	}

	#[cfg_attr(feature = "trace", instrument(skip(self)))]
	fn u32(&mut self) -> Result<u32> {
		Ok(self.read_u32::<LittleEndian>().wrap_err("reading u32")?)
	}

	#[cfg_attr(feature = "trace", instrument(skip(self)))]
	fn i32(&mut self) -> Result<i32> {
		Ok(self.read_i32::<LittleEndian>().wrap_err("reading i32")?)
	}

	#[cfg_attr(feature = "trace", instrument(skip(self)))]
	fn u64(&mut self) -> Result<u64> {
		Ok(self.read_u64::<LittleEndian>().wrap_err("reading u64")?)
	}

	#[cfg_attr(feature = "trace", instrument(skip(self)))]
	fn i64(&mut self) -> Result<i64> {
		Ok(self.read_i64::<LittleEndian>().wrap_err("reading i64")?)
	}

	#[cfg_attr(feature = "trace", instrument(skip(self)))]
	fn f32(&mut self) -> Result<f32> {
		Ok(self.read_f32::<LittleEndian>().wrap_err("reading f32")?)
	}

	#[cfg_attr(feature = "trace", instrument(skip(self)))]
	fn f64(&mut self) -> Result<f64> {
		Ok(self.read_f64::<LittleEndian>().wrap_err("reading f64")?)
	}

	#[cfg_attr(feature = "trace", instrument(skip(self)))]
	fn uleb128(&mut self) -> Result<Uleb128> {
		let val = leb128::read::unsigned(self).wrap_err("reading uleb128")?;
		Ok(Uleb128(val.try_into().wrap_err("converting number")?))
	}

	#[cfg_attr(feature = "trace", instrument(skip(self)))]
	fn sleb128(&mut self) -> Result<Sleb128> {
		let val = leb128::read::signed(self).wrap_err("reading sleb128")?;
		Ok(Sleb128(val.try_into().wrap_err("converting number")?))
	}
}

impl<T: ReadBytesExt> ReadThings for T {}
