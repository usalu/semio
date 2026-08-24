//! 🦀️ AVI 1.0 exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-REFACTOR
//! wave 7.
//!
//! Every scenario copies the real, committed `📼️bauen-mit-bestand-mjpeg.avi` fixture (derived once
//! from this repository's only real video — see the feature file's own header) into the case work
//! directory first; the committed fixture is never written to. `oracle` drives the registered
//! independent `riff`-composed codec
//! (`../../🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`'s own
//! `oracle_apply_mutation`/`oracle_apply_mutation_inverse`); `subject` drives this repository's own
//! `decode_avi`/`encode_avi`/`apply_avi_mutation` over the full 13-kind `AviMutation` vocabulary.
//! Both results are read back by the SAME independent `project_avi_1_0` before the
//! `semantic-avi-v1` profile compares them. The subject half is gated behind the generated host's
//! `sut` feature so the oracle-only run never compiles the local implementation.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::avi::standards::v1_0::subsets::any::{oracle_apply_mutation, oracle_apply_mutation_inverse, project_avi_1_0};
use semio_s_plugin_stdio_test_oracle::law;

//#region 🔖️Kinds
/// 📇️ Kebab-case spelling of every `AviMutation` variant, mirrored from
/// `../../🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`'s own `KINDS` --
/// duplicated rather than imported because the ORACLE-only build of this adapter must never link
/// `semio-s-plugin-stdio`.
const KINDS: &[&str] = &[
    "no-mutation",
    "set-snapshot",
    "set-main-header",
    "set-idx1-present",
    "insert-stream",
    "remove-stream",
    "set-stream-header",
    "set-stream-format",
    "insert-chunk",
    "remove-chunk",
    "set-chunk-keyframe",
    "add-unknown-chunk",
    "remove-unknown-chunk",
];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://📼️bauen-mit-bestand-mjpeg.avi";

/// 🧫️ Copies the immutable real fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("bauen-mit-bestand-mjpeg.avi"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 🔮️ One handler shared by every `mutate-<kind>` scenario id -- the scenario's own `<id>`/`<params>`
/// spec is carried in its doc string, not in the function it dispatches to.
/// 👁️ `@id-mutate`: applies the row's kind with the registered reference implementation and ASSERTS
/// the result is distinguishable from the untouched fixture. The exemption list is empty — every
/// kind this vocabulary declares reaches the compared projection — so a kind that stops moving it
/// fails here rather than reporting a green identical to `no-mutation`'s.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let before = project_avi_1_0(&input)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_avi_1_0(&bytes)?;
    law::mutation_is_observable(&spec.str("kind"), &projection, &before, &[])?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ One handler shared by every `inverse-<kind>` scenario id. `oracle_apply_mutation_inverse`
/// applies the kind and then its OWN independently computed inverse; this handler is what its doc
/// comment always said the caller does -- it ASSERTS the result projects back onto the pristine
/// original. The law needs no subject, so leaving it to the parity phase would make the scenario
/// pass whenever the reference `riff` composition merely did not error.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let before = project_avi_1_0(&input)?;
    let bytes = oracle_apply_mutation_inverse(&input, &spec)?;
    let projection = project_avi_1_0(&bytes)?;
    law::inverse_restores(&spec.str("kind"), &projection, &before)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔒️ The ORACLE side of the no-byte-pass-through law, ASSERTED here and not merely described: the
/// independent `riff` composition fully parses the real video container and re-serializes it from
/// its own model alone (the same "no-mutation" routing `oracle_apply_mutation` already gives every
/// other kind), so its output must differ from the input byte-wise -- our writer cannot reproduce
/// another muxer's padding and chunk layout -- while projecting onto exactly the same semantics.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let no_mutation = Json::Object(vec![("kind".to_string(), Json::String("no-mutation".to_string())), ("params".to_string(), Json::Object(vec![]))]);
    let bytes = oracle_apply_mutation(&input, &no_mutation)?;
    law::reparsed_not_copied(&bytes, &input)?;
    let before = project_avi_1_0(&input)?;
    let projection = project_avi_1_0(&bytes)?;
    law::round_trip_preserves(&projection, &before)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{mutable_input, KINDS};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::avi::standards::v1_0::subsets::any::io::{decode_avi, encode_avi};
    use semio_s_plugin_stdio::artifacts::avi::standards::v1_0::subsets::any::schema::mutations::{apply_avi_mutation, AviMutation};
    use semio_s_plugin_stdio::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::{AviChunk, AviMainHeader, AviSnapshot, AviStream, AviStreamFormat, AviStreamHeader, RiffChunk, STDIO_AVI_DOCUMENT_SCHEMA};
    use semio_s_plugin_stdio_test_oracle::artifacts::avi::standards::v1_0::subsets::any::project_avi_1_0;

    //#region 🔖️Hex
    /// 🔤️ The same lowercase-hex binary-in-text convention the oracle side uses -- duplicated
    /// rather than imported since this module must not depend on the oracle crate's private
    /// `hex_decode`.
    fn hex_decode(text: &str) -> Result<Vec<u8>, String> {
        if text.len() % 2 != 0 {
            return Err(format!("odd hex length ({} chars)", text.len()));
        }
        (0..text.len()).step_by(2).map(|i| u8::from_str_radix(&text[i..i + 2], 16).map_err(|error| format!("invalid hex {:?}: {error}", &text[i..i + 2]))).collect()
    }
    //#endregion 🔖️Hex

    //#region 🔖️SpecCodec
    fn number(value: &Json, key: &str) -> f64 {
        match value.get(key) {
            Some(Json::Number(n)) => *n,
            _ => 0.0,
        }
    }

    fn index(value: &Json, key: &str) -> usize {
        number(value, key).max(0.0) as usize
    }

    fn flag(value: &Json, key: &str) -> bool {
        matches!(value.get(key), Some(Json::Bool(true)))
    }

    /// 📄️ The same `{"microSecPerFrame": ..., ...}` JSON grammar the oracle side speaks, decoded
    /// into the PRODUCTION `AviMainHeader` here instead of the oracle's own independent type.
    fn main_header_from_json(value: &Json) -> AviMainHeader {
        let mut reserved = vec![0u32; 4];
        for (position, entry) in value.array("reserved").iter().take(4).enumerate() {
            if let Json::Number(n) = entry {
                reserved[position] = *n as u32;
            }
        }
        AviMainHeader {
            micro_sec_per_frame: number(value, "microSecPerFrame") as u32,
            max_bytes_per_sec: number(value, "maxBytesPerSec") as u32,
            padding_granularity: number(value, "paddingGranularity") as u32,
            flags: number(value, "flags") as u32,
            total_frames: number(value, "totalFrames") as u32,
            initial_frames: number(value, "initialFrames") as u32,
            streams: number(value, "streams") as u32,
            suggested_buffer_size: number(value, "suggestedBufferSize") as u32,
            width: number(value, "width") as u32,
            height: number(value, "height") as u32,
            reserved,
        }
    }

    fn strh_from_json(value: &Json) -> AviStreamHeader {
        AviStreamHeader {
            fcc_type: value.str("fccType"),
            fcc_handler: value.str("fccHandler"),
            flags: number(value, "flags") as u32,
            priority: number(value, "priority") as u16,
            language: number(value, "language") as u16,
            initial_frames: number(value, "initialFrames") as u32,
            scale: number(value, "scale") as u32,
            rate: number(value, "rate") as u32,
            start: number(value, "start") as u32,
            length: number(value, "length") as u32,
            suggested_buffer_size: number(value, "suggestedBufferSize") as u32,
            quality: number(value, "quality") as i32,
            sample_size: number(value, "sampleSize") as u32,
            rc_frame_left: number(value, "rcFrameLeft") as i32,
            rc_frame_top: number(value, "rcFrameTop") as i32,
            rc_frame_right: number(value, "rcFrameRight") as i32,
            rc_frame_bottom: number(value, "rcFrameBottom") as i32,
            // 🧭 The oracle's own strh JSON grammar has no `rcFrameWidth`/`strhExtra` keys (its
            // `write_strh` always normalizes to the full 64-byte form -- see the oracle module's own
            // doc comment); a mutation that sets a whole `strh` this way is authoring a FRESH header,
            // so this mirrors that same complete/preferred form rather than inventing new JSON keys
            // the oracle side would never produce or consume.
            rc_frame_width: 16,
            strh_extra: Vec::new(),
        }
    }

    fn strf_from_json(value: &Json) -> Result<AviStreamFormat, String> {
        match value.str("format").as_str() {
            "bitmapInfo" => Ok(AviStreamFormat::BitmapInfo {
                size: number(value, "size") as u32,
                width: number(value, "width") as i32,
                height: number(value, "height") as i32,
                planes: number(value, "planes") as u16,
                bit_count: number(value, "bitCount") as u16,
                compression: value.str("compression"),
                size_image: number(value, "sizeImage") as u32,
                x_pels_per_meter: number(value, "xPelsPerMeter") as i32,
                y_pels_per_meter: number(value, "yPelsPerMeter") as i32,
                colors_used: number(value, "colorsUsed") as u32,
                colors_important: number(value, "colorsImportant") as u32,
            }),
            "waveFormat" => Ok(AviStreamFormat::WaveFormat {
                format_tag: number(value, "formatTag") as u16,
                channels: number(value, "channels") as u16,
                samples_per_sec: number(value, "samplesPerSec") as u32,
                avg_bytes_per_sec: number(value, "avgBytesPerSec") as u32,
                block_align: number(value, "blockAlign") as u16,
                bits_per_sample: number(value, "bitsPerSample") as u16,
                extra: match value.get("extra") {
                    Some(Json::String(hex)) if !hex.is_empty() => hex_decode(hex)?,
                    _ => Vec::new(),
                },
            }),
            "raw" => Ok(AviStreamFormat::Raw {
                data: match value.get("data") {
                    Some(Json::String(hex)) if !hex.is_empty() => hex_decode(hex)?,
                    _ => Vec::new(),
                },
            }),
            other => Err(format!("unknown strf format {other:?}")),
        }
    }

    fn chunk_from_json(value: &Json) -> AviChunk {
        AviChunk {
            fourcc: value.str("fourcc"),
            data: match value.get("data") {
                Some(Json::String(hex)) if !hex.is_empty() => hex_decode(hex).unwrap_or_default(),
                _ => Vec::new(),
            },
            keyframe: flag(value, "keyframe"),
        }
    }

    fn riff_chunk_from_json(value: &Json) -> RiffChunk {
        RiffChunk {
            fourcc: value.str("fourcc"),
            data: match value.get("data") {
                Some(Json::String(hex)) if !hex.is_empty() => hex_decode(hex).unwrap_or_default(),
                _ => Vec::new(),
            },
        }
    }

    fn stream_from_json(value: &Json) -> Result<AviStream, String> {
        // 🧭 The oracle's own stream JSON grammar has no `strlExtra` key (nested `strl` auxiliaries
        // such as `vprp`/`JUNK` have no addressable mutation surface -- see `AviMutation`'s module
        // doc comment), so a mutation-authored stream never carries any.
        Ok(AviStream { strh: strh_from_json(&value.get("strh").cloned().unwrap_or(Json::Null)), strf: strf_from_json(&value.get("strf").cloned().unwrap_or(Json::Null))?, chunks: value.array("chunks").iter().map(chunk_from_json).collect(), strl_extra: Vec::new() })
    }

    fn snapshot_from_json(value: &Json) -> Result<AviSnapshot, String> {
        Ok(AviSnapshot {
            schema: STDIO_AVI_DOCUMENT_SCHEMA.to_string(),
            main_header: main_header_from_json(&value.get("mainHeader").cloned().unwrap_or(Json::Null)),
            streams: value.array("streams").iter().map(stream_from_json).collect::<Result<_, _>>()?,
            idx1_present: flag(value, "idx1Present"),
            // 🧭 Same reasoning as `strl_extra` above, one level up: no `hdrlExtra` key in the
            // oracle's `set-snapshot` JSON grammar.
            hdrl_extra: Vec::new(),
            unknown_chunks: value.array("unknownChunks").iter().map(riff_chunk_from_json).collect(),
        })
    }

    /// 📄️ The scenario's `<id>`/`<params>` spec turned into the ONE typed `AviMutation` this subset
    /// declares for it.
    fn mutation_from_spec(spec: &Json) -> Result<AviMutation, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        match spec.str("kind").as_str() {
            "no-mutation" => Ok(AviMutation::NoMutation),
            "set-snapshot" => Ok(AviMutation::SetSnapshot { snapshot: snapshot_from_json(&params)? }),
            "set-main-header" => Ok(AviMutation::SetMainHeader { main_header: main_header_from_json(&params.get("mainHeader").cloned().unwrap_or(Json::Null)) }),
            "set-idx1-present" => Ok(AviMutation::SetIdx1Present { idx1_present: flag(&params, "idx1Present") }),
            "insert-stream" => Ok(AviMutation::InsertStream { index: index(&params, "index"), stream: stream_from_json(&params.get("stream").cloned().unwrap_or(Json::Null))? }),
            "remove-stream" => Ok(AviMutation::RemoveStream { index: index(&params, "index") }),
            "set-stream-header" => Ok(AviMutation::SetStreamHeader { stream_index: index(&params, "streamIndex"), strh: strh_from_json(&params.get("strh").cloned().unwrap_or(Json::Null)) }),
            "set-stream-format" => Ok(AviMutation::SetStreamFormat { stream_index: index(&params, "streamIndex"), strf: strf_from_json(&params.get("strf").cloned().unwrap_or(Json::Null))? }),
            "insert-chunk" => Ok(AviMutation::InsertChunk { stream_index: index(&params, "streamIndex"), index: index(&params, "index"), chunk: chunk_from_json(&params.get("chunk").cloned().unwrap_or(Json::Null)) }),
            "remove-chunk" => Ok(AviMutation::RemoveChunk { stream_index: index(&params, "streamIndex"), index: index(&params, "index") }),
            "set-chunk-keyframe" => Ok(AviMutation::SetChunkKeyframe { stream_index: index(&params, "streamIndex"), index: index(&params, "index"), keyframe: flag(&params, "keyframe") }),
            "add-unknown-chunk" => Ok(AviMutation::AddUnknownChunk { index: index(&params, "index"), item: riff_chunk_from_json(&params.get("item").cloned().unwrap_or(Json::Null)) }),
            "remove-unknown-chunk" => Ok(AviMutation::RemoveUnknownChunk { index: index(&params, "index") }),
            other => Err(format!("mutation kind {other:?} has no subject implementation")),
        }
    }
    //#endregion 🔖️SpecCodec

    //#region 🔖️Inverse
    /// ↩️ `AviMutation::inverse` in closed form -- every variant's own `Mutation::inverse` arm,
    /// transplanted rather than called through the trait, same precedent `💬️bcf`'s own `inverse_of`
    /// gives: written in closed form so this adapter needs no extra crate dependency beyond
    /// `semio-s-plugin-stdio` itself.
    fn inverse_of(mutation: &AviMutation, base: &AviSnapshot) -> AviMutation {
        match mutation {
            AviMutation::NoMutation => AviMutation::NoMutation,
            AviMutation::SetSnapshot { .. } => AviMutation::SetSnapshot { snapshot: base.clone() },
            AviMutation::SetMainHeader { .. } => AviMutation::SetMainHeader { main_header: base.main_header.clone() },
            AviMutation::SetIdx1Present { .. } => AviMutation::SetIdx1Present { idx1_present: base.idx1_present },
            AviMutation::InsertStream { index, .. } => AviMutation::RemoveStream { index: *index },
            AviMutation::RemoveStream { index } => match base.streams.get(*index) {
                Some(stream) => AviMutation::InsertStream { index: *index, stream: stream.clone() },
                None => AviMutation::NoMutation,
            },
            AviMutation::SetStreamHeader { stream_index, .. } => match base.streams.get(*stream_index) {
                Some(stream) => AviMutation::SetStreamHeader { stream_index: *stream_index, strh: stream.strh.clone() },
                None => AviMutation::NoMutation,
            },
            AviMutation::SetStreamFormat { stream_index, .. } => match base.streams.get(*stream_index) {
                Some(stream) => AviMutation::SetStreamFormat { stream_index: *stream_index, strf: stream.strf.clone() },
                None => AviMutation::NoMutation,
            },
            AviMutation::InsertChunk { stream_index, index, .. } => AviMutation::RemoveChunk { stream_index: *stream_index, index: *index },
            AviMutation::RemoveChunk { stream_index, index } => match base.streams.get(*stream_index).and_then(|stream| stream.chunks.get(*index)) {
                Some(chunk) => AviMutation::InsertChunk { stream_index: *stream_index, index: *index, chunk: chunk.clone() },
                None => AviMutation::NoMutation,
            },
            AviMutation::SetChunkKeyframe { stream_index, index, .. } => match base.streams.get(*stream_index).and_then(|stream| stream.chunks.get(*index)) {
                Some(chunk) => AviMutation::SetChunkKeyframe { stream_index: *stream_index, index: *index, keyframe: chunk.keyframe },
                None => AviMutation::NoMutation,
            },
            AviMutation::AddUnknownChunk { index, .. } => AviMutation::RemoveUnknownChunk { index: *index },
            AviMutation::RemoveUnknownChunk { index } => match base.unknown_chunks.get(*index) {
                Some(item) => AviMutation::AddUnknownChunk { index: *index, item: item.clone() },
                None => AviMutation::NoMutation,
            },
        }
    }
    //#endregion 🔖️Inverse

    //#region 🔖️Handlers
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let base = decode_avi(&mutable_input(ctx)?).map_err(|error| format!("decode_avi failed: {error}"))?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        let mut snapshot = base;
        apply_avi_mutation(&mut snapshot, &mutation);
        let bytes = encode_avi(&snapshot);
        let projection = project_avi_1_0(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = decode_avi(&mutable_input(ctx)?).map_err(|error| format!("decode_avi failed: {error}"))?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        let undo = inverse_of(&mutation, &base);
        let mut snapshot = base;
        apply_avi_mutation(&mut snapshot, &mutation);
        apply_avi_mutation(&mut snapshot, &undo);
        let bytes = encode_avi(&snapshot);
        let projection = project_avi_1_0(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🔒️ The no-byte-pass-through rule: the subject must fully parse the real artifact into its
    /// typed snapshot and re-serialize from the model alone -- `decode_avi`/`encode_avi` are this
    /// subset's ONLY channel from input to output.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_avi(&input).map_err(|error| format!("decode_avi failed: {error}"))?;
        let output = encode_avi(&snapshot);
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_avi_1_0(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }
    //#endregion 🔖️Handlers

    /// 🧭️ Re-exported so `super::adapter()` can register the same 13-kind sweep for the subject role
    /// without duplicating `KINDS` a third time.
    pub const SUBJECT_KINDS: &[&str] = KINDS;
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. `mutate-<kind>`/`inverse-<kind>` share ONE
/// handler per role across all 13 kinds -- the scenario id only selects which fixture row's
/// `<id>`/`<params>` doc string the shared handler reads, per `Adapter::oracle`/`subject`'s own
/// per-scenario dispatch table.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle).oracle(&format!("inverse-{kind}"), inverse_oracle);
    }
    built = built.oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        for kind in subject::SUBJECT_KINDS {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
        }
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
