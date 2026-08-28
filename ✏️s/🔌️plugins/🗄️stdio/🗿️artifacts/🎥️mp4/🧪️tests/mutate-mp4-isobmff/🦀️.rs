//! 🦀️ MP4 ISO-BMFF exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-
//! REFACTOR wave 7.
//!
//! Every scenario copies the immutable real 1.5s H.264 excerpt into the case work directory first;
//! the committed fixture is never written to. `oracle` drives the registered `mp4` 0.14 reference
//! implementation (`../../🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`'s own
//! `oracle_apply_mutation`); `subject` drives this repository's own `decode_mp4`/`encode_mp4`/
//! `apply_mp4_mutation` over the full 10-kind `Mp4Mutation` vocabulary. Both results are read back by
//! the SAME independent `project_mp4_mutation` (`mp4`) before the `semantic-mp4-mutate-v1` profile
//! compares them. The subject half is gated behind the generated host's `sut` feature so the
//! oracle-only run never compiles the local implementation.

use semio_repo_test_host::{Adapter, Context, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::mp4::standards::v_isobmff::subsets::any::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_identity_round_trip, project_mp4_mutation};
use semio_s_plugin_stdio_test_oracle::law;

//#region 🔖️Kinds
/// 📇️ The catalog's own kinds (`../../🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧪️oracle/🔣️.json`),
/// duplicated as a plain constant rather than reached through the subject crate — this loop drives
/// oracle registration too, which must build and run with the subject crate absent entirely.
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-ftyp", "insert-track", "remove-track", "set-track-dimensions", "set-track-codec", "insert-sample", "remove-sample", "set-sample-sync"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://🎥️bauen-mit-bestand-ausschnitt.mp4";

/// 🧫️ Copies the immutable fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.mp4"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 🔮️ Applies the declared mutation with `mp4` and projects the result independently.
/// 👁️ `@id-mutate`: applies the row's kind with the registered reference implementation and ASSERTS
/// the result is distinguishable from the untouched fixture. The exemption list is empty — every
/// kind this vocabulary declares reaches the compared projection — so a kind that stops moving it
/// fails here rather than reporting a green identical to `no-mutation`'s.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let input = mutable_input(ctx)?;
    let before = project_mp4_mutation(&input)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_mp4_mutation(&bytes)?;
    law::mutation_is_observable(&spec.str("kind"), &projection, &before, &[])?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ The inverse law, asserted rather than assumed: `mp4` applies the row's kind, then its own
/// independently computed inverse on top of that result, and the re-muxed movie must project back
/// onto the pristine original. Returning the untouched original (what this used to do) asserted
/// nothing — the scenario passed whenever `mp4` merely parsed the fixture.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let before = project_mp4_mutation(&input)?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = oracle_apply_mutation_inverse(&input, &spec, &mutated)?;
    let projection = project_mp4_mutation(&restored)?;
    law::inverse_restores(&spec.str("kind"), &projection, &before)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ The no-byte-pass-through law on the ORACLE side: `Mp4Reader` parses the real movie into
/// tracks and samples and `Mp4Writer` re-muxes a fresh file from that model alone, so the bytes must
/// move (a second muxer's box order and `mdat` layout are not this fixture's) while the semantic
/// projection must not.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let output = oracle_identity_round_trip(&input)?;
    law::reparsed_not_copied(&output, &input)?;
    let before = project_mp4_mutation(&input)?;
    let after = project_mp4_mutation(&output)?;
    law::round_trip_preserves(&after, &before)?;
    Ok(Outcome::with_raw(output, after))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::mp4::standards::isobmff::subsets::any::io::{decode_mp4, encode_mp4};
    use semio_s_plugin_stdio::artifacts::mp4::standards::isobmff::subsets::any::schema::mutations::{apply_mp4_mutation, Mp4Mutation};
    use semio_s_plugin_stdio::artifacts::mp4::standards::isobmff::subsets::any::schema::snapshot::{Mp4Codec, Mp4Ftyp, Mp4Sample, Mp4Snapshot, Mp4Track};
    use semio_s_plugin_stdio_test_oracle::artifacts::mp4::standards::v_isobmff::subsets::any::project_mp4_mutation;
    use semio_s_plugin_stdio_test_oracle::law;

    //#region 🔖️SpecReading
    /// 🔎️ A second, independently written reading of the SAME `params` JSON schema the oracle reads
    /// in `../../../🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs` — deliberately not
    /// shared code, so a bug in one reading has nothing to hide behind in the other.
    fn number(value: &Json, key: &str, fallback: f64) -> f64 {
        match value.get(key) {
            Some(Json::Number(found)) => *found,
            _ => fallback,
        }
    }
    fn usize_field(value: &Json, key: &str) -> usize {
        number(value, key, 0.0).max(0.0) as usize
    }
    fn boolean(value: &Json, key: &str, fallback: bool) -> bool {
        match value.get(key) {
            Some(Json::Bool(found)) => *found,
            _ => fallback,
        }
    }
    fn strings(value: &Json, key: &str) -> Vec<String> {
        match value.get(key) {
            Some(Json::Array(items)) => items.iter().filter_map(|item| if let Json::String(text) = item { Some(text.clone()) } else { None }).collect(),
            _ => Vec::new(),
        }
    }
    fn bytes(value: &Json, key: &str) -> Vec<u8> {
        match value.get(key) {
            Some(Json::Array(items)) => items.iter().filter_map(|item| if let Json::Number(n) = item { Some(*n as u8) } else { None }).collect(),
            _ => Vec::new(),
        }
    }

    fn ftyp_from_json(value: &Json, fallback: &Mp4Ftyp) -> Mp4Ftyp {
        let major_brand = value.str("majorBrand");
        let compatible_brands = strings(value, "compatibleBrands");
        Mp4Ftyp {
            major_brand: if major_brand.is_empty() { fallback.major_brand.clone() } else { major_brand },
            minor_version: number(value, "minorVersion", fallback.minor_version as f64) as u32,
            compatible_brands: if compatible_brands.is_empty() { fallback.compatible_brands.clone() } else { compatible_brands },
        }
    }

    fn sample_from_json(value: &Json) -> Mp4Sample {
        Mp4Sample { data: bytes(value, "data"), duration: number(value, "duration", 0.0) as u32, cts_offset: number(value, "ctsOffset", 0.0) as i32, sync: boolean(value, "sync", true) }
    }

    /// 🧮️ Re-groups a retained `stsc` chunking onto a shortened sample list, so the document
    /// `set-snapshot` hands over is internally consistent rather than carrying a grouping that
    /// claims more samples than the track holds. The encoder reconciles a stale grouping on its own
    /// (`../../🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/🦀️component.rs`'s
    /// `normalized_chunk_sample_counts`, pinned by its own test), but a case that means "replace the
    /// document with THIS one" must hand over a document a real producer could have written.
    fn grouping_for(retained: &[u32], samples: usize) -> Vec<u32> {
        let mut remaining = samples;
        let mut counts = Vec::new();
        for count in retained {
            let taken = (*count as usize).min(remaining);
            remaining -= taken;
            if taken > 0 {
                counts.push(taken as u32);
            }
        }
        if remaining > 0 {
            counts.push(remaining as u32);
        }
        counts
    }

    /// 🦠️ Builds the real `Mp4Mutation` this scenario's `{"kind", "params"}` doc string describes.
    /// `set-snapshot` mirrors the oracle's own reading (replace `ftyp`, drop the first track's last
    /// sample) — a real multi-facet whole-document replace rather than a `SetFtyp` alias.
    /// `insert-track` duplicates the real track 0 (the fixture's only track — it carries no audio),
    /// mirroring the oracle's own bound on what a real second track can be here.
    fn mutation_from_spec(spec: &Json, base: &Mp4Snapshot) -> Result<Mp4Mutation, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Object(Vec::new()));
        match spec.str("kind").as_str() {
            "no-mutation" => Ok(Mp4Mutation::NoMutation),
            "set-snapshot" => {
                let mut snapshot = base.clone();
                snapshot.ftyp = ftyp_from_json(&params.get("ftyp").cloned().unwrap_or(Json::Object(Vec::new())), &base.ftyp);
                if let Some(track) = snapshot.tracks.first_mut() {
                    track.samples.pop();
                    track.chunk_sample_counts = grouping_for(&track.chunk_sample_counts, track.samples.len());
                }
                Ok(Mp4Mutation::SetSnapshot { snapshot })
            }
            "set-ftyp" => Ok(Mp4Mutation::SetFtyp { ftyp: ftyp_from_json(&params, &base.ftyp) }),
            "insert-track" => {
                let source = base.tracks.first().ok_or("mp4: no track to duplicate for insert-track")?;
                let track_id = base.tracks.iter().map(|track| track.track_id).max().unwrap_or(0) + 1;
                Ok(Mp4Mutation::InsertTrack { index: usize_field(&params, "index"), track: Mp4Track { track_id, ..source.clone() } })
            }
            "remove-track" => Ok(Mp4Mutation::RemoveTrack { index: usize_field(&params, "index") }),
            "set-track-dimensions" => Ok(Mp4Mutation::SetTrackDimensions { track_index: usize_field(&params, "trackIndex"), width: number(&params, "width", 0.0) as u32, height: number(&params, "height", 0.0) as u32 }),
            "set-track-codec" => {
                let track_index = usize_field(&params, "trackIndex");
                let fallback_nal = base.tracks.get(track_index).map(|track| track.codec.nal_length_size).unwrap_or(4);
                Ok(Mp4Mutation::SetTrackCodec { track_index, codec: Mp4Codec { sps: vec![bytes(&params, "sps")], pps: vec![bytes(&params, "pps")], nal_length_size: fallback_nal, extension: None } })
            }
            "insert-sample" => Ok(Mp4Mutation::InsertSample { track_index: usize_field(&params, "trackIndex"), index: usize_field(&params, "index"), sample: sample_from_json(&params.get("sample").cloned().unwrap_or(Json::Object(Vec::new()))) }),
            "remove-sample" => Ok(Mp4Mutation::RemoveSample { track_index: usize_field(&params, "trackIndex"), index: usize_field(&params, "index") }),
            "set-sample-sync" => Ok(Mp4Mutation::SetSampleSync { track_index: usize_field(&params, "trackIndex"), index: usize_field(&params, "index"), sync: boolean(&params, "sync", true) }),
            other => Err(format!("test case does not know mutation kind {other:?}")),
        }
    }
    //#endregion 🔖️SpecReading

    //#region 🔖️Inverse
    /// ↩️ `Mp4Mutation::inverse` in closed form — the same per-variant mapping
    /// `../../🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`'s own
    /// `Mutation::inverse` impl gives, transplanted rather than called through the trait (same
    /// precedent as `mutate-pdf-1-7`'s own `inverse_of`/`mutate-wav-riff-pcm`'s own
    /// `restore_mutation`): a bug in one reading has nothing to hide behind in the other.
    fn restore_mutation(applied: &Mp4Mutation, original: &Mp4Snapshot) -> Mp4Mutation {
        match applied {
            Mp4Mutation::NoMutation => Mp4Mutation::NoMutation,
            Mp4Mutation::SetSnapshot { .. } => Mp4Mutation::SetSnapshot { snapshot: original.clone() },
            Mp4Mutation::SetFtyp { .. } => Mp4Mutation::SetFtyp { ftyp: original.ftyp.clone() },
            Mp4Mutation::InsertTrack { index, .. } => Mp4Mutation::RemoveTrack { index: *index },
            Mp4Mutation::RemoveTrack { index } => match original.tracks.get(*index) {
                Some(track) => Mp4Mutation::InsertTrack { index: *index, track: track.clone() },
                None => Mp4Mutation::NoMutation,
            },
            Mp4Mutation::SetTrackDimensions { track_index, .. } => match original.tracks.get(*track_index) {
                Some(track) => Mp4Mutation::SetTrackDimensions { track_index: *track_index, width: track.width, height: track.height },
                None => Mp4Mutation::NoMutation,
            },
            Mp4Mutation::SetTrackCodec { track_index, .. } => match original.tracks.get(*track_index) {
                Some(track) => Mp4Mutation::SetTrackCodec { track_index: *track_index, codec: track.codec.clone() },
                None => Mp4Mutation::NoMutation,
            },
            Mp4Mutation::InsertSample { .. } | Mp4Mutation::RemoveSample { .. } => Mp4Mutation::SetSnapshot { snapshot: original.clone() },
            Mp4Mutation::SetSampleSync { track_index, index, .. } => match original.tracks.get(*track_index).and_then(|track| track.samples.get(*index)) {
                Some(sample) => Mp4Mutation::SetSampleSync { track_index: *track_index, index: *index, sync: sample.sync },
                None => Mp4Mutation::NoMutation,
            },
        }
    }
    //#endregion 🔖️Inverse

    //#region 🔖️Scenarios
    /// 🎯️ Decode → apply the declared mutation → re-encode, projected through the SAME independent
    /// reader the oracle used.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let base = decode_mp4(&input).map_err(|error| format!("decode_mp4 failed: {error}"))?;
        let mutation = mutation_from_spec(&ctx.doc_json()?, &base)?;
        let mut snapshot = base;
        apply_mp4_mutation(&mut snapshot, &mutation);
        let bytes = encode_mp4(&snapshot);
        let projection = project_mp4_mutation(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🎯️ Decode → apply the declared mutation → apply its inverse → re-encode. The result must
    /// project back onto the pristine original — the inverse oracle's own reference claim.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let original = decode_mp4(&input).map_err(|error| format!("decode_mp4 failed: {error}"))?;
        let mutation = mutation_from_spec(&ctx.doc_json()?, &original)?;
        let mut snapshot = original.clone();
        apply_mp4_mutation(&mut snapshot, &mutation);
        apply_mp4_mutation(&mut snapshot, &restore_mutation(&mutation, &original));
        let bytes = encode_mp4(&snapshot);
        let projection = project_mp4_mutation(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🎯️ Full parse into the typed snapshot, then re-serialize from the model ALONE — asserted
    /// through `law::carrier_is_exact`, the DOCUMENTED MIRROR of the no-byte-pass-through tripwire,
    /// because for THIS codec reproducing the input exactly is the correct answer and anything else
    /// is the defect. The reason is the third of the law's three admissible ones: `Mp4Snapshot`
    /// carries no raw-byte escape hatch of any kind — every `mvhd`/`tkhd`/`mdhd` field, every edit
    /// list entry, the visual sample entry, `colr`/`pasp`/`btrt`, the `avcC` extension and the
    /// `stsc`/`stco` chunk grouping are typed fields — and `encode_mp4` rebuilds the whole `moov`
    /// from them into one deterministic normal form (`ftyp`, `moov`, canonical empty `free`,
    /// `mdat`) that this ffmpeg `-c copy -movflags +faststart` fixture's own layout already is.
    /// That is not an excuse for a weaker claim, it is a STRONGER one, and the artifact holds
    /// itself to it independently of this case:
    /// `../../🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/🦀️component.rs`'s own
    /// `exact_bauen_mit_bestand_fixture_round_trips_byte_for_byte` asserts the same equality on the
    /// full real recording. `law::reparsed_not_copied` would be exactly backwards here: it would
    /// demand that a lossless container codec LOSE something. The evidence that a parse really
    /// happened is the ten `mutate-*` rows above, which drive the same decode/encode pipeline and
    /// every one of which moves both the bytes and the compared projection.
    /// The ORACLE half of this same scenario keeps `law::reparsed_not_copied`, because `mp4` 0.14's
    /// `Mp4Writer` is a different writer with its own box order and `mdat` layout.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_mp4(&input).map_err(|error| format!("decode_mp4 failed: {error}"))?;
        let output = encode_mp4(&snapshot);
        law::carrier_is_exact(&output, &input)?;
        let projection = project_mp4_mutation(&output)?;
        law::round_trip_preserves(&projection, &project_mp4_mutation(&input)?)?;
        Ok(Outcome::with_raw(output, projection))
    }
    //#endregion 🔖️Scenarios
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registers by FULL expanded scenario id
/// (`mutate-<kind>` / `inverse-<kind>`), never the outline's base id — a missing registration is a
/// hard error, never a skip.
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
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
