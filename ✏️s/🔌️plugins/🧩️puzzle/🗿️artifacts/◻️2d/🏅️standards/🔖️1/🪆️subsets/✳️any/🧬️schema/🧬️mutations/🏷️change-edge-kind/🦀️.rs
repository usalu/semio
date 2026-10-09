//! Puzzle2d mutation — `ChangeEdgeKind`: changes an edge's `edge_kind` catalog reference.

use semio_framework_value::paged::PagedUtf8;

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::Puzzle2dSnapshot;

//#region 🔖️Mutation
/// `change-edge-kind` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-edge-kind")]
pub struct ChangeEdgeKind {
    pub id: PagedUtf8<{ usize::MAX }>,
    pub new_edge_kind: Option<PagedUtf8<{ usize::MAX }>>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_edge_kind(id: PagedUtf8<{ usize::MAX }>, new_edge_kind: Option<PagedUtf8<{ usize::MAX }>>) -> Puzzle2dMutation {
    Puzzle2dMutation::ChangeEdgeKind(ChangeEdgeKind { id, new_edge_kind })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for ChangeEdgeKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "edge", kind: "change-edge-kind", record: "ChangedEdgeKind" };

    fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Result<Vec<Puzzle2dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change edge \"{}\" kind", self.id), &format!("Art von Kante \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.to_string_owner()]
    }
}
//#endregion 🔖️Mutation
