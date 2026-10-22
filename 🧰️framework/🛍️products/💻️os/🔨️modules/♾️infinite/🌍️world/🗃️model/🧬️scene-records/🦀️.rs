//! 🧬️ World3d scene records: lod, chunking, targets, references, pick targets, environment and terrain style.

use super::*;

//#region SceneRecords

fn dsl_id_to_string(value: &semio_framework::DslValue) -> Option<String> {
    value.as_str().map(str::to_string).or_else(|| value.as_f64().map(|n| if n.fract() == 0.0 { format!("{}", n as u64) } else { n.to_string() }))
}

fn dsl_string_vec(value: &semio_framework::DslValue) -> Vec<String> {
    value.as_array().map(|items| items.iter().filter_map(dsl_id_to_string).collect()).unwrap_or_default()
}

#[derive(Clone, Debug, Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
#[cfg(any(test, feature = "testkit"))]
pub struct WorldMeshLodEntry {
    pub lod: f64,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WorldLodRecord {
    #[serde(default = "default_true")]
    #[value(default = "default_true")]
    pub automatic: bool,
    #[serde(default = "default_manual_lod")]
    #[value(default = "default_manual_lod")]
    pub manual: f64,
    #[serde(default = "default_distance_reference")]
    #[value(default = "default_distance_reference")]
    pub distance_reference: f64,
    #[serde(default)]
    #[value(default)]
    pub depth_variable: bool,
    #[serde(default = "default_grid_factor")]
    #[value(default = "default_grid_factor")]
    pub grid_factor: f64,
    #[serde(default)]
    #[value(default)]
    pub grid_snap_enabled: bool,
    #[serde(default = "default_true")]
    #[value(default = "default_true")]
    pub show_grid: bool,
    #[serde(default)]
    #[value(default)]
    pub grid_datum: Option<[f64; 3]>,
}

impl Default for WorldLodRecord {
    fn default() -> Self {
        default_lod_record()
    }
}

#[derive(Clone, Debug, Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WorldChunkingRecord {
    pub chunk_size: f64,
    pub max_distance: f64,
}

pub fn default_manual_lod() -> f64 {
    100.0
}

pub fn default_distance_reference() -> f64 {
    100.0
}

pub fn default_grid_factor() -> f64 {
    10.0
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WorldSelectionTargets {
    #[serde(default)]
    #[value(default)]
    pub vertex: bool,
    #[serde(default)]
    #[value(default)]
    pub edge: bool,
    #[serde(default)]
    #[value(default)]
    pub face: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize, Default, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WorldVortexRecord {
    pub full_id: String,
    #[value(default)]
    pub position: Option<[f64; 3]>,
    #[value(default)]
    pub direction: Option<[f64; 3]>,
    #[value(default)]
    pub display_direction: Option<String>,
    #[value(default)]
    pub radius: Option<f64>,
    #[value(default)]
    pub color: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Default, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WorldAttractionRecord {
    #[value(default)]
    pub id: Option<String>,
    #[value(default)]
    pub from: Option<[f64; 3]>,
    #[value(default)]
    pub to: Option<[f64; 3]>,
    #[value(default)]
    pub color: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorldTutorialGeometry {
    pub point: [f32; 2],
    pub polyline: Option<Vec<[f32; 2]>>,
}

#[derive(Clone, Debug, Deserialize, Default, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WorldTargetVolumeRecord {
    #[value(default)]
    pub origin: Option<[f64; 3]>,
    #[value(default)]
    pub orientation: Option<[f64; 4]>,
    #[value(default)]
    pub scale: Option<[f64; 3]>,
    #[value(default)]
    pub color: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Default, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WorldReferenceRecord {
    #[value(default)]
    pub id: Option<String>,
    #[value(default)]
    pub url: Option<String>,
    #[value(default)]
    pub origin: Option<[f64; 3]>,
    #[value(default)]
    pub width_world: Option<f64>,
    #[value(default)]
    pub locked: Option<bool>,
    #[value(default)]
    pub opacity: Option<f64>,
    #[value(default)]
    pub hidden: Option<bool>,
}

/// 🖼️ React's authored fallback for a reference without `widthWorld`.
const WORLD_REFERENCE_DEFAULT_WIDTH: f32 = 10.0;

pub fn world_reference_width(reference: &WorldReferenceRecord) -> f32 {
    reference.width_world.map(|width| width as f32).unwrap_or(WORLD_REFERENCE_DEFAULT_WIDTH)
}

/// 🤝️ One item of the `engagementPreview` lane — the live construction feedback a plugin's
/// interaction statechart publishes for the state it is in (`cad`'s `preview_display_items`). Same
/// four wire kinds React's `World3dHost` parses (`WorldEngagementPreviewItem`, `🟦️.tsx:344-372`):
/// `point`, `segment`, `box-preview`, `linear-handle`.
///
/// 🩸️ The lane had NO wgpu reader at all before ticket 26/09/17 packet W2f, so every rubber-band
/// point, segment, footprint box and height handle of every plugin construction interaction was
/// invisible on this target while React drew them.
#[derive(Clone, Debug, Deserialize, Default, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WorldEngagementPreviewRecord {
    #[value(default)]
    pub kind: Option<String>,
    #[value(default)]
    pub role: Option<String>,
    #[value(default)]
    pub position: Option<[f64; 3]>,
    #[value(default)]
    pub from: Option<[f64; 3]>,
    #[value(default)]
    pub to: Option<[f64; 3]>,
    #[value(default)]
    pub corner_a: Option<[f64; 3]>,
    #[value(default)]
    pub corner_b: Option<[f64; 3]>,
    #[value(default)]
    pub height: Option<f64>,
    #[value(default)]
    pub axis: Option<[f64; 3]>,
    #[value(default)]
    pub origin: Option<[f64; 3]>,
}

//#region 🧲️PickTargets
/// 🧲️ How many `pick_targets_json` rows one surface RETAINS. The lane is bounded on both ends —
/// the producer caps what it publishes (`CAD_PICK_OVERLAY_ITEM_BUDGET`) and this caps what a
/// consumer keeps — so a Concrete-Forest pane with thousands of kernel members can never grow an
/// unbounded `Vec` behind a pointer move.
pub const WORLD_PICK_TARGET_CAPACITY: usize = 256;

/// 🧲️ How many points ONE retained pick target keeps. A sampled arc/circle/ellipse edge is 32-64
/// samples on React's side (`edgeSamplePoints`), and the bounds this host hit-tests are unchanged
/// by dropping the tail of a longer run.
pub const WORLD_PICK_TARGET_POINT_CAPACITY: usize = 64;

/// 🧲️ React's `targetRayScore` box padding — `box.expandByScalar(kind === "vertex" ? 0.12 : 0.08)`
/// (`✏️editor/⚙️engine/📺️renderer/🟦️.tsx:2301`). World units, not pixels: the CAD editor's own
/// pick proxies are world-sized spheres/boxes, so this host uses the same numbers rather than a
/// screen-space radius that would disagree with React at every zoom.
const WORLD_PICK_TARGET_VERTEX_PAD: f32 = 0.12;

const WORLD_PICK_TARGET_PAD: f32 = 0.08;

/// 🧲️ React's `spatialPickPriority * 1e-4` tie-break (`🟦️.tsx:2288-2302`) — at equal ray distance
/// the FINEST kind wins (vertex 0 → edge 1 → face 2 → object 3), which is why a click on a corner
/// of a solid selects the vertex and not the solid.
const WORLD_PICK_TARGET_PRIORITY_EPSILON: f32 = 1e-4;

/// 🎨️ One `pick_targets_json` row's resolved highlight material — the wire half of the CAD picking
/// engine's `SpatialTargetStyle`. `color`/`emissive` are theme TOKEN ids (`--foreground`,
/// `--color-changed-selected`, …) or a `#rrggbb` literal, never a resolved colour: the token is
/// what survives an appearance switch.
#[derive(Clone, Debug, Deserialize, Default, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WorldPickTargetStyleRecord {
    #[value(default)]
    pub color: Option<String>,
    #[value(default)]
    pub emissive: Option<String>,
    #[value(default)]
    pub opacity: Option<f64>,
    #[value(default)]
    line_width: Option<f64>,
}

/// 🧲️ One sub-object pick target — the wire half of the CAD picking engine's `SpatialPickTarget`
/// (`✏️editor/⚙️engine/🧲️picking/🏷️pick-vocabulary/🦀️.rs`).
///
/// 🩸️ Before ticket 26/09/17 packet W14g this surface had NO pick-target lane: the overlay rode
/// `engagementPreview`, a paint-only lane, so sub-object picking was shown and never hit-tested and
/// every click still resolved to the whole instance through the domain's `object` granularity.
#[derive(Clone, Debug, Deserialize, Default, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WorldPickTargetRecord {
    #[value(default)]
    pub kind: String,
    #[value(default)]
    pub id: String,
    #[value(default)]
    pub point: Option<[f64; 3]>,
    #[value(default)]
    pub points: Vec<[f64; 3]>,
    #[value(default)]
    typology: Option<String>,
    /// 👁️ `false` for a hidden or locked entity — it still PAINTS (dimmed, through `style`) and is
    /// never hit-tested, exactly as React's `SpatialPickGeometryLayer` renders such a target through
    /// `SpatialPickTargetNode` while `selectableTargets` leaves out its `SpatialPickHitTarget`.
    #[value(default)]
    selectable: Option<bool>,
    #[value(default)]
    pub style: Option<WorldPickTargetStyleRecord>,
}

impl WorldPickTargetRecord {
    /// 🪪️ `kind:id`, the key every hover/selection comparison is made on — the twin of the CAD
    /// engine's `spatial_pick_target_key`.
    pub fn key(&self) -> String {
        format!("{}:{}", self.kind, self.id)
    }

    /// 👁️ See [`WorldPickTargetRecord::selectable`].
    pub fn is_selectable(&self) -> bool {
        self.selectable.unwrap_or(true)
    }
}

/// 🧲️ React's `spatialPickPriority` (`📺️renderer/🟦️.tsx:2288`). An unknown kind sorts LAST so a
/// producer that grows a fifth kind degrades to "coarsest", never to "wins every tie".
fn world_pick_target_priority(kind: &str) -> u8 {
    match kind {
        "vertex" => 0,
        "edge" => 1,
        "face" => 2,
        "object" => 3,
        _ => 4,
    }
}

/// 🧲️ The padded world AABB React's `targetRayScore` raycasts — `points` when the row carries any,
/// otherwise the single `point`. `None` for a row that carries neither.
pub fn world_pick_target_bounds(target: &WorldPickTargetRecord) -> Option<([f32; 3], [f32; 3])> {
    let fallback = [target.point?];
    let points: &[[f64; 3]] = if target.points.is_empty() { &fallback } else { &target.points };
    let mut min = [f32::MAX; 3];
    let mut max = [f32::MIN; 3];
    for point in points {
        for axis in 0..3 {
            min[axis] = min[axis].min(point[axis] as f32);
            max[axis] = max[axis].max(point[axis] as f32);
        }
    }
    let pad = if target.kind == "vertex" { WORLD_PICK_TARGET_VERTEX_PAD } else { WORLD_PICK_TARGET_PAD };
    for axis in 0..3 {
        min[axis] -= pad;
        max[axis] += pad;
    }
    Some((min, max))
}

/// 🧲️ React's `targetRayScore` — ray distance to the padded box plus the kind's tie-break weight.
/// Lower wins, and the same number is comparable with a mesh hit's own ray distance, which is what
/// lets a pick target and the instance under it compete in ONE ordering (r3f raycasts both in the
/// same scene).
pub fn world_pick_target_ray_score(target: &WorldPickTargetRecord, origin: Vec3, direction: Vec3) -> Option<f32> {
    let (min, max) = world_pick_target_bounds(target)?;
    let distance = ray_aabb_slab(origin, direction, min, max)?;
    Some(distance + f32::from(world_pick_target_priority(&target.kind)) * WORLD_PICK_TARGET_PRIORITY_EPSILON)
}

/// 🪪️ The CAD engine's `spatial_hover_key_aliases` as the framework sees it: a kernel member key
/// (`solid:`/`shell:`/`wire:`/`anchor:`) and the coarser pick key that represents it
/// (`object:`/`face:`/`edge:`/`vertex:`) name the SAME thing, so a guest that echoes one spelling
/// must still match a host that dispatched the other.
fn world_pick_key_alias(key: &str) -> Option<String> {
    let (kind, id) = key.split_once(':')?;
    let mapped = match kind {
        "vertex" | "anchor" => "vertex",
        "edge" | "wire" => "edge",
        "face" | "shell" => "face",
        "solid" => "object",
        "object" => "solid",
        _ => return None,
    };
    Some(format!("{mapped}:{id}"))
}

/// 🪪️ True when two hover/selection keys name the same pick target, aliases included.
pub fn world_pick_keys_match(left: Option<&str>, right: Option<&str>) -> bool {
    let (Some(left), Some(right)) = (left.filter(|key| !key.is_empty()), right.filter(|key| !key.is_empty())) else {
        return false;
    };
    left == right || world_pick_key_alias(left).as_deref() == Some(right) || world_pick_key_alias(right).as_deref() == Some(left)
}

/// 🚚️ Parses the lane into the bounded retained set — every cap applied here, never at paint time.
pub fn world_pick_targets_from_json(json: &str) -> Vec<WorldPickTargetRecord> {
    let mut targets: Vec<WorldPickTargetRecord> = serde_json::from_str(json).unwrap_or_default();
    targets.truncate(WORLD_PICK_TARGET_CAPACITY);
    for target in &mut targets {
        target.points.truncate(WORLD_PICK_TARGET_POINT_CAPACITY);
    }
    targets
}
//#endregion 🧲️PickTargets

#[derive(Clone, Debug)]
pub struct WorldCatalogueDropPreviewRecord {
    pub object_kind: String,
    pub mesh_url: Option<String>,
    pub origin: [f64; 3],
}

/// 🌉️ Hand-written, not derived: `scale: Option<serde_json::Value>` has no `ToValue`/`FromValue`
/// for `serde_json::Value` (only the `DslValue <-> serde_json::Value` `From` bridges in
/// `🌱️value/🦀️.rs` exist) — same reason as `semio_framework_graph_manifest::KindDef`.
#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WorldBrushPreviewRecord {
    pub mesh_url: Option<String>,
    pub origin: Option<[f64; 3]>,
    pub orientation: Option<[f64; 4]>,
    pub scale: Option<serde_json::Value>,
    pub target_vortex_full_id: Option<String>,
    pub object_kind_id: Option<String>,
    pub source_vortex_index: Option<usize>,
    pub color: Option<String>,
}

impl semio_framework_value::ToValue for WorldBrushPreviewRecord {
    fn to_value(&self) -> semio_framework_value::DslValue {
        let mut entries: Vec<(String, semio_framework_value::DslValue)> = vec![
            ("meshUrl".to_string(), semio_framework_value::ToValue::to_value(&self.mesh_url)),
            ("origin".to_string(), semio_framework_value::ToValue::to_value(&self.origin)),
            ("orientation".to_string(), semio_framework_value::ToValue::to_value(&self.orientation)),
            ("targetVortexFullId".to_string(), semio_framework_value::ToValue::to_value(&self.target_vortex_full_id)),
            ("objectKindId".to_string(), semio_framework_value::ToValue::to_value(&self.object_kind_id)),
            ("sourceVortexIndex".to_string(), semio_framework_value::ToValue::to_value(&self.source_vortex_index)),
            ("color".to_string(), semio_framework_value::ToValue::to_value(&self.color)),
        ];
        entries.push((
            "scale".to_string(),
            match &self.scale {
                Some(scale) => semio_framework_value::DslValue::from(scale),
                None => semio_framework_value::DslValue::Null,
            },
        ));
        semio_framework_value::DslValue::object(entries)
    }
}

impl semio_framework_value::FromValue for WorldBrushPreviewRecord {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for WorldBrushPreviewRecord, found {value:?}")));
        };
        let mut out = WorldBrushPreviewRecord::default();
        for (key, entry) in fields {
            match key.as_str() {
                "meshUrl" => out.mesh_url = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("meshUrl"))?,
                "origin" => out.origin = <Option<[f64; 3]> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("origin"))?,
                "orientation" => out.orientation = <Option<[f64; 4]> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("orientation"))?,
                "scale" => out.scale = if matches!(entry, semio_framework_value::DslValue::Null) { None } else { Some(serde_json::Value::from(&entry)) },
                "targetVortexFullId" => out.target_vortex_full_id = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("targetVortexFullId"))?,
                "objectKindId" => out.object_kind_id = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("objectKindId"))?,
                "sourceVortexIndex" => out.source_vortex_index = <Option<usize> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("sourceVortexIndex"))?,
                "color" => out.color = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("color"))?,
                _ => {}
            }
        }
        Ok(out)
    }
}

//#region Environment
/// ☀️ Directional sun light in the same horizontal coordinate system React uses.
#[derive(Clone, Debug, Deserialize, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WorldEnvironmentSunRecord {
    #[value(default)]
    pub enabled: Option<bool>,
    #[value(default)]
    pub azimuth: Option<f64>,
    #[value(default)]
    pub elevation: Option<f64>,
    #[value(default)]
    pub intensity: Option<f64>,
    #[value(default)]
    pub color: Option<String>,
}

/// 💡️ Ambient light carried into the scene pass and GPU globals.
#[derive(Clone, Debug, Deserialize, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WorldEnvironmentAmbientRecord {
    #[value(default)]
    pub intensity: Option<f64>,
    #[value(default)]
    pub color: Option<String>,
}

/// 🌑️ Directional shadow controls shared by World3d and IconRender.
#[derive(Clone, Debug, Deserialize, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WorldEnvironmentShadowRecord {
    #[value(default)]
    pub enabled: Option<bool>,
    #[value(default)]
    pub opacity: Option<f64>,
    #[value(default)]
    pub softness: Option<f64>,
}

/// 🎨️ Neutral-instance material override. Selection and hover keep their semantic base/emissive
/// rows while sharing the material's metalness and roughness, as React's `GlbInstanceMesh` does.
#[derive(Clone, Debug, Deserialize, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WorldEnvironmentMaterialRecord {
    #[value(default)]
    pub color: Option<String>,
    #[value(default)]
    pub metalness: Option<f64>,
    #[value(default)]
    pub roughness: Option<f64>,
    #[value(default)]
    pub emissive: Option<String>,
    #[value(default)]
    pub emissive_intensity: Option<f64>,
    #[value(default)]
    pub stroke: Option<String>,
}

/// 🌍️ `World3dScene.environmentJson` mirror — see `world-3d-host.tsx`'s `WorldEnvironmentRecord`.
/// Every visible light/material field is resolved into [`ScenePass3d`] before GPU preparation.
#[derive(Clone, Debug, Deserialize, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WorldEnvironmentRecord {
    #[value(default)]
    pub background: Option<String>,
    #[value(default)]
    pub ambient: Option<WorldEnvironmentAmbientRecord>,
    #[value(default)]
    pub sun: Option<WorldEnvironmentSunRecord>,
    #[value(default)]
    pub shadow: Option<WorldEnvironmentShadowRecord>,
    #[value(default)]
    pub material: Option<WorldEnvironmentMaterialRecord>,
}
//#endregion Environment

//#region TerrainStyle
/// 🌐️⛰️ `World3dScene.terrainJson` mirror — GIS-3D terrain style/source descriptor consumed by
/// `🗺️WorldTerrainLayer` in the React reference. `color_ramp`/`min_zoom`/`max_zoom` are parsed but
/// not branched on, mirroring the React reference (single hardcoded hypsometric ramp; zoom bounds
/// fixed inside `semio_framework_surface_terrain::tiles`) — not a gap, a faithful match of upstream.
#[derive(Clone, Debug, Deserialize, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WorldTerrainStyle {
    pub tile_url_template: String,
    #[serde(default)]
    #[value(default)]
    pub project_origin_lon: f64,
    #[serde(default)]
    #[value(default)]
    pub project_origin_lat: f64,
    #[serde(default = "default_terrain_exaggeration")]
    #[value(default = "default_terrain_exaggeration")]
    pub exaggeration: f64,
    #[serde(default = "default_terrain_color_ramp")]
    #[allow(dead_code)]
    #[value(default = "default_terrain_color_ramp")]
    pub color_ramp: String,
    #[serde(default = "default_terrain_min_zoom")]
    #[allow(dead_code)]
    #[value(default = "default_terrain_min_zoom")]
    pub min_zoom: u32,
    #[serde(default = "default_terrain_max_zoom")]
    #[allow(dead_code)]
    #[value(default = "default_terrain_max_zoom")]
    pub max_zoom: u32,
}

fn default_terrain_exaggeration() -> f64 {
    1.0
}

fn default_terrain_color_ramp() -> String {
    "hypsometric".into()
}

fn default_terrain_min_zoom() -> u32 {
    6
}

fn default_terrain_max_zoom() -> u32 {
    14
}
//#endregion TerrainStyle
//#endregion SceneRecords
