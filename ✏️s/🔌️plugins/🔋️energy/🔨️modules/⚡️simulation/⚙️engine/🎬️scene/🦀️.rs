//! 🎬️ Energy model — the ONE pure `Model -> World3d scene payload` builder, shared by the editor's
//! and the viewer's `🧊️model` windows. It lives here, not under either surface, because a viewer may
//! never import from the sibling mutation-capable surface (`policyViewerPurityBreaches`) and the two
//! windows must show byte-identical geometry.
//!
//! 🧭️ Conventions (see `crate::geometry` and `crate::bestest`'s docstring): right-handed, **z up**,
//! every polygon wound counter-clockwise seen from OUTSIDE, so the Newell normal points away from
//! the zone interior. Units are meters throughout.
//!
//! 🎨️ Colour is baked PER VERTEX into `meshes_json`. The react host
//! (`🌐️World3dHost/🟦️.tsx`'s `PaintTexturedMesh`) renders a data mesh with `side: DoubleSide` and,
//! when the geometry carries a `color` attribute, forces the material colour to white and the
//! emissive to black so the baked colours show through UNMODIFIED — which also means the host's own
//! selected/hovered paint no longer tints such a mesh. Selection and hover tint therefore have to be
//! baked here too; the per-instance `selected`/`hovered` flags are still published so the host's
//! outline/chrome stays correct.
//!
//! 🚫️ There is no alpha lane for a data mesh (`transparent` follows the style's own opacity, never
//! the payload), so a window renders as a solid light blue rather than a translucent one.

use crate::model::{EntityId, Fenestration, Model, Surface, SurfaceClass};
use serde_json::{json, Value};
use std::collections::HashMap;

//#region 🔖️Constants
/// 🪟️ How far a fenestration polygon is pushed along its host surface's outward normal so it never
/// z-fights with the wall it sits in. 5 mm — below any real construction thickness, far above the
/// depth buffer's resolution at building scale.
pub const ENERGY_SCENE_WINDOW_OFFSET_M: f64 = 0.005;

/// 🎥️ `world3d_fit_json` padding — the same 1.25 cad/puzzle3d use.
pub const ENERGY_SCENE_FIT_PADDING: f64 = 1.25;

/// 🎨️ Highlight a selected entity is mixed towards — the framework's own `primary` blue.
pub const ENERGY_SCENE_SELECTED_COLOR: [f64; 3] = [0.231, 0.510, 0.965];
/// 🎨️ How far towards [`ENERGY_SCENE_SELECTED_COLOR`] a selected entity is mixed.
pub const ENERGY_SCENE_SELECTED_MIX: f64 = 0.65;
/// 🎨️ How far towards white a hovered entity is lightened.
pub const ENERGY_SCENE_HOVERED_MIX: f64 = 0.25;

/// 🎨️ Per-`SurfaceClass` base swatch — a warm light grey envelope, darker roofs, mid floors, with
/// the unlit/opaque classes greyed down so the exterior envelope reads first.
pub const ENERGY_SCENE_EXTERIOR_WALL_COLOR: [f64; 3] = [0.839, 0.812, 0.769];
pub const ENERGY_SCENE_INTERIOR_WALL_COLOR: [f64; 3] = [0.749, 0.733, 0.706];
pub const ENERGY_SCENE_ROOF_COLOR: [f64; 3] = [0.478, 0.412, 0.376];
pub const ENERGY_SCENE_CEILING_COLOR: [f64; 3] = [0.647, 0.616, 0.580];
pub const ENERGY_SCENE_FLOOR_COLOR: [f64; 3] = [0.400, 0.400, 0.412];
pub const ENERGY_SCENE_INTERZONE_COLOR: [f64; 3] = [0.596, 0.643, 0.690];
pub const ENERGY_SCENE_ADIABATIC_COLOR: [f64; 3] = [0.706, 0.667, 0.596];
pub const ENERGY_SCENE_GROUND_COLOR: [f64; 3] = [0.376, 0.353, 0.322];
/// 🪟️ Glazing swatch — a light blue, opaque (see the module docstring's alpha note).
pub const ENERGY_SCENE_FENESTRATION_COLOR: [f64; 3] = [0.537, 0.745, 0.898];
/// 🌳️ Shading swatch — a muted green that never reads as envelope.
pub const ENERGY_SCENE_SHADING_COLOR: [f64; 3] = [0.518, 0.600, 0.478];

/// 🪪️ Mesh-id prefixes. The INSTANCE id is always the bare entity id (see
/// [`energy_scene_target_id`]); only the mesh id is namespaced, because a mesh id is private to the
/// payload while an instance id is the interaction vocabulary the tree and the inspector share.
pub const ENERGY_SCENE_SURFACE_MESH_PREFIX: &str = "energy-surface-";
pub const ENERGY_SCENE_FENESTRATION_MESH_PREFIX: &str = "energy-window-";
pub const ENERGY_SCENE_SHADING_MESH_PREFIX: &str = "energy-shading-";

/// 🏷️ `objectKind` published per instance, so a host-side kind filter/legend can group by family.
pub const ENERGY_SCENE_OBJECT_KIND_SURFACE: &str = "surface";
pub const ENERGY_SCENE_OBJECT_KIND_FENESTRATION: &str = "fenestration";
pub const ENERGY_SCENE_OBJECT_KIND_SHADING: &str = "shading";
//#endregion 🔖️Constants

//#region 🔖️Identity
/// 🪪️ One entity's scene target id — the RAW `EntityId`, the same vocabulary
/// `crate::editor::model::interaction::energy_target_id` spells for the tree panel and the
/// inspector. Restated here (rather than imported) because this module is also the viewer's, and a
/// viewer may not import through `✏️editor`; the editor window's unit tests assert the two agree.
pub fn energy_scene_target_id(id: EntityId) -> String {
    id.0.to_string()
}
//#endregion 🔖️Identity

//#region 🔖️Style
/// 🎨️ Everything one render knows about paint: which entities are picked, which are pointed at, and
/// an optional per-surface result colour that REPLACES the class swatch (lane D's results overlay).
#[derive(Clone, Copy, Debug, Default)]
pub struct EnergySceneStyle<'a> {
    pub selected_ids: &'a [String],
    pub hovered_ids: &'a [String],
    pub overlay: Option<&'a HashMap<u32, [f64; 3]>>,
}

impl EnergySceneStyle<'_> {
    fn is_selected(&self, id: &str) -> bool {
        self.selected_ids.iter().any(|entry| entry == id)
    }

    fn is_hovered(&self, id: &str) -> bool {
        self.hovered_ids.iter().any(|entry| entry == id)
    }

    /// 🎨️ Base colour after the results overlay, then the selection/hover tint. The overlay is keyed
    /// by the raw entity id so lane D never has to know this module's colour table.
    fn paint(&self, entity: EntityId, target_id: &str, base: [f64; 3]) -> [f64; 3] {
        let mut color = self.overlay.and_then(|map| map.get(&entity.0).copied()).unwrap_or(base);
        if self.is_selected(target_id) {
            color = mix(color, ENERGY_SCENE_SELECTED_COLOR, ENERGY_SCENE_SELECTED_MIX);
        } else if self.is_hovered(target_id) {
            color = mix(color, [1.0, 1.0, 1.0], ENERGY_SCENE_HOVERED_MIX);
        }
        color
    }
}

/// 🎨️ Linear blend, clamped into the unit cube so a hand-authored overlay colour can never emit a
/// non-renderable component.
fn mix(from: [f64; 3], to: [f64; 3], t: f64) -> [f64; 3] {
    let t = t.clamp(0.0, 1.0);
    [(from[0] + (to[0] - from[0]) * t).clamp(0.0, 1.0), (from[1] + (to[1] - from[1]) * t).clamp(0.0, 1.0), (from[2] + (to[2] - from[2]) * t).clamp(0.0, 1.0)]
}

/// 🎨️ The fixed swatch of one surface class.
pub fn surface_class_color(class: SurfaceClass) -> [f64; 3] {
    match class {
        SurfaceClass::ExteriorWall => ENERGY_SCENE_EXTERIOR_WALL_COLOR,
        SurfaceClass::InteriorWall => ENERGY_SCENE_INTERIOR_WALL_COLOR,
        SurfaceClass::Roof => ENERGY_SCENE_ROOF_COLOR,
        SurfaceClass::Ceiling => ENERGY_SCENE_CEILING_COLOR,
        SurfaceClass::Floor => ENERGY_SCENE_FLOOR_COLOR,
        SurfaceClass::Interzone => ENERGY_SCENE_INTERZONE_COLOR,
        SurfaceClass::Adiabatic => ENERGY_SCENE_ADIABATIC_COLOR,
        SurfaceClass::Ground => ENERGY_SCENE_GROUND_COLOR,
    }
}
//#endregion 🔖️Style

//#region 🔖️Geometry
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn normalized(a: [f64; 3]) -> Option<[f64; 3]> {
    let length = dot(a, a).sqrt();
    if length <= 1e-12 {
        None
    } else {
        Some([a[0] / length, a[1] / length, a[2] / length])
    }
}

/// 🪟️ The RENDERED polygon of window `index` of `count` on `host`: exactly the polygon the engine
/// shades (`precompute::fenestration_polygon_in_bay` — the window's own `vertices_m` when it carries
/// one, otherwise the area/height/sill rectangle placed in its bay, the same derivation the epJSON
/// exporter's `aperture_rectangle` writes), lifted [`ENERGY_SCENE_WINDOW_OFFSET_M`] along the host's
/// outward normal so the glazing never z-fights the wall it sits in.
///
/// Delegating keeps ONE source of truth for where a window is: what the viewport draws and what the
/// solar model shades can never drift apart.
pub fn fenestration_polygon(host: &Surface, window: &Fenestration, index: usize, count: usize) -> Option<Vec<[f64; 3]>> {
    let polygon = crate::precompute::fenestration_polygon_in_bay(host, window, index, count);
    lift_off_host(host, polygon)
}

/// 🪟️ The rendered polygon of `window` inside `model` — finds the host surface and the window's bay
/// among its siblings the way the engine does.
pub fn model_fenestration_polygon(model: &Model, window: &Fenestration) -> Option<Vec<[f64; 3]>> {
    let host = model.surfaces.iter().find(|surface| surface.id == window.surface_id)?;
    lift_off_host(host, crate::precompute::model_fenestration_polygon(model, window))
}

/// 🪟️ Pushes a coplanar polygon off its host plane along the host's outward normal. `None` for a
/// degenerate polygon or a degenerate host.
fn lift_off_host(host: &Surface, polygon: Vec<[f64; 3]>) -> Option<Vec<[f64; 3]>> {
    if polygon.len() < 3 {
        return None;
    }
    let normal = normalized(crate::geometry::polygon_normal(&host.vertices_m))?;
    let lift = [normal[0] * ENERGY_SCENE_WINDOW_OFFSET_M, normal[1] * ENERGY_SCENE_WINDOW_OFFSET_M, normal[2] * ENERGY_SCENE_WINDOW_OFFSET_M];
    Some(polygon.into_iter().map(|corner| [corner[0] + lift[0], corner[1] + lift[1], corner[2] + lift[2]]).collect())
}

/// 🔺️ Fan-triangulate a planar polygon into the host's flat per-triangle-corner arrays, one flat
/// normal for the whole face. Fewer than three vertices emits nothing.
fn push_polygon(vertices: &[[f64; 3]], color: [f64; 3], positions: &mut Vec<f64>, normals: &mut Vec<f64>, colors: &mut Vec<f64>, indices: &mut Vec<u32>) {
    if vertices.len() < 3 {
        return;
    }
    let normal = normalized(crate::geometry::polygon_normal(vertices)).unwrap_or([0.0, 0.0, 1.0]);
    for corner in 1..vertices.len() - 1 {
        let triangle = [vertices[0], vertices[corner], vertices[corner + 1]];
        let base = (positions.len() / 3) as u32;
        for point in triangle {
            positions.extend_from_slice(&point);
            normals.extend_from_slice(&normal);
            colors.extend_from_slice(&color);
        }
        indices.extend_from_slice(&[base, base + 1, base + 2]);
    }
}

/// 🔺️ One data mesh out of one planar polygon.
fn polygon_mesh(mesh_id: &str, vertices: &[[f64; 3]], color: [f64; 3]) -> Option<Value> {
    let (mut positions, mut normals, mut colors, mut indices) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    push_polygon(vertices, color, &mut positions, &mut normals, &mut colors, &mut indices);
    if indices.is_empty() {
        return None;
    }
    Some(json!({ "id": mesh_id, "data": { "positions": positions, "normals": normals, "colors": colors, "indices": indices } }))
}

/// 🪪️ One instance record. `id` is the RAW entity id, which is what a `domain_id`-bound world window
/// reports back through the framework-reserved `interactionSelect`/`interactionHover` — that is the
/// whole mechanism behind "tree pick ⇄ 3d pick".
fn instance(target_id: &str, mesh_id: &str, label: &str, object_kind: &str, selected: bool, hovered: bool) -> Value {
    json!({
        "id": target_id,
        "meshId": mesh_id,
        "position": [0.0, 0.0, 0.0],
        "rotation": [0.0, 0.0, 0.0, 1.0],
        "scale": [1.0, 1.0, 1.0],
        "label": label,
        "objectKind": object_kind,
        "selected": selected,
        "hovered": hovered,
    })
}
//#endregion 🔖️Geometry

//#region 🔖️Scene
/// 🎬️ The `(meshes_json, instances_json)` pair for one model: one mesh + one instance per opaque
/// `Surface`, per `Fenestration` and per `ShadingSurface`. Total and deterministic — a degenerate
/// entity (fewer than three vertices, a zero-area window) contributes nothing rather than faulting.
pub fn energy_model_scene_parts(model: &Model, style: &EnergySceneStyle<'_>) -> (String, String) {
    let mut meshes: Vec<Value> = Vec::new();
    let mut instances: Vec<Value> = Vec::new();

    for surface in &model.surfaces {
        let target_id = energy_scene_target_id(surface.id);
        let color = style.paint(surface.id, &target_id, surface_class_color(surface.class));
        let mesh_id = format!("{ENERGY_SCENE_SURFACE_MESH_PREFIX}{}", surface.id.0);
        let Some(mesh) = polygon_mesh(&mesh_id, &surface.vertices_m, color) else { continue };
        meshes.push(mesh);
        instances.push(instance(&target_id, &mesh_id, &surface.name, ENERGY_SCENE_OBJECT_KIND_SURFACE, style.is_selected(&target_id), style.is_hovered(&target_id)));
    }

    for window in &model.fenestrations {
        let Some(polygon) = model_fenestration_polygon(model, window) else { continue };
        let target_id = energy_scene_target_id(window.id);
        let color = style.paint(window.id, &target_id, ENERGY_SCENE_FENESTRATION_COLOR);
        let mesh_id = format!("{ENERGY_SCENE_FENESTRATION_MESH_PREFIX}{}", window.id.0);
        let Some(mesh) = polygon_mesh(&mesh_id, &polygon, color) else { continue };
        meshes.push(mesh);
        instances.push(instance(&target_id, &mesh_id, &window.name, ENERGY_SCENE_OBJECT_KIND_FENESTRATION, style.is_selected(&target_id), style.is_hovered(&target_id)));
    }

    for shading in &model.shading_surfaces {
        let target_id = energy_scene_target_id(shading.id);
        let color = style.paint(shading.id, &target_id, ENERGY_SCENE_SHADING_COLOR);
        let mesh_id = format!("{ENERGY_SCENE_SHADING_MESH_PREFIX}{}", shading.id.0);
        let Some(mesh) = polygon_mesh(&mesh_id, &shading.vertices_m, color) else { continue };
        meshes.push(mesh);
        instances.push(instance(&target_id, &mesh_id, &shading.name, ENERGY_SCENE_OBJECT_KIND_SHADING, style.is_selected(&target_id), style.is_hovered(&target_id)));
    }

    (Value::Array(meshes).to_string(), Value::Array(instances).to_string())
}

/// 📦️ Axis-aligned bounds over every rendered polygon, `None` for a model with no geometry at all.
pub fn energy_model_bounds(model: &Model) -> Option<([f64; 3], [f64; 3])> {
    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    let mut seen = false;
    let mut expand = |vertices: &[[f64; 3]]| {
        for vertex in vertices {
            seen = true;
            for axis in 0..3 {
                min[axis] = min[axis].min(vertex[axis]);
                max[axis] = max[axis].max(vertex[axis]);
            }
        }
    };
    for surface in &model.surfaces {
        expand(&surface.vertices_m);
    }
    for shading in &model.shading_surfaces {
        expand(&shading.vertices_m);
    }
    if seen {
        Some((min, max))
    } else {
        None
    }
}

/// 🎥️ Fit revision: the camera auto-frames exactly ONCE per distinct geometry, never taking back a
/// camera the user has moved. Hashed over the model name plus every rendered entity id and vertex —
/// the same "frame once per document delivery" rule cad/generation3d state.
pub fn energy_model_fit_revision(model: &Model) -> u32 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    model.name.hash(&mut hasher);
    for surface in &model.surfaces {
        surface.id.0.hash(&mut hasher);
        hash_vertices(&surface.vertices_m, &mut hasher);
    }
    for window in &model.fenestrations {
        window.id.0.hash(&mut hasher);
        window.surface_id.0.hash(&mut hasher);
        for scalar in [window.area_m2, window.height_m, window.sill_height_m] {
            scalar.to_bits().hash(&mut hasher);
        }
        hash_vertices(&window.vertices_m, &mut hasher);
    }
    for shading in &model.shading_surfaces {
        shading.id.0.hash(&mut hasher);
        hash_vertices(&shading.vertices_m, &mut hasher);
    }
    (hasher.finish() & u64::from(u32::MAX)) as u32
}

fn hash_vertices<H: std::hash::Hasher>(vertices: &[[f64; 3]], hasher: &mut H) {
    use std::hash::Hash;
    for vertex in vertices {
        for axis in vertex {
            axis.to_bits().hash(hasher);
        }
    }
}

/// 📐️ Longest bounding-box edge, used to place the default camera at a sane distance for models
/// whose scale varies wildly between examples. `1.0` for a model with no geometry.
pub fn energy_model_extent(model: &Model) -> f64 {
    let Some((min, max)) = energy_model_bounds(model) else { return 1.0 };
    let span = [max[0] - min[0], max[1] - min[1], max[2] - min[2]];
    span.iter().fold(0.0_f64, |longest, value| longest.max(*value)).max(1.0)
}

/// 🎥️ A three-quarter view from the south-west and above, framed on the model's own centre — `fit`
/// re-frames it anyway, this is only what a host without fit support would show.
pub fn energy_model_camera_json(model: &Model) -> String {
    let (centre, distance) = match energy_model_bounds(model) {
        Some((min, max)) => ([(min[0] + max[0]) / 2.0, (min[1] + max[1]) / 2.0, (min[2] + max[2]) / 2.0], energy_model_extent(model) * 1.8),
        None => ([0.0, 0.0, 0.0], 8.0),
    };
    semio_framework_plugin::world3d_camera_json([centre[0] + distance, centre[1] - distance, centre[2] + distance * 0.7], centre, 45.0)
}

/// 🎬️ The whole `World3dScene` for one model. `domain_id` is `Some` only for a surface that actually
/// declares the interaction domain (the editor); the read-only viewer passes `None` and the react
/// host leaves the window on the OS's own world board with no plugin-owned picking.
pub fn energy_model_scene(model: &Model, style: &EnergySceneStyle<'_>, domain: Option<(&str, &str)>) -> semio_framework_plugin::World3dScene {
    let (meshes_json, instances_json) = energy_model_scene_parts(model, style);
    let mut scene = semio_framework_plugin::world3d_scene(
        energy_model_camera_json(model),
        meshes_json,
        instances_json,
        semio_framework_plugin::world3d_selection_json("rectangle", style.selected_ids, style.hovered_ids.first().map(String::as_str)),
        &semio_framework_plugin::WorldSunConfig::default(),
    );
    if let Some((domain_id, granularity_id)) = domain {
        scene.domain_id = Some(domain_id.to_string());
        scene.domain_granularity_id = Some(granularity_id.to_string());
    }
    scene.fit_json = Some(semio_framework_plugin::world3d_fit_json(energy_model_fit_revision(model), ENERGY_SCENE_FIT_PADDING, None));
    scene
}
//#endregion 🔖️Scene

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
