use enum_values::Values;
use bitflags::bitflags;
use lazy_static::lazy_static;

use super::format::Format;
use std::ops::BitOr;
use std::collections::HashMap;

bitflags! {
	pub struct OpcodeFlags: u32 {
		//if the instruction can throw an exception
		const CanThrow = 0x1; //                 0b00000000001;
		//if the instruction is an odex only instruction
		const OdexOnly = 0x2; //                 0b00000000010;
		//if execution can continue to the next instruction
		const CanContinue = 0x4; //              0b00000000100;
		//if the instruction sets the "hidden" result register
		const SetsResult = 0x8; //               0b00000001000;
		//if the instruction sets the value of it's first register
		const SetsRegister = 0x10; //            0b00000010000;
		//if the instruction sets the value of it's first register to a wide type
		const SetsWideRegister = 0x20; //        0b00000100000;
		//if the instruction is an iget-quick/iput-quick instruction
		const QuickFieldAccessor = 0x40; //      0b00001000000;
		//if the instruction is a *get-volatile/*put-volatile instruction
		const VolatileFieldAccessor = 0x80; //   0b00010000000;
		//if the instruction is a static sget-*/sput-*instruction
		const StaticFieldAccessor = 0x100; //    0b00100000000;
		//if the instruction is a jumbo instruction
		const JumboOpcode = 0x200; //            0b01000000000;
		//if the instruction can initialize an uninitialized object reference
		const CanInitializeReference = 0x400; // 0b10000000000;
	}
}

impl Default for OpcodeFlags {
	fn default() -> Self {
		OpcodeFlags::empty()
	}
}

#[derive(Values)]
#[values(value = "u32", name = "String", reference_type = "ReferenceType", reference_type_2 = "ReferenceType", format = "Format", flags = "OpcodeFlags")]
pub enum Opcode {
	#[values(value = "0x00", name = "nop", reference_type = "ReferenceType::None", format = "Format::Format10x", flags = "OpcodeFlags::CanContinue")]
	Nop,
	#[values(value = "0x01", name = "move", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	MOVE,
	#[values(value = "0x02", name = "move/from16", reference_type = "ReferenceType::None", format = "Format::Format22x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	MoveFrom16,
	#[values(value = "0x03", name = "move/16", reference_type = "ReferenceType::None", format = "Format::Format32x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	Move16,
	#[values(value = "0x04", name = "move-wide", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	MoveWide,
	#[values(value = "0x05", name = "move-wide/from16", reference_type = "ReferenceType::None", format = "Format::Format22x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	MoveWideFrom16,
	#[values(value = "0x06", name = "move-wide/16", reference_type = "ReferenceType::None", format = "Format::Format32x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	MoveWide16,
	#[values(value = "0x07", name = "move-object", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	MoveObject,
	#[values(value = "0x08", name = "move-object/from16", reference_type = "ReferenceType::None", format = "Format::Format22x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	MoveObjectFrom16,
	#[values(value = "0x09", name = "move-object/16", reference_type = "ReferenceType::None", format = "Format::Format32x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	MoveObject16,
	#[values(value = "0x0a", name = "move-result", reference_type = "ReferenceType::None", format = "Format::Format11x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	MoveResult,
	#[values(value = "0x0b", name = "move-result-wide", reference_type = "ReferenceType::None", format = "Format::Format11x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	MoveResultWide,
	#[values(value = "0x0c", name = "move-result-object", reference_type = "ReferenceType::None", format = "Format::Format11x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	MoveResultObject,
	#[values(value = "0x0d", name = "move-exception", reference_type = "ReferenceType::None", format = "Format::Format11x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	MoveException,
	#[values(value = "0x0e", name = "return-void", reference_type = "ReferenceType::None", format = "Format::Format10x")]
	ReturnVoid,
	#[values(value = "0x0f", name = "return", reference_type = "ReferenceType::None", format = "Format::Format11x")]
	RETURN,
	#[values(value = "0x10", name = "return-wide", reference_type = "ReferenceType::None", format = "Format::Format11x")]
	ReturnWide,
	#[values(value = "0x11", name = "return-object", reference_type = "ReferenceType::None", format = "Format::Format11x")]
	ReturnObject,
	#[values(value = "0x12", name = "const/4", reference_type = "ReferenceType::None", format = "Format::Format11n", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	Const4,
	#[values(value = "0x13", name = "const/16", reference_type = "ReferenceType::None", format = "Format::Format21s", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	Const16,
	#[values(value = "0x14", name = "const", reference_type = "ReferenceType::None", format = "Format::Format31i", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	CONST,
	#[values(value = "0x15", name = "const/high16", reference_type = "ReferenceType::None", format = "Format::Format21ih", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	ConstHigh16,
	#[values(value = "0x16", name = "const-wide/16", reference_type = "ReferenceType::None", format = "Format::Format21s", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	ConstWide16,
	#[values(value = "0x17", name = "const-wide/32", reference_type = "ReferenceType::None", format = "Format::Format31i", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	ConstWide32,
	#[values(value = "0x18", name = "const-wide", reference_type = "ReferenceType::None", format = "Format::Format51l", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	ConstWide,
	#[values(value = "0x19", name = "const-wide/high16", reference_type = "ReferenceType::None", format = "Format::Format21lh", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	ConstWideHigh16,
	#[values(value = "0x1a", name = "const-string", reference_type = "ReferenceType::String", format = "Format::Format21c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	ConstString,
	#[values(value = "0x1b", name = "const-string/jumbo", reference_type = "ReferenceType::String", format = "Format::Format31c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	ConstStringJumbo,
	#[values(value = "0x1c", name = "const-class", reference_type = "ReferenceType::Type", format = "Format::Format21c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	ConstClass,
	#[values(value = "0x1d", name = "monitor-enter", reference_type = "ReferenceType::None", format = "Format::Format11x", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue")]
	MonitorEnter,
	#[values(value = "0x1e", name = "monitor-exit", reference_type = "ReferenceType::None", format = "Format::Format11x", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue")]
	MonitorExit,
	#[values(value = "0x1f", name = "check-cast", reference_type = "ReferenceType::Type", format = "Format::Format21c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	CheckCast,
	#[values(value = "0x20", name = "instance-of", reference_type = "ReferenceType::Type", format = "Format::Format22c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	InstanceOf,
	#[values(value = "0x21", name = "array-length", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	ArrayLength,
	#[values(value = "0x22", name = "new-instance", reference_type = "ReferenceType::Type", format = "Format::Format21c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	NewInstance,
	#[values(value = "0x23", name = "new-array", reference_type = "ReferenceType::Type", format = "Format::Format22c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	NewArray,
	#[values(value = "0x24", name = "filled-new-array", reference_type = "ReferenceType::Type", format = "Format::Format35c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsResult")]
	FilledNewArray,
	#[values(value = "0x25", name = "filled-new-array/range", reference_type = "ReferenceType::Type", format = "Format::Format3rc", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsResult")]
	FilledNewArrayRange,
	#[values(value = "0x26", name = "fill-array-data", reference_type = "ReferenceType::None", format = "Format::Format31t", flags = "OpcodeFlags::CanContinue")]
	FillArrayData,
	#[values(value = "0x27", name = "throw", reference_type = "ReferenceType::None", format = "Format::Format11x", flags = "OpcodeFlags::CanThrow")]
	THROW,
	#[values(value = "0x28", name = "goto", reference_type = "ReferenceType::None", format = "Format::Format10t")]
	GOTO,
	#[values(value = "0x29", name = "goto/16", reference_type = "ReferenceType::None", format = "Format::Format20t")]
	Goto16,
	#[values(value = "0x2a", name = "goto/32", reference_type = "ReferenceType::None", format = "Format::Format30t")]
	Goto32,
	#[values(value = "0x2b", name = "packed-switch", reference_type = "ReferenceType::None", format = "Format::Format31t", flags = "OpcodeFlags::CanContinue")]
	PackedSwitch,
	#[values(value = "0x2c", name = "sparse-switch", reference_type = "ReferenceType::None", format = "Format::Format31t", flags = "OpcodeFlags::CanContinue")]
	SparseSwitch,
	#[values(value = "0x2d", name = "cmpl-float", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	CmplFloat,
	#[values(value = "0x2e", name = "cmpg-float", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	CmpgFloat,
	#[values(value = "0x2f", name = "cmpl-double", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	CmplDouble,
	#[values(value = "0x30", name = "cmpg-double", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	CmpgDouble,
	#[values(value = "0x31", name = "cmp-long", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	CmpLong,
	#[values(value = "0x32", name = "if-eq", reference_type = "ReferenceType::None", format = "Format::Format22t", flags = "OpcodeFlags::CanContinue")]
	IfEq,
	#[values(value = "0x33", name = "if-ne", reference_type = "ReferenceType::None", format = "Format::Format22t", flags = "OpcodeFlags::CanContinue")]
	IfNe,
	#[values(value = "0x34", name = "if-lt", reference_type = "ReferenceType::None", format = "Format::Format22t", flags = "OpcodeFlags::CanContinue")]
	IfLt,
	#[values(value = "0x35", name = "if-ge", reference_type = "ReferenceType::None", format = "Format::Format22t", flags = "OpcodeFlags::CanContinue")]
	IfGe,
	#[values(value = "0x36", name = "if-gt", reference_type = "ReferenceType::None", format = "Format::Format22t", flags = "OpcodeFlags::CanContinue")]
	IfGt,
	#[values(value = "0x37", name = "if-le", reference_type = "ReferenceType::None", format = "Format::Format22t", flags = "OpcodeFlags::CanContinue")]
	IfLe,
	#[values(value = "0x38", name = "if-eqz", reference_type = "ReferenceType::None", format = "Format::Format21t", flags = "OpcodeFlags::CanContinue")]
	IfEqz,
	#[values(value = "0x39", name = "if-nez", reference_type = "ReferenceType::None", format = "Format::Format21t", flags = "OpcodeFlags::CanContinue")]
	IfNez,
	#[values(value = "0x3a", name = "if-ltz", reference_type = "ReferenceType::None", format = "Format::Format21t", flags = "OpcodeFlags::CanContinue")]
	IfLtz,
	#[values(value = "0x3b", name = "if-gez", reference_type = "ReferenceType::None", format = "Format::Format21t", flags = "OpcodeFlags::CanContinue")]
	IfGez,
	#[values(value = "0x3c", name = "if-gtz", reference_type = "ReferenceType::None", format = "Format::Format21t", flags = "OpcodeFlags::CanContinue")]
	IfGtz,
	#[values(value = "0x3d", name = "if-lez", reference_type = "ReferenceType::None", format = "Format::Format21t", flags = "OpcodeFlags::CanContinue")]
	IfLez,
	#[values(value = "0x44", name = "aget", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	AGET,
	#[values(value = "0x45", name = "aget-wide", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	AgetWide,
	#[values(value = "0x46", name = "aget-object", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	AgetObject,
	#[values(value = "0x47", name = "aget-boolean", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	AgetBoolean,
	#[values(value = "0x48", name = "aget-byte", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	AgetByte,
	#[values(value = "0x49", name = "aget-char", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	AgetChar,
	#[values(value = "0x4a", name = "aget-short", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	AgetShort,
	#[values(value = "0x4b", name = "aput", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue")]
	APUT,
	#[values(value = "0x4c", name = "aput-wide", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue")]
	AputWide,
	#[values(value = "0x4d", name = "aput-object", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue")]
	AputObject,
	#[values(value = "0x4e", name = "aput-boolean", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue")]
	AputBoolean,
	#[values(value = "0x4f", name = "aput-byte", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue")]
	AputByte,
	#[values(value = "0x50", name = "aput-char", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue")]
	AputChar,
	#[values(value = "0x51", name = "aput-short", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue")]
	AputShort,
	#[values(value = "0x52", name = "iget", reference_type = "ReferenceType::Field", format = "Format::Format22c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	IGET,
	#[values(value = "0x53", name = "iget-wide", reference_type = "ReferenceType::Field", format = "Format::Format22c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	IgetWide,
	#[values(value = "0x54", name = "iget-object", reference_type = "ReferenceType::Field", format = "Format::Format22c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	IgetObject,
	#[values(value = "0x55", name = "iget-boolean", reference_type = "ReferenceType::Field", format = "Format::Format22c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	IgetBoolean,
	#[values(value = "0x56", name = "iget-byte", reference_type = "ReferenceType::Field", format = "Format::Format22c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	IgetByte,
	#[values(value = "0x57", name = "iget-char", reference_type = "ReferenceType::Field", format = "Format::Format22c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	IgetChar,
	#[values(value = "0x58", name = "iget-short", reference_type = "ReferenceType::Field", format = "Format::Format22c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	IgetShort,
	#[values(value = "0x59", name = "iput", reference_type = "ReferenceType::Field", format = "Format::Format22c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue")]
	IPUT,
	#[values(value = "0x5a", name = "iput-wide", reference_type = "ReferenceType::Field", format = "Format::Format22c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue")]
	IputWide,
	#[values(value = "0x5b", name = "iput-object", reference_type = "ReferenceType::Field", format = "Format::Format22c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue")]
	IputObject,
	#[values(value = "0x5c", name = "iput-boolean", reference_type = "ReferenceType::Field", format = "Format::Format22c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue")]
	IputBoolean,
	#[values(value = "0x5d", name = "iput-byte", reference_type = "ReferenceType::Field", format = "Format::Format22c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue")]
	IputByte,
	#[values(value = "0x5e", name = "iput-char", reference_type = "ReferenceType::Field", format = "Format::Format22c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue")]
	IputChar,
	#[values(value = "0x5f", name = "iput-short", reference_type = "ReferenceType::Field", format = "Format::Format22c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue")]
	IputShort,
	#[values(value = "0x60", name = "sget", reference_type = "ReferenceType::Field", format = "Format::Format21c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::StaticFieldAccessor")]
	SGET,
	#[values(value = "0x61", name = "sget-wide", reference_type = "ReferenceType::Field", format = "Format::Format21c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister | OpcodeFlags::StaticFieldAccessor")]
	SgetWide,
	#[values(value = "0x62", name = "sget-object", reference_type = "ReferenceType::Field", format = "Format::Format21c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::StaticFieldAccessor")]
	SgetObject,
	#[values(value = "0x63", name = "sget-boolean", reference_type = "ReferenceType::Field", format = "Format::Format21c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::StaticFieldAccessor")]
	SgetBoolean,
	#[values(value = "0x64", name = "sget-byte", reference_type = "ReferenceType::Field", format = "Format::Format21c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::StaticFieldAccessor")]
	SgetByte,
	#[values(value = "0x65", name = "sget-char", reference_type = "ReferenceType::Field", format = "Format::Format21c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::StaticFieldAccessor")]
	SgetChar,
	#[values(value = "0x66", name = "sget-short", reference_type = "ReferenceType::Field", format = "Format::Format21c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::StaticFieldAccessor")]
	SgetShort,
	#[values(value = "0x67", name = "sput", reference_type = "ReferenceType::Field", format = "Format::Format21c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::StaticFieldAccessor")]
	SPUT,
	#[values(value = "0x68", name = "sput-wide", reference_type = "ReferenceType::Field", format = "Format::Format21c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::StaticFieldAccessor")]
	SputWide,
	#[values(value = "0x69", name = "sput-object", reference_type = "ReferenceType::Field", format = "Format::Format21c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::StaticFieldAccessor")]
	SputObject,
	#[values(value = "0x6a", name = "sput-boolean", reference_type = "ReferenceType::Field", format = "Format::Format21c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::StaticFieldAccessor")]
	SputBoolean,
	#[values(value = "0x6b", name = "sput-byte", reference_type = "ReferenceType::Field", format = "Format::Format21c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::StaticFieldAccessor")]
	SputByte,
	#[values(value = "0x6c", name = "sput-char", reference_type = "ReferenceType::Field", format = "Format::Format21c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::StaticFieldAccessor")]
	SputChar,
	#[values(value = "0x6d", name = "sput-short", reference_type = "ReferenceType::Field", format = "Format::Format21c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::StaticFieldAccessor")]
	SputShort,
	#[values(value = "0x6e", name = "invoke-virtual", reference_type = "ReferenceType::Method", format = "Format::Format35c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsResult")]
	InvokeVirtual,
	#[values(value = "0x6f", name = "invoke-super", reference_type = "ReferenceType::Method", format = "Format::Format35c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsResult")]
	InvokeSuper,
	#[values(value = "0x70", name = "invoke-direct", reference_type = "ReferenceType::Method", format = "Format::Format35c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsResult | OpcodeFlags::CanInitializeReference")]
	InvokeDirect,
	#[values(value = "0x71", name = "invoke-static", reference_type = "ReferenceType::Method", format = "Format::Format35c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsResult")]
	InvokeStatic,
	#[values(value = "0x72", name = "invoke-interface", reference_type = "ReferenceType::Method", format = "Format::Format35c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsResult")]
	InvokeInterface,
	#[values(value = "0x74", name = "invoke-virtual/range", reference_type = "ReferenceType::Method", format = "Format::Format3rc", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsResult")]
	InvokeVirtualRange,
	#[values(value = "0x75", name = "invoke-super/range", reference_type = "ReferenceType::Method", format = "Format::Format3rc", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsResult")]
	InvokeSuperRange,
	#[values(value = "0x76", name = "invoke-direct/range", reference_type = "ReferenceType::Method", format = "Format::Format3rc", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsResult | OpcodeFlags::CanInitializeReference")]
	InvokeDirectRange,
	#[values(value = "0x77", name = "invoke-static/range", reference_type = "ReferenceType::Method", format = "Format::Format3rc", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsResult")]
	InvokeStaticRange,
	#[values(value = "0x78", name = "invoke-interface/range", reference_type = "ReferenceType::Method", format = "Format::Format3rc", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsResult")]
	InvokeInterfaceRange,
	#[values(value = "0x7b", name = "neg-int", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	NegInt,
	#[values(value = "0x7c", name = "not-int", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	NotInt,
	#[values(value = "0x7d", name = "neg-long", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	NegLong,
	#[values(value = "0x7e", name = "not-long", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	NotLong,
	#[values(value = "0x7f", name = "neg-float", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	NegFloat,
	#[values(value = "0x80", name = "neg-double", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	NegDouble,
	#[values(value = "0x81", name = "int-to-long", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	IntToLong,
	#[values(value = "0x82", name = "int-to-float", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	IntToFloat,
	#[values(value = "0x83", name = "int-to-double", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	IntToDouble,
	#[values(value = "0x84", name = "long-to-int", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	LongToInt,
	#[values(value = "0x85", name = "long-to-float", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	LongToFloat,
	#[values(value = "0x86", name = "long-to-double", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	LongToDouble,
	#[values(value = "0x87", name = "float-to-int", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	FloatToInt,
	#[values(value = "0x88", name = "float-to-long", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	FloatToLong,
	#[values(value = "0x89", name = "float-to-double", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	FloatToDouble,
	#[values(value = "0x8a", name = "double-to-int", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	DoubleToInt,
	#[values(value = "0x8b", name = "double-to-long", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	DoubleToLong,
	#[values(value = "0x8c", name = "double-to-float", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	DoubleToFloat,
	#[values(value = "0x8d", name = "int-to-byte", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	IntToByte,
	#[values(value = "0x8e", name = "int-to-char", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	IntToChar,
	#[values(value = "0x8f", name = "int-to-short", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	IntToShort,
	#[values(value = "0x90", name = "add-int", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	AddInt,
	#[values(value = "0x91", name = "sub-int", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	SubInt,
	#[values(value = "0x92", name = "mul-int", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	MulInt,
	#[values(value = "0x93", name = "div-int", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	DivInt,
	#[values(value = "0x94", name = "rem-int", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	RemInt,
	#[values(value = "0x95", name = "and-int", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	AndInt,
	#[values(value = "0x96", name = "or-int", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	OrInt,
	#[values(value = "0x97", name = "xor-int", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	XorInt,
	#[values(value = "0x98", name = "shl-int", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	ShlInt,
	#[values(value = "0x99", name = "shr-int", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	ShrInt,
	#[values(value = "0x9a", name = "ushr-int", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	UshrInt,
	#[values(value = "0x9b", name = "add-long", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	AddLong,
	#[values(value = "0x9c", name = "sub-long", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	SubLong,
	#[values(value = "0x9d", name = "mul-long", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	MulLong,
	#[values(value = "0x9e", name = "div-long", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	DivLong,
	#[values(value = "0x9f", name = "rem-long", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	RemLong,
	#[values(value = "0xa0", name = "and-long", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	AndLong,
	#[values(value = "0xa1", name = "or-long", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	OrLong,
	#[values(value = "0xa2", name = "xor-long", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	XorLong,
	#[values(value = "0xa3", name = "shl-long", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	ShlLong,
	#[values(value = "0xa4", name = "shr-long", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	ShrLong,
	#[values(value = "0xa5", name = "ushr-long", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	UshrLong,
	#[values(value = "0xa6", name = "add-float", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	AddFloat,
	#[values(value = "0xa7", name = "sub-float", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	SubFloat,
	#[values(value = "0xa8", name = "mul-float", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	MulFloat,
	#[values(value = "0xa9", name = "div-float", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	DivFloat,
	#[values(value = "0xaa", name = "rem-float", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	RemFloat,
	#[values(value = "0xab", name = "add-double", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	AddDouble,
	#[values(value = "0xac", name = "sub-double", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	SubDouble,
	#[values(value = "0xad", name = "mul-double", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	MulDouble,
	#[values(value = "0xae", name = "div-double", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	DivDouble,
	#[values(value = "0xaf", name = "rem-double", reference_type = "ReferenceType::None", format = "Format::Format23x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	RemDouble,
	#[values(value = "0xb0", name = "add-int/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	AddInt2addr,
	#[values(value = "0xb1", name = "sub-int/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	SubInt2addr,
	#[values(value = "0xb2", name = "mul-int/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	MulInt2addr,
	#[values(value = "0xb3", name = "div-int/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	DivInt2addr,
	#[values(value = "0xb4", name = "rem-int/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	RemInt2addr,
	#[values(value = "0xb5", name = "and-int/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	AndInt2addr,
	#[values(value = "0xb6", name = "or-int/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	OrInt2addr,
	#[values(value = "0xb7", name = "xor-int/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	XorInt2addr,
	#[values(value = "0xb8", name = "shl-int/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	ShlInt2addr,
	#[values(value = "0xb9", name = "shr-int/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	ShrInt2addr,
	#[values(value = "0xba", name = "ushr-int/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	UshrInt2addr,
	#[values(value = "0xbb", name = "add-long/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	AddLong2addr,
	#[values(value = "0xbc", name = "sub-long/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	SubLong2addr,
	#[values(value = "0xbd", name = "mul-long/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	MulLong2addr,
	#[values(value = "0xbe", name = "div-long/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	DivLong2addr,
	#[values(value = "0xbf", name = "rem-long/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	RemLong2addr,
	#[values(value = "0xc0", name = "and-long/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	AndLong2addr,
	#[values(value = "0xc1", name = "or-long/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	OrLong2addr,
	#[values(value = "0xc2", name = "xor-long/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	XorLong2addr,
	#[values(value = "0xc3", name = "shl-long/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	ShlLong2addr,
	#[values(value = "0xc4", name = "shr-long/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	ShrLong2addr,
	#[values(value = "0xc5", name = "ushr-long/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	UshrLong2addr,
	#[values(value = "0xc6", name = "add-float/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	AddFloat2addr,
	#[values(value = "0xc7", name = "sub-float/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	SubFloat2addr,
	#[values(value = "0xc8", name = "mul-float/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	MulFloat2addr,
	#[values(value = "0xc9", name = "div-float/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	DivFloat2addr,
	#[values(value = "0xca", name = "rem-float/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	RemFloat2addr,
	#[values(value = "0xcb", name = "add-double/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	AddDouble2addr,
	#[values(value = "0xcc", name = "sub-double/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	SubDouble2addr,
	#[values(value = "0xcd", name = "mul-double/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	MulDouble2addr,
	#[values(value = "0xce", name = "div-double/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	DivDouble2addr,
	#[values(value = "0xcf", name = "rem-double/2addr", reference_type = "ReferenceType::None", format = "Format::Format12x", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	RemDouble2addr,
	#[values(value = "0xd0", name = "add-int/lit16", reference_type = "ReferenceType::None", format = "Format::Format22s", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	AddIntLit16,
	#[values(value = "0xd1", name = "rsub-int", reference_type = "ReferenceType::None", format = "Format::Format22s", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	RsubInt,
	#[values(value = "0xd2", name = "mul-int/lit16", reference_type = "ReferenceType::None", format = "Format::Format22s", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	MulIntLit16,
	#[values(value = "0xd3", name = "div-int/lit16", reference_type = "ReferenceType::None", format = "Format::Format22s", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	DivIntLit16,
	#[values(value = "0xd4", name = "rem-int/lit16", reference_type = "ReferenceType::None", format = "Format::Format22s", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	RemIntLit16,
	#[values(value = "0xd5", name = "and-int/lit16", reference_type = "ReferenceType::None", format = "Format::Format22s", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	AndIntLit16,
	#[values(value = "0xd6", name = "or-int/lit16", reference_type = "ReferenceType::None", format = "Format::Format22s", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	OrIntLit16,
	#[values(value = "0xd7", name = "xor-int/lit16", reference_type = "ReferenceType::None", format = "Format::Format22s", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	XorIntLit16,
	#[values(value = "0xd8", name = "add-int/lit8", reference_type = "ReferenceType::None", format = "Format::Format22b", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	AddIntLit8,
	#[values(value = "0xd9", name = "rsub-int/lit8", reference_type = "ReferenceType::None", format = "Format::Format22b", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	RsubIntLit8,
	#[values(value = "0xda", name = "mul-int/lit8", reference_type = "ReferenceType::None", format = "Format::Format22b", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	MulIntLit8,
	#[values(value = "0xdb", name = "div-int/lit8", reference_type = "ReferenceType::None", format = "Format::Format22b", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	DivIntLit8,
	#[values(value = "0xdc", name = "rem-int/lit8", reference_type = "ReferenceType::None", format = "Format::Format22b", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	RemIntLit8,
	#[values(value = "0xdd", name = "and-int/lit8", reference_type = "ReferenceType::None", format = "Format::Format22b", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	AndIntLit8,
	#[values(value = "0xde", name = "or-int/lit8", reference_type = "ReferenceType::None", format = "Format::Format22b", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	OrIntLit8,
	#[values(value = "0xdf", name = "xor-int/lit8", reference_type = "ReferenceType::None", format = "Format::Format22b", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	XorIntLit8,
	#[values(value = "0xe0", name = "shl-int/lit8", reference_type = "ReferenceType::None", format = "Format::Format22b", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	ShlIntLit8,
	#[values(value = "0xe1", name = "shr-int/lit8", reference_type = "ReferenceType::None", format = "Format::Format22b", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	ShrIntLit8,
	#[values(value = "0xe2", name = "ushr-int/lit8", reference_type = "ReferenceType::None", format = "Format::Format22b", flags = "OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	UshrIntLit8,
	#[values(value = "0xe3", name = "iget-volatile", reference_type = "ReferenceType::Field", format = "Format::Format22c", flags = "OpcodeFlags::OdexOnly | OpcodeFlags::VolatileFieldAccessor | OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	IgetVolatile,
	#[values(value = "0xe4", name = "iput-volatile", reference_type = "ReferenceType::Field", format = "Format::Format22c", flags = "OpcodeFlags::OdexOnly | OpcodeFlags::VolatileFieldAccessor | OpcodeFlags::CanThrow | OpcodeFlags::CanContinue")]
	IputVolatile,
	#[values(value = "0xe5", name = "sget-volatile", reference_type = "ReferenceType::Field", format = "Format::Format21c", flags = "OpcodeFlags::OdexOnly | OpcodeFlags::VolatileFieldAccessor | OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::StaticFieldAccessor")]
	SgetVolatile,
	#[values(value = "0xe6", name = "sput-volatile", reference_type = "ReferenceType::Field", format = "Format::Format21c", flags = "OpcodeFlags::OdexOnly | OpcodeFlags::VolatileFieldAccessor | OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::StaticFieldAccessor")]
	SputVolatile,
	#[values(value = "0xe7", name = "iget-object-volatile", reference_type = "ReferenceType::Field", format = "Format::Format22c", flags = "OpcodeFlags::OdexOnly | OpcodeFlags::VolatileFieldAccessor | OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	IgetObjectVolatile,
	#[values(value = "0xe8", name = "iget-wide-volatile", reference_type = "ReferenceType::Field", format = "Format::Format22c", flags = "OpcodeFlags::OdexOnly | OpcodeFlags::VolatileFieldAccessor | OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister")]
	IgetWideVolatile,
	#[values(value = "0xe9", name = "iput-wide-volatile", reference_type = "ReferenceType::Field", format = "Format::Format22c", flags = "OpcodeFlags::OdexOnly | OpcodeFlags::VolatileFieldAccessor | OpcodeFlags::CanThrow | OpcodeFlags::CanContinue")]
	IputWideVolatile,
	#[values(value = "0xea", name = "sget-wide-volatile", reference_type = "ReferenceType::Field", format = "Format::Format21c", flags = "OpcodeFlags::OdexOnly | OpcodeFlags::VolatileFieldAccessor | OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::SetsWideRegister | OpcodeFlags::StaticFieldAccessor")]
	SgetWideVolatile,
	#[values(value = "0xeb", name = "sput-wide-volatile", reference_type = "ReferenceType::Field", format = "Format::Format21c", flags = "OpcodeFlags::OdexOnly | OpcodeFlags::VolatileFieldAccessor | OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::StaticFieldAccessor")]
	SputWideVolatile,
	#[values(value = "0xed", name = "throw-verification-error", reference_type = "ReferenceType::None", format = "Format::Format20bc", flags = "OpcodeFlags::OdexOnly | OpcodeFlags::CanThrow")]
	ThrowVerificationError,
	#[values(value = "0xee", name = "execute-inline", reference_type = "ReferenceType::None", format = "Format::Format35mi", flags = "OpcodeFlags::OdexOnly | OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsResult")]
	ExecuteInline,
	#[values(value = "0xef", name = "execute-inline/range", reference_type = "ReferenceType::None", format = "Format::Format3rmi", flags = "OpcodeFlags::OdexOnly | OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsResult")]
	ExecuteInlineRange,
	#[values(value = "0xf0", name = "invoke-direct-empty", reference_type = "ReferenceType::Method", format = "Format::Format35c", flags = "OpcodeFlags::OdexOnly | OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsResult | OpcodeFlags::CanInitializeReference")]
	InvokeDirectEmpty,
	#[values(value = "0xf0", name = "invoke-object-init/range", reference_type = "ReferenceType::Method", format = "Format::Format3rc", flags = "OpcodeFlags::OdexOnly | OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsResult | OpcodeFlags::CanInitializeReference")]
	InvokeObjectInitRange,
	#[values(value = "0x73", name = "return-void-no-barrier", reference_type = "ReferenceType::None", format = "Format::Format10x", flags = "OpcodeFlags::OdexOnly")]
	ReturnVoidNoBarrier,
	#[values(value = "0xfa", name = "invoke-super-quick", reference_type = "ReferenceType::None", format = "Format::Format35ms", flags = "OpcodeFlags::OdexOnly | OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsResult")]
	InvokeSuperQuick,
	#[values(value = "0xfb", name = "invoke-super-quick/range", reference_type = "ReferenceType::None", format = "Format::Format3rms", flags = "OpcodeFlags::OdexOnly | OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsResult")]
	InvokeSuperQuickRange,
	#[values(value = "0xfc", name = "iput-object-volatile", reference_type = "ReferenceType::Field", format = "Format::Format22c", flags = "OpcodeFlags::OdexOnly | OpcodeFlags::VolatileFieldAccessor | OpcodeFlags::CanThrow | OpcodeFlags::CanContinue")]
	IputObjectVolatile,
	#[values(value = "0xfd", name = "sget-object-volatile", reference_type = "ReferenceType::Field", format = "Format::Format21c", flags = "OpcodeFlags::OdexOnly | OpcodeFlags::VolatileFieldAccessor | OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister | OpcodeFlags::StaticFieldAccessor")]
	SgetObjectVolatile,
	#[values(value = "0xfe", name = "sput-object-volatile", reference_type = "ReferenceType::Field", format = "Format::Format21c", flags = "OpcodeFlags::OdexOnly | OpcodeFlags::VolatileFieldAccessor | OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::StaticFieldAccessor")]
	SputObjectVolatile,
	#[values(value = "0x100", name = "packed-switch-payload", reference_type = "ReferenceType::None", format = "Format::PackedSwitchPayload")]
	PackedSwitchPayload,
	#[values(value = "0x200", name = "sparse-switch-payload", reference_type = "ReferenceType::None", format = "Format::SparseSwitchPayload")]
	SparseSwitchPayload,
	#[values(value = "0x300", name = "array-payload", reference_type = "ReferenceType::None", format = "Format::ArrayPayload")]
	ArrayPayload,
	#[values(value = "0xfa", name = "invoke-polymorphic", reference_type = "ReferenceType::Method", reference_type_2 = "ReferenceType::MethodProto", format = "Format::Format45cc", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsResult")]
	InvokePolymorphic,
	#[values(value = "0xfb", name = "invoke-polymorphic/range", reference_type = "ReferenceType::Method", reference_type_2 = "ReferenceType::MethodProto", format = "Format::Format4rcc", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsResult")]
	InvokePolymorphicRange,
	#[values(value = "0xfc", name = "invoke-custom", reference_type = "ReferenceType::CallSite", format = "Format::Format35c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsResult")]
	InvokeCustom,
	#[values(value = "0xfd", name = "invoke-custom/range", reference_type = "ReferenceType::CallSite", format = "Format::Format3rc", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsResult")]
	InvokeCustomRange,
	#[values(value = "0xfe", name = "const-method-handle", reference_type = "ReferenceType::MethodHandle", format = "Format::Format21c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	ConstMethodHandle,
	#[values(value = "0xff", name = "const-method-type", reference_type = "ReferenceType::MethodProto", format = "Format::Format21c", flags = "OpcodeFlags::CanThrow | OpcodeFlags::CanContinue | OpcodeFlags::SetsRegister")]
	ConstMethodType,
}

enum ReferenceType {
	String = 0,
	Type = 1,
	Field = 2,
	Method = 3,
	MethodProto = 4,
	CallSite = 5,
	MethodHandle = 6,
	None = 7,
	_Undef = -1,
}

impl Default for ReferenceType {
	fn default() -> Self {
		ReferenceType::_Undef
	}
}

lazy_static! {
  pub static ref VALUE_TO_OPCODE: HashMap<u32, Opcode> = Opcode::gen_value_map();
}

impl Opcode {
	pub fn gen_value_map() -> HashMap<u32, Opcode> {
		let mut map = HashMap::new();
		for opcode in 	Self::all() {
			map.insert(opcode.value(), opcode);
		}
		map
	}

	pub fn all() -> Vec<Opcode> {
		use Opcode::*;

		vec![
			Nop,
			MOVE,
			MoveFrom16,
			Move16,
			MoveWide,
			MoveWideFrom16,
			MoveWide16,
			MoveObject,
			MoveObjectFrom16,
			MoveObject16,
			MoveResult,
			MoveResultWide,
			MoveResultObject,
			MoveException,
			ReturnVoid,
			RETURN,
			ReturnWide,
			ReturnObject,
			Const4,
			Const16,
			CONST,
			ConstHigh16,
			ConstWide16,
			ConstWide32,
			ConstWide,
			ConstWideHigh16,
			ConstString,
			ConstStringJumbo,
			ConstClass,
			MonitorEnter,
			MonitorExit,
			CheckCast,
			InstanceOf,
			ArrayLength,
			NewInstance,
			NewArray,
			FilledNewArray,
			FilledNewArrayRange,
			FillArrayData,
			THROW,
			GOTO,
			Goto16,
			Goto32,
			PackedSwitch,
			SparseSwitch,
			CmplFloat,
			CmpgFloat,
			CmplDouble,
			CmpgDouble,
			CmpLong,
			IfEq,
			IfNe,
			IfLt,
			IfGe,
			IfGt,
			IfLe,
			IfEqz,
			IfNez,
			IfLtz,
			IfGez,
			IfGtz,
			IfLez,
			AGET,
			AgetWide,
			AgetObject,
			AgetBoolean,
			AgetByte,
			AgetChar,
			AgetShort,
			APUT,
			AputWide,
			AputObject,
			AputBoolean,
			AputByte,
			AputChar,
			AputShort,
			IGET,
			IgetWide,
			IgetObject,
			IgetBoolean,
			IgetByte,
			IgetChar,
			IgetShort,
			IPUT,
			IputWide,
			IputObject,
			IputBoolean,
			IputByte,
			IputChar,
			IputShort,
			SGET,
			SgetWide,
			SgetObject,
			SgetBoolean,
			SgetByte,
			SgetChar,
			SgetShort,
			SPUT,
			SputWide,
			SputObject,
			SputBoolean,
			SputByte,
			SputChar,
			SputShort,
			InvokeVirtual,
			InvokeSuper,
			InvokeDirect,
			InvokeStatic,
			InvokeInterface,
			InvokeVirtualRange,
			InvokeSuperRange,
			InvokeDirectRange,
			InvokeStaticRange,
			InvokeInterfaceRange,
			NegInt,
			NotInt,
			NegLong,
			NotLong,
			NegFloat,
			NegDouble,
			IntToLong,
			IntToFloat,
			IntToDouble,
			LongToInt,
			LongToFloat,
			LongToDouble,
			FloatToInt,
			FloatToLong,
			FloatToDouble,
			DoubleToInt,
			DoubleToLong,
			DoubleToFloat,
			IntToByte,
			IntToChar,
			IntToShort,
			AddInt,
			SubInt,
			MulInt,
			DivInt,
			RemInt,
			AndInt,
			OrInt,
			XorInt,
			ShlInt,
			ShrInt,
			UshrInt,
			AddLong,
			SubLong,
			MulLong,
			DivLong,
			RemLong,
			AndLong,
			OrLong,
			XorLong,
			ShlLong,
			ShrLong,
			UshrLong,
			AddFloat,
			SubFloat,
			MulFloat,
			DivFloat,
			RemFloat,
			AddDouble,
			SubDouble,
			MulDouble,
			DivDouble,
			RemDouble,
			AddInt2addr,
			SubInt2addr,
			MulInt2addr,
			DivInt2addr,
			RemInt2addr,
			AndInt2addr,
			OrInt2addr,
			XorInt2addr,
			ShlInt2addr,
			ShrInt2addr,
			UshrInt2addr,
			AddLong2addr,
			SubLong2addr,
			MulLong2addr,
			DivLong2addr,
			RemLong2addr,
			AndLong2addr,
			OrLong2addr,
			XorLong2addr,
			ShlLong2addr,
			ShrLong2addr,
			UshrLong2addr,
			AddFloat2addr,
			SubFloat2addr,
			MulFloat2addr,
			DivFloat2addr,
			RemFloat2addr,
			AddDouble2addr,
			SubDouble2addr,
			MulDouble2addr,
			DivDouble2addr,
			RemDouble2addr,
			AddIntLit16,
			RsubInt,
			MulIntLit16,
			DivIntLit16,
			RemIntLit16,
			AndIntLit16,
			OrIntLit16,
			XorIntLit16,
			AddIntLit8,
			RsubIntLit8,
			MulIntLit8,
			DivIntLit8,
			RemIntLit8,
			AndIntLit8,
			OrIntLit8,
			XorIntLit8,
			ShlIntLit8,
			ShrIntLit8,
			UshrIntLit8,
			IgetVolatile,
			IputVolatile,
			SgetVolatile,
			SputVolatile,
			IgetObjectVolatile,
			IgetWideVolatile,
			IputWideVolatile,
			SgetWideVolatile,
			SputWideVolatile,
			ThrowVerificationError,
			ExecuteInline,
			ExecuteInlineRange,
			InvokeDirectEmpty,
			InvokeObjectInitRange,
			ReturnVoidNoBarrier,
			InvokeSuperQuick,
			InvokeSuperQuickRange,
			IputObjectVolatile,
			SgetObjectVolatile,
			SputObjectVolatile,
			PackedSwitchPayload,
			SparseSwitchPayload,
			ArrayPayload,
			InvokePolymorphic,
			InvokePolymorphicRange,
			InvokeCustom,
			InvokeCustomRange,
			ConstMethodHandle,
			ConstMethodType,
		]
	}
}
