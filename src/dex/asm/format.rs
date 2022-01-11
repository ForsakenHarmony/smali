use std::fmt::{Display, Formatter};

use enum_values::EnumValues;

#[derive(EnumValues, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[enum_values(size = "i8", name = "String", payload = "bool")]
pub enum Format {
	#[enum_values(size = "2", name = "10t")]
	Format10t,
	#[enum_values(size = "2", name = "10x")]
	Format10x,
	#[enum_values(size = "2", name = "11n")]
	Format11n,
	#[enum_values(size = "2", name = "11x")]
	Format11x,
	#[enum_values(size = "2", name = "12x")]
	Format12x,
	#[enum_values(size = "4", name = "20bc")]
	Format20bc,
	#[enum_values(size = "4", name = "20t")]
	Format20t,
	#[enum_values(size = "4", name = "21c")]
	Format21c,
	#[enum_values(size = "4", name = "21ih")]
	Format21ih,
	#[enum_values(size = "4", name = "21lh")]
	Format21lh,
	#[enum_values(size = "4", name = "21s")]
	Format21s,
	#[enum_values(size = "4", name = "21t")]
	Format21t,
	#[enum_values(size = "4", name = "22b")]
	Format22b,
	#[enum_values(size = "4", name = "22c")]
	Format22c,
	#[enum_values(size = "4", name = "22cs")]
	Format22cs,
	#[enum_values(size = "4", name = "22s")]
	Format22s,
	#[enum_values(size = "4", name = "22t")]
	Format22t,
	#[enum_values(size = "4", name = "22x")]
	Format22x,
	#[enum_values(size = "4", name = "23x")]
	Format23x,
	#[enum_values(size = "6", name = "30t")]
	Format30t,
	#[enum_values(size = "6", name = "31c")]
	Format31c,
	#[enum_values(size = "6", name = "31i")]
	Format31i,
	#[enum_values(size = "6", name = "31t")]
	Format31t,
	#[enum_values(size = "6", name = "32x")]
	Format32x,
	#[enum_values(size = "6", name = "35c")]
	Format35c,
	#[enum_values(size = "6", name = "35mi")]
	Format35mi,
	#[enum_values(size = "6", name = "35ms")]
	Format35ms,
	#[enum_values(size = "6", name = "3rc")]
	Format3rc,
	#[enum_values(size = "6", name = "3rmi")]
	Format3rmi,
	#[enum_values(size = "6", name = "3rms")]
	Format3rms,
	#[enum_values(size = "8", name = "45cc")]
	Format45cc,
	#[enum_values(size = "8", name = "4rcc")]
	Format4rcc,
	#[enum_values(size = "10", name = "51l")]
	Format51l,
	#[enum_values(size = "-1", payload)]
	ArrayPayload,
	#[enum_values(size = "-1", payload)]
	PackedSwitchPayload,
	#[enum_values(size = "-1", payload)]
	SparseSwitchPayload,
	#[enum_values(size = "-1")]
	UnresolvedOdexInstruction,
}

impl Display for Format {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.name())
	}
}
