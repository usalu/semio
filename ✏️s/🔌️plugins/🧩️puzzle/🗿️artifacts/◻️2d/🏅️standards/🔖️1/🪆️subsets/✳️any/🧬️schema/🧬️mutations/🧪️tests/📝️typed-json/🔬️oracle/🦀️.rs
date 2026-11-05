//! 🔬️ Independent Serde reference for the closed typed-mutation v2 fixture vocabulary.
use super::{Puzzle2dMutation, Puzzle2dSnapshot};
use semio_s_artifact_puzzle_2d_records::standards::v1::subsets::any::schema::records::{Puzzle2dNode, Puzzle2dNodeAnchor};
use serde::Deserialize;
use serde_json::Value;

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
enum Anchor {
    #[default]
    Fixed,
    Derived,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Node {
    id: String,
    #[serde(default)]
    anchor: Anchor,
    x: f64,
    y: f64,
    #[serde(default)]
    handles: Vec<Value>,
}

impl Node {
    fn into_domain(self) -> Result<Puzzle2dNode, String> {
        if !self.handles.is_empty() {
            return Err("typed-mutation v2 reference requires an empty handle roster".into());
        }
        Ok(Puzzle2dNode {
            id: self.id.into(),
            anchor: match self.anchor { Anchor::Fixed => Puzzle2dNodeAnchor::Fixed, Anchor::Derived => Puzzle2dNodeAnchor::Derived },
            x: self.x,
            y: self.y,
            ..Default::default()
        })
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Snapshot {
    schema: String,
    #[serde(default)]
    nodes: Vec<Node>,
    #[serde(default)]
    edges: Vec<Value>,
}

/// 📸️ Builds the fixture's typed state directly from independently validated scalar fields.
pub(super) fn snapshot(value: &Value) -> Result<Puzzle2dSnapshot, String> {
    let input: Snapshot = serde_json::from_value(value.clone()).map_err(|error| error.to_string())?;
    if !input.edges.is_empty() || input.nodes.iter().any(|node| !node.handles.is_empty()) {
        return Err("typed-mutation v2 reference requires empty handle and edge rosters".into());
    }
    Ok(Puzzle2dSnapshot {
        schema: input.schema.into(),
        nodes: input.nodes.into_iter().map(Node::into_domain).collect::<Result<Vec<_>, _>>()?.into(),
        ..Default::default()
    })
}

#[derive(Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase", deny_unknown_fields)]
enum Mutation {
    DeleteNode { id: String },
    MoveNode { id: String, #[serde(rename = "newX")] x: f64, #[serde(rename = "newY")] y: f64 },
    CreateNode { node: Node, index: Option<usize> },
}

/// 🦠️ Constructs the fixture's semantic vocabulary without calling the owned JSON/value decoder.
pub(super) fn mutation(value: &Value) -> Result<Puzzle2dMutation, String> {
    let input: Mutation = serde_json::from_value(value.clone()).map_err(|error| error.to_string())?;
    Ok(match input {
        Mutation::DeleteNode { id } => super::delete_node(id.into()),
        Mutation::MoveNode { id, x, y } => super::move_node(id.into(), x, y),
        Mutation::CreateNode { node, index } => super::create_node(node.into_domain()?, index),
    })
}

/// 🧹️ Pays the existing native fixture source's genuine terminal retirement for typed reference owners.
pub(super) fn retire<T: semio_framework_value::retirement::RetireOwned + Send + Sync + 'static>(value: T) {
    let mut source = semio_s_artifact_puzzle_2d_records::standards::v1::subsets::any::schema::testkit::admit_source(std::sync::Arc::new(value));
    semio_s_artifact_puzzle_2d_records::standards::v1::subsets::any::schema::testkit::close_source(&mut source);
}
