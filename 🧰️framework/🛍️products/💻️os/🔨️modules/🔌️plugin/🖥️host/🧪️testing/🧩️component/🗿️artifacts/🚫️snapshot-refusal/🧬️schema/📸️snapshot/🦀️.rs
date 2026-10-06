//! 🚫️ Complete typed document owned by the selected compiled refusal contributors.
use semio_framework_os_kernel::os_store as store;
use semio_framework_value_derive::{ToValue,FromValue};
#[derive(Clone,Debug,Default,PartialEq,ToValue,FromValue,semio_framework_dsl_record_derive::DslRecord)]
#[value(deny_unknown_fields)]
#[dsl(id="fixture.neutral-host-fixture.snapshot-refusal",layout="lines")]
pub struct Snapshot {pub value:i32}
impl semio_framework_schema_composition::ArtifactCompositionFields for Snapshot {
 fn visit_child_refs<'a,V:semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self,_visitor:&mut V)->Result<(),V::Error>{Ok(())}
}


