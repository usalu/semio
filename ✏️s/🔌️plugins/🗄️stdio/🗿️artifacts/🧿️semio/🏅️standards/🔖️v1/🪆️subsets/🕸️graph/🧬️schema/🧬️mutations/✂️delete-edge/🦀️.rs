//! ✂️ `delete-edge` — removes an id-keyed edge; no cascade needed (edges don't own other entities).

use crate::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::standards::v1::subsets::graph::schema::snapshot::{GraphEdgeId, SemioGraphSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteEdge {
    pub id: GraphEdgeId,
}

impl protocol::MutationKind<SemioGraphSnapshot, SemioGraphMutation> for DeleteEdge {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "edge", kind: "delete-edge", record: "DeletedEdge" };

    fn diff(&self, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<<SemioGraphMutation as protocol::Mutation<SemioGraphSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioGraphSnapshot) -> Result<Vec<SemioGraphMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete edge \"{}\"", self.id.value), &format!("Kante \"{}\" löschen", self.id.value))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.value.clone()]
    }
}
//#endregion 🔖️Payload
