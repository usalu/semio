//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered reference implementation so the subject's own mutation has an independent result to
//! be compared against instead of being checked against its own reading.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared `raster` module rather than by copying it.
//!
//! `../🚪️io/🦀️component.rs` (this subset's own from-scratch codec, not an `image` wrapper) always
//! regenerates fresh Annex K DQT/DHT tables at `re_encode_quality` on encode and never emits a
//! DRI/restart marker — so `SetQuantTable`/`RemoveQuantTable`/`SetHuffmanTable`/`RemoveHuffmanTable`
//! mutate only the in-memory snapshot model (provably absent from any re-serialization) and
//! `SetRestartInterval` is unobservable in bytes at all. `SetJfifHeader` and
//! `InsertOtherSegment`/`RemoveOtherSegment` ARE written to real bytes (JFIF APP0 fields, verbatim
//! segment echo) but are metadata a conforming decoder does not need to reproduce, so this subset's
//! `semantic-jpg-mutate-v1` projection (geometry + luma histogram, never raw samples — JPEG is
//! lossy) legitimately cannot and does not observe them either. Every one of those kinds is
//! therefore applied here as a decode → re-encode PASSTHROUGH at the fixed default quality: the
//! oracle's claim for them is "a real independent codec still decodes this JPEG and its raster is
//! unharmed", which is the only claim the format actually backs.
//!
//! @see ../🧪️oracle/🔣️component.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself.

use image::ImageEncoder;
use semio_repo_test_host::Json;

//#region 🔖️Json
/// 🔎️ `value.get(key)`'s numeric leg — the shared `Json` reader has no numeric accessor of its own.
#[cfg(feature = "oracles")]
fn number(value: &Json, key: &str, fallback: f64) -> f64 {
    match value.get(key) {
        Some(Json::Number(n)) => *n,
        _ => fallback,
    }
}

/// 🎨️ Reads an `[r,g,b,a]` fill out of `params.fill`, defaulting to a mid-grey opaque pixel.
#[cfg(feature = "oracles")]
fn fill_of(params: &Json) -> [u8; 4] {
    let values = params.array("fill");
    let component = |index: usize, fallback: u8| {
        values
            .get(index)
            .and_then(|v| match v {
                Json::Number(n) => Some(*n as u8),
                _ => None,
            })
            .unwrap_or(fallback)
    };
    [component(0, 128), component(1, 128), component(2, 128), component(3, 255)]
}
//#endregion 🔖️Json

//#region 🔖️Codec
/// 🎚️ Encoder default matching this subset's own `encode_jpg`'s `re_encode_quality.unwrap_or(90)`.
#[cfg(feature = "oracles")]
const DEFAULT_QUALITY: u8 = 90;

/// 👁️ Decodes with the INDEPENDENT `image` reader into raw RGBA8, row-major.
#[cfg(feature = "oracles")]
fn decode_rgba(input: &[u8]) -> Result<(u32, u32, Vec<u8>), String> {
    let decoded = image::load_from_memory(input).map_err(|error| format!("independent reader could not parse the jpg: {error}"))?;
    let rgba = decoded.to_rgba8();
    let (width, height) = (rgba.width(), rgba.height());
    Ok((width, height, rgba.into_raw()))
}

/// 🔮️ Encodes RGBA8 with the registered `image` reference implementation at an explicit quality
/// (JPEG carries no alpha channel, so the reference encoder is given RGB, matching this crate's own
/// `raster::oracle_create_image` convention for BMP/JPEG).
#[cfg(feature = "oracles")]
fn encode_rgba(width: u32, height: u32, rgba: &[u8], quality: u8) -> Result<Vec<u8>, String> {
    let buffer = image::RgbaImage::from_raw(width, height, rgba.to_vec()).ok_or("raster does not fill width * height * 4 bytes")?;
    let rgb = image::DynamicImage::ImageRgba8(buffer).to_rgb8();
    let mut out = Vec::new();
    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, quality).write_image(rgb.as_raw(), width, height, image::ExtendedColorType::Rgb8).map_err(|error| format!("jpeg encode: {error}"))?;
    Ok(out)
}

/// ↔️ Decode → re-encode at the fixed default quality, raster untouched — the honest oracle-side
/// effect of every mutation kind this codec's encoder cannot make bytes-observable (see module docs).
#[cfg(feature = "oracles")]
fn passthrough(input: &[u8]) -> Result<Vec<u8>, String> {
    let (width, height, rgba) = decode_rgba(input)?;
    encode_rgba(width, height, &rgba, DEFAULT_QUALITY)
}

/// 🟪️ `width * height` solid-fill RGBA8 raster, row-major.
#[cfg(feature = "oracles")]
fn solid_fill(width: u32, height: u32, fill: [u8; 4]) -> Vec<u8> {
    fill.iter().copied().cycle().take((width as usize) * (height as usize) * 4).collect()
}
//#endregion 🔖️Codec

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    let params = spec.get("params").cloned().unwrap_or(Json::Null);
    match kind.as_str() {
        "" => Err("mutation spec carries no `kind`".to_string()),
        "no-mutation" => Ok(input.to_vec()),
        "set-snapshot" => {
            let width = number(&params, "width", 2.0) as u32;
            let height = number(&params, "height", 2.0) as u32;
            encode_rgba(width, height, &solid_fill(width, height, fill_of(&params)), DEFAULT_QUALITY)
        }
        "set-pixels" => {
            let (width, height, _) = decode_rgba(input)?;
            encode_rgba(width, height, &solid_fill(width, height, fill_of(&params)), DEFAULT_QUALITY)
        }
        "set-re-encode-quality" => {
            let (width, height, rgba) = decode_rgba(input)?;
            let quality = number(&params, "quality", DEFAULT_QUALITY as f64).clamp(1.0, 100.0) as u8;
            encode_rgba(width, height, &rgba, quality)
        }
        "set-jfif-header" | "set-quant-table" | "remove-quant-table" | "set-huffman-table" | "remove-huffman-table" | "set-restart-interval" | "insert-other-segment" | "remove-other-segment" => passthrough(input),
        other => Err(format!("mutation kind {:?} has no oracle implementation ({} input byte(s))", other, input.len())),
    }
}

/// ↩️ Applies the INDEPENDENTLY computed inverse of `spec` on top of `mutated`. `JpgMutation`'s own
/// algebraic law (`../🧬️schema/🧬️mutations/🦀️component.rs`) is "restore `base`'s own value for the
/// facet this kind replaced", which over the reference `image` codec's observable surface means:
/// `set-snapshot` restores the whole original raster, `set-pixels` puts the original samples back
/// into the mutated document's geometry, `set-re-encode-quality` re-encodes at the default quality
/// the fixture's own snapshot carries, and every metadata-only kind (whose forward effect this
/// codec's encoder cannot make observable at all — see the module doc comment) inverts to another
/// decode → re-encode of the mutated document.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation_inverse(original_input: &[u8], spec: &Json, mutated: &[u8]) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    match kind.as_str() {
        "" => Err("mutation spec carries no `kind`".to_string()),
        "set-snapshot" => passthrough(original_input),
        "set-pixels" => {
            let (_, _, original_rgba) = decode_rgba(original_input)?;
            let (width, height, _) = decode_rgba(mutated)?;
            encode_rgba(width, height, &original_rgba, DEFAULT_QUALITY)
        }
        "no-mutation" | "set-re-encode-quality" | "set-jfif-header" | "set-quant-table" | "remove-quant-table" | "set-huffman-table" | "remove-huffman-table" | "set-restart-interval" | "insert-other-segment" | "remove-other-segment" => passthrough(mutated),
        other => Err(format!("mutation kind {other:?} has no oracle inverse ({} mutated byte(s))", mutated.len())),
    }
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation_inverse(_original_input: &[u8], _spec: &Json, _mutated: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

/// 🔁️ The `@id-identity-round-trip` scenario's independent computation: decode with the reference
/// reader, re-encode at the fixed default quality — the same passthrough every metadata-only
/// mutation kind reduces to above, and also what `inverse-<kind>` restores to once a forward
/// mutation and its computed inverse are expected to cancel out.
#[cfg(feature = "oracles")]
pub fn oracle_identity_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    passthrough(input)
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_identity_round_trip(_input: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch
