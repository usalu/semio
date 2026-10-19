use super::super::{dag_index_to_wire, DagDelta, DagDiff, DagMutation, DagSnapshot, ReorderedNode};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🔀️ One positional step of a `reorder-nodes` gesture: node `id` lands at z-stack position `to`, counted in the list as it stands after the steps before it.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeMove {
    pub id: String,
    pub to: u64,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "reorder-nodes")]
pub struct ReorderNodes {
    pub moves: Vec<NodeMove>,
}

impl ReorderNodes {
    /// 🔀️ The positional rows of this gesture: each step's source position is read from the node order as the steps before it leave it, kept as plain ids.
    /// A step naming a node the base lacks keeps `u64::MAX` as its source, which no list contains, so the central applier rejects the whole diff.
    fn rows(&self, base: &DagSnapshot) -> Vec<ReorderedNode> {
        let mut order: Vec<&str> = base.nodes.iter().map(|node| node.id.as_str()).collect();
        self.moves
            .iter()
            .map(|step| {
                let at = order.iter().position(|id| *id == step.id);
                if let (Some(at), Ok(to)) = (at, usize::try_from(step.to)) {
                    if to < order.len() {
                        let id = order.remove(at);
                        order.insert(to, id);
                    }
                }
                ReorderedNode { id: step.id.clone(), from: at.map_or(u64::MAX, dag_index_to_wire), to: step.to }
            })
            .collect()
    }
}

impl protocol::MutationKind<DagSnapshot, DagMutation> for ReorderNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "nodes", kind: "reorder-nodes", record: "ReorderedNodes" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Reorder nodes", "Knoten umordnen")
    }
    fn target(&self) -> Vec<String> {
        vec!["nodes".into()]
    }
}

impl protocol::ColdMutationKind<DagSnapshot, DagMutation> for ReorderNodes {
    fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        protocol::MutationOutcome::new(DagDiff::from(DagDelta { reordered_nodes: Some(self.rows(base)), ..Default::default() }))
    }
    fn inverse(&self, base: &DagSnapshot) -> Result<Vec<DagMutation>, semio_framework_value::ValueError> {
        let rows = self.rows(base);
        if rows.is_empty() || rows.iter().any(|row| row.from == u64::MAX) {
            return Ok(Vec::new());
        }
        Ok(vec![DagMutation::ReorderNodes(Self { moves: rows.into_iter().rev().map(|row| NodeMove { id: row.id, to: row.from }).collect() })])
    }
}
