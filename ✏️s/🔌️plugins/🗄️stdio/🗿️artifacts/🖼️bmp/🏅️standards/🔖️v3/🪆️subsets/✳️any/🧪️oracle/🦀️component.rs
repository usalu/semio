//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered reference implementation so the subject's own mutation has an independent result to
//! be compared against instead of being checked against its own reading.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared `raster` module rather than by copying it.
//!
//! `BmpSnapshot::pixels` is always the DECODED, palette-resolved canonical RGBA buffer — it is
//! independent of `BmpSnapshot::palette` (a `InsertPaletteEntry`/`RemovePaletteEntry`/
//! `SetPaletteEntry` mutation only ever touches `palette`, never remaps `pixels`). The subject's own
//! `encode_bmp` (`🚪️io/🦀️component.rs`'s own `EncodeScopeNote`) now DOES persist a real 1/4/8-bit
//! indexed BITMAPINFOHEADER when the snapshot declares one — the old always-24-bit,
//! palette-discarding behaviour was the bug this module used to document as a fact of life. What
//! stays true here is narrower than before: `image` 0.25's own reference `BmpEncoder` still has no
//! API to apply a targeted palette edit independently (it only ever accepts a whole palette
//! up-front for `L8`/`La8` targets), so this oracle's honest independent answer for
//! `insert-palette-entry`/`remove-palette-entry`/`set-palette-entry` remains "decode and re-encode
//! unchanged" — the DECODED sample buffer these three ever touch is nothing, on either side, since
//! they only ever edit `palette` metadata, and the subject's own fixture-derived scenarios (see
//! `../../../../🧪️tests/mutate-bmp-v3/component.feature`) target a palette entry no pixel actually
//! resolves to, so the subject's now-real indexed re-encode stays representable and decodes back to
//! the identical samples too — a faithful, not a fabricated, agreement, just no longer one caused by
//! the subject silently dropping the palette. A mutation that instead recolors or removes the ONLY
//! remaining palette entry some pixel's canonical color resolves to makes that pixel genuinely
//! unrepresentable in the new table; the subject's `encode_bmp` reports that as an `Err` rather than
//! silently narrowing or falling back to 24-bit — see `🚪️io/🦀️component.rs`'s own
//! `unrepresentable_palette_edit_is_reported_not_narrowed` test. `set-header-fields` is exercised
//! through `row_order`, the one header field with a real on-disk effect that still can never change
//! the DECODED, canonicalized samples either side's independent reader recovers. Only `set-snapshot`
//! and `set-pixel-data` touch pixel content, and both re-encode through it.
//!
//! The real fixture is 2334x2560 (~24M RGBA bytes); `raster::RasterSpec::projection`'s flat
//! per-sample JSON array is fine for the small synthetic gradients it was built for but is the
//! wrong shape at this size, so `project_bmp_mutation` (below) reports a content digest of the decoded
//! samples instead — still an EXACT comparison (BMP is lossless), just one that does not require
//! serializing tens of millions of JSON numbers per scenario.
//!
//! @see ../🧪️oracle/🔣️component.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself (`KINDS`).

use semio_repo_test_host::Json;

//#region 🔖️Oracles
#[cfg(feature = "oracles")]
mod oracles {
    use crate::raster::{oracle_create_image, RasterSpec};
    use semio_repo_test_host::{digest, Json};

    const FORMAT: &str = "bmp";

    //#region 🔖️Json
    /// 🔎️ Numeric member, or `None` for anything else — params are authored directly in the
    /// feature file, so a missing/mistyped field is a legitimate default, never a panic.
    fn num(params: &Json, key: &str) -> Option<f64> {
        match params.get(key) {
            Some(Json::Number(value)) => Some(*value),
            _ => None,
        }
    }
    fn as_arr(value: &Json) -> &[Json] {
        match value {
            Json::Array(items) => items,
            _ => &[],
        }
    }
    fn num_at(items: &[Json], index: usize) -> Option<f64> {
        match items.get(index) {
            Some(Json::Number(value)) => Some(*value),
            _ => None,
        }
    }
    fn empty_params() -> Json {
        Json::Object(Vec::new())
    }
    //#endregion 🔖️Json

    //#region 🔖️Decode
    /// 👁️ Decodes with the INDEPENDENT `image` reader, canonicalizing to 8-bit RGBA — mirrors what
    /// this repository's own `decode_bmp` canonicalizes `pixels` to, so a mutation applied on top
    /// is comparing like with like.
    fn decode_rgba(input: &[u8]) -> Result<(u32, u32, Vec<u8>), String> {
        let decoded = image::load_from_memory(input).map_err(|error| format!("independent reader could not parse the BMP: {error}"))?;
        let rgba = decoded.to_rgba8();
        Ok((rgba.width(), rgba.height(), rgba.into_raw()))
    }
    //#endregion 🔖️Decode

    //#region 🔖️Encode
    /// 🔁️ Decodes and re-encodes unchanged — the correct oracle answer for every kind whose
    /// forward effect never touches pixels/dimensions (`no-mutation` and the three palette kinds,
    /// plus `set-header-fields`'s `row_order`, none of which the DECODED sample buffer can ever
    /// show — see this module's own doc comment), and for `undo_mutation` universally (its own doc
    /// comment explains why).
    fn reencode_unchanged(input: &[u8]) -> Result<Vec<u8>, String> {
        let (width, height, rgba) = decode_rgba(input)?;
        oracle_create_image(&RasterSpec { width, height, rgba }, FORMAT)
    }
    //#endregion 🔖️Encode

    //#region 🔖️Forward
    fn fill_quad(params: &Json) -> Vec<u8> {
        let fill = as_arr(params.get("fill").unwrap_or(&Json::Null));
        (0..4).map(|index| num_at(fill, index).unwrap_or(0.0) as u8).collect()
    }

    fn solid_rgba(width: u32, height: u32, quad: &[u8]) -> Vec<u8> {
        let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);
        for _ in 0..(width as usize * height as usize) {
            rgba.extend_from_slice(quad);
        }
        rgba
    }

    /// 📄️ Replaces the WHOLE document — independent of the real input, matching `SetSnapshot`'s
    /// own wholesale-replace semantics (a small solid-color image rather than another ~24MB
    /// buffer inlined into the feature file).
    fn forward_set_snapshot(params: &Json) -> Result<Vec<u8>, String> {
        let width = num(params, "width").unwrap_or(1.0).max(1.0) as u32;
        let height = num(params, "height").unwrap_or(1.0).max(1.0) as u32;
        let quad = fill_quad(params);
        oracle_create_image(&RasterSpec { width, height, rgba: solid_rgba(width, height, &quad) }, FORMAT)
    }

    /// 🖼️ Replaces every pixel with a solid fill colour, keeping the real fixture's own
    /// dimensions — matching `SetPixelData`'s own whole-buffer-replace semantics.
    fn forward_set_pixel_data(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let (width, height, _rgba) = decode_rgba(input)?;
        let quad = fill_quad(params);
        oracle_create_image(&RasterSpec { width, height, rgba: solid_rgba(width, height, &quad) }, FORMAT)
    }
    //#endregion 🔖️Forward

    //#region 🔖️Project
    /// 👁️ Projects BMP bytes with the INDEPENDENT `image` decoder onto a digest-based shape —
    /// exact (BMP is lossless) without inlining the real fixture's ~24M decoded samples as JSON.
    pub fn project(input: &[u8]) -> Result<Json, String> {
        let (width, height, rgba) = decode_rgba(input)?;
        Ok(Json::Object(vec![
            ("format".to_string(), Json::String(FORMAT.to_string())),
            ("width".to_string(), Json::Number(width as f64)),
            ("height".to_string(), Json::Number(height as f64)),
            ("channels".to_string(), Json::Number(4.0)),
            ("bitDepth".to_string(), Json::Number(8.0)),
            ("pixelsDigest".to_string(), Json::String(digest(&rgba))),
        ]))
    }
    //#endregion 🔖️Project

    //#region 🔖️Dispatch
    /// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized
    /// bytes. An unrecognised kind is an error, never a silent no-op: a mutation that is quietly
    /// skipped reports as a passing test.
    pub fn apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
        let kind = spec.str("kind");
        let params = spec.get("params").cloned().unwrap_or_else(empty_params);
        match kind.as_str() {
            "" => Err("mutation spec carries no `kind`".to_string()),
            "no-mutation" => reencode_unchanged(input),
            "set-snapshot" => forward_set_snapshot(&params),
            "set-header-fields" => reencode_unchanged(input),
            "insert-palette-entry" => reencode_unchanged(input),
            "remove-palette-entry" => reencode_unchanged(input),
            "set-palette-entry" => reencode_unchanged(input),
            "set-pixel-data" => forward_set_pixel_data(input, &params),
            other => Err(format!("mutation kind {other:?} has no oracle implementation ({} input byte(s))", input.len())),
        }
    }

    /// ↩️ The `inverse-<kind>` scenarios' oracle: the reference's own inverse, APPLIED to the
    /// forward mutation's real output rather than short-circuited to the pristine original.
    /// `BmpMutation::inverse` (the vocabulary's own algebraic law,
    /// `../🧬️schema/🧬️mutations/🦀️component.rs`) is defined, per variant, as "restore `base`'s own
    /// value for the field this kind touches", so:
    ///
    /// * `set-snapshot` replaced the whole document, and its inverse is the whole original;
    /// * `set-pixel-data` replaced the sample buffer, and its inverse re-encodes the MUTATED
    ///   document's own geometry carrying the ORIGINAL's samples;
    /// * the three palette kinds and `set-header-fields` touch only metadata the decoded,
    ///   canonicalized sample buffer never carries, so re-encoding the MUTATED document is their
    ///   inverse (see this module's own doc comment for why that is faithful, not a shortcut).
    ///
    /// The point of routing through `mutated` is that the forward result now has to survive a real
    /// independent re-parse: the previous version never looked at it, so a forward mutation that
    /// emitted an undecodable BMP still reported the `inverse-<kind>` scenario as passing.
    pub fn undo_mutation(original_input: &[u8], spec: &Json, mutated: &[u8]) -> Result<Vec<u8>, String> {
        let kind = spec.str("kind");
        match kind.as_str() {
            "" => Err("mutation spec carries no `kind`".to_string()),
            "set-snapshot" => reencode_unchanged(original_input),
            "set-pixel-data" => {
                let (_, _, original_rgba) = decode_rgba(original_input)?;
                let (width, height, _) = decode_rgba(mutated)?;
                oracle_create_image(&RasterSpec { width, height, rgba: original_rgba }, FORMAT)
            }
            _ => reencode_unchanged(mutated),
        }
    }
    //#endregion 🔖️Dispatch
}
//#endregion 🔖️Oracles

//#region 🔖️Dispatch
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    oracles::apply_mutation(input, spec)
}

#[cfg(feature = "oracles")]
pub fn oracle_undo_mutation(original_input: &[u8], spec: &Json, mutated: &[u8]) -> Result<Vec<u8>, String> {
    oracles::undo_mutation(original_input, spec, mutated)
}

#[cfg(feature = "oracles")]
pub fn project_bmp_mutation(input: &[u8]) -> Result<Json, String> {
    oracles::project(input)
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_undo_mutation(_original_input: &[u8], _spec: &Json, _mutated: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn project_bmp_mutation(_input: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch
