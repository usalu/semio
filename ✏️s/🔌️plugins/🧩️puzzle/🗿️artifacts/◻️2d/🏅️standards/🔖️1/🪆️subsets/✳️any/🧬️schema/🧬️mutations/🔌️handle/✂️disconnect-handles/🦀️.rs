//! ✂️ Puzzle2d mutation — `DisconnectHandles`: removes a directed link between two handles.

use crate::standards::v1::subsets::any::schema::mutations::change_edge_visible::ChangeEdgeVisible;
use crate::standards::v1::subsets::any::schema::mutations::change_edge_locked::ChangeEdgeLocked;
use crate::standards::v1::subsets::any::schema::mutations::connect_handles::ConnectHandles;
use semio_framework_value::paged::PagedUtf8;

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::Puzzle2dSnapshot;

//#region 🔖️Mutation
/// ✂️ `disconnect-handles` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "disconnect-handles")]
pub struct DisconnectHandles {
    pub id: PagedUtf8<{ usize::MAX }>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn disconnect_handles<Op: From<DisconnectHandles>>(id: PagedUtf8<{ usize::MAX }>) -> Op {
    Op::from(DisconnectHandles { id })
}

impl<Op> protocol::MutationKind<Puzzle2dSnapshot, Op> for DisconnectHandles
where
    Op: protocol::Mutation<Puzzle2dSnapshot, Diff = Puzzle2dDiff> + From<DisconnectHandles> + From<ConnectHandles> + From<ChangeEdgeVisible> + From<ChangeEdgeLocked>,
 {

    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "disconnect", entity: "handles", kind: "disconnect-handles", record: "DisconnectedHandles" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Disconnect \"{}\"", self.id), &format!("\"{}\" trennen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.to_string_owner()]
    }

}
impl<Op> protocol::ColdMutationKind<Puzzle2dSnapshot, Op> for DisconnectHandles
where
    Op: protocol::ColdMutation<Puzzle2dSnapshot, ReferenceDiff = Puzzle2dDiff> + From<DisconnectHandles> + From<ConnectHandles> + From<ChangeEdgeVisible> + From<ChangeEdgeLocked>,
 {
fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
fn inverse(&self, base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
