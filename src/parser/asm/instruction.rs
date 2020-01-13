use crate::parser::asm::opcode::Opcode;

trait Instruction {
	fn opcode(&self) -> &Opcode;
	fn code_units(&self) -> u32 {
		self.opcode().format().size()
	}
}

trait OneRegisterInstruction where Self: Instruction {
	fn register_a(&self) -> u32;
}

trait TwoRegisterInstruction where Self: OneRegisterInstruction {
	fn register_b(&self) -> u32;
}

trait WideLiteralInstruction where Self: Instruction {
	fn wide_literal(&self) -> u64;
}

trait NarrowLiteralInstruction where Self: WideLiteralInstruction {
	fn narrow_literal(&self) -> u64;
}

trait HatLiteralInstruction where Self: Instruction {
	fn hat_literal(&self) -> u32;
}

trait NarrowHatLiteralInstruction where Self: NarrowLiteralInstruction + HatLiteralInstruction {}
trait LongHatLiteralInstruction where Self: WideLiteralInstruction + HatLiteralInstruction {}

macro_rules! instructions {
		() => ();
    { struct $n:ident { $($field_names:ident = ($fields:expr));* }; $($rest:tt)* } => {
    	struct $n {
    		opcode: Opcode,
    		$($fields),*
    	}

			impl Instruction for $n {
				fn opcode(&self) -> &Opcode {
					&self.opcode
				}
			}

			instructions!($($rest)*);
    };
}

instructions! {
	struct Instruction10x {
		register_a = (2);
	};
}

struct Instruction12x(Opcode, (u8, u8)); // actually u4, u4
struct Instruction11n(Opcode, (u8, u8)); // actually u4, u4

struct Instruction11x(Opcode, (u8));
struct Instruction10t(Opcode, (u8));

struct Instruction20t(Opcode, (u16));

struct Instruction20bc(Opcode, (u8, u16));

struct Instruction22x(Opcode, (u8, u16));
struct Instruction21t(Opcode, (u8, u16));
struct Instruction21s(Opcode, (u8, u16));
struct Instruction21ih(Opcode, (u16, u16));
struct Instruction21lh(Opcode, (u16, u16, u32));
struct Instruction21c(Opcode);

struct Instruction23x(Opcode);
struct Instruction22b(Opcode);

struct Instruction22t(Opcode);
struct Instruction22s(Opcode);
struct Instruction22c(Opcode);
struct Instruction22cs(Opcode);

struct Instruction30t(Opcode);

struct Instruction32x(Opcode);

struct Instruction31i(Opcode);
struct Instruction31t(Opcode);
struct Instruction31c(Opcode);

struct Instruction35c(Opcode);
struct Instruction35ms(Opcode);
struct Instruction35mi(Opcode);

struct Instruction3rc(Opcode);
struct Instruction3rms(Opcode);
struct Instruction3rmi(Opcode);

struct Instruction45cc(Opcode);

struct Instruction4rcc(Opcode);

struct Instruction51l(Opcode);

