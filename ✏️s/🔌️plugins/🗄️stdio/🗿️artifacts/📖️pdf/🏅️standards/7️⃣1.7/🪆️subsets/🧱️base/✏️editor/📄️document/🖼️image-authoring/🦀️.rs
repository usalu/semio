//! 🖼️ Places admitted local raster samples as ordinary PDF image XObjects.

use super::*;

#[path = "🧮️baseline-resource.rs"]
pub mod baseline_resource;

#[path = "📌️dct-placement.rs"]
pub mod dct_placement;

/// 📍 Addresses an insertion or one existing image placement in PDF page space.
#[derive(Clone, Copy)]
pub enum ImagePlacement {
    Insert { page: usize, matrix: PdfMatrix },
    Replace { page: usize, operator: usize },
}

/// 🌈 Straight RGBA8 pixels in image row order.
pub struct SampledRgba<'a> {
    pub width: u32,
    pub height: u32,
    pub pixels: &'a [u8],
}

/// 📏 Caps the sampled representation at one bounded component array per image.
pub const MAX_IMAGE_IMPORT_PIXELS: usize = 4_096;

/// 🔍 Validates a page address before reading a picked file or applying its samples.
pub fn check_image_placement(snapshot: &PdfSnapshot, placement: ImagePlacement) -> Result<usize, Fault> {
    let (page, replacement) = match placement {
        ImagePlacement::Insert { page, matrix } => {
            if !matrix.iter().all(|value| value.is_finite()) || matrix[0] <= 0.0 || matrix[3] <= 0.0 {
                return Err(fault("image placement requires finite coordinates and positive dimensions"));
            }
            (page, None)
        }
        ImagePlacement::Replace { page, operator } => (page, Some(operator)),
    };
    let target = snapshot.pages.get(page).ok_or_else(|| fault("PDF image page no longer exists"))?;
    if let Some(operator) = replacement {
        match target.content.get(operator) {
            Some(PdfOp::InlineImage { .. }) => {}
            Some(PdfOp::PaintXObject { name }) if snapshot.image(name).is_some() => {}
            _ => return Err(fault("PDF image placement no longer exists")),
        }
    }
    Ok(page)
}

/// 🖼️ Installs a fresh image, its optional soft mask and the selected paint in one edit.
pub fn place_sampled_image(snapshot: &PdfSnapshot, placement: ImagePlacement, raster: SampledRgba<'_>) -> Result<Vec<PdfMutation>, Fault> {
    let page = check_image_placement(snapshot, placement)?;
    let pixels = (raster.width as usize).checked_mul(raster.height as usize)
        .filter(|pixels| (1..=MAX_IMAGE_IMPORT_PIXELS).contains(pixels))
        .filter(|pixels| raster.pixels.len() == pixels * 4)
        .ok_or_else(|| fault("image pixel dimensions or sample counts exceed the import limit"))?;
    let mut rgb = Vec::with_capacity(pixels * 3);
    let mut alpha = Vec::with_capacity(pixels);
    for sample in raster.pixels.chunks_exact(4) {
        rgb.extend_from_slice(&sample[..3]);
        alpha.push(sample[3]);
    }
    let rgb = PdfImageBody::Samples { values: rgb.into_iter().map(u32::from).collect() };
    let alpha = alpha.iter().any(|sample| *sample != 255)
        .then(|| PdfImageBody::Samples { values: alpha.into_iter().map(u32::from).collect() });
    place_image_bodies(snapshot, placement, page, raster.width, raster.height, rgb, alpha)
}

fn place_image_bodies(
    snapshot: &PdfSnapshot, placement: ImagePlacement, page: usize, width: u32, height: u32,
    rgb: PdfImageBody, alpha: Option<PdfImageBody>,
) -> Result<Vec<PdfMutation>, Fault> {
    let mut names = image_names(snapshot);
    let id = names.next().expect("available image XObject id");
    let alpha = alpha.map(|body| (names.next().expect("available soft mask XObject id"), body));
    place_named_image_bodies(placement, page, width, height, id, rgb, alpha)
}

fn place_named_image_bodies(
    placement: ImagePlacement, page: usize, width: u32, height: u32,
    id: String, rgb: PdfImageBody, alpha: Option<(String, PdfImageBody)>,
) -> Result<Vec<PdfMutation>, Fault> {
    if width == 0 || height == 0 { return Err(fault("PDF image geometry is empty")); }
    if id.is_empty() || alpha.as_ref().is_some_and(|(mask, _)| mask.is_empty() || mask == &id) {
        return Err(fault("PDF image and mask require distinct reserved resource names"));
    }
    let mut image = PdfImage::rgb8(&id, width, height, Vec::new());
    image.body = rgb;
    let mut changes = Vec::with_capacity(3);
    if let Some((mask, alpha)) = alpha {
        let mut alpha_image = PdfImage::gray8(&mask, width, height, Vec::new());
        alpha_image.body = alpha;
        image.soft_mask = Some(mask);
        changes.push(PdfMutation::SetImage(SetImage { image: alpha_image, index: None }));
    }
    changes.push(PdfMutation::SetImage(SetImage { image, index: None }));
    changes.push(paint_image(page, placement, id));
    Ok(changes)
}

fn image_names(snapshot: &PdfSnapshot) -> impl Iterator<Item = String> + '_ {
    let taken: std::collections::HashSet<_> = snapshot.images.iter().map(|image| image.id.as_str())
        .chain(snapshot.forms.iter().map(|form| form.id.as_str())).collect();
    (1..).map(|index| format!("SemioImage{index}")).filter(move |name| !taken.contains(name.as_str()))
}

fn paint_image(page: usize, placement: ImagePlacement, id: String) -> PdfMutation {
    let paint = PdfOp::PaintXObject { name: id };
    match placement {
        ImagePlacement::Insert { matrix, .. } => PdfMutation::AppendPageContent(AppendPageContent {
            index: page,
            content: vec![PdfOp::Save, PdfOp::Transform { matrix }, paint, PdfOp::Restore],
        }),
        ImagePlacement::Replace { operator, .. } => PdfMutation::ReplaceContent(ReplaceContent { index: page, at: operator, op: paint }),
    }
}

#[path = "🎛️replace-button.rs"]
mod replace_button;
pub use replace_button::image_replacement_button;

#[path = "🧵️decode.rs"]
mod decode;
pub use decode::*;

#[path = "📥️admission.rs"]
mod admission;
#[path = "📂️picker-work.rs"]
mod picker_work;
pub use picker_work::PngPickerWork;
pub use admission::*;

#[path = "🧱️graph-writer.rs"]
mod graph_writer;
pub use graph_writer::PngGraphWriter;

#[path = "🏞️graph-placement.rs"]
mod graph_placement;
pub use graph_placement::{graph_png_preview, place_graph_png_named};

#[path = "🎞️decode-original.rs"]
mod original_decode;
use original_decode::RawPngWork;
#[path = "🧩️graph-edit.rs"]
mod graph_edit;
use graph_edit::PngGraphEdit;
#[path = "📬️format-work.rs"]
mod format_work;
pub use format_work::PngFormatWork;
#[path = "🧵️reserved-picker.rs"]
mod reserved_picker;
pub use reserved_picker::PickerPngWork;
