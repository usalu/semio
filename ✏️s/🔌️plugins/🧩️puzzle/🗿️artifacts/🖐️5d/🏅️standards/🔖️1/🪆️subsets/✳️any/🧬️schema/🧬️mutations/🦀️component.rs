//! 🧬️ Puzzle 5d artifact — the granular operation enum and its laws: id-keyed part/fastener edits
//! plus the scalar meta, each with a true inverse computed from the pre-operation projection, plus
//! the whole-document replace for example loads. Also carries the `serde_json::Value` bridge (and the
//! `Puzzle5dPlaySnapshot` newtype over it) the play app's untyped document still rides on, and the
//! `puzzle5d_document_delta_operations` before/after differ every document-mutating action goes through.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::puzzle5d::diff::{puzzle5d_index_of, Puzzle5dDiff};
use crate::artifacts::puzzle5d::{Puzzle5dFastener, Puzzle5dMeta, Puzzle5dPart, Puzzle5dSnapshot};
use protocol::{Mutation, MutationDiff};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️Operations
/// 🧮️ Puzzle-5d operation: id-keyed part/fastener edits plus scalar meta, each with a true inverse
/// computed from the pre-operation projection, and a whole-document replace for example loads (also
/// the only path that changes `schema`/`domain`/`label`/`kindCatalogs`/`kindCompatibility` —
/// static/rare fields with no granular editor today). Camera pose is session-only app runtime state
/// (`ActionKind::View`), never a document operation — see the app's `Puzzle5dConfig`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum Puzzle5dMutation {
    #[dsl(key = "setPart")]
    SetPart {
        index: usize,
        #[dsl(block)]
        part: Puzzle5dPart,
    },
    #[dsl(key = "removePart")]
    RemovePart { id: String },
    #[dsl(key = "setFastener")]
    SetFastener {
        index: usize,
        #[dsl(block)]
        fastener: Puzzle5dFastener,
    },
    #[dsl(key = "removeFastener")]
    RemoveFastener { id: String },
    #[dsl(key = "setMeta")]
    SetMeta {
        #[dsl(block)]
        meta: Puzzle5dMeta,
    },
    /// 🌍️ Replaces the whole document (example import / reset / engine fill).
    #[dsl(key = "setSnapshot")]
    SetSnapshot {
        #[dsl(block)]
        snapshot: Puzzle5dSnapshot,
    },
}





fn puzzle5d_mutation_diff(operation: &Puzzle5dMutation, base: &Puzzle5dSnapshot) -> Puzzle5dDiff {
    match operation {
        Puzzle5dMutation::SetPart { index, part } => crate::artifacts::puzzle5d::diff::diff_set_part(*index, part.clone(), base),
        Puzzle5dMutation::RemovePart { id } => crate::artifacts::puzzle5d::diff::diff_remove_part(id.clone()),
        Puzzle5dMutation::SetFastener { index, fastener } => crate::artifacts::puzzle5d::diff::diff_set_fastener(*index, fastener.clone(), base),
        Puzzle5dMutation::RemoveFastener { id } => crate::artifacts::puzzle5d::diff::diff_remove_fastener(id.clone()),
        Puzzle5dMutation::SetMeta { meta } => crate::artifacts::puzzle5d::diff::diff_set_meta(meta.clone()),
        Puzzle5dMutation::SetSnapshot { snapshot } => crate::artifacts::puzzle5d::diff::diff_set_snapshot(snapshot.clone()),
    }
}

impl Mutation<Puzzle5dSnapshot> for Puzzle5dMutation {
    type Diff = Puzzle5dDiff;

    fn diff(&self, projection: &Puzzle5dSnapshot) -> Puzzle5dDiff {
        puzzle5d_mutation_diff(self, projection)
    }

    fn inverse(&self, projection: &Puzzle5dSnapshot) -> Vec<Self> {
        match self {
            Puzzle5dMutation::SetPart { part, .. } => match puzzle5d_index_of(&projection.parts, &part.id) {
                Some(index) => vec![Puzzle5dMutation::SetPart { index, part: projection.parts[index].clone() }],
                None => vec![Puzzle5dMutation::RemovePart { id: part.id.clone() }],
            },
            Puzzle5dMutation::RemovePart { id } => puzzle5d_index_of(&projection.parts, id).map_or_else(Vec::new, |index| vec![Puzzle5dMutation::SetPart { index, part: projection.parts[index].clone() }]),
            Puzzle5dMutation::SetFastener { fastener, .. } => match puzzle5d_index_of(&projection.fasteners, &fastener.id) {
                Some(index) => vec![Puzzle5dMutation::SetFastener { index, fastener: projection.fasteners[index].clone() }],
                None => vec![Puzzle5dMutation::RemoveFastener { id: fastener.id.clone() }],
            },
            Puzzle5dMutation::RemoveFastener { id } => puzzle5d_index_of(&projection.fasteners, id).map_or_else(Vec::new, |index| vec![Puzzle5dMutation::SetFastener { index, fastener: projection.fasteners[index].clone() }]),
            Puzzle5dMutation::SetMeta { .. } => vec![Puzzle5dMutation::SetMeta { meta: projection.meta.clone() }],
            Puzzle5dMutation::SetSnapshot { .. } => vec![Puzzle5dMutation::SetSnapshot { snapshot: projection.clone() }],
        }
    }
}
//#endregion 🔖️Operations

//#region 🔖️ValueBridge
// 🌉️ The play app's scene-mutation helpers predate this typed projection and stay on a bare
// `serde_json::Value` scratch fixture. Bridging `Puzzle5dMutation`/`Puzzle5dDiff` onto that `Value`
// boundary too keeps `puzzle5d_document_delta_operations` and the app's `ArtifactApp::Snapshot`
// compiling unchanged — mirrors `puzzle2d`/`puzzle3d`'s bridge.
fn puzzle5d_value_item_id(item: &Value) -> Option<&str> {
    item.get("id").and_then(|value| value.as_str())
}

fn puzzle5d_upsert_value_item(document: &mut Value, collection: &str, index: usize, item: Value) {
    let Some(object) = document.as_object_mut() else {
        return;
    };
    let array = object.entry(collection.to_string()).or_insert_with(|| Value::Array(Vec::new()));
    let Some(array) = array.as_array_mut() else {
        return;
    };
    if let Some(id) = puzzle5d_value_item_id(&item).map(str::to_string) {
        if let Some(slot) = array.iter_mut().find(|entry| puzzle5d_value_item_id(entry) == Some(id.as_str())) {
            *slot = item;
            return;
        }
    }
    array.insert(index.min(array.len()), item);
}

fn puzzle5d_remove_value_item(document: &mut Value, collection: &str, id: &str) {
    if let Some(array) = document.get_mut(collection).and_then(|value| value.as_array_mut()) {
        array.retain(|entry| puzzle5d_value_item_id(entry) != Some(id));
    }
}

fn apply_puzzle5d_operation_to_value(document: &mut Value, operation: &Puzzle5dMutation) {
    match operation {
        Puzzle5dMutation::SetPart { index, part } => puzzle5d_upsert_value_item(document, "parts", *index, serde_json::to_value(part).unwrap_or(Value::Null)),
        Puzzle5dMutation::RemovePart { id } => puzzle5d_remove_value_item(document, "parts", id),
        Puzzle5dMutation::SetFastener { index, fastener } => puzzle5d_upsert_value_item(document, "fasteners", *index, serde_json::to_value(fastener).unwrap_or(Value::Null)),
        Puzzle5dMutation::RemoveFastener { id } => puzzle5d_remove_value_item(document, "fasteners", id),
        Puzzle5dMutation::SetMeta { meta } => {
            if let Some(object) = document.as_object_mut() {
                object.insert("meta".to_string(), serde_json::to_value(meta).unwrap_or(Value::Null));
            }
        }
        Puzzle5dMutation::SetSnapshot { snapshot: next } => *document = serde_json::to_value(next).unwrap_or_else(|_| document.clone()),
    }
}

fn puzzle5d_value_collection<'a>(document: &'a Value, collection: &str) -> &'a [Value] {
    document.get(collection).and_then(|value| value.as_array()).map_or(&[], Vec::as_slice)
}

fn puzzle5d_value_item_index<T: serde::de::DeserializeOwned>(document: &Value, collection: &str, id: &str) -> Option<(usize, T)> {
    let items = puzzle5d_value_collection(document, collection);
    let index = items.iter().position(|entry| puzzle5d_value_item_id(entry) == Some(id))?;
    serde_json::from_value(items[index].clone()).ok().map(|item| (index, item))
}

fn puzzle5d_reorder_value_collection(document: &mut Value, collection: &str, order: &[String]) {
    let Some(array) = document.get_mut(collection).and_then(|value| value.as_array_mut()) else {
        return;
    };
    let mut by_id: std::collections::BTreeMap<String, Value> = std::collections::BTreeMap::new();
    for item in array.drain(..) {
        if let Some(id) = puzzle5d_value_item_id(&item).map(str::to_string) {
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

impl MutationDiff<Value> for Puzzle5dDiff {
    fn apply(&self, projection: &Value) -> Value {
        if let Some(artifact) = &self.artifact {
            return serde_json::to_value(artifact.to_snapshot()).unwrap_or_else(|_| projection.clone());
        }
        let mut next = projection.clone();
        if let Some(delta) = &self.parts {
            for id in &delta.removed {
                puzzle5d_remove_value_item(&mut next, "parts", id);
            }
            for part in &delta.added {
                puzzle5d_upsert_value_item(&mut next, "parts", usize::MAX, serde_json::to_value(part).unwrap_or(Value::Null));
            }
            for entry in &delta.patched {
                if let Some(part) = &entry.patch.replacement {
                    puzzle5d_upsert_value_item(&mut next, "parts", usize::MAX, serde_json::to_value(part).unwrap_or(Value::Null));
                }
            }
            if let Some(order) = &delta.reordered {
                puzzle5d_reorder_value_collection(&mut next, "parts", order);
            }
        }
        if let Some(delta) = &self.fasteners {
            for id in &delta.removed {
                puzzle5d_remove_value_item(&mut next, "fasteners", id);
            }
            for fastener in &delta.added {
                puzzle5d_upsert_value_item(&mut next, "fasteners", usize::MAX, serde_json::to_value(fastener).unwrap_or(Value::Null));
            }
            for entry in &delta.patched {
                if let Some(fastener) = &entry.patch.replacement {
                    puzzle5d_upsert_value_item(&mut next, "fasteners", usize::MAX, serde_json::to_value(fastener).unwrap_or(Value::Null));
                }
            }
            if let Some(order) = &delta.reordered {
                puzzle5d_reorder_value_collection(&mut next, "fasteners", order);
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
        MutationDiff::<Puzzle5dSnapshot>::absorb(self, other);
    }
}

impl Mutation<Value> for Puzzle5dMutation {
    type Diff = Puzzle5dDiff;

    fn diff(&self, projection: &Value) -> Puzzle5dDiff {
        let base: Puzzle5dSnapshot = serde_json::from_value(projection.clone()).unwrap_or_default();
        puzzle5d_mutation_diff(self, &base)
    }

    fn inverse(&self, projection: &Value) -> Vec<Self> {
        match self {
            Puzzle5dMutation::SetPart { part, .. } => match puzzle5d_value_item_index::<Puzzle5dPart>(projection, "parts", &part.id) {
                Some((index, previous)) => vec![Puzzle5dMutation::SetPart { index, part: previous }],
                None => vec![Puzzle5dMutation::RemovePart { id: part.id.clone() }],
            },
            Puzzle5dMutation::RemovePart { id } => puzzle5d_value_item_index::<Puzzle5dPart>(projection, "parts", id).map_or_else(Vec::new, |(index, previous)| vec![Puzzle5dMutation::SetPart { index, part: previous }]),
            Puzzle5dMutation::SetFastener { fastener, .. } => match puzzle5d_value_item_index::<Puzzle5dFastener>(projection, "fasteners", &fastener.id) {
                Some((index, previous)) => vec![Puzzle5dMutation::SetFastener { index, fastener: previous }],
                None => vec![Puzzle5dMutation::RemoveFastener { id: fastener.id.clone() }],
            },
            Puzzle5dMutation::RemoveFastener { id } => {
                puzzle5d_value_item_index::<Puzzle5dFastener>(projection, "fasteners", id).map_or_else(Vec::new, |(index, previous)| vec![Puzzle5dMutation::SetFastener { index, fastener: previous }])
            }
            Puzzle5dMutation::SetMeta { .. } => {
                let meta: Puzzle5dMeta = projection.get("meta").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
                vec![Puzzle5dMutation::SetMeta { meta }]
            }
            Puzzle5dMutation::SetSnapshot { .. } => vec![Puzzle5dMutation::SetSnapshot { snapshot: serde_json::from_value(projection.clone()).unwrap_or_default() }],
        }
    }
}

/// 🧮️ Collects the sparse `set`/`removed` delta for one id-keyed `Value` array collection into typed
/// entries. Returns `false` (caller falls back to `SetSnapshot`) whenever an entry is missing an
/// `id` or fails to deserialize into `T`.
fn puzzle5d_collect_value_collection_delta<T>(before: &[Value], after: &[Value], set: &mut Vec<(usize, T)>, removed: &mut Vec<String>) -> bool
where
    T: serde::de::DeserializeOwned,
{
    let before_by_id: std::collections::HashMap<&str, &Value> = before.iter().filter_map(|entry| puzzle5d_value_item_id(entry).map(|id| (id, entry))).collect();
    let mut after_ids: std::collections::HashSet<&str> = std::collections::HashSet::with_capacity(after.len());
    for (index, entry) in after.iter().enumerate() {
        let Some(id) = puzzle5d_value_item_id(entry) else {
            return false;
        };
        after_ids.insert(id);
        if before_by_id.get(id).copied() != Some(entry) {
            let Ok(item) = serde_json::from_value::<T>(entry.clone()) else {
                return false;
            };
            set.push((index, item));
        }
    }
    for entry in before {
        let Some(id) = puzzle5d_value_item_id(entry) else {
            return false;
        };
        if !after_ids.contains(id) {
            removed.push(id.to_string());
        }
    }
    true
}

/// 🧮️ Computes the granular typed operation sequence turning `before` into `after` (both the bare
/// document JSON the play app mutates). Falls back to a single `SetSnapshot` whenever the granular
/// replay would not reproduce `after` exactly, or whenever `schema`/`domain`/`label`/`kindCatalogs`/
/// `kindCompatibility` changed (no granular editor for those today — see `Puzzle5dMutation`'s doc).
pub fn puzzle5d_document_delta_operations(before: &Value, after: &Value) -> Vec<Puzzle5dMutation> {
    if before == after {
        return Vec::new();
    }
    let fallback = |after: &Value| vec![Puzzle5dMutation::SetSnapshot { snapshot: serde_json::from_value(after.clone()).unwrap_or_default() }];
    let (Some(before_object), Some(after_object)) = (before.as_object(), after.as_object()) else {
        return fallback(after);
    };
    const KNOWN_KEYS: [&str; 8] = ["schema", "domain", "label", "meta", "kindCatalogs", "kindCompatibility", "parts", "fasteners"];
    if before_object.keys().chain(after_object.keys()).any(|key| !KNOWN_KEYS.contains(&key.as_str())) {
        return fallback(after);
    }
    for exact_key in ["schema", "domain", "label", "kindCatalogs", "kindCompatibility"] {
        if before_object.get(exact_key) != after_object.get(exact_key) {
            return fallback(after);
        }
    }
    let mut operations = Vec::new();
    macro_rules! collect_collection {
        ($key:literal, $set_op:expr, $remove_op:expr, $ty:ty) => {{
            let before_items = before_object.get($key).and_then(|value| value.as_array()).map_or(&[][..], Vec::as_slice);
            let after_items = after_object.get($key).and_then(|value| value.as_array()).map_or(&[][..], Vec::as_slice);
            if before_items != after_items {
                let mut set = Vec::new();
                let mut removed = Vec::new();
                if !puzzle5d_collect_value_collection_delta::<$ty>(before_items, after_items, &mut set, &mut removed) {
                    return fallback(after);
                }
                operations.extend(removed.into_iter().map($remove_op));
                operations.extend(set.into_iter().map($set_op));
            }
        }};
    }
    collect_collection!("parts", |(index, part)| Puzzle5dMutation::SetPart { index, part }, |id| Puzzle5dMutation::RemovePart { id }, Puzzle5dPart);
    collect_collection!("fasteners", |(index, fastener)| Puzzle5dMutation::SetFastener { index, fastener }, |id| Puzzle5dMutation::RemoveFastener { id }, Puzzle5dFastener);
    let before_meta = before_object.get("meta");
    let after_meta = after_object.get("meta");
    if before_meta != after_meta {
        let meta = match after_meta {
            Some(value) => match serde_json::from_value::<Puzzle5dMeta>(value.clone()) {
                Ok(meta) => meta,
                Err(_) => return fallback(after),
            },
            None => Puzzle5dMeta::default(),
        };
        operations.push(Puzzle5dMutation::SetMeta { meta });
    }
    let mut replay = before.clone();
    for operation in &operations {
        apply_puzzle5d_operation_to_value(&mut replay, operation);
    }
    let normalize = |value: &Value| {
        serde_json::to_value(serde_json::from_value::<Puzzle5dSnapshot>(value.clone()).unwrap_or_default()).unwrap_or_else(|_| value.clone())
    };
    if normalize(&replay) == normalize(after) {
        operations
    } else {
        fallback(after)
    }
}

//#region 🔖️PlaySnapshot
/// 🌱️ The play app's `Puzzle5dPlayApp` predates the typed `Puzzle5dSnapshot` above and stays on
/// this ad-hoc `serde_json::Value` fixture shape for its scene-mutation helpers. This newtype exists
/// only to satisfy `ArtifactApp::Snapshot: store::ArtifactDsl + store::ArtifactPack`;
/// `parse_dsl`/`print_dsl`/`encode_pack_with`/`decode_pack_with` all round-trip straight through the
/// still-standing `serde_json::Value` impls (JSON text / JSON-bridge pack encoding respectively),
/// same local-bridge shape as `puzzle2d`'s `Puzzle2dPlaySnapshot` and `puzzle3d`'s
/// `Puzzle3dPlaySnapshot`. `Mutation`/`MutationDiff` delegate straight through to the `Value`
/// impls above too.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Puzzle5dPlaySnapshot(pub Value);

impl PartialEq for Puzzle5dPlaySnapshot {
    fn eq(&self, other: &Self) -> bool {
        store::pack_rt::json_values_equal(&self.0, &other.0)
    }
}

impl store::ArtifactDsl for Puzzle5dPlaySnapshot {
    const EXTENSION: &'static str = "puzzle5d-play";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(text).map(Puzzle5dPlaySnapshot).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        serde_json::to_string_pretty(&self.0).unwrap_or_default()
    }
}

impl store::ArtifactPack for Puzzle5dPlaySnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        dsl::to_dsl_value(&self.0).map_err(store::PackError::Schema)?.encode_pack_with(options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let value = dsl::DslValue::decode_pack_with(bytes, options)?;
        dsl::from_dsl_value(value).map(Puzzle5dPlaySnapshot).map_err(store::PackError::Schema)
    }
}

impl MutationDiff<Puzzle5dPlaySnapshot> for Puzzle5dDiff {
    fn apply(&self, projection: &Puzzle5dPlaySnapshot) -> Puzzle5dPlaySnapshot {
        Puzzle5dPlaySnapshot(MutationDiff::<Value>::apply(self, &projection.0))
    }

    fn absorb(&mut self, other: Self) {
        MutationDiff::<Puzzle5dSnapshot>::absorb(self, other);
    }
}

impl Mutation<Puzzle5dPlaySnapshot> for Puzzle5dMutation {
    type Diff = Puzzle5dDiff;

    fn diff(&self, projection: &Puzzle5dPlaySnapshot) -> Puzzle5dDiff {
        Mutation::<Value>::diff(self, &projection.0)
    }

    fn inverse(&self, projection: &Puzzle5dPlaySnapshot) -> Vec<Self> {
        Mutation::<Value>::inverse(self, &projection.0)
    }
}
//#endregion 🔖️PlaySnapshot
//#endregion 🔖️ValueBridge

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle5d_delta_ops_round_trip_and_stay_granular() {
        let before = serde_json::json!({
            "schema": crate::artifacts::puzzle5d::PUZZLE_5D_SCHEMA, "domain": "architecture",
            "meta": { "description": "" },
            "parts": [
                { "id": "p1", "2d": { "x": 0.0, "y": 0.0 }, "3d": { "origin": [0.0,0.0,0.0] }, "grips": [] },
                { "id": "p2", "2d": { "x": 1.0, "y": 0.0 }, "3d": { "origin": [1.0,0.0,0.0] }, "grips": [] },
            ],
            "fasteners": [],
        });
        let after = serde_json::json!({
            "schema": crate::artifacts::puzzle5d::PUZZLE_5D_SCHEMA, "domain": "architecture",
            "meta": { "description": "" },
            "parts": [
                { "id": "p2", "2d": { "x": 9.0, "y": 0.0 }, "3d": { "origin": [9.0,0.0,0.0] }, "grips": [] },
                { "id": "p3", "2d": { "x": 2.0, "y": 0.0 }, "3d": { "origin": [2.0,0.0,0.0] }, "grips": [] },
            ],
            "fasteners": [],
        });
        let operations = puzzle5d_document_delta_operations(&before, &after);
        assert!(operations.iter().any(|operation| matches!(operation, Puzzle5dMutation::SetPart { .. })));
        assert!(!operations.iter().any(|operation| matches!(operation, Puzzle5dMutation::SetSnapshot { .. })), "granular delta must not fall back to whole-document replace here");
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


pub fn apply_puzzle5d_mutation(projection: &mut Puzzle5dSnapshot, mutation: &Puzzle5dMutation) {
    *projection = vcs::apply_mutation(projection, mutation);
}

pub fn inverse_puzzle5d_mutation(projection: &Puzzle5dPlaySnapshot, mutation: &Puzzle5dMutation) -> Vec<Puzzle5dMutation> {
    mutation.inverse(projection)
}
