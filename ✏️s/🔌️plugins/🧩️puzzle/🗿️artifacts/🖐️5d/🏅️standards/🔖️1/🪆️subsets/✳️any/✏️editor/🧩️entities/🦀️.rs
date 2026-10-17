//! 🧩️ Entities of the puzzle 5d editor: the reserved sizes and scoped ids, the structural-twin document model (grips, fasteners, parts, target volumes) and the grip, mesh and geometry helpers that read it. See [`super`].

use crate::editor::puzzle5d::snapshot::Puzzle5dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;
use semio_framework_plugin::{kernel::PasteAnchor, kernel::PastePlacement};
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::InteractionTarget;
use semio_framework_plugin::INTERACTION_SELECT_ACTION_ID;
use semio_framework_value::{FromValue, ToValue};
use semio_framework_pack_json::Value;
use std::collections::HashSet;
use std::sync::LazyLock;

//#region 🔖️Constants
pub const PUZZLE5D_PLAY_APP_ID: &str = "puzzle5d-play";
pub const PUZZLE5D_PLAY_CONTROLLER_ID: &str = "puzzle5d-play";
pub const PUZZLE5D_BOARD_WINDOW_KIND_ID: &str = "puzzle5d-2d";
pub const PUZZLE5D_WORLD_WINDOW_KIND_ID: &str = "puzzle5d-3d";
pub const PUZZLE5D_BRUSH_UTILITY_ID: &str = "brush";
pub const PUZZLE5D_FILL_TOOL_ID: &str = "fill";
pub const PUZZLE5D_TRANSFORM_UTILITY_ID: &str = "transform";
pub const PUZZLE5D_PLAY_WINDOWS: [&str; 2] = [PUZZLE5D_BOARD_WINDOW_KIND_ID, PUZZLE5D_WORLD_WINDOW_KIND_ID];
pub const PUZZLE5D_SCHEMA: &str = "puzzle.5d";
pub const PUZZLE5D_BOARD_SNAPSHOT_SCHEMA: &str = "board.ports.directed.v1";
pub const PUZZLE5D_EXAMPLE_CONCRETE_FOREST: &str = "concrete-forest";
pub const PUZZLE5D_EXAMPLE_NAKAGIN: &str = "nakagin-capsule-tower";
pub const PUZZLE5D_EXAMPLE_CAPSULE_DREAM: &str = "capsule-dream";

pub const PUZZLE5D_FALLBACK_MESH_KIND: &str = "box";
/// 🧰️ Active utility fallback when the host `ViewModel` has not selected one yet.
pub const PUZZLE5D_DEFAULT_UTILITY: &str = "select";
/// 🎯️ The fill count a fresh 5d document offers. There is no ceiling to pair it with: the wrapped 3d
/// planner plans toward whatever the operator types and reports a stall reason when the document
/// cannot hold that many.
pub const PUZZLE5D_DEFAULT_FILL_COUNT: u32 = 100;
pub const PUZZLE5D_LOD_MODE_AUTOMATIC: &str = "automatic";
pub const PUZZLE5D_SUGGESTION_OFFSET_MIN: f64 = 0.0;
pub const PUZZLE5D_SUGGESTION_OFFSET_MAX: f64 = 160.0;
pub const PUZZLE5D_SUGGESTION_OFFSET_STEP: f64 = 4.0;
pub const PUZZLE5D_DEFAULT_SUGGESTION_OFFSET: f64 = 80.0;
pub const PUZZLE5D_DEFAULT_PART_RADIUS: f64 = 20.0;
pub const PUZZLE5D_BOARD_PLACEMENT_GAP: f64 = 16.0;
pub const PUZZLE5D_PROXIMITY_RADIUS: f64 = 0.75;
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the one interaction domain this app
/// declares — the previously-separate `Puzzle5dSelection` bags (part/grip/fastener) collapse into
/// one framework-owned domain, distinguished by `DomainSelection.granularity`.
pub const PUZZLE5D_INTERACTION_DOMAIN: &str = "vortex";
pub const PUZZLE5D_GRANULARITY_PART: &str = "part";
pub const PUZZLE5D_GRANULARITY_GRIP: &str = "grip";
pub const PUZZLE5D_GRANULARITY_FASTENER: &str = "fastener";
/// 🧊️ The granularity a target-volume row picks with — puzzle 3d's own `targetVolume` spelling, so the
/// two artifacts' outliners, inspectors and hosts read one vocabulary.
pub const PUZZLE5D_GRANULARITY_TARGET_VOLUME: &str = "targetVolume";
/// 🐁️ The pointer hover channel of the `vortex` domain — the one channel both panes paint from.
pub const PUZZLE5D_HOVER_CHANNEL: &str = "pointer";

/// 🤏️ Grip marker emission modes (`setGripShow`).
pub const PUZZLE5D_GRIP_SHOW_ALWAYS: &str = "always";
pub const PUZZLE5D_GRIP_SHOW_SELECTED: &str = "selected";
/// 🧭️ Grip direction arrow modes (`setGripDirection`).
pub const PUZZLE5D_GRIP_DIRECTION_OUTWARDS: &str = "outwards";
pub const PUZZLE5D_GRIP_DIRECTION_INWARDS: &str = "inwards";
/// 🔭️ The band the world pane's manual LOD slider — and `setLodManual` — clamps against.
pub const PUZZLE5D_LOD_SLIDER_MIN: f64 = 0.0;
pub const PUZZLE5D_LOD_SLIDER_MAX: f64 = 1000.0;
/// 🌐️ The band `setGridSpacing` clamps the world pane's grid pitch (m) into.
pub const PUZZLE5D_GRID_SPACING_MIN: f64 = 0.5;
pub const PUZZLE5D_GRID_SPACING_MAX: f64 = 50.0;
/// 🧊️ The band `setVoxelDims` clamps each Volume-Brush voxel axis (in grid-spacing units) into, and
/// the extent one Alt+click paints before the operator moves a slider.
pub const PUZZLE5D_VOXEL_DIM_MIN: f64 = 1.0;
pub const PUZZLE5D_VOXEL_DIM_MAX: f64 = 64.0;
pub const PUZZLE5D_DEFAULT_VOXEL_DIMS: [u32; 3] = [4, 4, 4];
/// 🎨️ The wire colour the world pane paints a target volume in — the same pink puzzle 3d uses, so an
/// operator reads one constraint the same way in either artifact.
pub const PUZZLE5D_TARGET_VOLUME_COLOR: &str = "#f472b6";
/// 🆔️ The id stem `addTargetVolume` mints from, shared with the part/fastener id counter.
pub const PUZZLE5D_TARGET_VOLUME_ID_STEM: &str = "target-volume";

/// 📐️ The band `setGridFactor` clamps the board pane's snap factor into.
pub const PUZZLE5D_GRID_FACTOR_MIN: f64 = 0.25;
pub const PUZZLE5D_GRID_FACTOR_MAX: f64 = 16.0;
/// 📡️ The ceiling `setProximityRadius` clamps the auto-connect distance (m) to — past this every part
/// in a normal document would be a candidate neighbour and a drop would connect the wrong grip.
pub const PUZZLE5D_PROXIMITY_RADIUS_MAX: f64 = 25.0;
/// 🧱️ The band `setChunkSize` clamps the broad-phase chunk edge (m) into. The floor is a real minimum:
/// a zero edge buckets the whole document into one cell and the placement search degenerates.
pub const PUZZLE5D_CHUNK_SIZE_MIN: f64 = 0.25;
pub const PUZZLE5D_CHUNK_SIZE_MAX: f64 = 512.0;

/// 🌉️ This app's own scratch fixture stays a local structural-twin mirror (`Puzzle5dDocument`) of
/// `crate::Puzzle5dSnapshot` — see that artifact's `🔖️ValueBridge` region — so
/// the DSL-text example fixtures are parsed once into the typed projection and re-serialized to the
/// JSON string this module's `document_from_json`/`.example(...)` call sites expect.
pub static CONCRETE_FOREST_EXAMPLE_JSON: LazyLock<String> = LazyLock::new(|| crate::examples::puzzle5d::concrete_forest::SOURCE.document_json().to_string());
pub static NAKAGIN_EXAMPLE_JSON: LazyLock<String> = LazyLock::new(|| crate::examples::puzzle5d::nakagin_capsule_tower::SOURCE.document_json().to_string());
pub static CAPSULE_DREAM_EXAMPLE_JSON: LazyLock<String> = LazyLock::new(|| crate::examples::puzzle5d::capsule_dream::SOURCE.document_json().to_string());
pub static CONCRETE_FOREST_EXAMPLE_DOCUMENT: LazyLock<Puzzle5dDocument> = LazyLock::new(|| document_from_json(CONCRETE_FOREST_EXAMPLE_JSON.as_str()));
pub static NAKAGIN_EXAMPLE_DOCUMENT: LazyLock<Puzzle5dDocument> = LazyLock::new(|| document_from_json(NAKAGIN_EXAMPLE_JSON.as_str()));
pub static CAPSULE_DREAM_EXAMPLE_DOCUMENT: LazyLock<Puzzle5dDocument> = LazyLock::new(|| document_from_json(CAPSULE_DREAM_EXAMPLE_JSON.as_str()));
pub static EMPTY_EXAMPLE_DOCUMENT: LazyLock<Puzzle5dDocument> = LazyLock::new(empty_document);


pub const PUZZLE5D_RESERVED_RAW_BYTES: usize = 65_536;
pub const PUZZLE5D_RESERVED_OUTPUT_BYTES: usize = 1_048_576;
pub const PUZZLE5D_RESERVED_PAGE_BYTES: usize = 4_096;
pub const PUZZLE5D_IMPORT_MEDIA_BYTES: usize = semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
pub const PUZZLE5D_IMPORT_SEMANTIC_ITEMS: usize = 32;
pub const PUZZLE5D_IMPORT_DECODED_ITEMS: usize = PUZZLE5D_IMPORT_SEMANTIC_ITEMS * PUZZLE5D_IMPORT_SEMANTIC_ITEMS + PUZZLE5D_IMPORT_SEMANTIC_ITEMS * 5;
pub const PUZZLE5D_IMPORT_MUTATION_ITEMS: usize = PUZZLE5D_IMPORT_SEMANTIC_ITEMS * 2 + 1;
pub const PUZZLE5D_IMPORT_MUTATIONS_PER_PAGE: usize = semio_framework_job::JOB_PAYLOAD_PAGE_BYTES / size_of::<Puzzle5dMutation>();
pub const PUZZLE5D_IMPORT_MUTATION_PAGES: usize = PUZZLE5D_IMPORT_MUTATION_ITEMS.div_ceil(PUZZLE5D_IMPORT_MUTATIONS_PER_PAGE);

pub fn puzzle5d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: PUZZLE5D_PLAY_CONTROLLER_ID.into(), action: action.into(), args: args.map(|value| semio_framework_pack_json::to_dsl_value(&value)) }
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: builds a framework `interactionSelect`
/// action targeting one `(granularity, id)` pair in the `vortex` domain — replaces the deleted
/// `setSelection` action builders every document tree row used to construct by hand.
pub fn puzzle5d_interaction_select(granularity: &str, id: &str) -> ActionDescriptor {
    let targets = semio_framework_pack_json::to_json_string(&vec![InteractionTarget { granularity: granularity.into(), id: id.into() }]);
    puzzle5d_action(INTERACTION_SELECT_ACTION_ID, Some(semio_framework_pack_json::json!({ "domainId": PUZZLE5D_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })))
}

#[derive(Clone, Debug, Default, semio_framework_value::RetireOwned)]
pub struct Puzzle5dFreshIds {
    occupied_parts: HashSet<String>,
    occupied_fasteners: HashSet<String>,
    occupied_target_volumes: HashSet<String>,
    pub part_cursor: u64,
    pub fastener_cursor: u64,
    target_volume_cursor: u64,
}

impl Puzzle5dFreshIds {
    pub fn from_document(document: &Puzzle5dDocument) -> Self {
        Self {
            occupied_parts: document.parts.iter().map(|part| part.id.clone()).collect(),
            occupied_fasteners: document.fasteners.iter().map(|fastener| fastener.id.clone()).collect(),
            occupied_target_volumes: document.target_volumes.iter().map(|volume| volume.id.clone()).collect(),
            ..Self::default()
        }
    }

    pub fn observe_part(&mut self, id: &str) {
        self.occupied_parts.insert(id.to_string());
    }

    pub fn observe_fastener(&mut self, id: &str) {
        self.occupied_fasteners.insert(id.to_string());
    }

    pub fn next_part(&mut self) -> String {
        next_scoped_id("part", &mut self.part_cursor, &mut self.occupied_parts)
    }

    pub fn next_fastener(&mut self) -> String {
        next_scoped_id("fastener", &mut self.fastener_cursor, &mut self.occupied_fasteners)
    }

    /// 🧊️ The next free `target-volume-<n>` id — minted off the document so a repainted volume never
    /// shadows one the operator already placed.
    pub fn next_target_volume(&mut self) -> String {
        next_scoped_id(PUZZLE5D_TARGET_VOLUME_ID_STEM, &mut self.target_volume_cursor, &mut self.occupied_target_volumes)
    }
}

fn next_scoped_id(prefix: &str, cursor: &mut u64, occupied: &mut HashSet<String>) -> String {
    loop {
        *cursor = cursor.saturating_add(1);
        let candidate = format!("{prefix}-{cursor}");
        if occupied.insert(candidate.clone()) {
            return candidate;
        }
    }
}
//#endregion 🔖️Constants

//#region 🔖️Document
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dGrip2d {
    #[value(default)]
    pub angle: f64,
    #[value(default, rename = "gripKind")]
    pub grip_kind: String,
    #[value(default)]
    pub radius: f64,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dGrip3d {
    #[value(default)]
    pub position: [f64; 3],
    #[value(default)]
    pub direction: Option<[f64; 3]>,
    #[value(default)]
    pub radius: f64,
    #[value(default)]
    pub label: Option<String>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dGrip {
    pub id: String,
    #[value(default, rename = "gripKind")]
    pub grip_kind: String,
    #[value(default, rename = "2d")]
    pub grip_2d: Puzzle5dGrip2d,
    #[value(default, rename = "3d")]
    pub grip_3d: Puzzle5dGrip3d,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "lowercase")]
pub enum Puzzle5dPartAnchor {
    #[default]
    Fixed,
    Derived,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dFastener {
    pub id: String,
    pub source: String,
    pub target: String,
    #[value(default, rename = "fastenerKind", skip_serializing_if = "Option::is_none")]
    pub fastener_kind: Option<String>,
    #[value(default)]
    pub gap: f64,
    #[value(default)]
    pub shift: f64,
    #[value(default)]
    pub rise: f64,
    #[value(default)]
    pub rotation: f64,
    #[value(default)]
    pub turn: f64,
    #[value(default)]
    pub tilt: f64,
    #[value(default)]
    pub x: f64,
    #[value(default)]
    pub y: f64,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dPart2d {
    #[value(default)]
    pub x: f64,
    #[value(default)]
    pub y: f64,
    #[value(default)]
    pub shape: String,
    #[value(default)]
    pub radius: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[value(default)]
    pub text: String,
    #[value(default, rename = "iconKind", skip_serializing_if = "Option::is_none")]
    pub icon_kind: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dPart3d {
    #[value(default)]
    pub origin: [f64; 3],
    #[value(default, rename = "meshUrl")]
    pub mesh_url: Option<String>,
    #[value(default)]
    pub orientation: Option<[f64; 4]>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<Value>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dPart {
    pub id: String,
    #[value(default, rename = "partKind", skip_serializing_if = "String::is_empty")]
    pub part_kind: String,
    #[value(default)]
    pub anchor: Puzzle5dPartAnchor,
    #[value(default, rename = "2d")]
    pub part_2d: Puzzle5dPart2d,
    #[value(default, rename = "3d")]
    pub part_3d: Puzzle5dPart3d,
    #[value(default)]
    pub grips: Vec<Puzzle5dGrip>,
}

/// 🧊️ One oriented box constraining where the fill planner may place, in the document's 3D pose
/// space. The Volume Brush paints grid-snapped axis-aligned instances sized by the world window's
/// voxel dims; the transform gumball edits arbitrary oriented boxes through `relocateTargetVolume`.
/// The board pane paints the flat rectangle this box projects to, never a second persisted pose.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dTargetVolume {
    pub id: String,
    #[value(default)]
    pub origin: [f64; 3],
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<[f64; 4]>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<Value>,
    #[value(default)]
    pub hidden: bool,
    #[value(default)]
    pub locked: bool,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dDocument {
    pub schema: String,
    #[value(default)]
    pub domain: String,
    #[value(default)]
    pub parts: Vec<Puzzle5dPart>,
    #[value(default)]
    pub fasteners: Vec<Puzzle5dFastener>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub target_volumes: Vec<Puzzle5dTargetVolume>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Value>,
    #[value(default, rename = "kindCatalogs", skip_serializing_if = "Option::is_none")]
    pub kind_catalogs: Option<Value>,
    #[value(default, rename = "kindCompatibility", skip_serializing_if = "Option::is_none")]
    pub kind_compatibility: Option<Value>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

pub fn empty_document() -> Puzzle5dDocument {
    Puzzle5dDocument { schema: PUZZLE5D_SCHEMA.into(), domain: "architecture".into(), parts: Vec::new(), fasteners: Vec::new(), target_volumes: Vec::new(), meta: None, kind_catalogs: None, kind_compatibility: None, label: None }
}

/// 📥️ Decodes a SHIPPED example's JSON into the editor twin. A failure here is a build defect in the
/// example asset, never user input, so it is loud: the silent `empty_document()` fallback this used to
/// carry is what let the 2026-09-17 example regression ship a zero-part Nakagin and Capsule Dream.
/// User-supplied JSON arrives through `📥️import-snapshot`, which refuses with a notice instead.
pub fn document_from_json(json_text: &str) -> Puzzle5dDocument {
    semio_framework_pack_json::from_json_str::<Puzzle5dDocument>(json_text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("puzzle5d example document decodes: {error}"))
}

pub fn concrete_forest_example_document() -> Puzzle5dDocument {
    CONCRETE_FOREST_EXAMPLE_DOCUMENT.clone()
}

pub fn nakagin_example_document() -> Puzzle5dDocument {
    NAKAGIN_EXAMPLE_DOCUMENT.clone()
}

pub fn capsule_dream_example_document() -> Puzzle5dDocument {
    CAPSULE_DREAM_EXAMPLE_DOCUMENT.clone()
}

pub fn default_document() -> Puzzle5dDocument {
    concrete_forest_example_document()
}

/// 🔎️ Reads a fixed-length `[f64; 3]` coordinate triple straight off a dsl `Value::Array` —
/// the direct replacement for the old `puzzle5d_record_from_projection::<[f64; 3]>(value.clone())` round
/// trip, which this file's own `Value` no longer supports (no `Deserialize`).
pub fn puzzle5d_value_as_f64_3(value: &Value) -> Option<[f64; 3]> {
    let values = value.as_array()?;
    Some([values.first()?.as_f64()?, values.get(1)?.as_f64()?, values.get(2)?.as_f64()?])
}

/// 🔎️ The `[f64; 4]` sibling of [`puzzle5d_value_as_f64_3`] (quaternion orientation reads).
pub fn puzzle5d_value_as_f64_4(value: &Value) -> Option<[f64; 4]> {
    let values = value.as_array()?;
    Some([values.first()?.as_f64()?, values.get(1)?.as_f64()?, values.get(2)?.as_f64()?, values.get(3)?.as_f64()?])
}

/// 🌉️ Bridges `Puzzle5dPlaySnapshot::value()` — the lazily materialized legacy `Value`
/// projection of the typed snapshot — into this file's own first-party `Value` via `DslValue` without ever
/// naming the foreign crate: `T`'s only real caller is `snapshot.value(): &Value`, resolved
/// structurally through the `DslValue: From<T>` bound — mirrors `🧊️3d/…/✏️editor/🦀️.rs`'s
/// `puzzle3d_projection_value`.
fn puzzle5d_projection_value<T:std::borrow::Borrow<semio_framework_value::DslValue>>(value:T)->Value{
    semio_framework_pack_json::from_dsl_value(value.borrow())
}

pub fn puzzle5d_editor_projection(snapshot: &Puzzle5dPlaySnapshot) -> Value {
    value_from_document(&puzzle5d_document_from_snapshot(snapshot.typed()).expect("admitted typed snapshot has a host projection"))
}

/// 🌱️ Admits a native host projection through the owned record contract.
pub fn puzzle5d_record_from_projection<T: FromValue>(value: Value) -> Result<T, semio_framework_value::ValueError> {
    T::from_value(semio_framework_pack_json::to_dsl_value(&value))
}

pub fn puzzle5d_paste_placement(args: &Value) -> Result<PastePlacement, semio_framework_value::ValueError> {
    let refusal = || semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "puzzle5d paste placement is malformed");
    let anchor = match args.get("anchor").map(|value| value.as_str().ok_or_else(refusal)).transpose()?.unwrap_or("original") {
        "original" => PasteAnchor::Original, "middle" => PasteAnchor::Middle, "centroid" => PasteAnchor::Centroid,
        "bottomLeft" => PasteAnchor::BottomLeft, "bottomRight" => PasteAnchor::BottomRight,
        "topLeft" => PasteAnchor::TopLeft, "topRight" => PasteAnchor::TopRight,
        _ => return Err(refusal()),
    };
    let position = args.get("position").filter(|value| !value.is_null()).map(|value| {
        let values = value.as_array().ok_or_else(refusal)?;
        if values.len() != 3 { return Err(refusal()); }
        Ok([values[0].as_f64().ok_or_else(refusal)?, values[1].as_f64().ok_or_else(refusal)?, values[2].as_f64().ok_or_else(refusal)?])
    }).transpose()?;
    Ok(PastePlacement { anchor, position })
}

/// 🧬️ Admits each host entity and splits catalogue custody into the typed child reference.
pub fn puzzle5d_snapshot_from_document(document: &Puzzle5dDocument) -> Result<Puzzle5dSnapshot, semio_framework_value::ValueError> {
    if document.schema != PUZZLE5D_SCHEMA {
        return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "puzzle5d document schema is not admitted"));
    }
    let catalogs = document.kind_catalogs.as_ref().map(|value| crate::standards::v1::subsets::any::schema::document_records::Puzzle5dKindCatalogs::from_value(value.to_value())).transpose()?;
    let (kind_catalogs, kind_catalogs_extra) = crate::standards::v1::subsets::any::schema::kind_catalogs::split_and_seed_kind_catalogs(catalogs);
    Ok(Puzzle5dSnapshot {
        schema: document.schema.clone(), domain: document.domain.clone(), label: document.label.clone(),
        meta: document.meta.as_ref().map(|value| crate::standards::v1::subsets::any::schema::document_records::Puzzle5dMeta::from_value(value.to_value())).transpose()?.unwrap_or_default(),
        kind_catalogs, kind_catalogs_extra,
        kind_compatibility: document.kind_compatibility.as_ref().map(|value| Vec::<crate::standards::v1::subsets::any::schema::document_records::Puzzle5dKindCompatibility>::from_value(value.to_value())).transpose()?.unwrap_or_default(),
        parts: document.parts.iter().map(|part| crate::standards::v1::subsets::any::schema::document_records::Puzzle5dPart::from_value(part.to_value())).collect::<Result<_, _>>()?,
        fasteners: document.fasteners.iter().map(|fastener| crate::standards::v1::subsets::any::schema::document_records::Puzzle5dFastener::from_value(fastener.to_value())).collect::<Result<_, _>>()?,
        target_volumes: document.target_volumes.iter().map(|volume| crate::standards::v1::subsets::any::schema::document_records::Puzzle5dTargetVolume::from_value(volume.to_value())).collect::<Result<_, _>>()?,
    })
}

/// 👁️ Projects one admitted snapshot for the native editor without decoding an external representation.
pub fn puzzle5d_document_from_snapshot(snapshot: &Puzzle5dSnapshot) -> Result<Puzzle5dDocument, semio_framework_value::ValueError> {
    let mut value = semio_framework_pack_json::from_dsl_value(&snapshot.to_value());
    if let Some(object) = value.as_object_mut() {
        match crate::standards::v1::subsets::any::schema::kind_catalogs::kind_catalogs_of(&snapshot.kind_catalogs, &snapshot.kind_catalogs_extra) {
            Some(catalogs) => { object.insert("kindCatalogs", semio_framework_pack_json::from_dsl_value(&catalogs.to_value())); }
            None => { object.remove("kindCatalogs"); }
        }
    }
    puzzle5d_record_from_projection(value)
}

fn value_from_document(document: &Puzzle5dDocument) -> Value {
    semio_framework_pack_json::from_dsl_value(&document.to_value())
}

/// 📋️ The concrete kinds one materialized paste consists of: a `create-part` per fresh part, then a `connect-grips`
/// per fresh fastener (whose endpoints already name the fresh parts).
pub fn puzzle5d_paste_mutations(parts: Vec<Puzzle5dPart>, fasteners: Vec<Puzzle5dFastener>) -> Result<Vec<Puzzle5dMutation>, semio_framework_value::ValueError> {
    let mut mutations = Vec::with_capacity(parts.len() + fasteners.len());
    for part in parts {
        let typed = <crate::standards::v1::subsets::any::schema::document_records::Puzzle5dPart as FromValue>::from_value(ToValue::to_value(&part))?;
        mutations.push(crate::standards::v1::subsets::any::schema::mutations::create_part(typed, None));
    }
    for fastener in fasteners {
        mutations.push(crate::standards::v1::subsets::any::schema::mutations::connect_grips(fastener.id, fastener.source, fastener.target, fastener.fastener_kind, fastener.gap, fastener.shift, fastener.rise, fastener.rotation, fastener.turn, fastener.tilt, fastener.x, fastener.y, None));
    }
    Ok(mutations)
}

/// 🪟️ B1: puzzle5d has exactly two window KINDS (2D and 3D), each single-instance — unlike puzzle3d's
/// split top/perspective panes (two INSTANCES of one kind), puzzle5d's own dispatch never distinguishes
/// a window instance id from its kind id (every action matches the literal kind id via
/// `PUZZLE5D_PLAY_WINDOWS.contains(&window)`), so this needs none of `Puzzle3dConfig`'s self-maintained
/// `window_ids`/`load_window`/`save_window` machinery — each kind's sole instance id is the kind id
/// itself. Kept as a named helper (rather than inlining `vec![kind_id.to_string()]`) purely so
/// `window_engagements`/`window_measures` read the same "one entry per live window instance" shape
/// `ArtifactApp`'s doc comment describes, and so a future genuine multi-instance need has one seam to extend.
pub fn window_instance_ids(kind_id: &str) -> Vec<String> {
    vec![kind_id.to_string()]
}

pub fn puzzle5d_grip_full_id(part_id: &str, grip_id: &str) -> String {
    if grip_id.contains(':') {
        grip_id.to_string()
    } else {
        format!("{part_id}:{grip_id}")
    }
}

/// 📐️ Resolves one numeric-field edit: an absolute `value` (typed entry) wins when present,
/// otherwise a `delta` (stepper nudge) is added to `current`. `None` when neither parses.
pub fn puzzle5d_resolve_number_edit(current: f64, value: Option<&Value>, delta: Option<&Value>) -> Option<f64> {
    if let Some(absolute) = value.and_then(Value::as_f64) {
        return Some(absolute);
    }
    delta.and_then(Value::as_f64).map(|delta| current + delta)
}

/// 📏️ Settings counterpart to [`puzzle5d_resolve_number_edit`]: reads `value`/`delta` straight out of
/// an action's `args`, for the single global/window settings whose slider or stepper dispatches to its
/// own dedicated action (puzzle 3d's `puzzle3d_absolute_or_delta`).
pub fn puzzle5d_absolute_or_delta(args: Option<&Value>, current: f64) -> Option<f64> {
    puzzle5d_resolve_number_edit(current, args.and_then(|value| value.get("value")), args.and_then(|value| value.get("delta")))
}

/// 📐️ Parses a nested stepper-group field id as `"<base>.<axis>"` (`x`/`y`/`z`), returning the axis
/// index when `field` names a component of `base` — the dot-path convention `ui_inspector_vec3_group`
/// uses for its per-axis actions.
pub fn puzzle5d_axis_index(field: &str, base: &str) -> Option<usize> {
    match field.strip_prefix(base)?.strip_prefix('.')? {
        "x" => Some(0),
        "y" => Some(1),
        "z" => Some(2),
        _ => None,
    }
}


pub fn resolve_part_mesh_url(part: &Puzzle5dPart, kind_catalogs: Option<&Value>) -> Option<String> {
    if let Some(url) = part.part_3d.mesh_url.as_ref().filter(|url| !url.is_empty()) {
        return Some(url.clone());
    }
    resolve_part_kind_mesh_url(&part.part_kind, kind_catalogs)
}

pub fn resolve_part_kind_mesh_url(part_kind: &str, kind_catalogs: Option<&Value>) -> Option<String> {
    let parts = kind_catalogs?.get("parts")?.as_array()?;
    parts.iter().find(|entry| entry.get("id").and_then(|v| v.as_str()) == Some(part_kind)).and_then(|entry| entry.get("meshUrl").and_then(|v| v.as_str()).map(str::to_string))
}

pub fn collect_mesh_urls(document: &Puzzle5dDocument) -> Vec<String> {
    let mut urls = HashSet::new();
    for part in &document.parts {
        if let Some(url) = resolve_part_mesh_url(part, document.kind_catalogs.as_ref()) {
            urls.insert(url);
        }
    }
    if let Some(parts) = document.kind_catalogs.as_ref().and_then(|catalogs| catalogs.get("parts")).and_then(|v| v.as_array()) {
        for entry in parts {
            if let Some(url) = entry.get("meshUrl").and_then(|v| v.as_str()) {
                urls.insert(url.to_string());
            }
        }
    }
    urls.into_iter().collect()
}

fn part_kind_grip_templates(document: &Puzzle5dDocument, part_kind: &str) -> Vec<Value> {
    document
        .kind_catalogs
        .as_ref()
        .and_then(|catalogs| catalogs.get("parts"))
        .and_then(|parts| parts.as_array())
        .and_then(|parts| parts.iter().find(|entry| entry.get("id").and_then(|v| v.as_str()) == Some(part_kind)))
        .and_then(|entry| entry.get("grips"))
        .and_then(|grips| grips.as_array())
        .cloned()
        .unwrap_or_default()
}

pub fn grips_from_templates(document: &Puzzle5dDocument, part_kind: &str) -> Vec<Puzzle5dGrip> {
    part_kind_grip_templates(document, part_kind)
        .iter()
        .enumerate()
        .map(|(index, template)| {
            let grip_kind = template.get("gripKind").and_then(|v| v.as_str()).unwrap_or("grip").to_string();
            let grip_2d: Puzzle5dGrip2d = template.get("2d").and_then(|v| puzzle5d_record_from_projection(v.clone()).ok()).unwrap_or_default();
            let grip_3d: Puzzle5dGrip3d = template.get("3d").and_then(|v| puzzle5d_record_from_projection(v.clone()).ok()).unwrap_or_default();
            Puzzle5dGrip { id: format!("v{index}"), grip_kind, grip_2d, grip_3d }
        })
        .collect()
}

pub use crate::standards::v1::subsets::any::schema::mutations::{quat_from_axis_angle,quat_mul};


pub fn quat_rotate_vector(quat: [f64; 4], vector: [f64; 3]) -> [f64; 3] {
    let [x, y, z, w] = quat;
    let vx = vector[0];
    let vy = vector[1];
    let vz = vector[2];
    let ix = w * vx + y * vz - z * vy;
    let iy = w * vy + z * vx - x * vz;
    let iz = w * vz + x * vy - y * vx;
    let iw = -x * vx - y * vy - z * vz;
    [ix * w + iw * -x + iy * -z - iz * -y, iy * w + iw * -y + iz * -x - ix * -z, iz * w + iw * -z + ix * -y - iy * -x]
}

pub fn world_grip_position(part: &Puzzle5dPart, grip: &Puzzle5dGrip) -> [f64; 3] {
    let orientation = part.part_3d.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
    let rotated = quat_rotate_vector(orientation, grip.grip_3d.position);
    [part.part_3d.origin[0] + rotated[0], part.part_3d.origin[1] + rotated[1], part.part_3d.origin[2] + rotated[2]]
}

pub fn world_grip_direction(part: &Puzzle5dPart, grip: &Puzzle5dGrip) -> [f64; 3] {
    let direction = grip.grip_3d.direction.unwrap_or([0.0, 0.0, -1.0]);
    quat_rotate_vector(part.part_3d.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), direction)
}

pub fn resolve_grip_world_position(document: &Puzzle5dDocument, full_id: &str) -> Option<[f64; 3]> {
    for part in &document.parts {
        for grip in &part.grips {
            if puzzle5d_grip_full_id(&part.id, &grip.id) == full_id {
                return Some(world_grip_position(part, grip));
            }
        }
    }
    None
}

pub fn find_part_by_grip_full_id<'a>(document: &'a Puzzle5dDocument, full_id: &str) -> Option<(&'a Puzzle5dPart, &'a Puzzle5dGrip)> {
    for part in &document.parts {
        for grip in &part.grips {
            if puzzle5d_grip_full_id(&part.id, &grip.id) == full_id {
                return Some((part, grip));
            }
        }
    }
    None
}

pub fn mesh_selection_ids(args: Option<&Value>, fallback: &[String]) -> Vec<String> {
    args.and_then(|value| value.get("ids")).and_then(|value| <Vec<String> as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(value)).ok()).filter(|ids| !ids.is_empty()).unwrap_or_else(|| fallback.to_vec())
}

pub fn part_scale_json(part: &Puzzle5dPart) -> [f64; 3] {
    match &part.part_3d.scale {
        Some(Value::Array(values)) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0), values[1].as_f64().unwrap_or(1.0), values[2].as_f64().unwrap_or(1.0)],
        Some(Value::Number(value)) => {
            let factor = value.as_f64();
            [factor, factor, factor]
        }
        _ => [1.0, 1.0, 1.0],
    }
}

/// 🧊️ One target volume's world extent as an `[x, y, z]` triple — the `Puzzle5dScale` union read the
/// same way `part_scale_json` reads a part's, so a volume painted uniformly and one scaled per axis
/// both reach the world host as three numbers.
pub fn target_volume_scale_json(volume: &Puzzle5dTargetVolume) -> [f64; 3] {
    match &volume.scale {
        Some(Value::Array(values)) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0), values[1].as_f64().unwrap_or(1.0), values[2].as_f64().unwrap_or(1.0)],
        Some(Value::Number(value)) => {
            let factor = value.as_f64();
            [factor, factor, factor]
        }
        _ => [1.0, 1.0, 1.0],
    }
}

/// 📐️ The flat rectangle a target volume constrains on the board — `[x, y, width, height]` in board
/// units, centred on the volume's own centre. The projection is the SAME linear board↔world map
/// `add_palette_part` and `🧬️schema/💡️inferences/🎛️flat-position` place paired parts with
/// ([`PUZZLE5D_FLAT_TO_WORLD`], world XY ground plane, board Y pointing the other way), so a part the
/// planner placed inside the box lands inside the painted rectangle — never a second persisted pose.
pub fn target_volume_flat_rect(volume: &Puzzle5dTargetVolume) -> [f64; 4] {
    let extent = target_volume_scale_json(volume);
    let to_flat = 1.0 / PUZZLE5D_FLAT_TO_WORLD;
    [volume.origin[0] * to_flat, -volume.origin[1] * to_flat, extent[0].abs() * to_flat, extent[1].abs() * to_flat]
}

pub use crate::standards::v1::subsets::any::schema::mutations::PUZZLE5D_FLAT_TO_WORLD;

//#endregion 🔖️Document
