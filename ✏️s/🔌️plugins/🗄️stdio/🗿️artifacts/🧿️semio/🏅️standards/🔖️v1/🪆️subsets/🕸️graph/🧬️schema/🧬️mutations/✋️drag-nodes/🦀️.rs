//! ✋️ `drag-nodes` — a relative drag of a set of graph nodes by one common canvas offset (the flow subset's twin, ticket
//! 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING design §12, §13.3, §20.15). The gesture's own inputs (which nodes, which offset)
//! are the payload, so editing the drag in history re-derives every position from whatever base it replays on. Editors that
//! compose this subset as their content child (dag, reasoning/wires) yield one per node drag.

use crate::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::standards::v1::subsets::graph::schema::snapshot::{GraphNodeId, SemioGraphSnapshot};

//#region 🔖️Payload
/// ✋️ `drag-nodes` payload — the node ids it moves and the offset every one of them moves by.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DragNodes {
    pub targets: Vec<GraphNodeId>,
    pub dx: f64,
    pub dy: f64,
}

impl protocol::MutationKind<SemioGraphSnapshot, SemioGraphMutation> for DragNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "drag", entity: "nodes", kind: "drag-nodes", record: "DraggedNodes" };

    fn diff(&self, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<<SemioGraphMutation as protocol::Mutation<SemioGraphSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioGraphSnapshot) -> Result<Vec<SemioGraphMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let number = |value: f64| {
            let rounded = (value * 100.0).round() / 100.0;
            let text = format!("{:.2}", if rounded == 0.0 { 0.0 } else { rounded });
            let en = text.trim_end_matches('0').trim_end_matches('.').to_string();
            let de = en.replace('.', ",");
            (en, de)
        };
        let ((dx_en, dx_de), (dy_en, dy_de)) = (number(self.dx), number(self.dy));
        let (items_en, items_de) = match self.targets.len() {
            1 => ("1 node".to_string(), "1 Knoten".to_string()),
            count => (format!("{count} nodes"), format!("{count} Knoten")),
        };
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Drag {items_en} by ({dx_en}, {dy_en})"), &format!("{items_de} um ({dx_de}; {dy_de}) ziehen"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.iter().map(|id| id.value.clone()).collect()
    }
}
//#endregion 🔖️Payload
