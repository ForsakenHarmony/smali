use color_eyre::{
	eyre::{bail, eyre},
	Result,
};

use crate::dex::{
	asm::{
		format::Format,
		opcode::{Opcode, VALUE_TO_OPCODE},
	},
	parser::{Parse, Parser},
};

// trait Instruction {
// 	fn opcode(&self) -> &Opcode;
// 	fn code_units(&self) -> u32 {
// 		self.opcode().format().size() as u32
// 	}
// }
//
// trait OneRegisterInstruction where Self: Instruction {
// 	fn register_a(&self) -> u32;
// }
//
// trait TwoRegisterInstruction where Self: OneRegisterInstruction {
// 	fn register_b(&self) -> u32;
// }
//
// trait WideLiteralInstruction where Self: Instruction {
// 	fn wide_literal(&self) -> u64;
// }
//
// trait NarrowLiteralInstruction where Self: WideLiteralInstruction {
// 	fn narrow_literal(&self) -> u64;
// }
//
// trait HatLiteralInstruction where Self: Instruction {
// 	fn hat_literal(&self) -> u32;
// }
//
// trait NarrowHatLiteralInstruction where Self: NarrowLiteralInstruction + HatLiteralInstruction {}
// trait LongHatLiteralInstruction where Self: WideLiteralInstruction + HatLiteralInstruction {}

// macro_rules! instructions {
// 		() => ();
//     { struct $n:ident { $($field_names:ident = ($fields:expr));* }; $($rest:tt)* } => {
//     	struct $n {
//     		opcode: Opcode,
//     		$($fields),*
//     	}
//
// 			impl Instruction for $n {
// 				fn opcode(&self) -> &Opcode {
// 					&self.opcode
// 				}
// 			}
//
// 			instructions!($($rest)*);
//     };
// }
//
// instructions! {
// 	struct Instruction10x {
// 		register_a = (2);
// 	};
// }

/// https://source.android.com/devices/tech/dalvik/instruction-formats#formats
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Instruction {
	/// data: `ØØ|op`
	///
	/// `op`
	Instruction10x(Opcode),

	/// data: `B|A|op`
	///
	/// `op vA, vB`
	///
	/// data is u4 * 2
	Instruction12x(Opcode, (u8, u8)),
	/// data: `B|A|op`
	///
	/// `op vA, #+B`
	///
	/// data is u4 * 2
	Instruction11n(Opcode, (u8, u8)),

	/// data: `AA|op`
	///
	/// `op vAA`
	Instruction11x(Opcode, (u8,)),
	/// data: `AA|op`
	///
	/// `op +AA`
	Instruction10t(Opcode, (u8,)),

	/// data: `ØØ|op AAAA`
	///
	/// `op +AAAA`
	Instruction20t(Opcode, (u16,)),

	/// data: `AA|op BBBB`
	///
	/// `op AA, kind@BBBB`
	Instruction20bc(Opcode, (u8, u16)),

	/// data: `AA|op BBBB`
	///
	/// `op vAA, vBBBB`
	Instruction22x(Opcode, (u8, u16)),
	/// data: `AA|op BBBB`
	///
	/// `op vAA, +BBBB`
	Instruction21t(Opcode, (u8, u16)),
	/// data: `AA|op BBBB`
	///
	/// `op vAA, #+BBBB`
	Instruction21s(Opcode, (u8, u16)),
	/// data: `AA|op BBBB`
	///
	/// ```
	/// op vAA, #+BBBB0000
	/// op vAA, #+BBBB000000000000
	/// ```
	Instruction21h(Opcode, (u8, u16)),
	/// data: `AA|op BBBB`
	///
	// TODO: figure this out
	// /// ``
	// Instruction21ih(Opcode, (u16, u16)),
	// Instruction21lh(Opcode, (u16, u16, u32)),
	/// data: `AA|op BBBB`
	///
	/// ```
	/// op vAA, type@BBBB            check-cast
	/// op vAA, field@BBBB           const-class
	/// op vAA, method_handle@BBBB   const-method-handle
	/// op vAA, proto@BBBB           const-method-type
	/// op vAA, string@BBBB          const-string
	/// ```
	Instruction21c(Opcode, (u8, u16)),

	/// data: `AA|op CC|BB`
	///
	/// `op vAA, vBB, vCC`
	Instruction23x(Opcode, (u8, u8, u8)),
	/// data: `AA|op CC|BB`
	///
	/// `op vAA, vBB, #+CC`
	Instruction22b(Opcode, (u8, u8, u8)),

	/// data: `B|A|op CCCC`
	///
	/// `op vA, vB, +CCCC`
	Instruction22t(Opcode, (u8, u8, u16)),
	/// data: `B|A|op CCCC`
	///
	/// `op vA, vB, #+CCCC`
	Instruction22s(Opcode, (u8, u8, u16)),
	/// data: `B|A|op CCCC`
	///
	/// `op vA, vB, type@CCCC`
	/// `op vA, vB, field@CCCC`
	Instruction22c(Opcode, (u8, u8, u16)),
	/// data: `B|A|op CCCC`
	///
	/// `op vA, vB, fieldoff@CCCC`
	Instruction22cs(Opcode, (u8, u8, u16)),

	/// data: `ØØ|op AAAAlo AAAAhi`
	///
	/// `op +AAAAAAAA`
	Instruction30t(Opcode, (u32,)),

	/// data: `ØØ|op AAAA BBBB`
	///
	/// `op vAAAA, vBBBB`
	Instruction32x(Opcode, (u16, u16)),

	/// data: `AA|op BBBBlo BBBBhi`
	///
	/// `op vAA, #+BBBBBBBB`
	Instruction31i(Opcode, (u8, u32)),
	/// data: `AA|op BBBBlo BBBBhi`
	///
	/// `op vAA, +BBBBBBBB`
	Instruction31t(Opcode, (u8, u32)),
	/// data: `AA|op BBBBlo BBBBhi`
	///
	/// `op vAA, string@BBBBBBBB`
	Instruction31c(Opcode, (u8, u32)),

	/// data: `A|G|op BBBB F|E|D|C`
	///
	/// ```
	/// [A=5] op {vC, vD, vE, vF, vG}, meth@BBBB
	/// [A=5] op {vC, vD, vE, vF, vG}, site@BBBB
	/// [A=5] op {vC, vD, vE, vF, vG}, type@BBBB
	/// [A=4] op {vC, vD, vE, vF}, kind@BBBB
	/// [A=3] op {vC, vD, vE}, kind@BBBB
	/// [A=2] op {vC, vD}, kind@BBBB
	/// [A=1] op {vC}, kind@BBBB
	/// [A=0] op {}, kind@BBBB
	///
	/// The unusual choice in lettering here reflects a desire to make the count and the reference index have the same label as in format 3rc.
	/// ```
	Instruction35c(Opcode, (u8, u8, u16, u8, u8, u8, u8)),
	/// data: `A|G|op BBBB F|E|D|C`
	///
	/// ```
	/// [A=5] op {vC, vD, vE, vF, vG}, vtaboff@BBBB
	/// [A=4] op {vC, vD, vE, vF}, vtaboff@BBBB
	/// [A=3] op {vC, vD, vE}, vtaboff@BBBB
	/// [A=2] op {vC, vD}, vtaboff@BBBB
	/// [A=1] op {vC}, vtaboff@BBBB
	///
	/// The unusual choice in lettering here reflects a desire to make the count and the reference index have the same label as in format 3rms.
	/// ```
	Instruction35ms(Opcode, (u8, u8, u16, u8, u8, u8, u8)),
	/// data: `A|G|op BBBB F|E|D|C`
	///
	/// ```
	/// [A=5] op {vC, vD, vE, vF, vG}, inline@BBBB
	/// [A=4] op {vC, vD, vE, vF}, inline@BBBB
	/// [A=3] op {vC, vD, vE}, inline@BBBB
	/// [A=2] op {vC, vD}, inline@BBBB
	/// [A=1] op {vC}, inline@BBBB
	///
	/// The unusual choice in lettering here reflects a desire to make the count and the reference index have the same label as in format 3rmi.
	/// ```
	Instruction35mi(Opcode, (u8, u8, u16, u8, u8, u8, u8)),

	/// data: `AA|op BBBB CCCC`
	///
	/// ```
	/// op {vCCCC .. vNNNN}, meth@BBBB
	/// op {vCCCC .. vNNNN}, site@BBBB
	/// op {vCCCC .. vNNNN}, type@BBBB
	///
	/// where NNNN = CCCC+AA-1, that is A determines the count 0..255, and C determines the first register
	/// ```
	Instruction3rc(Opcode, (u8, u16, u16)),
	/// data: `AA|op BBBB CCCC`
	///
	/// ```
	/// op {vCCCC .. vNNNN}, vtaboff@BBBB
	///
	/// where NNNN = CCCC+AA-1, that is A determines the count 0..255, and C determines the first register
	/// ```
	///
	/// `suggested format for statically linked invoke-virtual and invoke-super instructions of format 3rc`
	Instruction3rms(Opcode, (u8, u16, u16)),
	/// data: `AA|op BBBB CCCC`
	///
	/// ```
	/// op {vCCCC .. vNNNN}, inline@BBBB
	///
	/// where NNNN = CCCC+AA-1, that is A determines the count 0..255, and C determines the first register
	/// ```
	///
	/// `suggested format for inline linked invoke-static and invoke-virtual instructions of format 3rc`
	Instruction3rmi(Opcode, (u8, u16, u16)),

	/// data: `A|G|op BBBB F|E|D|C HHHH `
	///
	/// ```
	/// [A=5] op {vC, vD, vE, vF, vG}, meth@BBBB, proto@HHHH
	/// [A=4] op {vC, vD, vE, vF}, meth@BBBB, proto@HHHH
	/// [A=3] op {vC, vD, vE}, meth@BBBB, proto@HHHH
	/// [A=2] op {vC, vD}, meth@BBBB, proto@HHHH
	/// [A=1] op {vC}, meth@BBBB, proto@HHHH
	/// ```
	///
	/// `invoke-polymorphic`
	Instruction45cc(Opcode, (u8, u8, u16, u8, u8, u8, u8, u16)),

	/// data: `AA|op BBBB CCCC HHHH `
	///
	/// ```
	/// op> {vCCCC .. vNNNN}, meth@BBBB, proto@HHHH
	///
	/// where NNNN = CCCC+AA-1, that is A determines the count 0..255, and C determines the first register
	/// ```
	///
	/// `invoke-polymorphic/range`
	Instruction4rcc(Opcode, (u8, u16, u16, u16)),

	/// data: `AA|op BBBBlo BBBB BBBB BBBBhi`
	///
	/// ```
	/// op vAA, #+BBBBBBBBBBBBBBBB
	/// ```
	///
	/// `const-wide`
	Instruction51l(Opcode, (u8, u64)),

	/// https://source.android.com/devices/tech/dalvik/dalvik-bytecode#packed-switch
	PackedSwitchPayload {
		size:      u16,
		first_key: i32,
		targets:   Vec<i32>,
	},
	/// https://source.android.com/devices/tech/dalvik/dalvik-bytecode#sparse-switch
	SparseSwitchPayload {
		size:    u16,
		keys:    Vec<i32>,
		targets: Vec<i32>,
	},
	/// https://source.android.com/devices/tech/dalvik/dalvik-bytecode#fill-array
	FillArrayDataPayload {
		element_width: u16,
		size:          u32,
		data:          Vec<u8>,
	},
}

#[cfg(not(feature = "trace"))]
macro_rules! assert_unused_byte {
	($parser:ident, $format:literal) => {{
		let __unused = $parser.u8()?;
		assert_eq!(
			__unused, 0,
			"expected unused {} byte to be 0, but got {}",
			$format, __unused
		);
	}};
}

#[cfg(feature = "trace")]
macro_rules! assert_unused_byte {
	($parser:ident, $format:literal) => {{
		let __unused = $parser.u8()?;
		color_eyre::eyre::ensure!(
			__unused == 0,
			"expected unused {} byte to be 0, but got {}",
			$format,
			__unused
		);
	}};
}

impl Parse for Instruction {
	#[cfg_attr(feature = "trace", instrument(skip(parser), fields(op, offset = parser.get_offset())))]
	fn parse<P: Parser>(parser: &mut P) -> Result<Self> {
		let op = {
			let mut opcode_value = parser.u8()? as u16;
			// noop could hint at one of the special payloads
			if opcode_value == 0 {
				opcode_value = (parser.u8()? as u16) << 8;
				if opcode_value == 0 {
					parser.seek(std::io::SeekFrom::Current(-1))?;
				}
			}
			*VALUE_TO_OPCODE
				.get(&opcode_value)
				.ok_or_else(|| eyre!("unknown opcode: {:#x?}", opcode_value))?
		};

		#[cfg(feature = "trace")]
		{
			tracing::Span::current().record("op", &tracing::field::display(op));
		}

		Ok(match op.format() {
			Format::Format10x => {
				assert_unused_byte!(parser, "10x");
				Instruction::Instruction10x(op)
			}

			Format::Format12x => {
				let (a, b) = parser.split_u8()?;
				Instruction::Instruction12x(op, (a, b))
			}
			Format::Format11n => {
				let (a, b) = parser.split_u8()?;
				Instruction::Instruction11n(op, (a, b))
			}

			Format::Format11x => {
				let aa = parser.u8()?;
				Instruction::Instruction11x(op, (aa,))
			}
			Format::Format10t => {
				let aa = parser.u8()?;
				Instruction::Instruction10t(op, (aa,))
			}

			Format::Format20t => {
				assert_unused_byte!(parser, "20t");
				let aaaa = parser.u16()?;

				Instruction::Instruction20t(op, (aaaa,))
			}

			Format::Format20bc => {
				let aa = parser.u8()?;
				let bbbb = parser.u16()?;

				Instruction::Instruction20bc(op, (aa, bbbb))
			}

			Format::Format22x => {
				let aa = parser.u8()?;
				let bbbb = parser.u16()?;

				Instruction::Instruction22x(op, (aa, bbbb))
			}
			Format::Format21t => {
				let aa = parser.u8()?;
				let bbbb = parser.u16()?;

				Instruction::Instruction21t(op, (aa, bbbb))
			}
			Format::Format21s => {
				let aa = parser.u8()?;
				let bbbb = parser.u16()?;

				Instruction::Instruction21s(op, (aa, bbbb))
			}
			Format::Format21ih => {
				let aa = parser.u8()?;
				let bbbb = parser.u16()?;

				Instruction::Instruction21h(op, (aa, bbbb))
				// Instruction::Instruction21ih(op, (0, 0))
			}
			Format::Format21lh => {
				let aa = parser.u8()?;
				let bbbb = parser.u16()?;

				Instruction::Instruction21h(op, (aa, bbbb))
				// Instruction::Instruction21lh(op, (0, 0, 0))
			}
			Format::Format21c => {
				let aa = parser.u8()?;
				let bbbb = parser.u16()?;

				Instruction::Instruction21c(op, (aa, bbbb))
			}

			Format::Format23x => {
				let aa = parser.u8()?;
				let bb = parser.u8()?;
				let cc = parser.u8()?;

				Instruction::Instruction23x(op, (aa, bb, cc))
			}
			Format::Format22b => {
				let aa = parser.u8()?;
				let bb = parser.u8()?;
				let cc = parser.u8()?;

				Instruction::Instruction22b(op, (aa, bb, cc))
			}

			Format::Format22t => {
				let (a, b) = parser.split_u8()?;
				let cccc = parser.u16()?;

				Instruction::Instruction22t(op, (a, b, cccc))
			}
			Format::Format22s => {
				let (a, b) = parser.split_u8()?;
				let cccc = parser.u16()?;

				Instruction::Instruction22s(op, (a, b, cccc))
			}
			Format::Format22c => {
				let (a, b) = parser.split_u8()?;
				let cccc = parser.u16()?;

				Instruction::Instruction22c(op, (a, b, cccc))
			}
			Format::Format22cs => {
				let (a, b) = parser.split_u8()?;
				let cccc = parser.u16()?;

				Instruction::Instruction22cs(op, (a, b, cccc))
			}

			Format::Format30t => {
				assert_unused_byte!(parser, "30t");
				let aaaa_aaaa = parser.u32()?;

				Instruction::Instruction30t(op, (aaaa_aaaa,))
			}

			Format::Format32x => {
				assert_unused_byte!(parser, "32x");
				let aaaa = parser.u16()?;
				let bbbb = parser.u16()?;

				Instruction::Instruction32x(op, (aaaa, bbbb))
			}

			Format::Format31i => {
				let aa = parser.u8()?;
				let bbbb_bbbb = parser.u32()?;

				Instruction::Instruction31i(op, (aa, bbbb_bbbb))
			}
			Format::Format31t => {
				let aa = parser.u8()?;
				let bbbb_bbbb = parser.u32()?;

				Instruction::Instruction31t(op, (aa, bbbb_bbbb))
			}
			Format::Format31c => {
				let aa = parser.u8()?;
				let bbbb_bbbb = parser.u32()?;

				Instruction::Instruction31c(op, (aa, bbbb_bbbb))
			}

			Format::Format35c => {
				let (a, g) = parser.split_u8()?;
				let bbbb = parser.u16()?;
				let (f, e) = parser.split_u8()?;
				let (d, c) = parser.split_u8()?;

				Instruction::Instruction35c(op, (a, g, bbbb, f, e, d, c))
			}
			Format::Format35ms => {
				let (a, g) = parser.split_u8()?;
				let bbbb = parser.u16()?;
				let (f, e) = parser.split_u8()?;
				let (d, c) = parser.split_u8()?;

				Instruction::Instruction35ms(op, (a, g, bbbb, f, e, d, c))
			}
			Format::Format35mi => {
				let (a, g) = parser.split_u8()?;
				let bbbb = parser.u16()?;
				let (f, e) = parser.split_u8()?;
				let (d, c) = parser.split_u8()?;

				Instruction::Instruction35mi(op, (a, g, bbbb, f, e, d, c))
			}

			Format::Format3rc => {
				let aa = parser.u8()?;
				let bbbb = parser.u16()?;
				let cccc = parser.u16()?;

				Instruction::Instruction3rc(op, (aa, bbbb, cccc))
			}
			Format::Format3rms => {
				let aa = parser.u8()?;
				let bbbb = parser.u16()?;
				let cccc = parser.u16()?;

				Instruction::Instruction3rms(op, (aa, bbbb, cccc))
			}
			Format::Format3rmi => {
				let aa = parser.u8()?;
				let bbbb = parser.u16()?;
				let cccc = parser.u16()?;

				Instruction::Instruction3rmi(op, (aa, bbbb, cccc))
			}

			Format::Format45cc => {
				let (a, g) = parser.split_u8()?;
				let bbbb = parser.u16()?;
				let (f, e) = parser.split_u8()?;
				let (d, c) = parser.split_u8()?;
				let hhhh = parser.u16()?;

				Instruction::Instruction45cc(op, (a, g, bbbb, f, e, d, c, hhhh))
			}

			Format::Format4rcc => {
				let aa = parser.u8()?;
				let bbbb = parser.u16()?;
				let cccc = parser.u16()?;
				let dddd = parser.u16()?;

				Instruction::Instruction4rcc(op, (aa, bbbb, cccc, dddd))
			}

			Format::Format51l => {
				let aa = parser.u8()?;
				let bbbb_bbbb_bbbb_bbbb = parser.u64()?;

				Instruction::Instruction51l(op, (aa, bbbb_bbbb_bbbb_bbbb))
			}

			Format::PackedSwitchPayload => {
				let size = parser.u16()?;
				let first_key = parser.i32()?;
				let targets = parser.parse_list(size as u32)?;

				Instruction::PackedSwitchPayload {
					size,
					first_key,
					targets,
				}
			}
			Format::SparseSwitchPayload => {
				let size = parser.u16()?;
				let keys = parser.parse_list(size as u32)?;
				let targets = parser.parse_list(size as u32)?;

				Instruction::SparseSwitchPayload {
					size,
					keys,
					targets,
				}
			}
			Format::ArrayPayload => {
				let element_width = parser.u16()?;
				let size = parser.u32()?;
				let data = parser.parse_list(element_width as u32 * size)?;
				// > Note: The total number of code units for an instance of this table is (size * element_width + 1) / 2 + 4.
				// this is padding?
				if (element_width as u32 * size) % 2 != 0 {
					parser.u8()?;
				}

				Instruction::FillArrayDataPayload {
					element_width,
					size,
					data,
				}
			}

			f => bail!("unknown format: {:?}", f),
		})
	}
}
