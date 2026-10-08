//! 🚪️ IO stdio.bmp (v3/✳️any) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_v3::subsets::any::io::BmpAnalyzer;
    use crate::BmpSnapshot;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

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

/// 🧭 Native BMP v3 physical layout, admission and canonical emission.
use crate::schema::snapshot::{BmpImage, BmpNativeSample, BmpPaletteEntry, BmpPixels, BmpProfile, BmpRowOrder};
use crate::{BmpMutation, BmpSnapshot, STDIO_BMP_DOCUMENT_SCHEMA};
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,ValueError,ValueRefusalKind};
fn native_invalid(message:impl Into<String>)->ValueError {ValueError::new(ValueRefusalKind::InvalidValue,message)}

const BMP_MAGIC: [u8; 2] = *b"BM";
const BITMAPINFOHEADER_SIZE: u32 = 40;
const BI_RGB: u32 = 0;
const BI_BITFIELDS: u32 = 3;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BmpPngPreview { pub width: u32, pub height: u32, pub bytes: Vec<u8> }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BmpLayout {
    pub profile: BmpProfile, pub file_size: u32, pub reserved_1: u16, pub reserved_2: u16, pub data_offset: usize,
    pub width: u32, pub height: u32, pub row_order: BmpRowOrder, pub planes: u16, pub bits_per_pixel: u16, pub compression: u32,
    pub image_size: u32, pub x_pixels_per_meter: i32, pub y_pixels_per_meter: i32, pub colors_used: u32, pub colors_important: u32,
    pub masks: [u32; 4], pub palette_offset: usize, pub palette_entries: usize, pub metadata_end: usize, pub row_stride: usize,
    pub row_payload: usize, pub pixel_bytes: usize, pub pixel_end: usize,
}

#[path = "🧩️layout/🦀️.rs"]
pub(crate) mod layout;

fn range<'a>(bytes: &'a [u8], offset: usize, len: usize, name: &str) -> Result<&'a [u8], String> { bytes.get(offset..offset.checked_add(len).ok_or("bmp: native range overflow")?).ok_or_else(|| format!("bmp: truncated {name}")) }
fn checked_row_geometry(width: u32, bits_per_pixel: u16) -> Result<(usize, usize), String> { let bits = (width as usize).checked_mul(bits_per_pixel as usize).ok_or("bmp: native row extent overflow")?; Ok((bits.div_ceil(32).checked_mul(4).ok_or("bmp: native row stride overflow")?, bits.div_ceil(8))) }
pub(crate) fn row_bytes(width: u32, bits_per_pixel: u16) -> usize { checked_row_geometry(width, bits_per_pixel).expect("admitted native row").0 }
pub fn bmp_layout_bytes(bytes: &[u8]) -> Result<BmpLayout, String> { layout::inspect(bytes).map_err(|failure| failure.to_string()) }
pub fn bmp_layout(snapshot: &BmpSnapshot) -> Result<BmpLayout, String> { bmp_layout_bytes(&encode_bmp(snapshot)?) }
fn source_row(layout: &BmpLayout, y: usize) -> usize { match layout.row_order { BmpRowOrder::TopDown => y, BmpRowOrder::BottomUp => layout.height as usize - 1 - y } }
fn packed_index(row: &[u8], x: usize, depth: u16) -> u8 { match depth { 1 => row[x / 8] >> (7 - x % 8) & 1, 4 => row[x / 2] >> (if x % 2 == 0 { 4 } else { 0 }) & 15, _ => row[x] } }
fn component(word: u32, mask: u32) -> u32 { if mask == 0 { 0 } else { (word & mask) >> mask.trailing_zeros() } }

pub fn decode_bmp(bytes:&[u8])->Result<BmpSnapshot,String> {let mut progress=|_|true;let mut control=NativeDecodeControl::new(512*1024*1024,&mut progress);decode_bmp_controlled(bytes,&mut control).map_err(ValueError::into_message)}

pub fn decode_bmp_controlled(bytes:&[u8],control:&mut NativeDecodeControl<'_>)->Result<BmpSnapshot,ValueError> {
    let layout = bmp_layout_bytes(bytes).map_err(native_invalid)?;
    let mut palette=control.allocate_vec(layout.palette_entries)?;control.begin_stage(layout.palette_entries)?;
    for index in 0..layout.palette_entries {let offset=layout.palette_offset+index*4;let entry=&bytes[offset..offset+4];palette.push(BmpPaletteEntry {b:entry[0],g:entry[1],r:entry[2],reserved:entry[3]});control.step()?;}
    let masks = if layout.profile == BmpProfile::DirectRgb24 { [0xff0000, 0xff00, 0xff, 0] } else { layout.masks };
    let count = (layout.width as usize).checked_mul(layout.height as usize).ok_or_else(||native_invalid("bmp: native sample count overflow"))?;
    let pixels = if layout.profile.is_indexed() {
        let mut indices=control.allocate_vec(count)?;control.begin_stage(count)?;
        for y in 0..layout.height as usize { let row = range(bytes, layout.data_offset + source_row(&layout, y) * layout.row_stride, layout.row_payload, "indexed row").map_err(native_invalid)?; for x in 0..layout.width as usize { let index=packed_index(row,x,layout.bits_per_pixel);if usize::from(index)>=palette.len() {return Err(native_invalid("bmp: native palette index is absent"));}indices.push(index);control.step()?; } }
        BmpPixels::Indexed { indices }
    } else {
        let mut samples=control.allocate_vec(count)?;control.begin_stage(count)?;
        let assigned = masks.iter().fold(0, |assigned, mask| assigned | mask);
        let size = layout.bits_per_pixel as usize / 8;
        for y in 0..layout.height as usize { let row = range(bytes, layout.data_offset + source_row(&layout, y) * layout.row_stride, layout.row_payload, "direct row").map_err(native_invalid)?; for x in 0..layout.width as usize {
            let lane = &row[x * size..(x + 1) * size];
            let mut word = 0u32; for (index, byte) in lane.iter().enumerate() { word |= u32::from(*byte) << (index * 8); }
            samples.push(BmpNativeSample { red: component(word, masks[0]), green: component(word, masks[1]), blue: component(word, masks[2]), alpha: component(word, masks[3]), reserved: word & !assigned });control.step()?;
        } }
        BmpPixels::Direct { samples }
    };
    let snapshot = BmpSnapshot { schema:control.copy_text(STDIO_BMP_DOCUMENT_SCHEMA)?, image: BmpImage { width: layout.width, height: layout.height, row_order: layout.row_order, profile: layout.profile, masks, palette, pixels, x_pixels_per_meter: layout.x_pixels_per_meter, y_pixels_per_meter: layout.y_pixels_per_meter, colors_used: layout.colors_used, colors_important: layout.colors_important, reserved_1: layout.reserved_1, reserved_2: layout.reserved_2, opaque_gap: control.copy_bytes(&bytes[layout.metadata_end..layout.data_offset])?, opaque_trailer: control.copy_bytes(&bytes[layout.pixel_end..])? } };
    snapshot.image.validate_header().map_err(native_invalid)?;
    Ok(snapshot)
}

pub fn encode_bmp(snapshot:&BmpSnapshot)->Result<Vec<u8>,String> {let mut progress=|_|true;let mut control=NativeEncodeControl::new(512*1024*1024,&mut progress);encode_bmp_controlled(snapshot,&mut control).map_err(ValueError::into_message)}

pub fn encode_bmp_controlled(snapshot:&BmpSnapshot,control:&mut NativeEncodeControl<'_>)->Result<Vec<u8>,ValueError> {
    if snapshot.schema!=STDIO_BMP_DOCUMENT_SCHEMA {return Err(native_invalid("bmp: undeclared semantic schema"));}snapshot.image.validate_header().map_err(native_invalid)?;
    let image = &snapshot.image;
    let depth = image.profile.bits_per_pixel();
    let (stride, _) = checked_row_geometry(image.width,depth).map_err(native_invalid)?;
    let mask_bytes = if image.profile.bitfields() { 12 } else { 0 };
    let offset = 54usize.checked_add(mask_bytes).and_then(|n| n.checked_add(image.palette.len().checked_mul(4)?)).and_then(|n| n.checked_add(image.opaque_gap.len())).ok_or_else(||native_invalid("bmp: metadata extent overflow"))?;
    let pixels_extent = stride.checked_mul(image.height as usize).ok_or_else(||native_invalid("bmp: native raster extent overflow"))?;
    let total = offset.checked_add(pixels_extent).and_then(|n| n.checked_add(image.opaque_trailer.len())).ok_or_else(||native_invalid("bmp: native file extent overflow"))?;
    let mut bytes=control.allocate_vec(total)?;control.begin_stage(total)?;while bytes.len()<total {let added=(total-bytes.len()).min(256);bytes.resize(bytes.len()+added,0);control.advance(added)?;}
    bytes[..2].copy_from_slice(&BMP_MAGIC);
    bytes[2..6].copy_from_slice(&u32::try_from(total).map_err(|_|native_invalid("bmp: native file exceeds v3 limits"))?.to_le_bytes());
    bytes[6..8].copy_from_slice(&image.reserved_1.to_le_bytes()); bytes[8..10].copy_from_slice(&image.reserved_2.to_le_bytes());
    bytes[10..14].copy_from_slice(&u32::try_from(offset).map_err(|_|native_invalid("bmp: native offset exceeds v3 limits"))?.to_le_bytes());
    bytes[14..18].copy_from_slice(&BITMAPINFOHEADER_SIZE.to_le_bytes()); bytes[18..22].copy_from_slice(&(image.width as i32).to_le_bytes());
    let height = if image.row_order == BmpRowOrder::TopDown { -(image.height as i32) } else { image.height as i32 };
    bytes[22..26].copy_from_slice(&height.to_le_bytes()); bytes[26..28].copy_from_slice(&1u16.to_le_bytes()); bytes[28..30].copy_from_slice(&depth.to_le_bytes());
    bytes[30..34].copy_from_slice(&(if image.profile.bitfields() { BI_BITFIELDS } else { BI_RGB }).to_le_bytes());
    bytes[34..38].copy_from_slice(&u32::try_from(pixels_extent).map_err(|_|native_invalid("bmp: native raster exceeds v3 limits"))?.to_le_bytes());
    bytes[38..42].copy_from_slice(&image.x_pixels_per_meter.to_le_bytes()); bytes[42..46].copy_from_slice(&image.y_pixels_per_meter.to_le_bytes());
    bytes[46..50].copy_from_slice(&image.colors_used.to_le_bytes()); bytes[50..54].copy_from_slice(&image.colors_important.to_le_bytes());
    let mut cursor = 54;
    if image.profile.bitfields() { for mask in &image.masks[..3] { bytes[cursor..cursor + 4].copy_from_slice(&mask.to_le_bytes()); cursor += 4; } }
    control.begin_stage(image.palette.len())?;
    for entry in &image.palette { bytes[cursor..cursor + 4].copy_from_slice(&[entry.b, entry.g, entry.r, entry.reserved]); cursor += 4;control.step()?; }
    control.begin_stage(image.opaque_gap.len())?;for (index,piece) in image.opaque_gap.chunks(256).enumerate() {let start=cursor+index*256;bytes[start..start+piece.len()].copy_from_slice(piece);control.advance(piece.len())?;}
    control.begin_stage(image.width as usize*image.height as usize)?;
    for y in 0..image.height as usize {
        let source = y * image.width as usize;
        let destination = offset + (if image.row_order == BmpRowOrder::TopDown { y } else { image.height as usize - 1 - y }) * stride;
        for x in 0..image.width as usize {image.validate_sample(source+x).map_err(native_invalid)?;match &image.pixels {
            BmpPixels::Indexed { indices } => { let index = indices[source + x]; match depth { 1 => bytes[destination + x / 8] |= index << (7 - x % 8), 4 => bytes[destination + x / 2] |= index << (if x % 2 == 0 { 4 } else { 0 }), _ => bytes[destination + x] = index } }
            BmpPixels::Direct { samples } => { let sample = samples[source + x]; let mut word = sample.reserved; for (value, mask) in [sample.red, sample.green, sample.blue, sample.alpha].into_iter().zip(image.masks) { if mask != 0 { word |= value << mask.trailing_zeros(); } } let size = depth as usize / 8; bytes[destination + x * size..destination + (x + 1) * size].copy_from_slice(&word.to_le_bytes()[..size]); }
        }control.step()?; }
    }
    control.begin_stage(image.opaque_trailer.len())?;for (index,piece) in image.opaque_trailer.chunks(256).enumerate() {let start=offset+pixels_extent+index*256;bytes[start..start+piece.len()].copy_from_slice(piece);control.advance(piece.len())?;}
    Ok(bytes)
}

pub fn empty_bmp_bytes() -> Vec<u8> { encode_bmp(&BmpSnapshot::default()).expect("owned default BMP") }
pub fn demo_bmp_bytes() -> Vec<u8> { encode_bmp(&crate::schema::demo_bmp_snapshot()).expect("owned demo BMP") }
pub fn bmp_direct_rgb24_from_rgba8(width: u32, height: u32, rgba8: &[u8], x_pixels_per_meter: i32, y_pixels_per_meter: i32) -> Result<BmpSnapshot, String> {
    let count = (width as usize).checked_mul(height as usize).and_then(|n| n.checked_mul(4)).ok_or("bmp: RGBA source extent overflow")?;
    if count != rgba8.len() {return Err(format!("bmp: RGBA source expected {count} bytes"));}if rgba8.chunks_exact(4).any(|sample|sample[3]!=255) {return Err("bmp: RGB24 cannot represent nonopaque samples".into());}
    let image = BmpImage { width, height, x_pixels_per_meter, y_pixels_per_meter, pixels: BmpPixels::Direct { samples: rgba8.chunks_exact(4).map(|sample| BmpNativeSample { red: sample[0].into(), green: sample[1].into(), blue: sample[2].into(), alpha: 0, reserved: 0 }).collect() }, ..BmpImage::default() };
    let snapshot = BmpSnapshot { schema: STDIO_BMP_DOCUMENT_SCHEMA.into(), image }; snapshot.validate()?; Ok(snapshot)
}
pub fn bmp_png_preview(snapshot: &BmpSnapshot) -> Result<BmpPngPreview, String> {
    let pixels = crate::schema::operations::bmp_rgba8_preview(snapshot)?;
    let width = snapshot.image.width; let height = snapshot.image.height;
    if pixels.len() > 64 * 1024 * 1024 { return Err("bmp: preview exceeds display ownership limit".into()); }
    let bytes = semio_framework_pixels::encode_png(&semio_framework_pixels::RasterImage { width, height, pixels }).map_err(|failure| failure.to_string())?;
    Ok(BmpPngPreview { width, height, bytes })
}

pub fn register() {
    crate::io_registry::register();
    register_artifact_schema();
    register_artifact_inferences();
    register_pilot_languages();
    register_schema_specs();
    semio_framework_plugin::io::register_native_snapshot_codec(
        semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.bmp", standard: semio_framework_artifact_reference::StandardId("v3"), subset: semio_framework_artifact_reference::SubsetId("*") },
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
    use semio_framework_plugin::{composer_entry_of, io::ComposerEntry};
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
            self.snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
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
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

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

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            const SIG: [u8; 2] = *b"BM";
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if bytes.len() >= 2 && bytes[0..2] == SIG {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
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
                        return semio_framework_plugin::io::Confidence::Low;
                    }
                    let mut decoded = [0u8; 2];
                    for (i, byte) in decoded.iter_mut().enumerate() {
                        match u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16) {
                            Ok(b) => *byte = b,
                            Err(_) => return semio_framework_plugin::io::Confidence::Low,
                        }
                    }
                    if decoded == SIG {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = BmpParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <BmpSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <BmpSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
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

#[cfg(test)]
#[path="🧪️tests/🧬️owned-native-oracle/🦀️.rs"]
mod owned_native_oracle_tests;
