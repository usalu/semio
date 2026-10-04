//! 🦀️ Exhaustive GIF 89a mutation case — Rust adapter. Every one of the 21 declared `GifMutation`
//! kinds, applied to a real 4.4 MB / 800x800 / 54-frame animated GIF (`💃️dancing/🖼️assets/🧪️dancing/🖼️.gif`,
//! committed under the 87a subset's own example directory and read here via `asset://`). The oracle
//! drives the registered `gif` reference implementation; the subject fully parses the artifact into
//! its typed snapshot and re-serializes from it alone — never splicing source bytes. The subject half
//! is gated behind the generated host's `sut` feature so the oracle-only run never compiles the local
//! implementation — §5.3 of the frozen plan, not a workaround for a broken crate. `semio-s-plugin-stdio`
//! builds; the subject and parity phases both run.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_artifact_stdio_gif_test_oracle::standards::v89a::subsets::base::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_arrange, oracle_identity_round_trip, project};
use semio_repo_test_host::law;


//#region 🔖️Input
/// 🧫️ Copies the immutable document the scenario's own `Given` names — the real animation, or for a raster
/// outline the small animation a whole index buffer fits in — into the work directory and returns the mutable
/// copy's bytes. The committed document itself is never written to.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let input = ctx.step_fixture_uris().into_iter().next().ok_or_else(|| format!("scenario {} names no input document", ctx.scenario.id))?;
    let copy = ctx.copy_fixture(&input, Some("input.gif"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}

/// 🎬️ The document a kind actually acts on. The real animation carries a genuine comment extension
/// and a genuine NETSCAPE2.0 loop extension and nothing else, and the NETSCAPE one is the loop-count
/// axis rather than an `appExtensions` entry — so `remove-app-extension` is handed the real document
/// with its target inserted first, by the reference implementation. Every other kind gets the
/// committed bytes untouched.
fn arranged_input(ctx: &Context, spec: &Json) -> Result<Vec<u8>, String> {
    oracle_arrange(&mutable_input(ctx)?, spec)
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 🔮️ `@id-mutate`: applies the row's kind, projects the result, and ASSERTS it is distinguishable
/// from the untouched animation. One handler serves all 21 scenario ids — the kind and its params
/// come from the scenario's own doc string. The exemption list is empty: every declared kind of this
/// vocabulary reaches the projection, including `set-frame-interlace`, whose flag the projection now
/// reads off the Image Descriptor rather than from `Frame::interlaced` (which the reference decoder
/// resets to `false` on every read, making the kind invisible).
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let input = arranged_input(ctx, &spec)?;
    let before = project(&input)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project(&bytes)?;
    law::mutation_is_observable(&spec.str("kind"), &projection, &before, &[])?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ `@id-inverse`: applies the row's kind, then its computed inverse, and asserts the semantic
/// projection is fully recovered — the algebraic law every mutation kind must satisfy.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let input = arranged_input(ctx, &spec)?;
    let original_projection = project(&input)?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = oracle_apply_mutation_inverse(&input, &spec, &mutated)?;
    let restored_projection = project(&restored)?;
    law::inverse_restores(&spec.str("kind"), &restored_projection, &original_projection)?;
    Ok(Outcome::with_raw(restored, restored_projection))
}

/// 🔁️ `@id-identity-round-trip`: the no-byte-pass-through tripwire. Decoding and re-encoding through
/// the reference codec alone must change the bytes (a different LZW writer, different block layout)
/// while leaving the semantic projection unchanged.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let output = oracle_identity_round_trip(&input)?;
    law::reparsed_not_copied(&output, &input)?;
    let before = project(&input)?;
    let after = project(&output)?;
    law::round_trip_preserves(&after, &before)?;
    Ok(Outcome::with_raw(output, after))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{arranged_input, mutable_input};
    use semio_repo_test_host::{Adapter, Context, Json, Outcome};
    use semio_s_artifact_stdio_gif_test_oracle::standards::v89a::subsets::base::project;
    use semio_s_artifact_stdio_gif::standards::v89a::subsets::any::io::{decode_gif, encode_gif};
    use semio_repo_test_host::law::wire_operation;
    use semio_s_artifact_stdio_gif::{mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_s_artifact_stdio_gif::standards::v89a::subsets::any::schema::mutations::{apply_gif_mutation, GifMutation};
    use semio_s_artifact_stdio_gif::standards::v89a::subsets::any::schema::snapshot::GifSnapshot;
    use semio_s_artifact_stdio_gif::ArtifactDsl;

    //#region 🔖️SpecToMutation
    /// 🦠️ Decodes the scenario's `{"kind", "params"}` doc string: `params` is the leaf's own wire payload, read
    /// through the vocabulary's derive-generated decoder rather than a params grammar written beside it.
    fn mutation_from_spec(spec: &Json) -> Result<GifMutation, String> {
        wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }
    //#endregion 🔖️SpecToMutation

    //#region 🔖️Codec
    /// 🚫️ The no-byte-pass-through channel: complete semantic parse, the subset's own text codec out
    /// and back in, then re-serialize from the model alone — never the source bytes.
    fn decode_through_text(input: &[u8]) -> Result<GifSnapshot, String> {
        let snapshot = decode_gif(input)?;
        let text = ArtifactDsl::print_dsl(&snapshot);
        <GifSnapshot as ArtifactDsl>::parse_dsl(&text).map_err(|error| format!("parse_dsl failed: {error}"))
    }
    //#endregion 🔖️Codec

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let mut snapshot = decode_through_text(&arranged_input(ctx, &spec)?)?;
        let mutation = mutation_from_spec(&spec)?;
        apply_gif_mutation(&mut snapshot, &mutation);
        let bytes = encode_gif(&snapshot).map_err(|error| format!("encode_gif failed: {error}"))?;
        let projection = project(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let original = decode_through_text(&arranged_input(ctx, &spec)?)?;
        let original_projection = project(&encode_gif(&original).map_err(|error| format!("encode_gif failed: {error}"))?)?;
        let mutation = mutation_from_spec(&spec)?;
        let mut mutated = original.clone();
        apply_gif_mutation(&mut mutated, &mutation);
        for inverse in mutation_inverse(&mutation, &original).expect("valid retained mutation inverse fixture") {
            apply_gif_mutation(&mut mutated, &inverse);
        }
        let bytes = encode_gif(&mutated).map_err(|error| format!("encode_gif failed: {error}"))?;
        let restored_projection = project(&bytes)?;
        if restored_projection != original_projection {
            return Err(format!("inverse of {:?} did not recover the original semantic projection", spec.str("kind")));
        }
        Ok(Outcome::with_raw(bytes, restored_projection))
    }

    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_through_text(&input)?;
        let output = encode_gif(&snapshot).map_err(|error| format!("encode_gif failed: {error}"))?;
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let before = project(&input)?;
        let after = project(&output)?;
        if before != after {
            return Err("decode/re-encode round trip changed the semantic projection".to_string());
        }
        Ok(Outcome::with_raw(output, after))
    }

    /// 🧭️ Registers all 21 kinds' `mutate`/`inverse` scenario ids plus the round trip, mirroring
    /// `super::adapter`'s oracle registration.
    pub fn register(mut built: Adapter) -> Adapter {
        built = built.subject("mutate", mutate);
        built = built.subject("inverse", inverse);
        built.subject("identity-round-trip", identity_round_trip)
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    built = built.oracle("mutate", mutate_oracle);
    built = built.oracle("inverse", inverse_oracle);
    built = built.oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = subject::register(built);
    }
    built
}
//#endregion 🔖️Registration
