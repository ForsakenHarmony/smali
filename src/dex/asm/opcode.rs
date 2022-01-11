use std::{
	collections::HashMap,
	fmt::{Display, Formatter},
};

use bitflags::bitflags;
use enum_values::EnumValues;
use lazy_static::lazy_static;

use super::format::Format;

bitflags! {
	pub struct OpcodeFlags: u32 {
		//if the instruction can throw an exception
		const CAN_THROW = 0x1; //                 0b00000000001;
		//if the instruction is an odex only instruction
		const ODEX_ONLY = 0x2; //                 0b00000000010;
		//if execution can continue to the next instruction
		const CAN_CONTINUE = 0x4; //              0b00000000100;
		//if the instruction sets the "hidden" result register
		const SETS_RESULT = 0x8; //               0b00000001000;
		//if the instruction sets the value of it's first register
		const SETS_REGISTER = 0x10; //            0b00000010000;
		//if the instruction sets the value of it's first register to a wide type
		const SETS_WIDE_REGISTER = 0x20; //        0b00000100000;
		//if the instruction is an iget-quick/iput-quick instruction
		const QUICK_FIELD_ACCESSOR = 0x40; //      0b00001000000;
		//if the instruction is a *get-volatile/*put-volatile instruction
		const VOLATILE_FIELD_ACCESSOR = 0x80; //   0b00010000000;
		//if the instruction is a static sget-*/sput-*instruction
		const STATIC_FIELD_ACCESSOR = 0x100; //    0b00100000000;
		//if the instruction is a jumbo instruction
		const JUMBO_OPCODE = 0x200; //            0b01000000000;
		//if the instruction can initialize an uninitialized object reference
		const CAN_INITIALIZE_REFERENCE = 0x400; // 0b10000000000;
	}
}

impl Default for OpcodeFlags {
	fn default() -> Self {
		OpcodeFlags::empty()
	}
}

#[derive(EnumValues, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[enum_values(
	value = "u16",
	name = "String",
	reference_type = "ReferenceType",
	reference_type_2 = "ReferenceType",
	format = "Format",
	flags = "OpcodeFlags"
)]
/// https://source.android.com/devices/tech/dalvik/dalvik-bytecode#instructions
pub enum Opcode {
	/// Waste cycles.
	///
	/// `nop`
	#[enum_values(
		value = "0x00",
		name = "nop",
		reference_type = "ReferenceType::None",
		format = "Format::Format10x",
		flags = "OpcodeFlags::CAN_CONTINUE"
	)]
	Nop,
	/// Move the contents of one non-object register to another.
	///
	/// `move vA, vB`
	#[enum_values(
		value = "0x01",
		name = "move",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	Move,
	/// Move the contents of one non-object register to another.
	///
	/// `move/from16 vAA, vBBBB`
	#[enum_values(
		value = "0x02",
		name = "move/from16",
		reference_type = "ReferenceType::None",
		format = "Format::Format22x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	MoveFrom16,
	/// Move the contents of one non-object register to another.
	///
	/// `move/16 vAAAA, vBBBB`
	#[enum_values(
		value = "0x03",
		name = "move/16",
		reference_type = "ReferenceType::None",
		format = "Format::Format32x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	Move16,
	/// Move the contents of one register-pair to another.
	///
	/// `move-wide vA, vB`
	#[enum_values(
		value = "0x04",
		name = "move-wide",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	MoveWide,
	/// Move the contents of one register-pair to another.
	///
	/// `move-wide/from16 vAA, vBBBB`
	#[enum_values(
		value = "0x05",
		name = "move-wide/from16",
		reference_type = "ReferenceType::None",
		format = "Format::Format22x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	MoveWideFrom16,
	/// Move the contents of one register-pair to another.
	///
	/// `move-wide/16 vAAAA, vBBBB`
	#[enum_values(
		value = "0x06",
		name = "move-wide/16",
		reference_type = "ReferenceType::None",
		format = "Format::Format32x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	MoveWide16,
	/// Move the contents of one object-bearing register to another.
	///
	/// `move-object vA, vB`
	#[enum_values(
		value = "0x07",
		name = "move-object",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	MoveObject,
	/// Move the contents of one object-bearing register to another.
	///
	/// `move-object/from16 vAA, vBBBB`
	#[enum_values(
		value = "0x08",
		name = "move-object/from16",
		reference_type = "ReferenceType::None",
		format = "Format::Format22x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	MoveObjectFrom16,
	/// Move the contents of one object-bearing register to another.
	///
	/// `move-object/16 vAAAA, vBBBB`
	#[enum_values(
		value = "0x09",
		name = "move-object/16",
		reference_type = "ReferenceType::None",
		format = "Format::Format32x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	MoveObject16,
	/// Move the single-word non-object result of the most recent invoke-kind into the indicated register. This must be done as the instruction immediately after an invoke-kind whose (single-word, non-object) result is not to be ignored; anywhere else is invalid.
	///
	/// `move-result vAA`
	#[enum_values(
		value = "0x0a",
		name = "move-result",
		reference_type = "ReferenceType::None",
		format = "Format::Format11x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	MoveResult,
	/// Move the double-word result of the most recent invoke-kind into the indicated register pair. This must be done as the instruction immediately after an invoke-kind whose (double-word) result is not to be ignored; anywhere else is invalid.
	///
	/// `move-result-wide vAA`
	#[enum_values(
		value = "0x0b",
		name = "move-result-wide",
		reference_type = "ReferenceType::None",
		format = "Format::Format11x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	MoveResultWide,
	/// Move the object result of the most recent invoke-kind into the indicated register. This must be done as the instruction immediately after an invoke-kind or filled-new-array whose (object) result is not to be ignored; anywhere else is invalid.
	///
	/// `move-result-object vAA`
	#[enum_values(
		value = "0x0c",
		name = "move-result-object",
		reference_type = "ReferenceType::None",
		format = "Format::Format11x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	MoveResultObject,
	/// Save a just-caught exception into the given register. This must be the first instruction of any exception handler whose caught exception is not to be ignored, and this instruction must only ever occur as the first instruction of an exception handler; anywhere else is invalid.
	///
	/// `move-exception vAA`
	#[enum_values(
		value = "0x0d",
		name = "move-exception",
		reference_type = "ReferenceType::None",
		format = "Format::Format11x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	MoveException,
	/// Return from a void method.
	///
	/// `return-void`
	#[enum_values(
		value = "0x0e",
		name = "return-void",
		reference_type = "ReferenceType::None",
		format = "Format::Format10x"
	)]
	ReturnVoid,
	/// Return from a single-width (32-bit) non-object value-returning method.
	///
	/// `return vAA`
	#[enum_values(
		value = "0x0f",
		name = "return",
		reference_type = "ReferenceType::None",
		format = "Format::Format11x"
	)]
	Return,
	/// Return from a double-width (64-bit) value-returning method.
	///
	/// `return-wide vAA`
	#[enum_values(
		value = "0x10",
		name = "return-wide",
		reference_type = "ReferenceType::None",
		format = "Format::Format11x"
	)]
	ReturnWide,
	/// Return from an object-returning method.
	///
	/// `return-object vAA`
	#[enum_values(
		value = "0x11",
		name = "return-object",
		reference_type = "ReferenceType::None",
		format = "Format::Format11x"
	)]
	ReturnObject,
	/// Move the given literal value (sign-extended to 32 bits) into the specified register.
	///
	/// `const/4 vA, #+B`
	#[enum_values(
		value = "0x12",
		name = "const/4",
		reference_type = "ReferenceType::None",
		format = "Format::Format11n",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	Const4,
	/// Move the given literal value (sign-extended to 32 bits) into the specified register.
	///
	/// `const/16 vAA, #+BBBB`
	#[enum_values(
		value = "0x13",
		name = "const/16",
		reference_type = "ReferenceType::None",
		format = "Format::Format21s",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	Const16,
	///
	#[enum_values(
		value = "0x14",
		name = "const",
		reference_type = "ReferenceType::None",
		format = "Format::Format31i",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	CONST,
	#[enum_values(
		value = "0x15",
		name = "const/high16",
		reference_type = "ReferenceType::None",
		format = "Format::Format21ih",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	ConstHigh16,
	#[enum_values(
		value = "0x16",
		name = "const-wide/16",
		reference_type = "ReferenceType::None",
		format = "Format::Format21s",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	ConstWide16,
	#[enum_values(
		value = "0x17",
		name = "const-wide/32",
		reference_type = "ReferenceType::None",
		format = "Format::Format31i",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	ConstWide32,
	#[enum_values(
		value = "0x18",
		name = "const-wide",
		reference_type = "ReferenceType::None",
		format = "Format::Format51l",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	ConstWide,
	#[enum_values(
		value = "0x19",
		name = "const-wide/high16",
		reference_type = "ReferenceType::None",
		format = "Format::Format21lh",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	ConstWideHigh16,
	#[enum_values(
		value = "0x1a",
		name = "const-string",
		reference_type = "ReferenceType::String",
		format = "Format::Format21c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	ConstString,
	#[enum_values(
		value = "0x1b",
		name = "const-string/jumbo",
		reference_type = "ReferenceType::String",
		format = "Format::Format31c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	ConstStringJumbo,
	#[enum_values(
		value = "0x1c",
		name = "const-class",
		reference_type = "ReferenceType::Type",
		format = "Format::Format21c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	ConstClass,
	#[enum_values(
		value = "0x1d",
		name = "monitor-enter",
		reference_type = "ReferenceType::None",
		format = "Format::Format11x",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE"
	)]
	MonitorEnter,
	#[enum_values(
		value = "0x1e",
		name = "monitor-exit",
		reference_type = "ReferenceType::None",
		format = "Format::Format11x",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE"
	)]
	MonitorExit,
	#[enum_values(
		value = "0x1f",
		name = "check-cast",
		reference_type = "ReferenceType::Type",
		format = "Format::Format21c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	CheckCast,
	#[enum_values(
		value = "0x20",
		name = "instance-of",
		reference_type = "ReferenceType::Type",
		format = "Format::Format22c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	InstanceOf,
	#[enum_values(
		value = "0x21",
		name = "array-length",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	ArrayLength,
	#[enum_values(
		value = "0x22",
		name = "new-instance",
		reference_type = "ReferenceType::Type",
		format = "Format::Format21c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	NewInstance,
	#[enum_values(
		value = "0x23",
		name = "new-array",
		reference_type = "ReferenceType::Type",
		format = "Format::Format22c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	NewArray,
	#[enum_values(
		value = "0x24",
		name = "filled-new-array",
		reference_type = "ReferenceType::Type",
		format = "Format::Format35c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_RESULT"
	)]
	FilledNewArray,
	#[enum_values(
		value = "0x25",
		name = "filled-new-array/range",
		reference_type = "ReferenceType::Type",
		format = "Format::Format3rc",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_RESULT"
	)]
	FilledNewArrayRange,
	#[enum_values(
		value = "0x26",
		name = "fill-array-data",
		reference_type = "ReferenceType::None",
		format = "Format::Format31t",
		flags = "OpcodeFlags::CAN_CONTINUE"
	)]
	FillArrayData,
	#[enum_values(
		value = "0x27",
		name = "throw",
		reference_type = "ReferenceType::None",
		format = "Format::Format11x",
		flags = "OpcodeFlags::CAN_THROW"
	)]
	THROW,
	#[enum_values(
		value = "0x28",
		name = "goto",
		reference_type = "ReferenceType::None",
		format = "Format::Format10t"
	)]
	GOTO,
	#[enum_values(
		value = "0x29",
		name = "goto/16",
		reference_type = "ReferenceType::None",
		format = "Format::Format20t"
	)]
	Goto16,
	#[enum_values(
		value = "0x2a",
		name = "goto/32",
		reference_type = "ReferenceType::None",
		format = "Format::Format30t"
	)]
	Goto32,
	#[enum_values(
		value = "0x2b",
		name = "packed-switch",
		reference_type = "ReferenceType::None",
		format = "Format::Format31t",
		flags = "OpcodeFlags::CAN_CONTINUE"
	)]
	PackedSwitch,
	#[enum_values(
		value = "0x2c",
		name = "sparse-switch",
		reference_type = "ReferenceType::None",
		format = "Format::Format31t",
		flags = "OpcodeFlags::CAN_CONTINUE"
	)]
	SparseSwitch,
	#[enum_values(
		value = "0x2d",
		name = "cmpl-float",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	CmplFloat,
	#[enum_values(
		value = "0x2e",
		name = "cmpg-float",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	CmpgFloat,
	#[enum_values(
		value = "0x2f",
		name = "cmpl-double",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	CmplDouble,
	#[enum_values(
		value = "0x30",
		name = "cmpg-double",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	CmpgDouble,
	#[enum_values(
		value = "0x31",
		name = "cmp-long",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	CmpLong,
	#[enum_values(
		value = "0x32",
		name = "if-eq",
		reference_type = "ReferenceType::None",
		format = "Format::Format22t",
		flags = "OpcodeFlags::CAN_CONTINUE"
	)]
	IfEq,
	#[enum_values(
		value = "0x33",
		name = "if-ne",
		reference_type = "ReferenceType::None",
		format = "Format::Format22t",
		flags = "OpcodeFlags::CAN_CONTINUE"
	)]
	IfNe,
	#[enum_values(
		value = "0x34",
		name = "if-lt",
		reference_type = "ReferenceType::None",
		format = "Format::Format22t",
		flags = "OpcodeFlags::CAN_CONTINUE"
	)]
	IfLt,
	#[enum_values(
		value = "0x35",
		name = "if-ge",
		reference_type = "ReferenceType::None",
		format = "Format::Format22t",
		flags = "OpcodeFlags::CAN_CONTINUE"
	)]
	IfGe,
	#[enum_values(
		value = "0x36",
		name = "if-gt",
		reference_type = "ReferenceType::None",
		format = "Format::Format22t",
		flags = "OpcodeFlags::CAN_CONTINUE"
	)]
	IfGt,
	#[enum_values(
		value = "0x37",
		name = "if-le",
		reference_type = "ReferenceType::None",
		format = "Format::Format22t",
		flags = "OpcodeFlags::CAN_CONTINUE"
	)]
	IfLe,
	#[enum_values(
		value = "0x38",
		name = "if-eqz",
		reference_type = "ReferenceType::None",
		format = "Format::Format21t",
		flags = "OpcodeFlags::CAN_CONTINUE"
	)]
	IfEqz,
	#[enum_values(
		value = "0x39",
		name = "if-nez",
		reference_type = "ReferenceType::None",
		format = "Format::Format21t",
		flags = "OpcodeFlags::CAN_CONTINUE"
	)]
	IfNez,
	#[enum_values(
		value = "0x3a",
		name = "if-ltz",
		reference_type = "ReferenceType::None",
		format = "Format::Format21t",
		flags = "OpcodeFlags::CAN_CONTINUE"
	)]
	IfLtz,
	#[enum_values(
		value = "0x3b",
		name = "if-gez",
		reference_type = "ReferenceType::None",
		format = "Format::Format21t",
		flags = "OpcodeFlags::CAN_CONTINUE"
	)]
	IfGez,
	#[enum_values(
		value = "0x3c",
		name = "if-gtz",
		reference_type = "ReferenceType::None",
		format = "Format::Format21t",
		flags = "OpcodeFlags::CAN_CONTINUE"
	)]
	IfGtz,
	#[enum_values(
		value = "0x3d",
		name = "if-lez",
		reference_type = "ReferenceType::None",
		format = "Format::Format21t",
		flags = "OpcodeFlags::CAN_CONTINUE"
	)]
	IfLez,
	#[enum_values(
		value = "0x44",
		name = "aget",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	AGET,
	#[enum_values(
		value = "0x45",
		name = "aget-wide",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	AgetWide,
	#[enum_values(
		value = "0x46",
		name = "aget-object",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	AgetObject,
	#[enum_values(
		value = "0x47",
		name = "aget-boolean",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	AgetBoolean,
	#[enum_values(
		value = "0x48",
		name = "aget-byte",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	AgetByte,
	#[enum_values(
		value = "0x49",
		name = "aget-char",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	AgetChar,
	#[enum_values(
		value = "0x4a",
		name = "aget-short",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	AgetShort,
	#[enum_values(
		value = "0x4b",
		name = "aput",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE"
	)]
	APUT,
	#[enum_values(
		value = "0x4c",
		name = "aput-wide",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE"
	)]
	AputWide,
	#[enum_values(
		value = "0x4d",
		name = "aput-object",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE"
	)]
	AputObject,
	#[enum_values(
		value = "0x4e",
		name = "aput-boolean",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE"
	)]
	AputBoolean,
	#[enum_values(
		value = "0x4f",
		name = "aput-byte",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE"
	)]
	AputByte,
	#[enum_values(
		value = "0x50",
		name = "aput-char",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE"
	)]
	AputChar,
	#[enum_values(
		value = "0x51",
		name = "aput-short",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE"
	)]
	AputShort,
	#[enum_values(
		value = "0x52",
		name = "iget",
		reference_type = "ReferenceType::Field",
		format = "Format::Format22c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	IGET,
	#[enum_values(
		value = "0x53",
		name = "iget-wide",
		reference_type = "ReferenceType::Field",
		format = "Format::Format22c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	IgetWide,
	#[enum_values(
		value = "0x54",
		name = "iget-object",
		reference_type = "ReferenceType::Field",
		format = "Format::Format22c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	IgetObject,
	#[enum_values(
		value = "0x55",
		name = "iget-boolean",
		reference_type = "ReferenceType::Field",
		format = "Format::Format22c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	IgetBoolean,
	#[enum_values(
		value = "0x56",
		name = "iget-byte",
		reference_type = "ReferenceType::Field",
		format = "Format::Format22c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	IgetByte,
	#[enum_values(
		value = "0x57",
		name = "iget-char",
		reference_type = "ReferenceType::Field",
		format = "Format::Format22c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	IgetChar,
	#[enum_values(
		value = "0x58",
		name = "iget-short",
		reference_type = "ReferenceType::Field",
		format = "Format::Format22c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	IgetShort,
	#[enum_values(
		value = "0x59",
		name = "iput",
		reference_type = "ReferenceType::Field",
		format = "Format::Format22c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE"
	)]
	IPUT,
	#[enum_values(
		value = "0x5a",
		name = "iput-wide",
		reference_type = "ReferenceType::Field",
		format = "Format::Format22c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE"
	)]
	IputWide,
	#[enum_values(
		value = "0x5b",
		name = "iput-object",
		reference_type = "ReferenceType::Field",
		format = "Format::Format22c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE"
	)]
	IputObject,
	#[enum_values(
		value = "0x5c",
		name = "iput-boolean",
		reference_type = "ReferenceType::Field",
		format = "Format::Format22c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE"
	)]
	IputBoolean,
	#[enum_values(
		value = "0x5d",
		name = "iput-byte",
		reference_type = "ReferenceType::Field",
		format = "Format::Format22c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE"
	)]
	IputByte,
	#[enum_values(
		value = "0x5e",
		name = "iput-char",
		reference_type = "ReferenceType::Field",
		format = "Format::Format22c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE"
	)]
	IputChar,
	#[enum_values(
		value = "0x5f",
		name = "iput-short",
		reference_type = "ReferenceType::Field",
		format = "Format::Format22c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE"
	)]
	IputShort,
	#[enum_values(
		value = "0x60",
		name = "sget",
		reference_type = "ReferenceType::Field",
		format = "Format::Format21c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::STATIC_FIELD_ACCESSOR"
	)]
	SGET,
	#[enum_values(
		value = "0x61",
		name = "sget-wide",
		reference_type = "ReferenceType::Field",
		format = "Format::Format21c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER | OpcodeFlags::STATIC_FIELD_ACCESSOR"
	)]
	SgetWide,
	#[enum_values(
		value = "0x62",
		name = "sget-object",
		reference_type = "ReferenceType::Field",
		format = "Format::Format21c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::STATIC_FIELD_ACCESSOR"
	)]
	SgetObject,
	#[enum_values(
		value = "0x63",
		name = "sget-boolean",
		reference_type = "ReferenceType::Field",
		format = "Format::Format21c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::STATIC_FIELD_ACCESSOR"
	)]
	SgetBoolean,
	#[enum_values(
		value = "0x64",
		name = "sget-byte",
		reference_type = "ReferenceType::Field",
		format = "Format::Format21c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::STATIC_FIELD_ACCESSOR"
	)]
	SgetByte,
	#[enum_values(
		value = "0x65",
		name = "sget-char",
		reference_type = "ReferenceType::Field",
		format = "Format::Format21c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::STATIC_FIELD_ACCESSOR"
	)]
	SgetChar,
	#[enum_values(
		value = "0x66",
		name = "sget-short",
		reference_type = "ReferenceType::Field",
		format = "Format::Format21c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::STATIC_FIELD_ACCESSOR"
	)]
	SgetShort,
	#[enum_values(
		value = "0x67",
		name = "sput",
		reference_type = "ReferenceType::Field",
		format = "Format::Format21c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::STATIC_FIELD_ACCESSOR"
	)]
	SPUT,
	#[enum_values(
		value = "0x68",
		name = "sput-wide",
		reference_type = "ReferenceType::Field",
		format = "Format::Format21c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::STATIC_FIELD_ACCESSOR"
	)]
	SputWide,
	#[enum_values(
		value = "0x69",
		name = "sput-object",
		reference_type = "ReferenceType::Field",
		format = "Format::Format21c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::STATIC_FIELD_ACCESSOR"
	)]
	SputObject,
	#[enum_values(
		value = "0x6a",
		name = "sput-boolean",
		reference_type = "ReferenceType::Field",
		format = "Format::Format21c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::STATIC_FIELD_ACCESSOR"
	)]
	SputBoolean,
	#[enum_values(
		value = "0x6b",
		name = "sput-byte",
		reference_type = "ReferenceType::Field",
		format = "Format::Format21c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::STATIC_FIELD_ACCESSOR"
	)]
	SputByte,
	#[enum_values(
		value = "0x6c",
		name = "sput-char",
		reference_type = "ReferenceType::Field",
		format = "Format::Format21c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::STATIC_FIELD_ACCESSOR"
	)]
	SputChar,
	#[enum_values(
		value = "0x6d",
		name = "sput-short",
		reference_type = "ReferenceType::Field",
		format = "Format::Format21c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::STATIC_FIELD_ACCESSOR"
	)]
	SputShort,
	#[enum_values(
		value = "0x6e",
		name = "invoke-virtual",
		reference_type = "ReferenceType::Method",
		format = "Format::Format35c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_RESULT"
	)]
	InvokeVirtual,
	#[enum_values(
		value = "0x6f",
		name = "invoke-super",
		reference_type = "ReferenceType::Method",
		format = "Format::Format35c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_RESULT"
	)]
	InvokeSuper,
	#[enum_values(
		value = "0x70",
		name = "invoke-direct",
		reference_type = "ReferenceType::Method",
		format = "Format::Format35c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_RESULT | OpcodeFlags::CAN_INITIALIZE_REFERENCE"
	)]
	InvokeDirect,
	#[enum_values(
		value = "0x71",
		name = "invoke-static",
		reference_type = "ReferenceType::Method",
		format = "Format::Format35c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_RESULT"
	)]
	InvokeStatic,
	#[enum_values(
		value = "0x72",
		name = "invoke-interface",
		reference_type = "ReferenceType::Method",
		format = "Format::Format35c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_RESULT"
	)]
	InvokeInterface,
	#[enum_values(
		value = "0x74",
		name = "invoke-virtual/range",
		reference_type = "ReferenceType::Method",
		format = "Format::Format3rc",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_RESULT"
	)]
	InvokeVirtualRange,
	#[enum_values(
		value = "0x75",
		name = "invoke-super/range",
		reference_type = "ReferenceType::Method",
		format = "Format::Format3rc",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_RESULT"
	)]
	InvokeSuperRange,
	#[enum_values(
		value = "0x76",
		name = "invoke-direct/range",
		reference_type = "ReferenceType::Method",
		format = "Format::Format3rc",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_RESULT | OpcodeFlags::CAN_INITIALIZE_REFERENCE"
	)]
	InvokeDirectRange,
	#[enum_values(
		value = "0x77",
		name = "invoke-static/range",
		reference_type = "ReferenceType::Method",
		format = "Format::Format3rc",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_RESULT"
	)]
	InvokeStaticRange,
	#[enum_values(
		value = "0x78",
		name = "invoke-interface/range",
		reference_type = "ReferenceType::Method",
		format = "Format::Format3rc",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_RESULT"
	)]
	InvokeInterfaceRange,
	#[enum_values(
		value = "0x7b",
		name = "neg-int",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	NegInt,
	#[enum_values(
		value = "0x7c",
		name = "not-int",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	NotInt,
	#[enum_values(
		value = "0x7d",
		name = "neg-long",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	NegLong,
	#[enum_values(
		value = "0x7e",
		name = "not-long",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	NotLong,
	#[enum_values(
		value = "0x7f",
		name = "neg-float",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	NegFloat,
	#[enum_values(
		value = "0x80",
		name = "neg-double",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	NegDouble,
	#[enum_values(
		value = "0x81",
		name = "int-to-long",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	IntToLong,
	#[enum_values(
		value = "0x82",
		name = "int-to-float",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	IntToFloat,
	#[enum_values(
		value = "0x83",
		name = "int-to-double",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	IntToDouble,
	#[enum_values(
		value = "0x84",
		name = "long-to-int",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	LongToInt,
	#[enum_values(
		value = "0x85",
		name = "long-to-float",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	LongToFloat,
	#[enum_values(
		value = "0x86",
		name = "long-to-double",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	LongToDouble,
	#[enum_values(
		value = "0x87",
		name = "float-to-int",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	FloatToInt,
	#[enum_values(
		value = "0x88",
		name = "float-to-long",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	FloatToLong,
	#[enum_values(
		value = "0x89",
		name = "float-to-double",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	FloatToDouble,
	#[enum_values(
		value = "0x8a",
		name = "double-to-int",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	DoubleToInt,
	#[enum_values(
		value = "0x8b",
		name = "double-to-long",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	DoubleToLong,
	#[enum_values(
		value = "0x8c",
		name = "double-to-float",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	DoubleToFloat,
	#[enum_values(
		value = "0x8d",
		name = "int-to-byte",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	IntToByte,
	#[enum_values(
		value = "0x8e",
		name = "int-to-char",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	IntToChar,
	#[enum_values(
		value = "0x8f",
		name = "int-to-short",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	IntToShort,
	#[enum_values(
		value = "0x90",
		name = "add-int",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	AddInt,
	#[enum_values(
		value = "0x91",
		name = "sub-int",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	SubInt,
	#[enum_values(
		value = "0x92",
		name = "mul-int",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	MulInt,
	#[enum_values(
		value = "0x93",
		name = "div-int",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	DivInt,
	#[enum_values(
		value = "0x94",
		name = "rem-int",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	RemInt,
	#[enum_values(
		value = "0x95",
		name = "and-int",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	AndInt,
	#[enum_values(
		value = "0x96",
		name = "or-int",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	OrInt,
	#[enum_values(
		value = "0x97",
		name = "xor-int",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	XorInt,
	#[enum_values(
		value = "0x98",
		name = "shl-int",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	ShlInt,
	#[enum_values(
		value = "0x99",
		name = "shr-int",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	ShrInt,
	#[enum_values(
		value = "0x9a",
		name = "ushr-int",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	UshrInt,
	#[enum_values(
		value = "0x9b",
		name = "add-long",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	AddLong,
	#[enum_values(
		value = "0x9c",
		name = "sub-long",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	SubLong,
	#[enum_values(
		value = "0x9d",
		name = "mul-long",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	MulLong,
	#[enum_values(
		value = "0x9e",
		name = "div-long",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	DivLong,
	#[enum_values(
		value = "0x9f",
		name = "rem-long",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	RemLong,
	#[enum_values(
		value = "0xa0",
		name = "and-long",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	AndLong,
	#[enum_values(
		value = "0xa1",
		name = "or-long",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	OrLong,
	#[enum_values(
		value = "0xa2",
		name = "xor-long",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	XorLong,
	#[enum_values(
		value = "0xa3",
		name = "shl-long",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	ShlLong,
	#[enum_values(
		value = "0xa4",
		name = "shr-long",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	ShrLong,
	#[enum_values(
		value = "0xa5",
		name = "ushr-long",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	UshrLong,
	#[enum_values(
		value = "0xa6",
		name = "add-float",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	AddFloat,
	#[enum_values(
		value = "0xa7",
		name = "sub-float",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	SubFloat,
	#[enum_values(
		value = "0xa8",
		name = "mul-float",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	MulFloat,
	#[enum_values(
		value = "0xa9",
		name = "div-float",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	DivFloat,
	#[enum_values(
		value = "0xaa",
		name = "rem-float",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	RemFloat,
	#[enum_values(
		value = "0xab",
		name = "add-double",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	AddDouble,
	#[enum_values(
		value = "0xac",
		name = "sub-double",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	SubDouble,
	#[enum_values(
		value = "0xad",
		name = "mul-double",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	MulDouble,
	#[enum_values(
		value = "0xae",
		name = "div-double",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	DivDouble,
	#[enum_values(
		value = "0xaf",
		name = "rem-double",
		reference_type = "ReferenceType::None",
		format = "Format::Format23x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	RemDouble,
	#[enum_values(
		value = "0xb0",
		name = "add-int/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	AddInt2addr,
	#[enum_values(
		value = "0xb1",
		name = "sub-int/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	SubInt2addr,
	#[enum_values(
		value = "0xb2",
		name = "mul-int/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	MulInt2addr,
	#[enum_values(
		value = "0xb3",
		name = "div-int/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	DivInt2addr,
	#[enum_values(
		value = "0xb4",
		name = "rem-int/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	RemInt2addr,
	#[enum_values(
		value = "0xb5",
		name = "and-int/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	AndInt2addr,
	#[enum_values(
		value = "0xb6",
		name = "or-int/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	OrInt2addr,
	#[enum_values(
		value = "0xb7",
		name = "xor-int/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	XorInt2addr,
	#[enum_values(
		value = "0xb8",
		name = "shl-int/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	ShlInt2addr,
	#[enum_values(
		value = "0xb9",
		name = "shr-int/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	ShrInt2addr,
	#[enum_values(
		value = "0xba",
		name = "ushr-int/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	UshrInt2addr,
	#[enum_values(
		value = "0xbb",
		name = "add-long/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	AddLong2addr,
	#[enum_values(
		value = "0xbc",
		name = "sub-long/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	SubLong2addr,
	#[enum_values(
		value = "0xbd",
		name = "mul-long/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	MulLong2addr,
	#[enum_values(
		value = "0xbe",
		name = "div-long/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	DivLong2addr,
	#[enum_values(
		value = "0xbf",
		name = "rem-long/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	RemLong2addr,
	#[enum_values(
		value = "0xc0",
		name = "and-long/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	AndLong2addr,
	#[enum_values(
		value = "0xc1",
		name = "or-long/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	OrLong2addr,
	#[enum_values(
		value = "0xc2",
		name = "xor-long/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	XorLong2addr,
	#[enum_values(
		value = "0xc3",
		name = "shl-long/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	ShlLong2addr,
	#[enum_values(
		value = "0xc4",
		name = "shr-long/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	ShrLong2addr,
	#[enum_values(
		value = "0xc5",
		name = "ushr-long/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	UshrLong2addr,
	#[enum_values(
		value = "0xc6",
		name = "add-float/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	AddFloat2addr,
	#[enum_values(
		value = "0xc7",
		name = "sub-float/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	SubFloat2addr,
	#[enum_values(
		value = "0xc8",
		name = "mul-float/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	MulFloat2addr,
	#[enum_values(
		value = "0xc9",
		name = "div-float/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	DivFloat2addr,
	#[enum_values(
		value = "0xca",
		name = "rem-float/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	RemFloat2addr,
	#[enum_values(
		value = "0xcb",
		name = "add-double/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	AddDouble2addr,
	#[enum_values(
		value = "0xcc",
		name = "sub-double/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	SubDouble2addr,
	#[enum_values(
		value = "0xcd",
		name = "mul-double/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	MulDouble2addr,
	#[enum_values(
		value = "0xce",
		name = "div-double/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	DivDouble2addr,
	#[enum_values(
		value = "0xcf",
		name = "rem-double/2addr",
		reference_type = "ReferenceType::None",
		format = "Format::Format12x",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	RemDouble2addr,
	#[enum_values(
		value = "0xd0",
		name = "add-int/lit16",
		reference_type = "ReferenceType::None",
		format = "Format::Format22s",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	AddIntLit16,
	#[enum_values(
		value = "0xd1",
		name = "rsub-int",
		reference_type = "ReferenceType::None",
		format = "Format::Format22s",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	RsubInt,
	#[enum_values(
		value = "0xd2",
		name = "mul-int/lit16",
		reference_type = "ReferenceType::None",
		format = "Format::Format22s",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	MulIntLit16,
	#[enum_values(
		value = "0xd3",
		name = "div-int/lit16",
		reference_type = "ReferenceType::None",
		format = "Format::Format22s",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	DivIntLit16,
	#[enum_values(
		value = "0xd4",
		name = "rem-int/lit16",
		reference_type = "ReferenceType::None",
		format = "Format::Format22s",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	RemIntLit16,
	#[enum_values(
		value = "0xd5",
		name = "and-int/lit16",
		reference_type = "ReferenceType::None",
		format = "Format::Format22s",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	AndIntLit16,
	#[enum_values(
		value = "0xd6",
		name = "or-int/lit16",
		reference_type = "ReferenceType::None",
		format = "Format::Format22s",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	OrIntLit16,
	#[enum_values(
		value = "0xd7",
		name = "xor-int/lit16",
		reference_type = "ReferenceType::None",
		format = "Format::Format22s",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	XorIntLit16,
	#[enum_values(
		value = "0xd8",
		name = "add-int/lit8",
		reference_type = "ReferenceType::None",
		format = "Format::Format22b",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	AddIntLit8,
	#[enum_values(
		value = "0xd9",
		name = "rsub-int/lit8",
		reference_type = "ReferenceType::None",
		format = "Format::Format22b",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	RsubIntLit8,
	#[enum_values(
		value = "0xda",
		name = "mul-int/lit8",
		reference_type = "ReferenceType::None",
		format = "Format::Format22b",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	MulIntLit8,
	#[enum_values(
		value = "0xdb",
		name = "div-int/lit8",
		reference_type = "ReferenceType::None",
		format = "Format::Format22b",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	DivIntLit8,
	#[enum_values(
		value = "0xdc",
		name = "rem-int/lit8",
		reference_type = "ReferenceType::None",
		format = "Format::Format22b",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	RemIntLit8,
	#[enum_values(
		value = "0xdd",
		name = "and-int/lit8",
		reference_type = "ReferenceType::None",
		format = "Format::Format22b",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	AndIntLit8,
	#[enum_values(
		value = "0xde",
		name = "or-int/lit8",
		reference_type = "ReferenceType::None",
		format = "Format::Format22b",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	OrIntLit8,
	#[enum_values(
		value = "0xdf",
		name = "xor-int/lit8",
		reference_type = "ReferenceType::None",
		format = "Format::Format22b",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	XorIntLit8,
	#[enum_values(
		value = "0xe0",
		name = "shl-int/lit8",
		reference_type = "ReferenceType::None",
		format = "Format::Format22b",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	ShlIntLit8,
	#[enum_values(
		value = "0xe1",
		name = "shr-int/lit8",
		reference_type = "ReferenceType::None",
		format = "Format::Format22b",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	ShrIntLit8,
	#[enum_values(
		value = "0xe2",
		name = "ushr-int/lit8",
		reference_type = "ReferenceType::None",
		format = "Format::Format22b",
		flags = "OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	UshrIntLit8,
	#[enum_values(
		value = "0xe3",
		name = "iget-volatile",
		reference_type = "ReferenceType::Field",
		format = "Format::Format22c",
		flags = "OpcodeFlags::ODEX_ONLY | OpcodeFlags::VOLATILE_FIELD_ACCESSOR | OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	IgetVolatile,
	#[enum_values(
		value = "0xe4",
		name = "iput-volatile",
		reference_type = "ReferenceType::Field",
		format = "Format::Format22c",
		flags = "OpcodeFlags::ODEX_ONLY | OpcodeFlags::VOLATILE_FIELD_ACCESSOR | OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE"
	)]
	IputVolatile,
	#[enum_values(
		value = "0xe5",
		name = "sget-volatile",
		reference_type = "ReferenceType::Field",
		format = "Format::Format21c",
		flags = "OpcodeFlags::ODEX_ONLY | OpcodeFlags::VOLATILE_FIELD_ACCESSOR | OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::STATIC_FIELD_ACCESSOR"
	)]
	SgetVolatile,
	#[enum_values(
		value = "0xe6",
		name = "sput-volatile",
		reference_type = "ReferenceType::Field",
		format = "Format::Format21c",
		flags = "OpcodeFlags::ODEX_ONLY | OpcodeFlags::VOLATILE_FIELD_ACCESSOR | OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::STATIC_FIELD_ACCESSOR"
	)]
	SputVolatile,
	#[enum_values(
		value = "0xe7",
		name = "iget-object-volatile",
		reference_type = "ReferenceType::Field",
		format = "Format::Format22c",
		flags = "OpcodeFlags::ODEX_ONLY | OpcodeFlags::VOLATILE_FIELD_ACCESSOR | OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	IgetObjectVolatile,
	#[enum_values(
		value = "0xe8",
		name = "iget-wide-volatile",
		reference_type = "ReferenceType::Field",
		format = "Format::Format22c",
		flags = "OpcodeFlags::ODEX_ONLY | OpcodeFlags::VOLATILE_FIELD_ACCESSOR | OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER"
	)]
	IgetWideVolatile,
	#[enum_values(
		value = "0xe9",
		name = "iput-wide-volatile",
		reference_type = "ReferenceType::Field",
		format = "Format::Format22c",
		flags = "OpcodeFlags::ODEX_ONLY | OpcodeFlags::VOLATILE_FIELD_ACCESSOR | OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE"
	)]
	IputWideVolatile,
	#[enum_values(
		value = "0xea",
		name = "sget-wide-volatile",
		reference_type = "ReferenceType::Field",
		format = "Format::Format21c",
		flags = "OpcodeFlags::ODEX_ONLY | OpcodeFlags::VOLATILE_FIELD_ACCESSOR | OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::SETS_WIDE_REGISTER | OpcodeFlags::STATIC_FIELD_ACCESSOR"
	)]
	SgetWideVolatile,
	#[enum_values(
		value = "0xeb",
		name = "sput-wide-volatile",
		reference_type = "ReferenceType::Field",
		format = "Format::Format21c",
		flags = "OpcodeFlags::ODEX_ONLY | OpcodeFlags::VOLATILE_FIELD_ACCESSOR | OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::STATIC_FIELD_ACCESSOR"
	)]
	SputWideVolatile,
	#[enum_values(
		value = "0xed",
		name = "throw-verification-error",
		reference_type = "ReferenceType::None",
		format = "Format::Format20bc",
		flags = "OpcodeFlags::ODEX_ONLY | OpcodeFlags::CAN_THROW"
	)]
	ThrowVerificationError,
	#[enum_values(
		value = "0xee",
		name = "execute-inline",
		reference_type = "ReferenceType::None",
		format = "Format::Format35mi",
		flags = "OpcodeFlags::ODEX_ONLY | OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_RESULT"
	)]
	ExecuteInline,
	#[enum_values(
		value = "0xef",
		name = "execute-inline/range",
		reference_type = "ReferenceType::None",
		format = "Format::Format3rmi",
		flags = "OpcodeFlags::ODEX_ONLY | OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_RESULT"
	)]
	ExecuteInlineRange,
	#[enum_values(
		value = "0xf0",
		name = "invoke-direct-empty",
		reference_type = "ReferenceType::Method",
		format = "Format::Format35c",
		flags = "OpcodeFlags::ODEX_ONLY | OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_RESULT | OpcodeFlags::CAN_INITIALIZE_REFERENCE"
	)]
	InvokeDirectEmpty,
	#[enum_values(
		value = "0xf0",
		name = "invoke-object-init/range",
		reference_type = "ReferenceType::Method",
		format = "Format::Format3rc",
		flags = "OpcodeFlags::ODEX_ONLY | OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_RESULT | OpcodeFlags::CAN_INITIALIZE_REFERENCE"
	)]
	InvokeObjectInitRange,
	#[enum_values(
		value = "0x73",
		name = "return-void-no-barrier",
		reference_type = "ReferenceType::None",
		format = "Format::Format10x",
		flags = "OpcodeFlags::ODEX_ONLY"
	)]
	ReturnVoidNoBarrier,
	#[enum_values(
		value = "0xfa",
		name = "invoke-super-quick",
		reference_type = "ReferenceType::None",
		format = "Format::Format35ms",
		flags = "OpcodeFlags::ODEX_ONLY | OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_RESULT"
	)]
	InvokeSuperQuick,
	#[enum_values(
		value = "0xfb",
		name = "invoke-super-quick/range",
		reference_type = "ReferenceType::None",
		format = "Format::Format3rms",
		flags = "OpcodeFlags::ODEX_ONLY | OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_RESULT"
	)]
	InvokeSuperQuickRange,
	#[enum_values(
		value = "0xfc",
		name = "iput-object-volatile",
		reference_type = "ReferenceType::Field",
		format = "Format::Format22c",
		flags = "OpcodeFlags::ODEX_ONLY | OpcodeFlags::VOLATILE_FIELD_ACCESSOR | OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE"
	)]
	IputObjectVolatile,
	#[enum_values(
		value = "0xfd",
		name = "sget-object-volatile",
		reference_type = "ReferenceType::Field",
		format = "Format::Format21c",
		flags = "OpcodeFlags::ODEX_ONLY | OpcodeFlags::VOLATILE_FIELD_ACCESSOR | OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER | OpcodeFlags::STATIC_FIELD_ACCESSOR"
	)]
	SgetObjectVolatile,
	#[enum_values(
		value = "0xfe",
		name = "sput-object-volatile",
		reference_type = "ReferenceType::Field",
		format = "Format::Format21c",
		flags = "OpcodeFlags::ODEX_ONLY | OpcodeFlags::VOLATILE_FIELD_ACCESSOR | OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::STATIC_FIELD_ACCESSOR"
	)]
	SputObjectVolatile,
	#[enum_values(
		value = "0x100",
		name = "packed-switch-payload",
		reference_type = "ReferenceType::None",
		format = "Format::PackedSwitchPayload"
	)]
	PackedSwitchPayload,
	#[enum_values(
		value = "0x200",
		name = "sparse-switch-payload",
		reference_type = "ReferenceType::None",
		format = "Format::SparseSwitchPayload"
	)]
	SparseSwitchPayload,
	#[enum_values(
		value = "0x300",
		name = "array-payload",
		reference_type = "ReferenceType::None",
		format = "Format::ArrayPayload"
	)]
	ArrayPayload,
	#[enum_values(
		value = "0xfa",
		name = "invoke-polymorphic",
		reference_type = "ReferenceType::Method",
		reference_type_2 = "ReferenceType::MethodProto",
		format = "Format::Format45cc",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_RESULT"
	)]
	InvokePolymorphic,
	#[enum_values(
		value = "0xfb",
		name = "invoke-polymorphic/range",
		reference_type = "ReferenceType::Method",
		reference_type_2 = "ReferenceType::MethodProto",
		format = "Format::Format4rcc",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_RESULT"
	)]
	InvokePolymorphicRange,
	#[enum_values(
		value = "0xfc",
		name = "invoke-custom",
		reference_type = "ReferenceType::CallSite",
		format = "Format::Format35c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_RESULT"
	)]
	InvokeCustom,
	#[enum_values(
		value = "0xfd",
		name = "invoke-custom/range",
		reference_type = "ReferenceType::CallSite",
		format = "Format::Format3rc",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_RESULT"
	)]
	InvokeCustomRange,
	#[enum_values(
		value = "0xfe",
		name = "const-method-handle",
		reference_type = "ReferenceType::MethodHandle",
		format = "Format::Format21c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	ConstMethodHandle,
	#[enum_values(
		value = "0xff",
		name = "const-method-type",
		reference_type = "ReferenceType::MethodProto",
		format = "Format::Format21c",
		flags = "OpcodeFlags::CAN_THROW | OpcodeFlags::CAN_CONTINUE | OpcodeFlags::SETS_REGISTER"
	)]
	ConstMethodType,
}

pub enum ReferenceType {
	String       = 0,
	Type         = 1,
	Field        = 2,
	Method       = 3,
	MethodProto  = 4,
	CallSite     = 5,
	MethodHandle = 6,
	None         = 7,
	_Undef       = -1,
}

impl Default for ReferenceType {
	fn default() -> Self {
		ReferenceType::_Undef
	}
}

lazy_static! {
	pub static ref VALUE_TO_OPCODE: HashMap<u16, Opcode> = Opcode::gen_value_map();
}

impl Opcode {
	pub fn gen_value_map() -> HashMap<u16, Opcode> {
		let mut map = HashMap::new();
		for opcode in Self::all() {
			map.insert(opcode.value(), opcode);
		}
		map
	}

	pub fn all() -> Vec<Opcode> {
		use Opcode::*;

		vec![
			Nop,
			Move,
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
			Return,
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

impl Display for Opcode {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}({})", self.name(), self.format())
	}
}
