use enum_values::Values;

#[derive(Values)]
#[values(size = "i8", payload = "bool")]
pub enum Format {
	#[values(size = "1")]
	Format10t,
	#[values(size = "1")]
	Format10x,
	#[values(size = "1")]
	Format11n,
	#[values(size = "1")]
	Format11x,
	#[values(size = "1")]
	Format12x,
	#[values(size = "2")]
	Format20bc,
	#[values(size = "2")]
	Format20t,
	#[values(size = "2")]
	Format21c,
	#[values(size = "2")]
	Format21ih,
	#[values(size = "2")]
	Format21lh,
	#[values(size = "2")]
	Format21s,
	#[values(size = "2")]
	Format21t,
	#[values(size = "2")]
	Format22b,
	#[values(size = "2")]
	Format22c,
	#[values(size = "2")]
	Format22cs,
	#[values(size = "2")]
	Format22s,
	#[values(size = "2")]
	Format22t,
	#[values(size = "2")]
	Format22x,
	#[values(size = "2")]
	Format23x,
	#[values(size = "3")]
	Format30t,
	#[values(size = "3")]
	Format31c,
	#[values(size = "3")]
	Format31i,
	#[values(size = "3")]
	Format31t,
	#[values(size = "3")]
	Format32x,
	#[values(size = "3")]
	Format35c,
	#[values(size = "3")]
	Format35mi,
	#[values(size = "3")]
	Format35ms,
	#[values(size = "3")]
	Format3rc,
	#[values(size = "3")]
	Format3rmi,
	#[values(size = "3")]
	Format3rms,
	#[values(size = "4")]
	Format45cc,
	#[values(size = "4")]
	Format4rcc,
	#[values(size = "5")]
	Format51l,
	#[values(size = "-1", payload)]
	ArrayPayload,
	#[values(size = "-1", payload)]
	PackedSwitchPayload,
	#[values(size = "-1", payload)]
	SparseSwitchPayload,
	#[values(size = "-1")]
	UnresolvedOdexInstruction,
}

//impl Format {
//	pub fn size(&self) -> i8 {
//		use Format::*;
//
//		match self {
//			Format10t | Format10x | Format11n |Format11x | Format12x => 2,
//			Format20bc | Format20t | Format21c | Format21ih | Format21lh | Format21s | Format21t | Format22b | Format22c | Format22cs | Format22s | Format22t | Format22x | Format23x => 4,
//			Format30t | Format31c | Format31i | Format31t | Format32x | Format35c | Format35mi | Format35ms | Format3rc | Format3rmi | Format3rms => 6,
//			Format45cc | Format4rcc => 8,
//			Format51l => 10,
//			ArrayPayload | PackedSwitchPayload | SparseSwitchPayload | UnresolvedOdexInstruction => -1,
//		}
//	}
//
//	pub fn payload(&self) -> bool {
//		use Format::*;
//
//		match self {
//			ArrayPayload | PackedSwitchPayload | SparseSwitchPayload => true,
//			_ => false,
//		}
//	}
//}
