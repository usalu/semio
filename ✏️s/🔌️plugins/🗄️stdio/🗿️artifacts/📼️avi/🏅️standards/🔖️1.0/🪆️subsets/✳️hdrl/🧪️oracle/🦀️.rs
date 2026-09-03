//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed
//! independently of this repository's own codec so the subject has something real to be compared
//! against instead of being checked against its own reading.
//!
//! Reference: `riff` 2.0 (the generic RIFF/`LIST`/chunk framing — id, size, even-byte padding,
//! nesting) composed with a hand-written AVI 1.0 `hdrl`/`strl`/`movi`/`idx1` codec (the `avih`/
//! `strh`/`strf` field layouts and the `idx1` keyframe-flag/offset convention, all written fresh
//! against the format's own public byte layout — see the "What `riff` provides / what this module
//! provides" note in `../🔣️oracle.json`'s `rationale`). No standalone Rust crate reads
//! AND writes AVI credibly: `avirus` 0.2.5 never parses a typed header at all and its "write" path
//! copies the whole `hdrl` section byte-for-byte (exactly the pass-through this platform forbids);
//! `rff-format-avi` 0.1.0's own doc comment says its muxer is "still scaffolded"; `oxideav-avi`
//! 0.0.9 has real AVI-specific read+write depth but zero independent adoption (created four months
//! before this ticket, 0 GitHub stars/forks, would also pull in an equally unvalidated
//! `oxideav-core`). `riff` itself is the credible, independent, and heavily used part (11.9M
//! downloads, MIT, created 2018) — the SAME composition shape `💬️bcf`'s oracle already established
//! for `zip`+`quick-xml` (no standalone BCF crate exists either, so it composes the generic archive
//! reader/writer with the generic XML reader/writer over BCF's own real shapes). Full investigation
//! and evidence: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/📓️w7-avi-1-0-mutate-report.md`.
//!
//! Every type, parser and writer below is a fresh, independent implementation — it never imports
//! this crate's own `AviSnapshot`/`AviMainHeader`/`AviStreamHeader`/`AviStreamFormat`/`AviChunk`/
//! `RiffChunk` types (see this crate's own purity gate). The AVI 1.0 byte layouts themselves
//! (`AVIMAINHEADER` 56 bytes, `AVISTREAMHEADER` 64 bytes, `BITMAPINFOHEADER`/`WAVEFORMATEX`, the
//! `idx1` `AVIIF_KEYFRAME` 0x10 flag and its movi-list-relative offset convention) are the format's
//! own public specification, not this repository's code — an independent reader and an independent
//! writer both have to agree with the SAME spec to be readers/writers of AVI at all.
//!
//! Real-fixture honesty: the committed `shared://🎬️.avi` carries real
//! auxiliary structure this subset's schema does not model at all — a `vprp` (video properties)
//! chunk and a 4120-byte `JUNK` padding chunk nested INSIDE its `strl`, plus a 260-byte `JUNK`
//! nested inside `hdrl` itself. Neither this module nor the production `decode_avi` represents
//! per-stream or per-`hdrl` auxiliary chunks in `AviSnapshot` at all (only `avih`, `strh`, `strf`,
//! `movi` chunks, `idx1`, and TOP-LEVEL unknown chunks have a modelled slot), so both codecs
//! silently drop that ~4.4 KB of real nested data on decode — a genuine schema-completeness gap,
//! not a bug either implementation introduces, and not hidden by loosening the projection: the
//! projection below only ever claims the fields the schema actually has slots for.
//!
//! A second, sharper real-fixture finding: the fixture's own `strh` is 56 bytes, not 64 —
//! `ffmpeg`'s AVI-1.0 muxer writes the classic `AVISTREAMHEADER` with `rcFrame` simply omitted, a
//! real, common, spec-legal producer behaviour. This module's [`parse_strh`] tolerates it (missing
//! trailing bytes default to zero, the same tolerance every real-world AVI reader needs). The
//! production `decode_avi` in `../../🚪️io/🦀️.rs` requires exactly 64 bytes and returns
//! `Err("avi: strh shorter than 64 bytes")` on this real file — a genuine pre-existing subject-side
//! gap this real fixture exposes, not introduced here, and not one this oracle module can or should
//! paper over by weakening its own tolerance to match. It stays visible: the `sut`-gated subject
//! handlers in the case's own `🦀️.rs` will fail at `decode_avi` the moment the subject
//! phase compiles, exactly the outcome wave 7's TIFF/BMP precedents already established as correct.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared family modules rather than by copying it. AVI has no shared family helper (unlike
//! `document`/`raster`/`archive`/...): the `riff` composition it needs is specific to this one
//! subset's container shape, so it lives here rather than in a shared module.
//!
//! Two entry points mirror the `💬️bcf`/`📰️xml`/`🎨️svg` precedent: [`oracle_apply_mutation`] performs
//! the FORWARD mutation (the `mutate-<kind>` scenarios), [`oracle_apply_mutation_inverse`] performs
//! the forward mutation and then its computed inverse in sequence (the `inverse-<kind>` scenarios) —
//! the same "apply, then apply the inverse, land back on the start" law `AviMutation::inverse`
//! proves at the Rust-model level, proven here independently against `riff` instead.
//! [`project_avi_1_0`] is the shared independent-reader projection both this module's own handlers
//! AND the case's subject handlers read their results back through before comparison.
//!
//! Binary payloads (a `movi` chunk's data, an unknown top-level chunk's data, `strf`'s `extra`
//! bytes) travel through mutation params as lowercase hex — the same convention `💬️bcf`'s oracle and
//! `AviSnapshot::parse_dsl`/`print_dsl` already use for binary-in-text.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself (`AviMutation::KINDS`).

use semio_repo_test_host::Json;

#[cfg(feature = "oracles")]
//#region 🔖️Oracles
mod oracles {
    use riff::{Chunk, ChunkContents, ChunkId, LIST_ID, RIFF_ID};
    use semio_repo_test_host::{digest, Json};
    use std::io::Cursor;

    //#region 🔖️Model
    /// 🏷️ `avih` — the independent mirror of `AVIMAINHEADER`, unrelated to this crate's own
    /// `AviMainHeader`. All 14 DWORDs, `dwReserved[4]` verbatim.
    #[derive(Clone, Debug, Default, PartialEq)]
    struct OMainHeader {
        micro_sec_per_frame: u32,
        max_bytes_per_sec: u32,
        padding_granularity: u32,
        flags: u32,
        total_frames: u32,
        initial_frames: u32,
        streams: u32,
        suggested_buffer_size: u32,
        width: u32,
        height: u32,
        reserved: [u32; 4],
    }

    /// 🏷️ `strh` — `AVISTREAMHEADER`, 64 bytes (`rcFrame` is 4 `LONG`s).
    #[derive(Clone, Debug, Default, PartialEq)]
    struct OStreamHeader {
        fcc_type: String,
        fcc_handler: String,
        flags: u32,
        priority: u16,
        language: u16,
        initial_frames: u32,
        scale: u32,
        rate: u32,
        start: u32,
        length: u32,
        suggested_buffer_size: u32,
        quality: i32,
        sample_size: u32,
        rc_frame_left: i32,
        rc_frame_top: i32,
        rc_frame_right: i32,
        rc_frame_bottom: i32,
    }

    /// 🎨️ `strf`, discriminated by the owning stream's `fccType` — the independent mirror of this
    /// crate's own `AviStreamFormat`.
    #[derive(Clone, Debug, PartialEq)]
    enum OStreamFormat {
        BitmapInfo { size: u32, width: i32, height: i32, planes: u16, bit_count: u16, compression: String, size_image: u32, x_pels_per_meter: i32, y_pels_per_meter: i32, colors_used: u32, colors_important: u32 },
        WaveFormat { format_tag: u16, channels: u16, samples_per_sec: u32, avg_bytes_per_sec: u32, block_align: u16, bits_per_sample: u16, extra: Vec<u8> },
        Raw { data: Vec<u8> },
    }

    impl Default for OStreamFormat {
        fn default() -> Self {
            Self::Raw { data: Vec::new() }
        }
    }

    /// 🎞️ One `movi` chunk belonging to a stream.
    #[derive(Clone, Debug, Default, PartialEq)]
    struct OChunk {
        fourcc: String,
        data: Vec<u8>,
        keyframe: bool,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    struct OStream {
        strh: OStreamHeader,
        strf: OStreamFormat,
        chunks: Vec<OChunk>,
    }

    /// 📦️ Typed-raw retention for a TOP-LEVEL RIFF child this subset's schema does not otherwise
    /// type (anything inside `AVI `'s body besides `hdrl`/`movi`/`idx1`) — a `LIST` of unknown type
    /// is tagged `"LIST:<type>"`, matching this crate's own `RiffChunk` convention so both sides
    /// agree on what "the same unknown chunk" means.
    #[derive(Clone, Debug, Default, PartialEq)]
    struct ORiffChunk {
        fourcc: String,
        data: Vec<u8>,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    struct ODoc {
        main_header: OMainHeader,
        streams: Vec<OStream>,
        idx1_present: bool,
        unknown_chunks: Vec<ORiffChunk>,
    }
    //#endregion 🔖️Model

    //#region 🔖️Hex
    /// 🔤️ Lowercase hex, the same binary-in-text convention `💬️bcf`'s oracle and `AviSnapshot`'s own
    /// DSL form already use.
    fn hex_encode(bytes: &[u8]) -> String {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    fn hex_decode(text: &str) -> Result<Vec<u8>, String> {
        if text.len() % 2 != 0 {
            return Err(format!("odd hex length ({} chars)", text.len()));
        }
        (0..text.len()).step_by(2).map(|i| u8::from_str_radix(&text[i..i + 2], 16).map_err(|error| format!("invalid hex {:?}: {error}", &text[i..i + 2]))).collect()
    }
    //#endregion 🔖️Hex

    //#region 🔖️Fourcc
    fn fourcc_str(id: ChunkId) -> String {
        String::from_utf8_lossy(&id.value).into_owned()
    }

    /// 🔤️ Pads/truncates to exactly 4 bytes with spaces — the same convention every real AVI writer
    /// (and this crate's own `AviSnapshot` codec) uses for a fourcc shorter than 4 characters.
    fn fourcc4(text: &str) -> [u8; 4] {
        let mut out = [b' '; 4];
        for (index, byte) in text.as_bytes().iter().take(4).enumerate() {
            out[index] = *byte;
        }
        out
    }

    fn cid(text: &str) -> ChunkId {
        ChunkId { value: fourcc4(text) }
    }
    //#endregion 🔖️Fourcc

    //#region 🔖️FieldLayout
    /// 📐️ `AVIMAINHEADER`/`AVISTREAMHEADER`/`BITMAPINFOHEADER`/`WAVEFORMATEX` — the format's own
    /// public little-endian byte layout, written fresh (never read from this repository's own
    /// `🚪️io/🦀️.rs`). <https://learn.microsoft.com/windows/win32/directshow/avimainheader>
    /// <https://learn.microsoft.com/windows/win32/api/wingdi/ns-wingdi-bitmapinfoheader>
    fn u32le(bytes: &[u8], offset: usize) -> u32 {
        u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
    }
    fn i32le(bytes: &[u8], offset: usize) -> i32 {
        i32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
    }
    fn u16le(bytes: &[u8], offset: usize) -> u16 {
        u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap())
    }

    fn parse_avih(payload: &[u8]) -> Result<OMainHeader, String> {
        if payload.len() < 56 {
            return Err(format!("avi: avih is {} byte(s), need 56", payload.len()));
        }
        Ok(OMainHeader {
            micro_sec_per_frame: u32le(payload, 0),
            max_bytes_per_sec: u32le(payload, 4),
            padding_granularity: u32le(payload, 8),
            flags: u32le(payload, 12),
            total_frames: u32le(payload, 16),
            initial_frames: u32le(payload, 20),
            streams: u32le(payload, 24),
            suggested_buffer_size: u32le(payload, 28),
            width: u32le(payload, 32),
            height: u32le(payload, 36),
            reserved: [u32le(payload, 40), u32le(payload, 44), u32le(payload, 48), u32le(payload, 52)],
        })
    }

    fn write_avih(header: &OMainHeader) -> Vec<u8> {
        let mut out = Vec::with_capacity(56);
        for value in [header.micro_sec_per_frame, header.max_bytes_per_sec, header.padding_granularity, header.flags, header.total_frames, header.initial_frames, header.streams, header.suggested_buffer_size, header.width, header.height] {
            out.extend_from_slice(&value.to_le_bytes());
        }
        for value in header.reserved {
            out.extend_from_slice(&value.to_le_bytes());
        }
        out
    }

    /// 📐️ Accepts a `strh` as short as 56 bytes — the classic (pre-OpenDML, `RECT rcFrame` simply
    /// omitted) `AVISTREAMHEADER` real encoders including `ffmpeg`'s own AVI-1.0 muxer still write,
    /// confirmed against the real committed fixture (its own `strh` IS 56 bytes, not 64) — treating
    /// any missing trailing `rcFrame` field as zero, the same tolerance every real-world AVI reader
    /// has to have. Production's own `decode_avi` requires exactly 64 and rejects this real file;
    /// see the module doc comment's honesty note.
    fn parse_strh(payload: &[u8]) -> Result<OStreamHeader, String> {
        if payload.len() < 56 {
            return Err(format!("avi: strh is {} byte(s), need at least 56", payload.len()));
        }
        let rc = |offset: usize| if payload.len() >= offset + 4 { i32le(payload, offset) } else { 0 };
        Ok(OStreamHeader {
            fcc_type: String::from_utf8_lossy(&payload[0..4]).into_owned(),
            fcc_handler: String::from_utf8_lossy(&payload[4..8]).into_owned(),
            flags: u32le(payload, 8),
            priority: u16le(payload, 12),
            language: u16le(payload, 14),
            initial_frames: u32le(payload, 16),
            scale: u32le(payload, 20),
            rate: u32le(payload, 24),
            start: u32le(payload, 28),
            length: u32le(payload, 32),
            suggested_buffer_size: u32le(payload, 36),
            quality: i32le(payload, 40),
            sample_size: u32le(payload, 44),
            rc_frame_left: rc(48),
            rc_frame_top: rc(52),
            rc_frame_right: rc(56),
            rc_frame_bottom: rc(60),
        })
    }

    /// ✍️ Always emits the full 64-byte form (`rcFrame` included, zero if it was never present on
    /// decode) — normalizing a short real-world `strh` to the complete struct on write, the same
    /// kind of documented-normal-form choice `AviSnapshot`'s own encode already makes elsewhere.
    fn write_strh(header: &OStreamHeader) -> Vec<u8> {
        let mut out = Vec::with_capacity(64);
        out.extend_from_slice(&fourcc4(&header.fcc_type));
        out.extend_from_slice(&fourcc4(&header.fcc_handler));
        out.extend_from_slice(&header.flags.to_le_bytes());
        out.extend_from_slice(&header.priority.to_le_bytes());
        out.extend_from_slice(&header.language.to_le_bytes());
        out.extend_from_slice(&header.initial_frames.to_le_bytes());
        out.extend_from_slice(&header.scale.to_le_bytes());
        out.extend_from_slice(&header.rate.to_le_bytes());
        out.extend_from_slice(&header.start.to_le_bytes());
        out.extend_from_slice(&header.length.to_le_bytes());
        out.extend_from_slice(&header.suggested_buffer_size.to_le_bytes());
        out.extend_from_slice(&header.quality.to_le_bytes());
        out.extend_from_slice(&header.sample_size.to_le_bytes());
        for value in [header.rc_frame_left, header.rc_frame_top, header.rc_frame_right, header.rc_frame_bottom] {
            out.extend_from_slice(&value.to_le_bytes());
        }
        out
    }

    fn parse_strf(fcc_type: &str, payload: &[u8]) -> OStreamFormat {
        if fcc_type == "vids" && payload.len() >= 40 {
            return OStreamFormat::BitmapInfo {
                size: u32le(payload, 0),
                width: i32le(payload, 4),
                height: i32le(payload, 8),
                planes: u16le(payload, 12),
                bit_count: u16le(payload, 14),
                compression: String::from_utf8_lossy(&payload[16..20]).into_owned(),
                size_image: u32le(payload, 20),
                x_pels_per_meter: i32le(payload, 24),
                y_pels_per_meter: i32le(payload, 28),
                colors_used: u32le(payload, 32),
                colors_important: u32le(payload, 36),
            };
        }
        if fcc_type == "auds" && payload.len() >= 16 {
            return OStreamFormat::WaveFormat {
                format_tag: u16le(payload, 0),
                channels: u16le(payload, 2),
                samples_per_sec: u32le(payload, 4),
                avg_bytes_per_sec: u32le(payload, 8),
                block_align: u16le(payload, 12),
                bits_per_sample: u16le(payload, 14),
                extra: payload.get(16..).map(|slice| slice.to_vec()).unwrap_or_default(),
            };
        }
        OStreamFormat::Raw { data: payload.to_vec() }
    }

    fn write_strf(format: &OStreamFormat) -> Vec<u8> {
        match format {
            OStreamFormat::BitmapInfo { size, width, height, planes, bit_count, compression, size_image, x_pels_per_meter, y_pels_per_meter, colors_used, colors_important } => {
                let mut out = Vec::with_capacity(40);
                out.extend_from_slice(&size.to_le_bytes());
                out.extend_from_slice(&width.to_le_bytes());
                out.extend_from_slice(&height.to_le_bytes());
                out.extend_from_slice(&planes.to_le_bytes());
                out.extend_from_slice(&bit_count.to_le_bytes());
                out.extend_from_slice(&fourcc4(compression));
                out.extend_from_slice(&size_image.to_le_bytes());
                out.extend_from_slice(&x_pels_per_meter.to_le_bytes());
                out.extend_from_slice(&y_pels_per_meter.to_le_bytes());
                out.extend_from_slice(&colors_used.to_le_bytes());
                out.extend_from_slice(&colors_important.to_le_bytes());
                out
            }
            OStreamFormat::WaveFormat { format_tag, channels, samples_per_sec, avg_bytes_per_sec, block_align, bits_per_sample, extra } => {
                let mut out = Vec::with_capacity(16 + extra.len());
                out.extend_from_slice(&format_tag.to_le_bytes());
                out.extend_from_slice(&channels.to_le_bytes());
                out.extend_from_slice(&samples_per_sec.to_le_bytes());
                out.extend_from_slice(&avg_bytes_per_sec.to_le_bytes());
                out.extend_from_slice(&block_align.to_le_bytes());
                out.extend_from_slice(&bits_per_sample.to_le_bytes());
                out.extend_from_slice(extra);
                out
            }
            OStreamFormat::Raw { data } => data.clone(),
        }
    }
    //#endregion 🔖️FieldLayout

    //#region 🔖️Riff
    /// 📥️ Real RIFF/AVI decode, the chunk framing walked entirely through `riff::Chunk` (never this
    /// repository's own hand-rolled chunk walker): `hdrl` (`avih` + every `strl`'s `strh`/`strf`),
    /// `movi` (every chunk, assigned to its owning stream by the leading 2-digit stream number in
    /// its fourcc — the format's own numbering convention), `idx1` (positionally matched to `movi`
    /// chunks for the `AVIIF_KEYFRAME` 0x10 flag; absent/mismatched index ⇒ every chunk is a sync
    /// point, the same fallback the format's own spec and this subset's schema both document).
    /// Nested `hdrl`/`strl` auxiliary children (`JUNK` padding, `vprp`) have no slot in this
    /// subset's schema at all and are dropped — see the module doc comment's honesty note.
    fn decode(bytes: &[u8]) -> Result<ODoc, String> {
        let riff_chunk = Chunk::read(&mut Cursor::new(bytes), 0).map_err(|error| format!("riff: {error}"))?;
        if riff_chunk.id() != RIFF_ID {
            return Err("avi: missing RIFF magic".to_string());
        }
        let form = riff_chunk.read_type(&mut Cursor::new(bytes)).map_err(|error| error.to_string())?;
        if fourcc_str(form) != "AVI " {
            return Err(format!("avi: RIFF form is {:?}, not \"AVI \"", fourcc_str(form)));
        }

        let mut main_header = None;
        let mut streams: Vec<OStream> = Vec::new();
        let mut movi_chunks: Vec<(String, Vec<u8>)> = Vec::new();
        let mut idx1_present = false;
        let mut idx1_flags: Vec<u32> = Vec::new();
        let mut unknown_chunks: Vec<ORiffChunk> = Vec::new();

        let top_children: Vec<Chunk> = riff_chunk.iter(&mut Cursor::new(bytes)).collect::<std::io::Result<_>>().map_err(|error| error.to_string())?;
        for child in top_children {
            if child.id() == LIST_ID {
                let list_type = child.read_type(&mut Cursor::new(bytes)).map_err(|error| error.to_string())?;
                match fourcc_str(list_type).as_str() {
                    "hdrl" => {
                        let hdrl_children: Vec<Chunk> = child.iter(&mut Cursor::new(bytes)).collect::<std::io::Result<_>>().map_err(|error| error.to_string())?;
                        for hdrl_child in hdrl_children {
                            if fourcc_str(hdrl_child.id()) == "avih" {
                                let payload = hdrl_child.read_contents(&mut Cursor::new(bytes)).map_err(|error| error.to_string())?;
                                main_header = Some(parse_avih(&payload)?);
                            } else if hdrl_child.id() == LIST_ID {
                                let sub_type = hdrl_child.read_type(&mut Cursor::new(bytes)).map_err(|error| error.to_string())?;
                                if fourcc_str(sub_type) == "strl" {
                                    let strl_children: Vec<Chunk> = hdrl_child.iter(&mut Cursor::new(bytes)).collect::<std::io::Result<_>>().map_err(|error| error.to_string())?;
                                    let mut strh = None;
                                    let mut strf_bytes: Option<Vec<u8>> = None;
                                    for strl_child in strl_children {
                                        match fourcc_str(strl_child.id()).as_str() {
                                            "strh" => strh = Some(parse_strh(&strl_child.read_contents(&mut Cursor::new(bytes)).map_err(|error| error.to_string())?)?),
                                            "strf" => strf_bytes = Some(strl_child.read_contents(&mut Cursor::new(bytes)).map_err(|error| error.to_string())?),
                                            _ => {} // 📦 JUNK/vprp/other strl auxiliaries — no slot in this subset's schema, see module doc
                                        }
                                    }
                                    let strh = strh.ok_or("avi: strl missing strh")?;
                                    let strf = parse_strf(&strh.fcc_type, &strf_bytes.ok_or("avi: strl missing strf")?);
                                    streams.push(OStream { strh, strf, chunks: Vec::new() });
                                }
                                // other nested LIST types inside hdrl: no slot in this subset's schema, dropped
                            }
                            // other hdrl children (e.g. JUNK padding): no slot in this subset's schema, dropped
                        }
                    }
                    "movi" => {
                        let movi_children: Vec<Chunk> = child.iter(&mut Cursor::new(bytes)).collect::<std::io::Result<_>>().map_err(|error| error.to_string())?;
                        for movi_child in movi_children {
                            let fourcc = fourcc_str(movi_child.id());
                            let payload = movi_child.read_contents(&mut Cursor::new(bytes)).map_err(|error| error.to_string())?;
                            movi_chunks.push((fourcc, payload));
                        }
                    }
                    other_type => {
                        let full_payload = child.read_contents(&mut Cursor::new(bytes)).map_err(|error| error.to_string())?;
                        unknown_chunks.push(ORiffChunk { fourcc: format!("LIST:{other_type}"), data: full_payload.get(4..).unwrap_or_default().to_vec() });
                    }
                }
            } else if fourcc_str(child.id()) == "idx1" {
                idx1_present = true;
                let payload = child.read_contents(&mut Cursor::new(bytes)).map_err(|error| error.to_string())?;
                let mut rest = payload.as_slice();
                while rest.len() >= 16 {
                    idx1_flags.push(u32::from_le_bytes(rest[4..8].try_into().unwrap()));
                    rest = &rest[16..];
                }
            } else {
                let payload = child.read_contents(&mut Cursor::new(bytes)).map_err(|error| error.to_string())?;
                unknown_chunks.push(ORiffChunk { fourcc: fourcc_str(child.id()), data: payload });
            }
        }

        let idx1_matches_by_position = idx1_present && idx1_flags.len() == movi_chunks.len();
        for (position, (fourcc, data)) in movi_chunks.into_iter().enumerate() {
            let stream_index: usize = fourcc.get(0..2).and_then(|slice| slice.parse().ok()).unwrap_or(0);
            let keyframe = if idx1_matches_by_position { idx1_flags[position] & 0x10 != 0 } else { true };
            match streams.get_mut(stream_index) {
                Some(stream) => stream.chunks.push(OChunk { fourcc, data, keyframe }),
                None => unknown_chunks.push(ORiffChunk { fourcc: format!("movi:{fourcc}"), data }),
            }
        }

        Ok(ODoc { main_header: main_header.ok_or("avi: hdrl missing avih")?, streams, idx1_present, unknown_chunks })
    }

    /// ✍️ Real RIFF/AVI encode, built entirely through `riff::ChunkContents::write` (never this
    /// repository's own hand-rolled chunk writer): `hdrl(avih + strl(strh,strf)*)`, `movi(chunk*)`,
    /// `idx1` (if present) with offsets relative to the `movi` LIST payload start INCLUDING its own
    /// `movi` tag — the documented normal form `AviSnapshot`'s own schema doc already commits to —
    /// then every retained unknown chunk. Nested `hdrl`/`strl` auxiliaries this subset's schema has
    /// no slot for are not reproduced (see module doc comment).
    fn encode(doc: &ODoc) -> Vec<u8> {
        let avih = ChunkContents::Data(cid("avih"), write_avih(&doc.main_header));
        let mut hdrl_children = vec![avih];
        for stream in &doc.streams {
            let strh = ChunkContents::Data(cid("strh"), write_strh(&stream.strh));
            let strf = ChunkContents::Data(cid("strf"), write_strf(&stream.strf));
            hdrl_children.push(ChunkContents::Children(LIST_ID, cid("strl"), vec![strh, strf]));
        }
        let hdrl = ChunkContents::Children(LIST_ID, cid("hdrl"), hdrl_children);

        let movi_children: Vec<ChunkContents> = doc.streams.iter().flat_map(|stream| stream.chunks.iter().map(|chunk| ChunkContents::Data(cid(&chunk.fourcc), chunk.data.clone()))).collect();
        let movi = ChunkContents::Children(LIST_ID, cid("movi"), movi_children);

        let mut top = vec![hdrl, movi];

        if doc.idx1_present {
            let mut payload = Vec::new();
            let mut offset = 4u32; // 🧭 relative to the movi LIST payload start, including its own "movi" tag.
            for stream in &doc.streams {
                for chunk in &stream.chunks {
                    payload.extend_from_slice(&fourcc4(&chunk.fourcc));
                    payload.extend_from_slice(&(if chunk.keyframe { 0x10u32 } else { 0 }).to_le_bytes());
                    payload.extend_from_slice(&offset.to_le_bytes());
                    payload.extend_from_slice(&(chunk.data.len() as u32).to_le_bytes());
                    offset += 8 + chunk.data.len() as u32 + (chunk.data.len() as u32 % 2);
                }
            }
            top.push(ChunkContents::Data(cid("idx1"), payload));
        }

        for unknown in &doc.unknown_chunks {
            if let Some(list_type) = unknown.fourcc.strip_prefix("LIST:") {
                let mut payload = fourcc4(list_type).to_vec();
                payload.extend_from_slice(&unknown.data);
                top.push(ChunkContents::Data(ChunkId { value: *b"LIST" }, payload));
            } else if !unknown.fourcc.starts_with("movi:") {
                top.push(ChunkContents::Data(cid(&unknown.fourcc), unknown.data.clone()));
            }
        }

        let riff = ChunkContents::Children(RIFF_ID, ChunkId { value: *b"AVI " }, top);
        let mut buffer = Cursor::new(Vec::new());
        riff.write(&mut buffer).expect("write to an in-memory Vec<u8> cannot fail");
        buffer.into_inner()
    }
    //#endregion 🔖️Riff

    //#region 🔖️JsonValue
    fn obj(entries: Vec<(&str, Json)>) -> Json {
        Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
    }

    fn num(value: &Json, key: &str) -> f64 {
        match value.get(key) {
            Some(Json::Number(number)) => *number,
            _ => 0.0,
        }
    }

    fn flag(value: &Json, key: &str) -> bool {
        matches!(value.get(key), Some(Json::Bool(true)))
    }

    fn index_of(params: &Json, key: &str) -> usize {
        num(params, key).max(0.0) as usize
    }

    fn main_header_to_json(header: &OMainHeader) -> Json {
        obj(vec![
            ("microSecPerFrame", Json::Number(header.micro_sec_per_frame as f64)),
            ("maxBytesPerSec", Json::Number(header.max_bytes_per_sec as f64)),
            ("paddingGranularity", Json::Number(header.padding_granularity as f64)),
            ("flags", Json::Number(header.flags as f64)),
            ("totalFrames", Json::Number(header.total_frames as f64)),
            ("initialFrames", Json::Number(header.initial_frames as f64)),
            ("streams", Json::Number(header.streams as f64)),
            ("suggestedBufferSize", Json::Number(header.suggested_buffer_size as f64)),
            ("width", Json::Number(header.width as f64)),
            ("height", Json::Number(header.height as f64)),
            ("reserved", Json::Array(header.reserved.iter().map(|value| Json::Number(*value as f64)).collect())),
        ])
    }

    fn main_header_from_json(value: &Json) -> OMainHeader {
        let mut reserved = [0u32; 4];
        for (index, entry) in value.array("reserved").iter().take(4).enumerate() {
            if let Json::Number(number) = entry {
                reserved[index] = *number as u32;
            }
        }
        OMainHeader {
            micro_sec_per_frame: num(value, "microSecPerFrame") as u32,
            max_bytes_per_sec: num(value, "maxBytesPerSec") as u32,
            padding_granularity: num(value, "paddingGranularity") as u32,
            flags: num(value, "flags") as u32,
            total_frames: num(value, "totalFrames") as u32,
            initial_frames: num(value, "initialFrames") as u32,
            streams: num(value, "streams") as u32,
            suggested_buffer_size: num(value, "suggestedBufferSize") as u32,
            width: num(value, "width") as u32,
            height: num(value, "height") as u32,
            reserved,
        }
    }

    fn strh_to_json(header: &OStreamHeader) -> Json {
        obj(vec![
            ("fccType", Json::String(header.fcc_type.clone())),
            ("fccHandler", Json::String(header.fcc_handler.clone())),
            ("flags", Json::Number(header.flags as f64)),
            ("priority", Json::Number(header.priority as f64)),
            ("language", Json::Number(header.language as f64)),
            ("initialFrames", Json::Number(header.initial_frames as f64)),
            ("scale", Json::Number(header.scale as f64)),
            ("rate", Json::Number(header.rate as f64)),
            ("start", Json::Number(header.start as f64)),
            ("length", Json::Number(header.length as f64)),
            ("suggestedBufferSize", Json::Number(header.suggested_buffer_size as f64)),
            ("quality", Json::Number(header.quality as f64)),
            ("sampleSize", Json::Number(header.sample_size as f64)),
            ("rcFrameLeft", Json::Number(header.rc_frame_left as f64)),
            ("rcFrameTop", Json::Number(header.rc_frame_top as f64)),
            ("rcFrameRight", Json::Number(header.rc_frame_right as f64)),
            ("rcFrameBottom", Json::Number(header.rc_frame_bottom as f64)),
        ])
    }

    fn strh_from_json(value: &Json) -> OStreamHeader {
        OStreamHeader {
            fcc_type: value.str("fccType"),
            fcc_handler: value.str("fccHandler"),
            flags: num(value, "flags") as u32,
            priority: num(value, "priority") as u16,
            language: num(value, "language") as u16,
            initial_frames: num(value, "initialFrames") as u32,
            scale: num(value, "scale") as u32,
            rate: num(value, "rate") as u32,
            start: num(value, "start") as u32,
            length: num(value, "length") as u32,
            suggested_buffer_size: num(value, "suggestedBufferSize") as u32,
            quality: num(value, "quality") as i32,
            sample_size: num(value, "sampleSize") as u32,
            rc_frame_left: num(value, "rcFrameLeft") as i32,
            rc_frame_top: num(value, "rcFrameTop") as i32,
            rc_frame_right: num(value, "rcFrameRight") as i32,
            rc_frame_bottom: num(value, "rcFrameBottom") as i32,
        }
    }

    /// 🎨️ `{"format": "bitmapInfo"|"waveFormat"|"raw", ...}` — the same `#[serde(tag = "format",
    /// rename_all = "camelCase")]` spelling this crate's own `AviStreamFormat` would serialize to,
    /// kept here as a plain string match rather than an actual `serde` dependency on that type.
    fn strf_to_json(format: &OStreamFormat) -> Json {
        match format {
            OStreamFormat::BitmapInfo { size, width, height, planes, bit_count, compression, size_image, x_pels_per_meter, y_pels_per_meter, colors_used, colors_important } => obj(vec![
                ("format", Json::String("bitmapInfo".to_string())),
                ("size", Json::Number(*size as f64)),
                ("width", Json::Number(*width as f64)),
                ("height", Json::Number(*height as f64)),
                ("planes", Json::Number(*planes as f64)),
                ("bitCount", Json::Number(*bit_count as f64)),
                ("compression", Json::String(compression.clone())),
                ("sizeImage", Json::Number(*size_image as f64)),
                ("xPelsPerMeter", Json::Number(*x_pels_per_meter as f64)),
                ("yPelsPerMeter", Json::Number(*y_pels_per_meter as f64)),
                ("colorsUsed", Json::Number(*colors_used as f64)),
                ("colorsImportant", Json::Number(*colors_important as f64)),
            ]),
            OStreamFormat::WaveFormat { format_tag, channels, samples_per_sec, avg_bytes_per_sec, block_align, bits_per_sample, extra } => obj(vec![
                ("format", Json::String("waveFormat".to_string())),
                ("formatTag", Json::Number(*format_tag as f64)),
                ("channels", Json::Number(*channels as f64)),
                ("samplesPerSec", Json::Number(*samples_per_sec as f64)),
                ("avgBytesPerSec", Json::Number(*avg_bytes_per_sec as f64)),
                ("blockAlign", Json::Number(*block_align as f64)),
                ("bitsPerSample", Json::Number(*bits_per_sample as f64)),
                ("extra", Json::String(hex_encode(extra))),
            ]),
            OStreamFormat::Raw { data } => obj(vec![("format", Json::String("raw".to_string())), ("data", Json::String(hex_encode(data)))]),
        }
    }

    fn strf_from_json(value: &Json) -> Result<OStreamFormat, String> {
        match value.str("format").as_str() {
            "bitmapInfo" => Ok(OStreamFormat::BitmapInfo {
                size: num(value, "size") as u32,
                width: num(value, "width") as i32,
                height: num(value, "height") as i32,
                planes: num(value, "planes") as u16,
                bit_count: num(value, "bitCount") as u16,
                compression: value.str("compression"),
                size_image: num(value, "sizeImage") as u32,
                x_pels_per_meter: num(value, "xPelsPerMeter") as i32,
                y_pels_per_meter: num(value, "yPelsPerMeter") as i32,
                colors_used: num(value, "colorsUsed") as u32,
                colors_important: num(value, "colorsImportant") as u32,
            }),
            "waveFormat" => Ok(OStreamFormat::WaveFormat {
                format_tag: num(value, "formatTag") as u16,
                channels: num(value, "channels") as u16,
                samples_per_sec: num(value, "samplesPerSec") as u32,
                avg_bytes_per_sec: num(value, "avgBytesPerSec") as u32,
                block_align: num(value, "blockAlign") as u16,
                bits_per_sample: num(value, "bitsPerSample") as u16,
                extra: match value.get("extra") {
                    Some(Json::String(hex)) if !hex.is_empty() => hex_decode(hex)?,
                    _ => Vec::new(),
                },
            }),
            "raw" => Ok(OStreamFormat::Raw {
                data: match value.get("data") {
                    Some(Json::String(hex)) if !hex.is_empty() => hex_decode(hex)?,
                    _ => Vec::new(),
                },
            }),
            other => Err(format!("unknown strf format {other:?}")),
        }
    }

    fn chunk_to_json(chunk: &OChunk) -> Json {
        obj(vec![("fourcc", Json::String(chunk.fourcc.clone())), ("data", Json::String(hex_encode(&chunk.data))), ("keyframe", Json::Bool(chunk.keyframe))])
    }

    fn chunk_from_json(value: &Json) -> OChunk {
        OChunk {
            fourcc: value.str("fourcc"),
            data: match value.get("data") {
                Some(Json::String(hex)) if !hex.is_empty() => hex_decode(hex).unwrap_or_default(),
                _ => Vec::new(),
            },
            keyframe: flag(value, "keyframe"),
        }
    }

    fn riff_chunk_to_json(item: &ORiffChunk) -> Json {
        obj(vec![("fourcc", Json::String(item.fourcc.clone())), ("data", Json::String(hex_encode(&item.data)))])
    }

    fn riff_chunk_from_json(value: &Json) -> ORiffChunk {
        ORiffChunk {
            fourcc: value.str("fourcc"),
            data: match value.get("data") {
                Some(Json::String(hex)) if !hex.is_empty() => hex_decode(hex).unwrap_or_default(),
                _ => Vec::new(),
            },
        }
    }

    fn stream_to_json(stream: &OStream) -> Json {
        obj(vec![("strh", strh_to_json(&stream.strh)), ("strf", strf_to_json(&stream.strf)), ("chunks", Json::Array(stream.chunks.iter().map(chunk_to_json).collect()))])
    }

    fn stream_from_json(value: &Json) -> Result<OStream, String> {
        Ok(OStream { strh: strh_from_json(&value.get("strh").cloned().unwrap_or(Json::Null)), strf: strf_from_json(&value.get("strf").cloned().unwrap_or(Json::Null))?, chunks: value.array("chunks").iter().map(chunk_from_json).collect() })
    }

    fn doc_to_json(doc: &ODoc) -> Json {
        obj(vec![
            ("mainHeader", main_header_to_json(&doc.main_header)),
            ("streams", Json::Array(doc.streams.iter().map(stream_to_json).collect())),
            ("idx1Present", Json::Bool(doc.idx1_present)),
            ("unknownChunks", Json::Array(doc.unknown_chunks.iter().map(riff_chunk_to_json).collect())),
        ])
    }

    fn doc_from_json(value: &Json) -> Result<ODoc, String> {
        Ok(ODoc {
            main_header: main_header_from_json(&value.get("mainHeader").cloned().unwrap_or(Json::Null)),
            streams: value.array("streams").iter().map(stream_from_json).collect::<Result<_, _>>()?,
            idx1_present: flag(value, "idx1Present"),
            unknown_chunks: value.array("unknownChunks").iter().map(riff_chunk_from_json).collect(),
        })
    }
    //#endregion 🔖️JsonValue

    //#region 🔖️Forward
    /// 🦠️ Applies one declared mutation kind, described by `spec` (`{"kind": ..., "params": {...}}`),
    /// to an already-decoded document. An unrecognised kind, or an out-of-range stream/chunk/
    /// unknown-chunk index, is an error — never a silent no-op.
    fn apply_kind(doc: &mut ODoc, kind: &str, params: &Json) -> Result<(), String> {
        match kind {
            "no-mutation" => {}
            "set-snapshot" => *doc = doc_from_json(params)?,
            "set-main-header" => doc.main_header = main_header_from_json(&params.get("mainHeader").cloned().unwrap_or(Json::Null)),
            "set-idx1-present" => doc.idx1_present = flag(params, "idx1Present"),
            "insert-stream" => {
                let index = index_of(params, "index");
                let stream = stream_from_json(&params.get("stream").cloned().unwrap_or(Json::Null))?;
                if index > doc.streams.len() {
                    return Err(format!("insert-stream: index {index} out of bounds for {} stream(s)", doc.streams.len()));
                }
                doc.streams.insert(index, stream);
            }
            "remove-stream" => {
                let index = index_of(params, "index");
                if index >= doc.streams.len() {
                    return Err(format!("remove-stream: no stream at index {index}"));
                }
                doc.streams.remove(index);
            }
            "set-stream-header" => {
                let index = index_of(params, "streamIndex");
                let strh = strh_from_json(&params.get("strh").cloned().unwrap_or(Json::Null));
                doc.streams.get_mut(index).ok_or_else(|| format!("set-stream-header: no stream at index {index}"))?.strh = strh;
            }
            "set-stream-format" => {
                let index = index_of(params, "streamIndex");
                let strf = strf_from_json(&params.get("strf").cloned().unwrap_or(Json::Null))?;
                doc.streams.get_mut(index).ok_or_else(|| format!("set-stream-format: no stream at index {index}"))?.strf = strf;
            }
            "insert-chunk" => {
                let stream_index = index_of(params, "streamIndex");
                let index = index_of(params, "index");
                let chunk = chunk_from_json(&params.get("chunk").cloned().unwrap_or(Json::Null));
                let stream = doc.streams.get_mut(stream_index).ok_or_else(|| format!("insert-chunk: no stream at index {stream_index}"))?;
                if index > stream.chunks.len() {
                    return Err(format!("insert-chunk: index {index} out of bounds for {} chunk(s)", stream.chunks.len()));
                }
                stream.chunks.insert(index, chunk);
            }
            "remove-chunk" => {
                let stream_index = index_of(params, "streamIndex");
                let index = index_of(params, "index");
                let stream = doc.streams.get_mut(stream_index).ok_or_else(|| format!("remove-chunk: no stream at index {stream_index}"))?;
                if index >= stream.chunks.len() {
                    return Err(format!("remove-chunk: no chunk at index {index}"));
                }
                stream.chunks.remove(index);
            }
            "set-chunk-keyframe" => {
                let stream_index = index_of(params, "streamIndex");
                let index = index_of(params, "index");
                let keyframe = flag(params, "keyframe");
                let stream = doc.streams.get_mut(stream_index).ok_or_else(|| format!("set-chunk-keyframe: no stream at index {stream_index}"))?;
                stream.chunks.get_mut(index).ok_or_else(|| format!("set-chunk-keyframe: no chunk at index {index}"))?.keyframe = keyframe;
            }
            "add-unknown-chunk" => {
                let index = index_of(params, "index");
                let item = riff_chunk_from_json(&params.get("item").cloned().unwrap_or(Json::Null));
                if index > doc.unknown_chunks.len() {
                    return Err(format!("add-unknown-chunk: index {index} out of bounds for {} chunk(s)", doc.unknown_chunks.len()));
                }
                doc.unknown_chunks.insert(index, item);
            }
            "remove-unknown-chunk" => {
                let index = index_of(params, "index");
                if index >= doc.unknown_chunks.len() {
                    return Err(format!("remove-unknown-chunk: no unknown chunk at index {index}"));
                }
                doc.unknown_chunks.remove(index);
            }
            other => return Err(format!("mutation kind {other:?} has no oracle implementation")),
        }
        Ok(())
    }
    //#endregion 🔖️Forward

    //#region 🔖️Inverse
    /// ↩️ Reads `base` (the CURRENT, pre-mutation document) to build the spec that undoes `{kind,
    /// params}` — same law `AviMutation::inverse` proves at the Rust-model level, computed here
    /// against `riff` instead.
    fn inverse_spec(base: &ODoc, kind: &str, params: &Json) -> Json {
        let spec = |inverse_kind: &str, inverse_params: Json| obj(vec![("kind", Json::String(inverse_kind.to_string())), ("params", inverse_params)]);
        match kind {
            "no-mutation" => spec("no-mutation", obj(vec![])),
            "set-snapshot" => spec("set-snapshot", doc_to_json(base)),
            "set-main-header" => spec("set-main-header", obj(vec![("mainHeader", main_header_to_json(&base.main_header))])),
            "set-idx1-present" => spec("set-idx1-present", obj(vec![("idx1Present", Json::Bool(base.idx1_present))])),
            "insert-stream" => spec("remove-stream", obj(vec![("index", Json::Number(index_of(params, "index") as f64))])),
            "remove-stream" => {
                let index = index_of(params, "index");
                match base.streams.get(index) {
                    Some(stream) => spec("insert-stream", obj(vec![("index", Json::Number(index as f64)), ("stream", stream_to_json(stream))])),
                    None => spec("no-mutation", obj(vec![])),
                }
            }
            "set-stream-header" => {
                let index = index_of(params, "streamIndex");
                match base.streams.get(index) {
                    Some(stream) => spec("set-stream-header", obj(vec![("streamIndex", Json::Number(index as f64)), ("strh", strh_to_json(&stream.strh))])),
                    None => spec("no-mutation", obj(vec![])),
                }
            }
            "set-stream-format" => {
                let index = index_of(params, "streamIndex");
                match base.streams.get(index) {
                    Some(stream) => spec("set-stream-format", obj(vec![("streamIndex", Json::Number(index as f64)), ("strf", strf_to_json(&stream.strf))])),
                    None => spec("no-mutation", obj(vec![])),
                }
            }
            "insert-chunk" => spec("remove-chunk", obj(vec![("streamIndex", Json::Number(index_of(params, "streamIndex") as f64)), ("index", Json::Number(index_of(params, "index") as f64))])),
            "remove-chunk" => {
                let stream_index = index_of(params, "streamIndex");
                let index = index_of(params, "index");
                match base.streams.get(stream_index).and_then(|stream| stream.chunks.get(index)) {
                    Some(chunk) => spec("insert-chunk", obj(vec![("streamIndex", Json::Number(stream_index as f64)), ("index", Json::Number(index as f64)), ("chunk", chunk_to_json(chunk))])),
                    None => spec("no-mutation", obj(vec![])),
                }
            }
            "set-chunk-keyframe" => {
                let stream_index = index_of(params, "streamIndex");
                let index = index_of(params, "index");
                match base.streams.get(stream_index).and_then(|stream| stream.chunks.get(index)) {
                    Some(chunk) => spec("set-chunk-keyframe", obj(vec![("streamIndex", Json::Number(stream_index as f64)), ("index", Json::Number(index as f64)), ("keyframe", Json::Bool(chunk.keyframe))])),
                    None => spec("no-mutation", obj(vec![])),
                }
            }
            "add-unknown-chunk" => spec("remove-unknown-chunk", obj(vec![("index", Json::Number(index_of(params, "index") as f64))])),
            "remove-unknown-chunk" => {
                let index = index_of(params, "index");
                match base.unknown_chunks.get(index) {
                    Some(item) => spec("add-unknown-chunk", obj(vec![("index", Json::Number(index as f64)), ("item", riff_chunk_to_json(item))])),
                    None => spec("no-mutation", obj(vec![])),
                }
            }
            other => spec(other, params.clone()),
        }
    }
    //#endregion 🔖️Inverse

    //#region 🔖️Routing
    pub fn apply_mutation(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        let mut doc = decode(input)?;
        apply_kind(&mut doc, kind, params)?;
        Ok(encode(&doc))
    }

    /// ↩️ Applies `{kind, params}` and then its computed inverse, in sequence — the caller compares
    /// its projection against the ORIGINAL input's own.
    pub fn apply_mutation_inverse(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        let base = decode(input)?;
        let inverse = inverse_spec(&base, kind, params);
        let mutated = apply_mutation(input, kind, params)?;
        apply_mutation(&mutated, &inverse.str("kind"), &inverse.get("params").cloned().unwrap_or(Json::Null))
    }

    //#region 🔖️Projection
    /// 🎞️ A chunk's payload projects as size+digest, never raw bytes — the same opaque-binary-
    /// payload treatment `💬️bcf`'s viewpoint snapshot and `🎥️mp4`'s own sample payload use, since a
    /// single real `movi` chunk in the committed fixture runs into the tens of kilobytes.
    fn chunk_projection(chunk: &OChunk) -> Json {
        obj(vec![("fourcc", Json::String(chunk.fourcc.clone())), ("keyframe", Json::Bool(chunk.keyframe)), ("dataSize", Json::Number(chunk.data.len() as f64)), ("dataDigest", Json::String(digest(&chunk.data)))])
    }

    fn unknown_chunk_projection(item: &ORiffChunk) -> Json {
        obj(vec![("fourcc", Json::String(item.fourcc.clone())), ("dataSize", Json::Number(item.data.len() as f64)), ("dataDigest", Json::String(digest(&item.data)))])
    }

    fn stream_projection(stream: &OStream) -> Json {
        obj(vec![("strh", strh_to_json(&stream.strh)), ("strf", strf_to_json(&stream.strf)), ("chunks", Json::Array(stream.chunks.iter().map(chunk_projection).collect()))])
    }

    /// 👁️ This subset's own semantic projection — main header, every stream IN ORDER (stream index
    /// is semantic identity in AVI, unlike BCF's guid-keyed topics), every chunk in playback order,
    /// `idx1Present`, and every top-level unknown chunk in order — independently re-derived by
    /// re-decoding `bytes` through this module's own `riff` composition. Only fields this subset's
    /// schema actually has a slot for are ever claimed (see module doc comment).
    pub fn project(bytes: &[u8]) -> Result<Json, String> {
        let doc = decode(bytes)?;
        Ok(obj(vec![
            ("mainHeader", main_header_to_json(&doc.main_header)),
            ("streams", Json::Array(doc.streams.iter().map(stream_projection).collect())),
            ("idx1Present", Json::Bool(doc.idx1_present)),
            ("unknownChunks", Json::Array(doc.unknown_chunks.iter().map(unknown_chunk_projection).collect())),
        ]))
    }
    //#endregion 🔖️Projection
    //#endregion 🔖️Routing

    //#region 🔖️Tests
    /// 🧪️ Plain `#[test]`, deliberately NOT `#[semio_framework_async_macros::async_test]` — this
    /// standalone oracle crate does not depend on that macro, and seven earlier agents' use of it
    /// here silently broke the whole test target so none of their tests ever ran.
    #[cfg(test)]
    mod tests {
        use super::*;

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free) — see R9
        fn tiny_snapshot() -> ODoc {
            ODoc {
                main_header: OMainHeader { micro_sec_per_frame: 100_000, max_bytes_per_sec: 1400, padding_granularity: 0, flags: 0x10, total_frames: 2, initial_frames: 0, streams: 1, suggested_buffer_size: 140, width: 16, height: 16, reserved: [0, 0, 0, 0] },
                streams: vec![OStream {
                    strh: OStreamHeader { fcc_type: "vids".into(), fcc_handler: "MJPG".into(), flags: 0, priority: 0, language: 0, initial_frames: 0, scale: 1, rate: 10, start: 0, length: 2, suggested_buffer_size: 140, quality: -1, sample_size: 0, rc_frame_left: 0, rc_frame_top: 0, rc_frame_right: 16, rc_frame_bottom: 16 },
                    strf: OStreamFormat::BitmapInfo { size: 40, width: 16, height: 16, planes: 1, bit_count: 24, compression: "MJPG".into(), size_image: 140, x_pels_per_meter: 0, y_pels_per_meter: 0, colors_used: 0, colors_important: 0 },
                    chunks: vec![OChunk { fourcc: "00dc".into(), data: vec![1, 2, 3, 4], keyframe: true }, OChunk { fourcc: "00dc".into(), data: vec![5, 6, 7], keyframe: false }],
                }],
                idx1_present: true,
                unknown_chunks: vec![ORiffChunk { fourcc: "JUNK".into(), data: vec![0, 0, 0, 0] }],
            }
        }

        #[test]
        fn encode_decode_round_trips_a_synthetic_document() {
            let doc = tiny_snapshot();
            let bytes = encode(&doc);
            assert!(bytes.starts_with(b"RIFF"));
            assert_eq!(&bytes[8..12], b"AVI ");
            let back = decode(&bytes).expect("decode");
            assert_eq!(back, doc);
        }

        #[test]
        fn decode_recognises_a_non_avi_riff_form() {
            let mut wave = b"RIFF".to_vec();
            wave.extend_from_slice(&4u32.to_le_bytes());
            wave.extend_from_slice(b"WAVE");
            let error = decode(&wave).unwrap_err();
            assert!(error.contains("RIFF form"), "unexpected error: {error}");
        }

        #[test]
        fn apply_mutation_sets_idx1_present_and_inverse_restores_it() {
            let bytes = encode(&tiny_snapshot());
            let spec = obj(vec![("idx1Present", Json::Bool(false))]);
            let mutated = apply_mutation(&bytes, "set-idx1-present", &spec).expect("apply");
            assert!(!decode(&mutated).unwrap().idx1_present);
            let restored = apply_mutation_inverse(&bytes, "set-idx1-present", &spec).expect("apply+inverse");
            assert!(decode(&restored).unwrap().idx1_present);
        }

        #[test]
        fn apply_mutation_removes_and_inverse_restores_a_chunk() {
            let bytes = encode(&tiny_snapshot());
            let spec = obj(vec![("streamIndex", Json::Number(0.0)), ("index", Json::Number(0.0))]);
            let mutated = apply_mutation(&bytes, "remove-chunk", &spec).expect("apply");
            assert_eq!(decode(&mutated).unwrap().streams[0].chunks.len(), 1);
            let restored = apply_mutation_inverse(&bytes, "remove-chunk", &spec).expect("apply+inverse");
            assert_eq!(decode(&restored).unwrap(), tiny_snapshot());
        }

        #[test]
        fn unrecognised_kind_is_an_error_not_a_silent_no_op() {
            let bytes = encode(&tiny_snapshot());
            let error = apply_mutation(&bytes, "not-a-real-kind", &Json::Object(vec![])).unwrap_err();
            assert!(error.contains("no oracle implementation"), "unexpected error: {error}");
        }

        #[test]
        fn project_reports_stream_and_chunk_shape() {
            let bytes = encode(&tiny_snapshot());
            let projection = project(&bytes).expect("project");
            assert_eq!(projection.array("streams").len(), 1);
            assert_eq!(projection.array("streams")[0].array("chunks").len(), 2);
            assert!(projection.get("idx1Present").is_some());
        }
    }
    //#endregion 🔖️Tests
}
//#endregion 🔖️Oracles

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    if kind.is_empty() {
        return Err("mutation spec carries no `kind`".to_string());
    }
    oracles::apply_mutation(input, &kind, &spec.get("params").cloned().unwrap_or(Json::Null))
}

/// ↩️ Applies one declared mutation kind and then its own computed inverse, in sequence, proving the
/// same `apply(inverse(m, base), apply(m, base)) == base` law `AviMutation::inverse` proves at the
/// Rust-model level, here against the registered `riff` composition instead.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation_inverse(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    if kind.is_empty() {
        return Err("mutation spec carries no `kind`".to_string());
    }
    oracles::apply_mutation_inverse(input, &kind, &spec.get("params").cloned().unwrap_or(Json::Null))
}

/// 👁️ This subset's own semantic projection. @see [`oracles::project`].
#[cfg(feature = "oracles")]
pub fn project_avi_1_0(bytes: &[u8]) -> Result<Json, String> {
    oracles::project(bytes)
}

/// 🚫️ Without the `oracles` feature the reference implementations are not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation_inverse(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn project_avi_1_0(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch
