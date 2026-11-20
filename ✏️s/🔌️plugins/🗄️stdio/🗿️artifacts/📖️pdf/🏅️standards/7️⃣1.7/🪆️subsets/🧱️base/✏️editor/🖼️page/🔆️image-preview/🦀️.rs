//! 🔆️ Renders embedded PDF image samples for the editor canvas.

use super::*;
use semio_framework_artifact_reference::ArtifactRef;
use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::io::{binary::snapshot::decoded_components::JpgComponentDecoder, ycbcr_to_rgb};
use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::{
    io::foreign_artifacts::{require_pdf_artifact_kind, NativePdfArtifactResources, PdfArtifactResourcePort},
    modules::images::PdfImagePreviewSamples,
};
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::PdfStreamFilter;

const MAX_PREVIEW_EDGE: f64 = 256.0;
const MAX_DCT_WORKING_BYTES: usize = 96 * 1024 * 1024;

pub(super) fn image_layer(snapshot: &PdfSnapshot, content: &[PdfOp], item: &Placed) -> Option<Value> {
    let page = &snapshot.pages[item.object.page];
    let resources = NativePdfArtifactResources::from_objects(&snapshot.objects);
    let resolve = |reference: &ArtifactRef, size, channels| dct_samples(&resources, reference, size, channels);
    let samples = match content.get(item.object.end) {
        Some(PdfOp::InlineImage { image }) => PdfImagePreviewSamples::inline(image, &resolve)?,
        _ => PdfImagePreviewSamples::xobject(snapshot.image(&item.object.text)?, |id| snapshot.image(id), &resolve)?,
    };
    let (width, height) = samples.dimensions();

    let display = |u, v| {
        let (x, y) = apply(item.object.ctm, u, 1.0 - v);
        display_point(page, item.top, x, y)
    };
    let (x0, y0) = display(0.0, 0.0);
    let (x1, y1) = display(1.0, 0.0);
    let (x2, y2) = display(0.0, 1.0);
    let (a, b, c, d) = (x1 - x0, y1 - y0, x2 - x0, y2 - y0);
    let inverse = invert([a, b, c, d, x0, y0])?;
    let horizontal = f64::from(width) / a.hypot(b);
    let vertical = f64::from(height) / c.hypot(d);
    let columns = a.abs() * horizontal + c.abs() * vertical;
    let rows = b.abs() * horizontal + d.abs() * vertical;
    let scale = (MAX_PREVIEW_EDGE / columns.max(rows)).min(1.0);
    let preview_width = (columns * scale).round().max(1.0) as u32;
    let preview_height = (rows * scale).round().max(1.0) as u32;
    let mut raster = semio_framework_pixels::RasterImage::new(preview_width, preview_height);
    for (index, rgba) in raster.pixels.chunks_exact_mut(4).enumerate() {
        let col = index % preview_width as usize;
        let row = index / preview_width as usize;
        let x = item.canvas_x + item.canvas_w * (col as f64 + 0.5) / f64::from(preview_width);
        let y = item.canvas_y + item.canvas_h * (row as f64 + 0.5) / f64::from(preview_height);
        let (u, v) = apply(inverse, x, y);
        rgba.copy_from_slice(&samples.rgba(u, v));
    }
    let png = semio_framework_pixels::encode_png(&raster).ok()?;
    let url = format!("data:image/png;base64,{}", semio_framework_io_base64::base64_standard_encode(png));
    Some(json!({
        "id": item.object.id,
        "kind": "image",
        "x": item.canvas_x,
        "y": item.canvas_y,
        "width": item.canvas_w,
        "height": item.canvas_h,
        "transform": [1.0, 0.0, 0.0, 1.0, item.canvas_x, item.canvas_y],
        "image": { "src": url },
    }))
}

fn dct_samples(resources: &impl PdfArtifactResourcePort, reference: &ArtifactRef, size: (u32, u32), channels: usize) -> Option<Vec<u8>> {
    require_pdf_artifact_kind(reference, "s.stdio.jpeg").ok()?;
    let PdfObject::Stream { data, filters, .. } = resources.resolve(reference).ok()? else { return None };
    let PdfStreamFilter::Dct { color_transform } = filters.iter().find(|filter| filter.is_image_codec())? else { return None };
    let source = data.as_slice();
    let mut decoder = JpgComponentDecoder::new(&source, MAX_DCT_WORKING_BYTES, &mut || false).ok()?;
    let transform = decoder.adobe_color_transform().map(u32::from).or(*color_transform).unwrap_or(1);
    if channels == 3 && !matches!(transform, 0 | 1) { return None; }
    let mut decoded = loop {
        if let Some(decoded) = decoder.step(&source, 4096, &mut || false).ok()? { break decoded; }
    };
    if (decoded.width, decoded.height) != size || decoded.component_ids.len() != channels { return None; }
    if channels == 3 && transform == 1 {
        for pixel in decoded.samples.chunks_exact_mut(3) {
            let (r, g, b) = ycbcr_to_rgb(f64::from(pixel[0]), f64::from(pixel[1]), f64::from(pixel[2]));
            pixel.copy_from_slice(&[r, g, b]);
        }
    }
    Some(decoded.samples)
}
