//! ❄️ Explicit borrowed wire-value reference arithmetic is separate from required native retained ownership.
use super::{ColdMutationDiff,MutationOutcome};
/// 📖️ Reference decisions and executable inverse values never serve as a default native preparation.
pub trait ColdMutation<P>:Sized{
 type ReferenceDiff:ColdMutationDiff<P>;
 fn diff(&self,base:&P)->MutationOutcome<Self::ReferenceDiff>;
 fn inverse(&self,base:&P)->Result<Vec<Self>,semio_framework_value::ValueError>;
 fn foreign_step_source<'a>(&'a self,_base:&'a P,_index:usize)->Result<Option<super::ForeignStepSource<'a>>,semio_framework_value::ValueError>{Ok(None)}
}
