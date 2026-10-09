//! 🔌 Puzzle2d mutation — `ReplaceNodeHandle`: whole-value swap of one handle's presentation
//! fields (kind/angle/radius/color/icon/scale/visible/locked together, one property-panel gesture).

use semio_framework_value::paged::PagedUtf8;

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::{Puzzle2dHandle, Puzzle2dSnapshot};

//#region 🔖️Mutation
/// 🔌 `replace-node-handle` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "replace-node-handle")]
pub struct ReplaceNodeHandle {
    pub node_id: PagedUtf8<{ usize::MAX }>,
    pub handle_id: PagedUtf8<{ usize::MAX }>,
    #[dsl(block)]
    pub new_handle: Puzzle2dHandle,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_node_handle(node_id: PagedUtf8<{ usize::MAX }>, handle_id: PagedUtf8<{ usize::MAX }>, new_handle: Puzzle2dHandle) -> Puzzle2dMutation {
    Puzzle2dMutation::ReplaceNodeHandle(ReplaceNodeHandle { node_id, handle_id, new_handle })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for ReplaceNodeHandle {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "node-handle", kind: "replace-node-handle", record: "ReplacedNodeHandle" };

    fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Result<Vec<Puzzle2dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace handle \"{}\" on node \"{}\"", self.handle_id, self.node_id), &format!("Griff \"{}\" an Knoten \"{}\" ersetzen", self.handle_id, self.node_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.node_id.to_string_owner(), self.handle_id.to_string_owner()]
    }
}
//#endregion 🔖️Mutation
