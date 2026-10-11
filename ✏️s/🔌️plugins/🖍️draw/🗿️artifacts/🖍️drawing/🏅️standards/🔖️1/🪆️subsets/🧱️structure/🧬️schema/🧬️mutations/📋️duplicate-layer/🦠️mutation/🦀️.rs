//! 🧬️ Drawing mutation with explicit source-to-target subtree identity facts.
use crate::diff::DrawingDiff;
use crate::mutations::DrawingMutation;
use crate::DrawingSnapshot;

//#region 🔖️Mutation
/// 📋️ Copies a layer using the complete admitted identity assignment set.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "duplicate-layer")]
pub struct DuplicateLayer {
    pub identities: semio_framework_value::list::PagedList<crate::schema::identity::DrawingIdentityAssignment,{usize::MAX}>,
    pub layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn duplicate_layer(layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>, identities: semio_framework_value::list::PagedList<crate::schema::identity::DrawingIdentityAssignment,{usize::MAX}>) -> DrawingMutation {
    DrawingMutation::DuplicateLayer(DuplicateLayer { layer_id, identities })
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
