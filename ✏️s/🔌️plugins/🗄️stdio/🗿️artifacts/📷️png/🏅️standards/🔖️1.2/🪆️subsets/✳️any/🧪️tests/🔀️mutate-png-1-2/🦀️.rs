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


//#region 🔖️Input
/// 🧫️ Copies the immutable document the scenario's own `Given` names into the work directory and returns the
/// mutable copy's bytes — the committed 250 KB, 2334x2560, 8-bit COLORMAP architectural floor plan
/// (`rathaus-ahlen-grundriss.png`), or for the raster outlines the small COLORMAP document a whole-raster wire
/// payload fits in. Neither is ever written to.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let input = ctx.step_fixture_uris().into_iter().next().ok_or_else(|| format!("scenario {} names no input document", ctx.scenario.id))?;
    let copy = ctx.copy_fixture(&input, Some("input.png"))?;
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
    use semio_s_artifact_stdio_png::ArtifactDsl;
    use semio_s_artifact_stdio_png::standards::v1_2::subsets::any::io::{decode_png, encode_png};
    use semio_s_artifact_stdio_png::standards::v1_2::subsets::any::schema::mutations::{apply_png_mutation, decode_png_mutation_payload, inverse_png_mutation, PngMutation};
    use semio_s_artifact_stdio_png::standards::v1_2::subsets::any::schema::snapshot::PngSnapshot;

    //#region 🔖️MutationFromSpec
    /// 🦠️ Decodes the scenario's `{"kind", "params"}` doc string: `params` is the leaf's own wire payload, read
    /// through the vocabulary's derive-generated decoder rather than a params grammar written beside it.
    fn mutation_from_spec(spec: &Json) -> Result<PngMutation, String> {
        decode_png_mutation_payload(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null).to_string())
    }
    //#endregion 🔖️MutationFromSpec

    //#region 🔖️Handlers
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let mut snapshot = decode_png(&arranged_input(ctx, &spec)?).map_err(|error| format!("decode_png failed: {error}"))?;
        let _ = apply_png_mutation(&mut snapshot, &mutation_from_spec(&spec)?);
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
        let mutation = mutation_from_spec(&spec)?;
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
