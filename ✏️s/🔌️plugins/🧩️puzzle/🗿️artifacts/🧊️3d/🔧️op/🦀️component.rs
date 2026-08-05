//! 🔧️ Puzzle 3d artifact — the granular operation enum and its laws: id-keyed collection edits plus
//! the scalar meta, each with a true inverse computed from the pre-operation projection, plus the
//! whole-document replace for example loads. Also carries the `serde_json::Value` bridge (and the
//! `Puzzle3dPlayProjection` newtype over it) the play app's untyped fixture still rides on, and the
//! `puzzle3d_document_delta_operations` before/after differ every fixture-mutating action goes through.

use crate::artifacts::puzzle3d::diff::{puzzle3d_diff_absorb, puzzle3d_index_of, Puzzle3dDiff};
use crate::artifacts::puzzle3d::{Puzzle3dAttraction, Puzzle3dMeta, Puzzle3dObject, Puzzle3dProjection, Puzzle3dReference, Puzzle3dTargetVolume};
use protocol::{Operation, OperationDiff};
use serde::{Deserialize, Serialize};

//#region 🔖️Operations
/// 🧮️ Puzzle-3d operation: id-keyed collection edits plus scalar meta, each with a true inverse
/// computed from the pre-operation projection, and a whole-document replace for example loads.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum Puzzle3dOperation {
    #[dsl(key = "setObject")]
    SetObject {
        index: usize,
        #[dsl(block)]
        object: Puzzle3dObject,
    },
    #[dsl(key = "removeObject")]
    RemoveObject { id: String },
    #[dsl(key = "setAttraction")]
    SetAttraction {
        index: usize,
        #[dsl(block)]
        attraction: Puzzle3dAttraction,
    },
    #[dsl(key = "removeAttraction")]
    RemoveAttraction { id: String },
    #[dsl(key = "setTargetVolume")]
    SetTargetVolume {
        index: usize,
        #[dsl(block)]
        target_volume: Puzzle3dTargetVolume,
    },
    #[dsl(key = "removeTargetVolume")]
    RemoveTargetVolume { id: String },
    #[dsl(key = "setReference")]
    SetReference {
        index: usize,
        #[dsl(block)]
        reference: Puzzle3dReference,
    },
    #[dsl(key = "removeReference")]
    RemoveReference { id: String },
    #[dsl(key = "setMeta")]
    SetMeta {
        #[dsl(block)]
        meta: Puzzle3dMeta,
    },
    /// 🌍️ Replaces the whole document (example import / reset / engine fill).
    #[dsl(key = "setDocument")]
    SetDocument {
        #[dsl(block)]
        document: Puzzle3dProjection,
    },
}

fn puzzle3d_operation_diff(operation: &Puzzle3dOperation) -> Puzzle3dDiff {
    let mut diff = Puzzle3dDiff::default();
    match operation {
        Puzzle3dOperation::SetObject { index, object } => diff.objects.set.push((*index, object.clone())),
        Puzzle3dOperation::RemoveObject { id } => diff.objects.removed.push(id.clone()),
        Puzzle3dOperation::SetAttraction { index, attraction } => diff.attractions.set.push((*index, attraction.clone())),
        Puzzle3dOperation::RemoveAttraction { id } => diff.attractions.removed.push(id.clone()),
        Puzzle3dOperation::SetTargetVolume { index, target_volume } => diff.target_volumes.set.push((*index, target_volume.clone())),
        Puzzle3dOperation::RemoveTargetVolume { id } => diff.target_volumes.removed.push(id.clone()),
        Puzzle3dOperation::SetReference { index, reference } => diff.references.set.push((*index, reference.clone())),
        Puzzle3dOperation::RemoveReference { id } => diff.references.removed.push(id.clone()),
        Puzzle3dOperation::SetMeta { meta } => diff.meta = Some(meta.clone()),
        Puzzle3dOperation::SetDocument { document } => diff.document = Some(document.clone()),
    }
    diff
}

impl Operation<Puzzle3dProjection> for Puzzle3dOperation {
    type Diff = Puzzle3dDiff;

    fn diff(&self, _projection: &Puzzle3dProjection) -> Puzzle3dDiff {
        puzzle3d_operation_diff(self)
    }

    fn backwards(&self, projection: &Puzzle3dProjection) -> Vec<Self> {
        match self {
            Puzzle3dOperation::SetObject { object, .. } => match puzzle3d_index_of(&projection.objects, &object.id) {
                Some(index) => vec![Puzzle3dOperation::SetObject { index, object: projection.objects[index].clone() }],
                None => vec![Puzzle3dOperation::RemoveObject { id: object.id.clone() }],
            },
            Puzzle3dOperation::RemoveObject { id } => puzzle3d_index_of(&projection.objects, id).map(|index| vec![Puzzle3dOperation::SetObject { index, object: projection.objects[index].clone() }]).unwrap_or_default(),
            Puzzle3dOperation::SetAttraction { attraction, .. } => match puzzle3d_index_of(&projection.attractions, &attraction.id) {
                Some(index) => vec![Puzzle3dOperation::SetAttraction { index, attraction: projection.attractions[index].clone() }],
                None => vec![Puzzle3dOperation::RemoveAttraction { id: attraction.id.clone() }],
            },
            Puzzle3dOperation::RemoveAttraction { id } => puzzle3d_index_of(&projection.attractions, id).map(|index| vec![Puzzle3dOperation::SetAttraction { index, attraction: projection.attractions[index].clone() }]).unwrap_or_default(),
            Puzzle3dOperation::SetTargetVolume { target_volume, .. } => match puzzle3d_index_of(&projection.target_volumes, &target_volume.id) {
                Some(index) => vec![Puzzle3dOperation::SetTargetVolume { index, target_volume: projection.target_volumes[index].clone() }],
                None => vec![Puzzle3dOperation::RemoveTargetVolume { id: target_volume.id.clone() }],
            },
            Puzzle3dOperation::RemoveTargetVolume { id } => {
                puzzle3d_index_of(&projection.target_volumes, id).map(|index| vec![Puzzle3dOperation::SetTargetVolume { index, target_volume: projection.target_volumes[index].clone() }]).unwrap_or_default()
            }
            Puzzle3dOperation::SetReference { reference, .. } => match puzzle3d_index_of(&projection.references, &reference.id) {
                Some(index) => vec![Puzzle3dOperation::SetReference { index, reference: projection.references[index].clone() }],
                None => vec![Puzzle3dOperation::RemoveReference { id: reference.id.clone() }],
            },
            Puzzle3dOperation::RemoveReference { id } => puzzle3d_index_of(&projection.references, id).map(|index| vec![Puzzle3dOperation::SetReference { index, reference: projection.references[index].clone() }]).unwrap_or_default(),
            Puzzle3dOperation::SetMeta { .. } => vec![Puzzle3dOperation::SetMeta { meta: projection.meta.clone() }],
            Puzzle3dOperation::SetDocument { .. } => vec![Puzzle3dOperation::SetDocument { document: projection.clone() }],
        }
    }
}
//#endregion 🔖️Operations

//#region 🔖️ValueBridge
// 🌉️ The play app's scene-mutation helpers predate this typed projection and stay on a bare
// `serde_json::Value` scratch fixture. Bridging `Puzzle3dOperation`/`Puzzle3dDiff` onto that `Value`
// boundary too keeps `puzzle3d_document_delta_operations` and the app's `DocumentApp::Projection`
// newtype compiling unchanged — mirrors `puzzle2d`'s bridge.
fn puzzle3d_value_item_id(item: &serde_json::Value) -> Option<&str> {
    item.get("id").and_then(|value| value.as_str())
}

fn puzzle3d_upsert_value_item(document: &mut serde_json::Value, collection: &str, index: usize, item: serde_json::Value) {
    let Some(object) = document.as_object_mut() else {
        return;
    };
    let array = object.entry(collection.to_string()).or_insert_with(|| serde_json::Value::Array(Vec::new()));
    let Some(array) = array.as_array_mut() else {
        return;
    };
    if let Some(id) = puzzle3d_value_item_id(&item).map(str::to_string) {
        if let Some(slot) = array.iter_mut().find(|entry| puzzle3d_value_item_id(entry) == Some(id.as_str())) {
            *slot = item;
            return;
        }
    }
    array.insert(index.min(array.len()), item);
}

fn puzzle3d_remove_value_item(document: &mut serde_json::Value, collection: &str, id: &str) {
    if let Some(array) = document.get_mut(collection).and_then(|value| value.as_array_mut()) {
        array.retain(|entry| puzzle3d_value_item_id(entry) != Some(id));
    }
}

fn apply_puzzle3d_operation_to_value(document: &mut serde_json::Value, operation: &Puzzle3dOperation) {
    match operation {
        Puzzle3dOperation::SetObject { index, object } => puzzle3d_upsert_value_item(document, "objects", *index, serde_json::to_value(object).unwrap_or(serde_json::Value::Null)),
        Puzzle3dOperation::RemoveObject { id } => puzzle3d_remove_value_item(document, "objects", id),
        Puzzle3dOperation::SetAttraction { index, attraction } => puzzle3d_upsert_value_item(document, "attractions", *index, serde_json::to_value(attraction).unwrap_or(serde_json::Value::Null)),
        Puzzle3dOperation::RemoveAttraction { id } => puzzle3d_remove_value_item(document, "attractions", id),
        Puzzle3dOperation::SetTargetVolume { index, target_volume } => puzzle3d_upsert_value_item(document, "targetVolumes", *index, serde_json::to_value(target_volume).unwrap_or(serde_json::Value::Null)),
        Puzzle3dOperation::RemoveTargetVolume { id } => puzzle3d_remove_value_item(document, "targetVolumes", id),
        Puzzle3dOperation::SetReference { index, reference } => puzzle3d_upsert_value_item(document, "references", *index, serde_json::to_value(reference).unwrap_or(serde_json::Value::Null)),
        Puzzle3dOperation::RemoveReference { id } => puzzle3d_remove_value_item(document, "references", id),
        Puzzle3dOperation::SetMeta { meta } => {
            if let Some(object) = document.as_object_mut() {
                object.insert("meta".to_string(), serde_json::to_value(meta).unwrap_or(serde_json::Value::Null));
            }
        }
        Puzzle3dOperation::SetDocument { document: next } => *document = serde_json::to_value(next).unwrap_or_else(|_| document.clone()),
    }
}

fn puzzle3d_value_collection<'a>(document: &'a serde_json::Value, collection: &str) -> &'a [serde_json::Value] {
    document.get(collection).and_then(|value| value.as_array()).map(Vec::as_slice).unwrap_or(&[])
}

fn puzzle3d_value_item_index<T: serde::de::DeserializeOwned>(document: &serde_json::Value, collection: &str, id: &str) -> Option<(usize, T)> {
    let items = puzzle3d_value_collection(document, collection);
    let index = items.iter().position(|entry| puzzle3d_value_item_id(entry) == Some(id))?;
    serde_json::from_value(items[index].clone()).ok().map(|item| (index, item))
}

impl OperationDiff<serde_json::Value> for Puzzle3dDiff {
    fn apply(&self, projection: &serde_json::Value) -> serde_json::Value {
        if let Some(document) = &self.document {
            return serde_json::to_value(document).unwrap_or_else(|_| projection.clone());
        }
        let mut next = projection.clone();
        for id in &self.objects.removed {
            puzzle3d_remove_value_item(&mut next, "objects", id);
        }
        for (index, object) in &self.objects.set {
            puzzle3d_upsert_value_item(&mut next, "objects", *index, serde_json::to_value(object).unwrap_or(serde_json::Value::Null));
        }
        for id in &self.attractions.removed {
            puzzle3d_remove_value_item(&mut next, "attractions", id);
        }
        for (index, attraction) in &self.attractions.set {
            puzzle3d_upsert_value_item(&mut next, "attractions", *index, serde_json::to_value(attraction).unwrap_or(serde_json::Value::Null));
        }
        for id in &self.target_volumes.removed {
            puzzle3d_remove_value_item(&mut next, "targetVolumes", id);
        }
        for (index, target_volume) in &self.target_volumes.set {
            puzzle3d_upsert_value_item(&mut next, "targetVolumes", *index, serde_json::to_value(target_volume).unwrap_or(serde_json::Value::Null));
        }
        for id in &self.references.removed {
            puzzle3d_remove_value_item(&mut next, "references", id);
        }
        for (index, reference) in &self.references.set {
            puzzle3d_upsert_value_item(&mut next, "references", *index, serde_json::to_value(reference).unwrap_or(serde_json::Value::Null));
        }
        if let Some(meta) = &self.meta {
            if let Some(object) = next.as_object_mut() {
                object.insert("meta".to_string(), serde_json::to_value(meta).unwrap_or(serde_json::Value::Null));
            }
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        puzzle3d_diff_absorb(self, other);
    }
}

impl Operation<serde_json::Value> for Puzzle3dOperation {
    type Diff = Puzzle3dDiff;

    fn diff(&self, _projection: &serde_json::Value) -> Puzzle3dDiff {
        puzzle3d_operation_diff(self)
    }

    fn backwards(&self, projection: &serde_json::Value) -> Vec<Self> {
        match self {
            Puzzle3dOperation::SetObject { object, .. } => match puzzle3d_value_item_index::<Puzzle3dObject>(projection, "objects", &object.id) {
                Some((index, previous)) => vec![Puzzle3dOperation::SetObject { index, object: previous }],
                None => vec![Puzzle3dOperation::RemoveObject { id: object.id.clone() }],
            },
            Puzzle3dOperation::RemoveObject { id } => puzzle3d_value_item_index::<Puzzle3dObject>(projection, "objects", id).map(|(index, previous)| vec![Puzzle3dOperation::SetObject { index, object: previous }]).unwrap_or_default(),
            Puzzle3dOperation::SetAttraction { attraction, .. } => match puzzle3d_value_item_index::<Puzzle3dAttraction>(projection, "attractions", &attraction.id) {
                Some((index, previous)) => vec![Puzzle3dOperation::SetAttraction { index, attraction: previous }],
                None => vec![Puzzle3dOperation::RemoveAttraction { id: attraction.id.clone() }],
            },
            Puzzle3dOperation::RemoveAttraction { id } => {
                puzzle3d_value_item_index::<Puzzle3dAttraction>(projection, "attractions", id).map(|(index, previous)| vec![Puzzle3dOperation::SetAttraction { index, attraction: previous }]).unwrap_or_default()
            }
            Puzzle3dOperation::SetTargetVolume { target_volume, .. } => match puzzle3d_value_item_index::<Puzzle3dTargetVolume>(projection, "targetVolumes", &target_volume.id) {
                Some((index, previous)) => vec![Puzzle3dOperation::SetTargetVolume { index, target_volume: previous }],
                None => vec![Puzzle3dOperation::RemoveTargetVolume { id: target_volume.id.clone() }],
            },
            Puzzle3dOperation::RemoveTargetVolume { id } => {
                puzzle3d_value_item_index::<Puzzle3dTargetVolume>(projection, "targetVolumes", id).map(|(index, previous)| vec![Puzzle3dOperation::SetTargetVolume { index, target_volume: previous }]).unwrap_or_default()
            }
            Puzzle3dOperation::SetReference { reference, .. } => match puzzle3d_value_item_index::<Puzzle3dReference>(projection, "references", &reference.id) {
                Some((index, previous)) => vec![Puzzle3dOperation::SetReference { index, reference: previous }],
                None => vec![Puzzle3dOperation::RemoveReference { id: reference.id.clone() }],
            },
            Puzzle3dOperation::RemoveReference { id } => puzzle3d_value_item_index::<Puzzle3dReference>(projection, "references", id).map(|(index, previous)| vec![Puzzle3dOperation::SetReference { index, reference: previous }]).unwrap_or_default(),
            Puzzle3dOperation::SetMeta { .. } => {
                let meta: Puzzle3dMeta = projection.get("meta").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
                vec![Puzzle3dOperation::SetMeta { meta }]
            }
            Puzzle3dOperation::SetDocument { .. } => vec![Puzzle3dOperation::SetDocument { document: serde_json::from_value(projection.clone()).unwrap_or_default() }],
        }
    }
}

/// 🧮️ Collects the sparse `set`/`removed` delta for one id-keyed `Value` array collection into typed
/// entries. Returns `false` (caller falls back to `SetDocument`) whenever an entry is missing an
/// `id` or fails to deserialize into `T`.
fn puzzle3d_collect_value_collection_delta<T>(before: &[serde_json::Value], after: &[serde_json::Value], set: &mut Vec<(usize, T)>, removed: &mut Vec<String>) -> bool
where
    T: serde::de::DeserializeOwned,
{
    let before_by_id: std::collections::HashMap<&str, &serde_json::Value> = before.iter().filter_map(|entry| puzzle3d_value_item_id(entry).map(|id| (id, entry))).collect();
    let mut after_ids: std::collections::HashSet<&str> = std::collections::HashSet::with_capacity(after.len());
    for (index, entry) in after.iter().enumerate() {
        let Some(id) = puzzle3d_value_item_id(entry) else {
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
        let Some(id) = puzzle3d_value_item_id(entry) else {
            return false;
        };
        if !after_ids.contains(id) {
            removed.push(id.to_string());
        }
    }
    true
}

/// 🧮️ Computes the granular typed operation sequence turning `before` into `after` (both the bare
/// fixture JSON the play app mutates). Falls back to a single `SetDocument` whenever the granular
/// replay would not reproduce `after` exactly.
pub fn puzzle3d_document_delta_operations(before: &serde_json::Value, after: &serde_json::Value) -> Vec<Puzzle3dOperation> {
    if before == after {
        return Vec::new();
    }
    let fallback = |after: &serde_json::Value| vec![Puzzle3dOperation::SetDocument { document: serde_json::from_value(after.clone()).unwrap_or_default() }];
    let (Some(before_object), Some(after_object)) = (before.as_object(), after.as_object()) else {
        return fallback(after);
    };
    const KNOWN_KEYS: [&str; 7] = ["schema", "domain", "meta", "objects", "attractions", "targetVolumes", "references"];
    if before_object.keys().chain(after_object.keys()).any(|key| !KNOWN_KEYS.contains(&key.as_str())) {
        return fallback(after);
    }
    if before_object.get("schema") != after_object.get("schema") || before_object.get("domain") != after_object.get("domain") {
        return fallback(after);
    }
    let mut operations = Vec::new();
    macro_rules! collect_collection {
        ($key:literal, $set_op:expr, $remove_op:expr, $ty:ty) => {{
            let before_items = before_object.get($key).and_then(|value| value.as_array()).map(Vec::as_slice).unwrap_or(&[]);
            let after_items = after_object.get($key).and_then(|value| value.as_array()).map(Vec::as_slice).unwrap_or(&[]);
            if before_items != after_items {
                let mut set = Vec::new();
                let mut removed = Vec::new();
                if !puzzle3d_collect_value_collection_delta::<$ty>(before_items, after_items, &mut set, &mut removed) {
                    return fallback(after);
                }
                operations.extend(removed.into_iter().map($remove_op));
                operations.extend(set.into_iter().map($set_op));
            }
        }};
    }
    collect_collection!("objects", |(index, object)| Puzzle3dOperation::SetObject { index, object }, |id| Puzzle3dOperation::RemoveObject { id }, Puzzle3dObject);
    collect_collection!("attractions", |(index, attraction)| Puzzle3dOperation::SetAttraction { index, attraction }, |id| Puzzle3dOperation::RemoveAttraction { id }, Puzzle3dAttraction);
    collect_collection!("targetVolumes", |(index, target_volume)| Puzzle3dOperation::SetTargetVolume { index, target_volume }, |id| Puzzle3dOperation::RemoveTargetVolume { id }, Puzzle3dTargetVolume);
    collect_collection!("references", |(index, reference)| Puzzle3dOperation::SetReference { index, reference }, |id| Puzzle3dOperation::RemoveReference { id }, Puzzle3dReference);
    let before_meta = before_object.get("meta");
    let after_meta = after_object.get("meta");
    if before_meta != after_meta {
        let meta = match after_meta {
            Some(value) => match serde_json::from_value::<Puzzle3dMeta>(value.clone()) {
                Ok(meta) => meta,
                Err(_) => return fallback(after),
            },
            None => Puzzle3dMeta::default(),
        };
        operations.push(Puzzle3dOperation::SetMeta { meta });
    }
    let mut replay = before.clone();
    for operation in &operations {
        apply_puzzle3d_operation_to_value(&mut replay, operation);
    }
    if &replay == after {
        operations
    } else {
        fallback(after)
    }
}

//#region 🔖️PlayProjection
/// 🌱️ `Puzzle3dPlayApp` predates the typed `Puzzle3dProjection` above and stays on this ad-hoc
/// `serde_json::Value` fixture shape for its scene-mutation helpers. This newtype exists only to
/// satisfy `DocumentApp::Projection: store::DocumentDsl + store::DocumentPack` post the repo-wide
/// `store::DocumentDsl for serde_json::Value` bridge's removal (final DSL-syntax convergence gate);
/// `parse_dsl`/`print_dsl`/`encode_pack_with`/`decode_pack_with` all round-trip straight through the
/// still-standing `serde_json::Value` impls (JSON text / JSON-bridge pack encoding respectively),
/// same local-bridge shape as `puzzle2d`'s `Puzzle2dPlayProjection` and `semio_compose_rs`'s
/// `KitSnapshot`. `Operation`/`OperationDiff` delegate straight through to the `Value` impls above too.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Puzzle3dPlayProjection(pub serde_json::Value);

impl PartialEq for Puzzle3dPlayProjection {
    fn eq(&self, other: &Self) -> bool {
        store::pack_rt::json_values_equal(&self.0, &other.0)
    }
}

impl store::DocumentDsl for Puzzle3dPlayProjection {
    const EXTENSION: &'static str = "puzzle3d-play";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(text).map(Puzzle3dPlayProjection).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        serde_json::to_string_pretty(&self.0).unwrap_or_default()
    }
}

impl store::DocumentPack for Puzzle3dPlayProjection {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        dsl::to_dsl_value(&self.0).map_err(store::PackError::Schema)?.encode_pack_with(options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let value = dsl::DslValue::decode_pack_with(bytes, options)?;
        dsl::from_dsl_value(value).map(Puzzle3dPlayProjection).map_err(store::PackError::Schema)
    }
}

impl OperationDiff<Puzzle3dPlayProjection> for Puzzle3dDiff {
    fn apply(&self, projection: &Puzzle3dPlayProjection) -> Puzzle3dPlayProjection {
        Puzzle3dPlayProjection(OperationDiff::<serde_json::Value>::apply(self, &projection.0))
    }

    fn absorb(&mut self, other: Self) {
        puzzle3d_diff_absorb(self, other);
    }
}

impl Operation<Puzzle3dPlayProjection> for Puzzle3dOperation {
    type Diff = Puzzle3dDiff;

    fn diff(&self, projection: &Puzzle3dPlayProjection) -> Puzzle3dDiff {
        Operation::<serde_json::Value>::diff(self, &projection.0)
    }

    fn backwards(&self, projection: &Puzzle3dPlayProjection) -> Vec<Self> {
        Operation::<serde_json::Value>::backwards(self, &projection.0)
    }
}
//#endregion 🔖️PlayProjection
//#endregion 🔖️ValueBridge

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::puzzle3d::PUZZLE_3D_SCHEMA;

    #[test]
    fn puzzle3d_delta_ops_round_trip_and_stay_granular() {
        let before = serde_json::json!({ "schema": PUZZLE_3D_SCHEMA, "domain": "architecture", "objects": [{ "id": "o1", "origin": [0.0, 0.0, 0.0], "vortices": [], "hidden": false, "locked": false }, { "id": "o2", "origin": [1.0, 0.0, 0.0], "vortices": [], "hidden": false, "locked": false }], "attractions": [] });
        let after = serde_json::json!({ "schema": PUZZLE_3D_SCHEMA, "domain": "architecture", "objects": [{ "id": "o2", "origin": [9.0, 0.0, 0.0], "vortices": [], "hidden": false, "locked": false }, { "id": "o3", "origin": [2.0, 0.0, 0.0], "vortices": [], "hidden": false, "locked": false }], "attractions": [] });
        let operations = puzzle3d_document_delta_operations(&before, &after);
        assert!(!operations.iter().any(|operation| matches!(operation, Puzzle3dOperation::SetDocument { .. })));
        let mut forward = before.clone();
        let mut inverses = Vec::new();
        for operation in &operations {
            inverses.extend(Operation::<serde_json::Value>::backwards(operation, &forward));
            forward = Operation::<serde_json::Value>::diff(operation, &forward).apply(&forward);
        }
        assert_eq!(forward, after);
        for inverse in inverses.iter().rev() {
            forward = Operation::<serde_json::Value>::diff(inverse, &forward).apply(&forward);
        }
        assert_eq!(forward, before);
    }

    /// 🪢️ Regression guard for the linear (`HashMap`-based) rewrite of `puzzle3d_collect_value_collection_delta`
    /// — must still emit operations for exactly the changed/added/removed entries and skip untouched ones,
    /// exactly like the previous O(N²) `find`-based implementation.
    #[test]
    fn puzzle3d_collection_delta_only_touches_changed_entries() {
        let before = serde_json::json!({
            "schema": PUZZLE_3D_SCHEMA, "domain": "architecture", "attractions": [],
            "objects": [
                { "id": "unchanged", "origin": [0.0, 0.0, 0.0], "vortices": [], "hidden": false, "locked": false },
                { "id": "updated", "origin": [1.0, 0.0, 0.0], "vortices": [], "hidden": false, "locked": false },
                { "id": "removed", "origin": [2.0, 0.0, 0.0], "vortices": [], "hidden": false, "locked": false },
            ],
        });
        let after = serde_json::json!({
            "schema": PUZZLE_3D_SCHEMA, "domain": "architecture", "attractions": [],
            "objects": [
                { "id": "unchanged", "origin": [0.0, 0.0, 0.0], "vortices": [], "hidden": false, "locked": false },
                { "id": "updated", "origin": [1.0, 5.0, 0.0], "vortices": [], "hidden": false, "locked": false },
                { "id": "added", "origin": [3.0, 0.0, 0.0], "vortices": [], "hidden": false, "locked": false },
            ],
        });
        let operations = puzzle3d_document_delta_operations(&before, &after);
        let upserted_ids: Vec<&str> = operations
            .iter()
            .filter_map(|operation| match operation {
                Puzzle3dOperation::SetObject { object, .. } => Some(object.id.as_str()),
                _ => None,
            })
            .collect();
        let removed_ids: Vec<&str> = operations
            .iter()
            .filter_map(|operation| match operation {
                Puzzle3dOperation::RemoveObject { id } => Some(id.as_str()),
                _ => None,
            })
            .collect();
        assert_eq!(upserted_ids.len(), 2, "only changed/added objects are upserted, never unchanged ones: {operations:?}");
        assert!(upserted_ids.contains(&"updated"));
        assert!(upserted_ids.contains(&"added"));
        assert_eq!(removed_ids, vec!["removed"]);
        let mut forward = before.clone();
        for operation in &operations {
            forward = Operation::<serde_json::Value>::diff(operation, &forward).apply(&forward);
        }
        assert_eq!(forward, after);
    }
}
//#endregion 🧪️Tests
