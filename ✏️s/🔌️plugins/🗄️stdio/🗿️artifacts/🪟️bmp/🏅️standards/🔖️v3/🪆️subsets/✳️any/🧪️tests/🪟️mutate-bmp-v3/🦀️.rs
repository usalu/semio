//! 🦀️ BMP v3/any exhaustive mutation case — Rust adapter, structured like
//! `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🧪️tests/✏️edit-existing-pdf/🦀️.rs`: oracle
//! handlers at top level, subject handlers inside `#[cfg(feature = "sut")] mod subject`, both
//! projected through the same INDEPENDENT `image` reader (`project_bmp_mutation`) before comparison.
//!
//! Every `mutate-<kind>`/`inverse-<kind>` pair is registered from the ONE `KINDS` list this file,
//! the catalog manifest and the vocabulary's own `KINDS` constant all separately spell out —
//! `bun ./📜️script.ts contract` is what keeps all three honest against each other (the framework
//! never parses Rust to check it itself).
//!
//! The oracle side never touches this repository's own codec: `oracle_apply_mutation`/
//! `oracle_undo_mutation` (this subset's own `../../🔮️oracles/🦀️.rs`) perform every kind
//! independently against the registered `image` reference crate. The subject side fully parses the
//! real document into the typed `BmpSnapshot` and re-serializes from it — never splices bytes.

use semio_repo_test_host::{Adapter, Context, Outcome};
use semio_s_artifact_stdio_bmp_test_oracle::standards::v_v3::subsets::any::{oracle_apply_mutation, oracle_identity_round_trip, oracle_undo_mutation, project_bmp_mutation};
use semio_repo_test_host::law;


//#region 🔖️Input
/// 🧫️ Copies the immutable document the scenario's own `Given` names into the work directory and returns the
/// mutable copy's bytes — the committed 2334x2560, 8-bit palette architectural floor plan
/// (`🏛️rathaus-ahlen-grundriss/🖼️.bmp`, derived once — see `🥒️.feature`'s own description), the small indexed
/// document, or the direct-colour document. None is ever written to.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let input = ctx.step_input_uris().into_iter().next().ok_or_else(|| format!("scenario {} names no input document", ctx.scenario.id))?;
    let copy = ctx.copy_input(&input, Some("input.bmp"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 👁️ `@id-mutate`: applies the row's kind with the reference `image` codec and ASSERTS the result
/// is distinguishable from the untouched fixture. BMP v3 is lossless and every one of this
/// vocabulary's four kinds reaches the compared projection, so the exemption list is empty and
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
    use semio_repo_test_host::law;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_bmp_test_oracle::standards::v_v3::subsets::any::project_bmp_mutation;
    use semio_s_artifact_stdio_bmp::standards::v_v3::subsets::any::io::{decode_bmp, encode_bmp};
    use semio_repo_test_host::law::wire_operation;
    use semio_s_artifact_stdio_bmp::{mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_s_artifact_stdio_bmp::standards::v_v3::subsets::any::schema::mutations::{apply_bmp_mutation, BmpMutation};
    use semio_s_artifact_stdio_bmp::BmpSnapshot;
    use semio_s_artifact_stdio_bmp::ArtifactDsl;

    //#region 🔖️MutationFromSpec
    /// 🦠️ Decodes the scenario's `{"kind", "params"}` doc string: `params` is the leaf's own wire payload, read
    /// through the vocabulary's derive-generated decoder rather than a params grammar written beside it.
    fn mutation_from_spec(spec: &Json) -> Result<BmpMutation, String> {
        wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }
    //#endregion 🔖️MutationFromSpec

    //#region 🔖️Handlers
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode_bmp(&mutable_input(ctx)?).map_err(|error| format!("decode_bmp failed: {error}"))?;
        let _ = apply_bmp_mutation(&mut snapshot, &mutation_from_spec(&ctx.doc_json()?)?);
        let bytes = encode_bmp(&snapshot).map_err(|error| format!("encode_bmp failed: {error}"))?;
        let projection = project_bmp_mutation(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// ↩️ Applies the forward mutation, then applies EVERY mutation `BmpMutation::inverse` returns
    /// (the vocabulary's own algebraic law, index-aware, computed against the pre-forward `base`)
    /// — the real production undo pipeline, not a hand-derived counter-mutation.
    pub fn undo(ctx: &Context) -> Result<Outcome, String> {
        let base = decode_bmp(&mutable_input(ctx)?).map_err(|error| format!("decode_bmp failed: {error}"))?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        let mut snapshot = base.clone();
        let _ = apply_bmp_mutation(&mut snapshot, &mutation);
        for inverse in mutation_inverse(&mutation, &base).expect("valid retained mutation inverse fixture") {
            let _ = apply_bmp_mutation(&mut snapshot, &inverse);
        }
        let bytes = encode_bmp(&snapshot).map_err(|error| format!("encode_bmp failed: {error}"))?;
        let projection = project_bmp_mutation(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🔁️ `decode_bmp` → `print_dsl` (the DSL hex-dump text codec — BMP has no separate textual
    /// format of its own, see
    /// `../../../🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`) → `parse_dsl`
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
/// 🧭️ Registration entry point the generated host calls. Handlers are registered under the Scenario Outline
/// base ids, which the host resolves for every Examples row, and plain scenarios under their own ids.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    built = built.oracle("mutate", mutate_oracle);
    built = built.oracle("inverse", inverse_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate);
        built = built.subject("inverse", subject::undo);
    }
    built = built.oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
