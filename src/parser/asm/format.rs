use enum_values::Values;

#[derive(Values)]
#[values(size = "i8", payload = "bool")]
pub enum Format {
	#[values(size = "2")]
	Format10t,
	#[values(size = "2")]
	Format10x,
	#[values(size = "2")]
	Format11n,
	#[values(size = "2")]
	Format11x,
	#[values(size = "2")]
	Format12x,
	#[values(size = "4")]
	Format20bc,
	#[values(size = "4")]
	Format20t,
	#[values(size = "4")]
	Format21c,
	#[values(size = "4")]
	Format21ih,
	#[values(size = "4")]
	Format21lh,
	#[values(size = "4")]
	Format21s,
	#[values(size = "4")]
	Format21t,
	#[values(size = "4")]
	Format22b,
	#[values(size = "4")]
	Format22c,
	#[values(size = "4")]
	Format22cs,
	#[values(size = "4")]
	Format22s,
	#[values(size = "4")]
	Format22t,
	#[values(size = "4")]
	Format22x,
	#[values(size = "4")]
	Format23x,
	#[values(size = "6")]
	Format30t,
	#[values(size = "6")]
	Format31c,
	#[values(size = "6")]
	Format31i,
	#[values(size = "6")]
	Format31t,
	#[values(size = "6")]
	Format32x,
	#[values(size = "6")]
	Format35c,
	#[values(size = "6")]
	Format35mi,
	#[values(size = "6")]
	Format35ms,
	#[values(size = "6")]
	Format3rc,
	#[values(size = "6")]
	Format3rmi,
	#[values(size = "6")]
	Format3rms,
	#[values(size = "8")]
	Format45cc,
	#[values(size = "8")]
	Format4rcc,
	#[values(size = "10")]
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
