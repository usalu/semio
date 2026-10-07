//! 📤️ `s.stdio.semio/v1/drawing` → `png` (1.2) — this repository's own anti-aliased vector
//! rasterizer, so every domain artifact that projects into a drawing gets a real raster export.
//!
//! Painting model (the same one the sibling svg leaf writes, so an svg renderer is a usable oracle):
//! visible layers in order, each node painted source-over into a premultiplied canvas; a `Path`
//! fills with the non-zero rule (every subpath implicitly closed) and then strokes (butt caps,
//! miter joins, SVG's default miter limit 4); a node without a style fills black and does not
//! stroke, exactly SVG's defaults; `opacity` multiplies both paints. Coverage is exact signed-area
//! accumulation per pixel, clamped to one, so edges are anti-aliased without supersampling.
//!
//! 🔖 Documented lossiness (`IoFidelity::Lossy`): `Text` nodes are not painted (no font program is
//! carried by the drawing); `Image` nodes are painted only for `image/png` payloads, nearest
//! sampled; the raster is `canvas` units at one pixel each, uniformly shrunk so neither edge
//! exceeds [`DRAWING_RASTER_MAX_EDGE`].
//!
//! @see https://www.w3.org/TR/SVG11/painting.html
//! @see https://www.w3.org/TR/SVG11/implnote.html#ArcImplementationNotes

use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioRgba, SemioTransform};
use crate::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot};
use crate::standards::v1::subsets::drawing::schema::geometry::{Affine, Subpath, compose_affine, distance, flatten, semio_transform_affine, transform_point};
use {semio_framework_plugin::ArtifactSerializer,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_png::{schema::snapshot::{PngImage, PngColorType}, PngSnapshot};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("drawing") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };

/// 📏️ Largest raster edge in pixels; larger canvases are scaled down uniformly.
pub const DRAWING_RASTER_MAX_EDGE: f64 = 4096.0;
const SVG_DEFAULT_MITER_LIMIT: f64 = 4.0;

//#region 🔖️Raster
/// 🖼️ One straight (non-premultiplied) RGBA8 raster, row-major, top row first.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SemioRaster {
    pub width: u32,
    pub height: u32,
    pub rgba8: Vec<u8>,
}

struct Canvas {
    width: usize,
    height: usize,
    premultiplied: Vec<[f32; 4]>,
    accumulation: Vec<f32>,
    dirty: Option<[usize; 4]>,
}

impl Canvas {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn new(width: usize, height: usize, background: Option<&SemioRgba>) -> Self {
        let fill = background.map_or([0.0; 4], |c| [c.r * c.a, c.g * c.a, c.b * c.a, c.a]);
        Self { width, height, premultiplied: vec![fill; width * height], accumulation: vec![0.0; (width + 2) * height], dirty: None }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn edge(&mut self, from: [f64; 2], to: [f64; 2]) {
        let stride = self.width + 2;
        let right = self.width as f32;
        let (p0, p1) = ([from[0] as f32, from[1] as f32], [to[0] as f32, to[1] as f32]);
        if (p0[1] - p1[1]).abs() <= f32::EPSILON || !(p0[0].is_finite() && p0[1].is_finite() && p1[0].is_finite() && p1[1].is_finite()) {
            return;
        }
        let (direction, top, bottom) = if p0[1] < p1[1] { (1.0f32, p0, p1) } else { (-1.0f32, p1, p0) };
        let dxdy = (bottom[0] - top[0]) / (bottom[1] - top[1]);
        let first_row = top[1].max(0.0).floor() as usize;
        let last_row = (bottom[1].ceil().max(0.0) as usize).min(self.height);
        for row in first_row..last_row {
            let row_top = (row as f32).max(top[1]);
            let row_bottom = ((row + 1) as f32).min(bottom[1]);
            let dy = row_bottom - row_top;
            if dy <= 0.0 {
                continue;
            }
            let x_at_top = (top[0] + (row_top - top[1]) * dxdy).clamp(0.0, right);
            let x_at_bottom = (top[0] + (row_bottom - top[1]) * dxdy).clamp(0.0, right);
            let d = dy * direction;
            let (x0, x1) = if x_at_top < x_at_bottom { (x_at_top, x_at_bottom) } else { (x_at_bottom, x_at_top) };
            let line = row * stride;
            let touched = [row, row, x0.floor() as usize, x1.ceil() as usize + 1];
            self.dirty = Some(self.dirty.map_or(touched, |[r0, r1, c0, c1]| [r0.min(row), r1.max(row), c0.min(touched[2]), c1.max(touched[3])]));
            let x0_floor = x0.floor();
            let x0i = x0_floor as usize;
            let x1_ceil = x1.ceil();
            let x1i = x1_ceil as usize;
            if x1i <= x0i + 1 {
                let middle = 0.5 * (x0 + x1) - x0_floor;
                self.accumulation[line + x0i] += d - d * middle;
                self.accumulation[line + x0i + 1] += d * middle;
            } else {
                let inverse_width = (x1 - x0).recip();
                let x0_fraction = x0 - x0_floor;
                let a0 = 0.5 * inverse_width * (1.0 - x0_fraction) * (1.0 - x0_fraction);
                let x1_fraction = x1 - x1_ceil + 1.0;
                let am = 0.5 * inverse_width * x1_fraction * x1_fraction;
                self.accumulation[line + x0i] += d * a0;
                if x1i == x0i + 2 {
                    self.accumulation[line + x0i + 1] += d * (1.0 - a0 - am);
                } else {
                    let a1 = inverse_width * (1.5 - x0_fraction);
                    self.accumulation[line + x0i + 1] += d * (a1 - a0);
                    for column in x0i + 2..x1i - 1 {
                        self.accumulation[line + column] += d * inverse_width;
                    }
                    let a2 = a1 + (x1i - x0i - 3) as f32 * inverse_width;
                    self.accumulation[line + x1i - 1] += d * (1.0 - a2 - am);
                }
                self.accumulation[line + x1i] += d * am;
            }
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn polygon(&mut self, points: &[[f64; 2]]) {
        for index in 0..points.len() {
            self.edge(points[index], points[(index + 1) % points.len()]);
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn composite(&mut self, color: [f32; 4]) {
        let stride = self.width + 2;
        let Some([first_row, last_row, first_column, last_column]) = self.dirty.take() else { return };
        for row in first_row..=last_row {
            let mut winding = 0.0f32;
            for column in first_column..=last_column.min(stride - 1) {
                winding += self.accumulation[row * stride + column];
                self.accumulation[row * stride + column] = 0.0;
                if column >= self.width {
                    continue;
                }
                let alpha = winding.abs().min(1.0) * color[3];
                if alpha <= 0.0 {
                    continue;
                }
                let pixel = &mut self.premultiplied[row * self.width + column];
                for channel in 0..3 {
                    pixel[channel] = color[channel] * alpha + pixel[channel] * (1.0 - alpha);
                }
                pixel[3] = alpha + pixel[3] * (1.0 - alpha);
            }
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn blit(&mut self, inverse: &Affine, source: &SemioRaster, size: [f64; 2], alpha: f32) {
        for row in 0..self.height {
            for column in 0..self.width {
                let local = transform_point(inverse, [column as f64 + 0.5, row as f64 + 0.5]);
                if local[0] < 0.0 || local[1] < 0.0 || local[0] >= size[0] || local[1] >= size[1] {
                    continue;
                }
                let sx = ((local[0] / size[0]) * source.width as f64) as usize;
                let sy = ((local[1] / size[1]) * source.height as f64) as usize;
                let at = (sy.min(source.height as usize - 1) * source.width as usize + sx.min(source.width as usize - 1)) * 4;
                let Some(texel) = source.rgba8.get(at..at + 4) else { continue };
                let a = texel[3] as f32 / 255.0 * alpha;
                let pixel = &mut self.premultiplied[row * self.width + column];
                for channel in 0..3 {
                    pixel[channel] = texel[channel] as f32 / 255.0 * a + pixel[channel] * (1.0 - a);
                }
                pixel[3] = a + pixel[3] * (1.0 - a);
            }
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn into_raster(self) -> SemioRaster {
        let mut rgba8 = Vec::with_capacity(self.width * self.height * 4);
        for pixel in &self.premultiplied {
            let alpha = pixel[3].clamp(0.0, 1.0);
            let unpremultiply = |value: f32| if alpha > 0.0 { ((value / alpha).clamp(0.0, 1.0) * 255.0).round() as u8 } else { 0 };
            rgba8.extend_from_slice(&[unpremultiply(pixel[0]), unpremultiply(pixel[1]), unpremultiply(pixel[2]), (alpha * 255.0).round() as u8]);
        }
        SemioRaster { width: self.width as u32, height: self.height as u32, rgba8 }
    }
}
//#endregion 🔖️Raster



//#region 🔖️Stroke
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn positively_wound(mut polygon: Vec<[f64; 2]>) -> Vec<[f64; 2]> {
    let area: f64 = (0..polygon.len()).map(|i| {
        let (a, b) = (polygon[i], polygon[(i + 1) % polygon.len()]);
        a[0] * b[1] - b[0] * a[1]
    }).sum();
    if area < 0.0 {
        polygon.reverse();
    }
    polygon
}

/// 🖊️ The stroke outline of one subpath as positively wound polygons (segment quads plus miter or
/// bevel joins), in the subpath's own coordinates.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn stroke_polygons(subpath: &Subpath, half_width: f64) -> Vec<Vec<[f64; 2]>> {
    let mut points: Vec<[f64; 2]> = Vec::with_capacity(subpath.points.len());
    for p in &subpath.points {
        if points.last().is_none_or(|last| distance(*last, *p) > 1e-9) {
            points.push(*p);
        }
    }
    if subpath.closed && points.len() > 2 && distance(points[0], *points.last().expect("non-empty")) <= 1e-9 {
        points.pop();
    }
    if points.len() < 2 {
        return Vec::new();
    }
    let segment_count = if subpath.closed { points.len() } else { points.len() - 1 };
    let direction = |i: usize| {
        let (a, b) = (points[i % points.len()], points[(i + 1) % points.len()]);
        let length = distance(a, b);
        [(b[0] - a[0]) / length, (b[1] - a[1]) / length]
    };
    let mut polygons = Vec::new();
    for i in 0..segment_count {
        let (a, b) = (points[i], points[(i + 1) % points.len()]);
        let d = direction(i);
        let n = [-d[1] * half_width, d[0] * half_width];
        polygons.push(positively_wound(vec![[a[0] + n[0], a[1] + n[1]], [b[0] + n[0], b[1] + n[1]], [b[0] - n[0], b[1] - n[1]], [a[0] - n[0], a[1] - n[1]]]));
    }
    let joints: Vec<usize> = if subpath.closed { (0..points.len()).collect() } else { (1..points.len() - 1).collect() };
    for joint in joints {
        let incoming = direction((joint + points.len() - 1) % points.len());
        let outgoing = direction(joint);
        let turn = incoming[0] * outgoing[1] - incoming[1] * outgoing[0];
        if turn.abs() < 1e-12 {
            continue;
        }
        let side = if turn > 0.0 { -1.0 } else { 1.0 };
        let v = points[joint];
        let n_in = [-incoming[1] * half_width * side, incoming[0] * half_width * side];
        let n_out = [-outgoing[1] * half_width * side, outgoing[0] * half_width * side];
        let a = [v[0] + n_in[0], v[1] + n_in[1]];
        let b = [v[0] + n_out[0], v[1] + n_out[1]];
        let cos_phi = ((n_in[0] * n_out[0] + n_in[1] * n_out[1]) / (half_width * half_width)).clamp(-1.0, 1.0);
        let cos_half = ((1.0 + cos_phi) / 2.0).sqrt();
        let polygon = if cos_half > 1.0 / SVG_DEFAULT_MITER_LIMIT {
            let bisector = [n_in[0] + n_out[0], n_in[1] + n_out[1]];
            let length = (bisector[0].powi(2) + bisector[1].powi(2)).sqrt();
            let reach = half_width / cos_half;
            vec![v, a, [v[0] + bisector[0] / length * reach, v[1] + bisector[1] / length * reach], b]
        } else {
            vec![v, a, b]
        };
        polygons.push(positively_wound(polygon));
    }
    polygons
}
//#endregion 🔖️Stroke

//#region 🔖️Paint
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn premultiplied_paint(color: &SemioRgba, opacity: f32) -> [f32; 4] {
    [color.r.clamp(0.0, 1.0), color.g.clamp(0.0, 1.0), color.b.clamp(0.0, 1.0), (color.a * opacity).clamp(0.0, 1.0)]
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn paint_node(canvas: &mut Canvas, node: &DrawNode, styles: &[DrawStyle], matrix: &Affine) {
    match node {
        DrawNode::Group { transform, children } => {
            let inner = compose_affine(matrix, &semio_transform_affine(transform));
            for child in children {
                paint_node(canvas, child, styles, &inner);
            }
        }
        DrawNode::Path { segments, style } => {
            let style = style.as_deref().and_then(|name| styles.iter().find(|s| s.name == name));
            let black = SemioRgba { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
            let (fill, stroke, width, opacity) = match style {
                Some(s) => (s.fill, s.stroke, s.stroke_width.unwrap_or(1.0), s.opacity.unwrap_or(1.0)),
                None => (Some(black), None, 1.0, 1.0),
            };
            let pixel_scale = (matrix[0] * matrix[3] - matrix[1] * matrix[2]).abs().sqrt().max(1e-9);
            let subpaths = flatten(segments, pixel_scale);
            if let Some(fill) = fill {
                for subpath in &subpaths {
                    let transformed: Vec<[f64; 2]> = subpath.points.iter().map(|p| transform_point(matrix, *p)).collect();
                    canvas.polygon(&transformed);
                }
                canvas.composite(premultiplied_paint(&fill, opacity));
            }
            if let Some(stroke) = stroke.filter(|_| width > 0.0) {
                for subpath in &subpaths {
                    for polygon in stroke_polygons(subpath, width / 2.0) {
                        let transformed: Vec<[f64; 2]> = polygon.iter().map(|p| transform_point(matrix, *p)).collect();
                        canvas.polygon(&transformed);
                    }
                }
                canvas.composite(premultiplied_paint(&stroke, opacity));
            }
        }
        DrawNode::Image { at, width, height, mime, bytes } => {
            if mime != "image/png" || *width <= 0.0 || *height <= 0.0 {
                return;
            }
            let Ok(snapshot) = semio_s_artifact_stdio_png::standards::v1_2::subsets::any::io::decode_png(bytes) else { return };
            let Ok(rgba8) = semio_s_artifact_stdio_png::schema::operations::png_rgba8_preview(&snapshot.image) else { return };
            let source = SemioRaster { width: snapshot.image.width, height: snapshot.image.height, rgba8 };
            if source.width == 0 || source.height == 0 { return; }
            let placed = compose_affine(matrix, &[1.0, 0.0, 0.0, 1.0, at.x, at.y]);
            let determinant = placed[0] * placed[3] - placed[1] * placed[2];
            if determinant.abs() < 1e-12 {
                return;
            }
            let inverse = [placed[3] / determinant, -placed[1] / determinant, -placed[2] / determinant, placed[0] / determinant, (placed[2] * placed[5] - placed[3] * placed[4]) / determinant, (placed[1] * placed[4] - placed[0] * placed[5]) / determinant];
            canvas.blit(&inverse, &source, [*width, *height], 1.0);
        }
        DrawNode::Text { .. } => {}
    }
}

/// 🖼️ Paints every visible layer of `drawing` into an RGBA8 raster (see the module doc for the model).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn rasterize_drawing(drawing: &SemioDrawingSnapshot) -> Result<SemioRaster, String> {
    let (width, height) = (drawing.canvas.width, drawing.canvas.height);
    if !(width.is_finite() && height.is_finite() && width > 0.0 && height > 0.0) {
        return Err("semio/drawing→png: the canvas has no positive finite size".into());
    }
    let scale = (DRAWING_RASTER_MAX_EDGE / width.max(height)).min(1.0);
    let (columns, rows) = ((width * scale).ceil().max(1.0) as usize, (height * scale).ceil().max(1.0) as usize);
    let mut canvas = Canvas::new(columns, rows, drawing.canvas.background.as_ref());
    let base = [scale, 0.0, 0.0, scale, 0.0, 0.0];
    for layer in drawing.layers.iter().filter(|layer| layer.visible) {
        paint_node(&mut canvas, &layer.root, &drawing.styles, &base);
    }
    Ok(canvas.into_raster())
}
//#endregion 🔖️Paint

//#region 🔖️Serializer
pub struct SemioDrawingToPng;

impl ArtifactSerializer for SemioDrawingToPng {
    type From = SemioDrawingSnapshot;
    type Into = PngSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let raster = rasterize_drawing(from).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))?;
        let snapshot = PngSnapshot { schema: semio_s_artifact_stdio_png::STDIO_PNG_DOCUMENT_SCHEMA.into(), image: PngImage {
            width: raster.width, height: raster.height, color_type: PngColorType::Rgba, bit_depth: 8,
            samples: raster.rgba8.into_iter().map(u16::from).collect(), ..Default::default()
        } };
        snapshot.validate().map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))?;
        Ok(snapshot)
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
