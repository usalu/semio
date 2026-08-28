//! 🦀️ BMP v3/any exhaustive mutation case — Rust adapter, structured like
//! `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/edit-existing-pdf/🦀️.rs`: oracle
//! handlers at top level, subject handlers inside `#[cfg(feature = "sut")] mod subject`, both
//! projected through the same INDEPENDENT `image` reader (`project_bmp_mutation`) before comparison.
//!
//! Every `mutate-<kind>`/`inverse-<kind>` pair is registered from the ONE `KINDS` list this file,
//! the catalog manifest and the vocabulary's own `KINDS` constant all separately spell out —
//! `bun ./📜️script.ts contract` is what keeps all three honest against each other (the framework
//! never parses Rust to check it itself).
//!
//! The oracle side never touches this repository's own codec: `oracle_apply_mutation`/
//! `oracle_undo_mutation` (this subset's own `🧪️oracle/🦀️component.rs`) perform every kind
//! independently against the registered `image` reference crate. The subject side fully parses the
//! real document into the typed `BmpSnapshot` and re-serializes from it — never splices bytes.

use semio_s_plugin_stdio_test_oracle::artifacts::bmp::standards::v3::subsets::any::oracle_identity_round_trip;
use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::bmp::standards::v_v3::subsets::any::{oracle_apply_mutation, oracle_undo_mutation, project_bmp_mutation};
use semio_s_plugin_stdio_test_oracle::law;

//#region 🔖️Kinds
/// 📇️ Mirrors `../../🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`'s own
/// `KINDS` and `../../🏅️standards/🔖️v3/🪆️subsets/✳️any/🧪️oracle/🔣️.json`'s
/// `mutationCatalogs[0].kinds` — kept in the SAME declaration order in all three; a mismatch is
/// caught loudly (either by the contract phase, or by the runner's own "no registration for
/// scenario" error) rather than silently.
const KINDS: &[&str] = &["change-header-fields", "insert-palette-entry", "remove-palette-entry", "replace-palette-entry", "replace-pixel-data"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://🖼️rathaus-ahlen-grundriss.bmp";

/// 🧫️ Copies the immutable real fixture into the work directory and returns the mutable copy's
/// bytes — the committed 250 KB-source, 2334x2560, 8-bit palette architectural floor plan
/// (`rathaus-ahlen-grundriss.bmp`, derived once — see `component.feature`'s own description) is
/// never written to.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.bmp"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}


//#endregion 🔖️Input

//#region 🔖️Oracle
/// 👁️ `@id-mutate`: applies the row's kind with the reference `image` codec and ASSERTS the result
/// is distinguishable from the untouched fixture. BMP v3 is lossless and every one of this
/// vocabulary's seven kinds reaches the compared projection, so the exemption list is empty and
/// stays empty: a kind that stops moving it is a regression in the oracle or the projection, not a
/// fact about the format.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let before = project_bmp_mutation(&input)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_bmp_mutation(&bytes)?;
    law::mutation_is_observable(&spec.str("kind"), &projection, &before, &[])?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ The inverse law, asserted rather than assumed: the reference `image` codec applies the row's
/// kind, then its own computed inverse ON TOP OF that real forward result, and the outcome must
/// project back onto the pristine original. Returning `undo_mutation(original)` without ever
/// applying the forward mutation (what this used to do) asserted nothing — the scenario passed
/// whenever the reference crate re-encoded the untouched fixture without erroring.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let before = project_bmp_mutation(&input)?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let bytes = oracle_undo_mutation(&input, &spec, &mutated)?;
    let projection = project_bmp_mutation(&bytes)?;
    law::inverse_restores(&spec.str("kind"), &projection, &before)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔁️ The identity law on the ORACLE side, in its EXACT-BYTES form rather than its
/// no-pass-through form — the two are mirrors, and which one applies is a property of the carrier.
///
/// An uncompressed BMP v3 leaves a writer no freedom at all: a 14-byte BITMAPFILEHEADER and a
/// 40-byte BITMAPINFOHEADER whose every field is determined by the image, a colour table that is
/// the palette verbatim, and a pixel array that is the index buffer padded to a 4-byte row stride.
/// There is no filter choice, no compression level, no chunk ordering. On top of that, the
/// committed fixture was AUTHORED by this same reference encoder (see the subset oracle's
/// `fixture_derivation`), so anything other than a byte-for-byte reproduction is a defect in the
/// reader or the writer, not writer freedom — and `carrier_is_exact` is a strictly stronger claim
/// than "the bytes moved" would be. `law::reparsed_not_copied` would be exactly backwards here.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_identity_round_trip(&input)?;
    let before = project_bmp_mutation(&input)?;
    let projection = project_bmp_mutation(&bytes)?;
    law::round_trip_preserves(&projection, &before)?;
    law::carrier_is_exact(&bytes, &input)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_s_plugin_stdio_test_oracle::law;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::artifacts::bmp::standards::v_v3::subsets::any::project_bmp_mutation;
    use semio_s_plugin_stdio::artifacts::bmp::standards::v_v3::subsets::any::io::{decode_bmp, encode_bmp};
    use semio_s_plugin_stdio::artifacts::bmp::standards::v_v3::subsets::any::schema::mutations::{apply_bmp_mutation, inverse_bmp_mutation, BmpMutation};
    use semio_s_plugin_stdio::artifacts::bmp::standards::v_v3::subsets::any::schema::snapshot::{BmpPaletteEntry, BmpRowOrder};
    use semio_s_plugin_stdio::artifacts::bmp::BmpSnapshot;
    use semio_s_plugin_stdio::ArtifactDsl;

    //#region 🔖️Json
    fn num(params: &Json, key: &str) -> Option<f64> {
        match params.get(key) {
            Some(Json::Number(value)) => Some(*value),
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
    fn fill_quad(params: &Json) -> Vec<u8> {
        let fill = as_arr(params.get("fill").unwrap_or(&Json::Null));
        (0..4).map(|index| num_at(fill, index).unwrap_or(0.0) as u8).collect()
    }
    fn solid_pixels(width: u32, height: u32, quad: &[u8]) -> Vec<u8> {
        let mut pixels = Vec::with_capacity(width as usize * height as usize * 4);
        for _ in 0..(width as usize * height as usize) {
            pixels.extend_from_slice(quad);
        }
        pixels
    }
    fn entry_from(value: &Json) -> BmpPaletteEntry {
        BmpPaletteEntry { b: num(value, "b").unwrap_or(0.0) as u8, g: num(value, "g").unwrap_or(0.0) as u8, r: num(value, "r").unwrap_or(0.0) as u8, reserved: num(value, "reserved").unwrap_or(0.0) as u8 }
    }
    //#endregion 🔖️Json

    //#region 🔖️MutationFromSpec
    /// 🔮️ Builds the real typed `BmpMutation` the feature's `{"kind","params"}` docstring
    /// describes — the ONLY channel from the scenario's authored parameters to the production
    /// mutation pipeline; `apply_bmp_mutation` does the rest.
    fn mutation_from_spec(kind: &str, params: &Json, base: &BmpSnapshot) -> Result<BmpMutation, String> {
        match kind {
            "change-header-fields" => Ok(BmpMutation::ChangeHeaderFields(semio_s_plugin_stdio::artifacts::bmp::schema::mutations::ChangeHeaderFieldsMutation {
                header_size: None,
                width: None,
                height: None,
                row_order: as_str(params, "rowOrder").map(|value| if value == "top-down" { BmpRowOrder::TopDown } else { BmpRowOrder::BottomUp }),
                planes: None,
                bits_per_pixel: None,
                compression: None,
                image_size: None,
                x_pixels_per_meter: num(params, "xPixelsPerMeter").map(|value| value as i32),
                y_pixels_per_meter: num(params, "yPixelsPerMeter").map(|value| value as i32),
                colors_used: None,
                colors_important: None,
            })),
            "insert-palette-entry" => Ok(BmpMutation::InsertPaletteEntry(semio_s_plugin_stdio::artifacts::bmp::schema::mutations::InsertPaletteEntryMutation { index: num(params, "index").unwrap_or(0.0) as usize, entry: entry_from(params.get("entry").unwrap_or(&Json::Null)) })),
            "remove-palette-entry" => Ok(BmpMutation::RemovePaletteEntry(semio_s_plugin_stdio::artifacts::bmp::schema::mutations::RemovePaletteEntryMutation { index: num(params, "index").unwrap_or(0.0) as usize })),
            "replace-palette-entry" => Ok(BmpMutation::ReplacePaletteEntry(semio_s_plugin_stdio::artifacts::bmp::schema::mutations::ReplacePaletteEntryMutation { index: num(params, "index").unwrap_or(0.0) as usize, entry: entry_from(params.get("entry").unwrap_or(&Json::Null)) })),
            "replace-pixel-data" => Ok(BmpMutation::ReplacePixelData(semio_s_plugin_stdio::artifacts::bmp::schema::mutations::ReplacePixelDataMutation { pixels: solid_pixels(base.width, base.height, &fill_quad(params)) })),
            other => Err(format!("mutation kind {other:?} has no subject implementation")),
        }
    }
    //#endregion 🔖️MutationFromSpec

    //#region 🔖️Handlers
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode_bmp(&mutable_input(ctx)?).map_err(|error| format!("decode_bmp failed: {error}"))?;
        let spec = ctx.doc_json()?;
        let mutation = mutation_from_spec(&spec.str("kind"), spec.get("params").unwrap_or(&Json::Null), &snapshot)?;
        let _ = apply_bmp_mutation(&mut snapshot, &mutation);
        let bytes = encode_bmp(&snapshot).map_err(|error| format!("encode_bmp failed: {error}"))?;
        let projection = project_bmp_mutation(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// ↩️ Applies the forward mutation, then applies EVERY mutation `BmpMutation::inverse` returns
    /// (the vocabulary's own algebraic law, index-aware, computed against the pre-forward `base`)
    /// — the real production undo pipeline, not a hand-derived counter-mutation.
    pub fn undo(ctx: &Context) -> Result<Outcome, String> {
        let base = decode_bmp(&mutable_input(ctx)?).map_err(|error| format!("decode_bmp failed: {error}"))?;
        let spec = ctx.doc_json()?;
        let mutation = mutation_from_spec(&spec.str("kind"), spec.get("params").unwrap_or(&Json::Null), &base)?;
        let mut snapshot = base.clone();
        let _ = apply_bmp_mutation(&mut snapshot, &mutation);
        for inverse in inverse_bmp_mutation(&mutation, &base) {
            let _ = apply_bmp_mutation(&mut snapshot, &inverse);
        }
        let bytes = encode_bmp(&snapshot).map_err(|error| format!("encode_bmp failed: {error}"))?;
        let projection = project_bmp_mutation(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🔁️ `decode_bmp` → `print_dsl` (the DSL hex-dump text codec — BMP has no separate textual
    /// format of its own, see
    /// `../../../🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`) → `parse_dsl`
    /// → `encode_bmp` is the ONLY channel from input to output.
    ///
    /// The law asserted is EXACT bytes, not "the bytes moved" — see `identity_round_trip_oracle`'s
    /// own doc comment for why an uncompressed BMP v3 leaves a writer no freedom to differ in. A
    /// pass-through tripwire would be meaningless here (the correct answer and the cheat answer are
    /// the same bytes) whereas exactness fails the moment either codec drifts, which is the real
    /// risk. The channel above is what rules the cheat out structurally: nothing but the typed
    /// snapshot, printed to text and reparsed, reaches `encode_bmp`.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_bmp(&input).map_err(|error| format!("decode_bmp failed: {error}"))?;
        let text = <BmpSnapshot as ArtifactDsl>::print_dsl(&snapshot);
        let reparsed = <BmpSnapshot as ArtifactDsl>::parse_dsl(&text).map_err(|error| format!("parse_dsl failed: {error:?}"))?;
        let output = encode_bmp(&reparsed).map_err(|error| format!("encode_bmp failed: {error}"))?;
        let projection = project_bmp_mutation(&output)?;
        law::carrier_is_exact(&output, &input)?;
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
