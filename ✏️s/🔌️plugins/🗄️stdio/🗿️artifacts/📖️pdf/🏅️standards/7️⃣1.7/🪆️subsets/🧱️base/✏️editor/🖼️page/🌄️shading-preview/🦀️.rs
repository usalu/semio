//! 🌄️ Projects supported PDF shadings onto the editor canvas.

use super::*;
use semio_framework_artifact_reference::ArtifactRef;
use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::foreign_artifacts::{NativePdfArtifactResources, PdfArtifactResourcePort};
use semio_s_artifact_stdio_pdf_paint_codecs as paint_codecs;
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::PdfShading;

const AXIAL_STOPS: usize = 64;
const COLOUR_TABLE: usize = 256;
const MAX_SHADING_EDGE: f64 = 512.0;

pub fn shading_layer(snapshot: &PdfSnapshot, page: &PdfPage, item: &Placed) -> Option<Value> {
    let shading = snapshot.shadings.iter().find(|shading| shading.id == item.object.text)?;
    match &shading.kind {
        PdfShadingKind::Axial { coords, domain, function, extend } => axial_layer(snapshot, page, item, shading, *coords, *domain, function, *extend),
        PdfShadingKind::Radial { coords, domain, function, extend } => radial_layer(snapshot, page, item, shading, *coords, *domain, function, *extend),
        PdfShadingKind::FunctionBased { domain, matrix, function } => function_layer(snapshot, page, item, shading, domain.unwrap_or([0.0, 1.0, 0.0, 1.0]), matrix.unwrap_or(PDF_IDENTITY_MATRIX), function),
        PdfShadingKind::Mesh { shading_type, bits_per_coordinate, bits_per_component, bits_per_flag, vertices_per_row, decode, function, reference } => mesh_layer(snapshot, page, item, shading, paint_codecs::mesh::MeshParameters {
            shading_type: *shading_type, bits_per_coordinate: *bits_per_coordinate, bits_per_component: *bits_per_component, bits_per_flag: bits_per_flag.unwrap_or(8), vertices_per_row: vertices_per_row.unwrap_or(0),
            decode: decode.clone(), components: if function.is_some() { 1 } else { shading.color_space.components()? as usize }, pad_vertices: true,
        }, function.as_ref(), reference),
    }
}

fn colour_at(snapshot: &PdfSnapshot, shading: &PdfShading, function: &PdfFunction, input: &[f64]) -> Option<[f64; 3]> {
    function_colour::rgb(snapshot, &shading.color_space, &function_colour::evaluate(function, input)?)
}

#[allow(clippy::too_many_arguments, reason = "one argument per shading dictionary entry")]
fn axial_layer(snapshot: &PdfSnapshot, page: &PdfPage, item: &Placed, shading: &PdfShading, [x1, y1, x2, y2]: [f64; 4], domain: Option<[f64; 2]>, function: &PdfFunction, extend: [bool; 2]) -> Option<Value> {
    let squared_length = (x2 - x1).powi(2) + (y2 - y1).powi(2);
    if !squared_length.is_finite() || squared_length <= 0.0 {
        return None;
    }
    let (box_coords, to_local) = match shading.bbox {
        Some(rect) => (rect, PDF_IDENTITY_MATRIX),
        None => (page.media_box, invert(item.object.ctm)?),
    };
    let [left, bottom, right, top] = box_coords;
    let mut polygon = vec![apply(to_local, left, bottom), apply(to_local, right, bottom), apply(to_local, right, top), apply(to_local, left, top)];
    let projection = |(x, y): (f64, f64)| ((x - x1) * (x2 - x1) + (y - y1) * (y2 - y1)) / squared_length;
    if !extend[0] {
        polygon = clip_polygon(&polygon, &projection);
    }
    if !extend[1] {
        polygon = clip_polygon(&polygon, |point| 1.0 - projection(point));
    }
    let mut segments = Vec::new();
    for (index, point) in polygon.into_iter().enumerate() {
        segments.push(json!({ "kind": if index == 0 { "move" } else { "line" }, "to": point }));
    }
    if !segments.is_empty() {
        segments.push(json!({ "kind": "close" }));
    }
    let [start, end] = domain.unwrap_or([0.0, 1.0]);
    let linear = matches!(function, PdfFunction::Exponential { domain, range: None, n, .. } if *n == 1.0 && domain.len() == 2 && start >= domain[0] && end <= domain[1])
        && matches!(shading.color_space, PdfColorSpace::DeviceGray | PdfColorSpace::DeviceRgb);
    let steps = if linear { 1 } else { AXIAL_STOPS };
    let mut stops = Vec::with_capacity(steps + 1);
    for index in 0..=steps {
        let offset = index as f64 / steps as f64;
        let [r, g, b] = colour_at(snapshot, shading, function, &[start + offset * (end - start)])?;
        stops.push(json!({ "offset": offset, "color": [r, g, b, 1.0] }));
    }
    let (x, y, a, b, c, d) = shading_basis(page, item);
    Some(json!({
        "id": item.object.id,
        "segments": segments,
        "transform": [a - x, b - y, c - x, d - y, x, y],
        "fill": { "kind": "linearGradient", "x1": x1, "y1": y1, "x2": x2, "y2": y2, "stops": stops },
    }))
}

fn shading_basis(page: &PdfPage, item: &Placed) -> (f64, f64, f64, f64, f64, f64) {
    let display = |x, y| {
        let (x, y) = apply(item.object.ctm, x, y);
        display_point(page, item.top, x, y)
    };
    let (x, y) = display(0.0, 0.0);
    let (a, b) = display(1.0, 0.0);
    let (c, d) = display(0.0, 1.0);
    (x, y, a, b, c, d)
}

#[allow(clippy::too_many_arguments, reason = "one argument per shading dictionary entry")]
fn radial_layer(snapshot: &PdfSnapshot, page: &PdfPage, item: &Placed, shading: &PdfShading, [x0, y0, r0, x1, y1, r1]: [f64; 6], domain: Option<[f64; 2]>, function: &PdfFunction, extend: [bool; 2]) -> Option<Value> {
    if r0 < 0.0 || r1 < 0.0 || !(x0 + y0 + r0 + x1 + y1 + r1).is_finite() {
        return None;
    }
    let [start, end] = domain.unwrap_or([0.0, 1.0]);
    let table = colour_table(snapshot, shading, function, start, end)?;
    let (dx, dy, dr) = (x1 - x0, y1 - y0, r1 - r0);
    let a = dx * dx + dy * dy - dr * dr;
    let paint = |x: f64, y: f64| {
        let (px, py) = (x - x0, y - y0);
        let b = px * dx + py * dy + r0 * dr;
        let c = px * px + py * py - r0 * r0;
        let roots = if a.abs() < 1e-12 {
            if b.abs() < 1e-12 { return None; }
            [Some(c / (2.0 * b)), None]
        } else {
            let discriminant = b * b - a * c;
            if discriminant < 0.0 { return None; }
            let root = discriminant.sqrt();
            let (high, low) = ((b + root) / a, (b - root) / a);
            [Some(high.max(low)), Some(high.min(low))]
        };
        let valid = |s: &f64| r0 + s * dr >= 0.0 && (*s >= 0.0 || extend[0]) && (*s <= 1.0 || extend[1]);
        roots.into_iter().flatten().find(valid).map(|s| table[(s.clamp(0.0, 1.0) * (COLOUR_TABLE - 1) as f64).round() as usize])
    };
    raster_shading(page, item, shading, paint)
}

#[allow(clippy::too_many_arguments, reason = "one argument per mesh shading dictionary entry")]
fn mesh_layer(snapshot: &PdfSnapshot, page: &PdfPage, item: &Placed, shading: &PdfShading, parameters: paint_codecs::mesh::MeshParameters, function: Option<&PdfFunction>, reference: &ArtifactRef) -> Option<Value> {
    let resources = NativePdfArtifactResources::from_objects(&snapshot.objects);
    let PdfObject::Stream { data, .. } = resources.resolve(reference).ok()? else { return None };
    let triangles = paint_codecs::mesh::triangles(&data.to_vec_owner(), &parameters, &mut |_| true).ok()?;
    let (x, y, a, b, c, d) = shading_basis(page, item);
    let [region_x, region_y, region_w, region_h] = if shading.bbox.is_some() { [item.canvas_x, item.canvas_y, item.canvas_w, item.canvas_h] } else { item.viewport.clip_rect() };
    if region_w <= 0.0 || region_h <= 0.0 { return None; }
    let scale = (MAX_SHADING_EDGE / region_w.max(region_h)).min(1.0);
    let (width, height) = ((region_w * scale).round().max(1.0) as u32, (region_h * scale).round().max(1.0) as u32);
    let (sx, sy) = (f64::from(width) / region_w, f64::from(height) / region_h);
    let transform = [(a - x) * sx, (b - y) * sy, (c - x) * sx, (d - y) * sy, (x - region_x) * sx, (y - region_y) * sy];
    let components = parameters.components;
    let shade = |values: &[f64; 4]| match function { Some(function) => colour_at(snapshot, shading, function, &[values[0]]), None => function_colour::rgb(snapshot, &shading.color_space, &values[..components]) };
    let pixels = paint_codecs::mesh::rasterize(&triangles, transform, width, height, &shade, &mut |_| true).ok()?;
    let mut raster = semio_framework_pixels::RasterImage::new(width, height);
    raster.pixels = pixels;
    raster_layer(item, [region_x, region_y, region_w, region_h], &raster)
}

fn function_layer(snapshot: &PdfSnapshot, page: &PdfPage, item: &Placed, shading: &PdfShading, domain: [f64; 4], matrix: PdfMatrix, function: &PdfFunction) -> Option<Value> {
    let inverse = invert(matrix)?;
    let paint = |x: f64, y: f64| {
        let (u, v) = apply(inverse, x, y);
        if u < domain[0] || u > domain[1] || v < domain[2] || v > domain[3] { return None; }
        colour_at(snapshot, shading, function, &[u, v])
    };
    raster_shading(page, item, shading, paint)
}

fn colour_table(snapshot: &PdfSnapshot, shading: &PdfShading, function: &PdfFunction, start: f64, end: f64) -> Option<Vec<[f64; 3]>> {
    (0..COLOUR_TABLE).map(|index| colour_at(snapshot, shading, function, &[start + index as f64 / (COLOUR_TABLE - 1) as f64 * (end - start)])).collect()
}

fn raster_shading(page: &PdfPage, item: &Placed, shading: &PdfShading, paint: impl Fn(f64, f64) -> Option<[f64; 3]>) -> Option<Value> {
    let (x, y, a, b, c, d) = shading_basis(page, item);
    let inverse = invert([a - x, b - y, c - x, d - y, x, y])?;
    let local_clip = shading.bbox.map(|[left, bottom, right, top]| (left.min(right), bottom.min(top), left.max(right), bottom.max(top)));
    let [region_x, region_y, region_w, region_h] = if shading.bbox.is_some() { [item.canvas_x, item.canvas_y, item.canvas_w, item.canvas_h] } else { item.viewport.clip_rect() };
    if region_w <= 0.0 || region_h <= 0.0 { return None; }
    let scale = (MAX_SHADING_EDGE / region_w.max(region_h)).min(1.0);
    let (width, height) = ((region_w * scale).round().max(1.0) as u32, (region_h * scale).round().max(1.0) as u32);
    let mut raster = semio_framework_pixels::RasterImage::new(width, height);
    for (index, rgba) in raster.pixels.chunks_exact_mut(4).enumerate() {
        let canvas_x = region_x + region_w * ((index % width as usize) as f64 + 0.5) / f64::from(width);
        let canvas_y = region_y + region_h * ((index / width as usize) as f64 + 0.5) / f64::from(height);
        let (sx, sy) = apply(inverse, canvas_x, canvas_y);
        if let Some((left, bottom, right, top)) = local_clip {
            if sx < left || sx > right || sy < bottom || sy > top { continue; }
        }
        if let Some(rgb) = paint(sx, sy) {
            rgba.copy_from_slice(&[rgb[0], rgb[1], rgb[2], 1.0].map(|value| (value * 255.0).round() as u8));
        }
    }
    raster_layer(item, [region_x, region_y, region_w, region_h], &raster)
}

fn clip_polygon(polygon: &[(f64, f64)], signed_distance: impl Fn((f64, f64)) -> f64) -> Vec<(f64, f64)> {
    let Some(&last) = polygon.last() else { return Vec::new() };
    let mut clipped = Vec::new();
    let mut previous = last;
    let mut previous_distance = signed_distance(previous);
    for &point in polygon {
        let distance = signed_distance(point);
        if (distance < 0.0) != (previous_distance < 0.0) {
            let ratio = previous_distance / (previous_distance - distance);
            clipped.push((previous.0 + ratio * (point.0 - previous.0), previous.1 + ratio * (point.1 - previous.1)));
        }
        if distance >= 0.0 {
            clipped.push(point);
        }
        previous = point;
        previous_distance = distance;
    }
    clipped
}
