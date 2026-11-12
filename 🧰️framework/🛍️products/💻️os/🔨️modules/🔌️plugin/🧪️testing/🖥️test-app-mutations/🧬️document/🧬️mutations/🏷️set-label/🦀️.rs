//#region 🏷️SetLabel
use crate::test_app_mutation_fixture::document::{TestDiff, TestSnapshot};
use crate::test_app_mutation_fixture::document::mutations::TestMutation;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[doc(hidden)]
#[derive(Clone, semio_framework_value::CanonicalJsonTree, Debug, PartialEq, serde::Serialize, serde::Deserialize, ToValue, FromValue, semio_framework_value_derive::RetireOwned, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_framework_dsl_record_derive::DslReceiving)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract=::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetLabel {
    pub value: String,
}

impl MutationKind<TestSnapshot, TestMutation> for SetLabel {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "label", kind: "set-label", record: "SetLabel" };


    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native(&format!("Set label to {}", self.value), &format!("Beschriftung auf {} setzen", self.value))
    }
}

impl semio_os_command::ColdMutationKind<TestSnapshot, TestMutation> for SetLabel {
    fn diff(&self, base: &TestSnapshot) -> MutationOutcome<TestDiff> {
        let outcome = MutationOutcome::new(TestDiff { count: None, label: Some(self.value.clone()), slot: None });
        match base.label == self.value {
            true => outcome.warning("mutation.no-op", format!("the label already reads {}", self.value)),
            false => outcome,
        }
    }
    fn inverse(&self, base: &TestSnapshot) -> Result<Vec<TestMutation>, semio_framework_value::ValueError> {
        Ok((|| vec![Self { value: base.label.clone() }.into()])())
    }
}

#[doc(hidden)]
#[cfg(test)]
#[path = "../../../../../🧪️tests/🏷️test-app-document-set-label-unit/🦀️.rs"]
pub mod tests;

#[cfg(test)]
#[path = "🧪️tests/🧪️sum-law/🦀️.rs"]
mod sum_law_tests;
//#endregion 🏷️SetLabel
use semio_framework_value::ErasedSnapshotRetirement;
/// 🌱️ The authored SetLabel semantic source is born in separately admitted physical pages.
pub(crate) struct SetLabelSource{pub value:Option<semio_framework_value_paged_text::paged_text::PagedText<{isize::MAX as usize}>>}
impl SetLabel{
    /// 🏷️ Starts this authored leaf's source ownership before any canonical operation encoding.
    pub const fn source_owner()->SetLabelSource{SetLabelSource{value:None}}
}
impl SetLabelSource{
    pub fn allocated_bytes(&self)->usize{self.value.as_ref().map_or(0,|value|value.allocated_bytes())}
    pub fn value_view(&self)->Result<semio_framework_value_paged_text::paged_text::TextReadView<'_>,semio_framework_value::ValueError>{self.value.as_ref().ok_or_else(||semio_framework_value::ValueError::invariant("original label source is not published"))?.read_view()}
    /// 🎒️ Borrows this exact original authored owner and the actual mutation ordinal metadata.
    pub fn retained_pack_operation(&self)->semio_os_kernel::os_pack::record::BorrowedProjectedPackOperation<semio_framework_dsl_record::native_encoding::VariantProjection<'_,Self>>{semio_os_kernel::os_pack::record::BorrowedProjectedPackOperation::from_variant(self)}
}
impl semio_framework_dsl_record::BorrowedDslVariants for SetLabelSource{
    const VARIANTS:&'static[(&'static str,fn()->semio_framework_dsl_record::BorrowedRecordSpec)]=<TestMutation as semio_framework_dsl_record::BorrowedDslVariants>::VARIANTS;
    fn projected_borrowed_variant_identity(&self)->(&'static str,usize,semio_framework_dsl_record::BorrowedRecordSpec){let(keyword,spec)=Self::VARIANTS[1];(keyword,1,spec())}
    fn projected_borrowed_variant_view(&self,path:&[usize])->Result<semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>,semio_framework_value::ValueError>{use semio_framework_dsl_record::native_encoding::FieldProjectionView as V;match path{[]=>Ok(V::Record(&[0])),[0]=>Ok(V::TextSource(self.value_view()?)),_=>Err(semio_framework_dsl_record::native_encoding::projection_path_error())}}
}
impl semio_framework_value::ErasedSnapshotRetirement for SetLabelSource{
 fn next_demand(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{match self.value.as_ref(){None=>Ok(Default::default()),Some(value)if value.terminal_is_empty()=>Ok(semio_framework_value::RetirementDemand{copy_bytes:std::mem::size_of_val(value),depth:1,..Default::default()}),Some(value)=>value.next_demand(body)}}
 fn next_computational_work(&self,body:usize)->Result<u64,semio_framework_value::ValueError>{match self.value.as_ref(){None=>Ok(0),Some(value)if value.terminal_is_empty()=>Ok(1),Some(value)=>value.next_computational_work(body)}}
 fn close_step(&mut self,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll{
  let settle=|result:Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>,work:u128|semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result:result.map_err(Into::into),work});
  let Some(value)=self.value.as_mut() else{return settle(Ok(semio_framework_value::RetainedCloneStep::Complete(Default::default())),0)};
  if !value.terminal_is_empty(){return value.close_step(body,effect)}
  let demand=match self.next_demand(effect.grant().maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return settle(Err(error),0)};
  if !semio_framework_value::retained_clone::retained_clone_grant_admits(effect.grant(),demand)||effect.maximum_work_units()==0{return settle(Ok(semio_framework_value::RetainedCloneStep::Progress(Default::default())),0)}
  self.value=None;
  settle(Ok(semio_framework_value::RetainedCloneStep::Complete(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),1)
 }
 fn terminal_is_empty(&self)->bool{self.value.is_none()}
 fn next_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
 fn next_capacity_byte_demand(&self,body:usize)->Result<usize,semio_framework_value::ValueError>{Ok(self.next_demand(body)?.capacity_bytes)}
 fn next_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.next_demand(0)?.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.next_demand(0)?.depth)}
}
impl semio_framework_value::RetireOwned for SetLabelSource{
 type RetirementCursor=<Option<semio_framework_value_paged_text::paged_text::PagedText<{isize::MAX as usize}>> as semio_framework_value::RetireOwned>::RetirementCursor;
 fn retirement_ready(&self)->bool{semio_framework_value::RetireOwned::retirement_ready(&self.value)}
 fn retirement_seed(self)->Self::RetirementCursor{semio_framework_value::RetireOwned::retirement_seed(self.value)}
 fn retirement_birth_bytes(&self)->Option<usize>{self.retirement_ready().then_some(std::mem::size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{true}
}

impl SetLabel{
}
