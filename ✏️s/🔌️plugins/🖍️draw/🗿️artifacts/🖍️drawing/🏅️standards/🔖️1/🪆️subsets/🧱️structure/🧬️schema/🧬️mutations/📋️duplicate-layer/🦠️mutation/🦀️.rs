//! 🧬️ Drawing mutation — `DuplicateLayer`: copies an existing layer to a new, content-addressed id
//! right after its source.
use crate::diff::DrawingDiff;
use crate::mutations::DrawingMutation;
use crate::DrawingSnapshot;

//#region 🔖️Mutation
/// 🧬️ `duplicate-layer` payload — source address only; the duplicate's id is deterministic
/// (content-addressed from the source, see `engine::clone_drawing_layer_node`), so `diff`/`inverse`
/// recompute it from BASE rather than carrying it.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "duplicate-layer")]
pub struct DuplicateLayer {
    pub layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn duplicate_layer(layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>) -> DrawingMutation {
    DrawingMutation::DuplicateLayer(DuplicateLayer { layer_id })
}

impl protocol::MutationKind<DrawingSnapshot, DrawingMutation> for DuplicateLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "duplicate", entity: "layer", kind: "duplicate-layer", record: "DuplicatedLayer" };

    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Duplicate layer \"{}\"", self.layer_id), &format!("Ebene \"{}\" duplizieren", self.layer_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.to_string_owner()]
    }
}
//#endregion 🔖️Mutation
