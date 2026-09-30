//! 🦀️ TIFF 6.0 mutation case — Rust adapter. Every scenario copies the real, committed, genuinely
//! two-page fixture into the case work directory first; the committed document is never written to.
//! `oracle` drives this subset's own independent hand-rolled IFD-chain codec
//! (`../../🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🔮️oracles/🦀️.rs`), `subject` drives this
//! repository's own `decode_tiff`/`apply_tiff_mutation`/`encode_tiff`. Both results are read back by
//! the SAME independent `project_tiff` reader before the `semantic-raster-v1` profile compares them.
//! The subject half is gated behind the generated host's `sut` feature, so the oracle-only run never
//! compiles the local implementation.

use semio_s_plugin_stdio_test_oracle::artifacts::tiff::standards::v6_0::subsets::document::oracle_identity_round_trip;
use semio_repo_test_host::{Adapter, Context, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::tiff::standards::v6_0::subsets::document::{oracle_apply_mutation, oracle_apply_mutation_inverse, project_tiff};
use semio_s_plugin_stdio_test_oracle::law;


//#region 🔖️Input
/// 🧫️ Copies the immutable document the scenario's own `Given` names into the work directory and returns the
/// mutable copy's bytes — the real two-page scan, or for the raster outlines the small document a whole-raster
/// wire payload fits in. Neither is ever written to.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let input = ctx.step_fixture_uris().into_iter().next().ok_or_else(|| format!("scenario {} names no input document", ctx.scenario.id))?;
    let copy = ctx.copy_fixture(&input, Some("input.tiff"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 🧭️ The document as an unchanged round trip through the reference IFD-chain codec leaves it — the baseline the
/// observability and the inverse law are stated against. On the real scan it is the input itself (that writer
/// authored it, and the identity scenario pins the fixpoint); on the small raster document, which another writer
/// authored, it is that document in this writer's normal form, so neither law folds the normalization in.
fn unmutated_baseline(input: &[u8]) -> Result<semio_repo_test_host::Json, String> {
    project_tiff(&oracle_identity_round_trip(input)?)
}

/// 👁️ `@id-mutate`: applies the row's kind with the registered reference implementation and ASSERTS
/// the result is distinguishable from an unchanged round trip's. The exemption list is empty — every
/// kind this vocabulary declares reaches the compared projection — so a kind that stops moving it
/// fails here rather than reporting a green identical to an unchanged round trip's.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let input = mutable_input(ctx)?;
    let before = unmutated_baseline(&input)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_tiff(&bytes)?;
    law::mutation_is_observable(&spec.str("kind"), &projection, &before, &[])?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ "Undoing `<id>` restores the document" is a law checkable WITHOUT a subject, so this handler
/// checks it: apply the row's kind with the reference IFD-chain codec, apply that codec's own
/// independently computed inverse on top, and assert the result projects back onto the pristine
/// original as this writer emits it. Returning the untouched original (what this used to do) asserted
/// nothing at all — the scenario passed whenever the reference codec did not error.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let before = unmutated_baseline(&input)?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = oracle_apply_mutation_inverse(&input, &spec, &mutated)?;
    let projection = project_tiff(&restored)?;
    law::inverse_restores(&spec.str("kind"), &projection, &before)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ The identity round trip, asserted rather than assumed: the reference codec re-parses the real
/// two-page document into its own IFD chain and re-serializes from that alone, and the semantic
/// projection must survive unchanged.
///
/// 🚫️ The "re-encoded bytes must differ from the input" half of the law is NOT assertable on this
/// side and is deliberately not contrived: `shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff` is
/// itself the output of this very `write_tiff`
/// (`../../🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🔮️oracles/🦀️.rs`'s own `derive_real_world_fixture`),
/// so a canonical, deterministic writer reproducing it byte-for-byte is CORRECT, not a pass-through.
/// What is assertable — and asserted — is that this writer is a fixpoint on its own output: any
/// asymmetry between `read_tiff` and `write_tiff` would move the bytes. The pass-through tripwire
/// itself still binds on the SUBJECT side, whose encoder did not write this fixture.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let output = oracle_identity_round_trip(&input)?;
    let before = project_tiff(&input)?;
    let after = project_tiff(&output)?;
    law::round_trip_preserves(&after, &before)?;
    law::carrier_is_exact(&output, &input)?;
    Ok(Outcome::with_raw(output, after))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_tiff::standards::v6_0::subsets::document::io::{decode_tiff, encode_tiff};
    use semio_s_artifact_stdio_tiff::standards::v6_0::subsets::document::schema::mutations::{apply_tiff_mutation, decode_tiff_mutation_payload, inverse_tiff_mutation};
    use semio_s_artifact_stdio_tiff::TiffMutation;
    use semio_s_plugin_stdio_test_oracle::artifacts::tiff::standards::v6_0::subsets::document::project_tiff;

    //#region 🔖️SpecParsing
    /// 🦠️ Decodes the scenario's `{"kind", "params"}` doc string: `params` is the leaf's own wire payload, read
    /// through the vocabulary's derive-generated decoder rather than a params grammar written beside it.
    fn spec_to_mutation(spec: &Json) -> Result<TiffMutation, String> {
        decode_tiff_mutation_payload(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null).to_string())
    }
    //#endregion 🔖️SpecParsing

    //#region 🔖️Handlers
    /// 🚫️ The pass-through tripwire for the mutate rows: a mutated document can never legitimately be the input.
    fn no_byte_pass_through(output: &[u8], input: &[u8]) -> Result<(), String> {
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        Ok(())
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let input = mutable_input(ctx)?;
        let mutation = spec_to_mutation(&spec)?;
        let mut snapshot = decode_tiff(&input).map_err(|error| format!("decode_tiff failed: {error:?}"))?;
        apply_tiff_mutation(&mut snapshot, &mutation);
        let output = encode_tiff(&snapshot).map_err(|error| format!("encode_tiff failed: {error:?}"))?;
        no_byte_pass_through(&output, &input)?;
        let projection = project_tiff(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let input = mutable_input(ctx)?;
        let mutation = spec_to_mutation(&spec)?;
        let base = decode_tiff(&input).map_err(|error| format!("decode_tiff failed: {error:?}"))?;
        let mut snapshot = base.clone();
        apply_tiff_mutation(&mut snapshot, &mutation);
        for inverse in inverse_tiff_mutation(&mutation, &base) {
            apply_tiff_mutation(&mut snapshot, &inverse);
        }
        let output = encode_tiff(&snapshot).map_err(|error| format!("encode_tiff failed: {error:?}"))?;
        let projection = project_tiff(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    /// 🔒️ Decode → re-encode from the typed IFD chain alone, asserting `carrier_is_exact`: the real scan is in the
    /// canonical baseline layout (header, strips, IFD chain) this encoder also emits, and `TiffSnapshot` carries every
    /// tag typed and each IFD's strip bytes as its own raster, so reproducing the scan byte for byte is the writer
    /// being canonical — any dropped tag, reordered IFD or miscounted strip moves the bytes. The mutate rows are what
    /// prove a real parse happened.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_tiff(&input).map_err(|error| format!("decode_tiff failed: {error:?}"))?;
        let output = encode_tiff(&snapshot).map_err(|error| format!("encode_tiff failed: {error:?}"))?;
        semio_s_plugin_stdio_test_oracle::law::carrier_is_exact(&output, &input)?;
        let projection = project_tiff(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    built = built.oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse);
    }
    built = built.oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
