//! 🧬️ Curate artifact schema — every field of the artifact with its state class.

use crate::artifacts::curate::{CurateSnapshot, CuratedItem, Filters, GeometryRecipe, ObjectKind, ObjectKindExtra, SourcingMutation};
use schema::ArtifactSchema;
use semio_framework::parse_contributions;
use semio_framework_dispatch_macros::{dyn_enum, dyn_enum_close};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

//#region 🔖️Artifact
/// 🧬️ Full curate artifact state across the artifact, presence and config lanes. `catalog`/
/// `stock_extra` mirror `CurateSnapshot`'s own composed-child split (see that struct's doc comment).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.sourcing.curate")]
pub struct CurateArtifact {
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.kit")]
    pub catalog: store::ArtifactChild<SemioKitSnapshot>,
    #[state(artifact)]
    pub stock_extra: Vec<ObjectKindExtra>,
    #[state(artifact)]
    pub curated: Vec<CuratedItem>,
    #[state(config)]
    pub filters: Filters,
    #[state(config)]
    pub locale: String,
    #[state(config)]
    pub contributions_json: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
fn default_contributions_json() -> String {
    "[]".into()
}

impl Default for CurateArtifact {
    fn default() -> Self {
        Self { catalog: crate::artifacts::curate::catalog_child_handle(&[]), stock_extra: Vec::new(), curated: Vec::new(), filters: Filters::default(), locale: "en-US".into(), contributions_json: default_contributions_json() }
    }
}

impl CurateArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> CurateSnapshot {
        CurateSnapshot { catalog: self.catalog.clone(), stock_extra: self.stock_extra.clone(), curated: self.curated.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: CurateSnapshot) -> Self {
        Self { catalog: snapshot.catalog, stock_extra: snapshot.stock_extra, curated: snapshot.curated, ..Self::default() }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: CurateSnapshot) {
        self.catalog = snapshot.catalog;
        self.stock_extra = snapshot.stock_extra;
        self.curated = snapshot.curated;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.sourcing.curate` — twenty handcrafted schema leaves.
pub fn curate_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.sourcing.curate",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

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
/// (two consumers, hence schema-owned per this taxonomy's "more than one consumer" rule).
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
pub fn filtered_stock(document: &CurateSnapshot, filters: &Filters) -> Vec<ObjectKind> {
    crate::artifacts::curate::stock_of(document)
        .into_iter()
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
    apply_curation_decision(document, curation_decision_for_delta(document, object_id, delta));
}

/// 🎯️ Sets the curated count for `object_id` directly, clamped to `0..=availability`; removes the
/// entry when the count is 0. Silently no-operations if `object_id` isn't in the stock.
pub fn curate_set(document: &mut CurateSnapshot, object_id: &str, count: u32) {
    apply_curation_decision(document, curation_decision_for_set(document, object_id, count));
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️CurationDecisions
/// 🧭️ What a curated-count adjustment resolves to against a given base document — the single
/// source of truth both the mutating `curate_delta`/`curate_set` helpers above (schema-level
/// fixtures/tests) and the `crate::apps::curate::commands::curation` handlers (which must emit a
/// REAL `SourcingMutation` rather than mutate a document clone, now that whole-document replace is
/// banned from the mutation enum) fold through.
#[derive(Clone, Debug, PartialEq)]
pub enum CurationDecision {
    NoOp,
    Create(CuratedItem),
    ChangeCount { object_id: String, new_count: u32 },
    Delete { object_id: String },
}

/// 🎯️ Resolves a relative count adjustment against `document`'s CURRENT `curated`/`stock`, clamped
/// to `0..=availability`. Unknown `object_id` (absent from stock) resolves to `NoOp`.
pub fn curation_decision_for_delta(document: &CurateSnapshot, object_id: &str, delta: i64) -> CurationDecision {
    // 🔎️ `availability` lives on `stock_extra` (the sourcing-owned overflow half) — no need to
    // resolve the composed `catalog` child just to clamp a count, so this reads `stock_extra`
    // directly rather than going through `stock_of`'s full reassembly.
    let Some(extra) = document.stock_extra.iter().find(|extra| extra.id == object_id) else { return CurationDecision::NoOp };
    let next = (curated_count(document, object_id) as i64 + delta).clamp(0, extra.availability as i64) as u32;
    curation_decision_for_set(document, object_id, next)
}

/// 🎯️ Resolves an absolute count set against `document`, clamped to `0..=availability`. Unknown
/// `object_id` (absent from stock) resolves to `NoOp`.
pub fn curation_decision_for_set(document: &CurateSnapshot, object_id: &str, count: u32) -> CurationDecision {
    let Some(extra) = document.stock_extra.iter().find(|extra| extra.id == object_id) else { return CurationDecision::NoOp };
    let clamped = count.min(extra.availability);
    match document.curated.iter().find(|item| item.object_id == object_id) {
        Some(_item) if clamped == 0 => CurationDecision::Delete { object_id: object_id.to_string() },
        Some(item) if item.count == clamped => CurationDecision::NoOp,
        Some(_) => CurationDecision::ChangeCount { object_id: object_id.to_string(), new_count: clamped },
        None if clamped > 0 => CurationDecision::Create(CuratedItem { object_id: object_id.to_string(), count: clamped }),
        None => CurationDecision::NoOp,
    }
}

/// ▶️ Mutates `document.curated` in place to reflect `decision` — the shared apply step behind the
/// mutating `curate_delta`/`curate_set` schema helpers only; command handlers turn a
/// `CurationDecision` into a real `SourcingMutation` instead of calling this.
fn apply_curation_decision(document: &mut CurateSnapshot, decision: CurationDecision) {
    match decision {
        CurationDecision::NoOp => {}
        CurationDecision::Create(item) => document.curated.push(item),
        CurationDecision::ChangeCount { object_id, new_count } => {
            if let Some(item) = document.curated.iter_mut().find(|item| item.object_id == object_id) {
                item.count = new_count;
            }
        }
        CurationDecision::Delete { object_id } => document.curated.retain(|item| item.object_id != object_id),
    }
}
//#endregion 🔖️CurationDecisions

//#region 🔖️Modules
/// 🧩️ A sourcing module composes a typology subtree, demo catalogue kinds, and preview meshing for one
/// object family (e.g. beams, windows, slabs) — closed set, enum-dispatched via `SourcingModules` below
/// (O1/R11: closed set ⇒ `dyn_enum_close!`, not a trait object).
#[dyn_enum]
pub trait SourcingModule {
    fn module_id(&self) -> &str;
    fn label(&self) -> &str;
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
        fn module_id(&self) -> &str {
            "beams"
        }
        fn label(&self) -> &str {
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
        fn module_id(&self) -> &str {
            "windows"
        }
        fn label(&self) -> &str {
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
        fn module_id(&self) -> &str {
            "slabs"
        }
        fn label(&self) -> &str {
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

/// 🧩️ One hot-installed sourcing module deserialized from the `"sourcing.module"` topic contribution.
/// `pub` (not crate-private) solely so it can sit as a variant payload in the `pub enum SourcingModules`
/// below without tripping the `private_interfaces` lint — construction stays internal to this module.
#[derive(Clone)]
pub struct ContributedSourcingModule {
    module_id: String,
    label: String,
    typology: TypologyNode,
    kinds: Vec<ObjectKind>,
}

impl SourcingModule for ContributedSourcingModule {
    fn module_id(&self) -> &str {
        &self.module_id
    }

    fn label(&self) -> &str {
        &self.label
    }

    fn typology(&self) -> TypologyNode {
        self.typology.clone()
    }

    fn demo_kinds(&self) -> Vec<ObjectKind> {
        self.kinds.clone()
    }
}

/// 🔀️ The closed set of `SourcingModule` implementors, enum-dispatched (O1 — no `Box<dyn SourcingModule>`).
/// `dyn_enum_close!` generates the enum, `From<Variant>` impls, and the delegating `impl SourcingModule`.
dyn_enum_close! {
    pub enum SourcingModules: SourcingModule {
        Beams(beams::BeamsModule),
        Windows(windows::WindowsModule),
        Slabs(slabs::SlabsModule),
        Contributed(ContributedSourcingModule),
    }
}

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

pub(crate) const SOURCING_JSON_MAX_BYTES: usize = 256 * 1024;
pub(crate) const SOURCING_JSON_MAX_DEPTH: usize = 32;
pub(crate) const SOURCING_JSON_MAX_ITEMS: usize = 4 * 1024;
pub(crate) const SOURCING_JSON_MAX_STRING_BYTES: usize = 4 * 1024;

pub(crate) fn sourcing_json_envelope_is_bounded(input: &str) -> bool {
    if input.len() > SOURCING_JSON_MAX_BYTES {
        return false;
    }
    let mut depth = 0usize;
    let mut items = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    let mut string_bytes = 0usize;
    let mut in_scalar = false;
    for byte in input.bytes() {
        if in_string {
            if escaped {
                string_bytes = string_bytes.saturating_add(1);
                escaped = false;
            } else if byte == b'\\' {
                string_bytes = string_bytes.saturating_add(1);
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            } else {
                string_bytes = string_bytes.saturating_add(1);
            }
            if string_bytes > SOURCING_JSON_MAX_STRING_BYTES {
                return false;
            }
            continue;
        }
        if in_scalar {
            if byte.is_ascii_whitespace() || matches!(byte, b',' | b']' | b'}') {
                in_scalar = false;
            } else {
                continue;
            }
        }
        match byte {
            b'"' => {
                items = items.saturating_add(1);
                in_string = true;
                string_bytes = 0;
            }
            b'{' | b'[' => {
                items = items.saturating_add(1);
                depth = depth.saturating_add(1);
                if depth > SOURCING_JSON_MAX_DEPTH {
                    return false;
                }
            }
            b'}' | b']' => {
                let Some(next) = depth.checked_sub(1) else {
                    return false;
                };
                depth = next;
            }
            b':' | b',' => {}
            byte if byte.is_ascii_whitespace() => {}
            _ => {
                items = items.saturating_add(1);
                in_scalar = true;
            }
        }
        if items > SOURCING_JSON_MAX_ITEMS {
            return false;
        }
    }
    !in_string && !escaped && depth == 0
}

fn contributed_sourcing_modules(contributions_json: &str) -> Vec<ContributedSourcingModule> {
    if !sourcing_json_envelope_is_bounded(contributions_json) {
        return Vec::new();
    }
    let mut modules = Vec::new();
    for entry in parse_contributions(contributions_json) {
        let Some(payload) = entry.topic_contribution.as_ref().filter(|topic| topic.topic == "sourcing.module").and_then(|topic| topic.decode::<SourcingModuleTopicPayload>().ok()) else {
            continue;
        };
        let (app_id, module_id, label, typology_json, kinds_json) = (payload.app_id, payload.module_id, payload.label, payload.typology_json, payload.kinds_json);
        if app_id != SOURCING_CURATE_APP_ID {
            continue;
        }
        if !sourcing_json_envelope_is_bounded(&typology_json) || !sourcing_json_envelope_is_bounded(&kinds_json) {
            continue;
        }
        let Ok(typology) = serde_json::from_str::<TypologyNode>(&typology_json) else {
            continue;
        };
        let Ok(kinds) = serde_json::from_str::<Vec<ObjectKind>>(&kinds_json) else {
            continue;
        };
        if kinds.len() > SOURCING_JSON_MAX_ITEMS {
            continue;
        }
        modules.push(ContributedSourcingModule { module_id, label, typology, kinds });
    }
    modules
}

/// 🧩️ Every sourcing module known to this crate, in stable order.
pub fn sourcing_modules(contributions_json: &str) -> Vec<SourcingModules> {
    let mut modules: Vec<SourcingModules> = vec![beams::BeamsModule.into(), windows::WindowsModule.into(), slabs::SlabsModule.into()];
    modules.extend(contributed_sourcing_modules(contributions_json).into_iter().map(SourcingModules::from));
    modules
}

/// 🔎️ Looks up a single module by id.
pub fn module_for(contributions_json: &str, module_id: &str) -> Option<SourcingModules> {
    sourcing_modules(contributions_json).into_iter().find(|module| module.module_id() == module_id)
}
//#endregion 🔖️Modules

//#region 🔖️ModuleCatalogue
/// 🧩️ One module's typology + catalogue kinds — the pool window's filter chrome and the
/// `stockFromCatalogue` command both consume this (two consumers, hence schema-owned).
pub struct ModuleCatalogue {
    pub module_id: String,
    pub label: String,
    pub typology: TypologyNode,
    pub kinds: Vec<ObjectKind>,
}

pub fn available_modules(contributions_json: &str) -> Vec<ModuleCatalogue> {
    sourcing_modules(contributions_json).into_iter().map(|module| ModuleCatalogue { module_id: module.module_id().to_string(), label: module.label().to_string(), typology: module.typology(), kinds: module.demo_kinds() }).collect()
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
/// 🧩️ The canonical demo-stock catalogue — every built-in module's demo kinds, in module-registration
/// order. Single source of truth for `default_document`'s catalog content and every test fixture that
/// used to independently duplicate `sourcing_modules("[]").iter().flat_map(...)`.
pub fn demo_stock() -> Vec<ObjectKind> {
    sourcing_modules("[]").iter().flat_map(|module| module.demo_kinds()).collect()
}

/// 📄️ The demo-stock example, parsed once from `crate::artifacts::curate::dsl::DEMO_STOCK_TEXT` — the
/// source of truth for every "demo stock" call site (`setActiveExample`, `initial_snapshot`, tests).
/// The fixture's persisted `catalog` handle is content-addressed from `demo_stock()` (see
/// `crate::artifacts::curate::catalog_child_handle`) — re-deriving the same stock here and seeding the
/// working-scene cache with it resolves that exact handle, since a composed child is a handle only,
/// never inline content, in the persisted DSL text itself.
pub fn default_document() -> CurateSnapshot {
    crate::artifacts::curate::validate_catalog_payload(&demo_stock());
    <CurateSnapshot as store::ArtifactDsl>::parse_dsl(crate::artifacts::curate::dsl::DEMO_STOCK_TEXT).expect("authored demo stock must match the curate schema")
}

/// 📄️ The empty-curation example, parsed once from
/// `crate::artifacts::curate::dsl::EMPTY_CURATION_TEXT` — empty stock, so its `catalog` handle is the
/// same content-addressed empty-catalog handle `CurateSnapshot::default()` mints.
pub fn empty_document() -> CurateSnapshot {
    crate::artifacts::curate::validate_catalog_payload(&[]);
    <CurateSnapshot as store::ArtifactDsl>::parse_dsl(crate::artifacts::curate::dsl::EMPTY_CURATION_TEXT).expect("authored empty curation must match the curate schema")
}
//#endregion 🔖️Fixtures

//#region 🏗️Construction
/// 🏗️ W1-C's generic `SnapshotBuilder<Snapshot, Mutation>` (design.md §5 step 3) — replaces the
/// deleted `derive_artifact_facets!`-generated `CurateBuilder`/`CurateAnalyzer`/`CurateComposer`
/// cluster (and the hand-rolled `CurateBuilderConstruction`/`CurateAnalyzerAnalysis` it wrapped)
/// outright: construction is a plain snapshot+mutation build (no custom analysis/composition logic
/// this subset needs beyond the ordinary `Mutation`/`MutationDiff` algebra), so the trivial-subset
/// shape applies verbatim.
pub type Construction = semio_framework_plugin::app::SnapshotBuilder<CurateSnapshot, SourcingMutation>;
//#endregion 🏗️Construction

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_document() -> CurateSnapshot {
        crate::artifacts::curate::curate_snapshot_from_stock(demo_stock(), Vec::new())
    }

    #[semio_framework_async_macros::async_test]
    async fn filtered_stock_matches_query() {
        let document = sample_document();
        let filters = Filters { query: "glulam".into(), ..Default::default() };
        let filtered = filtered_stock(&document, &filters);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, "beam-glulam-gl24h");
    }

    #[semio_framework_async_macros::async_test]
    async fn filtered_stock_matches_module() {
        let document = sample_document();
        let filters = Filters { module_ids: vec!["slabs".into()], ..Default::default() };
        let filtered = filtered_stock(&document, &filters);
        assert!(filtered.iter().all(|kind| kind.module_id == "slabs"));
        assert_eq!(filtered.len(), 3);
    }

    #[semio_framework_async_macros::async_test]
    async fn filtered_stock_matches_typology_prefix() {
        let document = sample_document();
        let filters = Filters { typology_path: vec!["beams".into(), "steel".into()], ..Default::default() };
        let filtered = filtered_stock(&document, &filters);
        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().all(|kind| kind.typology_path.starts_with(&["beams".to_string(), "steel".to_string()])));
    }

    #[semio_framework_async_macros::async_test]
    async fn filtered_stock_matches_min_availability() {
        let document = sample_document();
        let filters = Filters { min_availability: 20, ..Default::default() };
        let filtered = filtered_stock(&document, &filters);
        assert!(filtered.iter().all(|kind| kind.availability >= 20));
        assert!(!filtered.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn curate_delta_clamps_to_availability_and_zero_floor() {
        let mut document = sample_document();
        curate_delta(&mut document, "beam-steel-hea160", 100);
        assert_eq!(curated_count(&document, "beam-steel-hea160"), 8);
        curate_delta(&mut document, "beam-steel-hea160", -1000);
        assert_eq!(curated_count(&document, "beam-steel-hea160"), 0);
        assert!(document.curated.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn curate_delta_unknown_object_is_noop() {
        let mut document = sample_document();
        curate_delta(&mut document, "does-not-exist", 5);
        assert!(document.curated.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn curate_set_removes_entry_at_zero() {
        let mut document = sample_document();
        curate_set(&mut document, "slab-clt-160", 5);
        assert_eq!(curated_count(&document, "slab-clt-160"), 5);
        curate_set(&mut document, "slab-clt-160", 0);
        assert_eq!(curated_count(&document, "slab-clt-160"), 0);
        assert!(document.curated.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn typology_contains_and_flatten() {
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

    #[semio_framework_async_macros::async_test]
    async fn box_recipe_produces_valid_mesh() {
        assert_mesh_spec_is_valid(&mesh_spec_for(&GeometryRecipe::Box { width: 0.2, height: 0.4, depth: 6.0 }));
    }

    #[semio_framework_async_macros::async_test]
    async fn frame_recipe_concatenates_four_pieces_into_a_valid_mesh() {
        let spec = mesh_spec_for(&GeometryRecipe::Frame { width: 1.0, height: 1.2, depth: 0.08, profile: 0.08 });
        assert_mesh_spec_is_valid(&spec);
        let single_box = box_mesh_spec(1.0, 0.08, 0.08);
        assert_eq!(spec.positions.len(), single_box.positions.len() * 4);
        assert_eq!(spec.indices.len(), single_box.indices.len() * 4);
    }

    #[semio_framework_async_macros::async_test]
    async fn grid_placement_centers_around_origin() {
        let positions: Vec<(f64, f64)> = (0..9).map(|i| grid_placement(9, i, 2.0)).collect();
        let sum_x: f64 = positions.iter().map(|(x, _)| x).sum();
        let sum_z: f64 = positions.iter().map(|(_, z)| z).sum();
        assert!(sum_x.abs() < 1e-9);
        assert!(sum_z.abs() < 1e-9);
        let unique: std::collections::HashSet<(i64, i64)> = positions.iter().map(|(x, z)| ((x * 1000.0) as i64, (z * 1000.0) as i64)).collect();
        assert_eq!(unique.len(), 9);
    }

    #[semio_framework_async_macros::async_test]
    async fn grid_scale_normalizes_to_cell_size() {
        let recipe = GeometryRecipe::Box { width: 0.2, height: 0.4, depth: 6.0 };
        let scale = grid_scale(&recipe, 2.0);
        assert!((bounding_extent(&recipe) * scale - 2.0).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn curate_document_dsl_round_trips_sample_and_empty() {
        store::os_store::test_support::assert_dsl_round_trip(&sample_document());
        store::os_store::test_support::assert_dsl_round_trip(&CurateSnapshot::default());
        store::os_store::test_support::assert_dsl_pack_equivalence(&sample_document());
        store::os_store::test_support::assert_dsl_pack_equivalence(&CurateSnapshot::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn available_modules_tracks_contributed_modules() {
        assert_eq!(available_modules("[]").len(), 3);
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
        let contributions_json = serde_json::to_string(&vec![entry]).unwrap();
        let modules = available_modules(&contributions_json);
        assert_eq!(modules.len(), 4);
        assert_eq!(modules[0].module_id, "beams");
    }

    #[semio_framework_async_macros::async_test]
    async fn sourcing_module_contributions_are_configuration_owned() {
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
        assert!(sourcing_modules(&json).iter().any(|module| module.module_id() == "hot-test"));
        assert!(!sourcing_modules("[]").iter().any(|module| module.module_id() == "hot-test"));
    }

    #[semio_framework_async_macros::async_test]
    async fn sourcing_contribution_envelope_rejects_depth_string_and_cardinality_plus_one_before_parse() {
        let depth_plus_one = format!("{}0{}", "[".repeat(SOURCING_JSON_MAX_DEPTH + 1), "]".repeat(SOURCING_JSON_MAX_DEPTH + 1));
        assert!(!sourcing_json_envelope_is_bounded(&depth_plus_one));
        assert_eq!(sourcing_modules(&depth_plus_one).len(), 3, "invalid contribution envelope installs nothing");

        let string_plus_one = format!("\"{}\"", "x".repeat(SOURCING_JSON_MAX_STRING_BYTES + 1));
        assert!(!sourcing_json_envelope_is_bounded(&string_plus_one));
        let items_plus_one = format!("[{}]", vec!["0"; SOURCING_JSON_MAX_ITEMS].join(","));
        assert!(!sourcing_json_envelope_is_bounded(&items_plus_one));
    }
}
//#endregion 🧪️Tests
