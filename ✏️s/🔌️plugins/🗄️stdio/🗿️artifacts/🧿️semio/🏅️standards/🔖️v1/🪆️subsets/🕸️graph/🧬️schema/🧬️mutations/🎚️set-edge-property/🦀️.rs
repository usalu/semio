//! 🎚️ `set-edge-property` — sets the value of one EXISTING keyed property of an edge to an absolute typed value: the edge
//! twin of `set-node-property` (composed editors keep per-edge fields as keyed graph properties, e.g. trinity jack's
//! `SET e.k = v`); a key the edge lacks is attached by `add-edge-property` and detached by `remove-edge-property`.

use crate::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::standards::v1::subsets::graph::schema::snapshot::{GraphEdgeId, SemioGraphSnapshot};
use crate::standards::v1::subsets::value::schema::snapshot::SemioValue;

//#region 🔖️Payload
/// 🎚️ `set-edge-property` payload — the edge, the key of its property and the value that property takes.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetEdgeProperty {
    pub edge_id: GraphEdgeId,
    pub key: String,
    pub value: SemioValue,
}

impl protocol::MutationKind<SemioGraphSnapshot, SemioGraphMutation> for SetEdgeProperty {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "edge-property", kind: "set-edge-property", record: "SetEdgeProperty" };

    fn diff(&self, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<<SemioGraphMutation as protocol::Mutation<SemioGraphSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioGraphSnapshot) -> Result<Vec<SemioGraphMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (key, owner) = (&self.key, &self.edge_id.value);
        match crate::standards::v1::subsets::value::schema::snapshot::semio_value_scalar_label(&self.value) {
            Some((en, de)) => semio_framework_ui_locale::LocalizedLabel::native(&format!("Set \"{key}\" of edge \"{owner}\" to {en}"), &format!("\"{key}\" von Kante \"{owner}\" auf {de} setzen")),
            None => semio_framework_ui_locale::LocalizedLabel::native(&format!("Set \"{key}\" of edge \"{owner}\""), &format!("\"{key}\" von Kante \"{owner}\" setzen")),
        }
    }
    fn target(&self) -> Vec<String> {
        vec![self.edge_id.value.clone()]
    }
}
//#endregion 🔖️Payload
