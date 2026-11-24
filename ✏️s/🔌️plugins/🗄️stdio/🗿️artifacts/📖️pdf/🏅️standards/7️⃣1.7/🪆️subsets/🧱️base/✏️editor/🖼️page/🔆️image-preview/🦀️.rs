//! 🔆️ Renders embedded PDF image samples for the editor canvas.

use super::*;
use semio_framework_artifact_reference::ArtifactRef;
use semio_s_artifact_stdio_pdf_paint_codecs as paint_codecs;
use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::io::{binary::snapshot::decoded_components::JpgComponentDecoder, ycbcr_to_rgb};
use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::{
    io::foreign_artifacts::{require_pdf_artifact_kind, NativePdfArtifactResources, PdfArtifactResourcePort},
    modules::images::PdfImagePreviewSamples,
};
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::{PdfImage, PdfImageBody, PdfStreamFilter};

const MAX_PREVIEW_EDGE: f64 = 256.0;
/// 🫧️ MuPDF smooths an image upscaled by less than this factor on both axes and keeps larger upscales as crisp samples.
const SMOOTH_UPSCALE_LIMIT: f64 = 4.0;
const MAX_DCT_WORKING_BYTES: usize = 96 * 1024 * 1024;

pub fn image_layer(snapshot: &PdfSnapshot, content: &[PdfOp], item: &Placed) -> Option<Value> {
    let page = &snapshot.pages[item.object.page];
    let resources = NativePdfArtifactResources::from_objects(&snapshot.objects);
    let resolve = |reference: &ArtifactRef, size, channels| semio_s_artifact_stdio_pdf_editor_document::graph_png_preview(&resources, reference, size, channels)
        .or_else(|| coded_samples(&resources, reference, size, channels));
    let convert = |space: &PdfColorSpace, components: &[f64]| function_colour::image_rgb(snapshot, space, components);
    let fill = item.object.fill;
    let inline = match content.get(item.object.end) { Some(PdfOp::InlineImage { image }) => Some(image), _ => None };
    let xobject = if inline.is_some() { None } else { Some(snapshot.image(&item.object.text)?) };
    let inferred = xobject.and_then(|image| infer_jpx_space(&resources, image));
    let samples = match (inline, xobject) {
        (Some(image), _) => PdfImagePreviewSamples::inline(image, fill, &convert, &resolve)?,
        (None, Some(image)) => PdfImagePreviewSamples::xobject(inferred.as_ref().unwrap_or(image), fill, &convert, |id| snapshot.image(id), &resolve)?,
        (None, None) => return None,
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
    let (upscale_x, upscale_y) = (a.hypot(b) / f64::from(width), c.hypot(d) / f64::from(height));
    let smooth = (1.0..SMOOTH_UPSCALE_LIMIT).contains(&upscale_x) && (1.0..SMOOTH_UPSCALE_LIMIT).contains(&upscale_y);
    let (horizontal, vertical) = if smooth { (1.0, 1.0) } else { (1.0 / upscale_x, 1.0 / upscale_y) };
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
        rgba.copy_from_slice(&if smooth { samples.rgba_smooth(u, v) } else { samples.rgba(u, v) });
    }
    raster_layer(item, [item.canvas_x, item.canvas_y, item.canvas_w, item.canvas_h], &raster)
}

/// 🧊️ Wraps a rendered preview raster as a canvas image layer covering `region`.
pub fn raster_layer(item: &Placed, [x, y, width, height]: [f64; 4], raster: &semio_framework_pixels::RasterImage) -> Option<Value> {
    let png = semio_framework_pixels::encode_png(raster).ok()?;
    let url = format!("data:image/png;base64,{}", semio_framework_io_base64::base64_standard_encode(png));
    Some(json!({
        "id": item.object.id,
        "kind": "image",
        "x": x,
        "y": y,
        "width": width,
        "height": height,
        "transform": [1.0, 0.0, 0.0, 1.0, x, y],
        "image": { "src": url },
    }))
}

fn infer_jpx_space(resources: &impl PdfArtifactResourcePort, image: &PdfImage) -> Option<PdfImage> {
    let PdfImageBody::Artifact { reference } = &image.body else { return None };
    if image.color_space.is_some() || image.image_mask { return None; }
    let PdfObject::Stream { data, filters, .. } = resources.resolve(reference).ok()? else { return None };
    filters.iter().find(|filter| matches!(filter, PdfStreamFilter::Jpx))?;
    let colour_components = paint_codecs::jpx::component_count(&data.to_vec_owner()).ok()?.checked_sub(usize::from(image.soft_mask_in_data == Some(1)))?;
    let space = match colour_components { 1 => PdfColorSpace::DeviceGray, 3 => PdfColorSpace::DeviceRgb, 4 => PdfColorSpace::DeviceCmyk, _ => return None };
    Some(PdfImage { color_space: Some(space), ..image.clone() })
}

fn coded_samples(resources: &impl PdfArtifactResourcePort, reference: &ArtifactRef, size: (u32, u32), channels: usize) -> Option<Vec<u8>> {
    let PdfObject::Stream { data, filters, .. } = resources.resolve(reference).ok()? else { return None };
    if filters.iter().all(|filter| !filter.is_image_codec()) {
        require_pdf_artifact_kind(reference, "s.stdio.samples").ok()?;
        let expected = (size.0 as usize).checked_mul(size.1 as usize)?.checked_mul(channels)?;
        return (matches!(filters.as_slice(), [] | [PdfStreamFilter::Flate { .. }]) && data.len() == expected).then(|| data.to_vec_owner());
    }
    let codec = filters.iter().find(|filter| filter.is_image_codec())?;
    let kind = match codec {
        PdfStreamFilter::Dct { .. } => "s.stdio.jpeg",
        PdfStreamFilter::Jpx => "s.stdio.jpeg2000",
        PdfStreamFilter::Ccitt { .. } => "s.stdio.ccitt",
        _ => "s.stdio.jbig2",
    };
    require_pdf_artifact_kind(reference, kind).ok()?;
    let source_bytes = data.to_vec_owner();
    let source = source_bytes.as_slice();
    match codec {
        PdfStreamFilter::Dct { color_transform } => dct_samples(source, *color_transform, size, channels),
        PdfStreamFilter::Jpx => {
            let image = paint_codecs::jpx::decode(source, &mut |_| true).ok()?;
            ((image.width, image.height) == size && image.components == channels).then_some(image.samples)
        }
        PdfStreamFilter::Ccitt { parameters } => {
            let columns = if parameters.columns == 0 { size.0 } else { parameters.columns };
            let packed = paint_codecs::ccitt::decode(source, &paint_codecs::ccitt::CcittParameters { k: parameters.k, columns, rows: size.1, black_is_1: parameters.black_is_1, byte_align: parameters.encoded_byte_align }, &mut |_| true).ok()?;
            let stride = columns.div_ceil(8) as usize;
            (channels == 1 && columns >= size.0).then(|| (0..size.1 as usize).flat_map(|y| (0..size.0 as usize).map(move |x| (y, x))).map(|(y, x)| packed[y * stride + x / 8] >> (7 - (x & 7)) & 1).collect())
        }
        PdfStreamFilter::Jbig2 { globals } => {
            let page = paint_codecs::jbig2::decode(globals.as_deref(), source, &mut |_| true).ok()?;
            ((page.width as u32, page.height as u32) == size && channels == 1).then(|| page.pixels.iter().map(|pixel| 1 - pixel).collect())
        }
        _ => None,
    }
}

/// 🎞️ Decodes DCT samples; four-component images are kept as stored planes because MuPDF 1.26.7 applies neither the Adobe YCCK transform nor `/ColorTransform` to them.
fn dct_samples(source: &[u8], color_transform: Option<u32>, size: (u32, u32), channels: usize) -> Option<Vec<u8>> {
    let mut decoder = JpgComponentDecoder::new(&source, MAX_DCT_WORKING_BYTES, &mut || false).ok()?;
    let transform = if channels == 4 { 0 } else { decoder.adobe_color_transform().map(u32::from).or(color_transform).unwrap_or(1) };
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
