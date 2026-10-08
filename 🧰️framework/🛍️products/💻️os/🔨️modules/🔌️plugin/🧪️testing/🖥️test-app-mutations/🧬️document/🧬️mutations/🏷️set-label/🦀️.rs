//#region 🏷️SetLabel
use super::super::{TestDiff, TestMutation, TestSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, ToValue, FromValue, semio_framework_value_derive::RetireOwned, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract=::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct SetLabel {
    pub value: String,
}

impl MutationKind<TestSnapshot, TestMutation> for SetLabel {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "label", kind: "set-label", record: "SetLabel" };
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
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set label to {}", self.value), &format!("Beschriftung auf {} setzen", self.value))
    }
}

#[cfg(test)]
#[path = "../../../../../🧪️tests/🏷️test-app-document-set-label-unit/🦀️.rs"]
mod tests;
//#endregion 🏷️SetLabel
use semio_framework_value::ErasedSnapshotRetirement;
/// 🌱️ The authored SetLabel semantic source is born in separately admitted physical pages.
pub(crate) struct SetLabelSource{value:semio_framework_value::paged_text::PagedText<{isize::MAX as usize}>}
impl SetLabel{
    /// 🏷️ Starts this authored leaf's source ownership before any canonical operation encoding.
    pub(crate) const fn source_owner()->SetLabelSource{SetLabelSource{value:semio_framework_value::paged_text::PagedText::empty()}}
}
impl SetLabelSource{
    pub(crate) fn read_from_encoding_source(&mut self,source:&impl semio_framework_value::paged_text::TextReadSource,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),semio_framework_value::ValueError>{self.value.read_from_encoding_source(source,control)}
    pub(crate) fn allocated_bytes(&self)->usize{self.value.allocated_bytes()}
    pub(crate) fn value_view(&self)->Result<semio_framework_value::paged_text::TextReadView<'_>,semio_framework_value::ValueError>{self.value.read_view()}
    /// 🎒️ Borrows this exact original authored owner and the actual mutation ordinal metadata.
    pub(crate) fn retained_pack_operation(&self)->semio_framework_os_kernel::os_pack::record::BorrowedProjectedPackOperation<semio_framework_dsl_record::native_encoding::VariantProjection<'_,Self>>{semio_framework_os_kernel::os_pack::record::BorrowedProjectedPackOperation::from_variant(self)}
    /// 🏷️ Streams the same original semantic source through the authored locale producer.
    pub(crate) fn label_into(&self,_terminology:semio_framework_ui_locale::Terminology,locale:semio_framework_ui_locale::Locale,output:&mut dyn semio_framework_value::paged_text::TextEncodingOutput)->Result<(),semio_framework_value::ValueError>{let text=self.value_view()?;match locale{semio_framework_ui_locale::Locale::En=>semio_framework_value::paged_text::write_encoding_format(output,format_args!("Set label to {text}")),semio_framework_ui_locale::Locale::De=>semio_framework_value::paged_text::write_encoding_format(output,format_args!("Beschriftung auf {text} setzen"))}}
}
impl semio_framework_dsl_record::BorrowedDslVariants for SetLabelSource{
    const VARIANTS:&'static[(&'static str,fn()->semio_framework_dsl_record::BorrowedRecordSpec)]=<TestMutation as semio_framework_dsl_record::BorrowedDslVariants>::VARIANTS;
    fn projected_borrowed_variant_identity(&self)->(&'static str,usize,semio_framework_dsl_record::BorrowedRecordSpec){let(keyword,spec)=Self::VARIANTS[1];(keyword,1,spec())}
    fn projected_borrowed_variant_view(&self,path:&[usize])->Result<semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>,semio_framework_value::ValueError>{use semio_framework_dsl_record::native_encoding::FieldProjectionView as V;match path{[]=>Ok(V::Record(&[0])),[0]=>Ok(V::TextSource(self.value_view()?)),_=>Err(semio_framework_dsl_record::native_encoding::projection_path_error())}}
}
impl semio_framework_value::ErasedSnapshotRetirement for SetLabelSource{
    fn close_step(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,semio_framework_value::ValueError>{self.value.close_step(grant)}
    fn terminal_is_empty(&self)->bool{self.value.terminal_is_empty()}
    fn next_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.value.next_copy_byte_demand()}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,semio_framework_value::ValueError>{self.value.next_capacity_byte_demand(copy)}
    fn next_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.value.next_release_byte_demand()}
    fn next_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.value.next_depth_demand()}
}


impl SetLabel{
    /// 🏷️ Streams this authored mutation's original native label into its bound retained text owner.
    pub(crate) fn label_into(&self,_terminology:semio_framework_ui_locale::Terminology,locale:semio_framework_ui_locale::Locale,output:&mut dyn semio_framework_value::paged_text::TextEncodingOutput)->Result<(),semio_framework_value::ValueError>{
        match locale{
            semio_framework_ui_locale::Locale::En=>semio_framework_value::paged_text::write_encoding_format(output,format_args!("Set label to {}",self.value)),
            semio_framework_ui_locale::Locale::De=>semio_framework_value::paged_text::write_encoding_format(output,format_args!("Beschriftung auf {} setzen",self.value)),
        }
    }
}
