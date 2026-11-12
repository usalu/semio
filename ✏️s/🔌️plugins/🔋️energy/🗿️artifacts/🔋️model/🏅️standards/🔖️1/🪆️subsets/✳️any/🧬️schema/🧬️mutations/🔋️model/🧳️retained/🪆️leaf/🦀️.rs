//! 🪆️ Authored identity leaves lend their actual text and exact inverse constructor to the retained semantic owner.
use super::*;
use protocol::{RetainedDiagnosticSpec,RetainedDiagnosticTarget};
use semio_s_artifact_energy_model_native_mutations::leaf::{RowLeafSpec,RowLeafPreparation};

/// 🏷️ Each authored schema field supplies its native address and typed inverse, without an erased payload map.
pub trait IdentityPayload<M>:Send+Sync+'static{
 const FIELD:IdentityField;
 const INVALID:RetainedDiagnosticSpec;
 const UNCHANGED:RetainedDiagnosticSpec;
 fn source(payload:RetainedCloneRef<'_,Self>)->RetainedCloneRef<'_,String> where Self:Sized;
 fn inverse(value:String)->M;
}

/// 🧭️ The shared semantic row leaf lifecycle drives the identity change with its one original text target.
pub struct IdentitySpec<L,M>(std::marker::PhantomData<fn()->(L,M)>);
impl<L:IdentityPayload<M>,M:Send+'static> RowLeafSpec<L,M,1> for IdentitySpec<L,M>{
 type Input=String;type Inverse=String;type Core=IdentityChangePreparation<L,M>;
 const INVALID:RetainedDiagnosticSpec=L::INVALID;const MISSING:RetainedDiagnosticSpec=L::INVALID;const UNCHANGED:RetainedDiagnosticSpec=L::UNCHANGED;
 fn input(payload:RetainedCloneRef<'_,L>)->RetainedCloneRef<'_,String>{L::source(payload)}
 fn key(_:RetainedCloneRef<'_,L>)->Option<RetainedCloneRef<'_,u32>>{None}
 fn inverse(_:RetainedCloneRef<'_,L>,value:String)->M{L::inverse(value)}
 fn diagnostic_targets(payload:RetainedCloneRef<'_,L>,_:RowOutcome)->Result<[RetainedDiagnosticTarget<'_>;1],ValueError>{Ok([RetainedDiagnosticTarget::Original(L::source(payload))])}
}
/// 🧳️ Diagnostics and inverse rows keep the exact original factory through every denied or cancelled turn.
pub type IdentityLeafPreparation<L,M>=RowLeafPreparation<L,M,IdentitySpec<L,M>,1>;
