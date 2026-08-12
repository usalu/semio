//! ⚙️ Sourcing curate artifact — headless compute over the `CurateSnapshot` projection (constitutional:
//! engine).
//!
//! Query/mutation logic over `CurateSnapshot` lives here as free functions (`filtered_stock`,
//! `curated_count`, `curate_delta`, `curate_set`) rather than inherent methods, mirroring every other
//! artifact in this taxonomy. The rule for what lands here rather than next to a single caller: a helper
//! with MORE THAN ONE consumer across the taxonomy tree lives here; a helper with exactly one consumer
//! lives in that consumer's component file (e.g. `crate::apps::curate::modes::curate::windows::pool`'s
//! `build_filter_bar`/`build_pool_table`, used only by the pool window).

use crate::artifacts::curate::{CurateSnapshot, CuratedItem, Filters, GeometryRecipe, ObjectKind, SOURCING_CURATE_SCHEMA};
use semio_framework::parse_contributions;
use serde_json::{json, Value};
use std::sync::Mutex;

//#region 🔖️Register
/// 🗂️ Registers `CurateSnapshot`'s pack↔dsl codec under `SOURCING_CURATE_SCHEMA` so `framework/sync`'s
/// folder endpoints and any other schema-string-keyed caller can print/parse curate documents. Called
/// from the plugin root's `semio_plugin!{ setup: … }`.
pub fn register() {
    crate::artifacts::curate::io_registry::register();

    register_artifact_schema();
    register_artifact_inference();
    crate::apps::curate::config::schema::register_app_schema();
    register_pilot_languages();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::curate::SourcingCurateApp>(SOURCING_CURATE_SCHEMA);
}

/// 📎 Registers the curate artifact schema descriptor into the process-local registry.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::curate::schema::curate_artifact_schema_descriptor());
}

/// 💡️ Registers `s.sourcing.curate.inference`'s facet leaves into the OS-wide inference catalog —
/// sibling to `register_artifact_schema()` (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
pub fn register_artifact_inference() {
    ::schema::register_artifact_inference_descriptor(crate::artifacts::curate::standards::v1::subsets::any::schema::inferences::curate_artifact_inference_descriptor());
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "sourcing.curate",
        extension: Some("curate"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::curate::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::curate::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::curate::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::curate::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("sourcing.curate"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "sourcing.curate.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::curate::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::curate::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::curate::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::curate::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("sourcing.curate.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "sourcing.curate.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::curate::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::curate::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("sourcing.curate.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "curate.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::curate::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::curate::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("curate.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "curate.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::curate::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::curate::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("curate.spr"),
    });
}

//#endregion 🔖️Register

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — the implicit document ports (keyed off
/// `SOURCING_CURATE_SCHEMA`, `MediaType{Kit,Kit}` matching the `"catalogue.sourcing"` `ArtifactKindSpec`)
/// plus the extra `catalog:out` output port: this app's `stock` (its `"catalogue.kinds"`-shaped rows)
/// mapped into the SAME `kit.catalog` JSON shape `block_3d::puzzle3d_catalog_fragment` produces, so
/// `s/plugin/puzzle`'s `kit:in` importer can consume either producer identically without knowing which
/// one it came from (see `sourcing_catalog_fragment`).
pub fn sourcing_curate_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: SOURCING_CURATE_SCHEMA.into(),
        document_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Kit, form: semio_framework_plugin::MediaForm::Kit },
        ports: vec![semio_framework_plugin::MediaPortSpec {
            id: "catalog:out".into(),
            label: "Catalog".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Kit, form: semio_framework_plugin::MediaForm::Type },
            kind_id: Some("kit.catalog".into()),
            required: false,
            multiplicity: semio_framework::PortMultiplicity::Many,
        }],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "catalogue.sourcing".into(), name: "Sourcing Curation".into(), dimension: "data".into(), component_kind: "catalogue".into() },
    }
}
//#endregion 🔖️Io

//#region 🔖️PuzzleCatalogFragment
/// 🌉️ Maps this app's stock (its `"catalogue.kinds"`-shaped rows) into the `s/plugin/puzzle` 3d catalog
/// shape (`objectKinds`/`vortexKinds`/`cableKinds`/`attractionKinds`/`kindCompatibility` — see
/// `block_3d::puzzle3d_catalog_fragment`, the sibling producer this mirrors byte-for-byte in shape), the
/// seam puzzle imports through its `Kit×Type` `kit:in` media port. Sourcing's `ObjectKind` carries no
/// mesh URL (geometry is a procedural `GeometryRecipe`, not an asset reference) or vortex/attachment
/// data, so every row's `meshUrl` is `null` and `vortices` is empty — puzzle's importer treats a missing
/// mesh as "no visual representation yet", not an error.
pub fn sourcing_catalog_fragment(document: &CurateSnapshot) -> Value {
    let object_kinds: Vec<Value> = document.stock.iter().map(|kind| json!({ "id": kind.id, "name": kind.name, "label": kind.name, "meshUrl": Value::Null, "vortices": Vec::<Value>::new() })).collect();
    json!({
        "schema": "manifest",
        "objectKinds": object_kinds,
        "vortexKinds": Vec::<Value>::new(),
        "cableKinds": Vec::<Value>::new(),
        "attractionKinds": Vec::<Value>::new(),
        "kindCompatibility": Vec::<Value>::new(),
    })
}
//#endregion 🔖️PuzzleCatalogFragment

//#region 🔖️Typology
/// 🌳️ One node in a module's typology tree — object kinds reference a node by its path of segment ids.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypologyNode {
    pub id: String,
    pub label: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<TypologyNode>,
}

impl TypologyNode {
    pub fn new(id: impl Into<String>, label: impl Into<String>, children: Vec<TypologyNode>) -> Self {
        Self { id: id.into(), label: label.into(), children }
    }
}

/// 🔎️ Whether `path` (a sequence of segment ids from the root) resolves to a node in `root`'s tree.
pub fn typology_contains(root: &TypologyNode, path: &[String]) -> bool {
    match path.split_first() {
        None => true,
        Some((head, rest)) if *head == root.id => {
            if rest.is_empty() {
                true
            } else {
                root.children.iter().any(|child| typology_contains(child, rest))
            }
        }
        _ => false,
    }
}

/// 📋️ Flattens a typology tree into `(full path from root, label)` pairs, depth-first, for filter UIs.
pub fn typology_flatten(root: &TypologyNode) -> Vec<(Vec<String>, String)> {
    fn walk(node: &TypologyNode, prefix: &[String], out: &mut Vec<(Vec<String>, String)>) {
        let mut path = prefix.to_vec();
        path.push(node.id.clone());
        out.push((path.clone(), node.label.clone()));
        for child in &node.children {
            walk(child, &path, out);
        }
    }
    let mut out = Vec::new();
    walk(root, &[], &mut out);
    out
}
//#endregion 🔖️Typology

//#region 🔖️Geometry
/// 🧱️ Flat indexed triangle mesh data, ready for `mesh_from_indexed` at the plugin boundary.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MeshDataSpec {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub indices: Vec<u32>,
}

/// ➕️➖️ Appends `other` onto `base`, offsetting `other`'s indices past `base`'s existing vertex count.
pub fn append_mesh_spec(base: &mut MeshDataSpec, other: MeshDataSpec) {
    let vertex_offset = (base.positions.len() / 3) as u32;
    base.positions.extend(other.positions);
    base.normals.extend(other.normals);
    base.indices.extend(other.indices.into_iter().map(|i| i + vertex_offset));
}

/// 📐️ Builds an axis-aligned box mesh centered at the origin, with per-face flat normals.
fn box_mesh_spec(width: f64, height: f64, depth: f64) -> MeshDataSpec {
    let (hw, hh, hd) = ((width * 0.5) as f32, (height * 0.5) as f32, (depth * 0.5) as f32);
    // 6 faces * 4 verts, wound counter-clockwise when viewed from outside along the face normal.
    let faces: [([f32; 3], [[f32; 3]; 4]); 6] = [
        ([0.0, 0.0, 1.0], [[-hw, -hh, hd], [hw, -hh, hd], [hw, hh, hd], [-hw, hh, hd]]),
        ([0.0, 0.0, -1.0], [[hw, -hh, -hd], [-hw, -hh, -hd], [-hw, hh, -hd], [hw, hh, -hd]]),
        ([1.0, 0.0, 0.0], [[hw, -hh, hd], [hw, -hh, -hd], [hw, hh, -hd], [hw, hh, hd]]),
        ([-1.0, 0.0, 0.0], [[-hw, -hh, -hd], [-hw, -hh, hd], [-hw, hh, hd], [-hw, hh, -hd]]),
        ([0.0, 1.0, 0.0], [[-hw, hh, hd], [hw, hh, hd], [hw, hh, -hd], [-hw, hh, -hd]]),
        ([0.0, -1.0, 0.0], [[-hw, -hh, -hd], [hw, -hh, -hd], [hw, -hh, hd], [-hw, -hh, hd]]),
    ];
    let mut spec = MeshDataSpec::default();
    for (normal, corners) in faces {
        let base = (spec.positions.len() / 3) as u32;
        for corner in corners {
            spec.positions.extend(corner);
            spec.normals.extend(normal);
        }
        spec.indices.extend([base, base + 1, base + 2, base, base + 2, base + 3]);
    }
    spec
}

/// 🪟️ Builds a rectangular frame (4 mitred boxes: top/bottom rails, left/right stiles) around an opening.
fn frame_mesh_spec(width: f64, height: f64, depth: f64, profile: f64) -> MeshDataSpec {
    let mut spec = MeshDataSpec::default();
    let mut add = |w: f64, h: f64, cx: f64, cy: f64| {
        let mut piece = box_mesh_spec(w, h, depth);
        for i in (0..piece.positions.len()).step_by(3) {
            piece.positions[i] += cx as f32;
            piece.positions[i + 1] += cy as f32;
        }
        append_mesh_spec(&mut spec, piece);
    };
    let half_h = height * 0.5;
    let half_w = width * 0.5;
    add(width, profile, 0.0, half_h - profile * 0.5);
    add(width, profile, 0.0, -half_h + profile * 0.5);
    let stile_h = height - profile * 2.0;
    add(profile, stile_h, -half_w + profile * 0.5, 0.0);
    add(profile, stile_h, half_w - profile * 0.5, 0.0);
    spec
}

/// 🧱️ Realizes a `GeometryRecipe` into flat mesh data.
pub fn mesh_spec_for(recipe: &GeometryRecipe) -> MeshDataSpec {
    match recipe {
        GeometryRecipe::Box { width, height, depth } => box_mesh_spec(*width, *height, *depth),
        GeometryRecipe::Frame { width, height, depth, profile } => frame_mesh_spec(*width, *height, *depth, *profile),
        GeometryRecipe::Slab { width, depth, thickness } => box_mesh_spec(*width, *thickness, *depth),
        GeometryRecipe::Mesh { positions, normals, indices } => MeshDataSpec { positions: positions.clone(), normals: normals.clone(), indices: indices.clone() },
    }
}

/// 📏️ The largest bounding dimension of a recipe's geometry, used to normalize grid-cell scale.
pub fn bounding_extent(recipe: &GeometryRecipe) -> f64 {
    match recipe {
        GeometryRecipe::Box { width, height, depth } => width.max(*height).max(*depth),
        GeometryRecipe::Frame { width, height, depth, .. } => width.max(*height).max(*depth),
        GeometryRecipe::Slab { width, depth, thickness } => width.max(*depth).max(*thickness),
        GeometryRecipe::Mesh { positions, .. } => positions.chunks(3).flat_map(|p| p.iter().map(|v| v.abs() as f64 * 2.0)).fold(0.0_f64, f64::max).max(1e-6),
    }
}
//#endregion 🔖️Geometry

//#region 🔖️World3d
/// 🌐️ JSON mesh atom for a stock kind's realized geometry — shared by the preview and grid windows
/// (two consumers, hence engine-owned per this file's DocumentHelpers rule).
pub fn kind_mesh_json(kind: &ObjectKind) -> Value {
    let spec = mesh_spec_for(&kind.geometry);
    let mesh = semio_framework::mesh_from_indexed(&spec.positions, &spec.normals, &spec.indices);
    json!({ "id": kind.id, "data": mesh })
}

/// 🌐️ JSON instance atom placing one stock kind's mesh at `position` — shared by the preview and grid
/// windows.
pub fn instance_json(kind: &ObjectKind, position: [f64; 3], scale: f64, selected: bool) -> Value {
    json!({
        "id": kind.id,
        "meshId": kind.id,
        "position": position,
        "rotation": [0.0, 0.0, 0.0, 1.0],
        "scale": [scale, scale, scale],
        "label": kind.name,
        "selected": selected,
        "hovered": false,
    })
}
//#endregion 🔖️World3d

//#region 🔖️DocumentHelpers
/// 🔎️ The stock kinds that currently satisfy every active filter dimension. `filters` lives on
/// `crate::apps::curate::config::SourcingCurateConfig` (session-only view state), so this takes it as a
/// separate parameter rather than reading it off the document.
pub fn filtered_stock<'a>(document: &'a CurateSnapshot, filters: &Filters) -> Vec<&'a ObjectKind> {
    document
        .stock
        .iter()
        .filter(|kind| {
            let query = filters.query.trim().to_lowercase();
            let matches_query = query.is_empty() || kind.name.to_lowercase().contains(&query);
            let matches_module = filters.module_ids.is_empty() || filters.module_ids.contains(&kind.module_id);
            let matches_typology = filters.typology_path.is_empty() || kind.typology_path.starts_with(&filters.typology_path);
            let matches_availability = kind.availability >= filters.min_availability;
            matches_query && matches_module && matches_typology && matches_availability
        })
        .collect()
}

/// 🔢️ How many units of `object_id` are currently in the curated set (0 if absent).
pub fn curated_count(document: &CurateSnapshot, object_id: &str) -> u32 {
    document.curated.iter().find(|item| item.object_id == object_id).map_or(0, |item| item.count)
}

/// ➕️➖️ Adjusts the curated count for `object_id` by `delta`, clamped to `0..=availability`; removes the
/// entry entirely when the count reaches 0. Silently no-operations if `object_id` isn't in the stock.
pub fn curate_delta(document: &mut CurateSnapshot, object_id: &str, delta: i64) {
    let Some(kind) = document.stock.iter().find(|kind| kind.id == object_id) else { return };
    let next = (curated_count(document, object_id) as i64 + delta).clamp(0, kind.availability as i64) as u32;
    curate_set(document, object_id, next);
}

/// 🎯️ Sets the curated count for `object_id` directly, clamped to `0..=availability`; removes the
/// entry when the count is 0. Silently no-operations if `object_id` isn't in the stock.
pub fn curate_set(document: &mut CurateSnapshot, object_id: &str, count: u32) {
    let Some(kind) = document.stock.iter().find(|kind| kind.id == object_id) else { return };
    let clamped = count.min(kind.availability);
    match document.curated.iter_mut().find(|item| item.object_id == object_id) {
        Some(item) if clamped == 0 => {
            let id = item.object_id.clone();
            document.curated.retain(|item| item.object_id != id);
        }
        Some(item) => item.count = clamped,
        None if clamped > 0 => document.curated.push(CuratedItem { object_id: object_id.to_string(), count: clamped }),
        None => {}
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Modules
/// 🧩️ A sourcing module composes a typology subtree, demo catalogue kinds, and preview meshing for one
/// object family (e.g. beams, windows, slabs) — modules are trait objects, not subclasses of a base app.
pub trait SourcingModule {
    fn module_id(&self) -> &'static str;
    fn label(&self) -> &'static str;
    fn typology(&self) -> TypologyNode;
    fn demo_kinds(&self) -> Vec<ObjectKind>;
    /// 🧱️ Realizes a kind's preview mesh; defaults to the generic geometry recipe realization.
    fn preview_mesh(&self, kind: &ObjectKind) -> MeshDataSpec {
        mesh_spec_for(&kind.geometry)
    }
}

pub mod beams {
    use super::{GeometryRecipe, ObjectKind, SourcingModule, TypologyNode};

    pub struct BeamsModule;

    impl SourcingModule for BeamsModule {
        fn module_id(&self) -> &'static str {
            "beams"
        }
        fn label(&self) -> &'static str {
            "Beams"
        }
        fn typology(&self) -> TypologyNode {
            TypologyNode::new(
                "beams",
                "Beams",
                vec![
                    TypologyNode::new("solid-timber", "Solid Timber", vec![TypologyNode::new("glulam", "Glulam", vec![]), TypologyNode::new("kvh", "KVH", vec![])]),
                    TypologyNode::new("steel", "Steel", vec![TypologyNode::new("ipe", "IPE", vec![]), TypologyNode::new("hea", "HEA", vec![])]),
                ],
            )
        }
        fn demo_kinds(&self) -> Vec<ObjectKind> {
            vec![
                ObjectKind {
                    id: "beam-glulam-gl24h".into(),
                    name: "Glulam GL24h 200×400".into(),
                    module_id: "beams".into(),
                    typology_path: vec!["beams".into(), "solid-timber".into(), "glulam".into()],
                    availability: 24,
                    geometry: Box::new(GeometryRecipe::Box { width: 0.2, height: 0.4, depth: 6.0 }),
                },
                ObjectKind {
                    id: "beam-kvh-c24".into(),
                    name: "KVH C24 100×200".into(),
                    module_id: "beams".into(),
                    typology_path: vec!["beams".into(), "solid-timber".into(), "kvh".into()],
                    availability: 60,
                    geometry: Box::new(GeometryRecipe::Box { width: 0.1, height: 0.2, depth: 4.0 }),
                },
                ObjectKind {
                    id: "beam-steel-ipe200".into(),
                    name: "Steel IPE 200".into(),
                    module_id: "beams".into(),
                    typology_path: vec!["beams".into(), "steel".into(), "ipe".into()],
                    availability: 12,
                    geometry: Box::new(GeometryRecipe::Box { width: 0.1, height: 0.2, depth: 5.0 }),
                },
                ObjectKind {
                    id: "beam-steel-hea160".into(),
                    name: "Steel HEA 160".into(),
                    module_id: "beams".into(),
                    typology_path: vec!["beams".into(), "steel".into(), "hea".into()],
                    availability: 8,
                    geometry: Box::new(GeometryRecipe::Box { width: 0.16, height: 0.152, depth: 5.0 }),
                },
            ]
        }
    }
}

pub mod windows {
    use super::{GeometryRecipe, ObjectKind, SourcingModule, TypologyNode};

    pub struct WindowsModule;

    impl SourcingModule for WindowsModule {
        fn module_id(&self) -> &'static str {
            "windows"
        }
        fn label(&self) -> &'static str {
            "Windows"
        }
        fn typology(&self) -> TypologyNode {
            TypologyNode::new("windows", "Windows", vec![TypologyNode::new("casement", "Casement", vec![]), TypologyNode::new("fixed", "Fixed", vec![]), TypologyNode::new("tilt-turn", "Tilt & Turn", vec![])])
        }
        fn demo_kinds(&self) -> Vec<ObjectKind> {
            vec![
                ObjectKind {
                    id: "window-casement-100x120".into(),
                    name: "Casement Window 100×120".into(),
                    module_id: "windows".into(),
                    typology_path: vec!["windows".into(), "casement".into()],
                    availability: 18,
                    geometry: Box::new(GeometryRecipe::Frame { width: 1.0, height: 1.2, depth: 0.08, profile: 0.08 }),
                },
                ObjectKind {
                    id: "window-fixed-150x150".into(),
                    name: "Fixed Window 150×150".into(),
                    module_id: "windows".into(),
                    typology_path: vec!["windows".into(), "fixed".into()],
                    availability: 10,
                    geometry: Box::new(GeometryRecipe::Frame { width: 1.5, height: 1.5, depth: 0.06, profile: 0.06 }),
                },
                ObjectKind {
                    id: "window-tilt-turn-120x140".into(),
                    name: "Tilt & Turn Window 120×140".into(),
                    module_id: "windows".into(),
                    typology_path: vec!["windows".into(), "tilt-turn".into()],
                    availability: 14,
                    geometry: Box::new(GeometryRecipe::Frame { width: 1.2, height: 1.4, depth: 0.09, profile: 0.09 }),
                },
            ]
        }
    }
}

pub mod slabs {
    use super::{GeometryRecipe, ObjectKind, SourcingModule, TypologyNode};

    pub struct SlabsModule;

    impl SourcingModule for SlabsModule {
        fn module_id(&self) -> &'static str {
            "slabs"
        }
        fn label(&self) -> &'static str {
            "Slabs"
        }
        fn typology(&self) -> TypologyNode {
            TypologyNode::new("slabs", "Slabs", vec![TypologyNode::new("concrete", "Concrete", vec![]), TypologyNode::new("clt", "CLT", vec![]), TypologyNode::new("hollow-core", "Hollow Core", vec![])])
        }
        fn demo_kinds(&self) -> Vec<ObjectKind> {
            vec![
                ObjectKind {
                    id: "slab-concrete-240".into(),
                    name: "Concrete Slab 240mm".into(),
                    module_id: "slabs".into(),
                    typology_path: vec!["slabs".into(), "concrete".into()],
                    availability: 30,
                    geometry: Box::new(GeometryRecipe::Slab { width: 2.4, depth: 1.2, thickness: 0.24 }),
                },
                ObjectKind {
                    id: "slab-clt-160".into(),
                    name: "CLT Slab 160mm".into(),
                    module_id: "slabs".into(),
                    typology_path: vec!["slabs".into(), "clt".into()],
                    availability: 20,
                    geometry: Box::new(GeometryRecipe::Slab { width: 2.95, depth: 1.25, thickness: 0.16 }),
                },
                ObjectKind {
                    id: "slab-hollow-core-265".into(),
                    name: "Hollow Core Slab 265mm".into(),
                    module_id: "slabs".into(),
                    typology_path: vec!["slabs".into(), "hollow-core".into()],
                    availability: 16,
                    geometry: Box::new(GeometryRecipe::Slab { width: 1.2, depth: 6.0, thickness: 0.265 }),
                },
            ]
        }
    }
}

fn leak_str(value: String) -> &'static str {
    Box::leak(value.into_boxed_str())
}

/// 🧩️ One hot-installed sourcing module deserialized from the `"sourcing.module"` topic contribution.
#[derive(Clone)]
struct ContributedSourcingModule {
    module_id: &'static str,
    label: &'static str,
    typology: TypologyNode,
    kinds: Vec<ObjectKind>,
}

impl SourcingModule for ContributedSourcingModule {
    fn module_id(&self) -> &'static str {
        self.module_id
    }

    fn label(&self) -> &'static str {
        self.label
    }

    fn typology(&self) -> TypologyNode {
        self.typology.clone()
    }

    fn demo_kinds(&self) -> Vec<ObjectKind> {
        self.kinds.clone()
    }
}

static CONTRIBUTED_SOURCING_MODULES: Mutex<Vec<ContributedSourcingModule>> = Mutex::new(Vec::new());
static LAST_SOURCING_CONTRIBUTIONS_JSON: Mutex<String> = Mutex::new(String::new());

const SOURCING_CURATE_APP_ID: &str = "sourcing-curate";

/// 🔌️ Refreshes contributed `sourcing.module` entries when the host pushes a new catalogue.
//#region 🔖️SourcingModuleTopicPayload
/// 🗂️ `topic_contribution.payload` shape for the `"sourcing.module"` topic.
/// See `TopicContribution` in `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`.
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct SourcingModuleTopicPayload {
    app_id: String,
    module_id: String,
    label: String,
    typology_json: String,
    kinds_json: String,
}
//#endregion 🔖️SourcingModuleTopicPayload

pub fn sync_sourcing_module_contributions(contributions_json: &str) {
    let mut last = LAST_SOURCING_CONTRIBUTIONS_JSON.lock().expect("sourcing contributions lock");
    if *last == contributions_json {
        return;
    }
    let mut modules = Vec::new();
    for entry in parse_contributions(contributions_json) {
        let Some(payload) = entry
            .topic_contribution
            .as_ref()
            .filter(|topic| topic.topic == "sourcing.module")
            .and_then(|topic| topic.decode::<SourcingModuleTopicPayload>().ok())
        else {
            continue;
        };
        let (app_id, module_id, label, typology_json, kinds_json) =
            (payload.app_id, payload.module_id, payload.label, payload.typology_json, payload.kinds_json);
        if app_id != SOURCING_CURATE_APP_ID {
            continue;
        }
        let Ok(typology) = serde_json::from_str::<TypologyNode>(&typology_json) else {
            continue;
        };
        let Ok(kinds) = serde_json::from_str::<Vec<ObjectKind>>(&kinds_json) else {
            continue;
        };
        modules.push(ContributedSourcingModule { module_id: leak_str(module_id), label: leak_str(label), typology, kinds });
    }
    *CONTRIBUTED_SOURCING_MODULES.lock().expect("sourcing contributed modules lock") = modules;
    *last = contributions_json.to_string();
}

/// 🧩️ Every sourcing module known to this crate, in stable order.
pub fn sourcing_modules() -> Vec<Box<dyn SourcingModule>> {
    let mut modules: Vec<Box<dyn SourcingModule>> = vec![
        Box::new(beams::BeamsModule),
        Box::new(windows::WindowsModule),
        Box::new(slabs::SlabsModule),
    ];
    let contributed = CONTRIBUTED_SOURCING_MODULES.lock().expect("sourcing contributed modules lock");
    modules.extend(contributed.iter().map(|module| Box::new(module.clone()) as Box<dyn SourcingModule>));
    modules
}

/// 🔎️ Looks up a single module by id.
pub fn module_for(module_id: &str) -> Option<Box<dyn SourcingModule>> {
    sourcing_modules().into_iter().find(|module| module.module_id() == module_id)
}
//#endregion 🔖️Modules

//#region 🔖️ModuleCatalogue
/// 🧩️ One module's typology + catalogue kinds — the pool window's filter chrome and the
/// `stockFromCatalogue` command both consume this (two consumers, hence engine-owned).
pub struct ModuleCatalogue {
    pub module_id: String,
    pub label: String,
    pub typology: TypologyNode,
    pub kinds: Vec<ObjectKind>,
}

pub fn available_modules() -> Vec<ModuleCatalogue> {
    sourcing_modules().into_iter().map(|module| ModuleCatalogue { module_id: module.module_id().to_string(), label: module.label().to_string(), typology: module.typology(), kinds: module.demo_kinds() }).collect()
}
//#endregion 🔖️ModuleCatalogue

//#region 🔖️GridLayout
/// 🔢️ Places item `index` of `count` total on a `ceil(sqrt(count))`-column grid, centered at the origin,
/// with `cell` spacing between slots — used to lay out the "all objects" 3D grid window.
pub fn grid_placement(count: usize, index: usize, cell: f64) -> (f64, f64) {
    if count == 0 {
        return (0.0, 0.0);
    }
    let columns = (count as f64).sqrt().ceil().max(1.0) as usize;
    let rows = count.div_ceil(columns);
    let column = index % columns;
    let row = index / columns;
    let x = (column as f64 - (columns as f64 - 1.0) * 0.5) * cell;
    let z = (row as f64 - (rows as f64 - 1.0) * 0.5) * cell;
    (x, z)
}

/// 📏️ The uniform scale factor that fits a recipe's largest dimension inside a `cell`-sized grid slot.
pub fn grid_scale(recipe: &GeometryRecipe, cell: f64) -> f64 {
    let extent = bounding_extent(recipe);
    if extent <= 0.0 {
        1.0
    } else {
        cell / extent
    }
}
//#endregion 🔖️GridLayout

//#region 🔖️Fixtures
/// 📄️ The demo-stock example, parsed once from `crate::artifacts::curate::dsl::DEMO_STOCK_TEXT` — the
/// source of truth for every "demo stock" call site (`setActiveExample`, `initial_snapshot`, tests).
pub fn default_document() -> CurateSnapshot {
    <CurateSnapshot as store::ArtifactDsl>::parse_dsl(crate::artifacts::curate::dsl::DEMO_STOCK_TEXT).unwrap_or_default()
}

/// 📄️ The empty-curation example, parsed once from
/// `crate::artifacts::curate::dsl::EMPTY_CURATION_TEXT`.
pub fn empty_document() -> CurateSnapshot {
    <CurateSnapshot as store::ArtifactDsl>::parse_dsl(crate::artifacts::curate::dsl::EMPTY_CURATION_TEXT).unwrap_or_default()
}
//#endregion 🔖️Fixtures

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_document() -> CurateSnapshot {
        CurateSnapshot { stock: sourcing_modules().iter().flat_map(|module| module.demo_kinds()).collect(), ..Default::default() }
    }

    #[test]
    fn filtered_stock_matches_query() {
        let document = sample_document();
        let filters = Filters { query: "glulam".into(), ..Default::default() };
        let filtered = filtered_stock(&document, &filters);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, "beam-glulam-gl24h");
    }

    #[test]
    fn filtered_stock_matches_module() {
        let document = sample_document();
        let filters = Filters { module_ids: vec!["slabs".into()], ..Default::default() };
        let filtered = filtered_stock(&document, &filters);
        assert!(filtered.iter().all(|kind| kind.module_id == "slabs"));
        assert_eq!(filtered.len(), 3);
    }

    #[test]
    fn filtered_stock_matches_typology_prefix() {
        let document = sample_document();
        let filters = Filters { typology_path: vec!["beams".into(), "steel".into()], ..Default::default() };
        let filtered = filtered_stock(&document, &filters);
        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().all(|kind| kind.typology_path.starts_with(&["beams".to_string(), "steel".to_string()])));
    }

    #[test]
    fn filtered_stock_matches_min_availability() {
        let document = sample_document();
        let filters = Filters { min_availability: 20, ..Default::default() };
        let filtered = filtered_stock(&document, &filters);
        assert!(filtered.iter().all(|kind| kind.availability >= 20));
        assert!(!filtered.is_empty());
    }

    #[test]
    fn sourcing_curate_io_declares_the_catalog_out_port_alongside_the_implicit_document_ports() {
        let io = sourcing_curate_io();
        assert_eq!(io.document_schema, SOURCING_CURATE_SCHEMA);
        let ports = io.all_ports();
        assert_eq!(ports.len(), 3, "document:in, document:out, catalog:out");
        let catalog_out = ports.iter().find(|port| port.id == "catalog:out").expect("catalog:out port declared");
        assert_eq!(catalog_out.kind_id.as_deref(), Some("kit.catalog"));
        assert_eq!(catalog_out.media_type.class, semio_framework_plugin::MediaClass::Kit);
        assert_eq!(catalog_out.media_type.form, semio_framework_plugin::MediaForm::Type);
    }

    #[test]
    fn sourcing_catalog_fragment_maps_stock_into_the_puzzle3d_kit_catalog_shape() {
        let document = sample_document();
        let fragment = sourcing_catalog_fragment(&document);
        assert_eq!(fragment["schema"], "manifest");
        let object_kinds = fragment["objectKinds"].as_array().expect("objectKinds array");
        assert_eq!(object_kinds.len(), document.stock.len());
        assert_eq!(object_kinds[0]["id"], document.stock[0].id);
        assert_eq!(object_kinds[0]["meshUrl"], Value::Null);
        assert!(object_kinds[0]["vortices"].as_array().unwrap().is_empty());
        assert!(fragment["vortexKinds"].as_array().unwrap().is_empty());
        assert!(fragment["cableKinds"].as_array().unwrap().is_empty());
        assert!(fragment["attractionKinds"].as_array().unwrap().is_empty());
        assert!(fragment["kindCompatibility"].as_array().unwrap().is_empty());
    }

    #[test]
    fn curate_delta_clamps_to_availability_and_zero_floor() {
        let mut document = sample_document();
        curate_delta(&mut document, "beam-steel-hea160", 100);
        assert_eq!(curated_count(&document, "beam-steel-hea160"), 8);
        curate_delta(&mut document, "beam-steel-hea160", -1000);
        assert_eq!(curated_count(&document, "beam-steel-hea160"), 0);
        assert!(document.curated.is_empty());
    }

    #[test]
    fn curate_delta_unknown_object_is_noop() {
        let mut document = sample_document();
        curate_delta(&mut document, "does-not-exist", 5);
        assert!(document.curated.is_empty());
    }

    #[test]
    fn curate_set_removes_entry_at_zero() {
        let mut document = sample_document();
        curate_set(&mut document, "slab-clt-160", 5);
        assert_eq!(curated_count(&document, "slab-clt-160"), 5);
        curate_set(&mut document, "slab-clt-160", 0);
        assert_eq!(curated_count(&document, "slab-clt-160"), 0);
        assert!(document.curated.is_empty());
    }

    #[test]
    fn typology_contains_and_flatten() {
        let module = beams::BeamsModule;
        let tree = module.typology();
        assert!(typology_contains(&tree, &["beams".into(), "steel".into(), "ipe".into()]));
        assert!(!typology_contains(&tree, &["beams".into(), "concrete".into()]));
        let flattened = typology_flatten(&tree);
        assert!(flattened.iter().any(|(path, _)| path == &vec!["beams".to_string(), "solid-timber".to_string(), "glulam".to_string()]));
    }

    fn assert_mesh_spec_is_valid(spec: &MeshDataSpec) {
        assert!(!spec.positions.is_empty());
        assert_eq!(spec.positions.len() % 3, 0);
        assert_eq!(spec.positions.len(), spec.normals.len());
        assert_eq!(spec.indices.len() % 3, 0);
        let vertex_count = (spec.positions.len() / 3) as u32;
        assert!(spec.indices.iter().all(|&i| i < vertex_count));
    }

    #[test]
    fn box_recipe_produces_valid_mesh() {
        assert_mesh_spec_is_valid(&mesh_spec_for(&GeometryRecipe::Box { width: 0.2, height: 0.4, depth: 6.0 }));
    }

    #[test]
    fn frame_recipe_concatenates_four_pieces_into_a_valid_mesh() {
        let spec = mesh_spec_for(&GeometryRecipe::Frame { width: 1.0, height: 1.2, depth: 0.08, profile: 0.08 });
        assert_mesh_spec_is_valid(&spec);
        let single_box = box_mesh_spec(1.0, 0.08, 0.08);
        assert_eq!(spec.positions.len(), single_box.positions.len() * 4);
        assert_eq!(spec.indices.len(), single_box.indices.len() * 4);
    }

    #[test]
    fn grid_placement_centers_around_origin() {
        let positions: Vec<(f64, f64)> = (0..9).map(|i| grid_placement(9, i, 2.0)).collect();
        let sum_x: f64 = positions.iter().map(|(x, _)| x).sum();
        let sum_z: f64 = positions.iter().map(|(_, z)| z).sum();
        assert!(sum_x.abs() < 1e-9);
        assert!(sum_z.abs() < 1e-9);
        let unique: std::collections::HashSet<(i64, i64)> = positions.iter().map(|(x, z)| ((x * 1000.0) as i64, (z * 1000.0) as i64)).collect();
        assert_eq!(unique.len(), 9);
    }

    #[test]
    fn grid_scale_normalizes_to_cell_size() {
        let recipe = GeometryRecipe::Box { width: 0.2, height: 0.4, depth: 6.0 };
        let scale = grid_scale(&recipe, 2.0);
        assert!((bounding_extent(&recipe) * scale - 2.0).abs() < 1e-9);
    }

    #[test]
    fn curate_document_dsl_round_trips_sample_and_empty() {
        store::os_store::test_support::assert_dsl_round_trip(&sample_document());
        store::os_store::test_support::assert_dsl_round_trip(&CurateSnapshot::default());
        store::os_store::test_support::assert_dsl_pack_equivalence(&sample_document());
        store::os_store::test_support::assert_dsl_pack_equivalence(&CurateSnapshot::default());
    }

    #[test]
    fn available_modules_tracks_contributed_modules() {
        sync_sourcing_module_contributions("[]");
        assert_eq!(available_modules().len(), 3);
        let beams = beams::BeamsModule;
        let entry = semio_framework::ProgramContributionEntry {
            plugin_id: "sourcing-module-beams".into(),
            topic_contribution: Some(semio_framework::TopicContribution::new(
                "sourcing.module",
                serde_json::json!({
                    "appId": SOURCING_CURATE_APP_ID,
                    "moduleId": beams.module_id(),
                    "label": beams.label(),
                    "iconId": "beam",
                    "typologyJson": serde_json::to_string(&beams.typology()).unwrap(),
                    "kindsJson": serde_json::to_string(&beams.demo_kinds()).unwrap(),
                }),
            )),
        };
        sync_sourcing_module_contributions(&serde_json::to_string(&vec![entry]).unwrap());
        let modules = available_modules();
        assert_eq!(modules.len(), 4);
        assert_eq!(modules[0].module_id, "beams");
        sync_sourcing_module_contributions("[]");
    }

    #[test]
    fn sync_sourcing_module_contributions_adds_hot_installed_modules() {
        use semio_framework::{ProgramContributionEntry, TopicContribution};
        let entry = ProgramContributionEntry {
            plugin_id: "sourcing-module-test".into(),
            topic_contribution: Some(TopicContribution::new(
                "sourcing.module",
                serde_json::json!({
                    "appId": "sourcing-curate",
                    "moduleId": "hot-test",
                    "label": "Hot Test",
                    "iconId": "box",
                    "typologyJson": serde_json::to_string(&TypologyNode::new("hot-test", "Hot Test", vec![])).unwrap(),
                    "kindsJson": "[]",
                }),
            )),
        };
        let json = serde_json::to_string(&vec![entry]).unwrap();
        sync_sourcing_module_contributions(&json);
        assert!(sourcing_modules().iter().any(|module| module.module_id() == "hot-test"));
        sync_sourcing_module_contributions("[]");
    }
}
//#endregion 🧪️Tests


//#region 🔖️ArtifactEngine
use crate::artifacts::curate::schema::CurateArtifact;

pub struct SourcingEngine {
    artifact: CurateArtifact,
    cached_snapshot: CurateSnapshot,
}

impl SourcingEngine {
    pub fn new(snapshot: CurateSnapshot) -> Self {
        Self {
            artifact: CurateArtifact::from_snapshot(snapshot.clone()),
            cached_snapshot: snapshot,
        }
    }
}
//#endregion 🔖️ArtifactEngine
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ArtifactBuilder, ComposerEntry, ComposedArtifact, ComposeError, Dialect, StandardId, SubsetId, ErasedComposeSource, IoPayload, IoConfidence, composer_entry_of};
    use crate::artifacts::curate::standards::v1::subsets::any::schema::CurateComposer as CurateAnyComposer;
    use crate::artifacts::curate::standards::v1::subsets::any::schema::CurateBuilder as CurateAnyBuilder;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    //#region 🔖️ExportEntries
    /// 🗄️ Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO W15: the typed registry (W11-W14) only ever grew
    /// IMPORT-direction entries (each composer's own `reads()`) -- nothing registers the REVERSE
    /// ("this domain artifact can be exported AS format Y"), because `ArtifactComposer` only models
    /// "produce my own snapshot." These entries wrap the artifact's EXISTING `🚪️io/📤️export/🧵️serializers`
    /// leaves (which already convert this artifact's snapshot straight to target-format bytes/text) as
    /// their own `ComposerEntry` rows: `writes` = the target format's dialect, `reads` = just this
    /// artifact's own dialect. `register_composer_entries` already inserts BOTH an Import key (target
    /// reads from us) and an Export key (we export to target) per entry, so no framework change was
    /// needed, only populating the missing direction. Generated by generators/w15_add_export_entries.py
    /// -- hand-validated pattern on note/json first (see that file's own tests), pilot kept as reference.
    const CURATE_DIALECT: Dialect = Dialect { artifact_kind: "s.curate", standard: StandardId("1"), subset: SubsetId("*") };
    const CURATE_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::curate::CurateSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == CURATE_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => CurateAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => CurateAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "CurateComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == CURATE_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::artifacts::curate::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "CurateComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_ZIP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };
    fn compose_export_zip(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::curate::io::export::serializers::artifacts::zip::v2_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_ZIP_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    fn compose_export_png(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::curate::io::export::serializers::artifacts::png::v1_2::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_PNG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::curate::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_STL_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };
    fn compose_export_stl(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::curate::io::export::serializers::artifacts::stl::v_ascii::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_STL_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_OBJ_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };
    fn compose_export_obj(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::curate::io::export::serializers::artifacts::obj::v3_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_OBJ_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    //#endregion 🔖️ExportEntries


    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![
            composer_entry_of::<CurateAnyComposer>(),
            ComposerEntry { writes: EXPORT_ZIP_DIALECT, reads: &[CURATE_DIALECT], compose: compose_export_zip },
            ComposerEntry { writes: EXPORT_PNG_DIALECT, reads: &[CURATE_DIALECT], compose: compose_export_png },
            ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[CURATE_DIALECT], compose: compose_export_json },
            ComposerEntry { writes: EXPORT_STL_DIALECT, reads: &[CURATE_DIALECT], compose: compose_export_stl },
            ComposerEntry { writes: EXPORT_OBJ_DIALECT, reads: &[CURATE_DIALECT], compose: compose_export_obj },
        ]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
