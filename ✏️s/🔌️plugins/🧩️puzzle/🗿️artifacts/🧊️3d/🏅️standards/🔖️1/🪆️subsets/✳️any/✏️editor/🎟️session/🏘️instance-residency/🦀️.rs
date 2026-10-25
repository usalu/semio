//! 🏘️ Instance residency: the per-object instance record text, its fingerprints and delta, the scene geometry fingerprint and the world mesh lane.

use super::*;
use semio_s_artifact_puzzle_3d_editor_snapshot::{object_scale_json, Puzzle3dKindMeshIndex};
use std::hash::Hash;

/// 🙈️ Hidden objects stay in the emitted array — `worldPick`'s `id` arg is the array index into it — but render at zero scale so they're effectively invisible without shifting any other object's index.
/// Selection/hover paint is driven by `selectionJson` on the host — never baked here so instance geometry stays stable across picks.
pub fn world_instances_geometry_json(scene_snapshot: &Puzzle3dSceneSnapshot) -> String {
    let mut residency = Puzzle3dInstanceResidency::default();
    residency.refresh(scene_snapshot, &std::collections::BTreeSet::new(), &[]);
    residency.instances_json().to_string()
}

/// 🧱️ ONE instance record, exactly the shape [`world_instances_geometry_json`] publishes; `provisional` marks an
/// object a running tool placed and has not finalized (the `provisional` mesh style, never pickable), `highlighted` an
/// object the open time-travel draft references (both hosts paint the `highlighted` instance row).
fn instance_record_json(object: &Puzzle3dObject, mesh_id: &str, provisional: bool, highlighted: bool) -> String {
    let scale = if object.hidden { serde_json::json!([0.0, 0.0, 0.0]) } else { serde_json::json!(object_scale_json(object)) };
    let mut instance = serde_json::json!({
        "id": object.id,
        "meshId": mesh_id,
        "position": [
            object.origin.first().copied().unwrap_or(0.0),
            object.origin.get(1).copied().unwrap_or(0.0),
            object.origin.get(2).copied().unwrap_or(0.0),
        ],
        "rotation": object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
        "scale": scale,
        "label": object.label.clone().or_else(|| object.object_kind.clone()).unwrap_or_else(|| object.id.clone()),
        "disabled": object.locked,
    });
    if let Some(kind) = &object.object_kind {
        instance["objectKind"] = serde_json::json!(kind);
    }
    if provisional {
        instance["provisional"] = serde_json::json!(true);
    }
    if highlighted {
        instance["highlighted"] = serde_json::json!(true);
    }
    serde_json::to_string(&instance).unwrap_or_else(|_| "{}".into())
}

/// 🔑️ The per-object change key for ONE instance record — every field [`instance_record_json`] reads
/// and nothing else, hashed structurally. No JSON is materialized: this is the difference between a
/// per-object key that costs a handful of `Hash::hash` calls and the whole-document
/// `format!`-then-hash [`scene_geometry_fingerprint`] used to be (measured 20 716 µs on the
/// 180-object Nakagin document, ticket 26/09/02 wave B44 §1).
fn instance_record_fingerprint(object: &Puzzle3dObject, mesh_id: &str, provisional: bool, highlighted: bool) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    object.id.hash(&mut hasher);
    provisional.hash(&mut hasher);
    highlighted.hash(&mut hasher);
    mesh_id.hash(&mut hasher);
    for axis in object.origin {
        axis.to_bits().hash(&mut hasher);
    }
    object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]).map(f64::to_bits).hash(&mut hasher);
    match &object.scale {
        Some(scale) => hash_dsl_value(scale, &mut hasher),
        None => 0_u8.hash(&mut hasher),
    }
    object.label.hash(&mut hasher);
    object.object_kind.hash(&mut hasher);
    object.hidden.hash(&mut hasher);
    object.locked.hash(&mut hasher);
    hasher.finish()
}

/// 🧮️ Structural hash of one DSL value — the allocation-free replacement for hashing its JSON text.
/// Floats go in by `to_bits`, so the key is exact rather than format-dependent, and an object's keys
/// are hashed in their authored order (the order `ToValue` emits, which is what the previous JSON
/// hash was sensitive to as well).
pub fn hash_dsl_value<H: Hasher>(value: &semio_framework_value::DslValue, hasher: &mut H) {
    match value {
        semio_framework_value::DslValue::Null => 0_u8.hash(hasher),
        semio_framework_value::DslValue::Bool(flag) => {
            1_u8.hash(hasher);
            flag.hash(hasher);
        }
        semio_framework_value::DslValue::Number(number) => {
            2_u8.hash(hasher);
            match number {
                semio_framework_value::Number::UInt(value) => value.hash(hasher),
                semio_framework_value::Number::Int(value) => value.hash(hasher),
                semio_framework_value::Number::Float(value) => value.to_bits().hash(hasher),
            }
        }
        semio_framework_value::DslValue::String(text) => {
            3_u8.hash(hasher);
            text.hash(hasher);
        }
        semio_framework_value::DslValue::Array(items) => {
            4_u8.hash(hasher);
            items.len().hash(hasher);
            for item in items {
                hash_dsl_value(item, hasher);
            }
        }
        semio_framework_value::DslValue::Object(entries) => {
            5_u8.hash(hasher);
            entries.len().hash(hasher);
            for (key, item) in entries {
                key.hash(hasher);
                hash_dsl_value(item, hasher);
            }
        }
        semio_framework_value::DslValue::Bytes(bytes) => {
            6_u8.hash(hasher);
            bytes.hash(hasher);
        }
    }
}

/// 🚚️ Per-object residency for the world `instances` lane: the one place the published instance text
/// is built, kept and invalidated PER OBJECT instead of per document.
///
/// ⏱️ Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B44. Before it, one viewport render of the 180-object
/// Nakagin document paid 20 716 µs hashing the whole scene_snapshot's JSON plus 2 449 µs re-serializing all
/// 180 records — for a gumball drag that moved ONE of them, and once per window and once more for the
/// outliner. This walks the objects, re-serializes only the records whose own
/// [`instance_record_fingerprint`] moved, and reports exactly which ids those were, so an intake that
/// wants to update its instanced meshes in place has the id list to do it with ([`Self::delta_json`]).
///
/// 🧾️ `instances_json` stays the AUTHORITATIVE full set on every publication: a pose-only lane would
/// be O(changed) bytes but would leave every consumer that does not merge deltas (the wgpu world
/// target builds `World3dScene` directly) rendering a stale pose. The delta rides ALONGSIDE it.
#[derive(Default)]
pub struct Puzzle3dInstanceResidency {
    pub(super) entries: std::collections::HashMap<String, (u64, String)>,
    order: Vec<String>,
    assembled: String,
    revision: u64,
    changed: Vec<String>,
    removed: Vec<String>,
    delta: String,
    rebuilt: u32,
}

impl Puzzle3dInstanceResidency {
    /// 🔄️ Reconciles the residency against one scene_snapshot, the running tool's provisional entities
    /// (`ArtifactView::tool_run().provisional_entities`, keyed by `fill_run_entity(object.id)`) and the object ids the open
    /// time-travel draft references (`InteractionView::draft_references`), so opening or closing a history edit republishes
    /// exactly the referenced records through the delta. Answers whether the published text changed — `false` means every
    /// record and the order are bit-identical and no consumer owes any work.
    pub fn refresh(&mut self, scene_snapshot: &Puzzle3dSceneSnapshot, provisional: &std::collections::BTreeSet<u64>, referenced: &[String]) -> bool {
        let kind_meshes = Puzzle3dKindMeshIndex::of(&scene_snapshot.meta);
        let mut order = Vec::with_capacity(scene_snapshot.objects.len());
        let mut changed = Vec::new();
        let mut rebuilt = 0_u32;
        for object in &scene_snapshot.objects {
            let mesh_id = kind_meshes.resolve(object).map_or_else(|| PUZZLE3D_FALLBACK_MESH_KIND.into(), world3d_mesh_id_from_url);
            let placed = !provisional.is_empty() && provisional.contains(&semio_s_artifact_puzzle_3d_precompute_fill::fill_run_entity(&object.id));
            let highlighted = referenced.iter().any(|id| id == &object.id);
            let fingerprint = instance_record_fingerprint(object, &mesh_id, placed, highlighted);
            if self.entries.get(&object.id).is_none_or(|(cached, _)| *cached != fingerprint) {
                self.entries.insert(object.id.clone(), (fingerprint, instance_record_json(object, &mesh_id, placed, highlighted)));
                changed.push(object.id.clone());
                rebuilt += 1;
            }
            order.push(object.id.clone());
        }
        let removed: Vec<String> = self.order.iter().filter(|id| order.iter().all(|live| live != *id)).cloned().collect();
        for id in &removed {
            self.entries.remove(id);
        }
        let reordered = self.order != order;
        self.order = order;
        if !reordered && changed.is_empty() && removed.is_empty() && !self.assembled.is_empty() {
            self.rebuilt = 0;
            return false;
        }
        self.changed = changed;
        self.removed = removed;
        self.rebuilt = rebuilt;
        self.assembled = self.assemble();
        self.revision = self.revision.saturating_add(1);
        self.delta = if self.delta_is_worth_publishing() { self.assemble_delta() } else { String::new() };
        true
    }

    /// ⚖️ Retained bytes, for the session registry's process-wide census.
    pub fn bytes(&self) -> usize {
        self.assembled.len().saturating_add(self.delta.len()).saturating_add(self.entries.values().map(|(_, record)| record.len() + 48).sum::<usize>())
    }

    fn assemble(&self) -> String {
        let mut text = String::with_capacity(self.order.iter().filter_map(|id| self.entries.get(id)).map(|(_, record)| record.len() + 1).sum::<usize>() + 2);
        text.push('[');
        for id in &self.order {
            if let Some((_, record)) = self.entries.get(id) {
                if text.len() > 1 {
                    text.push(',');
                }
                text.push_str(record);
            }
        }
        text.push(']');
        text
    }

    /// 🧾️ The authoritative full instance set, exactly what [`world_instances_geometry_json`] returns.
    pub fn instances_json(&self) -> &str {
        if self.assembled.is_empty() {
            "[]"
        } else {
            &self.assembled
        }
    }

    /// 🚚️ The last refresh's delta, keyed by instance id: the records that changed (whole records, so
    /// an applying consumer never has to diff fields) and the ids that went away. `base` is the
    /// revision the delta applies TO, `revision` the one it produces — a consumer whose retained set
    /// is not at `base` must fall back to `instancesJson`.
    pub fn delta_json(&self) -> Option<&str> {
        if self.delta.is_empty() {
            None
        } else {
            Some(&self.delta)
        }
    }

    /// ⚖️ Whether a delta is worth publishing at all. A cold publication (nothing retained anywhere
    /// yet) and one that names at least half the set buy a consumer nothing — it has to read
    /// `instancesJson` either way — while costing a second copy of the same bytes on the wire and one
    /// more lane carrier to page. Measured: publishing the cold 180-record delta alongside the
    /// 55 154-byte full set put `openVortexSuggestions`'s worst turn at 4.56 ms against this
    /// artifact's own 2 ms unoptimized ceiling.
    fn delta_is_worth_publishing(&self) -> bool {
        self.revision > 1 && self.changed.len() * 2 < self.order.len() && self.removed.len() * 2 < self.order.len().max(1)
    }

    /// 🚚️ Assembles the delta text ONCE per republication, by concatenating the record strings the
    /// residency already holds. Never by `serde_json` round-tripping them: a cold Nakagin publication
    /// names 180 changed records, and re-parsing them on every `delta_json` call (which every render
    /// makes, republishing or not) cost more than the whole-set re-serialization this wave removed —
    /// it broke `open_vortex_suggestions_every_step_stays_below_the_interactive_ceiling_for_nakagin`
    /// before being caught.
    fn assemble_delta(&self) -> String {
        let mut text = String::with_capacity(self.changed.iter().filter_map(|id| self.entries.get(id)).map(|(_, record)| record.len() + 1).sum::<usize>() + 128);
        text.push_str(r#"{"base":"#);
        text.push_str(&self.revision.saturating_sub(1).to_string());
        text.push_str(r#","revision":"#);
        text.push_str(&self.revision.to_string());
        text.push_str(r#","count":"#);
        text.push_str(&self.order.len().to_string());
        text.push_str(r#","changed":["#);
        let mut first = true;
        for id in &self.changed {
            if let Some((_, record)) = self.entries.get(id) {
                if !first {
                    text.push(',');
                }
                text.push_str(record);
                first = false;
            }
        }
        text.push_str(r#"],"removed":"#);
        text.push_str(&serde_json::to_string(&self.removed).unwrap_or_else(|_| "[]".into()));
        text.push('}');
        text
    }

    /// 🔬️ The ids whose record was re-serialized by the last refresh — the observable the O(changed)
    /// laws read instead of inferring incrementality from timings.
    pub fn changed_ids(&self) -> &[String] {
        &self.changed
    }

    pub fn removed_ids(&self) -> &[String] {
        &self.removed
    }

    /// 🔬️ How many records the last refresh genuinely re-serialized.
    pub fn rebuilt_records(&self) -> u32 {
        self.rebuilt
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }
}

use semio_s_artifact_puzzle_3d_editor_snapshot::VORTEX_MARKER_MESH_KIND;
use semio_framework_plugin::world3d_meshes_json_from_kinds_and_urls;

/// 🗄️ Cheap change key for everything the instance/mesh payloads (and the document tree) derive from.
///
/// ⏱️ Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B44: hashed STRUCTURALLY, never through JSON text.
/// The `format!` of four `to_json_string` calls this used to be materialized ~60 KiB per call on the
/// 180-object Nakagin document and measured **20 716 µs** — 54 % of a cache-HIT viewport render, paid
/// again per window and again for the outliner memo's key. The structural walk is the same O(n) key
/// over the same four members with none of the allocation.
pub fn scene_geometry_fingerprint(scene_snapshot: &Puzzle3dSceneSnapshot) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    scene_snapshot.objects.len().hash(&mut hasher);
    for object in &scene_snapshot.objects {
        hash_object(object, &mut hasher);
    }
    scene_snapshot.references.len().hash(&mut hasher);
    for reference in &scene_snapshot.references {
        reference.id.hash(&mut hasher);
        reference.source.url.hash(&mut hasher);
        reference.source.media_kind.hash(&mut hasher);
        hash_axes(&reference.origin, &mut hasher);
        reference.width_world.to_bits().hash(&mut hasher);
        reference.locked.hash(&mut hasher);
        reference.hidden.hash(&mut hasher);
    }
    scene_snapshot.target_volumes.len().hash(&mut hasher);
    for volume in &scene_snapshot.target_volumes {
        volume.id.hash(&mut hasher);
        hash_axes(&volume.origin, &mut hasher);
        volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]).map(f64::to_bits).hash(&mut hasher);
        hash_optional_dsl_value(volume.scale.as_ref(), &mut hasher);
        volume.hidden.hash(&mut hasher);
        volume.locked.hash(&mut hasher);
    }
    hash_optional_dsl_value(scene_snapshot.meta.kind_catalogs.as_ref(), &mut hasher);
    hash_optional_dsl_value(scene_snapshot.meta.kind_compatibility.as_ref(), &mut hasher);
    hasher.finish()
}

fn hash_axes<H: Hasher>(axes: &[f64], hasher: &mut H) {
    axes.len().hash(hasher);
    for axis in axes {
        axis.to_bits().hash(hasher);
    }
}

pub fn hash_optional_dsl_value<H: Hasher>(value: Option<&semio_framework_value::DslValue>, hasher: &mut H) {
    match value {
        Some(value) => {
            1_u8.hash(hasher);
            hash_dsl_value(value, hasher);
        }
        None => 0_u8.hash(hasher),
    }
}

/// 🧱️ Every persisted field of ONE object, vortices included — the geometry key's per-object half.
fn hash_object<H: Hasher>(object: &Puzzle3dObject, hasher: &mut H) {
    object.id.hash(hasher);
    object.label.hash(hasher);
    object.object_kind.hash(hasher);
    hash_axes(&object.origin, hasher);
    object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]).map(f64::to_bits).hash(hasher);
    hash_optional_dsl_value(object.scale.as_ref(), hasher);
    object.mesh_url.hash(hasher);
    object.hidden.hash(hasher);
    object.locked.hash(hasher);
    object.vortices.len().hash(hasher);
    for vortex in &object.vortices {
        vortex.id.hash(hasher);
        vortex.vortex_kind.hash(hasher);
        hash_axes(&vortex.position, hasher);
        vortex.direction.unwrap_or([0.0, 0.0, 0.0]).map(f64::to_bits).hash(hasher);
        vortex.radius.unwrap_or(0.0).to_bits().hash(hasher);
        vortex.hidden.hash(hasher);
        vortex.locked.hash(hasher);
    }
}

/// 🧵️ Kind meshes published ahead of every mesh url, in this order.
const WORLD_MESH_KINDS: [&str; 2] = [PUZZLE3D_FALLBACK_MESH_KIND, VORTEX_MARKER_MESH_KIND];

/// 🧵️ The world mesh lane in `meshesJson` order: the kind meshes, then every sorted mesh url whose mesh id
/// is new. A tool run trace subject's `mesh` is an index into exactly this list, so a fill run job is
/// built with it and `meshesJson` is published from it.
pub fn mesh_lane(scene_snapshot: &Puzzle3dSceneSnapshot) -> Vec<String> {
    let mut lane: Vec<String> = WORLD_MESH_KINDS.iter().map(|kind| (*kind).to_string()).collect();
    let mut ids: std::collections::HashSet<String> = lane.iter().cloned().collect();
    lane.extend(collect_mesh_urls(scene_snapshot).into_iter().filter(|url| ids.insert(world3d_mesh_id_from_url(url))));
    lane
}

pub fn world_meshes_json(scene_snapshot: &Puzzle3dSceneSnapshot) -> String {
    let lane = mesh_lane(scene_snapshot);
    world3d_meshes_json_from_kinds_and_urls(&lane[..WORLD_MESH_KINDS.len()], &lane[WORLD_MESH_KINDS.len()..])
}
