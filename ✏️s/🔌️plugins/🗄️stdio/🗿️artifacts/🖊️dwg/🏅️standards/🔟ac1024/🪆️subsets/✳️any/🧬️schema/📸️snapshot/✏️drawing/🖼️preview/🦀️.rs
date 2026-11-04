//! 🖼️ Pure 2D projection of the typed DWG geometry to a self-contained SVG document, shared by the editor and viewer windows.
//! Draws the eleven typed kinds of the logical drawing in the XY plane; objects without a typed geometry stay in the model untouched and are not drawn.
//! [Open Design DWG specification](https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf)

use super::geometry::{DwgLogicalEntity, DwgLogicalGeometryKind};
use super::DwgLogicalDrawing;
use std::fmt::Write;

pub const DWG_PREVIEW_CANVAS: (u32, u32) = (800, 600);

struct Extent {
    min: [f64; 2],
    max: [f64; 2],
}

impl Extent {
    fn include(&mut self, x: f64, y: f64) {
        if x.is_finite() && y.is_finite() {
            self.min = [self.min[0].min(x), self.min[1].min(y)];
            self.max = [self.max[0].max(x), self.max[1].max(y)];
        }
    }
}

fn number(value: f64) -> String {
    if value.is_finite() { format!("{value:.6}").trim_end_matches('0').trim_end_matches('.').to_string() } else { "0".into() }
}

fn escape(text: &str) -> String {
    text.chars().filter(|character| !character.is_control() || *character == '\t').fold(String::new(), |mut out, character| {
        match character {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            other => out.push(other),
        }
        out
    })
}

fn bulge_segment(out: &mut String, from: [f64; 2], to: [f64; 2], bulge: f64) {
    let chord = ((to[0] - from[0]).powi(2) + (to[1] - from[1]).powi(2)).sqrt();
    if bulge == 0.0 || chord == 0.0 || !bulge.is_finite() {
        let _ = write!(out, "L{} {}", number(to[0]), number(to[1]));
        return;
    }
    let radius = chord * (1.0 + bulge * bulge) / (4.0 * bulge.abs());
    let _ = write!(out, "A{} {} 0 {} {} {} {}", number(radius), number(radius), u8::from(bulge.abs() > 1.0), u8::from(bulge > 0.0), number(to[0]), number(to[1]));
}

fn value(values: &[f64], index: usize) -> f64 {
    values.get(index).copied().unwrap_or_default()
}

fn draw(out: &mut String, extent: &mut Extent, entity: &DwgLogicalEntity) {
    let geometry = &entity.geometry;
    let values = geometry.values.as_slice();
    match geometry.kind {
        DwgLogicalGeometryKind::Point => {
            extent.include(value(values, 0), value(values, 1));
            let _ = write!(out, "<circle cx=\"{}\" cy=\"{}\" r=\"0.5\" class=\"point\"/>", number(value(values, 0)), number(value(values, 1)));
        }
        DwgLogicalGeometryKind::Line => {
            extent.include(value(values, 0), value(values, 1));
            extent.include(value(values, 3), value(values, 4));
            let _ = write!(out, "<path d=\"M{} {}L{} {}\"/>", number(value(values, 0)), number(value(values, 1)), number(value(values, 3)), number(value(values, 4)));
        }
        DwgLogicalGeometryKind::Circle => {
            let (cx, cy, radius) = (value(values, 0), value(values, 1), value(values, 3).abs());
            extent.include(cx - radius, cy - radius);
            extent.include(cx + radius, cy + radius);
            let _ = write!(out, "<circle cx=\"{}\" cy=\"{}\" r=\"{}\"/>", number(cx), number(cy), number(radius));
        }
        DwgLogicalGeometryKind::Arc => {
            let (cx, cy, radius, start, end) = (value(values, 0), value(values, 1), value(values, 3).abs(), value(values, 4), value(values, 5));
            let sweep = (end - start).rem_euclid(std::f64::consts::TAU);
            let (from, to) = ((cx + radius * start.cos(), cy + radius * start.sin()), (cx + radius * end.cos(), cy + radius * end.sin()));
            extent.include(cx - radius, cy - radius);
            extent.include(cx + radius, cy + radius);
            let _ = write!(out, "<path d=\"M{} {}A{} {} 0 {} 1 {} {}\"/>", number(from.0), number(from.1), number(radius), number(radius), u8::from(sweep > std::f64::consts::PI), number(to.0), number(to.1));
        }
        DwgLogicalGeometryKind::Ellipse => {
            let (cx, cy, major) = (value(values, 0), value(values, 1), [value(values, 3), value(values, 4)]);
            let (rx, ry) = ((major[0].powi(2) + major[1].powi(2)).sqrt(), (major[0].powi(2) + major[1].powi(2)).sqrt() * value(values, 6));
            extent.include(cx - rx, cy - rx);
            extent.include(cx + rx, cy + rx);
            let _ = write!(out, "<ellipse cx=\"{}\" cy=\"{}\" rx=\"{}\" ry=\"{}\" transform=\"rotate({} {} {})\"/>", number(cx), number(cy), number(rx), number(ry), number(major[1].atan2(major[0]).to_degrees()), number(cx), number(cy));
        }
        DwgLogicalGeometryKind::LwPolyline => {
            let count = geometry.indices.first().copied().unwrap_or_default().max(0) as usize;
            let point = |index: usize| [value(values, 1 + index * 2), value(values, 2 + index * 2)];
            let bulge = |index: usize| value(values, 1 + count * 2 + index);
            if count == 0 {
                return;
            }
            let first = point(0);
            let _ = write!(out, "<path d=\"M{} {}", number(first[0]), number(first[1]));
            (0..count).for_each(|index| extent.include(point(index)[0], point(index)[1]));
            (1..count).for_each(|index| bulge_segment(out, point(index - 1), point(index), bulge(index - 1)));
            if geometry.closed {
                bulge_segment(out, point(count - 1), first, bulge(count - 1));
                out.push('Z');
            }
            out.push_str("\"/>");
        }
        DwgLogicalGeometryKind::Spline => {
            let count = geometry.indices.get(1).copied().unwrap_or_default().max(0) as usize;
            let points = (0..count).map(|index| (value(values, index * 3), value(values, index * 3 + 1))).collect::<Vec<_>>();
            points.iter().for_each(|point| extent.include(point.0, point.1));
            let path = points.iter().enumerate().map(|(index, point)| format!("{}{} {}", if index == 0 { "M" } else { "L" }, number(point.0), number(point.1))).collect::<String>();
            let _ = write!(out, "<path d=\"{path}\" stroke-dasharray=\"4 2\"/>");
        }
        DwgLogicalGeometryKind::Text => {
            let (x, y, height) = (value(values, 0), value(values, 1), value(values, 3).abs().max(f64::EPSILON));
            extent.include(x, y);
            extent.include(x, y + height);
            let _ = write!(out, "<text transform=\"translate({} {}) rotate({}) scale(1 -1)\" font-size=\"{}\" fill=\"#1a1a1a\" stroke=\"none\">{}</text>", number(x), number(y), number(value(values, 4).to_degrees()), number(height), escape(&geometry.text));
        }
        DwgLogicalGeometryKind::Face3d => {
            let points = (0..4).map(|corner| (value(values, corner * 3), value(values, corner * 3 + 1))).collect::<Vec<_>>();
            points.iter().for_each(|point| extent.include(point.0, point.1));
            let path = [0, 1, 2, 3].iter().map(|index| format!("{},{}", number(points[*index].0), number(points[*index].1))).collect::<Vec<_>>().join(" ");
            let _ = write!(out, "<polygon points=\"{path}\"/>");
        }
        DwgLogicalGeometryKind::Polyline3d => {
            let points = values.chunks_exact(3).map(|chunk| (chunk[0], chunk[1])).collect::<Vec<_>>();
            points.iter().for_each(|point| extent.include(point.0, point.1));
            let path = points.iter().enumerate().map(|(index, point)| format!("{}{} {}", if index == 0 { "M" } else { "L" }, number(point.0), number(point.1))).collect::<String>();
            let _ = write!(out, "<path d=\"{path}{}\"/>", if geometry.closed { "Z" } else { "" });
        }
        DwgLogicalGeometryKind::PolyfaceMesh => {
            let count = geometry.indices.first().copied().unwrap_or_default().max(0) as usize;
            let points = (0..count).map(|index| (value(values, index * 3), value(values, index * 3 + 1))).collect::<Vec<_>>();
            points.iter().for_each(|point| extent.include(point.0, point.1));
            for face in geometry.indices.iter().skip(1).copied().collect::<Vec<_>>().chunks_exact(4) {
                let corners = face.iter().filter(|index| **index != 0).filter_map(|index| points.get(index.unsigned_abs() as usize - 1)).map(|point| format!("{},{}", number(point.0), number(point.1))).collect::<Vec<_>>();
                if corners.len() >= 3 {
                    let _ = write!(out, "<polygon points=\"{}\"/>", corners.join(" "));
                }
            }
        }
    }
}

/// 🖼️ Projects the drawing's typed entities to SVG (Y axis flipped to the DWG convention).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn preview_svg(drawing: &DwgLogicalDrawing) -> String {
    let mut body = String::new();
    let mut extent = Extent { min: [f64::INFINITY; 2], max: [f64::NEG_INFINITY; 2] };
    drawing.entities().iter().for_each(|entity| draw(&mut body, &mut extent, entity));
    if !extent.min[0].is_finite() {
        extent = Extent { min: [0.0, 0.0], max: [1.0, 1.0] };
    }
    let width = (extent.max[0] - extent.min[0]).max(1e-9);
    let height = (extent.max[1] - extent.min[1]).max(1e-9);
    let margin = width.max(height) * 0.05;
    let (canvas_width, canvas_height) = DWG_PREVIEW_CANVAS;
    let (x, y, w, h) = (number(extent.min[0] - margin), number(extent.min[1] - margin), number(width + 2.0 * margin), number(height + 2.0 * margin));
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{canvas_width}\" height=\"{canvas_height}\" viewBox=\"{x} {y} {w} {h}\"><rect x=\"{x}\" y=\"{y}\" width=\"{w}\" height=\"{h}\" fill=\"#ffffff\"/><g transform=\"translate(0 {}) scale(1 -1)\" fill=\"none\" stroke=\"#1a1a1a\" stroke-width=\"{}\">{body}</g></svg>",
        number(extent.min[1] * 2.0 + height),
        number(width.max(height) / 400.0),
    )
}
