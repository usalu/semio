//! 🔒️ Puzzle2d mutation — `ChangeNodeLocked`: changes a node's locked flag.

use semio_framework_value::paged::PagedUtf8;

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::Puzzle2dSnapshot;

//#region 🔖️Mutation
/// 🔒️ `change-node-locked` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-node-locked")]
pub struct ChangeNodeLocked {
    pub id: PagedUtf8<{ usize::MAX }>,
    pub new_locked: Option<bool>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_node_locked(id: PagedUtf8<{ usize::MAX }>, new_locked: Option<bool>) -> Puzzle2dMutation {
    Puzzle2dMutation::ChangeNodeLocked(ChangeNodeLocked { id, new_locked })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for ChangeNodeLocked {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node", kind: "change-node-locked", record: "ChangedNodeLocked" };

    fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Result<Vec<Puzzle2dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change node \"{}\" locked", self.id), &format!("Sperre von Knoten \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.to_string_owner()]
    }
}
//#endregion 🔖️Mutation
