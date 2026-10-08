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
pub const KINDS: [&str; 3] = ["set-id3v2", "set-frames", "set-id3v1"];
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
    pub(super) struct MetadataFrame {pub id:String,pub content:id3::Content}
    pub(super) fn read_v2(v2:&[u8])->Result<Option<(id3::Version,Vec<MetadataFrame>)>,String>{
        if v2.is_empty(){return Ok(None);}let tag=id3::Tag::read_from2(Cursor::new(v2)).map_err(|e|e.to_string())?;
        let mut frames=Vec::new();for frame in tag.frames(){match frame.content(){id3::Content::Text(_)|id3::Content::ExtendedText(_)|id3::Content::Link(_)|id3::Content::ExtendedLink(_)|id3::Content::Comment(_)|id3::Content::Lyrics(_)|id3::Content::Picture(_)|id3::Content::Unknown(_)=>frames.push(MetadataFrame{id:frame.id().into(),content:frame.content().clone()}),other=>return Err(format!("unsupported oracle frame {other:?}"))}}Ok(Some((tag.version(),frames)))
    }
    pub(super) fn write_v2(_version:id3::Version,frames:&[MetadataFrame])->Result<Vec<u8>,String>{let mut tag=id3::Tag::with_version(id3::Version::Id3v24);for frame in frames{tag.add_frame(id3::Frame::with_content(&frame.id,frame.content.clone()));}let mut out=Vec::new();tag.write_to(&mut out,id3::Version::Id3v24).map_err(|e|e.to_string())?;Ok(out)}
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
        pub track: Option<u8>,
    }

    /// 🔎️ Reads the ID3v1 trailer with the reference (`id3::v1::Tag::read_from`, which seeks from
    /// the END of the stream — so it is handed the whole stream, not the isolated region).
    pub(super) fn read_v1(input: &[u8], v1: &[u8]) -> Result<Option<V1Fields>, String> {
        if v1.is_empty() {
            return Ok(None);
        }
        let tag = id3::v1::Tag::read_from(Cursor::new(input)).map_err(|error| format!("id3::v1::Tag::read_from failed: {error}"))?;
        Ok(Some(V1Fields { title: tag.title, artist: tag.artist, album: tag.album, year: tag.year, comment: tag.comment, genre_id: tag.genre_id, track:tag.track }))
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
        put(&fields.comment, 97, if fields.track.is_some(){28}else{30})?;
        if let Some(track)=fields.track{if track==0{return Err("track zero".into());}out[126]=track;}
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

    /// 🧾️ The twelve fields of one MPEG audio frame header, as ISO/IEC 11172-3 §2.4.1.3 lays them out after the
    /// sync word.
    pub(super) struct HeaderFields {
        pub mpeg_version_id: u8,
        pub layer: u8,
        pub protection_bit: bool,
        pub bitrate_index: u8,
        pub sample_rate_index: u8,
        pub padding: bool,
        pub private_bit: bool,
        pub channel_mode: u8,
        pub mode_extension: u8,
        pub copyright: bool,
        pub original: bool,
        pub emphasis: u8,
    }

    /// 📦️ Packs one header into its four bytes: the 11-bit sync word, then every field at its specified bit
    /// position. A field wider than its slot is refused — truncating it would forge a different header.
    pub(super) fn pack_header(fields: &HeaderFields) -> Result<[u8; 4], String> {
        let slot = |name: &str, value: u8, bits: u32| if u32::from(value) < (1 << bits) { Ok(value) } else { Err(format!("header field {name} is {value}, wider than its {bits}-bit slot")) };
        Ok([
            0xFF,
            0xE0 | slot("mpegVersionId", fields.mpeg_version_id, 2)? << 3 | slot("layer", fields.layer, 2)? << 1 | u8::from(fields.protection_bit),
            slot("bitrateIndex", fields.bitrate_index, 4)? << 4 | slot("sampleRateIndex", fields.sample_rate_index, 2)? << 2 | u8::from(fields.padding) << 1 | u8::from(fields.private_bit),
            slot("channelMode", fields.channel_mode, 2)? << 6 | slot("modeExtension", fields.mode_extension, 2)? << 4 | u8::from(fields.copyright) << 3 | u8::from(fields.original) << 2 | slot("emphasis", fields.emphasis, 2)?,
        ])
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
    fn content_json(content:&id3::Content)->Result<Json,String>{
        let text=|s:&str|Json::String(s.into());let values=|s:&str|Json::Array(s.split('\0').map(text).collect());let bytes=|v:&[u8]|Json::Array(v.iter().map(|&b|Json::Number(b.into())).collect());
        let fields=match content{
            id3::Content::Text(v)=>vec![("kind",text("text")),("values",values(v))],
            id3::Content::ExtendedText(v)=>vec![("kind",text("userText")),("description",text(&v.description)),("values",values(&v.value))],
            id3::Content::Link(v)=>vec![("kind",text("url")),("url",text(v))],
            id3::Content::ExtendedLink(v)=>vec![("kind",text("userUrl")),("description",text(&v.description)),("url",text(&v.link))],
            id3::Content::Comment(v)=>vec![("kind",text("comment")),("language",text(&v.lang)),("description",text(&v.description)),("text",text(&v.text))],
            id3::Content::Lyrics(v)=>vec![("kind",text("lyrics")),("language",text(&v.lang)),("description",text(&v.description)),("text",text(&v.text))],
            id3::Content::Picture(v)=>vec![("kind",text("picture")),("mime",text(&v.mime_type)),("pictureType",Json::Number(u8::from(v.picture_type).into())),("description",text(&v.description)),("payload",bytes(&v.data))],
            id3::Content::Unknown(v)=>vec![("kind",text("opaque")),("bytes",bytes(&v.data))],
            other=>return Err(format!("unsupported semantic oracle frame {other:?}")),
        };Ok(Json::Object(fields.into_iter().map(|(k,v)|(k.into(),v)).collect()))
    }
    fn metadata_frames_json(frames:&[MetadataFrame])->Result<Json,String>{Ok(Json::Array(frames.iter().map(|frame|Ok(Json::Object(vec![("id".into(),Json::String(frame.id.clone())),("content".into(),content_json(&frame.content)?)]))).collect::<Result<_,String>>()?))}
    /// 🎯️ The projection `semantic-mp3-mpeg1-layer3-v1` compares. ID3v2 padding, the synchsafe size
    /// field and the flags byte are writer freedom and are not projected at all.
    pub(super) fn project(input: &[u8]) -> Result<Json, String> {
        let regions = split(input)?;
        let v2 = match read_v2(&regions.v2)? {
            None => Json::Null,
            Some((_version, frames)) => Json::Object(vec![("frames".to_string(), metadata_frames_json(&frames)?)]),
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
                ("genre".to_string(), if fields.genre_id==255{Json::Null}else{Json::Number(f64::from(fields.genre_id))}),
                ("track".to_string(),fields.track.map_or(Json::Null,|v|Json::Number(v.into()))),
            ]),
        };
        Ok(Json::Object(vec![("id3v2".to_string(), v2), ("frames".to_string(), audio), ("id3v1".to_string(), v1)]))
    }
    //#endregion 🔖️Projection
}
//#endregion 🔖️Layers

//#region 🔖️WireReaders
/// 🔎️ A byte array member of a wire value. An absent member is the empty default the vocabulary declares.
#[cfg(feature = "oracles")]
fn wire_bytes(value: &Json, key: &str) -> Result<Vec<u8>, String> {
    match value.get(key) {
        None => Ok(Vec::new()),
        Some(Json::Array(items)) => items.iter().map(|item| match item { Json::Number(n) if (0.0..=255.0).contains(n) && n.fract() == 0.0 => Ok(*n as u8), other => Err(format!("`{key}` carries {} where a byte belongs", other.to_string())) }).collect(),
        Some(other) => Err(format!("`{key}` must be a byte array, not {}", other.to_string())),
    }
}

/// 🔎️ A small unsigned integer member of a wire value.
#[cfg(feature = "oracles")]
fn wire_u8(value: &Json, key: &str) -> Result<u8, String> {
    match value.get(key) {
        Some(Json::Number(n)) if (0.0..=255.0).contains(n) && n.fract() == 0.0 => Ok(*n as u8),
        other => Err(format!("`{key}` must be an integer 0..=255, not {}", other.map(Json::to_string).unwrap_or_else(|| "nothing".to_string()))),
    }
}

/// 🔎️ A boolean member of a wire value.
#[cfg(feature = "oracles")]
fn wire_bool(value: &Json, key: &str) -> Result<bool, String> {
    match value.get(key) {
        Some(Json::Bool(flag)) => Ok(*flag),
        other => Err(format!("`{key}` must be a boolean, not {}", other.map(Json::to_string).unwrap_or_else(|| "nothing".to_string()))),
    }
}

//#endregion 🔖️WireReaders

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
/// 🏷️ The leading ID3v2 region an `Id3v2Tag` wire value (or `null`, no tag at all) describes, written by `id3`.
#[cfg(feature = "oracles")]
fn id3v2_region(tag: Option<&Json>) -> Result<Vec<u8>, String> {
    let Some(tag @ Json::Object(_)) = tag else { return Ok(Vec::new()) };
    let mut frames=Vec::new();for frame in tag.array("frames"){let id=frame.str("id");let c=frame.get("content").ok_or("metadata content")?;let values=||->Result<String,String>{let rows=c.array("values");if rows.is_empty(){return Err("metadata values".into());}rows.iter().map(|v|match v{Json::String(s)=>Ok(s.clone()),_=>Err("metadata text value".into())}).collect::<Result<Vec<_>,String>>().map(|v|v.join("\0"))};
    let content=match c.str("kind").as_str(){
        "text"=>id3::Content::Text(values()?),"userText"=>id3::Content::ExtendedText(id3::frame::ExtendedText{description:c.str("description"),value:values()?}),
        "url"=>id3::Content::Link(c.str("url")),"userUrl"=>id3::Content::ExtendedLink(id3::frame::ExtendedLink{description:c.str("description"),link:c.str("url")}),
        "comment"=>id3::Content::Comment(id3::frame::Comment{lang:c.str("language"),description:c.str("description"),text:c.str("text")}),
        "lyrics"=>id3::Content::Lyrics(id3::frame::Lyrics{lang:c.str("language"),description:c.str("description"),text:c.str("text")}),
        "picture"=>id3::Content::Picture(id3::frame::Picture{mime_type:c.str("mime"),picture_type:id3::frame::PictureType::Undefined(wire_u8(c,"pictureType")?),description:c.str("description"),data:wire_bytes(c,"payload")?}),
        "opaque"=>id3::Content::Unknown(id3::frame::Unknown{data:wire_bytes(c,"bytes")?,version:id3::Version::Id3v24}),other=>return Err(format!("metadata variant {other}")),
    };frames.push(layers::MetadataFrame{id,content});}layers::write_v2(id3::Version::Id3v24,&frames)
}

/// 🎼️ The audio region an `Mp3Frame` wire list describes: each frame's four header bytes packed from its typed
/// fields per ISO/IEC 11172-3 §2.4.1.3, followed by its payload. The walk then re-derives every frame size.
#[cfg(feature = "oracles")]
fn audio_region(frames: Option<&Json>) -> Result<Vec<u8>, String> {
    let frames = match frames {
        None => Vec::new(),
        Some(Json::Array(items)) => items.clone(),
        Some(other) => return Err(format!("`frames` must be an array, not {}", other.to_string())),
    };
    let mut audio = Vec::new();
    for frame in &frames {
        let header = frame.get("header").ok_or_else(|| "an MPEG frame carries no header".to_string())?;
        audio.extend_from_slice(&layers::pack_header(&layers::HeaderFields {
            mpeg_version_id: wire_u8(header, "mpegVersionId")?,
            layer: wire_u8(header, "layer")?,
            protection_bit: wire_bool(header, "protectionBit")?,
            bitrate_index: wire_u8(header, "bitrateIndex")?,
            sample_rate_index: wire_u8(header, "sampleRateIndex")?,
            padding: wire_bool(header, "padding")?,
            private_bit: wire_bool(header, "privateBit")?,
            channel_mode: wire_u8(header, "channelMode")?,
            mode_extension: wire_u8(header, "modeExtension")?,
            copyright: wire_bool(header, "copyright")?,
            original: wire_bool(header, "original")?,
            emphasis: wire_u8(header, "emphasis")?,
        })?);
        audio.extend_from_slice(&wire_bytes(frame, "payload")?);
    }
    layers::walk(&audio)?;
    Ok(audio)
}

/// 🏷️ Writes named semantic ID3v1 metadata with the independent fixed-width writer.
#[cfg(feature="oracles")]
fn id3v1_region(tag:Option<&Json>)->Result<Vec<u8>,String>{let Some(tag @ Json::Object(_))=tag else{return Ok(Vec::new())};let optional=|key|match tag.get(key){Some(Json::Null)=>Ok(None),Some(_)=>wire_u8(tag,key).map(Some),None=>Err(format!("missing {key}"))};layers::write_v1(&layers::V1Fields{title:tag.str("title"),artist:tag.str("artist"),album:tag.str("album"),year:tag.str("year"),comment:tag.str("comment"),track:optional("track")?,genre_id:optional("genre")?.unwrap_or(255)})}

/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes. `params` is the
/// leaf's wire payload (`payload_value()`), and the three layers are addressed independently, exactly as
/// `Mp3Mutation` addresses `id3v2`/`frames`/`id3v1`; an absent or `null` tag removes that layer. An unrecognised
/// kind is an error, never a silent no-op: a mutation that is quietly skipped reports as a passing test.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let params = spec.get("params").cloned().unwrap_or(Json::Null);
    let mut regions = layers::split(input)?;
    match spec.str("kind").as_str() {
        "set-id3v2" => regions.v2 = id3v2_region(params.get("id3v2"))?,
        "set-frames" => regions.audio = audio_region(params.get("frames"))?,
        "set-id3v1" => regions.v1 = id3v1_region(params.get("id3v1"))?,
        "" => return Err("mutation spec carries no `kind`".to_string()),
        other => return Err(format!("mutation kind {other:?} has no oracle implementation ({} input byte(s))", input.len())),
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
/// ↩️ The independently computed inverse of `spec`, applied on top of `mutated`: every variant of this vocabulary
/// is a whole-layer replace, so its inverse restores exactly the layer(s) it replaced from the UNMUTATED `base` —
/// `Mp3Mutation::inverse()`'s own base-relative semantics — and leaves the others as the forward mutation left them.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation_inverse(base: &[u8], spec: &Json, mutated: &[u8]) -> Result<Vec<u8>, String> {
    let original = layers::split(base)?;
    let mut restored = layers::split(mutated)?;
    match spec.str("kind").as_str() {
        "set-id3v2" => restored.v2 = original.v2,
        "set-frames" => restored.audio = original.audio,
        "set-id3v1" => restored.v1 = original.v1,
        other => return Err(format!("mutation kind {other:?} has no oracle inverse")),
    }
    Ok(layers::join(&restored))
}

/// 🚫️ Without the `oracles` feature the reference implementations are not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation_inverse(_base: &[u8], _spec: &Json, _mutated: &[u8]) -> Result<Vec<u8>, String> {
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
        Some((version, frames)) => layers::write_v2(version, &frames)?,
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
