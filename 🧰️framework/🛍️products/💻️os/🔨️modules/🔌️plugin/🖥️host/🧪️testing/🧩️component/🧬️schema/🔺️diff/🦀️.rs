//! 🔺️ An empty operation roster has an identity diff.
use crate::Snapshot;
use semio_framework_os_kernel::os_spr as protocol;
use semio_framework_value_derive::{ToValue,FromValue};
#[derive(Clone,Debug,Default,PartialEq,ToValue,FromValue)]
pub struct Diff {}
impl protocol::DiffAlgebra<Snapshot> for Diff {

    fn is_empty(&self)->bool{true}
}

impl protocol::ColdDiffAlgebra<Snapshot> for Diff {
    fn inverse(&self,_base:&Snapshot)->Self{Self{}}
}
impl protocol::MutationDiff<Snapshot> for Diff {


}

impl protocol::ColdMutationDiff<Snapshot> for Diff {
    fn absorb(&mut self,_other:Self){}

    fn apply(&self,base:&mut Snapshot,_capability:protocol::ApplyCapability)->protocol::MutationApplyResult<()> {
        Ok(())
    }
}
