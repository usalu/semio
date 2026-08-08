//! 🧬️ Puzzle 2d artifact — the operation enum and its laws: id-keyed node/edge edits plus scalar
//! meta, each with a true inverse computed from the pre-operation projection, and a whole-document
//! replace for example loads. The `serde_json::Value` bridge (`🔖️ValueBridge`) and the play app's
//! `Puzzle2dPlayProjection` newtype (`🔖️PlayProjection`) live here too, beside the `Mutation`/
//! `MutationDiff` impls that give them meaning.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::puzzle2d::diff::{puzzle2d_index_of, Puzzle2dDiff};
use crate::artifacts::puzzle2d::{Puzzle2dEdge, Puzzle2dMeta, Puzzle2dNode, Puzzle2dSnapshot};
use protocol::{Mutation, MutationDiff};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️Operations
/// 🧮️ Puzzle-2d operation: id-keyed node/edge edits plus scalar meta, each with a true inverse
/// computed from the pre-operation projection, and a whole-document replace for example loads.
/// There is deliberately no camera operation: the camera is session-only `Puzzle2dPlayRuntime`
/// state in the play app (see `setCamera`'s `ActionKind::View`), never a VCS-tracked document edit.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum Puzzle2dMutation {
    #[dsl(key = "setNode")]
    SetNode {
        index: usize,
        #[dsl(block)]
        node: Puzzle2dNode,
    },
    #[dsl(key = "removeNode")]
    RemoveNode { id: String },
    #[dsl(key = "setEdge")]
    SetEdge {
        index: usize,
        #[dsl(block)]
        edge: Puzzle2dEdge,
    },
    #[dsl(key = "removeEdge")]
    RemoveEdge { id: String },
    #[dsl(key = "setMeta")]
    SetMeta {
        #[dsl(block)]
        meta: Puzzle2dMeta,
    },
    /// 🌍️ Replaces the whole document (example import / reset / engine fill).
    #[dsl(key = "setDocument")]
    SetDocument {
        #[dsl(block)]
        snapshot: Puzzle2dSnapshot,
    },
}





fn puzzle2d_mutation_diff(operation: &Puzzle2dMutation, base: &Puzzle2dSnapshot) -> Puzzle2dDiff {
    match operation {
        Puzzle2dMutation::SetNode { index, node } => crate::artifacts::puzzle2d::diff::diff_set_node(*index, node.clone(), base),
        Puzzle2dMutation::RemoveNode { id } => crate::artifacts::puzzle2d::diff::diff_remove_node(id.clone()),
        Puzzle2dMutation::SetEdge { index, edge } => crate::artifacts::puzzle2d::diff::diff_set_edge(*index, edge.clone(), base),
        Puzzle2dMutation::RemoveEdge { id } => crate::artifacts::puzzle2d::diff::diff_remove_edge(id.clone()),
        Puzzle2dMutation::SetMeta { meta } => crate::artifacts::puzzle2d::diff::diff_set_meta(meta.clone()),
        Puzzle2dMutation::SetDocument { snapshot } => crate::artifacts::puzzle2d::diff::diff_set_snapshot(snapshot.clone()),
    }
}

impl Mutation<Puzzle2dSnapshot> for Puzzle2dMutation {
    type Diff = Puzzle2dDiff;

    fn diff(&self, projection: &Puzzle2dSnapshot) -> Puzzle2dDiff {
        puzzle2d_mutation_diff(self, projection)
    }

    fn inverse(&self, projection: &Puzzle2dSnapshot) -> Vec<Self> {
        match self {
            Puzzle2dMutation::SetNode { node, .. } => match puzzle2d_index_of(&projection.nodes, &node.id) {
                Some(index) => vec![Puzzle2dMutation::SetNode { index, node: projection.nodes[index].clone() }],
                None => vec![Puzzle2dMutation::RemoveNode { id: node.id.clone() }],
            },
            Puzzle2dMutation::RemoveNode { id } => puzzle2d_index_of(&projection.nodes, id).map(|index| vec![Puzzle2dMutation::SetNode { index, node: projection.nodes[index].clone() }]).unwrap_or_default(),
            Puzzle2dMutation::SetEdge { edge, .. } => match puzzle2d_index_of(&projection.edges, &edge.id) {
                Some(index) => vec![Puzzle2dMutation::SetEdge { index, edge: projection.edges[index].clone() }],
                None => vec![Puzzle2dMutation::RemoveEdge { id: edge.id.clone() }],
            },
            Puzzle2dMutation::RemoveEdge { id } => puzzle2d_index_of(&projection.edges, id).map(|index| vec![Puzzle2dMutation::SetEdge { index, edge: projection.edges[index].clone() }]).unwrap_or_default(),
            Puzzle2dMutation::SetMeta { .. } => vec![Puzzle2dMutation::SetMeta { meta: projection.meta.clone() }],
            Puzzle2dMutation::SetDocument { .. } => vec![Puzzle2dMutation::SetDocument { snapshot: projection.clone() }],
        }
    }
}
//#endregion 🔖️Operations

//#region 🔖️ValueBridge
// 🌉️ `puzzle-plugin`'s scene-mutation helpers predate this typed projection and stay on a bare
// `serde_json::Value` scratch fixture (out of scope for this ticket — see
// `.🦑️repo/🎫️tickets/…/convertpuzzle2d3d5dtotypeddslderiveengine`). Bridging `Puzzle2dMutation`/`Puzzle2dDiff`
// onto that `Value` boundary too keeps `puzzle2d_document_delta_operations(&Value, &Value)` and the
// plugin's `DocumentApp::Projection = Value` compiling unchanged: `apply` serializes the typed
// payload back to JSON and splices it into the id-keyed array/field exactly like the pre-migration
// untyped operation did.
fn puzzle2d_value_item_id(item: &Value) -> Option<&str> {
    item.get("id").and_then(|value| value.as_str())
}

/// 🩹️ Replaces the id-matching entry in place, else inserts at `index` (clamped to the current
/// length) — matching `apply_puzzle2d_collection_diff`'s insert-at-recorded-index semantics on the
/// typed projection, so undo/redo restores the original array position on this `Value` bridge too.
fn puzzle2d_upsert_value_item(document: &mut Value, collection: &str, index: usize, item: Value) {
    let Some(object) = document.as_object_mut() else {
        return;
    };
    let array = object.entry(collection.to_string()).or_insert_with(|| Value::Array(Vec::new()));
    let Some(array) = array.as_array_mut() else {
        return;
    };
    if let Some(id) = puzzle2d_value_item_id(&item).map(str::to_string) {
        if let Some(slot) = array.iter_mut().find(|entry| puzzle2d_value_item_id(entry) == Some(id.as_str())) {
            *slot = item;
            return;
        }
    }
    array.insert(index.min(array.len()), item);
}

fn puzzle2d_remove_value_item(document: &mut Value, collection: &str, id: &str) {
    if let Some(array) = document.get_mut(collection).and_then(|value| value.as_array_mut()) {
        array.retain(|entry| puzzle2d_value_item_id(entry) != Some(id));
    }
}

fn apply_puzzle2d_operation_to_value(document: &mut Value, operation: &Puzzle2dMutation) {
    match operation {
        Puzzle2dMutation::SetNode { index, node } => puzzle2d_upsert_value_item(document, "nodes", *index, serde_json::to_value(node).unwrap_or(Value::Null)),
        Puzzle2dMutation::RemoveNode { id } => puzzle2d_remove_value_item(document, "nodes", id),
        Puzzle2dMutation::SetEdge { index, edge } => puzzle2d_upsert_value_item(document, "edges", *index, serde_json::to_value(edge).unwrap_or(Value::Null)),
        Puzzle2dMutation::RemoveEdge { id } => puzzle2d_remove_value_item(document, "edges", id),
        Puzzle2dMutation::SetMeta { meta } => {
            if let Some(object) = document.as_object_mut() {
                object.insert("meta".to_string(), serde_json::to_value(meta).unwrap_or(Value::Null));
            }
        }
        Puzzle2dMutation::SetDocument { snapshot: next } => *document = serde_json::to_value(next).unwrap_or_else(|_| document.clone()),
    }
}

fn puzzle2d_value_collection<'a>(document: &'a Value, collection: &str) -> &'a [Value] {
    document.get(collection).and_then(|value| value.as_array()).map_or(&[][..], Vec::as_slice)
}

fn puzzle2d_value_item_index<T: serde::de::DeserializeOwned>(document: &Value, collection: &str, id: &str) -> Option<(usize, T)> {
    let items = puzzle2d_value_collection(document, collection);
    let index = items.iter().position(|entry| puzzle2d_value_item_id(entry) == Some(id))?;
    serde_json::from_value(items[index].clone()).ok().map(|item| (index, item))
}

fn puzzle2d_reorder_value_collection(document: &mut Value, collection: &str, order: &[String]) {
    let Some(array) = document.get_mut(collection).and_then(|value| value.as_array_mut()) else {
        return;
    };
    let mut by_id: std::collections::BTreeMap<String, Value> = std::collections::BTreeMap::new();
    for item in array.drain(..) {
        if let Some(id) = puzzle2d_value_item_id(&item).map(str::to_string) {
            by_id.insert(id, item);
        }
    }
    let mut ordered = Vec::with_capacity(order.len());
    for id in order {
        if let Some(item) = by_id.remove(id) {
            ordered.push(item);
        }
    }
    ordered.extend(by_id.into_values());
    *array = ordered;
}

impl MutationDiff<Value> for Puzzle2dDiff {
    fn apply(&self, projection: &Value) -> Value {
        if let Some(artifact) = &self.artifact {
            return serde_json::to_value(artifact.to_snapshot()).unwrap_or_else(|_| projection.clone());
        }
        let mut next = projection.clone();
        if let Some(delta) = &self.nodes {
            for id in &delta.removed {
                puzzle2d_remove_value_item(&mut next, "nodes", id);
            }
            for node in &delta.added {
                puzzle2d_upsert_value_item(&mut next, "nodes", usize::MAX, serde_json::to_value(node).unwrap_or(Value::Null));
            }
            for entry in &delta.patched {
                if let Some(node) = &entry.patch.replacement {
                    puzzle2d_upsert_value_item(&mut next, "nodes", usize::MAX, serde_json::to_value(node).unwrap_or(Value::Null));
                }
            }
            if let Some(order) = &delta.reordered {
                puzzle2d_reorder_value_collection(&mut next, "nodes", order);
            }
        }
        if let Some(delta) = &self.edges {
            for id in &delta.removed {
                puzzle2d_remove_value_item(&mut next, "edges", id);
            }
            for edge in &delta.added {
                puzzle2d_upsert_value_item(&mut next, "edges", usize::MAX, serde_json::to_value(edge).unwrap_or(Value::Null));
            }
            for entry in &delta.patched {
                if let Some(edge) = &entry.patch.replacement {
                    puzzle2d_upsert_value_item(&mut next, "edges", usize::MAX, serde_json::to_value(edge).unwrap_or(Value::Null));
                }
            }
            if let Some(order) = &delta.reordered {
                puzzle2d_reorder_value_collection(&mut next, "edges", order);
            }
        }
        if let Some(meta) = &self.meta {
            if let Some(object) = next.as_object_mut() {
                object.insert("meta".to_string(), serde_json::to_value(meta).unwrap_or(Value::Null));
            }
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        MutationDiff::<Puzzle2dSnapshot>::absorb(self, other);
    }
}

impl Mutation<Value> for Puzzle2dMutation {
    type Diff = Puzzle2dDiff;

    fn diff(&self, projection: &Value) -> Puzzle2dDiff {
        let base: Puzzle2dSnapshot = serde_json::from_value(projection.clone()).unwrap_or_default();
        puzzle2d_mutation_diff(self, &base)
    }

    fn inverse(&self, projection: &Value) -> Vec<Self> {
        match self {
            Puzzle2dMutation::SetNode { node, .. } => match puzzle2d_value_item_index::<Puzzle2dNode>(projection, "nodes", &node.id) {
                Some((index, previous)) => vec![Puzzle2dMutation::SetNode { index, node: previous }],
                None => vec![Puzzle2dMutation::RemoveNode { id: node.id.clone() }],
            },
            Puzzle2dMutation::RemoveNode { id } => puzzle2d_value_item_index::<Puzzle2dNode>(projection, "nodes", id).map(|(index, previous)| vec![Puzzle2dMutation::SetNode { index, node: previous }]).unwrap_or_default(),
            Puzzle2dMutation::SetEdge { edge, .. } => match puzzle2d_value_item_index::<Puzzle2dEdge>(projection, "edges", &edge.id) {
                Some((index, previous)) => vec![Puzzle2dMutation::SetEdge { index, edge: previous }],
                None => vec![Puzzle2dMutation::RemoveEdge { id: edge.id.clone() }],
            },
            Puzzle2dMutation::RemoveEdge { id } => puzzle2d_value_item_index::<Puzzle2dEdge>(projection, "edges", id).map(|(index, previous)| vec![Puzzle2dMutation::SetEdge { index, edge: previous }]).unwrap_or_default(),
            Puzzle2dMutation::SetMeta { .. } => {
                let meta: Puzzle2dMeta = projection.get("meta").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
                vec![Puzzle2dMutation::SetMeta { meta }]
            }
            Puzzle2dMutation::SetDocument { .. } => vec![Puzzle2dMutation::SetDocument { snapshot: serde_json::from_value(projection.clone()).unwrap_or_default() }],
        }
    }
}

/// 🧮️ Collects the sparse `set`/`removed` delta for one id-keyed `Value` array collection into typed
/// entries. Returns `false` (caller falls back to `SetDocument`) whenever an entry is missing an
/// `id` or fails to deserialize into `T` — the granular path only ever fires when it's exact.
fn puzzle2d_collect_value_collection_delta<T>(before: &[Value], after: &[Value], set: &mut Vec<(usize, T)>, removed: &mut Vec<String>) -> bool
where
    T: serde::de::DeserializeOwned,
{
    for (index, entry) in after.iter().enumerate() {
        let Some(id) = puzzle2d_value_item_id(entry) else {
            return false;
        };
        if before.iter().find(|candidate| puzzle2d_value_item_id(candidate) == Some(id)) != Some(entry) {
            let Ok(item) = serde_json::from_value::<T>(entry.clone()) else {
                return false;
            };
            set.push((index, item));
        }
    }
    for entry in before {
        let Some(id) = puzzle2d_value_item_id(entry) else {
            return false;
        };
        if !after.iter().any(|candidate| puzzle2d_value_item_id(candidate) == Some(id)) {
            removed.push(id.to_string());
        }
    }
    true
}

/// 🧮️ Computes the granular typed operation sequence turning `before` into `after` (both the bare
/// fixture JSON `puzzle-plugin` mutates). Node/edge arrays diff per element id; meta becomes
/// `SetMeta`. Falls back to a single `SetDocument` whenever the granular replay would not reproduce
/// `after` exactly (reorders, id-less entries, malformed entries, unrecognized top-level keys,
/// schema changes) — so the emitted operations are always exact while staying granular for the
/// common edits. The camera is deliberately not a known key: it is session-only
/// `Puzzle2dPlayRuntime` state (see `setCamera`'s `ActionKind::View`), never persisted on the
/// document, so a fixture must never carry a top-level `"camera"` key at all.
pub fn puzzle2d_document_delta_operations(before: &Value, after: &Value) -> Vec<Puzzle2dMutation> {
    if before == after {
        return Vec::new();
    }
    let fallback = |after: &Value| vec![Puzzle2dMutation::SetDocument { snapshot: serde_json::from_value(after.clone()).unwrap_or_default() }];
    let (Some(before_object), Some(after_object)) = (before.as_object(), after.as_object()) else {
        return fallback(after);
    };
    const KNOWN_KEYS: [&str; 4] = ["schema", "nodes", "edges", "meta"];
    if before_object.keys().chain(after_object.keys()).any(|key| !KNOWN_KEYS.contains(&key.as_str())) {
        return fallback(after);
    }
    if before_object.get("schema") != after_object.get("schema") {
        return fallback(after);
    }
    let mut operations = Vec::new();
    let before_nodes = before_object.get("nodes").and_then(|value| value.as_array()).map_or(&[][..], Vec::as_slice);
    let after_nodes = after_object.get("nodes").and_then(|value| value.as_array()).map_or(&[][..], Vec::as_slice);
    if before_nodes != after_nodes {
        let mut set = Vec::new();
        let mut removed = Vec::new();
        if !puzzle2d_collect_value_collection_delta::<Puzzle2dNode>(before_nodes, after_nodes, &mut set, &mut removed) {
            return fallback(after);
        }
        operations.extend(removed.into_iter().map(|id| Puzzle2dMutation::RemoveNode { id }));
        operations.extend(set.into_iter().map(|(index, node)| Puzzle2dMutation::SetNode { index, node }));
    }
    let before_edges = before_object.get("edges").and_then(|value| value.as_array()).map_or(&[][..], Vec::as_slice);
    let after_edges = after_object.get("edges").and_then(|value| value.as_array()).map_or(&[][..], Vec::as_slice);
    if before_edges != after_edges {
        let mut set = Vec::new();
        let mut removed = Vec::new();
        if !puzzle2d_collect_value_collection_delta::<Puzzle2dEdge>(before_edges, after_edges, &mut set, &mut removed) {
            return fallback(after);
        }
        operations.extend(removed.into_iter().map(|id| Puzzle2dMutation::RemoveEdge { id }));
        operations.extend(set.into_iter().map(|(index, edge)| Puzzle2dMutation::SetEdge { index, edge }));
    }
    let before_meta = before_object.get("meta");
    let after_meta = after_object.get("meta");
    if before_meta != after_meta {
        let meta = match after_meta {
            Some(value) => match serde_json::from_value::<Puzzle2dMeta>(value.clone()) {
                Ok(meta) => meta,
                Err(_) => return fallback(after),
            },
            None => Puzzle2dMeta::default(),
        };
        operations.push(Puzzle2dMutation::SetMeta { meta });
    }
    let mut replay = before.clone();
    for operation in &operations {
        apply_puzzle2d_operation_to_value(&mut replay, operation);
    }
    if &replay == after {
        operations
    } else {
        fallback(after)
    }
}
//#endregion 🔖️ValueBridge

//#region 🔖️PlayProjection
/// 🌱️ The `Puzzle2dPlayApp` predates the typed `Puzzle2dSnapshot` above and stays on this ad-hoc
/// `serde_json::Value` fixture shape for its hundreds of Value-manipulating scene-mutation
/// helpers (see the app's own module docs) — out of scope to retrofit onto the typed struct.
/// This newtype exists only to satisfy `DocumentApp::Projection: store::DocumentDsl + store::DocumentPack`
/// post the repo-wide `store::DocumentDsl for serde_json::Value` bridge's removal (final DSL-syntax
/// convergence gate); `parse_dsl`/`print_dsl`/`encode_pack_with`/`decode_pack_with` all round-trip
/// straight through the still-standing `serde_json::Value` impls (JSON text / JSON-bridge pack
/// encoding respectively), same local-bridge shape as `semio_compose_rs`'s `KitSnapshot`. `Mutation`/
/// `MutationDiff` delegate straight through to the `Value` impls above too.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Puzzle2dPlayProjection(pub Value);

impl PartialEq for Puzzle2dPlayProjection {
    fn eq(&self, other: &Self) -> bool {
        store::pack_rt::json_values_equal(&self.0, &other.0)
    }
}

impl store::DocumentDsl for Puzzle2dPlayProjection {
    const EXTENSION: &'static str = "puzzle2d-play";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(text).map(Puzzle2dPlayProjection).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        serde_json::to_string_pretty(&self.0).unwrap_or_default()
    }
}

impl store::DocumentPack for Puzzle2dPlayProjection {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        dsl::to_dsl_value(&self.0).map_err(store::PackError::Schema)?.encode_pack_with(options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let value = dsl::DslValue::decode_pack_with(bytes, options)?;
        dsl::from_dsl_value(value).map(Puzzle2dPlayProjection).map_err(store::PackError::Schema)
    }
}

impl MutationDiff<Puzzle2dPlayProjection> for Puzzle2dDiff {
    fn apply(&self, projection: &Puzzle2dPlayProjection) -> Puzzle2dPlayProjection {
        Puzzle2dPlayProjection(MutationDiff::<Value>::apply(self, &projection.0))
    }

    fn absorb(&mut self, other: Self) {
        MutationDiff::<Puzzle2dSnapshot>::absorb(self, other);
    }
}

impl Mutation<Puzzle2dPlayProjection> for Puzzle2dMutation {
    type Diff = Puzzle2dDiff;

    fn diff(&self, projection: &Puzzle2dPlayProjection) -> Puzzle2dDiff {
        Mutation::<Value>::diff(self, &projection.0)
    }

    fn inverse(&self, projection: &Puzzle2dPlayProjection) -> Vec<Self> {
        Mutation::<Value>::inverse(self, &projection.0)
    }
}
//#endregion 🔖️PlayProjection

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::puzzle2d::PUZZLE_2D_SCHEMA;
    use serde_json::json;

    #[test]
    fn puzzle2d_delta_ops_are_granular_and_round_trip() {
        let before = json!({ "schema": PUZZLE_2D_SCHEMA, "nodes": [{ "id": "n1", "x": 0.0, "y": 0.0, "handles": [] }, { "id": "n2", "x": 10.0, "y": 0.0, "handles": [] }], "edges": [] });
        // Move n2, add n3, remove n1 — a disjoint mix of granular edits. The camera is deliberately
        // absent here: it is session-only `Puzzle2dConfig` state (see `setCamera`'s
        // `ActionKind::View`), never a document field the delta operations need to diff.
        let after = json!({ "schema": PUZZLE_2D_SCHEMA, "nodes": [{ "id": "n2", "x": 99.0, "y": 0.0, "handles": [] }, { "id": "n3", "x": 1.0, "y": 0.0, "handles": [] }], "edges": [] });
        let operations = puzzle2d_document_delta_operations(&before, &after);
        assert!(operations.iter().any(|operation| matches!(operation, Puzzle2dMutation::SetNode { .. })));
        assert!(!operations.iter().any(|operation| matches!(operation, Puzzle2dMutation::SetDocument { .. })), "granular delta must not fall back to whole-document replace here");
        // Forward replay (over the bare Value fixture, mirroring how the play app applies these) reproduces
        // `after`, and each operation's backwards restores `before`.
        let mut forward = before.clone();
        let mut inverses = Vec::new();
        for operation in &operations {
            inverses.extend(Mutation::<Value>::inverse(operation, &forward));
            forward = Mutation::<Value>::diff(operation, &forward).apply(&forward);
        }
        assert_eq!(forward, after);
        for inverse in inverses.iter().rev() {
            forward = Mutation::<Value>::diff(inverse, &forward).apply(&forward);
        }
        assert_eq!(forward, before, "backwards operations must restore the pre-edit document");
    }
}
//#endregion 🧪️Tests


pub fn apply_puzzle2d_mutation(projection: &mut Puzzle2dSnapshot, mutation: &Puzzle2dMutation) {
    *projection = vcs::apply_mutation(projection, mutation);
}

pub fn inverse_puzzle2d_mutation(projection: &Puzzle2dSnapshot, mutation: &Puzzle2dMutation) -> Vec<Puzzle2dMutation> {
    mutation.inverse(projection)
}
