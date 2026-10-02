//! 🦀️ JFIF 1.01 mutation case — Rust adapter. Every scenario copies the immutable real-world
//! fixture into the case work directory first; the committed fixture is never written to. `oracle`
//! drives the registered `image` reference implementation, `subject` drives this subset's own
//! from-scratch decode/mutate/encode round trip, and both results are read back by the SAME
//! independent reader before the `semantic-jpg-mutate-v1` profile compares them.
//!
//! The projection is this subset's own `project_jpg_mutation`, not the shared
//! `raster::project_image`: that one reports geometry and a luma histogram, so the JFIF header
//! fields and the APPn/COM segments — three of the twelve declared kinds — fell outside the
//! compared surface entirely and could not move it. Its raster half is unchanged and still coarse,
//! because JPEG is lossy and this platform's tolerance is per-number and absolute with no aggregate
//! mode; its metadata half is exact, because a marker segment survives a re-encode byte for byte or
//! it is wrong.
//!
//! The subject half is gated behind the generated host's `sut` feature so the oracle-only run never
//! compiles the local implementation.

use semio_repo_test_host::{Adapter, Context, Outcome};
use semio_s_artifact_stdio_jpg_test_oracle::standards::v_jfif_1_01::subsets::document::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_identity_round_trip, project_jpg_mutation};
use semio_repo_test_host::law;


//#region 🔖️Input
/// 🧫️ Copies the immutable document the scenario's own `Given` names into the work directory and returns the
/// mutable copy's bytes — the real scan, or for the raster outlines the small document a whole-raster wire payload
/// fits in.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let input = ctx.step_fixture_uris().into_iter().next().ok_or_else(|| format!("scenario {} names no input document", ctx.scenario.id))?;
    let copy = ctx.copy_fixture(&input, Some("input.jpg"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Lossy
/// 📏️ The absolute per-number slack `semantic-jpg-mutate-v1` itself declares
/// (`../../🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🔮️oracles/🔣️.json`), mirrored here so an
/// in-handler law is exactly as strict as the profile the case is measured by and never stricter.
/// It exists because JPEG is lossy and every step of these round trips re-quantizes: measured on
/// this fixture (2275x2560 = 5 824 000 pixels), one reference decode → re-encode at quality 90
/// moves 413 pixels out of luma bucket 0 and 382 out of bucket 7; the inverse round trip's second
/// re-encode raises that to 805; and `change-re-encode-quality`, which passes through quality 50,
/// moves 8841 out of bucket 7. All of those are drift a lossy codec is entitled to. The profile's
/// own description states what the slack is sized to still catch — wrong geometry, blank/inverted/
/// solid output, wildly shifted tonal balance — and a mutation this case's `replace-pixels`/
/// `replace-pixels` rows perform lands ~5.6 MILLION pixels in the wrong bucket, three orders of
/// magnitude past it, so the law below stays substantive rather than being excused by the slack.
const JPG_TOLERANCE: f64 = 400_000.0;

/// 🚫️ The five kinds this codec pair provably cannot make byte-observable, each because
/// `../../🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/🦀️.rs` regenerates fresh Annex K
/// DQT/DHT tables scaled by `re_encode_quality` on every encode and never emits a DRI marker at
/// all. They mutate the typed snapshot and nothing else, which the feature description states and
/// the subset's own oracle module documents against the encoder's source. Everything NOT on this
/// list — including `change-jfif-header`, `insert-other-segment` and `remove-other-segment`, which are
/// written to real bytes — must move the projection, and the law below fails the scenario if it
/// does not.
const UNOBSERVABLE: &[&str] = &["replace-quant-table", "remove-quant-table", "replace-huffman-table", "remove-huffman-table", "change-restart-interval"];

/// 🔲️ The whole-raster kinds, whose rows run on a 32x24 document: its 768 pixels are fewer than [`JPG_TOLERANCE`],
/// which is sized for the scan and would excuse ANY change there, so their observability claim is exact — the
/// replacement must move the histogram at all. The inverse law and the profile comparison keep the profile's slack.
const RASTER_KINDS: &[&str] = &["replace-pixels"];
//#endregion 🔖️Lossy

//#region 🔖️Oracle
/// 🧭️ The document as an unchanged round trip leaves it — one full decode and re-encode by the reference
/// codec, and the baseline both the observability and the inverse law are stated against.
///
/// It is deliberately NOT the committed bytes. JPEG is lossy and both encoders regenerate their
/// quantization tables from `re_encode_quality` rather than preserving the scanner's, so a single
/// decode/re-encode already moves the raster and replaces the DQT. Measuring "did this mutation
/// change anything" or "did the inverse restore it" against the untouched scan would fold that
/// unavoidable normalization into every scenario — making every kind look observable and every
/// inverse look broken, both for the same reason and neither about the mutation.
fn unmutated_baseline(original: &[u8]) -> Result<semio_repo_test_host::Json, String> {
    project_jpg_mutation(&oracle_identity_round_trip(original)?)
}

/// 🔮️ Applies the scenario's declared mutation with the reference `image` codec and ASSERTS the
/// result is distinguishable from that same codec's own untouched output, under the slack the case
/// is measured by. Without that assertion a kind whose effect lands outside the projection passes
/// exactly as an unchanged round trip does — which is what all eight non-raster kinds did while this case
/// compared only geometry and a luma histogram.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let original = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let slack = if RASTER_KINDS.contains(&kind.as_str()) { 0.0 } else { JPG_TOLERANCE };
    let before = unmutated_baseline(&original)?;
    let bytes = oracle_apply_mutation(&original, &spec)?;
    let projection = project_jpg_mutation(&bytes)?;
    law::mutation_is_observable_within(&kind, &projection, &before, UNOBSERVABLE, &[], slack)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ The inverse law, asserted: apply the row's kind with the reference `image` codec, apply that
/// reference's OWN computed inverse on top of the real forward result, and require the outcome to
/// project back onto the pristine original within `semantic-jpg-mutate-v1`'s own declared slack.
/// The previous version applied the forward mutation, discarded it, and returned a plain
/// decode → re-encode of the untouched original — it asserted nothing, and passed whenever `image`
/// did not error.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let original = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let before = unmutated_baseline(&original)?;
    let forward = oracle_apply_mutation(&original, &spec)?;
    let restored = oracle_apply_mutation_inverse(&original, &spec, &forward)?;
    let projection = project_jpg_mutation(&restored)?;
    law::inverse_restores_within(&spec.str("kind"), &projection, &before, &[], JPG_TOLERANCE)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ The identity round trip, asserted rather than assumed: a decode → re-encode with the
/// reference `image` codec must move the bytes — a second encoder's Annex K tables and entropy
/// coding are not this scan's — and must leave the semantic projection where it was, within
/// `semantic-jpg-mutate-v1`'s own declared slack for lossy re-quantization.
///
/// `quantTables` is the one member excluded, and it is the one member a round trip provably cannot
/// preserve: BOTH codecs regenerate the DQT from `re_encode_quality` over the Annex K.1 base
/// tables rather than carrying the source's forward, so the committed scan's own tables (written by
/// whatever produced it) are gone by construction. Excluding it here is the same writer-freedom
/// statement the histogram's slack makes, named rather than absorbed — and the member still carries
/// its full weight in the observability law, where it is what makes `change-re-encode-quality` visible
/// at all.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let original = mutable_input(ctx)?;
    let bytes = oracle_identity_round_trip(&original)?;
    law::reparsed_not_copied(&bytes, &original)?;
    let before = project_jpg_mutation(&original)?;
    let projection = project_jpg_mutation(&bytes)?;
    law::round_trip_preserves_within(&projection, &before, &["quantTables"], JPG_TOLERANCE)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_jpg::schema::mutations::{apply_jpg_mutation, JpgMutation};
    use semio_repo_test_host::law::wire_operation;
    use semio_s_artifact_stdio_jpg::{mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_s_artifact_stdio_jpg::io::{decode_jpg, encode_jpg};
    use semio_s_artifact_stdio_jpg_test_oracle::standards::v_jfif_1_01::subsets::document::project_jpg_mutation;

    //#region 🔖️MutationFromSpec
    /// 🦠️ Decodes the scenario's `{"kind", "params"}` doc string: `params` is the leaf's own wire payload, read
    /// through the vocabulary's derive-generated decoder rather than a params grammar written beside it.
    fn mutation_from_spec(spec: &Json) -> Result<JpgMutation, String> {
        wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }
    //#endregion 🔖️MutationFromSpec

    //#region 🔖️Handlers
    /// ▶️ Full parse → typed mutate → re-serialize, never a splice of the input bytes.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let bytes = mutable_input(ctx)?;
        let mut snapshot = decode_jpg(&bytes).map_err(|error| format!("decode_jpg failed: {error:?}"))?;
        apply_jpg_mutation(&mut snapshot, &mutation_from_spec(&ctx.doc_json()?)?);
        let output = encode_jpg(&snapshot).map_err(|error| format!("encode_jpg failed: {error:?}"))?;
        if output == bytes {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_jpg_mutation(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    /// ↩️ Applies the forward mutation, then applies EVERY mutation `JpgMutation::inverse` returns
    /// (the vocabulary's own algebraic law, computed against the pre-forward `base`) on top of that
    /// real forward result. The previous version applied the mutation, threw the result away and
    /// re-encoded a fresh parse of the pristine original — which restores the document by
    /// construction and so asserted nothing about the inverse at all.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let bytes = mutable_input(ctx)?;
        let base = decode_jpg(&bytes).map_err(|error| format!("decode_jpg failed: {error:?}"))?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        let mut snapshot = base.clone();
        apply_jpg_mutation(&mut snapshot, &mutation);
        for undo in mutation_inverse(&mutation, &base) {
            apply_jpg_mutation(&mut snapshot, &undo);
        }
        let output = encode_jpg(&snapshot).map_err(|error| format!("encode_jpg (restore) failed: {error:?}"))?;
        if output == bytes {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_jpg_mutation(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    /// 🔁️ Decode → re-encode with this subset's own codec, no mutation at all.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let bytes = mutable_input(ctx)?;
        let snapshot = decode_jpg(&bytes).map_err(|error| format!("decode_jpg failed: {error:?}"))?;
        let output = encode_jpg(&snapshot).map_err(|error| format!("encode_jpg failed: {error:?}"))?;
        if output == bytes {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_jpg_mutation(&output)?;
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
    built = built.oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse);
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
