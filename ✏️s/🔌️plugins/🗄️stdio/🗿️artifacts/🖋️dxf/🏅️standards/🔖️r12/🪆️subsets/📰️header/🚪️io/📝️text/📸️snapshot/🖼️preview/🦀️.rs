//! 🖼️ Pure 2D projection of the typed DXF drawing to a self-contained SVG document, shared by the editor and viewer windows.
//! [DXF reference](https://paulbourke.net/dataformats/dxf/min3d.html)

use crate::standards::v_r12::subsets::any::schema::snapshot::{DxfBlock, DxfEntity, DxfSnapshot};
use std::fmt::Write;

pub const DXF_PREVIEW_CANVAS: (u32, u32) = (800, 600);
const DXF_PREVIEW_MAX_BLOCK_DEPTH: usize = 8;

#[derive(Clone, Copy)]
struct Extent {
    min: [f64; 2],
    max: [f64; 2],
}

impl Extent {
    const EMPTY: Extent = Extent { min: [f64::INFINITY; 2], max: [f64::NEG_INFINITY; 2] };
    fn include(&mut self, x: f64, y: f64) {
        if x.is_finite() && y.is_finite() {
            self.min = [self.min[0].min(x), self.min[1].min(y)];
            self.max = [self.max[0].max(x), self.max[1].max(y)];
        }
    }
}

fn escape(text: &str) -> String {
    text.chars().fold(String::new(), |mut out, character| {
        match character {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            other if !other.is_control() || other == '\t' => out.push(other),
            _ => {}
        }
        out
    })
}

fn number(value: f64) -> String {
    if value.is_finite() { format!("{value:.6}").trim_end_matches('0').trim_end_matches('.').to_string() } else { "0".into() }
}

fn arc_point(center: [f64; 3], radius: f64, degrees: f64) -> (f64, f64) {
    let radians = degrees.to_radians();
    (center[0] + radius * radians.cos(), center[1] + radius * radians.sin())
}

fn bulge_segment(out: &mut String, from: (f64, f64), to: (f64, f64), bulge: f64) {
    let chord = ((to.0 - from.0).powi(2) + (to.1 - from.1).powi(2)).sqrt();
    if bulge == 0.0 || chord == 0.0 || !bulge.is_finite() {
        let _ = write!(out, "L{} {}", number(to.0), number(to.1));
        return;
    }
    let radius = chord * (1.0 + bulge * bulge) / (4.0 * bulge.abs());
    let _ = write!(out, "A{} {} 0 {} {} {} {}", number(radius), number(radius), u8::from(bulge.abs() > 1.0), u8::from(bulge > 0.0), number(to.0), number(to.1));
}

fn draw_entities(out: &mut String, extent: &mut Extent, entities: &[DxfEntity], blocks: &[DxfBlock], depth: usize) {
    for entity in entities {
        match entity {
            DxfEntity::Line(line) => {
                extent.include(line.start[0], line.start[1]);
                extent.include(line.end[0], line.end[1]);
                let _ = write!(out, "<path d=\"M{} {}L{} {}\"/>", number(line.start[0]), number(line.start[1]), number(line.end[0]), number(line.end[1]));
            }
            DxfEntity::Circle(circle) => {
                extent.include(circle.center[0] - circle.radius, circle.center[1] - circle.radius);
                extent.include(circle.center[0] + circle.radius, circle.center[1] + circle.radius);
                let _ = write!(out, "<circle cx=\"{}\" cy=\"{}\" r=\"{}\"/>", number(circle.center[0]), number(circle.center[1]), number(circle.radius.abs()));
            }
            DxfEntity::Arc(arc) => {
                let sweep = (arc.end_angle - arc.start_angle).rem_euclid(360.0);
                let from = arc_point(arc.center, arc.radius, arc.start_angle);
                let to = arc_point(arc.center, arc.radius, arc.end_angle);
                extent.include(from.0, from.1);
                extent.include(to.0, to.1);
                extent.include(arc.center[0] - arc.radius, arc.center[1] - arc.radius);
                extent.include(arc.center[0] + arc.radius, arc.center[1] + arc.radius);
                let _ = write!(out, "<path d=\"M{} {}A{} {} 0 {} 1 {} {}\"/>", number(from.0), number(from.1), number(arc.radius.abs()), number(arc.radius.abs()), u8::from(sweep > 180.0), number(to.0), number(to.1));
            }
            DxfEntity::Polyline(polyline) => {
                let Some(first) = polyline.vertices.first() else { continue };
                let _ = write!(out, "<path d=\"M{} {}", number(first.x), number(first.y));
                polyline.vertices.iter().for_each(|vertex| extent.include(vertex.x, vertex.y));
                polyline.vertices.windows(2).for_each(|pair| bulge_segment(out, (pair[0].x, pair[0].y), (pair[1].x, pair[1].y), pair[0].bulge));
                if polyline.closed {
                    let last = polyline.vertices.last().unwrap_or(first);
                    bulge_segment(out, (last.x, last.y), (first.x, first.y), last.bulge);
                    out.push('Z');
                }
                out.push_str("\"/>");
            }
            DxfEntity::Text(text) => {
                extent.include(text.position[0], text.position[1]);
                extent.include(text.position[0], text.position[1] + text.height);
                let _ = write!(out, "<text transform=\"translate({} {}) scale(1 -1)\" font-size=\"{}\" fill=\"#1a1a1a\" stroke=\"none\">{}</text>", number(text.position[0]), number(text.position[1]), number(text.height.abs().max(f64::EPSILON)), escape(&text.value));
            }
            DxfEntity::Solid(solid) => {
                let order = [0, 1, 3, 2];
                solid.points.iter().for_each(|point| extent.include(point[0], point[1]));
                let points = order.iter().map(|index| format!("{},{}", number(solid.points[*index][0]), number(solid.points[*index][1]))).collect::<Vec<_>>().join(" ");
                let _ = write!(out, "<polygon points=\"{points}\" fill=\"#1a1a1a\"/>");
            }
            DxfEntity::Insert(insert) => {
                extent.include(insert.position[0], insert.position[1]);
                let Some(block) = blocks.iter().find(|block| block.name == insert.block_name).filter(|_| depth < DXF_PREVIEW_MAX_BLOCK_DEPTH) else { continue };
                let mut inner = String::new();
                let mut inner_extent = Extent::EMPTY;
                draw_entities(&mut inner, &mut inner_extent, &block.entities, blocks, depth + 1);
                let transform = format!("translate({} {}) rotate({}) scale({} {}) translate({} {})", number(insert.position[0]), number(insert.position[1]), number(insert.rotation), number(insert.scale[0]), number(insert.scale[1]), number(-block.base_point[0]), number(-block.base_point[1]));
                let _ = write!(out, "<g transform=\"{transform}\">{inner}</g>");
                if inner_extent.min[0].is_finite() {
                    let radians = insert.rotation.to_radians();
                    for corner in [[inner_extent.min[0], inner_extent.min[1]], [inner_extent.max[0], inner_extent.min[1]], [inner_extent.max[0], inner_extent.max[1]], [inner_extent.min[0], inner_extent.max[1]]] {
                        let local = [(corner[0] - block.base_point[0]) * insert.scale[0], (corner[1] - block.base_point[1]) * insert.scale[1]];
                        extent.include(insert.position[0] + local[0] * radians.cos() - local[1] * radians.sin(), insert.position[1] + local[0] * radians.sin() + local[1] * radians.cos());
                    }
                }
            }
            DxfEntity::Other(_) => {}
        }
    }
}

/// 🖼️ Projects the typed drawing to SVG (Y axis flipped to the DXF convention); unknown entities are skipped, never invented.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn preview_svg(document: &DxfSnapshot) -> String {
    let mut body = String::new();
    let mut extent = Extent::EMPTY;
    draw_entities(&mut body, &mut extent, &document.entities, &document.blocks, 0);
    if !extent.min[0].is_finite() {
        extent = Extent { min: [0.0, 0.0], max: [1.0, 1.0] };
    }
    let width = (extent.max[0] - extent.min[0]).max(1e-9);
    let height = (extent.max[1] - extent.min[1]).max(1e-9);
    let margin = width.max(height) * 0.05;
    let (canvas_width, canvas_height) = DXF_PREVIEW_CANVAS;
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{canvas_width}\" height=\"{canvas_height}\" viewBox=\"{} {} {} {}\"><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#ffffff\"/><g transform=\"translate(0 {}) scale(1 -1)\" fill=\"none\" stroke=\"#1a1a1a\" stroke-width=\"{}\">{body}</g></svg>",
        number(extent.min[0] - margin),
        number(extent.min[1] - margin),
        number(width + 2.0 * margin),
        number(height + 2.0 * margin),
        number(extent.min[0] - margin),
        number(extent.min[1] - margin),
        number(width + 2.0 * margin),
        number(height + 2.0 * margin),
        number(extent.min[1] * 2.0 + height),
        number(width.max(height) / 400.0),
    )
}

