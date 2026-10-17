//! 📐️ The vertex vector math, aperture placement and the field helpers of the encoder.

use super::*;

//#region 🔖️Geometry
pub(super) type Vec3 = [f64; 3];

fn sub(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn cross(a: Vec3, b: Vec3) -> Vec3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

fn dot(a: Vec3, b: Vec3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn norm(a: Vec3) -> f64 {
    dot(a, a).sqrt()
}

fn normalized(a: Vec3) -> Option<Vec3> {
    let length = norm(a);
    if length <= 1e-12 {
        None
    } else {
        Some([a[0] / length, a[1] / length, a[2] / length])
    }
}

/// 📐️ Newell outward normal of a planar polygon wound counter-clockwise seen from outside.
pub fn surface_normal(vertices: &[Vec3]) -> Option<Vec3> {
    if vertices.len() < 3 {
        return None;
    }
    let mut accumulated = [0.0f64; 3];
    for index in 0..vertices.len() {
        let current = vertices[index];
        let next = vertices[(index + 1) % vertices.len()];
        accumulated[0] += (current[1] - next[1]) * (current[2] + next[2]);
        accumulated[1] += (current[2] - next[2]) * (current[0] + next[0]);
        accumulated[2] += (current[0] - next[0]) * (current[1] + next[1]);
    }
    normalized(accumulated)
}

/// 📐️ Right-handed in-plane basis `(u, v)` with `u × v = n`; `u` is horizontal wherever the
/// surface is not itself horizontal, so `v` is the surface's own "up" and a window sill measures
/// along it. A horizontal surface's "up" is north, so `u` is "right" seen from outside everywhere:
/// east on a roof seen from above, west on a floor seen from below.
pub fn surface_basis(normal: Vec3) -> (Vec3, Vec3) {
    let up = [0.0, 0.0, 1.0];
    let horizontal = normalized(cross(up, normal)).or_else(|| normalized(cross([0.0, 1.0, 0.0], normal))).unwrap_or([1.0, 0.0, 0.0]);
    (horizontal, cross(normal, horizontal))
}

/// 📐️ Declared convention #3 — the same ring, rotated to start at its upper-left corner seen from
/// outside: the highest vertex along the surface's own "up" (`v` of [`surface_basis`]), the
/// leftmost (smallest `u`) of equally high ones. The winding is kept; a degenerate ring is returned
/// unchanged. NREL's own ASHRAE 140 encoding (honeybee → OpenStudio) starts every ring there too.
pub fn upper_left_first(vertices: &[Vec3]) -> Vec<Vec3> {
    let Some(normal) = surface_normal(vertices) else { return vertices.to_vec() };
    let (u, v) = surface_basis(normal);
    let top = vertices.iter().map(|vertex| dot(*vertex, v)).fold(f64::NEG_INFINITY, f64::max);
    let start = (0..vertices.len()).filter(|index| top - dot(vertices[*index], v) <= VERTEX_HEIGHT_TIE_M).min_by(|a, b| dot(vertices[*a], u).total_cmp(&dot(vertices[*b], u))).unwrap_or(0);
    vertices[start..].iter().chain(&vertices[..start]).copied().collect()
}

/// 🪟️ Declared convention #2 — the polygon of window `index` of `count` on `host`: the aperture's
/// own [`Fenestration::vertices_m`] when it carries them, otherwise the rectangle derived from
/// `area_m2`/`height_m`/`sill_height_m`.
///
/// The placement itself is NOT written here: it is [`crate::precompute::place_aperture`], the one
/// function the engine's own window placement also goes through, so a document this codec writes
/// describes the geometry the simulation actually shades. What stays here is this codec's own
/// declared convention #2 — the `APERTURE_ASPECT`/`APERTURE_SILL_M` fallbacks for an aperture that
/// states no height or sill, which the engine deliberately does not have — and the degeneracy
/// check that turns an unplaceable aperture into a diagnostic instead of a bad rectangle.
pub fn aperture_rectangle(host: &Surface, window: &Fenestration, index: usize, count: usize) -> Option<Vec<Vec3>> {
    if !window.vertices_m.is_empty() {
        return (window.vertices_m.len() >= 3).then(|| window.vertices_m.clone());
    }
    let normal = surface_normal(&host.vertices_m)?;
    let (u, v) = surface_basis(normal);
    let origin = *host.vertices_m.first()?;
    let projected: Vec<(f64, f64)> = host.vertices_m.iter().map(|vertex| (dot(sub(*vertex, origin), u), dot(sub(*vertex, origin), v))).collect();
    let (mut u_min, mut u_max, mut v_min) = (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY);
    for (uc, vc) in &projected {
        u_min = u_min.min(*uc);
        u_max = u_max.max(*uc);
        v_min = v_min.min(*vc);
    }
    if !u_min.is_finite() || !u_max.is_finite() || !v_min.is_finite() || window.area_m2 <= 0.0 {
        return None;
    }
    let height = if window.height_m > 0.0 { window.height_m } else { (window.area_m2 / APERTURE_ASPECT).sqrt() };
    let sill = if window.sill_height_m > 0.0 { window.sill_height_m } else { APERTURE_SILL_M };
    Some(crate::precompute::place_aperture(&host.vertices_m, &[], window.area_m2, height, sill, index, count))
}
//#endregion 🔖️Geometry

//#region 🔖️Helpers
pub(super) fn entry(name: impl Into<String>, fields: impl IntoIterator<Item = (String, Value)>) -> (String, Value) {
    (name.into(), Value::Object(Object::from_iter(fields)))
}

pub(super) fn field(key: &str, value: impl Into<Value>) -> (String, Value) {
    (key.to_string(), value.into())
}

pub(super) fn yes_no(flag: bool) -> Value {
    Value::from(if flag { "Yes" } else { "No" })
}

pub(super) fn vertex_value(vertex: Vec3) -> Value {
    Value::Object(Object::from_iter([field("vertex_x_coordinate", vertex[0]), field("vertex_y_coordinate", vertex[1]), field("vertex_z_coordinate", vertex[2])]))
}

/// 🧱️ EnergyPlus rejects a massive layer with zero density or a specific heat under 100 J/kg·K —
/// exactly how ANSI/ASHRAE 140 tabulates its floor insulation — so such a layer becomes a
/// `Material:NoMass` with `R = thickness / conductivity`, the same substitution NREL's own
/// BESTEST encoding makes.
pub fn is_massless(material: &Material) -> bool {
    material.density_kg_m3 <= 0.0 || material.specific_heat_j_kg_k < 100.0
}

/// 🪟️ The layered construction a window IS, when its `glazing_construction_id` names one the model
/// defines — see the module docstring's glazing paragraph.
pub fn glazing_stack<'a>(model: &'a Model, window: &Fenestration) -> Option<&'a Construction> {
    window.glazing_construction_id.and_then(|id| model.constructions.iter().find(|construction| construction.id == id))
}

pub(super) fn surface_type(class: SurfaceClass) -> &'static str {
    match class {
        SurfaceClass::Roof => "Roof",
        SurfaceClass::Ceiling => "Ceiling",
        SurfaceClass::Floor | SurfaceClass::Ground => "Floor",
        _ => "Wall",
    }
}

pub(super) fn boundary(condition: &OutsideBoundary) -> &'static str {
    match condition {
        OutsideBoundary::OutdoorAir => "Outdoors",
        OutsideBoundary::Ground => "Ground",
        OutsideBoundary::OtherSideTemperature => "OtherSideCoefficients",
        OutsideBoundary::Adiabatic | OutsideBoundary::Interzone(_) => "Adiabatic",
    }
}
//#endregion 🔖️Helpers
