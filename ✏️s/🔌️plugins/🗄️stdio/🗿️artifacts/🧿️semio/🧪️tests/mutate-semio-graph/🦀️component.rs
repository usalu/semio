//! 🦀️ Semio GRAPH exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-
//! REFACTOR. Recorded no-oracle decision `semio-graph-mutation-semantics` (`../../🏅️standards/
//! 🔖️v1/🪆️subsets/✳️graph/🧪️oracle/🔣️component.json`): `s.stdio.semio.graph` is a semio-NATIVE
//! format with no third-party reader or writer, so `oracle` here reads the committed,
//! independently handcrafted per-kind specification fixtures (`../../🏅️standards/🔖️v1/🪆️subsets/
//! ✳️graph/🧬️schema/🧬️mutations/<kind>/🧪️tests/<fixture>/`) literally — no recomputation, no
//! reimplementation of mutation semantics. `subject` drives this repository's own
//! `apply_semio_graph_mutation`, the entry point this ticket added, over the full 11-kind
//! `SemioGraphMutation` vocabulary. Both sides project the snapshot to structural JSON and
//! `ordered-json-v1` compares them. The oracle-only build must never link the subject crate (fleet
//! brief §5.3), so the fixtures' BEFORE snapshot and MUTATION payload are transcribed once, by
//! hand, as `SemioGraphSnapshot`/`SemioGraphMutation` Rust literals inside the `sut`-gated
//! `subject` module below — mechanically identical to the committed JSON, never independently
//! invented (compare against the JSON embedded via `include_str!` in `oracle_fixture`). The
//! generated test-host crate carries no `serde_json` dependency (only `semio-repo-test-host` and,
//! behind `sut`, this subset's own crate), so parsing committed JSON straight into typed structs is
//! not an option here; the framework's own dependency-free `protocol::Json`/`parse_json` carries
//! the oracle side instead. The subject half is gated behind the generated host's `sut` feature so
//! the oracle-only run never compiles the local implementation; the Rust SUBJECT phase is blocked
//! this wave by a concurrent os-kernel refactor (see the fleet brief), so it is written and gated
//! but not run.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioGraphMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the oracle-only build must not
/// link the subject crate. The contract's mutation-coverage gate keeps this list honest against the
/// catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against
/// the enum.
const KINDS: &[&str] = &["create-node", "delete-node", "change-node-kind", "change-node-label", "move-node", "add-node-port", "remove-node-port", "add-node-property", "remove-node-property", "create-edge", "delete-edge"];
//#endregion 🔖️Kinds

//#region 🔖️OracleFixtures
/// 🧫️ The committed `(before, after)` snapshot JSON for one kind, read literally — this IS the
/// independently handcrafted specification vector the no-oracle decision rests on, never
/// recomputed.
fn oracle_fixture(kind: &str) -> (&'static str, &'static str) {
    match kind {
        "create-node" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🏗️create-node/🧪️tests/appends-a-filter-node-to-the-end-of-the-node-set/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🏗️create-node/🧪️tests/appends-a-filter-node-to-the-end-of-the-node-set/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "delete-node" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🗑️delete-node/🧪️tests/removes-the-sink-node-and-severs-the-edge-into-it/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🗑️delete-node/🧪️tests/removes-the-sink-node-and-severs-the-edge-into-it/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "change-node-kind" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔧change-node-kind/🧪️tests/retypes-the-source-node-without-relabelling-it/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔧change-node-kind/🧪️tests/retypes-the-source-node-without-relabelling-it/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "change-node-label" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🖍️change-node-label/🧪️tests/relabels-the-source-node-without-retyping-it/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🖍️change-node-label/🧪️tests/relabels-the-source-node-without-retyping-it/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "move-node" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/📍move-node/🧪️tests/moves-the-sink-node-to-a-new-canvas-position/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/📍move-node/🧪️tests/moves-the-sink-node-to-a-new-canvas-position/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "add-node-port" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔌add-node-port/🧪️tests/inserts-an-in-port-ahead-of-the-existing-out-port/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔌add-node-port/🧪️tests/inserts-an-in-port-ahead-of-the-existing-out-port/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "remove-node-port" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔚remove-node-port/🧪️tests/detaches-the-trailing-out-port-from-the-source-node/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔚remove-node-port/🧪️tests/detaches-the-trailing-out-port-from-the-source-node/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "add-node-property" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/➕add-node-property/🧪️tests/inserts-a-weight-property-ahead-of-the-colour-property/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/➕add-node-property/🧪️tests/inserts-a-weight-property-ahead-of-the-colour-property/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "remove-node-property" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/➖remove-node-property/🧪️tests/detaches-the-trailing-weight-property-from-the-source-node/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/➖remove-node-property/🧪️tests/detaches-the-trailing-weight-property-from-the-source-node/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "create-edge" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔗create-edge/🧪️tests/connects-the-source-node-to-the-sink-node/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔗create-edge/🧪️tests/connects-the-source-node-to-the-sink-node/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "delete-edge" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/✂️delete-edge/🧪️tests/removes-the-feedback-edge-and-keeps-both-endpoints/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/✂️delete-edge/🧪️tests/removes-the-feedback-edge-and-keeps-both-endpoints/📸️snapshot/➡️after/🔣️component.json"),
        ),
        other => panic!("mutate-semio-graph: no fixture registered for kind {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("committed fixture JSON must parse: {error}"))
}
//#endregion 🔖️OracleFixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER snapshot, read literally.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (_before, after) = oracle_fixture(kind);
        Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))
    }
}

/// 🔮️ The inverse reference answer: the committed BEFORE snapshot — undoing any mutation must
/// return to exactly where the specification vector started.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (before, _after) = oracle_fixture(kind);
        Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
    }
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::graph::schema::mutations::{add_node_port, add_node_property, change_node_kind, change_node_label, create_edge, create_node, delete_edge, delete_node, move_node, remove_node_port, remove_node_property, SemioGraphMutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{GraphEdgeId, GraphNodeId, SemioGraphEdge, SemioGraphNode, SemioGraphPort, SemioGraphPortKind, SemioGraphSnapshot};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry};
    use protocol::Mutation;

    //#region 🔖️HandcraftedFixtures
    /// 🧫️ The SAME specification vector `../🦀️component.rs::oracle_fixture` embeds as JSON,
    /// transcribed once by hand into real `SemioGraphSnapshot`/`SemioGraphMutation` values — the
    /// oracle-only build must never link this crate, so there is no way to share one physical
    /// source between the two roles; committed side by side under the same kind's `🧪️tests/`
    /// directory, so a drift between them is a one-file diff away from being caught by eye.
    fn nid(value: &str) -> GraphNodeId {
        GraphNodeId { value: value.into() }
    }
    fn eid(value: &str) -> GraphEdgeId {
        GraphEdgeId { value: value.into() }
    }
    fn port(name: &str, kind: SemioGraphPortKind) -> SemioGraphPort {
        SemioGraphPort { name: name.into(), kind }
    }
    fn prop(key: &str, value: SemioValue) -> SemioValueEntry {
        SemioValueEntry { key: key.into(), value }
    }
    fn node(id: &str, kind: &str, label: &str, x: f64, y: f64, ports: Vec<SemioGraphPort>, properties: Vec<SemioValueEntry>) -> SemioGraphNode {
        SemioGraphNode { id: nid(id), kind: kind.into(), label: label.into(), position: SemioPoint2 { x, y }, ports, properties }
    }
    fn edge(id: &str, source: &str, target: &str, kind: &str, label: &str) -> SemioGraphEdge {
        SemioGraphEdge { id: eid(id), source: nid(source), target: nid(target), kind: kind.into(), label: label.into() }
    }
    fn snapshot(nodes: Vec<SemioGraphNode>, edges: Vec<SemioGraphEdge>) -> SemioGraphSnapshot {
        SemioGraphSnapshot { schema: "s.stdio.semio.graph".into(), nodes, edges }
    }
    /// 🕸️ The two-node/one-edge graph shared by every kind whose fixture leaves ports/properties
    /// untouched (`create-node`, `delete-node`, `change-node-kind`, `change-node-label`,
    /// `move-node`) — matches the committed BEFORE JSON field for field.
    fn base_graph() -> SemioGraphSnapshot {
        snapshot(vec![node("a", "source", "Source", 0.0, 0.0, vec![], vec![]), node("b", "sink", "Sink", 4.0, 0.0, vec![], vec![])], vec![edge("e1", "a", "b", "data", "A to B")])
    }

    fn fixture_for(kind: &str) -> (SemioGraphSnapshot, SemioGraphMutation) {
        match kind {
            "create-node" => (base_graph(), SemioGraphMutation::CreateNode(create_node::mutation::CreateNode { id: nid("c"), kind: "filter".into(), label: "Filter".into(), position: SemioPoint2 { x: 2.0, y: 2.0 }, ports: vec![], properties: vec![] })),
            "delete-node" => (base_graph(), SemioGraphMutation::DeleteNode(delete_node::mutation::DeleteNode { id: nid("b") })),
            "change-node-kind" => (base_graph(), SemioGraphMutation::ChangeNodeKind(change_node_kind::mutation::ChangeNodeKind { id: nid("a"), new_kind: "generator".into() })),
            "change-node-label" => (base_graph(), SemioGraphMutation::ChangeNodeLabel(change_node_label::mutation::ChangeNodeLabel { id: nid("a"), new_label: "Sensor".into() })),
            "move-node" => (base_graph(), SemioGraphMutation::MoveNode(move_node::mutation::MoveNode { id: nid("b"), new_position: SemioPoint2 { x: 6.0, y: -2.5 } })),
            "add-node-port" => (
                snapshot(vec![node("a", "source", "Source", 0.0, 0.0, vec![port("out", SemioGraphPortKind::Out)], vec![]), node("b", "sink", "Sink", 4.0, 0.0, vec![], vec![])], vec![edge("e1", "a", "b", "data", "A to B")]),
                SemioGraphMutation::AddNodePort(add_node_port::mutation::AddNodePort { node_id: nid("a"), index: 0, port: port("reset", SemioGraphPortKind::In) }),
            ),
            "remove-node-port" => (
                snapshot(
                    vec![node("a", "source", "Source", 0.0, 0.0, vec![port("reset", SemioGraphPortKind::In), port("out", SemioGraphPortKind::Out)], vec![]), node("b", "sink", "Sink", 4.0, 0.0, vec![], vec![])],
                    vec![edge("e1", "a", "b", "data", "A to B")],
                ),
                SemioGraphMutation::RemoveNodePort(remove_node_port::mutation::RemoveNodePort { node_id: nid("a"), index: 1 }),
            ),
            "add-node-property" => (
                snapshot(vec![node("a", "source", "Source", 0.0, 0.0, vec![], vec![prop("colour", SemioValue::Str { value: "red".into() })]), node("b", "sink", "Sink", 4.0, 0.0, vec![], vec![])], vec![edge("e1", "a", "b", "data", "A to B")]),
                SemioGraphMutation::AddNodeProperty(add_node_property::mutation::AddNodeProperty { node_id: nid("a"), index: 0, property: prop("weight", SemioValue::Float { lexeme: "0.5".into() }) }),
            ),
            "remove-node-property" => (
                snapshot(
                    vec![
                        node("a", "source", "Source", 0.0, 0.0, vec![], vec![prop("colour", SemioValue::Str { value: "red".into() }), prop("weight", SemioValue::Float { lexeme: "0.5".into() })]),
                        node("b", "sink", "Sink", 4.0, 0.0, vec![], vec![]),
                    ],
                    vec![edge("e1", "a", "b", "data", "A to B")],
                ),
                SemioGraphMutation::RemoveNodeProperty(remove_node_property::mutation::RemoveNodeProperty { node_id: nid("a"), index: 1 }),
            ),
            "create-edge" => (
                snapshot(vec![node("a", "source", "Source", 0.0, 0.0, vec![], vec![]), node("b", "sink", "Sink", 4.0, 0.0, vec![], vec![])], vec![]),
                SemioGraphMutation::CreateEdge(create_edge::mutation::CreateEdge { id: eid("e1"), source: nid("a"), target: nid("b"), kind: "data".into(), label: "A to B".into() }),
            ),
            "delete-edge" => (
                snapshot(
                    vec![node("a", "source", "Source", 0.0, 0.0, vec![], vec![]), node("b", "sink", "Sink", 4.0, 0.0, vec![], vec![])],
                    vec![edge("e1", "a", "b", "data", "A to B"), edge("e2", "b", "a", "feedback", "B back to A")],
                ),
                SemioGraphMutation::DeleteEdge(delete_edge::mutation::DeleteEdge { id: eid("e2") }),
            ),
            other => panic!("mutate-semio-graph: no fixture registered for kind {other:?}"),
        }
    }
    //#endregion 🔖️HandcraftedFixtures

    //#region 🔖️Projection
    fn port_kind_str(kind: SemioGraphPortKind) -> &'static str {
        match kind {
            SemioGraphPortKind::In => "in",
            SemioGraphPortKind::Out => "out",
            SemioGraphPortKind::InOut => "inOut",
        }
    }
    fn id_json(value: &str) -> Json {
        Json::Object(vec![("value".to_string(), Json::String(value.to_string()))])
    }
    fn port_json(port: &SemioGraphPort) -> Json {
        Json::Object(vec![("name".to_string(), Json::String(port.name.clone())), ("kind".to_string(), Json::String(port_kind_str(port.kind).to_string()))])
    }
    fn value_json(value: &SemioValue) -> Json {
        match value {
            SemioValue::Null => Json::Object(vec![("kind".to_string(), Json::String("null".to_string()))]),
            SemioValue::Bool { value } => Json::Object(vec![("kind".to_string(), Json::String("bool".to_string())), ("value".to_string(), Json::Bool(*value))]),
            SemioValue::Int { lexeme } => Json::Object(vec![("kind".to_string(), Json::String("int".to_string())), ("lexeme".to_string(), Json::String(lexeme.clone()))]),
            SemioValue::Float { lexeme } => Json::Object(vec![("kind".to_string(), Json::String("float".to_string())), ("lexeme".to_string(), Json::String(lexeme.clone()))]),
            SemioValue::Str { value } => Json::Object(vec![("kind".to_string(), Json::String("str".to_string())), ("value".to_string(), Json::String(value.clone()))]),
            SemioValue::Bytes { value } => Json::Object(vec![("kind".to_string(), Json::String("bytes".to_string())), ("value".to_string(), Json::Array(value.iter().map(|b| Json::Number(*b as f64)).collect()))]),
            SemioValue::List { items } => Json::Object(vec![("kind".to_string(), Json::String("list".to_string())), ("items".to_string(), Json::Array(items.iter().map(value_json).collect()))]),
            SemioValue::Map { entries } => Json::Object(vec![("kind".to_string(), Json::String("map".to_string())), ("entries".to_string(), Json::Array(entries.iter().map(property_json).collect()))]),
            SemioValue::Ref { id } => Json::Object(vec![("kind".to_string(), Json::String("ref".to_string())), ("id".to_string(), id_json(&id.value))]),
        }
    }
    fn property_json(property: &SemioValueEntry) -> Json {
        Json::Object(vec![("key".to_string(), Json::String(property.key.clone())), ("value".to_string(), value_json(&property.value))])
    }
    fn node_json(node: &SemioGraphNode) -> Json {
        Json::Object(vec![
            ("id".to_string(), id_json(&node.id.value)),
            ("kind".to_string(), Json::String(node.kind.clone())),
            ("label".to_string(), Json::String(node.label.clone())),
            ("position".to_string(), Json::Object(vec![("x".to_string(), Json::Number(node.position.x)), ("y".to_string(), Json::Number(node.position.y))])),
            ("ports".to_string(), Json::Array(node.ports.iter().map(port_json).collect())),
            ("properties".to_string(), Json::Array(node.properties.iter().map(property_json).collect())),
        ])
    }
    fn edge_json(edge: &SemioGraphEdge) -> Json {
        Json::Object(vec![
            ("id".to_string(), id_json(&edge.id.value)),
            ("source".to_string(), id_json(&edge.source.value)),
            ("target".to_string(), id_json(&edge.target.value)),
            ("kind".to_string(), Json::String(edge.kind.clone())),
            ("label".to_string(), Json::String(edge.label.clone())),
        ])
    }
    /// 🎯️ The projection every scenario compares under `ordered-json-v1`: the snapshot's own
    /// structural JSON shape, matching the committed fixtures field for field.
    fn snapshot_json(snapshot: &SemioGraphSnapshot) -> Json {
        Json::Object(vec![("schema".to_string(), Json::String(snapshot.schema.clone())), ("nodes".to_string(), Json::Array(snapshot.nodes.iter().map(node_json).collect())), ("edges".to_string(), Json::Array(snapshot.edges.iter().map(edge_json).collect()))])
    }
    //#endregion 🔖️Projection

    //#region 🔖️Handlers
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (mut base, mutation) = fixture_for(kind);
            let outcome = semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::graph::schema::mutations::apply_semio_graph_mutation(&mut base, &mutation);
            if !outcome.messages().is_empty() {
                return Err(format!("mutate-{kind}: mutation rejected: {:?}", outcome.messages()));
            }
            let projection = snapshot_json(&base);
            let bytes = projection.to_string().into_bytes();
            Ok(Outcome::with_raw(bytes, projection))
        }
    }

    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (base, mutation) = fixture_for(kind);
            let mut current = base.clone();
            let outcome = semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::graph::schema::mutations::apply_semio_graph_mutation(&mut current, &mutation);
            if !outcome.messages().is_empty() {
                return Err(format!("inverse-{kind}: forward mutation rejected: {:?}", outcome.messages()));
            }
            let undo = mutation.inverse(&base);
            for step in &undo {
                let step_outcome = semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::graph::schema::mutations::apply_semio_graph_mutation(&mut current, step);
                if !step_outcome.messages().is_empty() {
                    return Err(format!("inverse-{kind}: inverse step rejected: {:?}", step_outcome.messages()));
                }
            }
            let projection = snapshot_json(&current);
            let bytes = projection.to_string().into_bytes();
            Ok(Outcome::with_raw(bytes, projection))
        }
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle_for(kind)).oracle(&format!("inverse-{kind}"), inverse_oracle_for(kind));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind));
        }
    }
    built
}
//#endregion 🔖️Registration
