//! 🦀️ PNG 1.2/any exhaustive mutation case — Rust adapter, structured like
//! `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🧪️tests/✏️edit-existing-pdf/🦀️.rs`: oracle
//! handlers at top level, subject handlers inside `#[cfg(feature = "sut")] mod subject`, both
//! projected through the same INDEPENDENT `png` reader (`project_png_mutation`) before comparison.
//!
//! Every `mutate-<kind>`/`inverse-<kind>` pair is registered from the ONE `KINDS` list this file,
//! the catalog manifest and the vocabulary's own `KINDS` constant all separately spell out —
//! `bun ./📜️script.ts contract` is what keeps all three honest against each other (the framework
//! never parses Rust to check it itself).
//!
//! The oracle side never touches this repository's own codec: `oracle_apply_mutation`/
//! `oracle_undo_mutation` (this subset's own `🦀️.rs`) perform every kind
//! independently against the registered `png` reference crate. The subject side fully parses the
//! real document into the typed `PngSnapshot` and re-serializes from it — never splices bytes.

use semio_s_plugin_stdio_test_oracle::artifacts::png::standards::v1_2::subsets::any::oracle_identity_round_trip;
use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::png::standards::v1_2::subsets::any::{oracle_apply_mutation, oracle_arrange, oracle_undo_mutation, project_png_mutation};
use semio_s_plugin_stdio_test_oracle::law;

//#region 🔖️Kinds
/// 📇️ Mirrors `../../🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`'s own
/// `KINDS` and `../../🏅️standards/🔖️1.2/🪆️subsets/✳️any/🔣️oracle.json`'s
/// `mutationCatalogs[0].kinds` — kept in the SAME declaration order in all three; a mismatch is
/// caught loudly (either by the contract phase, or by the runner's own "no registration for
/// scenario" error) rather than silently.
const KINDS: &[&str] = &["change-header", "replace-palette", "change-transparency", "change-gamma", "change-chromaticities", "change-srgb-intent", "change-physical-dims", "change-timestamp", "change-background", "insert-text-chunk", "remove-text-chunk", "replace-text-chunk", "replace-pixels", "insert-unknown-chunk", "remove-unknown-chunk"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://🏛️rathaus-ahlen-grundriss/🖼️.png";

/// 🧫️ Copies the immutable real fixture into the work directory and returns the mutable copy's
/// bytes — the committed 250 KB, 2334x2560, 8-bit COLORMAP architectural floor plan
/// (`rathaus-ahlen-grundriss.png`) is never written to.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.png"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}



/// 🎬️ The document a kind actually acts on. The committed floor plan carries exactly
/// IHDR/PLTE/IDAT/IEND — no text chunk, no private chunk, no tRNS — so the three kinds that address
/// an EXISTING text or unknown chunk are handed the real document with their target inserted first,
/// by the reference implementation. Every other kind gets the committed bytes untouched.
fn arranged_input(ctx: &Context, spec: &Json) -> Result<Vec<u8>, String> {
    oracle_arrange(&mutable_input(ctx)?, spec)
}

/// 🚫️ The two kinds this subset's serialization genuinely cannot show, each for a reason stated in
/// the oracle module, in `encode_png`'s own `🚫️EncodeScopeNote` and in the feature description:
/// `change-header` (IHDR must describe the canonical RGBA IDAT that follows it, and `SetHeader` does
/// not resize `pixels`, so no field of it can reach the bytes) and `change-transparency` (§11.3.3
/// forbids tRNS at colour type 6, which is the only colour type either encoder writes). Naming them
/// here is what keeps the other fifteen honest: the law below fails any kind not on this list that
/// leaves the projection untouched.
const UNOBSERVABLE: &[&str] = &["change-header", "change-transparency"];
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 👁️ `@id-mutate`: applies the row's kind with the reference `png` codec and ASSERTS the result is
/// distinguishable from its own pre-state. Without that assertion a kind whose effect lands outside
/// the projection passes exactly as an unchanged round trip does, which is the defect this case carried while
/// its projection reported geometry and a sample digest alone.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let base = arranged_input(ctx, &spec)?;
    let before = project_png_mutation(&base)?;
    let bytes = oracle_apply_mutation(&base, &spec)?;
    let projection = project_png_mutation(&bytes)?;
    law::mutation_is_observable(&spec.str("kind"), &projection, &before, UNOBSERVABLE)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ The inverse law, asserted rather than assumed: the reference `png` codec applies the row's
/// kind, then its own computed inverse ON TOP OF that real forward result, and the outcome must
/// project back onto the pristine original. Returning `undo_mutation(original)` without ever
/// applying the forward mutation (what this used to do) asserted nothing — the scenario passed
/// whenever the reference crate re-encoded the untouched fixture without erroring.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let base = arranged_input(ctx, &spec)?;
    let before = project_png_mutation(&base)?;
    let mutated = oracle_apply_mutation(&base, &spec)?;
    let bytes = oracle_undo_mutation(&base, &spec, &mutated)?;
    let projection = project_png_mutation(&bytes)?;
    law::inverse_restores(&spec.str("kind"), &projection, &before)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔁️ The no-byte-pass-through law on the ORACLE side: the reference `png` codec decodes the real
/// document and re-encodes it from its own RGBA buffer alone, so the bytes must move (its filter
/// choices, deflate level and chunk layout are not this fixture's) while the semantic projection —
/// geometry plus the decoded-sample digest — must not.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_identity_round_trip(&input)?;
    law::reparsed_not_copied(&bytes, &input)?;
    let before = project_png_mutation(&input)?;
    let projection = project_png_mutation(&bytes)?;
    law::round_trip_preserves(&projection, &before)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{arranged_input, mutable_input};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::artifacts::png::standards::v1_2::subsets::any::project_png_mutation;
    use semio_s_plugin_stdio::ArtifactDsl;
    use semio_s_plugin_stdio::artifacts::png::standards::v1_2::subsets::any::io::{decode_png, encode_png};
    use semio_s_plugin_stdio::artifacts::png::standards::v1_2::subsets::any::schema::mutations::{apply_png_mutation, inverse_png_mutation, PngMutation};
    use semio_s_plugin_stdio::artifacts::png::standards::v1_2::subsets::any::schema::snapshot::{PngBackground, PngChromaticities, PngChunk, PngChunkMarker, PngColorType, PngPhysicalDims, PngRgb, PngSnapshot, PngSrgbIntent, PngTextChunk, PngTextKind, PngTimestamp};

    //#region 🔖️Json
    fn num(params: &Json, key: &str) -> Option<f64> {
        match params.get(key) {
            Some(Json::Number(value)) => Some(*value),
            _ => None,
        }
    }
    fn as_bool(params: &Json, key: &str) -> Option<bool> {
        match params.get(key) {
            Some(Json::Bool(value)) => Some(*value),
            _ => None,
        }
    }
    fn as_str<'a>(params: &'a Json, key: &str) -> Option<&'a str> {
        match params.get(key) {
            Some(Json::String(value)) => Some(value.as_str()),
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
    fn color_type_from(value: &str) -> PngColorType {
        match value {
            "grayscale" => PngColorType::Grayscale,
            "rgb" => PngColorType::Rgb,
            "palette" => PngColorType::Palette,
            "grayscale-alpha" => PngColorType::GrayscaleAlpha,
            _ => PngColorType::Rgba,
        }
    }
    fn srgb_from(value: &str) -> PngSrgbIntent {
        match value {
            "relative-colorimetric" => PngSrgbIntent::RelativeColorimetric,
            "saturation" => PngSrgbIntent::Saturation,
            "absolute-colorimetric" => PngSrgbIntent::AbsoluteColorimetric,
            _ => PngSrgbIntent::Perceptual,
        }
    }
    fn text_chunk_from(params: &Json) -> PngTextChunk {
        PngTextChunk { keyword: as_str(params, "keyword").unwrap_or("Comment").to_string(), value: as_str(params, "value").unwrap_or("").to_string(), compressed: false, kind: PngTextKind::Text, language_tag: String::new(), translated_keyword: String::new() }
    }
    fn unknown_chunk_from(params: &Json) -> PngChunk {
        let requested = as_str(params, "kind").unwrap_or("waVe");
        let mut kind = *b"waVe";
        for (slot, byte) in kind.iter_mut().zip(requested.bytes()) {
            *slot = byte;
        }
        PngChunk { kind, data: as_str(params, "data").unwrap_or("").as_bytes().to_vec() }
    }
    fn solid_pixels(base: &PngSnapshot, params: &Json) -> Vec<u8> {
        let fill = as_arr(params.get("fill").unwrap_or(&Json::Null));
        let quad: Vec<u8> = (0..4).map(|index| num_at(fill, index).unwrap_or(0.0) as u8).collect();
        let mut pixels = Vec::with_capacity(base.pixels.len());
        for _ in 0..(base.width as usize * base.height as usize) {
            pixels.extend_from_slice(&quad);
        }
        pixels
    }
    //#endregion 🔖️Json

    //#region 🔖️MutationFromSpec
    /// 🔮️ Builds the real typed `PngMutation` the feature's `{"kind","params"}` docstring
    /// describes — the ONLY channel from the scenario's authored parameters to the production
    /// mutation pipeline; `apply_png_mutation` does the rest.
    fn mutation_from_spec(kind: &str, params: &Json, base: &PngSnapshot) -> Result<PngMutation, String> {
        match kind {
            "change-header" => Ok(PngMutation::ChangeHeader(semio_s_plugin_stdio::artifacts::png::schema::mutations::ChangeHeaderMutation { width: num(params, "width").unwrap_or(base.width as f64) as u32, height: num(params, "height").unwrap_or(base.height as f64) as u32, bit_depth: num(params, "bitDepth").unwrap_or(base.bit_depth as f64) as u8, color_type: color_type_from(as_str(params, "colorType").unwrap_or("rgba")), interlace: as_bool(params, "interlace").unwrap_or(base.interlace) })),
            "replace-palette" => {
                let entries = as_arr(params.get("plte").unwrap_or(&Json::Null));
                let plte = entries.iter().map(|entry| { let channels = as_arr(entry); PngRgb { r: num_at(channels, 0).unwrap_or(0.0) as u8, g: num_at(channels, 1).unwrap_or(0.0) as u8, b: num_at(channels, 2).unwrap_or(0.0) as u8 } }).collect();
                Ok(PngMutation::ReplacePalette(semio_s_plugin_stdio::artifacts::png::schema::mutations::ReplacePaletteMutation { plte: Some(plte) }))
            }
            // 👁️ tRNS is structurally invalid alongside color type 6 (truecolor+alpha) — see this
            // subset's own oracle module for the full reasoning. `None` is the only decode-safe
            // exercise given `encode_png`'s always-RGBA6 output.
            "change-transparency" => Ok(PngMutation::ChangeTransparency(semio_s_plugin_stdio::artifacts::png::schema::mutations::ChangeTransparencyMutation { trns: None })),
            "change-gamma" => Ok(PngMutation::ChangeGamma(semio_s_plugin_stdio::artifacts::png::schema::mutations::ChangeGammaMutation { gama: num(params, "gama").map(|value| value as u32) })),
            "change-chromaticities" => Ok(PngMutation::ChangeChromaticities(semio_s_plugin_stdio::artifacts::png::schema::mutations::ChangeChromaticitiesMutation {
                chrm: Some(PngChromaticities { white_x: num(params, "whiteX").unwrap_or(0.0) as u32, white_y: num(params, "whiteY").unwrap_or(0.0) as u32, red_x: num(params, "redX").unwrap_or(0.0) as u32, red_y: num(params, "redY").unwrap_or(0.0) as u32, green_x: num(params, "greenX").unwrap_or(0.0) as u32, green_y: num(params, "greenY").unwrap_or(0.0) as u32, blue_x: num(params, "blueX").unwrap_or(0.0) as u32, blue_y: num(params, "blueY").unwrap_or(0.0) as u32 }),
            })),
            "change-srgb-intent" => Ok(PngMutation::ChangeSrgbIntent(semio_s_plugin_stdio::artifacts::png::schema::mutations::ChangeSrgbIntentMutation { srgb: Some(srgb_from(as_str(params, "srgb").unwrap_or("perceptual"))) })),
            "change-physical-dims" => Ok(PngMutation::ChangePhysicalDims(semio_s_plugin_stdio::artifacts::png::schema::mutations::ChangePhysicalDimsMutation { phys: Some(PngPhysicalDims { ppu_x: num(params, "ppuX").unwrap_or(0.0) as u32, ppu_y: num(params, "ppuY").unwrap_or(0.0) as u32, unit_is_meter: as_bool(params, "unitIsMeter").unwrap_or(false) }) })),
            "change-timestamp" => Ok(PngMutation::ChangeTimestamp(semio_s_plugin_stdio::artifacts::png::schema::mutations::ChangeTimestampMutation { time: Some(PngTimestamp { year: num(params, "year").unwrap_or(2024.0) as u16, month: num(params, "month").unwrap_or(1.0) as u8, day: num(params, "day").unwrap_or(1.0) as u8, hour: num(params, "hour").unwrap_or(0.0) as u8, minute: num(params, "minute").unwrap_or(0.0) as u8, second: num(params, "second").unwrap_or(0.0) as u8 }) })),
            // 🖼️ Always the `Rgb{r,g,b}` (6-byte) variant — the only bKGD layout compatible with
            // the color-type-6 output every re-encode here produces (§11.3.5.1).
            "change-background" => Ok(PngMutation::ChangeBackground(semio_s_plugin_stdio::artifacts::png::schema::mutations::ChangeBackgroundMutation { bkgd: Some(PngBackground::Rgb { r: num(params, "r").unwrap_or(0.0) as u16, g: num(params, "g").unwrap_or(0.0) as u16, b: num(params, "b").unwrap_or(0.0) as u16 }) })),
            "insert-text-chunk" => Ok(PngMutation::InsertTextChunk(semio_s_plugin_stdio::artifacts::png::schema::mutations::InsertTextChunkMutation { index: num(params, "index").unwrap_or(0.0) as usize, chunk: text_chunk_from(params) })),
            "remove-text-chunk" => Ok(PngMutation::RemoveTextChunk(semio_s_plugin_stdio::artifacts::png::schema::mutations::RemoveTextChunkMutation { index: num(params, "index").unwrap_or(0.0) as usize })),
            "replace-text-chunk" => Ok(PngMutation::ReplaceTextChunk(semio_s_plugin_stdio::artifacts::png::schema::mutations::ReplaceTextChunkMutation { index: num(params, "index").unwrap_or(0.0) as usize, chunk: text_chunk_from(params) })),
            "replace-pixels" => Ok(PngMutation::ReplacePixels(semio_s_plugin_stdio::artifacts::png::schema::mutations::ReplacePixelsMutation { pixels: solid_pixels(base, params) })),
            "insert-unknown-chunk" => Ok(PngMutation::InsertUnknownChunk(semio_s_plugin_stdio::artifacts::png::schema::mutations::InsertUnknownChunkMutation { index: num(params, "index").unwrap_or(0.0) as usize, chunk: unknown_chunk_from(params) })),
            "remove-unknown-chunk" => Ok(PngMutation::RemoveUnknownChunk(semio_s_plugin_stdio::artifacts::png::schema::mutations::RemoveUnknownChunkMutation { index: num(params, "index").unwrap_or(0.0) as usize })),
            other => Err(format!("mutation kind {other:?} has no subject implementation")),
        }
    }
    //#endregion 🔖️MutationFromSpec

    //#region 🔖️Handlers
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let mut snapshot = decode_png(&arranged_input(ctx, &spec)?).map_err(|error| format!("decode_png failed: {error}"))?;
        let mutation = mutation_from_spec(&spec.str("kind"), spec.get("params").unwrap_or(&Json::Null), &snapshot)?;
        let _ = apply_png_mutation(&mut snapshot, &mutation);
        let bytes = encode_png(&snapshot).map_err(|error| format!("encode_png failed: {error}"))?;
        let projection = project_png_mutation(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// ↩️ Applies the forward mutation, then applies EVERY mutation `PngMutation::inverse`
    /// returns (the vocabulary's own algebraic law, index-aware, computed against the pre-forward
    /// `base`) — the real production undo pipeline, not a hand-derived counter-mutation.
    pub fn undo(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let base = decode_png(&arranged_input(ctx, &spec)?).map_err(|error| format!("decode_png failed: {error}"))?;
        let mutation = mutation_from_spec(&spec.str("kind"), spec.get("params").unwrap_or(&Json::Null), &base)?;
        let mut snapshot = base.clone();
        let _ = apply_png_mutation(&mut snapshot, &mutation);
        for inverse in inverse_png_mutation(&mutation, &base) {
            let _ = apply_png_mutation(&mut snapshot, &inverse);
        }
        let bytes = encode_png(&snapshot).map_err(|error| format!("encode_png failed: {error}"))?;
        let projection = project_png_mutation(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🚫️ The no-byte-pass-through tripwire: `decode_png` → `print_dsl` (the subset's own text
    /// codec) → `parse_dsl` → `encode_png` is the ONLY channel from input to output; identical
    /// output bytes would mean the input was smuggled through rather than parsed.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_png(&input).map_err(|error| format!("decode_png failed: {error}"))?;
        let text = <PngSnapshot as ArtifactDsl>::print_dsl(&snapshot);
        let reparsed = <PngSnapshot as ArtifactDsl>::parse_dsl(&text).map_err(|error| format!("parse_dsl failed: {error:?}"))?;
        let output = encode_png(&reparsed).map_err(|error| format!("encode_png failed: {error}"))?;
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".into());
        }
        let projection = project_png_mutation(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. One `mutate-<kind>`/`inverse-<kind>`
/// pair per declared kind, plus the standalone `identity-round-trip` scenario.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for &kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle);
        built = built.oracle(&format!("inverse-{kind}"), inverse_oracle);
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate);
            built = built.subject(&format!("inverse-{kind}"), subject::undo);
        }
    }
    built = built.oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
