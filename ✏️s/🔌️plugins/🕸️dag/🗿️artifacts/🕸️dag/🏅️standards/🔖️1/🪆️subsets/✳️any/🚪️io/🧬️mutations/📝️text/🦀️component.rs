//! 🔧 DAG artifact — OpText/OpBinary codecs for `DagMutation`. Mutation apply/diff/inverse live in
//! `🧬️mutations`; this facet only handcrafts the op wire forms via a local `DagMutationDsl` mirror
//! — exactly like `din16798`'s `Din16798MutationDsl` bridge — since `DagNodeSpec`/`DagNodeKind`
//! can't derive `dsl::DslField` cleanly (their `kind`/`properties` fields aren't boxed; see the
//! doc comment on `DagNodeKindDsl` in `📸️snapshot/🦀️component.rs`). The old bridge into
//! `infinite_board_port_directed_dag::DagMutation` (the foreign kernel port type) is gone with it.

pub use crate::artifacts::dag::schema::mutations::{apply_dag_mutation, inverse_dag_mutation, DagMutation};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::dag::mutations::{
    change_node_abbreviation, change_node_icon, change_node_name, change_node_operator_kind, connect_nodes, create_node, delete_node, disconnect_nodes, move_node, rename_node, reorder_nodes,
    replace_node_kind, replace_node_properties, resize_node,
};
use crate::artifacts::dag::{DagNodeKind, DagNodeSpec};
use infinite_board_port_directed_dag::EdgeRouteStyle;
use graph::manifest::PropertyBag;
use protocol::OpText;

//#region 🔖️OpText
/// ✂️ Local DSL-only mirror of `DagMutation` — every real variant flattened into its own keyworded
/// record, converted at the `store::OpText` boundary only; `DagMutation` itself, and every consumer
/// matching on it, is completely untouched. `DagNodeSpec` (whole node, for `create-node`),
/// `DagNodeKind` (`replace-node-kind`) and `PropertyBag` (`replace-node-properties`,
/// `connect-nodes`'s edge properties) don't have a clean DSL scalar shape, so they round-trip as an
/// opaque `serde_json`-encoded string field here — a documented deviation, not a silent one.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum DagMutationDsl {
    CreateNode {
        node_json: String,
    },
    DeleteNode {
        id: String,
    },
    RenameNode {
        id: String,
        new_id: String,
    },
    ChangeNodeName {
        id: String,
        new_name: String,
    },
    MoveNode {
        id: String,
        x: f64,
        y: f64,
    },
    ResizeNode {
        id: String,
        width: f64,
        height: f64,
    },
    ChangeNodeIcon {
        id: String,
        new_icon: String,
    },
    ChangeNodeAbbreviation {
        id: String,
        new_abbreviation: String,
    },
    ChangeNodeOperatorKind {
        id: String,
        new_operator_kind: Option<String>,
    },
    ReplaceNodeKind {
        id: String,
        new_kind_json: String,
    },
    ReplaceNodeProperties {
        id: String,
        new_properties_json: String,
    },
    ReorderNodes {
        order: Vec<String>,
    },
    ConnectNodes {
        id: String,
        source: String,
        target: String,
        route_style: EdgeRouteStyle,
        properties_json: String,
    },
    DisconnectNodes {
        id: String,
    },
}

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl OpText for DagMutationDsl {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    async fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for DagMutationDsl {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

async fn json_of<T: serde::Serialize>(value: &T) -> String {
    serde_json::to_string(value).expect("dag mutation dsl field must serialize")
}

async fn dag_mutation_to_dsl(mutation: &DagMutation) -> DagMutationDsl {
    match mutation {
        DagMutation::CreateNode(payload) => DagMutationDsl::CreateNode { node_json: json_of(&payload.node) },
        DagMutation::DeleteNode(payload) => DagMutationDsl::DeleteNode { id: payload.id.clone() },
        DagMutation::RenameNode(payload) => DagMutationDsl::RenameNode { id: payload.id.clone(), new_id: payload.new_id.clone() },
        DagMutation::ChangeNodeName(payload) => DagMutationDsl::ChangeNodeName { id: payload.id.clone(), new_name: payload.new_name.clone() },
        DagMutation::MoveNode(payload) => DagMutationDsl::MoveNode { id: payload.id.clone(), x: payload.x, y: payload.y },
        DagMutation::ResizeNode(payload) => DagMutationDsl::ResizeNode { id: payload.id.clone(), width: payload.width, height: payload.height },
        DagMutation::ChangeNodeIcon(payload) => DagMutationDsl::ChangeNodeIcon { id: payload.id.clone(), new_icon: payload.new_icon.clone() },
        DagMutation::ChangeNodeAbbreviation(payload) => DagMutationDsl::ChangeNodeAbbreviation { id: payload.id.clone(), new_abbreviation: payload.new_abbreviation.clone() },
        DagMutation::ChangeNodeOperatorKind(payload) => DagMutationDsl::ChangeNodeOperatorKind { id: payload.id.clone(), new_operator_kind: payload.new_operator_kind.clone() },
        DagMutation::ReplaceNodeKind(payload) => DagMutationDsl::ReplaceNodeKind { id: payload.id.clone(), new_kind_json: json_of(&payload.new_kind) },
        DagMutation::ReplaceNodeProperties(payload) => DagMutationDsl::ReplaceNodeProperties { id: payload.id.clone(), new_properties_json: json_of(&payload.new_properties) },
        DagMutation::ReorderNodes(payload) => DagMutationDsl::ReorderNodes { order: payload.order.clone() },
        DagMutation::ConnectNodes(payload) => {
            DagMutationDsl::ConnectNodes { id: payload.id.clone(), source: payload.source.clone(), target: payload.target.clone(), route_style: payload.route_style, properties_json: json_of(&payload.properties) }
        }
        DagMutation::DisconnectNodes(payload) => DagMutationDsl::DisconnectNodes { id: payload.id.clone() },
    }
}

async fn dag_mutation_from_dsl(mutation: DagMutationDsl) -> DagMutation {
    match mutation {
        DagMutationDsl::CreateNode { node_json } => create_node(serde_json::from_str::<DagNodeSpec>(&node_json).expect("dag mutation dsl `node_json` must decode")),
        DagMutationDsl::DeleteNode { id } => delete_node(id),
        DagMutationDsl::RenameNode { id, new_id } => rename_node(id, new_id),
        DagMutationDsl::ChangeNodeName { id, new_name } => change_node_name(id, new_name),
        DagMutationDsl::MoveNode { id, x, y } => move_node(id, x, y),
        DagMutationDsl::ResizeNode { id, width, height } => resize_node(id, width, height),
        DagMutationDsl::ChangeNodeIcon { id, new_icon } => change_node_icon(id, new_icon),
        DagMutationDsl::ChangeNodeAbbreviation { id, new_abbreviation } => change_node_abbreviation(id, new_abbreviation),
        DagMutationDsl::ChangeNodeOperatorKind { id, new_operator_kind } => change_node_operator_kind(id, new_operator_kind),
        DagMutationDsl::ReplaceNodeKind { id, new_kind_json } => replace_node_kind(id, serde_json::from_str::<DagNodeKind>(&new_kind_json).expect("dag mutation dsl `new_kind_json` must decode")),
        DagMutationDsl::ReplaceNodeProperties { id, new_properties_json } => {
            replace_node_properties(id, serde_json::from_str::<PropertyBag>(&new_properties_json).expect("dag mutation dsl `new_properties_json` must decode"))
        }
        DagMutationDsl::ReorderNodes { order } => reorder_nodes(order),
        DagMutationDsl::ConnectNodes { id, source, target, route_style, properties_json } => {
            connect_nodes(id, source, target, route_style, serde_json::from_str::<PropertyBag>(&properties_json).expect("dag mutation dsl `properties_json` must decode"))
        }
        DagMutationDsl::DisconnectNodes { id } => disconnect_nodes(id),
    }
}

impl OpText for DagMutation {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(dag_mutation_from_dsl(<DagMutationDsl as OpText>::parse_op(line)?))
    }

    async fn print_op(&self) -> String {
        <DagMutationDsl as OpText>::print_op(&dag_mutation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` bridge above — `DagMutationDsl` already derives `OpBinary` via
/// `#[derive(dsl::DslEnum)]`, so this is a pure to/from-dsl forward.
impl protocol::OpBinary for DagMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dag_mutation_to_dsl(self).encode_op()
    }

    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(dag_mutation_from_dsl(DagMutationDsl::decode_op(bytes)?))
    }
}
//#endregion 🔖️OpText

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    async fn sample_node(id: &str) -> DagNodeSpec {
        crate::artifacts::dag::schema::default_node_for_kind("note", id, 0.0, 0.0)
    }

    #[test]
    async fn op_text_round_trips_create_node() {
        store::os_store::test_support::assert_op_line_round_trip(&create_node(sample_node("node-1")));
    }

    #[test]
    async fn op_text_round_trips_move_node() {
        store::os_store::test_support::assert_op_line_round_trip(&move_node("node-1".into(), 5.0, 6.0));
    }

    #[test]
    async fn op_text_round_trips_change_node_operator_kind_none() {
        store::os_store::test_support::assert_op_line_round_trip(&change_node_operator_kind("node-1".into(), None));
    }

    /// ⚖️ Every variant, not just the hand-picked ones above — full-coverage `OpText` round trip
    /// over the closed vocabulary, one sample value per field.
    #[test]
    async fn every_variant_op_text_round_trips() {
        for mutation in every_mutation() {
            store::os_store::test_support::assert_op_line_round_trip(&mutation);
        }
    }

    async fn every_mutation() -> Vec<DagMutation> {
        vec![
            create_node(sample_node("node-1")),
            delete_node("node-1".into()),
            rename_node("node-1".into(), "node-2".into()),
            change_node_name("node-1".into(), "Renamed".into()),
            move_node("node-1".into(), 5.0, 6.0),
            resize_node("node-1".into(), 200.0, 90.0),
            change_node_icon("node-1".into(), "emoji:🎚️".into()),
            change_node_abbreviation("node-1".into(), "Nd".into()),
            change_node_operator_kind("node-1".into(), Some("math.add".into())),
            change_node_operator_kind("node-1".into(), None),
            replace_node_kind("node-1".into(), sample_node("node-1").kind),
            replace_node_properties("node-1".into(), PropertyBag::default()),
            reorder_nodes(vec!["node-2".into(), "node-1".into()]),
            connect_nodes("edge-1".into(), "node-1@out".into(), "node-2@in".into(), EdgeRouteStyle::default(), PropertyBag::default()),
            disconnect_nodes("edge-1".into()),
        ]
    }
}
//#endregion 🧪️Tests
