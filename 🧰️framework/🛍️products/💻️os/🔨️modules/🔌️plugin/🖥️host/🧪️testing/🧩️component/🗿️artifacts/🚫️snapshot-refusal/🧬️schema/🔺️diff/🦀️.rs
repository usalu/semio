//! 🔺️ Identity diff of the refusal artifact's own empty operation roster.
use super::Snapshot;
use semio_framework_os_kernel::os_spr as protocol;
use semio_framework_value_derive::{ToValue,FromValue};
#[derive(Clone,Debug,Default,PartialEq,ToValue,FromValue)]
pub struct Diff {}
impl protocol::DiffAlgebra<Snapshot> for Diff {
    fn inverse(&self,_base:&Snapshot)->Self{Self{}}
    fn is_empty(&self)->bool{true}
}
impl protocol::MutationDiff<Snapshot> for Diff {
    fn apply(&self,base:&Snapshot,_capability:protocol::ApplyCapability)->protocol::MutationApplyResult<Snapshot>{Ok(base.clone())}
    fn absorb(&mut self,_other:Self){}
}
