//! 🚪️ IO stdio.bmp (v3/✳️any) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_v3::subsets::any::io::BmpAnalyzer;
    use crate::BmpSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.bmp", standard: StandardId("v3"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

    pub struct BmpComposerComposition;

    impl ArtifactComposition for BmpComposerComposition {
        type Snapshot = BmpSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_BINARY]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_BINARY)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "BmpComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = BmpAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "BmpComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

/// 🧭 Checked BMP v3 layout projections and exact byte-preserving edits.
use crate::schema::snapshot::{BmpPaletteEntry, BmpRowOrder};
use crate::{BmpMutation, BmpSnapshot, STDIO_BMP_DOCUMENT_SCHEMA};

const BMP_MAGIC: [u8; 2] = *b"BM";
const BITMAPINFOHEADER_SIZE: u32 = 40;
const BI_RGB: u32 = 0;
const BI_BITFIELDS: u32 = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum BmpProfile {
    IndexedRgb1,
    IndexedRgb4,
    IndexedRgb8,
    DirectRgb16,
    DirectRgb24,
    DirectRgb32,
    DirectBitfields16,
    DirectBitfields32,
}

impl BmpProfile {
    pub const fn id(self) -> &'static str {
        match self {
            Self::IndexedRgb1 => "indexedRgb1",
            Self::IndexedRgb4 => "indexedRgb4",
            Self::IndexedRgb8 => "indexedRgb8",
            Self::DirectRgb16 => "directRgb16",
            Self::DirectRgb24 => "directRgb24",
            Self::DirectRgb32 => "directRgb32",
            Self::DirectBitfields16 => "directBitfields16",
            Self::DirectBitfields32 => "directBitfields32",
        }
    }

    pub fn is_indexed(self) -> bool {
        matches!(self, Self::IndexedRgb1 | Self::IndexedRgb4 | Self::IndexedRgb8)
    }

    pub fn is_direct(self) -> bool {
        !self.is_indexed()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct BmpRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct BmpColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BmpPngPreview {
    pub width: u32,
    pub height: u32,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BmpLayout {
    pub profile: BmpProfile,
    pub file_size: u32,
    pub reserved_1: u16,
    pub reserved_2: u16,
    pub data_offset: usize,
    pub width: u32,
    pub height: u32,
    pub row_order: BmpRowOrder,
    pub planes: u16,
    pub bits_per_pixel: u16,
    pub compression: u32,
    pub image_size: u32,
    pub x_pixels_per_meter: i32,
    pub y_pixels_per_meter: i32,
    pub colors_used: u32,
    pub colors_important: u32,
    pub masks: [u32; 4],
    pub palette_offset: usize,
    pub palette_entries: usize,
    pub metadata_end: usize,
    pub row_stride: usize,
    pub row_payload: usize,
    pub pixel_bytes: usize,
    pub pixel_end: usize,
}

pub fn empty_bmp_bytes() -> Vec<u8> {
    vec![0x42, 0x4d, 0x3a, 0, 0, 0, 0, 0, 0, 0, 0x36, 0, 0, 0, 0x28, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 24, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 0]
}
pub fn demo_bmp_bytes() -> Vec<u8> {
    let mut bytes = vec![0; 78];
    bytes[..2].copy_from_slice(&BMP_MAGIC);
    bytes[2..6].copy_from_slice(&78u32.to_le_bytes());
    bytes[10..14].copy_from_slice(&54u32.to_le_bytes());
    bytes[14..18].copy_from_slice(&BITMAPINFOHEADER_SIZE.to_le_bytes());
    bytes[18..22].copy_from_slice(&4i32.to_le_bytes());
    bytes[22..26].copy_from_slice(&2i32.to_le_bytes());
    bytes[26..28].copy_from_slice(&1u16.to_le_bytes());
    bytes[28..30].copy_from_slice(&24u16.to_le_bytes());
    bytes[34..38].copy_from_slice(&24u32.to_le_bytes());
    bytes[38..42].copy_from_slice(&2835i32.to_le_bytes());
    bytes[42..46].copy_from_slice(&2835i32.to_le_bytes());
    bytes[54..].copy_from_slice(&[255, 255, 0, 255, 0, 255, 255, 255, 255, 128, 128, 128, 0, 0, 255, 0, 255, 0, 255, 0, 0, 0, 255, 255]);
    bytes
}

fn range<'a>(bytes: &'a [u8], offset: usize, len: usize, name: &str) -> Result<&'a [u8], String> {
    bytes.get(offset..offset.checked_add(len).ok_or_else(|| format!("bmp: {name} range overflow"))?).ok_or_else(|| format!("bmp: truncated {name}"))
}

fn checked_row_geometry(width: u32, bits_per_pixel: u16) -> Result<(usize, usize), String> {
    let row_bits = u64::from(width).checked_mul(u64::from(bits_per_pixel)).ok_or_else(|| "bmp: row bit count overflow".to_string())?;
    let row_stride = row_bits.checked_add(31).ok_or_else(|| "bmp: row alignment overflow".to_string())? / 32 * 4;
    let row_payload = row_bits.checked_add(7).ok_or_else(|| "bmp: row payload overflow".to_string())? / 8;
    Ok((usize::try_from(row_stride).map_err(|_| "bmp: row stride exceeds address space")?, usize::try_from(row_payload).map_err(|_| "bmp: row payload exceeds address space")?))
}

pub(crate) fn row_bytes(width: u32, bits_per_pixel: u16) -> usize {
    checked_row_geometry(width, bits_per_pixel).expect("bounded BMP row geometry").0
}

pub fn bmp_layout(snapshot: &BmpSnapshot) -> Result<BmpLayout, String> {
    if snapshot.schema != STDIO_BMP_DOCUMENT_SCHEMA {
        return Err(format!("bmp: schema must be {STDIO_BMP_DOCUMENT_SCHEMA}"));
    }
    bmp_layout_bytes(&snapshot.bytes)
}

#[path = "🧩️layout/🦀️.rs"]
pub(crate) mod layout;

/// 📐️ Exposes the canonical borrowed layout grammar at the ordinary native message terminal.
pub fn bmp_layout_bytes(bytes: &[u8]) -> Result<BmpLayout, String> {
    layout::inspect(bytes).map_err(|failure| failure.to_string())
}

pub fn decode_bmp(bytes: &[u8]) -> Result<BmpSnapshot, String> {
    bmp_layout_bytes(bytes)?;
    Ok(BmpSnapshot { schema: STDIO_BMP_DOCUMENT_SCHEMA.into(), bytes: bytes.to_vec() })
}

pub fn bmp_direct_rgb24_from_rgba8(width: u32, height: u32, rgba8: &[u8], x_pixels_per_meter: i32, y_pixels_per_meter: i32) -> Result<BmpSnapshot, String> {
    if (width == 0) != (height == 0) {
        return Err("bmp: empty dimensions must both be zero".into());
    }
    if width > i32::MAX as u32 || height > i32::MAX as u32 {
        return Err("bmp: dimensions exceed the signed BITMAPINFOHEADER range".into());
    }
    let pixel_count = usize::try_from(width)
        .map_err(|_| "bmp: width exceeds address space")?
        .checked_mul(usize::try_from(height).map_err(|_| "bmp: height exceeds address space")?)
        .ok_or_else(|| "bmp: pixel count overflow".to_string())?;
    let expected = pixel_count.checked_mul(4).ok_or_else(|| "bmp: RGBA8 byte count overflow".to_string())?;
    if rgba8.len() != expected {
        return Err(format!("bmp: RGBA8 source has {} bytes; expected {expected}", rgba8.len()));
    }
    if rgba8.chunks_exact(4).any(|pixel| pixel[3] != 255) {
        return Err("bmp: Direct RGB24 cannot represent nonopaque RGBA8 pixels".into());
    }
    let (row_stride, _) = checked_row_geometry(width, 24)?;
    let pixel_bytes = row_stride.checked_mul(height as usize).ok_or_else(|| "bmp: pixel storage length overflow".to_string())?;
    let file_len = 54usize.checked_add(pixel_bytes).ok_or_else(|| "bmp: file length overflow".to_string())?;
    let file_size = u32::try_from(file_len).map_err(|_| "bmp: file exceeds v3 file-size field")?;
    let image_size = u32::try_from(pixel_bytes).map_err(|_| "bmp: pixel storage exceeds v3 image-size field")?;
    let mut bytes = vec![0; file_len];
    bytes[..2].copy_from_slice(&BMP_MAGIC);
    bytes[2..6].copy_from_slice(&file_size.to_le_bytes());
    bytes[10..14].copy_from_slice(&54u32.to_le_bytes());
    bytes[14..18].copy_from_slice(&BITMAPINFOHEADER_SIZE.to_le_bytes());
    bytes[18..22].copy_from_slice(&(width as i32).to_le_bytes());
    bytes[22..26].copy_from_slice(&(height as i32).to_le_bytes());
    bytes[26..28].copy_from_slice(&1u16.to_le_bytes());
    bytes[28..30].copy_from_slice(&24u16.to_le_bytes());
    bytes[30..34].copy_from_slice(&BI_RGB.to_le_bytes());
    bytes[34..38].copy_from_slice(&image_size.to_le_bytes());
    bytes[38..42].copy_from_slice(&x_pixels_per_meter.to_le_bytes());
    bytes[42..46].copy_from_slice(&y_pixels_per_meter.to_le_bytes());
    let width = width as usize;
    for y in 0..height as usize {
        let source_row = y * width * 4;
        let destination_row = 54 + (height as usize - 1 - y) * row_stride;
        for x in 0..width {
            let source = source_row + x * 4;
            let destination = destination_row + x * 3;
            bytes[destination..destination + 3].copy_from_slice(&[rgba8[source + 2], rgba8[source + 1], rgba8[source]]);
        }
    }
    decode_bmp(&bytes)
}

pub fn encode_bmp(snapshot: &BmpSnapshot) -> Result<Vec<u8>, String> {
    bmp_layout(snapshot)?;
    Ok(snapshot.bytes.clone())
}

pub fn bmp_palette(snapshot: &BmpSnapshot) -> Result<Vec<BmpPaletteEntry>, String> {
    let layout = bmp_layout(snapshot)?;
    (0..layout.palette_entries)
        .map(|index| {
            let offset = layout.palette_offset + index * 4;
            let entry = range(&snapshot.bytes, offset, 4, "palette entry")?;
            Ok(BmpPaletteEntry { b: entry[0], g: entry[1], r: entry[2], reserved: entry[3] })
        })
        .collect()
}

fn mask_shift_width(mask: u32) -> (u32, u32) {
    if mask == 0 {
        return (0, 0);
    }
    (mask.trailing_zeros(), (mask >> mask.trailing_zeros()).trailing_ones())
}

fn extract_channel(raw: u32, mask: u32) -> u8 {
    let (shift, width) = mask_shift_width(mask);
    if width == 0 {
        return 255;
    }
    let value = (raw & mask) >> shift;
    let maximum = if width == 32 { u32::MAX } else { (1u32 << width) - 1 };
    ((u64::from(value) * 255 + u64::from(maximum) / 2) / u64::from(maximum)) as u8
}

fn packed_index(row: &[u8], x: usize, bits_per_pixel: u16) -> usize {
    match bits_per_pixel {
        1 => ((row[x / 8] >> (7 - x % 8)) & 1) as usize,
        4 => {
            if x.is_multiple_of(2) {
                (row[x / 2] >> 4) as usize
            } else {
                (row[x / 2] & 15) as usize
            }
        }
        8 => row[x] as usize,
        _ => unreachable!("checked indexed profile"),
    }
}

fn source_row(layout: &BmpLayout, y: usize) -> usize {
    match layout.row_order {
        BmpRowOrder::TopDown => y,
        BmpRowOrder::BottomUp => layout.height as usize - 1 - y,
    }
}

pub fn bmp_rgba8_preview(snapshot: &BmpSnapshot) -> Result<Vec<u8>, String> {
    let layout = bmp_layout(snapshot)?;
    let palette = bmp_palette(snapshot)?;
    let pixel_count = (layout.width as usize).checked_mul(layout.height as usize).ok_or_else(|| "bmp: preview pixel count overflow".to_string())?;
    let mut rgba = vec![0; pixel_count.checked_mul(4).ok_or_else(|| "bmp: preview byte count overflow".to_string())?];
    for y in 0..layout.height as usize {
        let row_offset = layout.data_offset + source_row(&layout, y) * layout.row_stride;
        let row = range(&snapshot.bytes, row_offset, layout.row_payload, "pixel row")?;
        for x in 0..layout.width as usize {
            let output = (y * layout.width as usize + x) * 4;
            let color = if layout.profile.is_indexed() {
                let index = packed_index(row, x, layout.bits_per_pixel);
                let entry = palette.get(index).ok_or_else(|| format!("bmp: pixel ({x},{y}) references absent palette index {index}"))?;
                BmpColor { red: entry.r, green: entry.g, blue: entry.b, alpha: 255 }
            } else {
                match layout.profile {
                    BmpProfile::DirectRgb24 => {
                        let offset = x * 3;
                        BmpColor { red: row[offset + 2], green: row[offset + 1], blue: row[offset], alpha: 255 }
                    }
                    BmpProfile::DirectRgb32 => {
                        let offset = x * 4;
                        BmpColor { red: row[offset + 2], green: row[offset + 1], blue: row[offset], alpha: 255 }
                    }
                    _ => {
                        let bytes_per_sample = layout.bits_per_pixel as usize / 8;
                        let offset = x * bytes_per_sample;
                        let raw = if bytes_per_sample == 2 { u32::from(u16::from_le_bytes([row[offset], row[offset + 1]])) } else { u32::from_le_bytes([row[offset], row[offset + 1], row[offset + 2], row[offset + 3]]) };
                        BmpColor {
                            red: extract_channel(raw, layout.masks[0]),
                            green: extract_channel(raw, layout.masks[1]),
                            blue: extract_channel(raw, layout.masks[2]),
                            alpha: if layout.masks[3] == 0 { 255 } else { extract_channel(raw, layout.masks[3]) },
                        }
                    }
                }
            };
            rgba[output..output + 4].copy_from_slice(&[color.red, color.green, color.blue, color.alpha]);
        }
    }
    Ok(rgba)
}

pub fn bmp_png_preview(snapshot: &BmpSnapshot) -> Result<BmpPngPreview, String> {
    const MAX_RGBA_BYTES: usize = 64 * 1024 * 1024;
    let layout = bmp_layout(snapshot)?;
    let rgba_bytes = usize::try_from(layout.width)
        .map_err(|_| "bmp: preview width exceeds address space")?
        .checked_mul(usize::try_from(layout.height).map_err(|_| "bmp: preview height exceeds address space")?)
        .and_then(|pixels| pixels.checked_mul(4))
        .ok_or_else(|| "bmp: preview byte count overflow".to_string())?;
    if rgba_bytes > MAX_RGBA_BYTES {
        return Err(format!("bmp: preview needs {rgba_bytes} RGBA bytes, above the {MAX_RGBA_BYTES}-byte display limit"));
    }
    let pixels = bmp_rgba8_preview(snapshot)?;
    let bytes = semio_framework_pixels::encode_png(&semio_framework_pixels::RasterImage { width: layout.width, height: layout.height, pixels }).map_err(|failure| failure.to_string())?;
    Ok(BmpPngPreview { width: layout.width, height: layout.height, bytes })
}

pub fn bmp_revision(snapshot: &BmpSnapshot) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in snapshot.schema.as_bytes().iter().chain(snapshot.bytes.iter()) {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn checked_region(layout: &BmpLayout, region: BmpRegion) -> Result<(), String> {
    if region.width == 0 || region.height == 0 {
        return Err("bmp: paint region must be nonempty".into());
    }
    let end_x = region.x.checked_add(region.width).ok_or_else(|| "bmp: region x overflow".to_string())?;
    let end_y = region.y.checked_add(region.height).ok_or_else(|| "bmp: region y overflow".to_string())?;
    if end_x > layout.width || end_y > layout.height {
        return Err(format!("bmp: paint region {region:?} exceeds {}x{} image", layout.width, layout.height));
    }
    Ok(())
}

fn require_revision(snapshot: &BmpSnapshot, revision: &str) -> Result<(), String> {
    let actual = bmp_revision(snapshot);
    if revision != actual {
        return Err(format!("bmp: stale bitmap revision {revision}; expected {actual}"));
    }
    Ok(())
}

pub fn paint_indexed_region_controlled(snapshot: &BmpSnapshot, revision: &str, region: BmpRegion, palette_index: u8, progress: &mut dyn FnMut(usize, usize) -> bool) -> Result<BmpSnapshot, String> {
    require_revision(snapshot, revision)?;
    let layout = bmp_layout(snapshot)?;
    if !layout.profile.is_indexed() {
        return Err("bmp: indexed paint requires a 1-, 4-, or 8-bit indexed profile".into());
    }
    if usize::from(palette_index) >= layout.palette_entries || usize::from(palette_index) >= (1usize << layout.bits_per_pixel) {
        return Err(format!("bmp: palette index {palette_index} is outside the checked palette"));
    }
    checked_region(&layout, region)?;
    let total = region.height as usize;
    let mut next = snapshot.clone();
    for local_y in 0..region.height as usize {
        if !progress(local_y, total) {
            return Err("bmp: indexed paint cancelled".into());
        }
        let y = region.y as usize + local_y;
        let row_offset = layout.data_offset + source_row(&layout, y) * layout.row_stride;
        for x in region.x as usize..(region.x + region.width) as usize {
            match layout.bits_per_pixel {
                1 => {
                    let byte = &mut next.bytes[row_offset + x / 8];
                    let mask = 1 << (7 - x % 8);
                    *byte = (*byte & !mask) | ((palette_index & 1) << (7 - x % 8));
                }
                4 => {
                    let byte = &mut next.bytes[row_offset + x / 2];
                    if x.is_multiple_of(2) {
                        *byte = (*byte & 0x0f) | (palette_index << 4);
                    } else {
                        *byte = (*byte & 0xf0) | (palette_index & 0x0f);
                    }
                }
                8 => next.bytes[row_offset + x] = palette_index,
                _ => unreachable!("checked indexed profile"),
            }
        }
    }
    if !progress(total, total) {
        return Err("bmp: indexed paint cancelled".into());
    }
    Ok(next)
}

fn pack_channel(value: u8, mask: u32) -> u32 {
    let (shift, width) = mask_shift_width(mask);
    if width == 0 {
        return 0;
    }
    let maximum = if width == 32 { u32::MAX } else { (1u32 << width) - 1 };
    ((((u64::from(value) * u64::from(maximum)) + 127) / 255) as u32) << shift
}

pub fn paint_direct_region_controlled(snapshot: &BmpSnapshot, revision: &str, region: BmpRegion, color: BmpColor, progress: &mut dyn FnMut(usize, usize) -> bool) -> Result<BmpSnapshot, String> {
    require_revision(snapshot, revision)?;
    let layout = bmp_layout(snapshot)?;
    if !layout.profile.is_direct() {
        return Err("bmp: direct paint requires a direct-color profile".into());
    }
    checked_region(&layout, region)?;
    let total = region.height as usize;
    let mut next = snapshot.clone();
    for local_y in 0..region.height as usize {
        if !progress(local_y, total) {
            return Err("bmp: direct paint cancelled".into());
        }
        let y = region.y as usize + local_y;
        let row_offset = layout.data_offset + source_row(&layout, y) * layout.row_stride;
        for x in region.x as usize..(region.x + region.width) as usize {
            match layout.profile {
                BmpProfile::DirectRgb24 => {
                    let offset = row_offset + x * 3;
                    next.bytes[offset..offset + 3].copy_from_slice(&[color.blue, color.green, color.red]);
                }
                BmpProfile::DirectRgb32 => {
                    let offset = row_offset + x * 4;
                    next.bytes[offset..offset + 3].copy_from_slice(&[color.blue, color.green, color.red]);
                }
                _ => {
                    let bytes_per_sample = layout.bits_per_pixel as usize / 8;
                    let offset = row_offset + x * bytes_per_sample;
                    let mut raw = if bytes_per_sample == 2 { u32::from(u16::from_le_bytes([next.bytes[offset], next.bytes[offset + 1]])) } else { u32::from_le_bytes(next.bytes[offset..offset + 4].try_into().expect("checked sample")) };
                    let edited_masks = layout.masks[0] | layout.masks[1] | layout.masks[2] | layout.masks[3];
                    raw = (raw & !edited_masks) | pack_channel(color.red, layout.masks[0]) | pack_channel(color.green, layout.masks[1]) | pack_channel(color.blue, layout.masks[2]) | pack_channel(color.alpha, layout.masks[3]);
                    if bytes_per_sample == 2 {
                        next.bytes[offset..offset + 2].copy_from_slice(&(raw as u16).to_le_bytes());
                    } else {
                        next.bytes[offset..offset + 4].copy_from_slice(&raw.to_le_bytes());
                    }
                }
            }
        }
    }
    if !progress(total, total) {
        return Err("bmp: direct paint cancelled".into());
    }
    Ok(next)
}

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {
    crate::io_registry::register();
    register_artifact_schema();
    register_artifact_inferences();
    register_pilot_languages();
    register_schema_specs();
    semio_framework_plugin::io::register_native_snapshot_codec(
        semio_framework_plugin::Dialect { artifact_kind: "s.stdio.bmp", standard: semio_framework_plugin::StandardId("v3"), subset: semio_framework_plugin::SubsetId("*") },
        store::ArtifactCodec::of::<BmpSnapshot, BmpMutation>(STDIO_BMP_DOCUMENT_SCHEMA),
    )
    .expect("static Stdio registration must be available and conflict-free");
}

/// 📇️ P2-FG2: `dsl::registry::register_schema_spec` (P2-M3's `FullResolver` insertion API) —
/// real, non-fabricated calls (unlike json/csv/zip/png's hand-rolled types, `BmpSnapshot`/
/// `BmpDiff` DO carry genuine derived `RecordSpec` constructors:
/// `#[derive(semio_framework_dsl_record_derive::DslRecord)]`/`#[derive(dsl::DslDiff)]` emit `__dsl_spec`/`__dsl_diff_spec`
/// respectively, see ../🪆️subsets/✳️any/🧬️schema/📸️snapshot and 🔺️diff's own doc comments).
/// Covers both the document's own schema id and its `"<doc>#diff"` diff schema id, per design
/// ruling B-R4, `stdio.txt`'s own exemplar pattern. `#[cfg]`-gated to match
/// `os_dsl::registry`'s own `#[cfg(not(target_arch = "wasm32"))]` — the registry simply does not
/// exist as a compiled item on `wasm32`. `BmpMutation`'s own mutations facet is skipped
/// (`dsl::DslOps` gives per-variant specs via `DslVariants`, no single canonical id to register
/// under — `register-schema-spec-one-spec-per-artifact`, this ticket's own recipe §5).
#[cfg(not(target_arch = "wasm32"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_schema_specs() {
    ::semio_framework_async::poll::resolve_ready(dsl::registry::register_schema_spec("stdio.bmp", BmpSnapshot::__dsl_spec));
    ::semio_framework_async::poll::resolve_ready(dsl::registry::register_schema_spec("stdio.bmp#diff", crate::schema::diff::BmpDiff::__dsl_spec));
}

#[cfg(target_arch = "wasm32")]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_schema_specs() {}

/// 📌️ Registers the full 5-role `LanguageSpec` set (Document/Ops/Diff/Pack/Spr — this ticket's
/// own recipe §4 checklist item, json's own exemplar shape) for handcrafted facet grammars
/// (text) and protocols (binary) — was a single Document-only registration before this wave.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_pilot_languages() {
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.bmp",
        extension: Some("bmp"),
        role: semio_framework_dsl::LanguageRole::Document,
        grammar: Some(crate::standards::v_v3::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::standards::v_v3::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::standards::v_v3::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::standards::v_v3::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.bmp"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.bmp.op",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Ops,
        grammar: Some(crate::standards::v_v3::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::standards::v_v3::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::standards::v_v3::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::standards::v_v3::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.bmp.op"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.bmp.diff",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Diff,
        grammar: Some(crate::standards::v_v3::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::standards::v_v3::subsets::any::io::text::diff::COMPONENT_GRAMMAR_PATH),
        // 🎫️ The 5-role scheme has no dedicated "diff binary" role even when a real diff
        // protocol file exists (this ticket's own recipe §4 checklist item) — `BmpDiff`'s own
        // `.spk`-container protocol IS real (see ../🪆️subsets/✳️any/🚪️io/💾️binary/🔺️diff/
        // 📡️.protocol.semio), just not registered here.
        protocol: None,
        protocol_path: None,
        hooks: semio_framework_dsl::passthrough_hooks("stdio.bmp.diff"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.bmp.pack",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::standards::v_v3::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::standards::v_v3::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.bmp.pack"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.bmp.spr",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::standards::v_v3::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::standards::v_v3::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.bmp.spr"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.bmp`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_artifact_schema() {
    ::semio_framework_schema_registry::register_artifact_schema_descriptor(crate::schema::bmp_artifact_schema_descriptor()).expect("schema descriptor publication");
}

/// 💡️ Registers `s.stdio.bmp.inference`'s facet leaves into the OS-wide inference catalog —
/// sibling to `register_artifact_schema()` above (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_artifact_inferences() {
    ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::standards::v_v3::subsets::any::schema::inferences::bmp_artifact_inference_descriptor()).expect("schema descriptor publication");
}
//#endregion 🔖️Register

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::v_v3::subsets::any::io::BmpComposer as BmpRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<BmpRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::{BmpDiff, BmpMutation, BmpSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.bmp` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct BmpBuilderConstruction {
        snapshot: BmpSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for BmpBuilderConstruction {
        type Snapshot = BmpSnapshot;
        type Mutation = BmpMutation;
        type Diff = BmpDiff;
        fn empty() -> Self {
            Self { snapshot: BmpSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<BmpSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<BmpSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::schema::mutations::apply_bmp_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <BmpDiff as protocol::MutationDiff<BmpSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::BmpSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.bmp` parts.
    #[derive(Clone, Debug, Default)]
    pub struct BmpParts {
        pub snapshot: Option<BmpSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.bmp` (v3/✳️any) sources.
    pub struct BmpAnalyzerAnalysis;

    impl ArtifactAnalysis for BmpAnalyzerAnalysis {
        type Parts = BmpParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.bmp", standard: StandardId("v3"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            const SIG: [u8; 2] = *b"BM";
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if bytes.len() >= 2 && bytes[0..2] == SIG {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    // 🔍 stdio.bmp's text envelope is a hex dump of the raw bytes after the
                    // `semio ...` preamble line — decode the first 2 bytes to sniff the real signature.
                    let body = match store::semio_format::split_text_preamble(text) {
                        Ok((_, rest)) => rest,
                        Err(_) => text,
                    };
                    let hex: String = body.chars().filter(|c| !c.is_whitespace()).take(4).collect();
                    if hex.len() < 4 {
                        return IoConfidence::Low;
                    }
                    let mut decoded = [0u8; 2];
                    for (i, byte) in decoded.iter_mut().enumerate() {
                        match u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16) {
                            Ok(b) => *byte = b,
                            Err(_) => return IoConfidence::Low,
                        }
                    }
                    if decoded == SIG {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = BmpParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <BmpSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <BmpSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec BmpBuilderFacets {
        construction: BmpBuilderConstruction,
        analysis: BmpAnalyzerAnalysis,
        composition: crate::standards::v_v3::subsets::any::io::derived_composition::BmpComposerComposition,
    }
    builder: BmpBuilder,
    analyzer: BmpAnalyzer,
    composer: BmpComposer,
);
