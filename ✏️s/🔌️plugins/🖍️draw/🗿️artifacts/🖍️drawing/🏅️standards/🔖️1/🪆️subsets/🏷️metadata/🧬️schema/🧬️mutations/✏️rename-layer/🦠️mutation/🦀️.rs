//! ✏️ Drawing mutation — `RenameLayer`: changes one layer's identity `name` field.
use crate::diff::DrawingDiff;
use crate::mutations::DrawingMutation;
use crate::DrawingSnapshot;

//#region 🔖️Mutation
/// ✏️ `rename-layer` payload — `new_name` per the taxonomy's naming convention for identity fields.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "rename-layer")]
pub struct RenameLayer {
    pub layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    pub new_name: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rename_layer(layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>, new_name: semio_framework_value::paged::PagedUtf8<{usize::MAX}>) -> DrawingMutation {
    DrawingMutation::RenameLayer(RenameLayer { layer_id, new_name })
}

impl protocol::MutationKind<DrawingSnapshot, DrawingMutation> for RenameLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "layer", kind: "rename-layer", record: "RenamedLayer" };

    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename layer \"{}\" to \"{}\"", self.layer_id, self.new_name), &format!("Ebene \"{}\" in \"{}\" umbenennen", self.layer_id, self.new_name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.to_string_owner()]
    }
}
//#endregion 🔖️Mutation
