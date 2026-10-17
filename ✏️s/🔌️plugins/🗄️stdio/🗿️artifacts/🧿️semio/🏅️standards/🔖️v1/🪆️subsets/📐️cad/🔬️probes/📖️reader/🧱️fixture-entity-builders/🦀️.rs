//! 🧱️ Fixture entity builders: the point, layer, line, circle, arc, polyline, text, insert and solid builders and the pinned wall clock. See [`super`].

use super::*;

//#region 🏭️Fixtures
// 🚫️async: pure constructor helper.
pub(super) fn pt(x: f64, y: f64) -> Point {
    Point::new(x, y, 0.0)
}

// 🚫️async: pure constructor helper.
fn on_layer(kind: EntityType, layer: &str) -> Entity {
    let mut e = Entity::new(kind);
    e.common.layer = layer.to_string();
    e
}

// 🚫️async: pure constructor helper.
pub(super) fn layer(name: &str, color: u8, line_type: &str, visible: bool) -> dxf::tables::Layer {
    let mut l = dxf::tables::Layer { name: name.to_string(), ..Default::default() };
    l.color = dxf::Color::from_index(color);
    l.line_type_name = line_type.to_string();
    l.is_layer_on = visible;
    l
}

// 🚫️async: pure constructor helper.
pub(super) fn line(a: (f64, f64), b: (f64, f64), l: &str) -> Entity {
    on_layer(EntityType::Line(Line { p1: pt(a.0, a.1), p2: pt(b.0, b.1), ..Default::default() }), l)
}

// 🚫️async: pure constructor helper.
pub(super) fn circle(c: (f64, f64), r: f64, l: &str) -> Entity {
    on_layer(EntityType::Circle(Circle { center: pt(c.0, c.1), radius: r, ..Default::default() }), l)
}

// 🚫️async: pure constructor helper.
pub(super) fn arc(c: (f64, f64), r: f64, a0: f64, a1: f64, l: &str) -> Entity {
    on_layer(EntityType::Arc(Arc { center: pt(c.0, c.1), radius: r, start_angle: a0, end_angle: a1, ..Default::default() }), l)
}

// 🚫️async: pure constructor helper.
pub(super) fn add_polyline(d: &mut Drawing, points: &[(f64, f64)], closed: bool, l: &str) {
    let mut p = Polyline::default();
    p.set_is_closed(closed);
    for (x, y) in points {
        p.add_vertex(d, Vertex::new(pt(*x, *y)));
    }
    d.add_entity(on_layer(EntityType::Polyline(p), l));
}

// 🚫️async: pure constructor helper.
pub(super) fn text(p: (f64, f64), height: f64, value: &str, l: &str) -> Entity {
    on_layer(EntityType::Text(Text { location: pt(p.0, p.1), text_height: height, value: value.to_string(), rotation: 0.0, ..Default::default() }), l)
}

// 🚫️async: pure constructor helper.
pub(super) fn insert(name: &str, p: (f64, f64), l: &str) -> Entity {
    on_layer(EntityType::Insert(Insert { name: name.to_string(), location: pt(p.0, p.1), x_scale_factor: 1.0, y_scale_factor: 1.0, rotation: 0.0, ..Default::default() }), l)
}

// 🚫️async: pure constructor helper.
pub(super) fn solid(c: [(f64, f64); 4], l: &str) -> Entity {
    on_layer(EntityType::Solid(Solid { first_corner: pt(c[0].0, c[0].1), second_corner: pt(c[1].0, c[1].1), third_corner: pt(c[2].0, c[2].1), fourth_corner: pt(c[3].0, c[3].1), ..Default::default() }), l)
}

/// ⏱️ Pins the four wall-clock header variables the `dxf` crate stamps from `Local::now()`/`Utc::now()`
/// (`$TDCREATE`, `$TDUCREATE`, `$TDUPDATE`, `$TDUUPDATE`) to J2000.0.
///
/// MEASURED, not anticipated: generating the corpus twice and diffing produced
/// `2461280.937766203657` against `2461280.937893518712` on every DXF file — a Julian day number
/// differing in the fractional part, i.e. the moment of the run. A fixture that changes on every
/// invocation cannot be reproduced, and `test fixture reproduce` would have failed all sixteen.
///
/// The pinning goes through the LIBRARY'S OWN parser rather than being patched into the final bytes:
/// the drawing is serialised, the four values are rewritten in that intermediate text, and the
/// result is handed back to `Drawing::load` — which accepts it, proving the edit is still a valid
/// DXF — so the bytes finally committed are ones the `dxf` crate wrote from a state it parsed.
// 🚫️async: pure codec helper.
pub(super) fn pin_wall_clock(drawing: &Drawing) -> Result<Drawing, String> {
    const PINNED: &str = "2451545.0";
    const TIME_VARS: [&str; 4] = ["$TDCREATE", "$TDUCREATE", "$TDUPDATE", "$TDUUPDATE"];
    let mut buffer: Vec<u8> = Vec::new();
    drawing.save(&mut buffer).map_err(|e| format!("dxf save to buffer: {e}"))?;
    let text = String::from_utf8_lossy(&buffer).into_owned();
    let mut lines: Vec<String> = text.split('\n').map(|l| l.trim_end_matches('\r').to_string()).collect();
    let mut index = 0;
    while index + 2 < lines.len() {
        if TIME_VARS.contains(&lines[index].trim()) && lines[index + 1].trim() == "40" {
            lines[index + 2] = PINNED.to_string();
        }
        index += 1;
    }
    let pinned = lines.join("\r\n");
    Drawing::load(&mut pinned.as_bytes()).map_err(|e| format!("dxf reload after pinning: {e}"))
}
//#endregion 🏭️Fixtures
