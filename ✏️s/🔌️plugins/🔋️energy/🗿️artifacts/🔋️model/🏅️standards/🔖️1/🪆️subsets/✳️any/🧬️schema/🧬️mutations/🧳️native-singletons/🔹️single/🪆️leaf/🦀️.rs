//! 🪆️ Singleton scalar leaves share the original semantic lifecycle and declare an empty diagnostic target list.
use super::*;
use crate::leaf::{RowLeafSpec,RowLeafPreparation};
use protocol::RetainedDiagnosticSpec;
/// 🏷️ Authored singleton recipes lend actual scalar input and inverse construction without a fabricated row key.
pub trait SingleLeafSpec<L,M>:SingleFieldPath{const INVALID:RetainedDiagnosticSpec;const INVALID_CONTEXT:RetainedDiagnosticSpec=Self::INVALID;const UNCHANGED:RetainedDiagnosticSpec;fn input(payload:RetainedCloneRef<'_,L>)->RetainedCloneRef<'_,Self::Scalar>;fn inverse(payload:RetainedCloneRef<'_,L>,value:Self::Scalar)->M;}
pub struct SingleSpec<S>(std::marker::PhantomData<fn()->S>);
impl<L,M,S:SingleLeafSpec<L,M>> RowLeafSpec<L,M,0> for SingleSpec<S>{type Input=S::Scalar;type Inverse=S::Scalar;type Core=SingleFieldPreparation<S>;const INVALID:RetainedDiagnosticSpec=S::INVALID;const INVALID_CONTEXT:RetainedDiagnosticSpec=S::INVALID_CONTEXT;const MISSING:RetainedDiagnosticSpec=S::INVALID;const UNCHANGED:RetainedDiagnosticSpec=S::UNCHANGED;fn input(p:RetainedCloneRef<'_,L>)->RetainedCloneRef<'_,Self::Input>{S::input(p)}fn key(_:RetainedCloneRef<'_,L>)->Option<RetainedCloneRef<'_,u32>>{None}fn inverse(p:RetainedCloneRef<'_,L>,v:Self::Inverse)->M{S::inverse(p,v)}fn diagnostic_targets(_:RetainedCloneRef<'_,L>,_:RowOutcome)->Result<[protocol::RetainedDiagnosticTarget<'_>;0],ValueError>{Ok([])}}
pub type SingleLeafPreparation<L,M,S>=RowLeafPreparation<L,M,SingleSpec<S>,0>;
