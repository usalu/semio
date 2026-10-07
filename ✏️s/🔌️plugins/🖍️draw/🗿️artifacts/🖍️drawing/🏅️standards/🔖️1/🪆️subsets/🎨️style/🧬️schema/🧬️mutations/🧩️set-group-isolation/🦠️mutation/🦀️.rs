//! 🧩️ Author whether a group isolates its children's compositing.
use crate::{DrawingSnapshot,mutations::DrawingMutation,diff::DrawingDiff};
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract=::protocol)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all="camelCase")]
#[cfg_attr(test,serde(rename_all="camelCase"))]
#[dsl(keyword="set-group-isolation")]
pub struct SetGroupIsolation {pub layer_id:String,pub isolation:bool}
pub fn set_group_isolation(layer_id:semio_framework_value::paged::PagedUtf8<{usize::MAX}>,isolation:bool)->DrawingMutation {DrawingMutation::SetGroupIsolation(SetGroupIsolation {layer_id,isolation})}
impl protocol::MutationKind<DrawingSnapshot,DrawingMutation> for SetGroupIsolation {
    const SEMANTICS:protocol::SemanticDescriptor=protocol::SemanticDescriptor {verb:"set",entity:"group",kind:"set-group-isolation",record:"SetGroupIsolation"};
    fn diff(&self,base:&DrawingSnapshot)->protocol::MutationOutcome<DrawingDiff> {super::diff::diff(self,base)}
    fn inverse(&self,base:&DrawingSnapshot)-> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
    Ok({super::inverse::inverse(self,base)?
    })
}
    fn label(&self)->semio_framework_ui_locale::LocalizedLabel {
        if self.isolation {semio_framework_ui_locale::LocalizedLabel::native(&format!("Isolate group \"{}\"",self.layer_id),&format!("Gruppe \"{}\" isolieren",self.layer_id))}
        else {semio_framework_ui_locale::LocalizedLabel::native(&format!("Pass group \"{}\" blending through",self.layer_id),&format!("Mischung von Gruppe \"{}\" durchreichen",self.layer_id))}
    }
    fn target(&self)->Vec<String> {vec![self.layer_id.to_string_owner()]}
}
