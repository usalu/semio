//! 🏙️ Block 3D app — headless compute (constitutional: engine).

use block_3d::{Block3dDefinition, BLOCK_3D_SCHEMA};
use block_shared::{BlockCamera3d, BlockRepresentation};
use semio_framework_plugin::{world3d_camera_projection_json, world3d_mesh_id_from_url, world3d_selection_json, WorldProjectionConfig};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

//#region 🔖️DocumentHelpers
pub fn empty_block3d_definition() -> Block3dDefinition {
    Block3dDefinition::default()
}

/// 🪪️ Finds the smallest `"{prefix}{n}"` id not already present in `existing`.
pub fn next_id<'a>(existing: impl Iterator<Item = &'a str>, prefix: &str) -> String {
    let ids: std::collections::HashSet<&str> = existing.collect();
    let mut i = ids.len();
    loop {
        let candidate = format!("{prefix}{i}");
        if !ids.iter().any(|id| *id == candidate) {
            return candidate;
        }
        i += 1;
    }
}

/// 🌐️ Resolves the active representation's mesh url — the first representation whose `tags` all
/// appear in `wanted_tags`, or the first representation overall, or `None` for an empty catalog.
pub fn resolve_active_mesh_url<'a>(definition: &'a Block3dDefinition, wanted_tags: &[&str]) -> Option<&'a str> {
    definition
        .representations
        .iter()
        .find(|representation| wanted_tags.iter().all(|tag| representation.tags.iter().any(|other| other == tag)))
        .or_else(|| definition.representations.first())
        .and_then(|representation| representation.mesh_url.as_deref())
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️PuzzleCatalogFragment
/// 🌉️ Maps this `ObjectKind` definition into the `s/plugin/puzzle` 3d catalog shape (`objectKinds`/
/// `vortexKinds`/`cableKinds`/`attractionKinds` — see `Puzzle3dKindCatalogs`), the seam puzzle imports
/// through its `Kit×Type` media port. The active representation's mesh (first row, or the first
/// matching `wanted_tags`) becomes the catalog row's `meshUrl`.
pub fn puzzle3d_catalog_fragment(definition: &Block3dDefinition, wanted_tags: &[&str]) -> Value {
    let vortices: Vec<Value> = definition.vortices.iter().map(|vortex| json!({ "id": vortex.id, "vortexKind": vortex.vortex_kind, "position": vortex.position, "direction": vortex.direction, "radius": vortex.radius })).collect();
    let object_kind = json!({
        "id": definition.object_kind.id,
        "name": definition.object_kind.name,
        "label": definition.object_kind.label,
        "meshUrl": resolve_active_mesh_url(definition, wanted_tags),
        "vortices": vortices,
    });
    let vortex_kinds: Vec<Value> = definition.vortex_kinds.iter().map(|kind| json!({ "id": kind.id, "name": kind.name, "label": kind.label, "color": kind.color, "defaultCableKind": kind.default_cable_kind })).collect();
    let kind_compatibility: Vec<Value> = definition.compatibility.iter().map(|rule| json!({ "source": rule.source, "target": rule.target, "bidirectional": rule.bidirectional })).collect();
    json!({
        "schema": "manifest",
        "objectKinds": [object_kind],
        "vortexKinds": vortex_kinds,
        "cableKinds": Vec::<Value>::new(),
        "attractionKinds": Vec::<Value>::new(),
        "kindCompatibility": kind_compatibility,
    })
}
//#endregion 🔖️PuzzleCatalogFragment

//#region 🔖️Config
/// 🪟️ Per-window-instance view state (representation subset, layout, active utility).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Block3dWindowView {
    pub window_id: String,
    #[serde(default)]
    pub representation_ids: Vec<String>,
    #[serde(default = "default_arrangement")]
    pub arrangement: String,
    #[serde(default = "default_spacing")]
    pub spacing: f64,
    #[serde(default = "default_active_utility")]
    pub active_utility: String,
}

fn default_arrangement() -> String {
    "overlap".into()
}

fn default_spacing() -> f64 {
    8.0
}

fn default_active_utility() -> String {
    BLOCK3D_UTILITY_SELECT.into()
}

impl Block3dWindowView {
    pub fn for_window(window_id: impl Into<String>) -> Self {
        Self { window_id: window_id.into(), representation_ids: Vec::new(), arrangement: default_arrangement(), spacing: default_spacing(), active_utility: default_active_utility() }
    }
}

/// 🖌️ Transient brush hover pose in world space (config-only).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Block3dBrushPreview {
    #[dsl(coord)]
    pub position: [f64; 3],
    #[dsl(dir)]
    pub direction: [f64; 3],
}

pub const BLOCK3D_UTILITY_SELECT: &str = "select";
pub const BLOCK3D_UTILITY_SURFACE_BRUSH: &str = "surfaceBrush";
pub const BLOCK3D_DEFAULT_WINDOW_ID: &str = "block3d-world";
pub const BLOCK3D_WORLD_OBJECT_ID: &str = "block3d-object";

/// 🧮️ `Block3dPlayApp`'s real `DocumentApp::Config` — B1 pure-trait conversion. Absorbs the former
/// `Block3dPlayApp` `RefCell` runtime fields (`selected_ids`/`active_representation_id`) plus the
/// locale this app now resolves itself (mirrors `shooting_engine::ShootingConfig::locale`). `wanted_tags`
/// is new — the tag filter `export_media("catalog:out", ..)` (see `🖱️ui`) is supposed to source
/// `puzzle3d_catalog_fragment`'s `wanted_tags` argument from; `DocumentApp::export_media`'s landed
/// signature (`🧰️framework/…/🔌️plugin`) doesn't thread `ConfigView` through yet (only `doc`), so
/// today's `export_media` always calls the fragment builder with an empty (all-tags) filter — this
/// field stays here, ready for whenever a later wave threads `cfg` into `export_media`, or for any
/// other in-process reader of the config record.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "block3dcfg")]
#[dsl(layout = "lines")]
pub struct Block3dConfig {
    /// 👁️ Multi-selected row ids in the document tree — was `Block3dPlayApp::selected_ids`.
    pub selected_ids: Vec<String>,
    /// 👁️ The representation shown in the inspector's representation select — was
    /// `Block3dPlayApp::active_representation_id`.
    pub active_representation_id: Option<String>,
    /// 🏷️ Tag filter for `puzzle3d_catalog_fragment`'s active-representation resolution — see the
    /// struct doc above for why `export_media` can't read it yet. Empty means "all tags".
    pub wanted_tags: Vec<String>,
    /// 🗣️ BCP-47 locale tag — was read off the deleted `ViewState.locale`.
    pub locale: String,
    #[serde(default)]
    #[dsl(table)]
    pub windows: Vec<Block3dWindowView>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brush_vortex_kind_id: Option<String>,
    #[serde(default = "default_brush_radius")]
    pub brush_radius: f64,
    #[serde(default)]
    pub brush_flip: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brush_preview: Option<Block3dBrushPreview>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub camera: Option<BlockCamera3d>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hovered_vortex_full_id: Option<String>,
}

fn default_brush_radius() -> f64 {
    0.3
}

impl Default for Block3dConfig {
    fn default() -> Self {
        Self {
            selected_ids: Vec::new(),
            active_representation_id: None,
            wanted_tags: Vec::new(),
            locale: "en-US".into(),
            windows: Vec::new(),
            brush_vortex_kind_id: None,
            brush_radius: default_brush_radius(),
            brush_flip: false,
            brush_preview: None,
            camera: None,
            hovered_vortex_full_id: None,
        }
    }
}

store::impl_whole_record_config!(Block3dConfig);

//#endregion 🔖️Config

//#region 🔖️Io
/// 🔌️ `Block3dPlayApp`'s typed media I/O surface (`AppDefinition.io`) — the implicit document ports
/// (`Kit×Type`, matching the `"3d.block"` artifact kind) plus the flagship `"catalog:out"` port this
/// ticket adds: the puzzle3d seam that finally gives `puzzle3d_catalog_fragment` a real caller (see
/// `🖱️ui`'s `Block3dPlayApp::export_media`).
pub fn block3d_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo::from_document(
        BLOCK_3D_SCHEMA,
        semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Kit, form: semio_framework_plugin::MediaForm::Type },
        semio_framework_plugin::ArtifactPresentation { id: "3d.block".into(), name: "Object Kind".into(), dimension: "3d".into(), component_kind: "block3d".into() },
    )
    .with_ports(vec![semio_framework_plugin::MediaPortSpec {
        id: "catalog:out".into(),
        label: "Kit Catalog".into(),
        direction: semio_framework_plugin::MediaPortDirection::Out,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Kit, form: semio_framework_plugin::MediaForm::Type },
        kind_id: Some("kit.catalog".into()),
        required: false,
        multiplicity: semio_framework_plugin::PortMultiplicity::Many,
    }])
}
//#endregion 🔖️Io

//#region 🔖️World
pub fn block3d_window_view<'a>(config: &'a Block3dConfig, window_id: &str) -> Block3dWindowView {
    config.windows.iter().find(|row| row.window_id == window_id).cloned().unwrap_or_else(|| Block3dWindowView::for_window(window_id))
}

pub fn block3d_active_utility(config: &Block3dConfig, window_id: &str) -> String {
    block3d_window_view(config, window_id).active_utility
}

pub fn visible_representations<'a>(definition: &'a Block3dDefinition, view: &Block3dWindowView) -> Vec<&'a BlockRepresentation> {
    if view.representation_ids.is_empty() {
        return definition.representations.iter().collect();
    }
    view.representation_ids.iter().filter_map(|id| definition.representations.iter().find(|representation| representation.id == *id)).collect()
}

pub fn arrangement_offset(arrangement: &str, index: usize, spacing: f64) -> [f64; 3] {
    let step = index as f64 * spacing;
    match arrangement {
        "x" => [step, 0.0, 0.0],
        "y" => [0.0, step, 0.0],
        "z" => [0.0, 0.0, step],
        _ => [0.0, 0.0, 0.0],
    }
}

pub fn effective_camera<'a>(definition: &'a Block3dDefinition, config: &'a Block3dConfig) -> &'a BlockCamera3d {
    config.camera.as_ref().unwrap_or(&definition.camera3d)
}

pub fn world_meshes_json(_definition: &Block3dDefinition, visible: &[&BlockRepresentation]) -> String {
    let meshes: Vec<serde_json::Value> = visible
        .iter()
        .filter_map(|representation| {
            let url = representation.mesh_url.as_deref()?;
            Some(json!({ "id": representation_mesh_id(representation), "url": url }))
        })
        .collect();
    serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into())
}

pub fn representation_mesh_id(representation: &BlockRepresentation) -> String {
    representation
        .mesh_url
        .as_deref()
        .map(world3d_mesh_id_from_url)
        .unwrap_or_else(|| format!("block3d-rep-{}", representation.id))
}

pub fn world_instances_json(definition: &Block3dDefinition, visible: &[&BlockRepresentation], view: &Block3dWindowView) -> String {
    let label = if definition.object_kind.label.is_empty() { definition.object_kind.name.clone() } else { definition.object_kind.label.clone() };
    let instances: Vec<serde_json::Value> = visible
        .iter()
        .enumerate()
        .map(|(index, representation)| {
            let offset = arrangement_offset(&view.arrangement, index, view.spacing);
            let mesh_id = representation_mesh_id(representation);
            json!({
                "id": representation.id,
                "meshId": mesh_id,
                "position": offset,
                "rotation": [0.0, 0.0, 0.0, 1.0],
                "scale": [1.0, 1.0, 1.0],
                "label": format!("{} — {}", label, representation.name),
                "objectKind": definition.object_kind.id,
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

fn vortex_kind_color(definition: &Block3dDefinition, vortex_kind_id: &str) -> String {
    definition.vortex_kinds.iter().find(|kind| kind.id == vortex_kind_id).map(|kind| kind.color.clone()).unwrap_or_else(|| "#888888".into())
}

pub fn block3d_vortex_full_id(object_id: &str, vortex_id: &str) -> String {
    format!("{object_id}:{vortex_id}")
}

pub fn world_vortices_json(definition: &Block3dDefinition, config: &Block3dConfig, visible: &[&BlockRepresentation], view: &Block3dWindowView) -> String {
    let mut records = Vec::new();
    for (index, representation) in visible.iter().enumerate() {
        let offset = arrangement_offset(&view.arrangement, index, view.spacing);
        for vortex in &definition.vortices {
            let position = [vortex.position[0] + offset[0], vortex.position[1] + offset[1], vortex.position[2] + offset[2]];
            records.push(json!({
                "fullId": block3d_vortex_full_id(&representation.id, &vortex.id),
                "objectId": representation.id,
                "vortexKind": vortex.vortex_kind,
                "position": position,
                "direction": vortex.direction,
                "radius": vortex.radius,
                "color": vortex_kind_color(definition, &vortex.vortex_kind),
            }));
        }
    }
    if let Some(preview) = &config.brush_preview {
        let direction = if config.brush_flip { [-preview.direction[0], -preview.direction[1], -preview.direction[2]] } else { preview.direction };
        records.push(json!({
            "fullId": "__brush_preview__",
            "objectId": visible.first().map(|r| r.id.as_str()).unwrap_or(BLOCK3D_WORLD_OBJECT_ID),
            "vortexKind": config.brush_vortex_kind_id.clone().unwrap_or_else(|| "brush".into()),
            "position": preview.position,
            "direction": direction,
            "radius": config.brush_radius,
            "color": "#60a5fa88",
        }));
    }
    serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
}

pub fn world_camera_json(definition: &Block3dDefinition, config: &Block3dConfig) -> String {
    let camera = effective_camera(definition, config);
    world3d_camera_projection_json(camera.position, camera.target, None, camera.zoom, &WorldProjectionConfig::default())
}

pub fn world_selection_json(config: &Block3dConfig) -> String {
    let vortex_ids: Vec<String> = config.selected_ids.iter().filter(|id| id.starts_with("vortex:")).map(|id| id.strip_prefix("vortex:").unwrap_or(id).to_string()).collect();
    let mut value: serde_json::Value = serde_json::from_str(&world3d_selection_json("replace", &[], None)).unwrap_or_else(|_| json!({}));
    if let Some(object) = value.as_object_mut() {
        object.insert("granularity".into(), json!("mesh"));
        object.insert("selectionMode".into(), json!("mesh"));
        object.insert("vortexIds".into(), json!(vortex_ids));
        if let Some(hover) = config.hovered_vortex_full_id.as_deref() {
            object.insert("hoveredVortexFullId".into(), json!(hover));
        }
    }
    value.to_string()
}

pub fn world_interaction_json(config: &Block3dConfig, window_id: &str) -> String {
    json!({ "activeUtility": block3d_active_utility(config, window_id) }).to_string()
}

pub fn world_hit_to_local_vortex(position: [f64; 3], normal: [f64; 3], instance_offset: [f64; 3], brush_flip: bool) -> (Block3dBrushPreview, [f64; 3]) {
    let local_position = [position[0] - instance_offset[0], position[1] - instance_offset[1], position[2] - instance_offset[2]];
    let direction = if brush_flip { [-normal[0], -normal[1], -normal[2]] } else { normal };
    (Block3dBrushPreview { position: local_position, direction }, local_position)
}

pub fn instance_offset_for_representation(definition: &Block3dDefinition, view: &Block3dWindowView, representation_id: &str) -> [f64; 3] {
    let visible = visible_representations(definition, view);
    visible.iter().position(|representation| representation.id == representation_id).map(|index| arrangement_offset(&view.arrangement, index, view.spacing)).unwrap_or([0.0, 0.0, 0.0])
}

pub fn default_vortex_kind() -> block_3d::Block3dVortexKind {
    block_3d::Block3dVortexKind { id: "vortex-kind-0".into(), name: "connector".into(), label: "Connector".into(), color: "#60a5fa".into(), default_cable_kind: "cable.link".into() }
}

pub fn resolve_brush_vortex_kind_id(definition: &Block3dDefinition, config: &Block3dConfig) -> String {
    config
        .brush_vortex_kind_id
        .clone()
        .or_else(|| definition.vortex_kinds.first().map(|kind| kind.id.clone()))
        .unwrap_or_else(|| "vortex-kind-0".into())
}

pub fn upsert_window_view_index(windows: &mut Vec<Block3dWindowView>, window_id: &str) -> usize {
    if let Some(index) = windows.iter().position(|row| row.window_id == window_id) {
        return index;
    }
    windows.push(Block3dWindowView::for_window(window_id));
    windows.len() - 1
}
//#endregion 🔖️World

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use block_3d::{Block3dVortexTemplate, BLOCK_3D_SCHEMA};
    use block_shared::{BlockKindIdentity, BlockRepresentation};

    #[test]
    fn empty_definition_matches_default() {
        assert_eq!(empty_block3d_definition(), Block3dDefinition::default());
    }

    #[test]
    fn resolve_active_mesh_url_prefers_matching_tags() {
        let mut definition = Block3dDefinition::default();
        definition.representations.push(BlockRepresentation { id: "r0".into(), name: "1:500".into(), mesh_url: Some("/mesh/low.glb".into()), tags: vec!["1to500".into()], lod: None, description: String::new(), attributes: Vec::new() });
        definition.representations.push(BlockRepresentation { id: "r1".into(), name: "full".into(), mesh_url: Some("/mesh/full.glb".into()), tags: vec!["full".into()], lod: None, description: String::new(), attributes: Vec::new() });
        assert_eq!(resolve_active_mesh_url(&definition, &["full"]), Some("/mesh/full.glb"));
        assert_eq!(resolve_active_mesh_url(&definition, &["missing"]), Some("/mesh/low.glb"));
    }

    #[test]
    fn puzzle3d_catalog_fragment_maps_vortices() {
        let mut definition = Block3dDefinition { schema: BLOCK_3D_SCHEMA.into(), object_kind: BlockKindIdentity { id: "capsule".into(), name: "capsule".into(), label: "Capsule".into(), ..Default::default() }, ..Block3dDefinition::default() };
        definition.vortices.push(Block3dVortexTemplate { id: "v0".into(), vortex_kind: "door".into(), position: [0.0, 0.0, 0.0], direction: [0.0, 1.0, 0.0], radius: 0.3, label: None });
        let fragment = puzzle3d_catalog_fragment(&definition, &[]);
        assert_eq!(fragment["objectKinds"][0]["id"], "capsule");
        assert_eq!(fragment["objectKinds"][0]["vortices"][0]["vortexKind"], "door");
    }

    #[test]
    fn block3d_config_default_has_no_selection_and_all_tags() {
        let config = Block3dConfig::default();
        assert!(config.selected_ids.is_empty());
        assert!(config.active_representation_id.is_none());
        assert!(config.wanted_tags.is_empty());
        assert_eq!(config.locale, "en-US");
        assert!(config.windows.is_empty());
        assert_eq!(config.brush_radius, 0.3);
    }

    #[test]
    fn arrangement_offset_spaces_along_x() {
        assert_eq!(arrangement_offset("x", 2, 4.0), [8.0, 0.0, 0.0]);
        assert_eq!(arrangement_offset("overlap", 2, 4.0), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn visible_representations_empty_filter_means_all() {
        let mut definition = empty_block3d_definition();
        definition.representations.push(BlockRepresentation { id: "a".into(), name: "a".into(), mesh_url: None, tags: Vec::new(), lod: None, description: String::new(), attributes: Vec::new() });
        definition.representations.push(BlockRepresentation { id: "b".into(), name: "b".into(), mesh_url: None, tags: Vec::new(), lod: None, description: String::new(), attributes: Vec::new() });
        let view = Block3dWindowView::for_window("w");
        assert_eq!(visible_representations(&definition, &view).len(), 2);
        let mut filtered = view;
        filtered.representation_ids = vec!["b".into()];
        assert_eq!(visible_representations(&definition, &filtered).len(), 1);
    }

    #[test]
    fn block3d_io_declares_the_catalog_out_port() {
        let io = block3d_io();
        assert_eq!(io.document_schema, BLOCK_3D_SCHEMA);
        let ports = io.all_ports();
        assert!(ports.iter().any(|port| port.id == "document:in"));
        assert!(ports.iter().any(|port| port.id == "document:out"));
        let catalog = ports.iter().find(|port| port.id == "catalog:out").expect("catalog:out port declared");
        assert_eq!(catalog.kind_id.as_deref(), Some("kit.catalog"));
        assert_eq!(catalog.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert_eq!(catalog.multiplicity, semio_framework_plugin::PortMultiplicity::Many);
    }
}
//#endregion 🧪️Tests
