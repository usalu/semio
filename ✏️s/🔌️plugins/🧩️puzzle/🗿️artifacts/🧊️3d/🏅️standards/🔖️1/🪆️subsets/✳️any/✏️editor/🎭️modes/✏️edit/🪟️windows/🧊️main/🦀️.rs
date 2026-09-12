//! 🧊️ Puzzle 3d play app — the one `World3d` window kind. Owns the viewport's whole scene
//! projection: the instance/mesh/vortex/attraction/target-volume/reference payloads, the selection
//! and gumball descriptor, the LOD/chunking/environment blocks and the interaction channel (active
//! utility, suggestion popup, fill-build progress, reveal cutoffs) the host renderer reads. Also
//! owns the engagement HUD and collects its chrome measures from the mode's `☑️options/*` and its own
//! `🪛️utilities/*`.
//!
//! 🪟️ One KIND, many INSTANCES: the default layout splits it into an orthographic "Top" and a
//! three-point "Perspective" pane, and every view-local option (camera, grid, LOD, vortex display,
//! sun, selection method) is per instance — see `🦀️config.rs`'s `load_window`/`save_window`.

use crate::editor::puzzle3d::config::Puzzle3dRuntime;
use crate::editor::puzzle3d::modes::edit::options;
use crate::editor::puzzle3d::modes::edit::windows::main::utilities;
use crate::editor::puzzle3d::precompute::Puzzle3dPrecomputeSession;
use crate::editor::puzzle3d::terminology::{puzzle3d_localized, Puzzle3dLabels};
use crate::editor::puzzle3d::{
    collect_mesh_urls, object_scale_json, puzzle3d_action, puzzle3d_vortex_full_id, quat_rotate_vector, target_volume_scale_json, Puzzle3dFixture, Puzzle3dFixtureMeta, Puzzle3dInteractionSnapshot, Puzzle3dKindMeshIndex, Puzzle3dObject,
    Puzzle3dScene, Puzzle3dVortex, PUZZLE3D_FALLBACK_MESH_KIND, PUZZLE3D_INTERACTION_DOMAIN, PUZZLE3D_VORTEX_SHOW_ALWAYS,
};
use semio_framework_plugin::{
    world3d_camera_projection_json, world3d_chunking_json, world3d_environment_json, world3d_fit_json, world3d_mesh_id_from_url, world3d_meshes_json_from_kinds_and_urls, World3dScene, world3d_selection_json, SurfaceKind, WindowEngagement,
    WindowEngagementInput, WindowEngagementOption, WindowEngagementSlot, WindowKindDefinition, WindowMeasure, WindowOptions,
};
use semio_framework_ui_contract::BuiltNode;
use serde_json::{json, Value};
use std::hash::{Hash, Hasher};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "puzzle3d-main";
pub const WINDOW_INSTANCE_TOP: &str = "puzzle3d-main-top";
pub const WINDOW_INSTANCE_PERSPECTIVE: &str = "puzzle3d-main-perspective";
pub const BODY_KEY: &str = "puzzle3d.play.composite";
pub const SURFACE_VIEWPORT: &str = "puzzle.3d.play.viewport";
/// 🪟️ Display-template id for an orthographic top pane — mirrors `encodeWorldProjectionTemplateId({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } })`.
pub const TEMPLATE_TOP: &str = r#"world-projection:{"mode":{"kind":"orthographic"},"orientation":{"type":"cardinal","view":"top"}}"#;
/// 🪟️ Display-template id for a three-point perspective pane — mirrors `encodeWorldProjectionTemplateId({ mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } })`.
pub const TEMPLATE_PERSPECTIVE: &str = r#"world-projection:{"mode":{"kind":"threePoint","fov":50},"orientation":{"type":"free"}}"#;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle3d::create_puzzle3d_app`.
pub fn definition(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels) -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: puzzle3d_localized(|l| l.window_main),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "puzzle".into(),
        // 🪟️ `options.measures` stays empty: puzzle3d's chrome is config-derived per frame by
        // `ArtifactApp::window_measures`, never frozen into the static manifest.
        options: WindowOptions { measures: Vec::new(), engagement: WindowEngagementSlot::Some(engagement(envelope, labels)) },
        actions: Vec::new(),
        utilities: vec![utilities::transform::UTILITY_ID.into(), utilities::brush::UTILITY_ID.into(), utilities::volume_brush::UTILITY_ID.into(), utilities::world_relocate::UTILITY_ID.into()],
        interactions: vec![semio_framework_plugin::InteractionRef::new(PUZZLE3D_INTERACTION_DOMAIN)],
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🎚️ The live chrome measures for one window instance, collected from the mode's `☑️options/*`
/// components plus this window's own `🪛️utilities/*` option groups. `interaction` is the live
/// `vortex`-domain read the Brush placement picker gates itself on.
pub fn window_measures(envelope: &Puzzle3dScene, precompute: &Puzzle3dPrecomputeSession, labels: &Puzzle3dLabels, interaction: &Puzzle3dInteractionSnapshot) -> Vec<WindowMeasure> {
    vec![
        options::projection::measure(&envelope.runtime),
        options::vortex::show_measure(&envelope.runtime, labels),
        options::vortex::direction_measure(&envelope.runtime, labels),
        options::lod::measure(&envelope.runtime, labels),
        options::grid::measure(&envelope.runtime, labels),
        options::select::measure(&envelope.runtime, labels),
        options::sun::measure(&envelope.runtime),
        utilities::transform::options(&envelope.runtime, labels),
        utilities::brush::options(envelope, precompute, labels, interaction),
        utilities::volume_brush::options(&envelope.runtime, labels),
    ]
}
//#endregion 🔖️Definition

//#region 🔖️SceneMode
/// 🧭️ The select/brush/fill interaction mode the world engine reads, derived from the flat active
/// utility (the transform gumball and `worldRelocate` both present as `select`).
pub fn scene_mode(active_utility: &str) -> &str {
    match active_utility {
        "brush" => "brush",
        "fill" => "fill",
        "volumeBrush" => "volumeBrush",
        _ => "select",
    }
}

/// 🎚️ The gumball handle the world engine draws when a transform utility is active.
pub fn transform_handle(active_utility: &str) -> Option<&'static str> {
    if active_utility == utilities::transform::UTILITY_ID {
        Some("transform")
    } else {
        None
    }
}

/// 🧭️ Whether the active utility is a transform gumball mode.
pub fn transform_utility_active(active_utility: &str) -> bool {
    transform_handle(active_utility).is_some()
}

/// 🕹️ Whether the world gumball should render: the transform utility is active, at least one handle
/// flag is on (`setTransformGumballFlag` — an all-off gumball would draw nothing to grab), and the
/// live `vortex` selection holds at least one object or target volume to move. Selection comes from
/// the framework-owned domain via [`Puzzle3dInteractionSnapshot`], never from stored app state
/// (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub fn gumball_active(runtime: &Puzzle3dRuntime, active_utility: &str, interaction: &Puzzle3dInteractionSnapshot) -> bool {
    transform_utility_active(active_utility) && (runtime.transform_move || runtime.transform_rotate) && !(interaction.selected_object_ids().is_empty() && interaction.selected_target_volume_ids().is_empty())
}
//#endregion 🔖️SceneMode

//#region 🔖️SceneJson
pub fn camera_json(runtime: &Puzzle3dRuntime) -> String {
    let camera = &runtime.camera;
    world3d_camera_projection_json(camera.position, camera.target, camera.up, camera.zoom, &camera.projection)
}

/// 📷️ How many bounding-box spans the opening pose sits back from the document's centre — the guest
/// twin of `World3dHost`'s own `autofitCameraFromInstances` seed (`span * 2.5`) narrowed by
/// `world3dFrameCameraFromInstances`'s `padding / 2.5` (1.35 / 2.5), i.e. `2.5 * 0.54`.
const PUZZLE3D_FRAMING_SPANS: f64 = 1.35;
/// 📷️ Floor for the opening orbit radius, so a single-object or empty document still opens on a pose
/// the user can orbit rather than one sitting inside the geometry — mirrors the host's own `1.4`.
const PUZZLE3D_FRAMING_MINIMUM_DISTANCE: f64 = 1.4;

/// 📷️ Whether this pane's camera is still the all-zero `Puzzle3dCamera::default()` no gesture and no
/// stored `WindowConfig` has ever replaced. Position == target is degenerate in any pose — a camera
/// standing exactly on what it looks at has no view direction at all — so it is the one reading that
/// can never be a real user pose.
pub fn camera_unset(camera: &crate::editor::puzzle3d::config::Puzzle3dCamera) -> bool {
    camera.position == camera.target
}

/// 📷️ The axis-aligned centre and largest span of everything this document draws — objects,
/// references and target volumes, since a fixture may legitimately carry no objects at all and still
/// have something on screen to frame.
fn framing_bounds(fixture: &Puzzle3dFixture) -> ([f64; 3], f64) {
    let origins = fixture
        .objects
        .iter()
        .map(|object| object.origin)
        .chain(fixture.references.iter().map(|reference| reference.origin))
        .chain(fixture.target_volumes.iter().map(|volume| volume.origin));
    let mut minimum = [f64::INFINITY; 3];
    let mut maximum = [f64::NEG_INFINITY; 3];
    let mut seen = false;
    for origin in origins {
        seen = true;
        for axis in 0..3 {
            minimum[axis] = minimum[axis].min(origin[axis]);
            maximum[axis] = maximum[axis].max(origin[axis]);
        }
    }
    if !seen {
        return ([0.0, 0.0, 0.0], 1.0);
    }
    let centre = [(minimum[0] + maximum[0]) * 0.5, (minimum[1] + maximum[1]) * 0.5, (minimum[2] + maximum[2]) * 0.5];
    let span = (maximum[0] - minimum[0]).max(maximum[1] - minimum[1]).max(maximum[2] - minimum[2]).max(1.0);
    (centre, span)
}

/// 📷️ The projection a pane opens under, from the display template its layout entry declares
/// ([`TEMPLATE_TOP`] / [`TEMPLATE_PERSPECTIVE`], stitched in `🎭️modes/✏️edit/🦀️.rs`): the Top pane is
/// an orthographic plan, every other instance keeps the three-point default.
fn framing_projection(window_id: &str) -> semio_framework_plugin::WorldProjectionConfig {
    let mut projection = semio_framework_plugin::WorldProjectionConfig::default();
    if window_id == WINDOW_INSTANCE_TOP {
        projection.kind = "orthographic".into();
        projection.orthographic_view = "top".into();
    }
    projection
}

/// 📷️ The pose ONE pane opens on before any user gesture — framed on what the document actually
/// holds, oriented by that pane's own display template. Without it `Puzzle3dCamera::default()` leaves
/// every pane's published camera at all zeros until the first `setCamera` lands, so the world lane
/// carries no view direction at boot and two panes of one document publish the SAME (zero) pose
/// (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B15; B12 §4.1 measured it live).
pub fn framed_camera(window_id: &str, fixture: &Puzzle3dFixture) -> crate::editor::puzzle3d::config::Puzzle3dCamera {
    let (target, span) = framing_bounds(fixture);
    let projection = framing_projection(window_id);
    let distance = (span * PUZZLE3D_FRAMING_SPANS).max(PUZZLE3D_FRAMING_MINIMUM_DISTANCE);
    let (position, up) = semio_framework_plugin::world3d_projection_pose(&projection, target, distance);
    crate::editor::puzzle3d::config::Puzzle3dCamera { position, target, zoom: 1.0, up: Some(up), projection }
}

/// 📷️ Gives a scene whose pane has never been framed its opening pose, and leaves a pane the user (or
/// a stored `WindowConfig`) has already posed exactly as it is. Idempotent, so every call site on the
/// render/measure/dispatch paths may run it unconditionally.
pub fn frame_unset_camera(scene: &mut Puzzle3dScene, window_id: &str) {
    if camera_unset(&scene.runtime.camera) {
        scene.runtime.camera = framed_camera(window_id, &scene.fixture);
    }
}

/// 🙈️ Hidden objects stay in the emitted array — `worldPick`'s `id` arg is the array index into it — but render at zero scale so they're effectively invisible without shifting any other object's index.
/// `revealIndex` is omitted entirely for untagged objects rather than emitted as `null`: the host's reveal cutoff (`framework/renderer/react`'s `applyRevealCutoff`) only skips instances with no reveal index, and a JSON `null` would coerce to `0` and hide every ordinary object behind the boot cutoff.
/// Selection/hover paint is driven by `selectionJson` on the host — never baked here so instance geometry stays stable across picks.
pub fn world_instances_geometry_json(fixture: &Puzzle3dFixture) -> String {
    let mut residency = Puzzle3dInstanceResidency::default();
    residency.refresh(fixture);
    residency.instances_json().to_string()
}

/// 🧱️ ONE instance record, exactly the shape [`world_instances_geometry_json`] publishes.
fn instance_record_json(object: &Puzzle3dObject, mesh_id: &str) -> String {
    let scale = if object.hidden { json!([0.0, 0.0, 0.0]) } else { json!(object_scale_json(object)) };
    let mut instance = json!({
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
        instance["objectKind"] = json!(kind);
    }
    if let Some(reveal_index) = object.reveal_index {
        instance["revealIndex"] = json!(reveal_index);
    }
    serde_json::to_string(&instance).unwrap_or_else(|_| "{}".into())
}

/// 🔑️ The per-object change key for ONE instance record — every field [`instance_record_json`] reads
/// and nothing else, hashed structurally. No JSON is materialized: this is the difference between a
/// per-object key that costs a handful of `Hash::hash` calls and the whole-document
/// `format!`-then-hash [`fixture_geometry_fingerprint`] used to be (measured 20 716 µs on the
/// 180-object Nakagin document, ticket 26/09/02 wave B44 §1).
fn instance_record_fingerprint(object: &Puzzle3dObject, mesh_id: &str) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    object.id.hash(&mut hasher);
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
    object.reveal_index.hash(&mut hasher);
    hasher.finish()
}

/// 🧮️ Structural hash of one DSL value — the allocation-free replacement for hashing its JSON text.
/// Floats go in by `to_bits`, so the key is exact rather than format-dependent, and an object's keys
/// are hashed in their authored order (the order `ToValue` emits, which is what the previous JSON
/// hash was sensitive to as well).
pub fn hash_dsl_value<H: Hasher>(value: &dsl::DslValue, hasher: &mut H) {
    match value {
        dsl::DslValue::Null => 0_u8.hash(hasher),
        dsl::DslValue::Bool(flag) => {
            1_u8.hash(hasher);
            flag.hash(hasher);
        }
        dsl::DslValue::Number(number) => {
            2_u8.hash(hasher);
            match number {
                dsl::Number::UInt(value) => value.hash(hasher),
                dsl::Number::Int(value) => value.hash(hasher),
                dsl::Number::Float(value) => value.to_bits().hash(hasher),
            }
        }
        dsl::DslValue::String(text) => {
            3_u8.hash(hasher);
            text.hash(hasher);
        }
        dsl::DslValue::Array(items) => {
            4_u8.hash(hasher);
            items.len().hash(hasher);
            for item in items {
                hash_dsl_value(item, hasher);
            }
        }
        dsl::DslValue::Object(entries) => {
            5_u8.hash(hasher);
            entries.len().hash(hasher);
            for (key, item) in entries {
                key.hash(hasher);
                hash_dsl_value(item, hasher);
            }
        }
    }
}

/// 🚚️ Per-object residency for the world `instances` lane: the one place the published instance text
/// is built, kept and invalidated PER OBJECT instead of per document.
///
/// ⏱️ Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B44. Before it, one viewport render of the 180-object
/// Nakagin document paid 20 716 µs hashing the whole fixture's JSON plus 2 449 µs re-serializing all
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
    entries: std::collections::HashMap<String, (u64, String)>,
    order: Vec<String>,
    assembled: String,
    revision: u64,
    changed: Vec<String>,
    removed: Vec<String>,
    delta: String,
    rebuilt: u32,
}

impl Puzzle3dInstanceResidency {
    /// 🔄️ Reconciles the residency against one fixture. Answers whether the published text changed —
    /// `false` means every record and the order are bit-identical and no consumer owes any work.
    pub fn refresh(&mut self, fixture: &Puzzle3dFixture) -> bool {
        let kind_meshes = Puzzle3dKindMeshIndex::of(&fixture.meta);
        let mut order = Vec::with_capacity(fixture.objects.len());
        let mut changed = Vec::new();
        let mut rebuilt = 0_u32;
        for object in &fixture.objects {
            let mesh_id = kind_meshes.resolve(object).map_or_else(|| PUZZLE3D_FALLBACK_MESH_KIND.into(), world3d_mesh_id_from_url);
            let fingerprint = instance_record_fingerprint(object, &mesh_id);
            if self.entries.get(&object.id).is_none_or(|(cached, _)| *cached != fingerprint) {
                self.entries.insert(object.id.clone(), (fingerprint, instance_record_json(object, &mesh_id)));
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
        // 🪟️ A no-op refresh leaves the LAST delta standing: two window instances of one document render
        // off one session, so the second render must still be able to hand its own consumer the delta the
        // first render produced (both carry the same `base`/`revision`, and a consumer whose retained set
        // is not at `base` falls back to `instancesJson`).
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

/// 🎯️ Padding `WorldAutoFit` frames a swapped document with — 1.25 leaves a quarter of the radius of
/// air around the bounding sphere, matching the host's own default.
pub const PUZZLE3D_FIT_PADDING: f64 = 1.25;

/// 🎯️ Document IDENTITY for the world's `fit` lane: what this document IS (its schema, its domain and
/// its kind catalogs), never what its geometry currently holds.
///
/// `WorldAutoFit` refits once per `${revision}:${meshes}` key, so this is the difference between
/// "frame the new fixture when the user switches example" and "yank the camera every time an object
/// moves": a catalog swap is a document swap, an object edit is not. Without the lane at all the
/// camera after a swap is whatever the previous document left in `cameraJson` — Nakagin happens to
/// sit inside Concrete Forest's framing, a fixture centred elsewhere would simply be off-screen
/// (ticket 26/09/02 W-P5 §7).
pub fn world_fit_revision(fixture: &Puzzle3dFixture) -> u32 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    fixture.schema.hash(&mut hasher);
    fixture.domain.hash(&mut hasher);
    hash_optional_dsl_value(fixture.meta.kind_catalogs.as_ref(), &mut hasher);
    hash_optional_dsl_value(fixture.meta.kind_compatibility.as_ref(), &mut hasher);
    (hasher.finish() >> 32) as u32
}

/// 🗄️ Cheap change key for everything the instance/mesh payloads (and the document tree) derive from.
///
/// ⏱️ Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B44: hashed STRUCTURALLY, never through JSON text.
/// The `format!` of four `to_json_string` calls this used to be materialized ~60 KiB per call on the
/// 180-object Nakagin document and measured **20 716 µs** — 54 % of a cache-HIT viewport render, paid
/// again per window and again for the outliner memo's key. The structural walk is the same O(n) key
/// over the same four members with none of the allocation.
pub fn fixture_geometry_fingerprint(fixture: &Puzzle3dFixture) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    fixture.objects.len().hash(&mut hasher);
    for object in &fixture.objects {
        hash_object(object, &mut hasher);
    }
    fixture.references.len().hash(&mut hasher);
    for reference in &fixture.references {
        reference.id.hash(&mut hasher);
        reference.source.url.hash(&mut hasher);
        reference.source.media_kind.hash(&mut hasher);
        hash_axes(&reference.origin, &mut hasher);
        reference.width_world.to_bits().hash(&mut hasher);
        reference.locked.hash(&mut hasher);
        reference.hidden.hash(&mut hasher);
    }
    fixture.target_volumes.len().hash(&mut hasher);
    for volume in &fixture.target_volumes {
        volume.id.hash(&mut hasher);
        hash_axes(&volume.origin, &mut hasher);
        volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]).map(f64::to_bits).hash(&mut hasher);
        hash_optional_dsl_value(volume.scale.as_ref(), &mut hasher);
        volume.hidden.hash(&mut hasher);
        volume.locked.hash(&mut hasher);
    }
    hash_optional_dsl_value(fixture.meta.kind_catalogs.as_ref(), &mut hasher);
    hash_optional_dsl_value(fixture.meta.kind_compatibility.as_ref(), &mut hasher);
    hasher.finish()
}

fn hash_axes<H: Hasher>(axes: &[f64], hasher: &mut H) {
    axes.len().hash(hasher);
    for axis in axes {
        axis.to_bits().hash(hasher);
    }
}

fn hash_optional_dsl_value<H: Hasher>(value: Option<&dsl::DslValue>, hasher: &mut H) {
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
    object.reveal_index.hash(hasher);
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

pub fn world_meshes_json(fixture: &Puzzle3dFixture) -> String {
    let urls = collect_mesh_urls(fixture);
    let kinds = vec![PUZZLE3D_FALLBACK_MESH_KIND.into(), "vortex-marker".into()];
    if urls.is_empty() {
        return world3d_meshes_json_from_kinds_and_urls(&kinds, &[]);
    }
    let mut meshes_json = world3d_meshes_json_from_kinds_and_urls(&kinds, &urls);
    if !meshes_json.contains(PUZZLE3D_FALLBACK_MESH_KIND) {
        let fallback = world3d_meshes_json_from_kinds_and_urls(&[PUZZLE3D_FALLBACK_MESH_KIND.into()], &[]);
        let mut merged: Vec<Value> = serde_json::from_str(&meshes_json).unwrap_or_default();
        let fallback_meshes: Vec<Value> = serde_json::from_str(&fallback).unwrap_or_default();
        merged.extend(fallback_meshes);
        meshes_json = serde_json::to_string(&merged).unwrap_or(meshes_json);
    }
    meshes_json
}

fn world_vortex_direction(object: &Puzzle3dObject, vortex: &Puzzle3dVortex) -> [f64; 3] {
    let direction = vortex.direction.unwrap_or([0.0, 0.0, -1.0]);
    quat_rotate_vector(object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), direction)
}

fn vortex_color(meta: &Puzzle3dFixtureMeta, vortex_kind: Option<&str>) -> String {
    catalog_entry_field(meta, "vortices", vortex_kind, &["color"], "#38bdf8")
}

fn object_kind_color(meta: &Puzzle3dFixtureMeta, object_kind: Option<&str>) -> String {
    catalog_entry_field(meta, "objects", object_kind, &["color"], "#38bdf8")
}

fn object_kind_icon(meta: &Puzzle3dFixtureMeta, object_kind: Option<&str>) -> String {
    catalog_entry_field(meta, "objects", object_kind, &["icon", "iconId"], "box")
}

/// 🎨️ First present `fields` entry on the `section` catalog row whose `id` is `kind_id`, else `fallback`.
fn catalog_entry_field(meta: &Puzzle3dFixtureMeta, section: &str, kind_id: Option<&str>, fields: &[&str], fallback: &str) -> String {
    let Some(kind_id) = kind_id else {
        return fallback.into();
    };
    let Some(catalogs) = meta.kind_catalogs.as_ref() else {
        return fallback.into();
    };
    let Some(entries) = catalogs.get(section).and_then(|value| value.as_array()) else {
        return fallback.into();
    };
    for entry in entries {
        if entry.get("id").and_then(|value| value.as_str()) == Some(kind_id) {
            return fields.iter().find_map(|field| entry.get(field).and_then(|value| value.as_str()).filter(|text| !text.is_empty())).unwrap_or(fallback).to_string();
        }
    }
    fallback.into()
}

/// 🖌️ Placement utilities that publish every object's vortex markers so the host can hit-test them
/// without a prior object selection (those utilities block instance pick).
fn vortex_markers_publish_for_utility(active_utility: &str) -> bool {
    matches!(active_utility, "brush" | "volumeBrush")
}

/// 👁️ True when this object's vortices should render — Always mode, a live hover/selection touch, or
/// an armed placement utility. `PUZZLE3D_VORTEX_SHOW_SELECTED` otherwise hides markers until the
/// object (or one of its own vortices) is marked.
fn object_vortices_visible(object: &Puzzle3dObject, runtime: &Puzzle3dRuntime, interaction: &Puzzle3dInteractionSnapshot, active_utility: &str) -> bool {
    runtime.vortex_show == PUZZLE3D_VORTEX_SHOW_ALWAYS || interaction.touches_object(object) || vortex_markers_publish_for_utility(active_utility)
}

/// 🌀️ Per-vortex marker records. `selected`/`hovered` are painted from the live `vortex` domain —
/// `WorldVortexMarkers` reads them off each record (its own palette lookup), not off `selectionJson`.
pub fn world_vortices_json(fixture: &Puzzle3dFixture, runtime: &Puzzle3dRuntime, interaction: &Puzzle3dInteractionSnapshot, active_utility: &str) -> String {
    let selected_vortices = interaction.selected_vortex_ids();
    let mut records = Vec::new();
    for object in &fixture.objects {
        if !object_vortices_visible(object, runtime, interaction, active_utility) {
            continue;
        }
        for vortex in &object.vortices {
            let position = crate::editor::puzzle3d::world_vortex_position(object, vortex);
            let direction = world_vortex_direction(object, vortex);
            let full_id = puzzle3d_vortex_full_id(&object.id, &vortex.id);
            records.push(json!({
                "fullId": full_id,
                "objectId": object.id,
                "vortexKind": vortex.vortex_kind,
                "position": position,
                "direction": direction,
                "radius": vortex.radius.unwrap_or(0.36),
                "color": vortex_color(&fixture.meta, vortex.vortex_kind.as_deref()),
                "displayDirection": runtime.vortex_direction,
                "selected": selected_vortices.iter().any(|id| id == &full_id),
                "hovered": interaction.hovered.iter().any(|id| id == &full_id),
            }));
        }
    }
    serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
}

pub fn world_attractions_json(fixture: &Puzzle3dFixture) -> String {
    let records: Vec<Value> = fixture
        .attractions
        .iter()
        .filter_map(|attraction| {
            let from = crate::editor::puzzle3d::resolve_vortex_world_position(fixture, &attraction.attracting)?;
            let to = crate::editor::puzzle3d::resolve_vortex_world_position(fixture, &attraction.attracted)?;
            Some(json!({
                "id": attraction.id,
                "from": from,
                "to": to,
                "color": "#60a5fa",
            }))
        })
        .collect();
    serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
}

pub fn world_target_volumes_json(fixture: &Puzzle3dFixture) -> String {
    let records: Vec<Value> = fixture
        .target_volumes
        .iter()
        .map(|volume| {
            json!({
                "id": volume.id,
                "origin": volume.origin,
                "orientation": volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                "scale": target_volume_scale_json(volume),
                "color": "#f472b6",
                "hidden": volume.hidden,
                "locked": volume.locked,
            })
        })
        .collect();
    serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
}

pub fn world_references_json(fixture: &Puzzle3dFixture) -> String {
    let records: Vec<Value> = fixture
        .references
        .iter()
        .map(|reference| {
            json!({
                "id": reference.id,
                "url": reference.source.url,
                "origin": reference.origin,
                "widthWorld": if reference.width_world > 0.0 { reference.width_world } else { 1.0 },
                "locked": reference.locked,
                "hidden": reference.hidden,
            })
        })
        .collect();
    serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
}

/// 🖌️ How many placement candidates ONE suggestion popup publishes. The scene surface it travels on is
/// a fixed-capacity payload (`semio_framework_ui_scene::encode`), and a vortex on a richly catalogued
/// document resolves arbitrarily many collision-free candidates — an unbounded list would make the whole
/// 3D render of that window fail closed the moment the popup opened. It is also the interaction answer:
/// a picker the user reads at a glance shows a bounded page, and `cycleBrushCandidate` walks the rest.
pub const PUZZLE3D_SUGGESTION_MENU_CANDIDATE_PAGE: usize = 8;

pub fn world_interaction_json(envelope: &Puzzle3dScene, session: &Puzzle3dPrecomputeSession, interaction: &Puzzle3dInteractionSnapshot, brush_preview: Option<&str>) -> String {
    let runtime = &envelope.runtime;
    let suggestion_menu = runtime.suggestion_menu.as_ref().map(|menu| {
        let (pending, candidates) = if !menu.vortex_full_id.is_empty() {
                let result = session.brush_candidates(&menu.vortex_full_id);
                let candidates: Vec<Value> = result
                    .free
                    .iter()
                    .take(PUZZLE3D_SUGGESTION_MENU_CANDIDATE_PAGE)
                    .enumerate()
                    .map(|(index, candidate)| {
                        let object_kind = Some(candidate.object_kind_id.as_str());
                        let object_label = candidate.object_kind_id.as_str();
                        let source_vortex_index = candidate.source_vortex_index;
                        let color = object_kind_color(&envelope.fixture.meta, object_kind);
                        let icon = object_kind_icon(&envelope.fixture.meta, object_kind);
                        json!({
                            "index": index,
                            "objectLabel": object_label,
                            "vortexLabel": format!("vortex {source_vortex_index}"),
                            "icon": icon,
                            "color": color,
                        })
                    })
                    .collect();
                (result.unknown_pending, candidates)
            } else { (false, Vec::new()) };
        eprintln!("[DEBUG] puzzle3d.openVortex.cache menu vortex={} pending={pending} candidates={}", menu.vortex_full_id, candidates.len());
        json!({
            "open": true,
            "x": menu.x,
            "y": menu.y,
            "windowId": menu.window_id,
            "vortexFullId": menu.vortex_full_id,
            "pending": pending,
            "candidates": candidates,
        })
    });
    let fill_build = session.fill_progress_summary();
    let fill_build = json!({
        "count": fill_build.count,
        "appliedCount": fill_build.applied_count,
        "maxCount": fill_build.max_count,
        "done": fill_build.done,
    });
    // 🪣️ Committed fill count as a viewport reveal cutoff — instances tagged `revealIndex` (see
    // `world_instances_geometry_json`) below this value are shown, the rest (already planned, not yet
    // committed) stay hidden until the host commits a higher value or the live drag store overrides
    // it locally. Keyed so future reveal-driven measures/tools can share the same channel.
    // 🥽️ Brush-mesh residency, the client's only handle on the fact that what a guest instantiation
    // holds does not outlive that instantiation: `meshResidency` is the guest's monotone install
    // counter (a lower value than the client last saw proves a restart, so its "already uploaded"
    // bookkeeping is void), and `meshReuploadUrls` names the identities a refused id-only announcement
    // is waiting on bytes for. Both are read by `Puzzle3dBrushMeshRegistry`
    // (`🧰️framework/…/🛠️ShellHelpers/🟦️.tsx`).
    let mut value = json!({
        "activeUtility": scene_mode(&envelope.active_utility),
        "brushCandidateIndex": runtime.brush_candidate_index,
        "voxelDims": runtime.voxel_dims,
        "gridFactor": runtime.grid_spacing,
        "suggestionMenu": suggestion_menu,
        "fillBuild": fill_build,
        "revealCutoffs": { "puzzle3d-fill": runtime.fill_count },
        "meshResidency": crate::editor::puzzle3d::precompute::shared_brush_mesh_installs(),
        "meshReuploadUrls": session.mesh_reupload_requests(),
    });
    // 🐁️ `hoveredVortexFullId` is the host's Alt+right-click suggestion target and its context-menu
    // priority key (`resolveWorldContextMenuTarget`) — it lives on the interaction record, not on
    // `selectionJson`, so it is projected from the live `vortex`-domain hover here.
    if let (Some(object), Some(hovered)) = (value.as_object_mut(), interaction.hovered_vortex_full_id(&envelope.fixture)) {
        object.insert("hoveredVortexFullId".into(), json!(hovered));
    }
    if let (Some(object), Some(preview)) = (value.as_object_mut(), brush_preview.filter(|payload| !payload.is_empty())) {
        object.insert("brushPreviewJson".into(), json!(preview));
    }
    value.to_string()
}

pub fn world3d_lod_json(runtime: &Puzzle3dRuntime) -> String {
    json!({
        "gridFactor": runtime.grid_spacing,
        "gridSnapEnabled": runtime.grid_snap_enabled,
        "showLodGrid": runtime.grid_visible,
        "automaticLod": runtime.lod_automatic,
        "depthVariableLod": runtime.lod_depth_variable,
        "manualLod": runtime.lod_manual,
    })
    .to_string()
}

/// 👻️ Ghost placement for the brush utility, or for a one-shot context-menu / Alt+right-click
/// suggestion popup (`suggestion_menu`) that must not switch the host-owned active utility into brush.
pub fn world_brush_preview_target(session: &Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene, interaction: &Puzzle3dInteractionSnapshot) -> Option<String> {
    envelope
        .runtime
        .suggestion_menu
        .as_ref()
        .map(|menu| menu.vortex_full_id.clone())
        .filter(|id| !id.is_empty())
        .or_else(|| crate::editor::puzzle3d::puzzle3d_brush_target_vortex(envelope, interaction))
        .or_else(|| session.brush_live_target().map(str::to_string))
}

pub fn world_brush_preview_json(session: &Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene, interaction: &Puzzle3dInteractionSnapshot) -> Option<String> {
    let brush = envelope.active_utility == utilities::brush::UTILITY_ID;
    let menu = envelope.runtime.suggestion_menu.is_some();
    if !brush && !menu {
        return None;
    }
    let vortex_id = world_brush_preview_target(session, envelope, interaction);
    eprintln!("[DEBUG] puzzle3d.brushPreview.hover utility={} brush={brush} menu={menu} vortex={:?}", envelope.active_utility, vortex_id);
    let Some(vortex_id) = vortex_id else {
        eprintln!("[DEBUG] puzzle3d.brushPreview.gate reason=no-target utility={} brush={brush} menu={menu}", envelope.active_utility);
        return None;
    };
    let cache = session.brush_candidates(&vortex_id);
    eprintln!("[DEBUG] puzzle3d.brushPreview.cache vortex={vortex_id} free={} pending={} resume={}", cache.free.len(), cache.unknown_pending, cache.resume_candidate_index);
    let Some(preview) = session.brush_preview(&vortex_id, envelope.runtime.brush_candidate_index) else {
        eprintln!(
            "[DEBUG] puzzle3d.brushPreview.gate reason=no-free-candidate vortex={vortex_id} free={} pending={} index={}",
            cache.free.len(),
            cache.unknown_pending,
            envelope.runtime.brush_candidate_index
        );
        return None;
    };
    let color = object_kind_color(&envelope.fixture.meta, Some(preview.object_kind_id.as_str()));
    let mut value = dsl::ToValue::to_value(&preview);
    if let dsl::DslValue::Object(entries) = &mut value {
        entries.push(("color".to_string(), dsl::DslValue::String(color)));
    }
    let json = dsl::json::to_json_string(&value);
    eprintln!("[DEBUG] puzzle3d.brushPreview.compute vortex={vortex_id} bytes={}", json.len());
    Some(json)
}

/// 🪣️ Latest-wins bounded fill diagnostic, with an optional ghost projection.
pub fn world_fill_preview_json(session: &Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene, labels: &Puzzle3dLabels) -> Option<String> {
    if envelope.active_utility != "fill" {
        return None;
    }
    let object_kind = session.fill_preview_object_kind();
    let color = object_kind_color(&envelope.fixture.meta, object_kind.as_deref());
    session.fill_preview_json_page(&color, labels.fill_progress.as_str())
}

/// 🕹️ The host's `WorldSelectionRecord` (`World3dHost/🟦️.tsx` `parseSelection`) for this window: the
/// framework-owned `vortex` domain projected onto exactly the field names that parser reads —
/// `ids`/`activeObjectId` (object instances), `targetVolumeIds`, `referenceSelectedId`,
/// `hoveredId`/`hoveredKindId`, plus the gumball descriptor. Vortex selection/hover is deliberately
/// NOT here: `WorldSelectionRecord` has no vortex field, so a marker's own `selected`/`hovered` flags
/// travel on `vorticesJson` (see `world_vortices_json`), which is where `WorldVortexMarkers` reads
/// them. Ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM.
pub fn world_selection_json(envelope: &Puzzle3dScene, interaction: &Puzzle3dInteractionSnapshot) -> String {
    let runtime = &envelope.runtime;
    let object_ids = interaction.selected_object_ids();
    let hovered_id = interaction.hovered_object_id(&envelope.fixture).map(str::to_string).or_else(|| interaction.hovered_reference_id(&envelope.fixture).map(|id| format!("reference:{id}")));
    let mut value: Value = serde_json::from_str(&world3d_selection_json(runtime.selection_method.as_str(), object_ids, hovered_id.as_deref())).unwrap_or_else(|_| json!({}));
    if let Some(object) = value.as_object_mut() {
        object.insert("granularity".into(), json!("mesh"));
        object.insert("selectionMode".into(), json!("mesh"));
        object.insert(
            "targets".into(),
            json!({
                "mesh": true,
                "vertex": false,
                "edge": false,
                "face": false,
            }),
        );
        object.insert("targetVolumeIds".into(), json!(interaction.selected_target_volume_ids()));
        if let Some(id) = object_ids.first() {
            object.insert("activeObjectId".into(), json!(id));
        }
        if let Some(id) = interaction.selected_reference_ids().first() {
            object.insert("referenceSelectedId".into(), json!(id));
        }
        if let Some(kind) = hovered_kind_id(&envelope.fixture, interaction) {
            object.insert("hoveredKindId".into(), json!(kind));
        }
        if let Some(transform_mode) = transform_handle(&envelope.active_utility) {
            object.insert("transformMode".into(), json!(transform_mode));
            object.insert(
                "gumballConfig".into(),
                json!({
                    "moveAxes": runtime.transform_move,
                    "movePlanes": runtime.transform_move,
                    "rotate": runtime.transform_rotate,
                    "scaleAxes": false,
                    "scalePlanes": false,
                    "scaleUniform": false,
                }),
            );
        }
        object.insert("gumballActive".into(), json!(gumball_active(runtime, &envelope.active_utility, interaction)));
    }
    value.to_string()
}

/// 🎨️ The hovered CATALOGUE kind id, when the `kind` granularity is what the pointer is over — the
/// host highlights every instance sharing that `objectKind` from this one field.
fn hovered_kind_id<'a>(fixture: &Puzzle3dFixture, interaction: &'a Puzzle3dInteractionSnapshot) -> Option<&'a str> {
    let catalogs = fixture.meta.kind_catalogs.as_ref()?;
    let entries = catalogs.get("objects").and_then(|value| value.as_array())?;
    interaction.hovered.iter().find(|id| entries.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(id.as_str()))).map(String::as_str)
}

//#endregion 🔖️SceneJson

//#region 🔖️Render
/// 🖼️ The world-3d surface node for this window — `instances_json`/`meshes_json` come pre-computed
/// from `Puzzle3dPlayApp`'s geometry cache (they only change with the fixture's geometry fingerprint).
pub fn render(
    envelope: &Puzzle3dScene,
    precompute: &Puzzle3dPrecomputeSession,
    labels: &Puzzle3dLabels,
    instances_json: String,
    meshes_json: String,
    instances_delta_json: Option<String>,
    interaction: &Puzzle3dInteractionSnapshot,
) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let brush_preview = world_fill_preview_json(precompute, envelope, labels).or_else(|| world_brush_preview_json(precompute, envelope, interaction));
    let vortices = world_vortices_json(&envelope.fixture, &envelope.runtime, interaction, envelope.active_utility.as_str());
    eprintln!(
        "[DEBUG] puzzle3d.brushPreview.lane utility={} preview={} vortices={}",
        envelope.active_utility,
        brush_preview.as_ref().map(String::len).unwrap_or(0),
        vortices.len()
    );
    eprintln!("[DEBUG] puzzle3d.vortices.publish utility={} bytes={} brush_or_volume={}", envelope.active_utility, vortices.len(), matches!(envelope.active_utility.as_str(), "brush" | "volumeBrush"));
    let mut scene = World3dScene::base(camera_json(&envelope.runtime), meshes_json, instances_json, world_selection_json(envelope, interaction));
    scene.instances_delta_json = instances_delta_json;
    scene.vortices_json = Some(vortices);
    scene.attractions_json = Some(world_attractions_json(&envelope.fixture));
    scene.target_volumes_json = Some(world_target_volumes_json(&envelope.fixture));
    scene.references_json = Some(world_references_json(&envelope.fixture));
    scene.interaction_json = Some(world_interaction_json(envelope, precompute, interaction, brush_preview.as_deref()));
    scene.brush_preview_json = Some(brush_preview.unwrap_or_default());
    scene.lod_json = Some(world3d_lod_json(&envelope.runtime));
    scene.chunking_json = Some(world3d_chunking_json(envelope.runtime.chunk_size, 8000.0));
    scene.environment_json = Some(world3d_environment_json(&envelope.runtime.sun));
    scene.fit_json = Some(world3d_fit_json(world_fit_revision(&envelope.fixture), PUZZLE3D_FIT_PADDING));
    // 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): bound, so `World3dHost`'s generic
    // dispatch path emits `interactionSelect`/`interactionHover` for this domain
    // (`world3dSelectionActionArgs`/`world3dHoverActionArgs`) instead of the legacy
    // `worldPick`/`worldSelect`/`setHover` verbs this crate has no handler for. Granularity is
    // declared explicitly because the host's own default (`"handle"`) is not one of this domain's
    // granularities and `validate_state` would prune every id picked under it.
    scene.domain_id = Some(PUZZLE3D_INTERACTION_DOMAIN.into());
    scene.domain_granularity_id = Some(crate::editor::puzzle3d::PUZZLE3D_GRANULARITY_OBJECT.into());
    semio_framework_plugin::scene_surface(SURFACE_VIEWPORT, semio_framework_ui_contract::SurfaceKind::World3d, &scene)
}

/// 🤝️ The engagement HUD for this window: the select/brush/fill switcher lives in the framework
/// utility bar (declared via `.utility` + `.window_kind_utilities`); the fill-count slider, voxel
/// steppers and brush placement picker are tagged [`WindowMeasure::Group`]s surfaced in the dedicated
/// "Utility Options" rail. The remaining chrome is the Add Object dialog opener, a command input, and a status line.
pub fn engagement(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels) -> WindowEngagement {
    let object_count = envelope.fixture.objects.len();
    let attraction_count = envelope.fixture.attractions.len();
    let active_utility = envelope.active_utility.as_str();
    let objects_label = labels.objects.as_str();
    let attractions_label = labels.attractions.as_str();
    let object_word = labels.object.as_str();
    let add_object_label = if object_word.eq_ignore_ascii_case("objekt") { format!("{object_word} hinzufügen…") } else { format!("Add {object_word}…") };
    WindowEngagement {
        session_active: Some(engagement_session_active(active_utility)),
        options: Some(vec![WindowEngagementOption {
            id: "shell-menu.action.openAddObjectDialog".into(),
            label: Some(add_object_label),
            icon_id: Some("plus".into()),
            pressed: None,
            disabled: None,
            action: Some(puzzle3d_action("openAddObjectDialog", None)),
        }]),
        input: Some(WindowEngagementInput {
            id: Some("puzzle3d-engagement".into()),
            value: Some(envelope.runtime.engagement_input.clone()),
            placeholder: Some(crate::editor::puzzle3d::commands::engagement_submit::PUZZLE3D_ENGAGEMENT_VERBS.join(", ")),
            disabled: None,
            on_change: Some(puzzle3d_action("engagementInput", None)),
            on_submit: Some(puzzle3d_action("engagementSubmit", None)),
            on_repeat_last: Some(puzzle3d_action("engagementRepeatLast", None)),
            on_abort: Some(puzzle3d_action("engagementAbort", None)),
        }),
        control: None,
        controls: None,
        status: Some(vec![semio_framework_plugin::WindowEngagementStatus { id: "puzzle3d-world-status".into(), text: format!("{object_count} {objects_label} · {attraction_count} {attractions_label}") }]),
        possible_engagements: None,
    }
}

/// 🧭️ Whether the engagement HUD should mark an active session for the given utility.
fn engagement_session_active(active_utility: &str) -> bool {
    matches!(active_utility, "brush" | "fill" | "worldRelocate")
}
//#endregion 🔖️Render

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod vortex_payload_laws;
