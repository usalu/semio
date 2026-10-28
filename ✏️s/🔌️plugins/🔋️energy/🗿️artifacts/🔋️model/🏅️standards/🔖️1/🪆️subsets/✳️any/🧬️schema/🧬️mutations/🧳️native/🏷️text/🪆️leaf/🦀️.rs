//! 🏷️ Name row policies share exact diagnostics and inverse factory ownership.
use super::*;
use crate::leaf::{RowFieldPreparation,RowLeafSpec,RowLeafPreparation};
use protocol::mutation::retained::RetainedDiagnosticSpec;
/// 🏷️ Each authored schema field supplies its native address and typed inverse, without an erased payload map.
pub trait TextLeafSpec<L,M>:TextRowPath{
 const DUPLICATE:protocol::mutation::retained::RetainedDiagnosticSpec=Self::INVALID;
 const INVALID:RetainedDiagnosticSpec;
 const UNCHANGED:RetainedDiagnosticSpec;
 const MISSING:RetainedDiagnosticSpec;
 fn input(payload:RetainedCloneRef<'_,L>)->RetainedCloneRef<'_,String>;
 fn key(payload:RetainedCloneRef<'_,L>)->RetainedCloneRef<'_,u32>;
 fn inverse(payload:RetainedCloneRef<'_,L>,value:String)->M;
}


/// 🧭️ The name recipe retains the original string preparation inside the shared semantic owner.
pub struct TextSpec<S>(std::marker::PhantomData<fn()->S>);
impl<L,M,S:TextLeafSpec<L,M>> RowLeafSpec<L,M> for TextSpec<S>{type Input=String;type Inverse=String;type Core=TextRowPreparation<S>;const INVALID:RetainedDiagnosticSpec=S::INVALID;const DUPLICATE:RetainedDiagnosticSpec=S::DUPLICATE;const UNCHANGED:RetainedDiagnosticSpec=S::UNCHANGED;const MISSING:RetainedDiagnosticSpec=S::MISSING;fn input(p:RetainedCloneRef<'_,L>)->RetainedCloneRef<'_,Self::Input>{S::input(p)}fn key(p:RetainedCloneRef<'_,L>)->Option<RetainedCloneRef<'_,u32>>{Some(S::key(p))}fn inverse(p:RetainedCloneRef<'_,L>,v:Self::Inverse)->M{S::inverse(p,v)}fn diagnostic_targets(p:RetainedCloneRef<'_,L>,_:TextOutcome)->Result<[protocol::mutation::retained::RetainedDiagnosticTarget<'_>;1],ValueError>{Ok([protocol::mutation::retained::RetainedDiagnosticTarget::Unsigned(S::key(p))])}}
pub type TextLeafPreparation<L,M,S>=RowLeafPreparation<L,M,TextSpec<S>>;
