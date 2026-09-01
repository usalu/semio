//! 🦀️ MP3 mpeg1-layer3 mutation case — Rust adapter.
//!
//! Every scenario copies the committed 193,275-byte real stream into the case work
//! directory first; the committed file is never written to. `oracle` drives the registered `id3`
//! 1.17 reference composed with a hand-written ISO/IEC 11172-3 frame walker
//! (`../../🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`), `subject` drives
//! this repository's own decode/apply/encode round trip, and both results are read back through the
//! same independent projection before `semantic-mp3-mpeg1-layer3-v1` compares them. The subject
//! half is gated behind the generated host's `sut` feature so the oracle-only run never compiles
//! the local implementation.
//!
//! The `inverse-<kind>` and `identity-round-trip` handlers ASSERT their laws here rather than
//! deferring them to the parity phase: both laws are checkable by one role alone, and a handler
//! that merely projects and returns passes whenever the reference did not error.

use semio_repo_test_host::{Adapter, Context, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::mp3::standards::v_mpeg1_layer3::subsets::any::{oracle_apply_mutation, oracle_inverse_spec, oracle_round_trip, project_mp3};
use semio_s_plugin_stdio_test_oracle::law::{inverse_restores_within, mutation_is_observable, reparsed_not_copied, round_trip_preserves_within};

//#region 🔖️Kinds
/// 🧾️ Mirrors `Mp3Mutation`'s declared vocabulary
/// (`../../🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`) — kept
/// in sync by the contract phase's `mutation-kind-uncovered`/`mutation-kind-undeclared` gates,
/// which fail loudly if this list and the catalog ever drift apart, and by the oracle module's own
/// `kinds_match_the_catalog_and_the_vocabulary` test.
const KINDS: [&str; 4] = ["set-snapshot", "set-id3v2", "set-frames", "set-id3v1"];
//#endregion 🔖️Kinds

//#region 🔖️Profile
/// 📏️ `semantic-mp3-mpeg1-layer3-v1`'s own declared writer freedom
/// (`../../🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧪️oracle/🔣️.json`), mirrored here so
/// an in-handler law check is exactly as strict as the profile the case is measured by, never
/// stricter. Every projected value is an exact integer, a boolean or a string, so the tolerance is
/// genuinely zero rather than a nominal one.
const MP3_WRITER_FREEDOM: &[&str] = &["flags", "tagSize", "paddingLength", "fileSize"];
const MP3_TOLERANCE: f64 = 0.0;
//#endregion 🔖️Profile

//#region 🔖️Input
const INPUT: &str = "shared://🎵️bauen-mit-bestand-ausschnitt.mp3";

/// 🧫️ Copies the immutable committed asset into the work directory and returns the copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.mp3"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 🔮️ The forward reference answer. Correct by design that it asserts nothing beyond the
/// reference's own success: this handler PRODUCES the reference result, and the comparison against
/// the subject is the parity phase's job.
/// 👁️ `@id-mutate`: applies the row's kind with the registered reference implementation and ASSERTS
/// the result is distinguishable from the untouched fixture. The exemption list is empty — every
/// kind this vocabulary declares reaches the compared projection — so a kind that stops moving it
/// fails here rather than reporting a green identical to `no-mutation`'s.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let before = project_mp3(&input)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_mp3(&bytes)?;
    mutation_is_observable(&spec.str("kind"), &projection, &before, &[])?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ Applies `<id>` forward, then its independently computed inverse — both derived from the SAME
/// untouched input, matching `Mp3Mutation::inverse()`'s own base-relative semantics — and ASSERTS
/// the law in role: the restored stream's projection must equal the real original's own. Without
/// the check the scenario would pass for any inverse `id3` merely tolerated.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let inverse_spec = oracle_inverse_spec(&input, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &inverse_spec)?;
    let projection = project_mp3(&restored)?;
    inverse_restores_within(&spec.str("kind"), &projection, &project_mp3(&input)?, MP3_WRITER_FREEDOM, MP3_TOLERANCE)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ The identity law, both halves asserted in role. `id3`'s writer chooses its own ID3v2 padding
/// and re-derives the tag region wholesale (57 committed bytes come back as 86), so bit-identical
/// output could only mean the bytes were copied rather than parsed — the reference is bound by
/// [`reparsed_not_copied`]. The SUBJECT is bound by the opposite law and asserts it below.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_round_trip(&input)?;
    reparsed_not_copied(&bytes, &input)?;
    let projection = project_mp3(&bytes)?;
    round_trip_preserves_within(&projection, &project_mp3(&input)?, MP3_WRITER_FREEDOM, MP3_TOLERANCE)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{mutable_input, MP3_TOLERANCE, MP3_WRITER_FREEDOM};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::mp3::standards::mpeg1_layer3::subsets::any::io::{decode_mp3, encode_mp3};
    use semio_s_plugin_stdio::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::mutations::{apply_mp3_mutation, set_frames, set_id3v1, set_id3v2, set_snapshot, Mp3Mutation};
    use semio_s_plugin_stdio::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::{Id3Frame, Id3v1Tag, Id3v2Tag, Mp3Snapshot};
    use semio_s_plugin_stdio_test_oracle::artifacts::mp3::standards::v_mpeg1_layer3::subsets::any::project_mp3;
    use semio_s_plugin_stdio_test_oracle::law::{carrier_is_exact, inverse_restores_within, round_trip_preserves_within};

    //#region 🔖️SpecReaders
    fn params_of(spec: &Json) -> Json {
        spec.get("params").cloned().unwrap_or(Json::Null)
    }

    /// 🏷️ Encodes one ID3v2.3 text frame's data field: an encoding byte (`0x00` = ISO-8859-1)
    /// followed by the text. This is the ID3v2.3 text-frame body the spec defines; the subject's
    /// `Id3Frame` retains it as opaque `data`.
    fn text_frame(id: &str, text: &str) -> Result<Id3Frame, String> {
        let mut data = vec![0u8];
        for ch in text.chars() {
            if (ch as u32) >= 0x100 {
                return Err(format!("text frame {id:?} carries {ch:?}, which is outside ISO-8859-1"));
            }
            data.push(ch as u8);
        }
        Ok(Id3Frame { id: id.to_string(), flags: 0, data })
    }

    fn tag_of(params: &Json) -> Result<Option<Id3v2Tag>, String> {
        match params.get("text") {
            Some(Json::Array(items)) if !items.is_empty() => {
                let frames = items.iter().map(|item| text_frame(&item.str("id"), &item.str("text"))).collect::<Result<Vec<Id3Frame>, String>>()?;
                Ok(Some(Id3v2Tag { major_version: 3, minor_version: 0, flags: 0, frames }))
            }
            _ => Ok(None),
        }
    }

    /// 🏷️ Builds the 128-byte ID3v1 trailer this subset retains verbatim, from the same fields the
    /// oracle writes — fixed offsets, fixed widths, zero-padded ISO-8859-1, per the ID3v1 layout.
    fn v1_of(params: &Json) -> Result<Option<Id3v1Tag>, String> {
        let Some(fields) = params.get("v1") else { return Ok(None) };
        if matches!(fields, Json::Null) {
            return Ok(None);
        }
        let mut raw = vec![0u8; 128];
        raw[0..3].copy_from_slice(b"TAG");
        for (key, start, width) in [("title", 3usize, 30usize), ("artist", 33, 30), ("album", 63, 30), ("year", 93, 4), ("comment", 97, 30)] {
            let value = fields.str(key);
            let bytes: Vec<u8> = value.chars().map(|ch| if (ch as u32) < 0x100 { Ok(ch as u8) } else { Err(format!("ID3v1 field {key} carries {ch:?}, which is outside ISO-8859-1")) }).collect::<Result<Vec<u8>, String>>()?;
            if bytes.len() > width {
                return Err(format!("ID3v1 field {key} is {} byte(s), past its {width}-byte slot", bytes.len()));
            }
            raw[start..start + bytes.len()].copy_from_slice(&bytes);
        }
        raw[127] = match fields.get("genreId") {
            Some(Json::Number(value)) => *value as u8,
            _ => 0,
        };
        Ok(Some(Id3v1Tag { raw }))
    }

    fn take_of(params: &Json, base: &Mp3Snapshot, kind: &str) -> Result<Vec<semio_s_plugin_stdio::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::Mp3Frame>, String> {
        match params.get("take") {
            Some(Json::Number(count)) => {
                let keep = *count as usize;
                if keep > base.frames.len() {
                    return Err(format!("{kind}: `take` is {keep} but the document carries only {} MPEG frame(s)", base.frames.len()));
                }
                Ok(base.frames[..keep].to_vec())
            }
            _ => Err(format!("{kind}: params carry no `take`")),
        }
    }
    //#endregion 🔖️SpecReaders

    //#region 🔖️Mutation
    /// 🧭️ Builds the real `Mp3Mutation` a spec describes.
    fn mutation_of(spec: &Json, base: &Mp3Snapshot) -> Result<Mp3Mutation, String> {
        let params = params_of(spec);
        let kind = spec.str("kind");
        Ok(match kind.as_str() {
            "set-id3v2" => Mp3Mutation::SetId3v2(set_id3v2::SetId3v2 { id3v2: tag_of(&params)? }),
            "set-frames" => Mp3Mutation::SetFrames(set_frames::SetFrames { frames: take_of(&params, base, &kind)? }),
            "set-id3v1" => Mp3Mutation::SetId3v1(set_id3v1::SetId3v1 { id3v1: v1_of(&params)? }),
            "set-snapshot" => Mp3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: Mp3Snapshot { schema: base.schema.clone(), id3v2: tag_of(&params)?, frames: take_of(&params, base, &kind)?, id3v1: v1_of(&params)? } }),
            other => return Err(format!("mutation kind {other:?} is not implemented by the subject")),
        })
    }

    /// ↩️ Mirrors `Mp3Mutation::inverse()`
    /// (`../../🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`)
    /// independently — the generated oracle-role host never links `protocol`, so the trait method
    /// itself is unreachable here. Every variant of this vocabulary is a whole-layer replace, so
    /// its inverse is the same verb carrying the layer `base` already had.
    fn inverse_of(spec: &Json, base: &Mp3Snapshot) -> Result<Mp3Mutation, String> {
        Ok(match spec.str("kind").as_str() {
            "set-id3v2" => Mp3Mutation::SetId3v2(set_id3v2::SetId3v2 { id3v2: base.id3v2.clone() }),
            "set-frames" => Mp3Mutation::SetFrames(set_frames::SetFrames { frames: base.frames.clone() }),
            "set-id3v1" => Mp3Mutation::SetId3v1(set_id3v1::SetId3v1 { id3v1: base.id3v1.clone() }),
            "set-snapshot" => Mp3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
            other => return Err(format!("mutation kind {other:?} is not implemented by the subject")),
        })
    }
    //#endregion 🔖️Mutation

    //#region 🔖️Handlers
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let mut snapshot = decode_mp3(&input).map_err(|error| format!("decode_mp3 failed: {error}"))?;
        let spec = ctx.doc_json()?;
        let mutation = mutation_of(&spec, &snapshot)?;
        apply_mp3_mutation(&mut snapshot, &mutation);
        let bytes = encode_mp3(&snapshot);
        let projection = project_mp3(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let base = decode_mp3(&input).map_err(|error| format!("decode_mp3 failed: {error}"))?;
        let spec = ctx.doc_json()?;
        let forward = mutation_of(&spec, &base)?;
        let backward = inverse_of(&spec, &base)?;
        let mut snapshot = base;
        apply_mp3_mutation(&mut snapshot, &forward);
        apply_mp3_mutation(&mut snapshot, &backward);
        let bytes = encode_mp3(&snapshot);
        let projection = project_mp3(&bytes)?;
        inverse_restores_within(&spec.str("kind"), &projection, &project_mp3(&input)?, MP3_WRITER_FREEDOM, MP3_TOLERANCE)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🔁️ The identity law for the SUBJECT's own encoder, which is bound by the opposite byte law
    /// to the reference's: `encode_mp3` re-emits each frame's retained payload verbatim and
    /// recomputes the ID3v2 sizes from the frame data, and this fixture's tag is already canonical
    /// under that rule (169-byte body = TSSE's 10+47 plus TIT2's 10+63 plus TPE1's 10+13 plus
    /// TLEN's 10+6, no trailing padding — LAME wrote it tight), so its own
    /// `codec_retention_law` says the output reproduces the input exactly. Demanding a byte
    /// DIFFERENCE here would be a fabricated law; [`carrier_is_exact`] is the one that actually
    /// binds, and it still fails loudly the moment either half drifts.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_mp3(&input).map_err(|error| format!("decode_mp3 failed: {error}"))?;
        let bytes = encode_mp3(&snapshot);
        carrier_is_exact(&bytes, &input)?;
        let projection = project_mp3(&bytes)?;
        round_trip_preserves_within(&projection, &project_mp3(&input)?, MP3_WRITER_FREEDOM, MP3_TOLERANCE)?;
        Ok(Outcome::with_raw(bytes, projection))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle).oracle(&format!("inverse-{kind}"), inverse_oracle);
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
        }
    }
    built = built.oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
