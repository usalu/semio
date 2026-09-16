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
//! 🚫️ There is no NUMERIC alpha lane for a data mesh — `transparent` follows the style kind's own
//! opacity, never the payload — so a window renders as a solid light blue rather than a translucent
//! one. The one translucency a payload CAN ask for is the instance flag `disabled: true`, whose
//! `MESH_STYLE_PAINT.disabled` entry is the only style with `opacity < 1` (0.45) and which also turns
//! the instance's raycast off entirely. That is exactly what a zone volume wants, and what it uses.

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
/// 🎨️ How far towards white a hovered entity is lightened. Hover has to be legible on its OWN — the
/// react host's `MESH_STYLE_PAINT.hovered` fill never reaches a vertex-coloured mesh (`PaintTexturedMesh`
/// forces the material to white and the emissive to black when the geometry carries `color`), so this
/// bake is the entire hover feedback. It moves every channel strictly UP, where selection moves
/// towards the primary blue — two directions, never confusable.
pub const ENERGY_SCENE_HOVERED_MIX: f64 = 0.35;

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
/// 📦️ Zone-volume swatch — a soft violet no envelope class or family uses, so the translucent hull
/// reads as "air", never as a surface.
pub const ENERGY_SCENE_ZONE_COLOR: [f64; 3] = [0.686, 0.612, 0.902];

/// 🪪️ Mesh-id prefixes. The INSTANCE id is always the bare entity id (see
/// [`energy_scene_target_id`]); only the mesh id is namespaced, because a mesh id is private to the
/// payload while an instance id is the interaction vocabulary the tree and the inspector share.
pub const ENERGY_SCENE_SURFACE_MESH_PREFIX: &str = "energy-surface-";
pub const ENERGY_SCENE_FENESTRATION_MESH_PREFIX: &str = "energy-window-";
pub const ENERGY_SCENE_SHADING_MESH_PREFIX: &str = "energy-shading-";
pub const ENERGY_SCENE_ZONE_MESH_PREFIX: &str = "energy-zone-";

/// 🏷️ `objectKind` published per instance, so a host-side kind filter/legend can group by family.
/// The four spellings are exactly the interaction domain's granularity ids for the same families
/// (`crate::editor::model::interaction::ENERGY_GRANULARITY_*`).
pub const ENERGY_SCENE_OBJECT_KIND_SURFACE: &str = "surface";
pub const ENERGY_SCENE_OBJECT_KIND_FENESTRATION: &str = "fenestration";
pub const ENERGY_SCENE_OBJECT_KIND_SHADING: &str = "shading";
pub const ENERGY_SCENE_OBJECT_KIND_ZONE: &str = "zone";

/// 📦️ Refusal bound on the hull input: a zone whose member surfaces carry more corners than this
/// draws no volume rather than running an O(n²) dedupe over an unbounded set. 4 096 corners is ~1 000
/// quad faces in ONE zone, two orders of magnitude above any authored example.
pub const ENERGY_SCENE_ZONE_HULL_MAXIMUM_POINTS: usize = 4_096;
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

fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
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

/// 🔺️ One data mesh out of a triangle soup (the zone hull's faces), one flat normal per triangle.
fn triangles_mesh(mesh_id: &str, triangles: &[[[f64; 3]; 3]], color: [f64; 3]) -> Option<Value> {
    let (mut positions, mut normals, mut colors, mut indices) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for triangle in triangles {
        push_polygon(triangle, color, &mut positions, &mut normals, &mut colors, &mut indices);
    }
    if indices.is_empty() {
        return None;
    }
    Some(json!({ "id": mesh_id, "data": { "positions": positions, "normals": normals, "colors": colors, "indices": indices } }))
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

/// 📦️ One zone volume's instance. It carries `disabled: true`, which is the ONE lane the react
/// `World3dHost` gives a payload over a mesh's opacity AND its pickability:
/// `MESH_STYLE_PAINT.disabled` is the only style with `opacity < 1` (0.45 — `transparent` follows the
/// style's opacity even for a vertex-coloured mesh, `PaintTexturedMesh`'s `transparent={style.opacity
/// < 1}`), and `instancePickEnabled = pickEnabled && !instance.disabled && !provisional` makes
/// `worldInstanceMeshRaycast` answer `() => null`, so the hull is INVISIBLE to every raycast and can
/// never occlude the pick of a wall inside it. There is no payload alpha lane, so 0.45 is what a
/// translucent zone is; the brief's 0.15 is not reachable without a host change.
fn zone_instance(target_id: &str, mesh_id: &str, label: &str, selected: bool, hovered: bool) -> Value {
    let mut record = instance(target_id, mesh_id, label, ENERGY_SCENE_OBJECT_KIND_ZONE, selected, hovered);
    if let Value::Object(fields) = &mut record {
        fields.insert("disabled".to_string(), Value::Bool(true));
    }
    record
}
//#endregion 🔖️Geometry

//#region 🔖️Hull
/// 📦️ Every corner of every surface that belongs to `zone`, in model order. Empty for a zone with no
/// member surface (which therefore draws no volume) and for one whose corner count exceeds
/// [`ENERGY_SCENE_ZONE_HULL_MAXIMUM_POINTS`].
pub fn zone_hull_points(model: &Model, zone: EntityId) -> Vec<[f64; 3]> {
    let mut points: Vec<[f64; 3]> = Vec::new();
    for surface in model.surfaces.iter().filter(|surface| surface.zone_id == zone) {
        if points.len() + surface.vertices_m.len() > ENERGY_SCENE_ZONE_HULL_MAXIMUM_POINTS {
            return Vec::new();
        }
        points.extend_from_slice(&surface.vertices_m);
    }
    points
}

/// 📦️ The convex hull of a point cloud as outward-wound triangles — the drawable volume a `Zone`
/// itself does not carry (it has a scalar `volume_m3` and no shape at all).
///
/// 🧮️ Why a hull and not `geometry::zone_volume_from_surfaces`' pyramid decomposition: that function
/// answers a SCALAR (it sums signed face-pyramid volumes about an interior reference point) and never
/// produces a surface — there is nothing in it to render. The hull is the cheapest honest shape whose
/// boundary encloses exactly the same corners the volume integral is taken over; for the rectangular
/// zones every authored example uses, the hull IS the zone box, so the drawn volume and the scalar
/// agree exactly.
///
/// Standard incremental construction: seed a non-degenerate tetrahedron, then for every remaining
/// point delete the faces it can see, walk the horizon of that hole and cone it back to the point.
/// Total: a cloud that is empty, too small, collinear or COPLANAR (a zone with only a floor, say)
/// encloses no volume and answers `None` rather than emitting a zero-thickness shell.
pub fn convex_hull_triangles(points: &[[f64; 3]]) -> Option<Vec<[[f64; 3]; 3]>> {
    if points.len() < 4 || points.len() > ENERGY_SCENE_ZONE_HULL_MAXIMUM_POINTS {
        return None;
    }
    let mut minimum = [f64::INFINITY; 3];
    let mut maximum = [f64::NEG_INFINITY; 3];
    for point in points {
        if !point.iter().all(|axis| axis.is_finite()) {
            return None;
        }
        for axis in 0..3 {
            minimum[axis] = minimum[axis].min(point[axis]);
            maximum[axis] = maximum[axis].max(point[axis]);
        }
    }
    let scale = (0..3).fold(0.0_f64, |longest, axis| longest.max(maximum[axis] - minimum[axis]));
    if !scale.is_finite() || scale <= 1e-9 {
        return None;
    }
    // 📏️ Every tolerance below is RELATIVE to the cloud's own extent, so a room in millimetres and a
    // district in kilometres are judged degenerate by the same rule.
    let epsilon = scale * 1e-9;

    let mut unique: Vec<[f64; 3]> = Vec::new();
    for point in points {
        if unique.iter().any(|seen| (0..3).all(|axis| (seen[axis] - point[axis]).abs() <= epsilon)) {
            continue;
        }
        unique.push(*point);
    }
    if unique.len() < 4 {
        return None;
    }

    let origin = unique[0];
    let (first, first_distance2) = farthest_point(&unique, |point| dot(sub3(point, origin), sub3(point, origin)));
    let axis_length = first_distance2.sqrt();
    if axis_length <= epsilon {
        return None;
    }
    let axis = sub3(unique[first], origin);
    let (second, line_area2) = farthest_point(&unique, |point| {
        let moment = cross3(axis, sub3(point, origin));
        dot(moment, moment)
    });
    if line_area2.sqrt() / axis_length <= epsilon {
        return None;
    }
    let plane_normal = normalized(cross3(axis, sub3(unique[second], origin)))?;
    let (third, height) = farthest_point(&unique, |point| dot(plane_normal, sub3(point, origin)).abs());
    if height <= epsilon {
        return None;
    }

    let seed = [0_usize, first, second, third];
    let centre = [
        seed.iter().map(|index| unique[*index][0]).sum::<f64>() / 4.0,
        seed.iter().map(|index| unique[*index][1]).sum::<f64>() / 4.0,
        seed.iter().map(|index| unique[*index][2]).sum::<f64>() / 4.0,
    ];
    let mut faces: Vec<[usize; 3]> = [[seed[0], seed[1], seed[2]], [seed[0], seed[1], seed[3]], [seed[0], seed[2], seed[3]], [seed[1], seed[2], seed[3]]].into_iter().map(|face| outward(face, &unique, centre)).collect();

    for index in 0..unique.len() {
        if seed.contains(&index) {
            continue;
        }
        let point = unique[index];
        let visible: Vec<usize> = faces.iter().enumerate().filter(|(_, face)| face_sees(&unique, **face, point) > epsilon).map(|(position, _)| position).collect();
        if visible.is_empty() {
            continue;
        }
        // 🕳️ The horizon: every directed edge of the removed cap that is NOT cancelled by its reverse
        // (an edge shared by two visible faces). What survives is exactly the rim of the hole.
        let mut horizon: Vec<(usize, usize)> = Vec::new();
        for position in &visible {
            let face = faces[*position];
            for edge in [(face[0], face[1]), (face[1], face[2]), (face[2], face[0])] {
                match horizon.iter().position(|other| *other == (edge.1, edge.0)) {
                    Some(twin) => {
                        horizon.remove(twin);
                    }
                    None => horizon.push(edge),
                }
            }
        }
        // `visible` is ascending, so removing from the back keeps the earlier indices valid.
        for position in visible.iter().rev() {
            faces.remove(*position);
        }
        // 🧭️ Coning a horizon edge to the new point preserves the winding the removed face had, so
        // every new face is outward without a second orientation pass.
        for (from, to) in horizon {
            faces.push([from, to, index]);
        }
    }

    if faces.len() < 4 {
        return None;
    }
    Some(faces.into_iter().map(|face| [unique[face[0]], unique[face[1]], unique[face[2]]]).collect())
}

/// 📏️ The index of the point scoring highest under `score`, and that score. `points` is never empty
/// at any call site (the hull has already refused a cloud smaller than four).
fn farthest_point(points: &[[f64; 3]], score: impl Fn([f64; 3]) -> f64) -> (usize, f64) {
    let mut best = (0_usize, f64::NEG_INFINITY);
    for (index, point) in points.iter().enumerate() {
        let value = score(*point);
        if value > best.1 {
            best = (index, value);
        }
    }
    best
}

/// 🧭️ Re-winds `face` so its normal points AWAY from `inside`.
fn outward(face: [usize; 3], points: &[[f64; 3]], inside: [f64; 3]) -> [usize; 3] {
    let normal = cross3(sub3(points[face[1]], points[face[0]]), sub3(points[face[2]], points[face[0]]));
    if dot(normal, sub3(inside, points[face[0]])) > 0.0 {
        [face[0], face[2], face[1]]
    } else {
        face
    }
}

/// 👁️ Signed distance of `point` above `face`'s plane — positive when the face can see it. A
/// degenerate face sees nothing.
fn face_sees(points: &[[f64; 3]], face: [usize; 3], point: [f64; 3]) -> f64 {
    let normal = cross3(sub3(points[face[1]], points[face[0]]), sub3(points[face[2]], points[face[0]]));
    let length = dot(normal, normal).sqrt();
    if length <= 0.0 {
        return f64::NEG_INFINITY;
    }
    dot(normal, sub3(point, points[face[0]])) / length
}
//#endregion 🔖️Hull

//#region 🔖️Scene
/// 🎬️ The `(meshes_json, instances_json)` pair for one model: one mesh + one instance per opaque
/// `Surface`, per `Fenestration`, per `ShadingSurface` and — translucent and unpickable — per `Zone`
/// that encloses a volume. Total and deterministic — a degenerate entity (fewer than three vertices,
/// a zero-area window, a zone with no surfaces or only coplanar ones) contributes nothing rather than
/// faulting.
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

    // 📦️ Zones LAST: a translucent hull must be published after the opaque envelope it wraps (three
    // draws transparent materials after opaque ones and sorts them back to front, so the walls are
    // already in the colour buffer when the hull blends over them), and appending keeps every opaque
    // family at the index it had before volumes existed.
    for zone in &model.zones {
        let Some(triangles) = convex_hull_triangles(&zone_hull_points(model, zone.id)) else { continue };
        let target_id = energy_scene_target_id(zone.id);
        let color = style.paint(zone.id, &target_id, ENERGY_SCENE_ZONE_COLOR);
        let mesh_id = format!("{ENERGY_SCENE_ZONE_MESH_PREFIX}{}", zone.id.0);
        let Some(mesh) = triangles_mesh(&mesh_id, &triangles, color) else { continue };
        meshes.push(mesh);
        instances.push(zone_instance(&target_id, &mesh_id, &zone.name, style.is_selected(&target_id), style.is_hovered(&target_id)));
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
    for zone in &model.zones {
        zone.id.0.hash(&mut hasher);
    }
    for surface in &model.surfaces {
        surface.id.0.hash(&mut hasher);
        // 📦️ Zone membership is geometry now: it decides which corners a zone's drawn volume hulls.
        surface.zone_id.0.hash(&mut hasher);
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
