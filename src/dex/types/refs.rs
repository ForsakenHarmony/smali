use std::{cmp::Ordering, convert::TryInto, fmt::Debug, ops::Deref};

use eyre::{eyre, Result, WrapErr};

use crate::dex::{
	parser::{Parse, Parser},
	resolver::{Resolve, ResolveFrom},
	types::file::DexFile,
};

pub struct Ref<T, N> {
	offset:  u32,
	_marker: std::marker::PhantomData<fn() -> (T, N)>,
}

impl<T, N> Ref<T, N> {
	pub fn new(offset: u32) -> Self {
		Ref {
			offset,
			_marker: Default::default(),
		}
	}
}

impl<T, N> Debug for Ref<T, N> {
	fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		fmt.debug_struct("Ref")
			.field("offset", &self.offset)
			.field("_marker", &self._marker)
			.finish()
	}
}

impl<T, N> PartialEq for Ref<T, N> {
	fn eq(&self, other: &Self) -> bool {
		self.offset.eq(&other.offset)
	}
}

impl<T, N> Eq for Ref<T, N> {}

impl<T, N> PartialOrd for Ref<T, N> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.offset.partial_cmp(&other.offset)
	}
}

impl<T, N> Ord for Ref<T, N> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.offset.cmp(&other.offset)
	}
}

impl<T, N> Copy for Ref<T, N> {}

impl<T, N> Clone for Ref<T, N> {
	fn clone(&self) -> Self {
		*self
	}
}

impl<T, N> Deref for Ref<T, N> {
	type Target = u32;

	fn deref(&self) -> &Self::Target {
		&self.offset
	}
}

impl<T, E, N> Parse for Ref<T, N>
where
	E: std::error::Error + Send + Sync + 'static,
	N: Parse + TryInto<u32, Error = E>,
{
	#[cfg_attr(
		feature = "trace",
		instrument(skip(parser), name = "<Ref as Parse>::parse")
	)]
	fn parse<P: Parser>(parser: &mut P) -> Result<Self> {
		Ok(Ref::new(
			parser
				.parse::<N>()
				.wrap_err("parsing offset for Ref")?
				.try_into()
				.wrap_err("converting offset to u32 for ref")?,
		))
	}
}

// impl<T, N> ResolveFrom<Ref<T, N>> for T {
// 	fn resolve<R: Read + Seek>(item: &Ref<T, N>, resolver: &Resolver<R>) -> Self {
// 		resolver.dex_file.
// 	}
// }

pub struct Idx<T, N> {
	idx:     usize,
	_marker: std::marker::PhantomData<*const (T, N)>,
}

impl<T, N> Idx<T, N> {
	pub fn new(idx: usize) -> Self {
		Idx {
			idx,
			_marker: Default::default(),
		}
	}
}

impl<T, N> core::fmt::Debug for Idx<T, N> {
	fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		fmt.debug_struct("Idx")
			.field("idx", &self.idx)
			.field("_marker", &self._marker)
			.finish()
	}
}

impl<T, N> PartialEq for Idx<T, N> {
	fn eq(&self, other: &Self) -> bool {
		self.idx.eq(&other.idx)
	}
}

impl<T, N> Eq for Idx<T, N> {}

impl<T, N> PartialOrd for Idx<T, N> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.idx.partial_cmp(&other.idx)
	}
}

impl<T, N> Ord for Idx<T, N> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.idx.cmp(&other.idx)
	}
}

impl<T, N> Copy for Idx<T, N> {}

impl<T, N> Clone for Idx<T, N> {
	fn clone(&self) -> Self {
		*self
	}
}

impl<T, N> Deref for Idx<T, N> {
	type Target = usize;

	fn deref(&self) -> &Self::Target {
		&self.idx
	}
}

impl<T, E, N> Parse for Idx<T, N>
where
	E: std::error::Error + Send + Sync + 'static,
	N: Parse + TryInto<usize, Error = E>,
{
	#[cfg_attr(
		feature = "trace",
		instrument(skip(parser), name = "<Idx as Parse>::parse")
	)]
	fn parse<P: Parser>(parser: &mut P) -> Result<Self> {
		Ok(Idx::new(
			parser
				.parse::<N>()
				.wrap_err("parsing idx for Idx")?
				.try_into()
				.wrap_err("converting idx to usize")?,
		))
	}
}

pub trait IdItem
where
	Self: Sized,
{
	type Output = Self;

	fn dex_section(dex_file: &DexFile) -> &[Self::Output];
}

impl<O: Clone, T: IdItem<Output = O>, N> ResolveFrom<Idx<T, N>> for O {
	// type From = Idx<T, N>;

	fn resolve_from(item: &Idx<T, N>, resolver: &impl Resolve) -> Result<Self> {
		T::dex_section(&resolver.dex_file())
			.get(**item)
			.map(|t| t.clone())
			.ok_or_else(|| eyre!("index {:?} out of bounds", item))
	}
}

impl<O: Clone, T: IdItem<Output = O>, N> Idx<T, N> {
	pub fn resolve(&self, resolver: &impl Resolve) -> Result<O> {
		O::resolve_from(self, resolver)
	}
}
//
// const NO_INDEX: usize = 0xffffffff;

// impl<O, T: IdItem<Output = O>> IdItem for Option<T> {
// 	type Output = O;
//
// 	fn dex_section(dex_file: &DexFile) -> &[Self::Output] {
// 		T::dex_section(dex_file)
// 	}
// }

// impl<O: Clone, T: IdItem<Output = O>, N> ResolveFrom<Idx<Option<T>, N>> for Option<O> {
// 	fn resolve_from<R: Resolve>(item: &Idx<Option<T>, N>, resolver: &R) -> Result<Self> {
// 		item.resolve(resolver)
// 	}
// }
//
// impl<O: Clone, T: IdItem<Output = O>, N> Idx<Option<T>, N> {
// 	pub(crate) fn resolve<R: Resolve>(&self, resolver: &R) -> Result<Option<O>> {
// 		if self.idx == NO_INDEX {
// 			return Ok(None);
// 		}
// 		Ok(Some(O::resolve_from(self, resolver)?))
// 	}
// }
