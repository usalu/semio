//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed
//! independently of this repository's own codec so the subject has something real to be compared
//! against instead of being checked against its own reading.
//!
//! Reference: `id3` 1.17 (MIT) for the ID3 layer, composed with a hand-written ISO/IEC 11172-3
//! frame walker for the MPEG audio layer. An `.mp3` byte stream is two independent layers stacked
//! in one file and no crate is authoritative over both:
//!
//! * ID3v2 (leading) — `id3::Tag::skip` locates the region's end, `Tag::read_from2` parses it and
//!   `Tag::write_to` re-serializes it from the crate's own frame model alone. That end offset is
//!   the same boundary `../🚪️io/🦀️.rs`'s `decode_mp3` has to find for itself, computed
//!   here by the reference instead of by the subject.
//! * MPEG frames (middle) — walked here from the specification: the 11-bit `0xFFE` sync word, the
//!   version/layer/bitrate-index/sample-rate-index/padding fields of the 4-byte header, and the
//!   Layer I (`(12·bitrate/rate + pad)·4`) vs Layer II/III (`144·bitrate/rate + pad`) frame-size
//!   formulae. `id3` neither reads nor writes these, and this module never calls the subject's own
//!   `find_frame_sync`/`parse_frame_header`.
//! * ID3v1 (trailing, 128 bytes) — `id3::v1::Tag::read_from` READS it; the crate has no ID3v1
//!   writer at all, so `set-id3v1` writes the trailer from the ID3v1 field layout directly. That
//!   layout leaves a writer no freedom whatsoever (fixed-width, zero-padded ISO-8859-1 fields at
//!   fixed offsets), so this is a narrow but honest half-differential: written here, read back by
//!   the reference.
//!
//! ⚖️ LICENCE: `symphonia`, the obvious pure-Rust MP3 decoder, is MPL-2.0 and no owner ruling on
//! that licence exists in this repository, so it is deliberately NOT linked. Nothing this subset's
//! vocabulary addresses (`id3v2`/`frames`/`id3v1`) needs decoded PCM, so the MPL question never had
//! to be answered to give this subset a real oracle.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared family modules rather than by copying it.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself.

use semio_repo_test_host::Json;

//#region 🔖️Kinds
/// 🏷️ The declared vocabulary of this subset, mirroring the production `KINDS`
/// (`../🧬️schema/🧬️mutations/🦀️.rs`, itself checked there against `Mp3Mutation::kind()`)
/// in declaration order. Duplicated rather than imported: the oracle crate must never link the
/// production crate, so this side can only compare STRINGS —
/// `kinds_match_the_catalog_and_the_vocabulary` below reads the committed manifest, vocabulary and
/// feature as text and fails if any of them drift apart. The check that a kind exists as a real
/// enum variant is the production-side test's, and only it can make that claim.
pub const KINDS: [&str; 5] = ["no-mutation", "set-snapshot", "set-id3v2", "set-frames", "set-id3v1"];
//#endregion 🔖️Kinds

//#region 🔖️Layers
/// 🧬️ The three-layer split, the frame walk and the two writers — everything that touches `id3` or
/// the ISO/IEC 11172-3 bit layout lives here, behind the `oracles` feature.
#[cfg(feature = "oracles")]
mod layers {
    use id3::TagLike;
    use semio_repo_test_host::Json;
    use std::io::{Cursor, Seek};

    /// 🧱️ One `.mp3` stream cut into its three independent layers. `v2`/`v1` are the raw region
    /// bytes (empty when the layer is absent); `audio` is everything in between.
    pub(super) struct Regions {
        pub v2: Vec<u8>,
        pub audio: Vec<u8>,
        pub v1: Vec<u8>,
    }

    /// ✂️ Splits a stream. The ID3v2 end offset comes from `id3::Tag::skip` — the reference's own
    /// header/synchsafe-size parse, not ours; the ID3v1 trailer is the last 128 bytes when they
    /// start with `TAG`, which is the whole of that layer's framing.
    pub(super) fn split(input: &[u8]) -> Result<Regions, String> {
        let mut cursor = Cursor::new(input);
        let has_v2 = id3::Tag::skip(&mut cursor).map_err(|error| format!("id3::Tag::skip failed: {error}"))?;
        let v2_end = if has_v2 { cursor.stream_position().map_err(|error| error.to_string())? as usize } else { 0 };
        if v2_end > input.len() {
            return Err(format!("id3::Tag::skip reported an ID3v2 region ending at {v2_end}, past the {}-byte input", input.len()));
        }
        let has_v1 = input.len() >= v2_end + 128 && &input[input.len() - 128..input.len() - 125] == b"TAG";
        let audio_end = if has_v1 { input.len() - 128 } else { input.len() };
        Ok(Regions { v2: input[..v2_end].to_vec(), audio: input[v2_end..audio_end].to_vec(), v1: input[audio_end..].to_vec() })
    }

    /// 🧵️ Re-joins three layers into one stream.
    pub(super) fn join(regions: &Regions) -> Vec<u8> {
        let mut out = Vec::with_capacity(regions.v2.len() + regions.audio.len() + regions.v1.len());
        out.extend_from_slice(&regions.v2);
        out.extend_from_slice(&regions.audio);
        out.extend_from_slice(&regions.v1);
        out
    }

    //#region 🔖️Id3v2
    /// 🏷️ One ID3v2 text frame as this oracle expresses it: the four-character frame id and its
    /// decoded text. A non-text frame is refused rather than silently dropped — losing one on a
    /// re-write would make the mutation look clean while destroying content.
    pub(super) struct TextFrame {
        pub id: String,
        pub text: String,
    }

    /// 🔎️ Reads the ID3v2 region with the reference. `None` when the stream carries no tag.
    pub(super) fn read_v2(v2: &[u8]) -> Result<Option<(id3::Version, Vec<TextFrame>)>, String> {
        if v2.is_empty() {
            return Ok(None);
        }
        let tag = id3::Tag::read_from2(Cursor::new(v2)).map_err(|error| format!("id3::Tag::read_from2 failed: {error}"))?;
        let mut frames = Vec::new();
        for frame in tag.frames() {
            match frame.content() {
                id3::Content::Text(text) => frames.push(TextFrame { id: frame.id().to_string(), text: text.clone() }),
                other => return Err(format!("ID3v2 frame {:?} carries {other:?}, which this oracle does not express — refusing to drop it silently", frame.id())),
            }
        }
        Ok(Some((tag.version(), frames)))
    }

    /// 🏷️ Writes an ID3v2.3 region with the reference's own encoder. An empty frame list means "no
    /// tag at all", which is a real state of the format, not an empty tag.
    pub(super) fn write_v2(frames: &[TextFrame]) -> Result<Vec<u8>, String> {
        if frames.is_empty() {
            return Ok(Vec::new());
        }
        let mut tag = id3::Tag::with_version(id3::Version::Id3v23);
        for frame in frames {
            if frame.id.len() != 4 {
                return Err(format!("ID3v2.3 frame id {:?} is not four characters", frame.id));
            }
            tag.add_frame(id3::Frame::text(&frame.id, frame.text.clone()));
        }
        let mut out = Vec::new();
        tag.write_to(&mut out, id3::Version::Id3v23).map_err(|error| format!("id3::Tag::write_to failed: {error}"))?;
        Ok(out)
    }
    //#endregion 🔖️Id3v2

    //#region 🔖️Id3v1
    /// 🏷️ The six ID3v1 fields this oracle expresses, in the trailer's own order.
    #[derive(Default)]
    pub(super) struct V1Fields {
        pub title: String,
        pub artist: String,
        pub album: String,
        pub year: String,
        pub comment: String,
        pub genre_id: u8,
    }

    /// 🔎️ Reads the ID3v1 trailer with the reference (`id3::v1::Tag::read_from`, which seeks from
    /// the END of the stream — so it is handed the whole stream, not the isolated region).
    pub(super) fn read_v1(input: &[u8], v1: &[u8]) -> Result<Option<V1Fields>, String> {
        if v1.is_empty() {
            return Ok(None);
        }
        let tag = id3::v1::Tag::read_from(Cursor::new(input)).map_err(|error| format!("id3::v1::Tag::read_from failed: {error}"))?;
        Ok(Some(V1Fields { title: tag.title, artist: tag.artist, album: tag.album, year: tag.year, comment: tag.comment, genre_id: tag.genre_id }))
    }

    /// 🏷️ Writes the 128-byte ID3v1 trailer. `id3` has no ID3v1 writer, and the layout leaves none
    /// of the freedom a writer usually has: `TAG` + 30/30/30/4/30 zero-padded ISO-8859-1 bytes +
    /// one genre byte, at fixed offsets. A character outside ISO-8859-1 is refused rather than
    /// lossily transliterated.
    pub(super) fn write_v1(fields: &V1Fields) -> Result<Vec<u8>, String> {
        let mut out = vec![0u8; 128];
        out[0..3].copy_from_slice(b"TAG");
        let mut put = |value: &str, start: usize, width: usize| -> Result<(), String> {
            let bytes: Vec<u8> = value
                .chars()
                .map(|ch| if (ch as u32) < 0x100 { Ok(ch as u8) } else { Err(format!("ID3v1 field {value:?} carries {ch:?}, which is outside ISO-8859-1")) })
                .collect::<Result<Vec<u8>, String>>()?;
            if bytes.len() > width {
                return Err(format!("ID3v1 field {value:?} is {} byte(s), past its {width}-byte slot", bytes.len()));
            }
            out[start..start + bytes.len()].copy_from_slice(&bytes);
            Ok(())
        };
        put(&fields.title, 3, 30)?;
        put(&fields.artist, 33, 30)?;
        put(&fields.album, 63, 30)?;
        put(&fields.year, 93, 4)?;
        put(&fields.comment, 97, 30)?;
        out[127] = fields.genre_id;
        Ok(out)
    }
    //#endregion 🔖️Id3v1

    //#region 🔖️MpegFrames
    /// 🎼️ One MPEG audio frame as the specification describes it, plus its own byte offset and
    /// total size.
    pub(super) struct MpegFrame {
        pub offset: usize,
        pub size: usize,
        pub mpeg_version_id: u8,
        pub layer: u8,
        pub bitrate_kbps: u16,
        pub sample_rate_hz: u32,
        pub padding: bool,
        pub channel_mode: u8,
    }

    /// 📐️ ISO/IEC 11172-3 / 13818-3 bitrate tables, keyed by `(version_id, layer)`. `version_id`:
    /// `0`=MPEG2.5, `2`=MPEG2, `3`=MPEG1 (`1` is reserved). `layer`: `1`=III, `2`=II, `3`=I (`0` is
    /// reserved). Index `0` ("free" bitrate — no frame-size formula applies) and `15` (reserved)
    /// are honest decode failures.
    fn bitrate_kbps(version_id: u8, layer: u8, index: u8) -> Option<u16> {
        const V1_L1: [u16; 16] = [0, 32, 64, 96, 128, 160, 192, 224, 256, 288, 320, 352, 384, 416, 448, 0];
        const V1_L2: [u16; 16] = [0, 32, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 384, 0];
        const V1_L3: [u16; 16] = [0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 0];
        const V2_L1: [u16; 16] = [0, 32, 48, 56, 64, 80, 96, 112, 128, 144, 160, 176, 192, 224, 256, 0];
        const V2_L23: [u16; 16] = [0, 8, 16, 24, 32, 40, 48, 56, 64, 80, 96, 112, 128, 144, 160, 0];
        if index == 0 || index == 15 {
            return None;
        }
        let table = match (version_id, layer) {
            (3, 3) => &V1_L1,
            (3, 2) => &V1_L2,
            (3, 1) => &V1_L3,
            (0 | 2, 3) => &V2_L1,
            (0 | 2, 1 | 2) => &V2_L23,
            _ => return None,
        };
        Some(table[index as usize])
    }

    /// 📐️ Sampling frequency, per version: MPEG1 44100/48000/32000, MPEG2 halves those, MPEG2.5
    /// quarters them. Index `3` is the reserved value.
    fn sample_rate_hz(version_id: u8, index: u8) -> Option<u32> {
        let base = match index {
            0 => 44_100u32,
            1 => 48_000,
            2 => 32_000,
            _ => return None,
        };
        match version_id {
            3 => Some(base),
            2 => Some(base / 2),
            0 => Some(base / 4),
            _ => None,
        }
    }

    /// 🚶 Walks the audio region into real frames. A byte that is not part of a decodable frame
    /// ends the walk, and any trailing remainder is reported — a silently ignored tail is how a
    /// truncated stream passes for a whole one.
    pub(super) fn walk(audio: &[u8]) -> Result<Vec<MpegFrame>, String> {
        let mut frames = Vec::new();
        let mut pos = 0usize;
        while pos + 4 <= audio.len() {
            if audio[pos] != 0xFF || (audio[pos + 1] & 0xE0) != 0xE0 {
                break;
            }
            let b1 = audio[pos + 1];
            let b2 = audio[pos + 2];
            let b3 = audio[pos + 3];
            let mpeg_version_id = (b1 >> 3) & 0x03;
            let layer = (b1 >> 1) & 0x03;
            if mpeg_version_id == 0x01 || layer == 0x00 {
                break;
            }
            let bitrate_index = (b2 >> 4) & 0x0F;
            let sample_rate_index = (b2 >> 2) & 0x03;
            let padding = ((b2 >> 1) & 0x01) != 0;
            let channel_mode = (b3 >> 6) & 0x03;
            let Some(bitrate) = bitrate_kbps(mpeg_version_id, layer, bitrate_index) else { break };
            let Some(rate) = sample_rate_hz(mpeg_version_id, sample_rate_index) else { break };
            let pad = u32::from(padding);
            let size = if layer == 3 { (12 * (u32::from(bitrate) * 1000) / rate + pad) * 4 } else { 144 * (u32::from(bitrate) * 1000) / rate + pad } as usize;
            if size < 4 || pos + size > audio.len() {
                break;
            }
            frames.push(MpegFrame { offset: pos, size, mpeg_version_id, layer, bitrate_kbps: bitrate, sample_rate_hz: rate, padding, channel_mode });
            pos += size;
        }
        if pos != audio.len() {
            return Err(format!("the MPEG frame walk stopped at byte {pos} of a {}-byte audio region — {} trailing byte(s) belong to no decodable frame", audio.len(), audio.len() - pos));
        }
        Ok(frames)
    }
    //#endregion 🔖️MpegFrames

    //#region 🔖️Projection
    fn text_frames_json(frames: &[TextFrame]) -> Json {
        Json::Array(frames.iter().map(|frame| Json::Object(vec![("id".to_string(), Json::String(frame.id.clone())), ("text".to_string(), Json::String(frame.text.clone()))])).collect())
    }

    /// 🎯️ The projection `semantic-mp3-mpeg1-layer3-v1` compares. ID3v2 padding, the synchsafe size
    /// field and the flags byte are writer freedom and are not projected at all.
    pub(super) fn project(input: &[u8]) -> Result<Json, String> {
        let regions = split(input)?;
        let v2 = match read_v2(&regions.v2)? {
            None => Json::Null,
            // 🧭️ `id3::Version::minor()` returns 2/3/4 — the ID3v2 specification's own MAJOR
            // version byte (offset 3), which is what `Id3v2Tag::major_version` holds too. The
            // revision byte (offset 4) is not modelled by the reference at all, so it is not
            // projected rather than being invented from a default.
            Some((version, frames)) => Json::Object(vec![("majorVersion".to_string(), Json::Number(f64::from(version.minor()))), ("frames".to_string(), text_frames_json(&frames))]),
        };
        let frames = walk(&regions.audio)?;
        let audio = Json::Array(
            frames
                .iter()
                .map(|frame| {
                    Json::Object(vec![
                        ("mpegVersionId".to_string(), Json::Number(f64::from(frame.mpeg_version_id))),
                        ("layer".to_string(), Json::Number(f64::from(frame.layer))),
                        ("bitrateKbps".to_string(), Json::Number(f64::from(frame.bitrate_kbps))),
                        ("sampleRateHz".to_string(), Json::Number(f64::from(frame.sample_rate_hz))),
                        ("padding".to_string(), Json::Bool(frame.padding)),
                        ("channelMode".to_string(), Json::Number(f64::from(frame.channel_mode))),
                        ("size".to_string(), Json::Number(frame.size as f64)),
                    ])
                })
                .collect(),
        );
        let v1 = match read_v1(input, &regions.v1)? {
            None => Json::Null,
            Some(fields) => Json::Object(vec![
                ("title".to_string(), Json::String(fields.title)),
                ("artist".to_string(), Json::String(fields.artist)),
                ("album".to_string(), Json::String(fields.album)),
                ("year".to_string(), Json::String(fields.year)),
                ("comment".to_string(), Json::String(fields.comment)),
                ("genreId".to_string(), Json::Number(f64::from(fields.genre_id))),
            ]),
        };
        Ok(Json::Object(vec![("id3v2".to_string(), v2), ("frames".to_string(), audio), ("id3v1".to_string(), v1)]))
    }
    //#endregion 🔖️Projection
}
//#endregion 🔖️Layers

//#region 🔖️SpecReaders
/// 🔎️ A spec's `params` object, or `Null`.
fn params_of(spec: &Json) -> Json {
    spec.get("params").cloned().unwrap_or(Json::Null)
}

/// 🔎️ `true` when the params carry `key` explicitly, even as `null` — the difference between
/// "clear the tag" and "leave the tag alone".
fn has(params: &Json, key: &str) -> bool {
    params.get(key).is_some()
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn hex_decode(text: &str) -> Result<Vec<u8>, String> {
    if text.len() % 2 != 0 {
        return Err(format!("hex payload has an odd length ({})", text.len()));
    }
    (0..text.len() / 2).map(|index| u8::from_str_radix(&text[index * 2..index * 2 + 2], 16).map_err(|error| format!("hex payload is malformed at pair {index}: {error}"))).collect()
}
//#endregion 🔖️SpecReaders

//#region 🔖️Projection
/// 🎯️ Reads a real `.mp3` stream into the semantic projection both roles are compared through.
#[cfg(feature = "oracles")]
pub fn project_mp3(bytes: &[u8]) -> Result<Json, String> {
    layers::project(bytes)
}

/// 🚫️ Without the `oracles` feature the reference implementations are not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn project_mp3(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Projection

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test.
///
/// The three layers are addressed independently, exactly as `Mp3Mutation` addresses
/// `id3v2`/`frames`/`id3v1`:
/// * `text` — an array of `{id, text}` ID3v2.3 text frames, or `null` to remove the tag entirely.
/// * `take` — keep the first N MPEG frames (a real truncation of the frame sequence).
/// * `framesHex` — an explicit audio region, hex-encoded. Only ever produced by
///   [`oracle_inverse_spec`], never hand-written in a feature file: restoring a frame sequence is
///   not expressible as a slice of the document it is being applied to.
/// * `v1` — an object of ID3v1 fields, or `null` to remove the trailer.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let params = params_of(spec);
    let mut regions = layers::split(input)?;
    let kind = spec.str("kind");
    let touches_v2 = matches!(kind.as_str(), "set-id3v2" | "set-snapshot");
    let touches_frames = matches!(kind.as_str(), "set-frames" | "set-snapshot");
    let touches_v1 = matches!(kind.as_str(), "set-id3v1" | "set-snapshot");
    match kind.as_str() {
        "" => return Err("mutation spec carries no `kind`".to_string()),
        "no-mutation" => return Ok(input.to_vec()),
        "set-snapshot" | "set-id3v2" | "set-frames" | "set-id3v1" => {}
        other => return Err(format!("mutation kind {other:?} has no oracle implementation ({} input byte(s))", input.len())),
    }
    if touches_v2 {
        if !has(&params, "text") {
            return Err(format!("{kind}: params carry no `text` — an ID3v2 mutation must state the frames it lands on, `null` to remove the tag"));
        }
        regions.v2 = match params.get("text") {
            Some(Json::Null) | None => Vec::new(),
            Some(Json::Array(items)) => {
                let frames: Vec<layers::TextFrame> = items.iter().map(|item| layers::TextFrame { id: item.str("id"), text: item.str("text") }).collect();
                layers::write_v2(&frames)?
            }
            Some(other) => return Err(format!("{kind}: `text` must be an array of {{id, text}} or null, not {}", other.to_string())),
        };
    }
    if touches_frames {
        regions.audio = match (params.get("framesHex"), params.get("take")) {
            (Some(Json::String(text)), _) => hex_decode(text)?,
            (_, Some(Json::Number(count))) => {
                let frames = layers::walk(&regions.audio)?;
                let keep = *count as usize;
                if keep > frames.len() {
                    return Err(format!("{kind}: `take` is {keep} but the document carries only {} MPEG frame(s)", frames.len()));
                }
                let end = frames.get(keep).map(|frame| frame.offset).unwrap_or(regions.audio.len());
                regions.audio[..end].to_vec()
            }
            _ => return Err(format!("{kind}: params carry neither `take` nor `framesHex` — a frame mutation must state the sequence it lands on")),
        };
    }
    if touches_v1 {
        if !has(&params, "v1") {
            return Err(format!("{kind}: params carry no `v1` — an ID3v1 mutation must state the trailer it lands on, `null` to remove it"));
        }
        regions.v1 = match params.get("v1") {
            Some(Json::Null) | None => Vec::new(),
            Some(fields) => layers::write_v1(&layers::V1Fields {
                title: fields.str("title"),
                artist: fields.str("artist"),
                album: fields.str("album"),
                year: fields.str("year"),
                comment: fields.str("comment"),
                genre_id: match fields.get("genreId") {
                    Some(Json::Number(value)) => *value as u8,
                    _ => 0,
                },
            })?,
        };
    }
    Ok(layers::join(&regions))
}

/// 🚫️ Without the `oracles` feature the reference implementations are not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🔖️Inverse
/// ↩️ The independently computed inverse of `spec` against the UNMUTATED `base`, matching
/// `Mp3Mutation::inverse()`'s own base-relative semantics: every variant of this vocabulary is a
/// whole-layer replace, so its inverse is the same verb carrying the layer `base` already had.
/// A restored frame sequence cannot be expressed as a slice of the mutated document, so it travels
/// as `framesHex` — bytes read out of `base` here, never authored by hand.
#[cfg(feature = "oracles")]
pub fn oracle_inverse_spec(base: &[u8], spec: &Json) -> Result<Json, String> {
    let regions = layers::split(base)?;
    let text = match layers::read_v2(&regions.v2)? {
        None => Json::Null,
        Some((_, frames)) => Json::Array(frames.iter().map(|frame| Json::Object(vec![("id".to_string(), Json::String(frame.id.clone())), ("text".to_string(), Json::String(frame.text.clone()))])).collect()),
    };
    let v1 = match layers::read_v1(base, &regions.v1)? {
        None => Json::Null,
        Some(fields) => Json::Object(vec![
            ("title".to_string(), Json::String(fields.title)),
            ("artist".to_string(), Json::String(fields.artist)),
            ("album".to_string(), Json::String(fields.album)),
            ("year".to_string(), Json::String(fields.year)),
            ("comment".to_string(), Json::String(fields.comment)),
            ("genreId".to_string(), Json::Number(f64::from(fields.genre_id))),
        ]),
    };
    let frames_hex = Json::String(hex_encode(&regions.audio));
    let params = match spec.str("kind").as_str() {
        "no-mutation" => Json::Object(vec![]),
        "set-id3v2" => Json::Object(vec![("text".to_string(), text)]),
        "set-frames" => Json::Object(vec![("framesHex".to_string(), frames_hex)]),
        "set-id3v1" => Json::Object(vec![("v1".to_string(), v1)]),
        "set-snapshot" => Json::Object(vec![("text".to_string(), text), ("framesHex".to_string(), frames_hex), ("v1".to_string(), v1)]),
        other => return Err(format!("mutation kind {other:?} has no oracle inverse")),
    };
    Ok(Json::Object(vec![("kind".to_string(), Json::String(spec.str("kind"))), ("params".to_string(), params)]))
}

/// 🚫️ Without the `oracles` feature the reference implementations are not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_inverse_spec(_base: &[u8], _spec: &Json) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Inverse

//#region 🔖️RoundTrip
/// 🔁️ Decodes the stream into the reference's own models and re-encodes from those alone: the
/// ID3v2 tag through `id3::Tag`'s frame model, the audio region through the walked frame list
/// (each frame's bytes taken from its own decoded offset and size, never from the region as one
/// slab), the ID3v1 trailer through the field layout. `id3`'s writer chooses its own padding, so
/// this does NOT reproduce the input bytes — which is the point: see the `identity-round-trip`
/// scenario's no-byte-pass-through half.
#[cfg(feature = "oracles")]
pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    let regions = layers::split(input)?;
    let v2 = match layers::read_v2(&regions.v2)? {
        None => Vec::new(),
        Some((_, frames)) => layers::write_v2(&frames)?,
    };
    let mut audio = Vec::with_capacity(regions.audio.len());
    for frame in layers::walk(&regions.audio)? {
        audio.extend_from_slice(&regions.audio[frame.offset..frame.offset + frame.size]);
    }
    let v1 = match layers::read_v1(input, &regions.v1)? {
        None => Vec::new(),
        Some(fields) => layers::write_v1(&fields)?,
    };
    Ok(layers::join(&layers::Regions { v2, audio, v1 }))
}

/// 🚫️ Without the `oracles` feature the reference implementations are not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_round_trip(_input: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️RoundTrip

//#region 🧪️Tests
#[cfg(all(test, feature = "oracles"))]
mod tests {
    use super::*;

    /// 🎵️ The real committed fixture — 193,275 bytes of genuinely encoded MPEG-1 Layer III audio,
    /// derived once from the repository's own real camera-captured video and encoded by `lame`, a
    /// real third-party encoder. Provenance and the exact derivation command are in the case's
    /// feature description and in the ticket's `mp3-fixture-derive/🐍️derive-real-mp3-fixture.py`.
    /// The gherkin case reads the same file through `ctx.copy_fixture`.
    fn fixture() -> Vec<u8> {
        include_bytes!("../../../../../🧫️fixtures/🎵️bauen-mit-bestand-ausschnitt.mp3").to_vec()
    }

    fn spec(kind: &str, params: Json) -> Json {
        Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params)])
    }

    fn object(entries: Vec<(&str, Json)>) -> Json {
        Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
    }

    #[test]
    fn the_three_layers_of_the_real_fixture_are_found_where_the_specification_puts_them() {
        let input = fixture();
        assert_eq!(input.len(), 193_275, "the committed real fixture");
        let regions = layers::split(&input).unwrap();
        assert_eq!(regions.v2.len(), 179, "the ID3v2.3 region LAME wrote is a 10-byte header plus a 169-byte body");
        assert_eq!(regions.audio.len(), 193_096);
        assert_eq!(regions.v2.len() + regions.audio.len(), input.len(), "the three layers must partition the stream, leaving nothing unaccounted for");
        assert!(regions.v1.is_empty(), "`lame --id3v2-only` wrote no ID3v1 trailer, which is what keeps `set-id3v1` an ADD");
    }

    /// 🚶 The frame walk over a REAL encoded stream, which is what makes this different from a
    /// walk over digital silence: 462 frames, and — the point — BOTH padding-slot values genuinely
    /// occur, so `144·bitrate/rate + pad` is exercised on both of its branches. 128000/44100 is not
    /// an integer, so a real CBR encoder MUST alternate the padding slot to hold the average rate;
    /// a fixture whose frames all measure 417 bytes never tests the `+ pad` term at all.
    #[test]
    fn the_frame_walk_reads_every_real_mpeg1_layer3_frame_and_both_padding_slots() {
        let input = fixture();
        let frames = layers::walk(&layers::split(&input).unwrap().audio).unwrap();
        assert_eq!(frames.len(), 462);
        for frame in &frames {
            assert_eq!((frame.mpeg_version_id, frame.layer), (3, 1), "MPEG1 Layer III");
            assert_eq!((frame.bitrate_kbps, frame.sample_rate_hz), (128, 44_100));
            assert_eq!(frame.channel_mode, 3, "mono");
            assert_eq!(frame.size, if frame.padding { 418 } else { 417 }, "144·128000/44100 = 417, plus the padding slot");
        }
        assert_eq!(frames.iter().filter(|frame| !frame.padding).count(), 20);
        assert_eq!(frames.iter().filter(|frame| frame.padding).count(), 442);
        assert_eq!(frames.iter().map(|frame| frame.size).sum::<usize>(), 193_096, "the walk must consume the audio region exactly");
    }

    /// 🏷️ The reference reads the tag a REAL encoder wrote, including two frames in encoding `1`
    /// (UTF-16 with a byte-order mark) — the form a real-world writer emits and the previous
    /// handcrafted fixture, which was ISO-8859-1 throughout, never exercised.
    #[test]
    fn the_reference_reads_the_text_frames_a_real_encoder_wrote() {
        let input = fixture();
        let projection = project_mp3(&input).unwrap();
        let v2 = projection.get("id3v2").unwrap();
        assert_eq!(v2.get("majorVersion").unwrap().clone(), Json::Number(3.0));
        let frame = |id: &str, text: &str| object(vec![("id", Json::String(id.to_string())), ("text", Json::String(text.to_string()))]);
        assert_eq!(
            v2.array("frames"),
            vec![
                frame("TSSE", "LAME 64bits version 3.100 (http://lame.sf.net)"),
                frame("TIT2", "Bauen mit Bestand (Ausschnitt)"),
                frame("TPE1", "semio"),
                frame("TLEN", "12000"),
            ]
        );
        assert_eq!(projection.get("id3v1").unwrap().clone(), Json::Null);
    }

    #[test]
    fn no_mutation_is_a_true_byte_identity() {
        let input = fixture();
        assert_eq!(oracle_apply_mutation(&input, &spec("no-mutation", Json::Object(vec![]))).unwrap(), input);
    }

    /// 🧾️ The case's OWN `Examples` rows, transcribed in the feature file's order. Checking the
    /// laws against the parameters the scenarios actually carry is the point — a row whose params
    /// address nothing would report green while testing nothing, and the runner never dispatches an
    /// observability check of its own. `set-frames`'s `take` of 231 truncates at the midpoint of a
    /// 462-frame stream and `set-snapshot`'s take of 3 crosses the first padding-slot change
    /// (frames 0 and 1 are 417 bytes, frame 2 is 418), so both land on real offset arithmetic
    /// rather than on the head of the region.
    fn feature_example_rows() -> Vec<Json> {
        let v1 = |title: &str| object(vec![("title", Json::String(title.to_string())), ("artist", Json::String("semio".to_string())), ("album", Json::String(String::new())), ("year", Json::String("2026".to_string())), ("comment", Json::String(String::new())), ("genreId", Json::Number(12.0))]);
        let text = |id: &str, value: &str| object(vec![("id", Json::String(id.to_string())), ("text", Json::String(value.to_string()))]);
        vec![
            spec("no-mutation", Json::Object(vec![])),
            spec("set-snapshot", object(vec![("text", Json::Array(vec![text("TALB", "replaced wholesale")])), ("take", Json::Number(3.0)), ("v1", v1("snapshot"))])),
            spec("set-id3v2", object(vec![("text", Json::Array(vec![text("TIT2", "renamed by the oracle"), text("TPE1", "semio")]))])),
            spec("set-frames", object(vec![("take", Json::Number(231.0))])),
            spec("set-id3v1", object(vec![("v1", v1("added trailer"))])),
        ]
    }

    #[test]
    fn every_kind_is_observable_and_its_own_inverse_restores_the_projection() {
        let input = fixture();
        let original = project_mp3(&input).unwrap();
        for case in feature_example_rows() {
            let kind = case.str("kind");
            let mutated = oracle_apply_mutation(&input, &case).unwrap_or_else(|error| panic!("{kind} failed: {error}"));
            let after = project_mp3(&mutated).unwrap();
            if kind != "no-mutation" {
                assert_ne!(after, original, "{kind} left the projection unchanged — a mutation that is not observable proves nothing");
            }
            let inverse = oracle_inverse_spec(&input, &case).unwrap();
            let restored = oracle_apply_mutation(&mutated, &inverse).unwrap_or_else(|error| panic!("{kind} inverse failed: {error}"));
            assert_eq!(project_mp3(&restored).unwrap(), original, "applying {kind} and then its own inverse must restore the original projection");
        }
    }

    #[test]
    fn the_round_trip_preserves_the_projection_without_handing_the_input_back() {
        let input = fixture();
        let output = oracle_round_trip(&input).unwrap();
        assert_ne!(output, input, "id3's writer re-derives the tag region, so a bit-identical result would mean nothing was parsed");
        assert_eq!(project_mp3(&output).unwrap(), project_mp3(&input).unwrap());
    }

    #[test]
    fn an_unknown_kind_is_an_error_not_a_silent_no_op() {
        assert!(oracle_apply_mutation(&fixture(), &spec("set-bitrate", Json::Object(vec![]))).is_err());
        assert!(oracle_apply_mutation(&fixture(), &spec("set-id3v2", Json::Object(vec![]))).unwrap_err().contains("no `text`"));
    }

    /// 🏷️ `KINDS` must equal the committed catalog AND the committed production vocabulary. The
    /// framework never parses Rust, so the catalog is what the contract gate counts against; this
    /// reads both files as text and fails the moment any of the three drift apart.
    #[test]
    fn kinds_match_the_catalog_and_the_vocabulary() {
        let manifest = include_str!("🔣️.json");
        let vocabulary = include_str!("../🧬️schema/🧬️mutations/🦀️.rs");
        let variants = ["SetSnapshot", "SetId3v2", "SetFrames", "SetId3v1"];
        assert_eq!(KINDS.len(), variants.len());
        for (kind, variant) in KINDS.iter().zip(variants.iter()) {
            assert!(manifest.contains(&format!("\"{kind}\"")), "catalog is missing kind {kind:?}");
            assert!(vocabulary.contains(&format!("{variant} ")) || vocabulary.contains(&format!("{variant},")) || vocabulary.contains(&format!("{variant} {{")), "Mp3Mutation is missing variant {variant:?} for kind {kind:?}");
        }
        let feature = include_str!("../../../../../🧪️tests/mutate-mp3-mpeg1-layer3/🥒️.feature");
        for kind in KINDS {
            assert!(feature.contains(&format!("| {kind} ")) || feature.contains(&format!("| {kind}\t")) || feature.contains(&format!("| {kind}  ")), "the case's Examples table is missing kind {kind:?}");
        }
    }
}
//#endregion 🧪️Tests
