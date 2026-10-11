//! 🎨️ Block2d mutation — `ChangeHandleKindColor`: a handle-kind catalog row's `color`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Block2dMutation;

//#region 🔖️Mutation
/// 🎨️ `change-handle-kind-color` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-handle-kind-color")]
pub struct ChangeHandleKindColor {
    pub id: String,
    pub new_color: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_handle_kind_color(id: String, new_color: String) -> Block2dMutation {
    Block2dMutation::ChangeHandleKindColor(ChangeHandleKindColor { id, new_color })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for ChangeHandleKindColor {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "handle-kind", kind: "change-handle-kind-color", record: "ChangedHandleKindColor" };

    fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Result<Vec<Block2dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change handle kind \"{}\" color to \"{}\"", self.id, self.new_color), &format!("Farbe von Griffart \"{}\" auf \"{}\" ändern", self.id, self.new_color))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
