//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::isobmff::subsets::any::schema::snapshot::Mp4Snapshot;
    use crate::standards::isobmff::subsets::any::io::Mp4Analyzer;
    use {semio_framework_plugin::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::ComposeError,semio_framework_plugin::ComposeSource,semio_framework_plugin::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.mp4", standard: StandardId("isobmff"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct Mp4ComposerComposition;

    impl ArtifactComposition for Mp4ComposerComposition {
        type Snapshot = Mp4Snapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "Mp4ComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = Mp4Analyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "Mp4ComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec. Called from
    /// this artifact's standard-level `engine::register()`.
    pub async fn register() {
        ::semio_framework_schema_registry::register_artifact_schema_descriptor(crate::standards::isobmff::subsets::any::schema::mp4_artifact_schema_descriptor()).expect("schema descriptor publication");
        register_artifact_inferences().await;
        semio_framework_plugin::io::register_native_document_codec(semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.mp4", standard: semio_framework_artifact_reference::StandardId("isobmff"), subset: semio_framework_artifact_reference::SubsetId("*") }, store::ArtifactCodec::bare::<Mp4Snapshot, crate::standards::isobmff::subsets::any::schema::mutations::Mp4Mutation>(crate::standards::isobmff::subsets::any::schema::snapshot::STDIO_MP4_DOCUMENT_SCHEMA))
            .expect("static Stdio registration must be available and conflict-free");
    }

    /// 💡️ Registers `s.stdio.mp4.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING P2/S3+S4).
    pub async fn register_artifact_inferences() {
        ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::standards::isobmff::subsets::any::schema::inferences::mp4_artifact_inference_descriptor()).expect("schema descriptor publication");
    }
    //#endregion 🔖️Register
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

// ⚙️ Mp4 (isobmff) engine — real ISO-BMFF box-tree decode/encode. Moved wholesale from
// remodel's video engine (`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/⚙️engine/🎥️video/🦀️.rs`,
// 5,163 LOC) per the master plan's extraction map, split into `📦️boxes` (box iterator/reader,
// moved from that file's `🔖️Bmff`/`🔖️Bytes` regions lines 12-236) and `🎥️h264` (moved from its
// `🔖️Bits`/`🔖️Rbsp`/`🔖️Sps` regions plus the `avcC` extract/build helpers from `🔖️Bmff`/
// `🔖️Mux`) submodules. `decode_mp4`/`encode_mp4` below are this file's own adaptation of that
// source's `probe_mp4` (lines 536-604) + `mp4_mux`/`mp4_build_moov`/`write_mp4_avc` (lines
// 3648-3740) — generalized from remodel's fixed single-run-length fixture muxer into a real
// per-sample `stts`/`ctts`/`stss` run-length encoder/decoder pair driven by this artifact's own
// `Mp4Snapshot` schema (moved logic, adapted shape — not reimplemented from first principles).
//
// The schema retains named ISO-BMFF concepts and semantic encoded sample payloads only. Native
// box syntax is parsed at import and deterministically rebuilt at export.

use crate::standards::isobmff::subsets::any::schema::snapshot::{Mp4Bitrate, Mp4Codec, Mp4CodecFormat, Mp4Color, Mp4Edit, Mp4Ftyp, Mp4HevcConfig, Mp4HevcNalArray, Mp4Movie, Mp4PixelAspectRatio, Mp4Sample, Mp4Snapshot, Mp4Track, Mp4TrackMetadata, Mp4VisualSampleEntry, STDIO_MP4_DOCUMENT_SCHEMA};

#[path = "📦️boxes/🦀️.rs"]
pub mod boxes;
#[path = "🎥️h264/🦀️.rs"]
pub mod h264;

use boxes::{find_box, find_boxes, iter_boxes, require_box, write_box, ByteReader};

//#region 🔖️Sniff
/// 🔍 True when `bytes` starts with a plausible ISO-BMFF top-level box whose 4-byte type is
/// ASCII AND is the real `ftyp` magic (the first box of every conformant MP4/ISO-BMFF file,
/// §4.3) — a real structural check, not a fixed byte-string match.
pub fn sniff_real_bytes(bytes: &[u8]) -> bool {
    if bytes.len() < 8 {
        return false;
    }
    let size = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    let box_type = &bytes[4..8];
    let ascii_type = box_type.iter().all(|&b| b.is_ascii_alphanumeric() || b == b' ');
    ascii_type && box_type == b"ftyp" && (size as usize >= 8 || size == 0 || size == 1)
}
//#endregion 🔖️Sniff

//#region 🔖️Stbl
/// 📥️ `stts` (moved from remodel's `parse_stts`) → `(sample_count, delta)*` run-length pairs.
fn parse_stts(payload: &[u8]) -> Result<Vec<(u32, u32)>, String> {
    let mut r = ByteReader::new(payload);
    r.skip(4).map_err(|e| e.to_string())?;
    let count = r.u32_be().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push((r.u32_be().map_err(|e| e.to_string())?, r.u32_be().map_err(|e| e.to_string())?));
    }
    Ok(out)
}

fn parse_ctts(payload: &[u8]) -> Result<Vec<(u32, i64)>, String> {
    let mut r = ByteReader::new(payload);
    let version = r.u8().map_err(|e| e.to_string())?;
    r.skip(3).map_err(|e| e.to_string())?;
    let count = r.u32_be().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let sample_count = r.u32_be().map_err(|e| e.to_string())?;
        let raw = r.u32_be().map_err(|e| e.to_string())?;
        let offset = if version == 1 { i64::from(raw as i32) } else { i64::from(raw) };
        out.push((sample_count, offset));
    }
    Ok(out)
}

fn parse_stsc(payload: &[u8]) -> Result<Vec<(u32, u32, u32)>, String> {
    let mut r = ByteReader::new(payload);
    r.skip(4).map_err(|e| e.to_string())?;
    let count = r.u32_be().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push((r.u32_be().map_err(|e| e.to_string())?, r.u32_be().map_err(|e| e.to_string())?, r.u32_be().map_err(|e| e.to_string())?));
    }
    Ok(out)
}

enum SampleSizes {
    Uniform { size: u32, count: u32 },
    PerSample(Vec<u32>),
}
impl SampleSizes {
    fn len(&self) -> usize {
        match self {
            Self::Uniform { count, .. } => *count as usize,
            Self::PerSample(v) => v.len(),
        }
    }
    fn get(&self, i: usize) -> Result<u32, String> {
        match self {
            Self::Uniform { size, count } => {
                if (i as u32) < *count {
                    Ok(*size)
                } else {
                    Err("stsz sample index out of range".into())
                }
            }
            Self::PerSample(v) => v.get(i).copied().ok_or_else(|| "stsz sample index out of range".into()),
        }
    }
}

fn parse_stsz(payload: &[u8]) -> Result<SampleSizes, String> {
    let mut r = ByteReader::new(payload);
    r.skip(4).map_err(|e| e.to_string())?;
    let sample_size = r.u32_be().map_err(|e| e.to_string())?;
    let count = r.u32_be().map_err(|e| e.to_string())?;
    if sample_size != 0 {
        return Ok(SampleSizes::Uniform { size: sample_size, count });
    }
    let mut sizes = Vec::with_capacity(count as usize);
    for _ in 0..count {
        sizes.push(r.u32_be().map_err(|e| e.to_string())?);
    }
    Ok(SampleSizes::PerSample(sizes))
}

fn parse_stco(payload: &[u8]) -> Result<Vec<u64>, String> {
    let mut r = ByteReader::new(payload);
    r.skip(4).map_err(|e| e.to_string())?;
    let count = r.u32_be().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(u64::from(r.u32_be().map_err(|e| e.to_string())?));
    }
    Ok(out)
}

fn parse_co64(payload: &[u8]) -> Result<Vec<u64>, String> {
    let mut r = ByteReader::new(payload);
    r.skip(4).map_err(|e| e.to_string())?;
    let count = r.u32_be().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(r.u64_be().map_err(|e| e.to_string())?);
    }
    Ok(out)
}

fn parse_stss(payload: &[u8]) -> Result<Vec<u32>, String> {
    let mut r = ByteReader::new(payload);
    r.skip(4).map_err(|e| e.to_string())?;
    let count = r.u32_be().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(r.u32_be().map_err(|e| e.to_string())?);
    }
    Ok(out)
}

fn samples_per_chunk_for(stsc: &[(u32, u32, u32)], chunk_number: u32) -> Result<u32, String> {
    let mut result = None;
    for &(first_chunk, spc, _) in stsc {
        if first_chunk == 0 {
            return Err("stsc first_chunk must be >= 1".into());
        }
        if first_chunk <= chunk_number {
            result = Some(spc);
        } else {
            break;
        }
    }
    result.ok_or_else(|| "stsc does not cover this chunk".into())
}

fn resolve_samples(stsc: &[(u32, u32, u32)], chunk_offsets: &[u64], sizes: &SampleSizes) -> Result<Vec<(u64, u32)>, String> {
    if stsc.is_empty() {
        return Err("stsc has no entries".into());
    }
    let mut out = Vec::with_capacity(sizes.len());
    let mut sample_index = 0usize;
    for (i, &chunk_offset) in chunk_offsets.iter().enumerate() {
        let chunk_number = i as u32 + 1;
        let spc = samples_per_chunk_for(stsc, chunk_number)?;
        let mut offset = chunk_offset;
        for _ in 0..spc {
            if sample_index >= sizes.len() {
                break;
            }
            let size = sizes.get(sample_index)?;
            out.push((offset, size));
            offset = offset.checked_add(u64::from(size)).ok_or("chunk offset overflow")?;
            sample_index += 1;
        }
    }
    Ok(out)
}

fn logical_chunk_sample_counts(stsc: &[(u32, u32, u32)], chunk_count: usize, sample_count: usize) -> Result<Vec<u32>, String> {
    let mut remaining = sample_count;
    let mut counts = Vec::with_capacity(chunk_count);
    for index in 0..chunk_count {
        let declared = samples_per_chunk_for(stsc, index as u32 + 1)? as usize;
        let count = declared.min(remaining);
        counts.push(count as u32);
        remaining -= count;
    }
    if remaining != 0 {
        return Err("stsc/stco do not cover every sample".into());
    }
    Ok(counts)
}

fn expand_run_length_u32(entries: &[(u32, u32)]) -> Vec<u32> {
    entries.iter().flat_map(|&(count, value)| std::iter::repeat_n(value, count as usize)).collect()
}
fn expand_run_length_i64(entries: &[(u32, i64)]) -> Vec<i64> {
    entries.iter().flat_map(|&(count, value)| std::iter::repeat_n(value, count as usize)).collect()
}

/// ✍️ Run-length encodes a per-sample `u32` series into `stts`/`stsz`-style `(count, value)*` runs.
fn run_length_encode_u32(values: &[u32]) -> Vec<(u32, u32)> {
    let mut out: Vec<(u32, u32)> = Vec::new();
    for &v in values {
        if let Some(last) = out.last_mut() {
            if last.1 == v {
                last.0 += 1;
                continue;
            }
        }
        out.push((1, v));
    }
    out
}
fn run_length_encode_i64(values: &[i64]) -> Vec<(u32, i64)> {
    let mut out: Vec<(u32, i64)> = Vec::new();
    for &v in values {
        if let Some(last) = out.last_mut() {
            if last.1 == v {
                last.0 += 1;
                continue;
            }
        }
        out.push((1, v));
    }
    out
}
//#endregion 🔖️Stbl

//#region 🔖️Tkhd
fn read_time(reader: &mut ByteReader<'_>, version: u8) -> Result<u64, String> {
    if version == 1 {
        reader.u64_be().map_err(|error| error.to_string())
    } else {
        reader.u32_be().map(u64::from).map_err(|error| error.to_string())
    }
}

fn parse_tkhd(payload: &[u8]) -> Result<(u32, Mp4TrackMetadata), String> {
    let mut r = ByteReader::new(payload);
    let version = r.u8().map_err(|e| e.to_string())?;
    let flags_bytes = r.take(3).map_err(|e| e.to_string())?;
    let flags = u32::from_be_bytes([0, flags_bytes[0], flags_bytes[1], flags_bytes[2]]);
    let creation_time = read_time(&mut r, version)?;
    let modification_time = read_time(&mut r, version)?;
    let track_id = r.u32_be().map_err(|e| e.to_string())?;
    r.skip(4).map_err(|e| e.to_string())?;
    let duration = read_time(&mut r, version)?;
    r.skip(8).map_err(|e| e.to_string())?;
    let layer = r.u16_be().map_err(|e| e.to_string())? as i16;
    let alternate_group = r.u16_be().map_err(|e| e.to_string())? as i16;
    let volume = r.u16_be().map_err(|e| e.to_string())? as i16;
    r.skip(2).map_err(|e| e.to_string())?;
    let mut matrix = [0i32; 9];
    for value in &mut matrix {
        *value = r.i32_be().map_err(|e| e.to_string())?;
    }
    Ok((track_id, Mp4TrackMetadata { flags, creation_time, modification_time, duration, layer, alternate_group, volume, matrix, ..Mp4TrackMetadata::default() }))
}

fn parse_mdhd(payload: &[u8], metadata: &mut Mp4TrackMetadata) -> Result<u32, String> {
    let mut r = ByteReader::new(payload);
    let version = r.u8().map_err(|e| e.to_string())?;
    r.skip(3).map_err(|e| e.to_string())?;
    metadata.media_creation_time = read_time(&mut r, version)?;
    metadata.media_modification_time = read_time(&mut r, version)?;
    let timescale = r.u32_be().map_err(|e| e.to_string())?;
    metadata.media_duration = read_time(&mut r, version)?;
    let packed_language = r.u16_be().map_err(|e| e.to_string())?;
    metadata.language = [10, 5, 0].into_iter().map(|shift| char::from(((packed_language >> shift) & 0x1f) as u8 + 0x60)).collect();
    metadata.quality = r.u16_be().map_err(|e| e.to_string())?;
    Ok(timescale)
}

fn parse_visual_sample_entry(payload: &[u8]) -> Result<(u16, u16, Mp4VisualSampleEntry, &[u8]), String> {
    let mut r = ByteReader::new(payload);
    r.skip(6).map_err(|e| e.to_string())?;
    let data_reference_index = r.u16_be().map_err(|e| e.to_string())?;
    let version = r.u16_be().map_err(|e| e.to_string())?;
    let revision_level = r.u16_be().map_err(|e| e.to_string())?;
    let vendor = r.u32_be().map_err(|e| e.to_string())?;
    let temporal_quality = r.u32_be().map_err(|e| e.to_string())?;
    let spatial_quality = r.u32_be().map_err(|e| e.to_string())?;
    let width = r.u16_be().map_err(|e| e.to_string())?;
    let height = r.u16_be().map_err(|e| e.to_string())?;
    let horizontal_resolution = r.u32_be().map_err(|e| e.to_string())?;
    let vertical_resolution = r.u32_be().map_err(|e| e.to_string())?;
    r.skip(4).map_err(|e| e.to_string())?;
    let frame_count = r.u16_be().map_err(|e| e.to_string())?;
    let compressor = r.take(32).map_err(|e| e.to_string())?;
    let compressor_length = usize::from(compressor[0]).min(31);
    let compressor_name = String::from_utf8_lossy(&compressor[1..1 + compressor_length]).into_owned();
    let depth = r.u16_be().map_err(|e| e.to_string())?;
    let color_table_id = r.u16_be().map_err(|e| e.to_string())? as i16;
    let visual = Mp4VisualSampleEntry { data_reference_index, version, revision_level, vendor, temporal_quality, spatial_quality, horizontal_resolution, vertical_resolution, frame_count, compressor_name, depth, color_table_id };
    Ok((width, height, visual, &payload[r.pos()..]))
}

fn parse_mvhd(payload: &[u8]) -> Result<Mp4Movie, String> {
    let mut r = ByteReader::new(payload);
    let version = r.u8().map_err(|e| e.to_string())?;
    r.skip(3).map_err(|e| e.to_string())?;
    let creation_time = read_time(&mut r, version)?;
    let modification_time = read_time(&mut r, version)?;
    let timescale = r.u32_be().map_err(|e| e.to_string())?;
    let duration = read_time(&mut r, version)?;
    let rate = r.i32_be().map_err(|e| e.to_string())?;
    let volume = r.u16_be().map_err(|e| e.to_string())? as i16;
    r.skip(10).map_err(|e| e.to_string())?;
    let mut matrix = [0i32; 9];
    for value in &mut matrix {
        *value = r.i32_be().map_err(|e| e.to_string())?;
    }
    r.skip(24).map_err(|e| e.to_string())?;
    let next_track_id = r.u32_be().map_err(|e| e.to_string())?;
    Ok(Mp4Movie { creation_time, modification_time, timescale, duration, rate, volume, matrix, next_track_id, title: None, encoder: None })
}

fn parse_edit_list(trak: &[u8]) -> Result<Vec<Mp4Edit>, String> {
    let Some(edts) = find_box(trak, b"edts").map_err(|e| e.to_string())? else {
        return Ok(Vec::new());
    };
    let Some(elst) = find_box(edts, b"elst").map_err(|e| e.to_string())? else {
        return Ok(Vec::new());
    };
    let mut r = ByteReader::new(elst);
    let version = r.u8().map_err(|e| e.to_string())?;
    r.skip(3).map_err(|e| e.to_string())?;
    let count = r.u32_be().map_err(|e| e.to_string())?;
    let mut edits = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let segment_duration = read_time(&mut r, version)?;
        let media_time = if version == 1 { r.u64_be().map_err(|e| e.to_string())? as i64 } else { i64::from(r.i32_be().map_err(|e| e.to_string())?) };
        edits.push(Mp4Edit { segment_duration, media_time, media_rate_integer: r.u16_be().map_err(|e| e.to_string())? as i16, media_rate_fraction: r.u16_be().map_err(|e| e.to_string())? as i16 });
    }
    Ok(edits)
}

fn parse_codec_extensions(children: &[u8], metadata: &mut Mp4TrackMetadata) -> Result<(), String> {
    if let Some(payload) = find_box(children, b"colr").map_err(|e| e.to_string())? {
        let mut r = ByteReader::new(payload);
        let color_type = r.fourcc().map_err(|e| e.to_string())?.as_str().into_owned();
        let primaries = r.u16_be().map_err(|e| e.to_string())?;
        let transfer = r.u16_be().map_err(|e| e.to_string())?;
        let matrix = r.u16_be().map_err(|e| e.to_string())?;
        let full_range = if r.remaining() > 0 { Some(r.u8().map_err(|e| e.to_string())? & 0x80 != 0) } else { None };
        metadata.color = Some(Mp4Color { color_type, primaries, transfer, matrix, full_range });
    }
    if let Some(payload) = find_box(children, b"pasp").map_err(|e| e.to_string())? {
        let mut r = ByteReader::new(payload);
        metadata.pixel_aspect_ratio = Some(Mp4PixelAspectRatio { horizontal_spacing: r.u32_be().map_err(|e| e.to_string())?, vertical_spacing: r.u32_be().map_err(|e| e.to_string())? });
    }
    if let Some(payload) = find_box(children, b"btrt").map_err(|e| e.to_string())? {
        let mut r = ByteReader::new(payload);
        metadata.bitrate = Some(Mp4Bitrate { buffer_size: r.u32_be().map_err(|e| e.to_string())?, maximum: r.u32_be().map_err(|e| e.to_string())?, average: r.u32_be().map_err(|e| e.to_string())? });
    }
    Ok(())
}

fn parse_metadata_item(ilst: &[u8], fourcc: &[u8; 4]) -> Result<Option<String>, String> {
    let Some(item) = find_box(ilst, fourcc).map_err(|e| e.to_string())? else {
        return Ok(None);
    };
    let Some(data) = find_box(item, b"data").map_err(|e| e.to_string())? else {
        return Ok(None);
    };
    if data.len() < 8 {
        return Err("MP4 metadata data box is truncated".into());
    }
    Ok(Some(String::from_utf8_lossy(&data[8..]).into_owned()))
}

fn parse_movie_metadata(moov: &[u8], movie: &mut Mp4Movie) -> Result<(), String> {
    let Some(udta) = find_box(moov, b"udta").map_err(|e| e.to_string())? else {
        return Ok(());
    };
    let Some(meta) = find_box(udta, b"meta").map_err(|e| e.to_string())? else {
        return Ok(());
    };
    if meta.len() < 4 {
        return Err("MP4 meta box is truncated".into());
    }
    let Some(ilst) = find_box(&meta[4..], b"ilst").map_err(|e| e.to_string())? else {
        return Ok(());
    };
    movie.title = parse_metadata_item(ilst, &[0xa9, b'n', b'a', b'm'])?;
    movie.encoder = parse_metadata_item(ilst, &[0xa9, b't', b'o', b'o'])?;
    Ok(())
}
//#endregion 🔖️Tkhd

//#region 🔖️HvcC
/// 📥️ `hvcC` (HEVCDecoderConfigurationRecord, ISO/IEC 14496-15 §8.3.3.1) → typed record plus
/// `lengthSizeMinusOne + 1`. Reserved bits are not retained; the writer emits them as all ones.
fn parse_hvcc(payload: &[u8]) -> Result<(Mp4HevcConfig, u8), String> {
    let mut r = ByteReader::new(payload);
    let err = |e: boxes::BoxError| e.to_string();
    let version = r.u8().map_err(err)?;
    if version != 1 {
        return Err(format!("unsupported hvcC configurationVersion {version}"));
    }
    let profile = r.u8().map_err(err)?;
    let general_profile_compatibility_flags = r.u32_be().map_err(err)?;
    let constraint_high = u64::from(r.u32_be().map_err(err)?);
    let constraint_low = u64::from(r.u16_be().map_err(err)?);
    let general_level_idc = r.u8().map_err(err)?;
    let min_spatial_segmentation_idc = r.u16_be().map_err(err)? & 0x0FFF;
    let parallelism_type = r.u8().map_err(err)? & 0x03;
    let chroma_format_idc = r.u8().map_err(err)? & 0x03;
    let bit_depth_luma_minus8 = r.u8().map_err(err)? & 0x07;
    let bit_depth_chroma_minus8 = r.u8().map_err(err)? & 0x07;
    let avg_frame_rate = r.u16_be().map_err(err)?;
    let packed = r.u8().map_err(err)?;
    let num_of_arrays = r.u8().map_err(err)?;
    let mut arrays = Vec::with_capacity(num_of_arrays as usize);
    for _ in 0..num_of_arrays {
        let head = r.u8().map_err(err)?;
        let num_nalus = r.u16_be().map_err(err)?;
        let mut nal_units = Vec::with_capacity(num_nalus as usize);
        for _ in 0..num_nalus {
            let length = r.u16_be().map_err(err)? as usize;
            nal_units.push(r.take(length).map_err(err)?.to_vec());
        }
        arrays.push(Mp4HevcNalArray { array_completeness: head & 0x80 != 0, nal_unit_type: head & 0x3F, nal_units });
    }
    let config = Mp4HevcConfig {
        general_profile_space: profile >> 6,
        general_tier_flag: profile & 0x20 != 0,
        general_profile_idc: profile & 0x1F,
        general_profile_compatibility_flags,
        general_constraint_indicator_flags: (constraint_high << 16) | constraint_low,
        general_level_idc,
        min_spatial_segmentation_idc,
        parallelism_type,
        chroma_format_idc,
        bit_depth_luma_minus8,
        bit_depth_chroma_minus8,
        avg_frame_rate,
        constant_frame_rate: packed >> 6,
        num_temporal_layers: (packed >> 3) & 0x07,
        temporal_id_nested: packed & 0x04 != 0,
        arrays,
    };
    Ok((config, (packed & 0x03) + 1))
}

/// ✍️ Builds an `hvcC` box from the typed record (reserved bits all ones, per §8.3.3.1).
fn build_hvcc(config: &Mp4HevcConfig, nal_length_size: u8) -> Vec<u8> {
    let mut out = vec![1u8, ((config.general_profile_space & 0x03) << 6) | (u8::from(config.general_tier_flag) << 5) | (config.general_profile_idc & 0x1F)];
    out.extend_from_slice(&config.general_profile_compatibility_flags.to_be_bytes());
    out.extend_from_slice(&config.general_constraint_indicator_flags.to_be_bytes()[2..8]);
    out.push(config.general_level_idc);
    out.extend_from_slice(&(0xF000 | (config.min_spatial_segmentation_idc & 0x0FFF)).to_be_bytes());
    out.push(0xFC | (config.parallelism_type & 0x03));
    out.push(0xFC | (config.chroma_format_idc & 0x03));
    out.push(0xF8 | (config.bit_depth_luma_minus8 & 0x07));
    out.push(0xF8 | (config.bit_depth_chroma_minus8 & 0x07));
    out.extend_from_slice(&config.avg_frame_rate.to_be_bytes());
    out.push(((config.constant_frame_rate & 0x03) << 6) | ((config.num_temporal_layers & 0x07) << 3) | (u8::from(config.temporal_id_nested) << 2) | (nal_length_size.saturating_sub(1) & 0x03));
    out.push(config.arrays.len() as u8);
    for array in &config.arrays {
        out.push((u8::from(array.array_completeness) << 7) | (array.nal_unit_type & 0x3F));
        out.extend_from_slice(&(array.nal_units.len() as u16).to_be_bytes());
        for nal in &array.nal_units {
            out.extend_from_slice(&(nal.len() as u16).to_be_bytes());
            out.extend_from_slice(nal);
        }
    }
    write_box(b"hvcC", &out)
}
//#endregion 🔖️HvcC

//#region 🔖️Decode
/// 📥️ Decodes real ISO-BMFF bytes into an `Mp4Snapshot` — walks `ftyp`/`moov`/`trak`(s)/`mdat`,
/// resolving the full per-sample table for every video-handler track. Adapted from remodel's
/// `probe_mp4` (which stops at the first video track and only reports probe metadata, not sample
/// bytes) — this version decodes EVERY `vide` track and copies each sample's real payload bytes
/// out of `mdat` into `Mp4Sample.data` (probe_mp4 never needed the bytes themselves, only
/// offsets, since remodel's own callers re-read from the source buffer lazily).
pub fn decode_mp4(bytes: &[u8]) -> Result<Mp4Snapshot, String> {
    let ftyp_payload = require_box(bytes, b"ftyp", "mp4 stream missing ftyp box").map_err(|e| e.to_string())?;
    let mut fr = ByteReader::new(ftyp_payload);
    let major_brand = fr.fourcc().map_err(|e| e.to_string())?.as_str().into_owned();
    let minor_version = fr.u32_be().map_err(|e| e.to_string())?;
    let mut compatible_brands = Vec::new();
    while fr.remaining() >= 4 {
        compatible_brands.push(fr.fourcc().map_err(|e| e.to_string())?.as_str().into_owned());
    }
    let ftyp = Mp4Ftyp { major_brand, minor_version, compatible_brands };

    let moov = require_box(bytes, b"moov", "mp4 stream missing moov box").map_err(|e| e.to_string())?;
    let mut movie = parse_mvhd(require_box(moov, b"mvhd", "moov missing mvhd").map_err(|e| e.to_string())?)?;
    parse_movie_metadata(moov, &mut movie)?;
    let mut tracks = Vec::new();
    for item in iter_boxes(bytes) {
        let b = item.map_err(|e| e.to_string())?;
        match &b.kind.0 {
            b"ftyp" | b"mdat" => {}
            b"moov" => {
                for trak in find_boxes(b.payload, b"trak").map_err(|e| e.to_string())? {
                    tracks.push(decode_trak(trak, bytes)?);
                }
            }
            b"free" | b"skip" => {}
            _ => return Err(format!("unsupported top-level ISO-BMFF box {}", b.kind.as_str())),
        }
    }
    Ok(Mp4Snapshot { schema: STDIO_MP4_DOCUMENT_SCHEMA.into(), ftyp, movie, tracks })
}

/// 📥️ Decodes one typed video track and rejects unsupported handler types.
fn decode_trak(trak: &[u8], file_bytes: &[u8]) -> Result<Mp4Track, String> {
    let tkhd = require_box(trak, b"tkhd", "trak missing tkhd").map_err(|e| e.to_string())?;
    let (track_id, mut metadata) = parse_tkhd(tkhd)?;
    metadata.edits = parse_edit_list(trak)?;
    let mdia = require_box(trak, b"mdia", "trak missing mdia").map_err(|e| e.to_string())?;
    let hdlr = require_box(mdia, b"hdlr", "mdia missing hdlr").map_err(|e| e.to_string())?;
    if hdlr.len() < 12 || &hdlr[8..12] != b"vide" {
        return Err("unsupported non-video MP4 track".into());
    }
    if hdlr.len() > 24 {
        metadata.handler_name = String::from_utf8_lossy(&hdlr[24..]).trim_end_matches('\0').to_string();
    }
    let mdhd = require_box(mdia, b"mdhd", "mdia missing mdhd").map_err(|e| e.to_string())?;
    let timescale = parse_mdhd(mdhd, &mut metadata)?;
    let minf = require_box(mdia, b"minf", "mdia missing minf").map_err(|e| e.to_string())?;
    let stbl = require_box(minf, b"stbl", "minf missing stbl").map_err(|e| e.to_string())?;
    let stsd = require_box(stbl, b"stsd", "stbl missing stsd").map_err(|e| e.to_string())?;

    let mut sr = ByteReader::new(stsd);
    sr.skip(4).map_err(|e| e.to_string())?;
    let entry_count = sr.u32_be().map_err(|e| e.to_string())?;
    if entry_count == 0 {
        return Err("stsd has no sample entries".into());
    }
    let rest = &stsd[sr.pos()..];
    let first = iter_boxes(rest).next().ok_or("stsd missing sample entry box")?.map_err(|e| e.to_string())?;
    let (width, height, visual, children) = parse_visual_sample_entry(first.payload)?;
    metadata.visual = visual;
    parse_codec_extensions(children, &mut metadata)?;
    let format = Mp4CodecFormat::from_fourcc(&first.kind.0).ok_or_else(|| format!("unsupported MP4 sample entry {}", first.kind.as_str()))?;
    let codec = if format.is_avc() {
        let avcc = require_box(children, b"avcC", "avc sample entry missing avcC").map_err(|e| e.to_string())?;
        let (sps, pps, nal_length_size, extension) = h264::parse_avcc_extended(avcc).map_err(|e| e.to_string())?;
        Mp4Codec { format, ..Mp4Codec::avc(sps, pps, nal_length_size, extension) }
    } else if format.is_hevc() {
        let hvcc = require_box(children, b"hvcC", "hevc sample entry missing hvcC").map_err(|e| e.to_string())?;
        let (config, nal_length_size) = parse_hvcc(hvcc)?;
        Mp4Codec::hevc(format, config, nal_length_size)
    } else {
        Mp4Codec::jpeg(format)
    };

    let stts = require_box(stbl, b"stts", "stbl missing stts").map_err(|e| e.to_string())?;
    let durations = expand_run_length_u32(&parse_stts(stts)?);
    let stsc_entries = parse_stsc(require_box(stbl, b"stsc", "stbl missing stsc").map_err(|e| e.to_string())?)?;
    let sizes = parse_stsz(require_box(stbl, b"stsz", "stbl missing stsz").map_err(|e| e.to_string())?)?;
    let chunk_offsets = match find_box(stbl, b"stco").map_err(|e| e.to_string())? {
        Some(p) => parse_stco(p)?,
        None => parse_co64(require_box(stbl, b"co64", "stbl missing stco/co64").map_err(|e| e.to_string())?)?,
    };
    let sync = match find_box(stbl, b"stss").map_err(|e| e.to_string())? {
        Some(p) => Some(parse_stss(p)?),
        None => None,
    };
    let sample_count = sizes.len();
    if durations.len() != sample_count {
        return Err("stts sample count does not match stsz".into());
    }
    let cts_offsets: Vec<i64> = match find_box(stbl, b"ctts").map_err(|e| e.to_string())? {
        Some(p) => {
            let expanded = expand_run_length_i64(&parse_ctts(p)?);
            if expanded.len() != sample_count {
                return Err("ctts sample count does not match stsz".into());
            }
            expanded
        }
        None => vec![0i64; sample_count],
    };
    let offsets_sizes = resolve_samples(&stsc_entries, &chunk_offsets, &sizes)?;
    if offsets_sizes.len() != sample_count {
        return Err("stsc/stco resolved sample count does not match stsz".into());
    }

    let mut samples = Vec::with_capacity(sample_count);
    for i in 0..sample_count {
        let (offset, size) = offsets_sizes[i];
        let data = file_bytes.get(offset as usize..offset as usize + size as usize).ok_or("mp4: sample byte range out of file bounds")?.to_vec();
        let is_sync = sync.as_ref().is_none_or(|list| list.contains(&(i as u32 + 1)));
        samples.push(Mp4Sample { data, duration: durations[i], cts_offset: cts_offsets[i] as i32, sync: is_sync });
    }

    let chunk_sample_counts = logical_chunk_sample_counts(&stsc_entries, chunk_offsets.len(), sample_count)?;
    Ok(Mp4Track { track_id, timescale, codec, width: u32::from(width), height: u32::from(height), metadata, chunk_sample_counts, samples })
}
//#endregion 🔖️Decode

//#region 🔖️Encode
/// 🐛 The 8 bytes of `reserved`+`data_reference_index` are followed by `pre_defined(2)` +
/// `reserved(2)` + `pre_defined[3](12)` = 16 bytes before `width`/`height` — matches
/// `parse_visual_sample_entry`'s read-side `skip(6+2+2+2+12)`.
fn mp4_visual_sample_entry(codec_fourcc: &[u8; 4], width: u16, height: u16, visual: &Mp4VisualSampleEntry, extra: &[u8]) -> Vec<u8> {
    let mut payload = vec![0u8; 6];
    payload.extend_from_slice(&visual.data_reference_index.to_be_bytes());
    payload.extend_from_slice(&visual.version.to_be_bytes());
    payload.extend_from_slice(&visual.revision_level.to_be_bytes());
    payload.extend_from_slice(&visual.vendor.to_be_bytes());
    payload.extend_from_slice(&visual.temporal_quality.to_be_bytes());
    payload.extend_from_slice(&visual.spatial_quality.to_be_bytes());
    payload.extend_from_slice(&width.to_be_bytes());
    payload.extend_from_slice(&height.to_be_bytes());
    payload.extend_from_slice(&visual.horizontal_resolution.to_be_bytes());
    payload.extend_from_slice(&visual.vertical_resolution.to_be_bytes());
    payload.extend_from_slice(&[0u8; 4]);
    payload.extend_from_slice(&visual.frame_count.to_be_bytes());
    let compressor = visual.compressor_name.as_bytes();
    let compressor_length = compressor.len().min(31);
    payload.push(compressor_length as u8);
    payload.extend_from_slice(&compressor[..compressor_length]);
    payload.resize(payload.len() + 31 - compressor_length, 0);
    payload.extend_from_slice(&visual.depth.to_be_bytes());
    payload.extend_from_slice(&visual.color_table_id.to_be_bytes());
    payload.extend_from_slice(extra);
    write_box(codec_fourcc, &payload)
}

fn build_codec_extensions(track: &Mp4Track) -> Vec<u8> {
    let mut result = Vec::new();
    if let Some(color) = &track.metadata.color {
        let mut payload = [b' '; 4].to_vec();
        for (index, byte) in color.color_type.as_bytes().iter().take(4).enumerate() {
            payload[index] = *byte;
        }
        payload.extend_from_slice(&color.primaries.to_be_bytes());
        payload.extend_from_slice(&color.transfer.to_be_bytes());
        payload.extend_from_slice(&color.matrix.to_be_bytes());
        if let Some(full_range) = color.full_range {
            payload.push(if full_range { 0x80 } else { 0 });
        }
        result.extend(write_box(b"colr", &payload));
    }
    if let Some(aspect) = &track.metadata.pixel_aspect_ratio {
        result.extend(write_box(b"pasp", &[aspect.horizontal_spacing.to_be_bytes(), aspect.vertical_spacing.to_be_bytes()].concat()));
    }
    if let Some(bitrate) = &track.metadata.bitrate {
        result.extend(write_box(b"btrt", &[bitrate.buffer_size.to_be_bytes(), bitrate.maximum.to_be_bytes(), bitrate.average.to_be_bytes()].concat()));
    }
    result
}

/// ✍️ The configuration record box the sample entry `format` names: `avcC` for AVC, `hvcC` for
/// HEVC (a missing `hevc` record is written as the default Main-profile record), nothing for JPEG.
fn build_codec_configuration(codec: &Mp4Codec) -> Vec<u8> {
    let format = codec.format;
    if format.is_avc() {
        h264::build_avcc_extended(&codec.sps, &codec.pps, codec.nal_length_size, codec.extension.as_ref())
    } else if format.is_hevc() {
        build_hvcc(&codec.hevc.clone().unwrap_or_default(), codec.nal_length_size)
    } else {
        Vec::new()
    }
}

fn build_stbl(track: &Mp4Track, chunk_offsets: &[u32]) -> Vec<u8> {
    let mut extra = build_codec_configuration(&track.codec);
    extra.extend(build_codec_extensions(track));
    let mut codec_fourcc = [0u8; 4];
    codec_fourcc.copy_from_slice(track.codec.format.fourcc().as_bytes());
    let mut stsd_payload = vec![0u8; 4];
    stsd_payload.extend_from_slice(&1u32.to_be_bytes());
    stsd_payload.extend(mp4_visual_sample_entry(&codec_fourcc, track.width as u16, track.height as u16, &track.metadata.visual, &extra));
    let stsd = write_box(b"stsd", &stsd_payload);
    [stsd, build_stts(track), build_stss(track), build_ctts(track), build_stsc(track), build_stsz(track), build_stco(chunk_offsets)].concat()
}

fn build_stts(track: &Mp4Track) -> Vec<u8> {
    let durations: Vec<u32> = track.samples.iter().map(|s| s.duration).collect();
    let runs = run_length_encode_u32(&durations);
    let mut payload = vec![0u8; 4];
    payload.extend_from_slice(&(runs.len() as u32).to_be_bytes());
    for (count, delta) in runs {
        payload.extend_from_slice(&count.to_be_bytes());
        payload.extend_from_slice(&delta.to_be_bytes());
    }
    write_box(b"stts", &payload)
}

fn build_ctts(track: &Mp4Track) -> Vec<u8> {
    if track.samples.iter().all(|s| s.cts_offset == 0) {
        return Vec::new();
    }
    let offsets: Vec<i64> = track.samples.iter().map(|s| i64::from(s.cts_offset)).collect();
    let version: u8 = if offsets.iter().any(|&v| v < 0) { 1 } else { 0 };
    let runs = run_length_encode_i64(&offsets);
    let mut payload = vec![version, 0, 0, 0];
    payload.extend_from_slice(&(runs.len() as u32).to_be_bytes());
    for (count, offset) in runs {
        payload.extend_from_slice(&count.to_be_bytes());
        payload.extend_from_slice(&(offset as i32 as u32).to_be_bytes());
    }
    write_box(b"ctts", &payload)
}

/// ✍️ The chunk grouping a track is written with. `chunk_sample_counts` is a RETAINED layout
/// hint — `stsc`/`stco` read back on decode so an unmutated round trip reproduces the source's own
/// chunking — and never an independent fact: the sample list is the truth. A grouping that does not
/// partition the current sample list is therefore DISCARDED for the one-chunk-per-track normal form
/// (adapted from remodel's `mp4_stsc`, which makes the same single-chunk simplification for its own
/// fixture muxer) rather than trusted. This is reachable by construction and not a defensive
/// nicety: `SetSnapshot` carries a whole caller-supplied document, and a caller that adds or drops
/// a sample cannot be expected to also restate a writer's chunking. Encoding a legal snapshot must
/// not depend on two fields being kept in step, so the encoder reconciles instead of aborting.
fn normalized_chunk_sample_counts(track: &Mp4Track) -> Vec<u32> {
    let covered = track.chunk_sample_counts.iter().map(|count| *count as usize).sum::<usize>();
    if track.chunk_sample_counts.is_empty() || covered != track.samples.len() {
        return vec![track.samples.len() as u32];
    }
    track.chunk_sample_counts.clone()
}

fn build_stsc(track: &Mp4Track) -> Vec<u8> {
    let counts = normalized_chunk_sample_counts(track);
    let mut entries = Vec::new();
    for (index, count) in counts.into_iter().enumerate() {
        if entries.last().is_some_and(|entry: &(u32, u32)| entry.1 == count) {
            continue;
        }
        entries.push((index as u32 + 1, count));
    }
    let mut payload = vec![0u8; 4];
    payload.extend_from_slice(&(entries.len() as u32).to_be_bytes());
    for (first_chunk, sample_count) in entries {
        payload.extend_from_slice(&first_chunk.to_be_bytes());
        payload.extend_from_slice(&sample_count.to_be_bytes());
        payload.extend_from_slice(&1u32.to_be_bytes());
    }
    write_box(b"stsc", &payload)
}

fn build_stsz(track: &Mp4Track) -> Vec<u8> {
    let sizes: Vec<u32> = track.samples.iter().map(|s| s.data.len() as u32).collect();
    let mut payload = vec![0u8; 4];
    let uniform = sizes.first().is_some_and(|&first| sizes.iter().all(|&s| s == first));
    if uniform && !sizes.is_empty() {
        payload.extend_from_slice(&sizes[0].to_be_bytes());
        payload.extend_from_slice(&(sizes.len() as u32).to_be_bytes());
    } else {
        payload.extend_from_slice(&0u32.to_be_bytes());
        payload.extend_from_slice(&(sizes.len() as u32).to_be_bytes());
        for s in &sizes {
            payload.extend_from_slice(&s.to_be_bytes());
        }
    }
    write_box(b"stsz", &payload)
}

fn build_stco(offsets: &[u32]) -> Vec<u8> {
    let mut payload = vec![0u8; 4];
    payload.extend_from_slice(&(offsets.len() as u32).to_be_bytes());
    for offset in offsets {
        payload.extend_from_slice(&offset.to_be_bytes());
    }
    write_box(b"stco", &payload)
}

fn build_stss(track: &Mp4Track) -> Vec<u8> {
    if track.samples.iter().all(|s| s.sync) {
        return Vec::new();
    }
    let indices: Vec<u32> = track.samples.iter().enumerate().filter(|(_, s)| s.sync).map(|(i, _)| i as u32 + 1).collect();
    let mut payload = vec![0u8; 4];
    payload.extend_from_slice(&(indices.len() as u32).to_be_bytes());
    for i in indices {
        payload.extend_from_slice(&i.to_be_bytes());
    }
    write_box(b"stss", &payload)
}

fn build_hdlr(track: &Mp4Track) -> Vec<u8> {
    let mut payload = vec![0u8; 8];
    payload.extend_from_slice(b"vide");
    payload.extend_from_slice(&[0u8; 12]);
    payload.extend_from_slice(track.metadata.handler_name.as_bytes());
    payload.push(0);
    write_box(b"hdlr", &payload)
}

/// ✍️ Real, spec-valid vmhd/dinf/dref — adapted addition (remodel's own fixture muxer omits
/// these for brevity; a genuinely conformant video `minf` needs them, so this artifact's encoder
/// adds them for real player/ffprobe compatibility).
fn build_vmhd() -> Vec<u8> {
    write_box(b"vmhd", &[0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0])
}
fn build_dinf() -> Vec<u8> {
    let url = write_box(b"url ", &[0, 0, 0, 1]);
    let mut dref_payload = vec![0u8; 4];
    dref_payload.extend_from_slice(&1u32.to_be_bytes());
    dref_payload.extend(url);
    write_box(b"dinf", &write_box(b"dref", &dref_payload))
}

fn packed_language(language: &str) -> u16 {
    let mut chars = language.bytes().chain(std::iter::repeat(b'`')).take(3).map(|byte| u16::from(byte.saturating_sub(0x60)) & 0x1f);
    (chars.next().unwrap_or(0) << 10) | (chars.next().unwrap_or(0) << 5) | chars.next().unwrap_or(0)
}

fn build_mdhd(track: &Mp4Track) -> Vec<u8> {
    let mut payload = vec![0u8; 4];
    payload.extend_from_slice(&(track.metadata.media_creation_time as u32).to_be_bytes());
    payload.extend_from_slice(&(track.metadata.media_modification_time as u32).to_be_bytes());
    payload.extend_from_slice(&track.timescale.to_be_bytes());
    payload.extend_from_slice(&(track.metadata.media_duration as u32).to_be_bytes());
    payload.extend_from_slice(&packed_language(&track.metadata.language).to_be_bytes());
    payload.extend_from_slice(&track.metadata.quality.to_be_bytes());
    write_box(b"mdhd", &payload)
}

fn build_tkhd(track: &Mp4Track) -> Vec<u8> {
    let mut payload = vec![0, (track.metadata.flags >> 16) as u8, (track.metadata.flags >> 8) as u8, track.metadata.flags as u8];
    payload.extend_from_slice(&(track.metadata.creation_time as u32).to_be_bytes());
    payload.extend_from_slice(&(track.metadata.modification_time as u32).to_be_bytes());
    payload.extend_from_slice(&track.track_id.to_be_bytes());
    payload.extend_from_slice(&[0u8; 4]);
    payload.extend_from_slice(&(track.metadata.duration as u32).to_be_bytes());
    payload.extend_from_slice(&[0u8; 8]);
    payload.extend_from_slice(&track.metadata.layer.to_be_bytes());
    payload.extend_from_slice(&track.metadata.alternate_group.to_be_bytes());
    payload.extend_from_slice(&track.metadata.volume.to_be_bytes());
    payload.extend_from_slice(&[0u8; 2]);
    for v in track.metadata.matrix {
        payload.extend_from_slice(&v.to_be_bytes());
    }
    payload.extend_from_slice(&(track.width << 16).to_be_bytes());
    payload.extend_from_slice(&(track.height << 16).to_be_bytes());
    write_box(b"tkhd", &payload)
}

fn build_edts(track: &Mp4Track) -> Vec<u8> {
    if track.metadata.edits.is_empty() {
        return Vec::new();
    }
    let mut payload = vec![0u8; 4];
    payload.extend_from_slice(&(track.metadata.edits.len() as u32).to_be_bytes());
    for edit in &track.metadata.edits {
        payload.extend_from_slice(&(edit.segment_duration as u32).to_be_bytes());
        payload.extend_from_slice(&(edit.media_time as i32).to_be_bytes());
        payload.extend_from_slice(&edit.media_rate_integer.to_be_bytes());
        payload.extend_from_slice(&edit.media_rate_fraction.to_be_bytes());
    }
    write_box(b"edts", &write_box(b"elst", &payload))
}

fn build_trak(track: &Mp4Track, chunk_offsets: &[u32]) -> Vec<u8> {
    let tkhd = build_tkhd(track);
    let stbl = write_box(b"stbl", &build_stbl(track, chunk_offsets));
    let minf = write_box(b"minf", &[build_vmhd(), build_dinf(), stbl].concat());
    let mdia = write_box(b"mdia", &[build_mdhd(track), build_hdlr(track), minf].concat());
    write_box(b"trak", &[tkhd, build_edts(track), mdia].concat())
}

fn build_mvhd(movie: &Mp4Movie) -> Vec<u8> {
    let mut payload = vec![0u8; 4];
    payload.extend_from_slice(&(movie.creation_time as u32).to_be_bytes());
    payload.extend_from_slice(&(movie.modification_time as u32).to_be_bytes());
    payload.extend_from_slice(&movie.timescale.to_be_bytes());
    payload.extend_from_slice(&(movie.duration as u32).to_be_bytes());
    payload.extend_from_slice(&movie.rate.to_be_bytes());
    payload.extend_from_slice(&movie.volume.to_be_bytes());
    payload.extend_from_slice(&[0u8; 2]);
    payload.extend_from_slice(&[0u8; 8]);
    for v in movie.matrix {
        payload.extend_from_slice(&v.to_be_bytes());
    }
    payload.extend_from_slice(&[0u8; 24]);
    payload.extend_from_slice(&movie.next_track_id.to_be_bytes());
    write_box(b"mvhd", &payload)
}

fn build_metadata_item(fourcc: &[u8; 4], value: &str) -> Vec<u8> {
    let mut data = vec![0, 0, 0, 1, 0, 0, 0, 0];
    data.extend_from_slice(value.as_bytes());
    write_box(fourcc, &write_box(b"data", &data))
}

fn build_udta(movie: &Mp4Movie) -> Vec<u8> {
    if movie.title.is_none() && movie.encoder.is_none() {
        return Vec::new();
    }
    let mut ilst = Vec::new();
    if let Some(title) = &movie.title {
        ilst.extend(build_metadata_item(&[0xa9, b'n', b'a', b'm'], title));
    }
    if let Some(encoder) = &movie.encoder {
        ilst.extend(build_metadata_item(&[0xa9, b't', b'o', b'o'], encoder));
    }
    let mut handler = vec![0u8; 8];
    handler.extend_from_slice(b"mdir");
    handler.extend_from_slice(b"appl");
    handler.extend_from_slice(&[0u8; 8]);
    handler.push(0);
    let meta_payload = [vec![0u8; 4], write_box(b"hdlr", &handler), write_box(b"ilst", &ilst)].concat();
    write_box(b"udta", &write_box(b"meta", &meta_payload))
}

fn build_moov(snapshot: &Mp4Snapshot, mdat_data_offset: u32) -> Vec<u8> {
    let mut offset = mdat_data_offset;
    let mut traks = Vec::new();
    for track in &snapshot.tracks {
        let mut sample_index = 0usize;
        let mut chunk_offsets = Vec::new();
        for count in normalized_chunk_sample_counts(track) {
            chunk_offsets.push(offset);
            for sample in &track.samples[sample_index..sample_index + count as usize] {
                offset = offset.checked_add(sample.data.len() as u32).expect("MP4 media offset overflow");
            }
            sample_index += count as usize;
        }
        traks.extend(build_trak(track, &chunk_offsets));
    }
    write_box(b"moov", &[build_mvhd(&snapshot.movie), traks, build_udta(&snapshot.movie)].concat())
}

/// ✍️ Real ISO-BMFF encode from `Mp4Snapshot` (see this module's doc comment for the exact
/// codec_retention_law scope: logical `ftyp` and semantic sample payloads are preserved; `moov`
/// internals are a fresh, spec-valid rebuild). The deterministic layout is `ftyp`, `moov`,
/// a canonical empty `free`, and `mdat`; a first pass measures `moov`, and the second writes
/// the resulting logical chunk offsets.
pub fn encode_mp4(snapshot: &Mp4Snapshot) -> Vec<u8> {
    let mut major_brand_bytes = [b' '; 4];
    for (i, b) in snapshot.ftyp.major_brand.as_bytes().iter().take(4).enumerate() {
        major_brand_bytes[i] = *b;
    }
    let mut ftyp_payload = Vec::new();
    ftyp_payload.extend_from_slice(&major_brand_bytes);
    ftyp_payload.extend_from_slice(&snapshot.ftyp.minor_version.to_be_bytes());
    for brand in &snapshot.ftyp.compatible_brands {
        let mut b4 = [b' '; 4];
        for (i, b) in brand.as_bytes().iter().take(4).enumerate() {
            b4[i] = *b;
        }
        ftyp_payload.extend_from_slice(&b4);
    }
    let ftyp = write_box(b"ftyp", &ftyp_payload);

    let all_sample_bytes: Vec<u8> = snapshot.tracks.iter().flat_map(|t| t.samples.iter().flat_map(|s| s.data.clone())).collect();
    let mdat = write_box(b"mdat", &all_sample_bytes);
    let measured_moov = build_moov(snapshot, 0);
    let free = write_box(b"free", &[]);
    let mdat_data_offset = u32::try_from(ftyp.len() + measured_moov.len() + free.len() + 8).expect("MP4 media offset exceeds stco range");
    let moov = build_moov(snapshot, mdat_data_offset);

    [ftyp, moov, free, mdat].concat()
}

/// 🧵️ One bounded advance of the native ISO-BMFF serializer.
#[derive(Debug, PartialEq, Eq)]
pub enum Mp4EncodeAdvance {
    Progress,
    Chunk(Vec<u8>),
    Complete,
}

#[derive(Clone, Debug, Default)]
struct Mp4TrackPlan {
    sample_bytes: u64,
    chunk_count: usize,
    retained_chunks: bool,
    stsc_entries: usize,
    stts_runs: usize,
    ctts_runs: usize,
    sync_samples: usize,
    all_sync: bool,
    ctts_present: bool,
    ctts_version: u8,
    uniform_sample_size: Option<u32>,
    codec_bytes: usize,
    stbl_bytes: usize,
    trak_bytes: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mp4MeasurePhase {
    TrackChunks,
    TrackSamples,
    TrackStsc,
    CodecSps,
    CodecPps,
    CodecAvcExtension,
    CodecHevcArrays,
    CodecHevcNals,
    FinalizeTrack,
    Finalize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mp4EmitPhase {
    FtypHeader,
    FtypFixed,
    FtypBrand,
    MoovHeader,
    Mvhd,
    TrackHeader,
    Tkhd,
    EdtsHeader,
    ElstHeader,
    Edit,
    MdiaHeader,
    Mdhd,
    HdlrHeader,
    HdlrFixed,
    HdlrName,
    HdlrNull,
    MinfHeader,
    Vmhd,
    Dinf,
    StblHeader,
    StsdHeader,
    SampleEntryHeader,
    SampleEntryFixed,
    CodecHeader,
    CodecFixed,
    CodecNalLength,
    CodecNalData,
    CodecCount,
    CodecExtensionFixed,
    CodecHevcArray,
    CodecHevcNalLength,
    CodecHevcNalData,
    CodecExtensions,
    SttsHeader,
    SttsRunMeasure,
    SttsRunEmit,
    StssHeader,
    StssEntry,
    CttsHeader,
    CttsRunMeasure,
    CttsRunEmit,
    StscHeader,
    StscEntry,
    StszHeader,
    StszEntry,
    StcoHeader,
    StcoEntry,
    StcoAdvance,
    TrackDone,
    UdtaHeader,
    MetaHeader,
    MetaPrefix,
    IlstHeader,
    TitleHeader,
    TitleData,
    EncoderHeader,
    EncoderData,
    Free,
    MdatHeader,
    MdatSample,
    Complete,
}

fn mp4_box_size(payload: usize) -> Result<usize, String> {
    let size = payload.checked_add(8).ok_or("mp4.encode.box-size-overflow")?;
    u32::try_from(size).map_err(|_| "mp4.encode.box-size-overflow".to_string())?;
    Ok(size)
}

fn mp4_box_header(kind: &[u8; 4], total: usize) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::with_capacity(8);
    bytes.extend_from_slice(&u32::try_from(total).map_err(|_| "mp4.encode.box-size-overflow")?.to_be_bytes());
    bytes.extend_from_slice(kind);
    Ok(bytes)
}

fn mp4_codec_payload_base(codec: &Mp4Codec) -> usize {
    if codec.format.is_avc() { 7 + usize::from(codec.extension.is_some()) * 4 } else if codec.format.is_hevc() { 23 } else { 0 }
}

fn mp4_codec_extension_bytes(track: &Mp4Track) -> usize {
    track.metadata.color.as_ref().map_or(0, |color| 18 + usize::from(color.full_range.is_some())) + usize::from(track.metadata.pixel_aspect_ratio.is_some()) * 16 + usize::from(track.metadata.bitrate.is_some()) * 20
}

fn mp4_metadata_item_size(value: &str) -> Result<usize, String> {
    mp4_box_size(mp4_box_size(8usize.checked_add(value.len()).ok_or("mp4.encode.metadata-size-overflow")?)?)
}

fn mp4_udta_size(movie: &Mp4Movie) -> Result<usize, String> {
    if movie.title.is_none() && movie.encoder.is_none() {
        return Ok(0);
    }
    let items = movie.title.as_deref().map(mp4_metadata_item_size).transpose()?.unwrap_or(0).checked_add(movie.encoder.as_deref().map(mp4_metadata_item_size).transpose()?.unwrap_or(0)).ok_or("mp4.encode.metadata-size-overflow")?;
    mp4_box_size(mp4_box_size(4usize.checked_add(33).and_then(|value| value.checked_add(mp4_box_size(items).ok()?)).ok_or("mp4.encode.metadata-size-overflow")?)?)
}

/// 🎚️ Incrementally serializes retained ISO-BMFF structure without materializing encoded media or sample tables.
pub struct Mp4EncodeCursor {
    measure: Option<Mp4MeasurePhase>,
    emit: Mp4EmitPhase,
    plans: Vec<Mp4TrackPlan>,
    working: Mp4TrackPlan,
    track_index: usize,
    item_index: usize,
    sub_index: usize,
    offset: usize,
    run_count: u32,
    run_u32: u32,
    run_i32: i32,
    chunk_sum: usize,
    previous_chunk_count: Option<u32>,
    moov_bytes: usize,
    mdat_bytes: u64,
    mdat_data_offset: u32,
    stco_sample_index: usize,
    stco_sample_remaining: usize,
    stco_offset: u64,
    emitted_bytes: u64,
}

impl Mp4EncodeCursor {
    pub fn new(_: &Mp4Snapshot) -> Self {
        Self {
            measure: Some(Mp4MeasurePhase::TrackChunks),
            emit: Mp4EmitPhase::FtypHeader,
            plans: Vec::new(),
            working: Mp4TrackPlan { all_sync: true, ..Mp4TrackPlan::default() },
            track_index: 0,
            item_index: 0,
            sub_index: 0,
            offset: 0,
            run_count: 0,
            run_u32: 0,
            run_i32: 0,
            chunk_sum: 0,
            previous_chunk_count: None,
            moov_bytes: 0,
            mdat_bytes: 0,
            mdat_data_offset: 0,
            stco_sample_index: 0,
            stco_sample_remaining: 0,
            stco_offset: 0,
            emitted_bytes: 0,
        }
    }

    pub fn emitted_bytes(&self) -> u64 { self.emitted_bytes }

    pub fn advance(&mut self, snapshot: &Mp4Snapshot, maximum_bytes: usize) -> Result<Mp4EncodeAdvance, String> {
        if maximum_bytes == 0 { return Err("mp4.encode.zero-byte-grant".into()); }
        if let Some(phase) = self.measure { return self.measure(snapshot, phase); }
        self.emit(snapshot, maximum_bytes)
    }

    fn measure(&mut self, snapshot: &Mp4Snapshot, phase: Mp4MeasurePhase) -> Result<Mp4EncodeAdvance, String> {
        let Some(track) = snapshot.tracks.get(self.track_index) else {
            self.measure = Some(Mp4MeasurePhase::Finalize);
            return self.finalize_measurement(snapshot);
        };
        match phase {
            Mp4MeasurePhase::TrackChunks => {
                if let Some(count) = track.chunk_sample_counts.get(self.item_index) {
                    self.chunk_sum = self.chunk_sum.checked_add(*count as usize).ok_or("mp4.encode.chunk-count-overflow")?;
                    self.item_index += 1;
                } else {
                    self.working.retained_chunks = !track.chunk_sample_counts.is_empty() && self.chunk_sum == track.samples.len();
                    self.working.chunk_count = if self.working.retained_chunks { track.chunk_sample_counts.len() } else { 1 };
                    self.item_index = 0;
                    self.measure = Some(Mp4MeasurePhase::TrackSamples);
                }
            }
            Mp4MeasurePhase::TrackSamples => {
                if let Some(sample) = track.samples.get(self.item_index) {
                    let size = u32::try_from(sample.data.len()).map_err(|_| "mp4.encode.sample-too-large")?;
                    self.working.sample_bytes = self.working.sample_bytes.checked_add(u64::from(size)).ok_or("mp4.encode.mdat-size-overflow")?;
                    if self.item_index == 0 || track.samples[self.item_index - 1].duration != sample.duration { self.working.stts_runs += 1; }
                    if self.item_index == 0 || track.samples[self.item_index - 1].cts_offset != sample.cts_offset { self.working.ctts_runs += 1; }
                    self.working.all_sync &= sample.sync;
                    self.working.sync_samples += usize::from(sample.sync);
                    self.working.ctts_present |= sample.cts_offset != 0;
                    self.working.ctts_version |= u8::from(sample.cts_offset < 0);
                    self.working.uniform_sample_size = match (self.item_index, self.working.uniform_sample_size) { (0, _) => Some(size), (_, Some(first)) if first == size => Some(first), _ => None };
                    self.item_index += 1;
                } else {
                    self.item_index = 0;
                    self.previous_chunk_count = None;
                    self.measure = Some(Mp4MeasurePhase::TrackStsc);
                }
            }
            Mp4MeasurePhase::TrackStsc => {
                if self.item_index < self.working.chunk_count {
                    let count = if self.working.retained_chunks { track.chunk_sample_counts[self.item_index] } else { track.samples.len() as u32 };
                    if self.previous_chunk_count != Some(count) { self.working.stsc_entries += 1; self.previous_chunk_count = Some(count); }
                    self.item_index += 1;
                } else {
                    self.working.codec_bytes = mp4_codec_payload_base(&track.codec);
                    self.item_index = 0;
                    self.measure = Some(if track.codec.format.is_avc() { Mp4MeasurePhase::CodecSps } else if track.codec.format.is_hevc() { Mp4MeasurePhase::CodecHevcArrays } else { Mp4MeasurePhase::FinalizeTrack });
                }
            }
            Mp4MeasurePhase::CodecSps => {
                if let Some(nal) = track.codec.sps.get(self.item_index) { self.working.codec_bytes = self.working.codec_bytes.checked_add(2 + nal.len()).ok_or("mp4.encode.codec-size-overflow")?; self.item_index += 1; } else { self.item_index = 0; self.measure = Some(Mp4MeasurePhase::CodecPps); }
            }
            Mp4MeasurePhase::CodecPps => {
                if let Some(nal) = track.codec.pps.get(self.item_index) { self.working.codec_bytes = self.working.codec_bytes.checked_add(2 + nal.len()).ok_or("mp4.encode.codec-size-overflow")?; self.item_index += 1; } else { self.item_index = 0; self.measure = Some(if track.codec.extension.is_some() { Mp4MeasurePhase::CodecAvcExtension } else { Mp4MeasurePhase::FinalizeTrack }); }
            }
            Mp4MeasurePhase::CodecAvcExtension => {
                let extension = track.codec.extension.as_ref().ok_or("mp4.encode.avc-extension-missing")?;
                if let Some(nal) = extension.sps_ext.get(self.item_index) { self.working.codec_bytes = self.working.codec_bytes.checked_add(2 + nal.len()).ok_or("mp4.encode.codec-size-overflow")?; self.item_index += 1; } else { self.measure = Some(Mp4MeasurePhase::FinalizeTrack); }
            }
            Mp4MeasurePhase::CodecHevcArrays => {
                let Some(config) = track.codec.hevc.as_ref() else { self.measure = Some(Mp4MeasurePhase::FinalizeTrack); return Ok(Mp4EncodeAdvance::Progress); };
                if let Some(array) = config.arrays.get(self.item_index) { self.working.codec_bytes = self.working.codec_bytes.checked_add(3).ok_or("mp4.encode.codec-size-overflow")?; if array.nal_units.is_empty() { self.item_index += 1; } else { self.sub_index = 0; self.measure = Some(Mp4MeasurePhase::CodecHevcNals); } } else { self.measure = Some(Mp4MeasurePhase::FinalizeTrack); }
            }
            Mp4MeasurePhase::CodecHevcNals => {
                let array = track.codec.hevc.as_ref().and_then(|config| config.arrays.get(self.item_index)).ok_or("mp4.encode.hevc-array-missing")?;
                if let Some(nal) = array.nal_units.get(self.sub_index) { self.working.codec_bytes = self.working.codec_bytes.checked_add(2 + nal.len()).ok_or("mp4.encode.codec-size-overflow")?; self.sub_index += 1; } else { self.item_index += 1; self.measure = Some(Mp4MeasurePhase::CodecHevcArrays); }
            }
            Mp4MeasurePhase::FinalizeTrack => self.finalize_track(track)?,
            Mp4MeasurePhase::Finalize => return self.finalize_measurement(snapshot),
        }
        Ok(Mp4EncodeAdvance::Progress)
    }

    fn finalize_track(&mut self, track: &Mp4Track) -> Result<(), String> {
        let codec = if track.codec.format.is_jpeg() { 0 } else { mp4_box_size(self.working.codec_bytes)? };
        let sample_entry = mp4_box_size(78usize.checked_add(codec).and_then(|value| value.checked_add(mp4_codec_extension_bytes(track))).ok_or("mp4.encode.stsd-size-overflow")?)?;
        let stsd = mp4_box_size(8usize.checked_add(sample_entry).ok_or("mp4.encode.stsd-size-overflow")?)?;
        let stts = mp4_box_size(8usize.checked_add(self.working.stts_runs.checked_mul(8).ok_or("mp4.encode.stts-size-overflow")?).ok_or("mp4.encode.stts-size-overflow")?)?;
        let stss = if self.working.all_sync { 0 } else { mp4_box_size(8usize.checked_add(self.working.sync_samples.checked_mul(4).ok_or("mp4.encode.stss-size-overflow")?).ok_or("mp4.encode.stss-size-overflow")?)? };
        let ctts = if self.working.ctts_present { mp4_box_size(8usize.checked_add(self.working.ctts_runs.checked_mul(8).ok_or("mp4.encode.ctts-size-overflow")?).ok_or("mp4.encode.ctts-size-overflow")?)? } else { 0 };
        let stsc = mp4_box_size(8usize.checked_add(self.working.stsc_entries.checked_mul(12).ok_or("mp4.encode.stsc-size-overflow")?).ok_or("mp4.encode.stsc-size-overflow")?)?;
        let stsz = mp4_box_size(12usize.checked_add(if self.working.uniform_sample_size.is_some() && !track.samples.is_empty() { 0 } else { track.samples.len().checked_mul(4).ok_or("mp4.encode.stsz-size-overflow")? }).ok_or("mp4.encode.stsz-size-overflow")?)?;
        let stco = mp4_box_size(8usize.checked_add(self.working.chunk_count.checked_mul(4).ok_or("mp4.encode.stco-size-overflow")?).ok_or("mp4.encode.stco-size-overflow")?)?;
        self.working.stbl_bytes = mp4_box_size(stsd.checked_add(stts).and_then(|value| value.checked_add(stss)).and_then(|value| value.checked_add(ctts)).and_then(|value| value.checked_add(stsc)).and_then(|value| value.checked_add(stsz)).and_then(|value| value.checked_add(stco)).ok_or("mp4.encode.stbl-size-overflow")?)?;
        let hdlr = mp4_box_size(25usize.checked_add(track.metadata.handler_name.len()).ok_or("mp4.encode.hdlr-size-overflow")?)?;
        let minf = mp4_box_size(20usize.checked_add(36).and_then(|value| value.checked_add(self.working.stbl_bytes)).ok_or("mp4.encode.minf-size-overflow")?)?;
        let mdia = mp4_box_size(32usize.checked_add(hdlr).and_then(|value| value.checked_add(minf)).ok_or("mp4.encode.mdia-size-overflow")?)?;
        let edts = if track.metadata.edits.is_empty() { 0 } else { mp4_box_size(mp4_box_size(8usize.checked_add(track.metadata.edits.len().checked_mul(12).ok_or("mp4.encode.edts-size-overflow")?).ok_or("mp4.encode.edts-size-overflow")?)?)? };
        self.working.trak_bytes = mp4_box_size(92usize.checked_add(edts).and_then(|value| value.checked_add(mdia)).ok_or("mp4.encode.trak-size-overflow")?)?;
        self.mdat_bytes = self.mdat_bytes.checked_add(self.working.sample_bytes).ok_or("mp4.encode.mdat-size-overflow")?;
        self.plans.push(std::mem::replace(&mut self.working, Mp4TrackPlan { all_sync: true, ..Mp4TrackPlan::default() }));
        self.track_index += 1;
        self.item_index = 0;
        self.sub_index = 0;
        self.chunk_sum = 0;
        self.previous_chunk_count = None;
        self.measure = Some(Mp4MeasurePhase::TrackChunks);
        Ok(())
    }

    fn finalize_measurement(&mut self, snapshot: &Mp4Snapshot) -> Result<Mp4EncodeAdvance, String> {
        let tracks = self.plans.iter().try_fold(0usize, |sum, plan| sum.checked_add(plan.trak_bytes).ok_or("mp4.encode.moov-size-overflow"))?;
        self.moov_bytes = mp4_box_size(108usize.checked_add(tracks).and_then(|value| value.checked_add(mp4_udta_size(&snapshot.movie).ok()?)).ok_or("mp4.encode.moov-size-overflow")?)?;
        let ftyp = mp4_box_size(8usize.checked_add(snapshot.ftyp.compatible_brands.len().checked_mul(4).ok_or("mp4.encode.ftyp-size-overflow")?).ok_or("mp4.encode.ftyp-size-overflow")?)?;
        self.mdat_data_offset = u32::try_from(ftyp.checked_add(self.moov_bytes).and_then(|value| value.checked_add(16)).ok_or("mp4.encode.media-offset-overflow")?).map_err(|_| "mp4.encode.media-offset-overflow")?;
        mp4_box_size(usize::try_from(self.mdat_bytes).map_err(|_| "mp4.encode.mdat-size-overflow")?)?;
        self.measure = None;
        self.track_index = 0;
        self.item_index = 0;
        self.sub_index = 0;
        Ok(Mp4EncodeAdvance::Progress)
    }

    fn emit(&mut self, snapshot: &Mp4Snapshot, maximum_bytes: usize) -> Result<Mp4EncodeAdvance, String> {
        match self.emit {
            Mp4EmitPhase::FtypHeader => {
                let size = mp4_box_size(8 + snapshot.ftyp.compatible_brands.len() * 4)?;
                self.emit_owned(mp4_box_header(b"ftyp", size)?, maximum_bytes, Mp4EmitPhase::FtypFixed)
            }
            Mp4EmitPhase::FtypFixed => {
                let mut bytes = [b' '; 8];
                for (index, byte) in snapshot.ftyp.major_brand.as_bytes().iter().take(4).enumerate() { bytes[index] = *byte; }
                bytes[4..].copy_from_slice(&snapshot.ftyp.minor_version.to_be_bytes());
                self.item_index = 0;
                self.emit_owned(bytes.to_vec(), maximum_bytes, Mp4EmitPhase::FtypBrand)
            }
            Mp4EmitPhase::FtypBrand => {
                let Some(brand) = snapshot.ftyp.compatible_brands.get(self.item_index) else { self.emit = Mp4EmitPhase::MoovHeader; return Ok(Mp4EncodeAdvance::Progress); };
                let mut bytes = [b' '; 4];
                for (index, byte) in brand.as_bytes().iter().take(4).enumerate() { bytes[index] = *byte; }
                self.item_index += 1;
                self.emit_owned(bytes.to_vec(), maximum_bytes, Mp4EmitPhase::FtypBrand)
            }
            Mp4EmitPhase::MoovHeader => {
                self.track_index = 0;
                self.emit_owned(mp4_box_header(b"moov", self.moov_bytes)?, maximum_bytes, Mp4EmitPhase::Mvhd)
            }
            Mp4EmitPhase::Mvhd => self.emit_owned(build_mvhd(&snapshot.movie), maximum_bytes, Mp4EmitPhase::TrackHeader),
            Mp4EmitPhase::TrackHeader => {
                let Some(plan) = self.plans.get(self.track_index) else { self.emit = Mp4EmitPhase::UdtaHeader; return Ok(Mp4EncodeAdvance::Progress); };
                self.emit_owned(mp4_box_header(b"trak", plan.trak_bytes)?, maximum_bytes, Mp4EmitPhase::Tkhd)
            }
            Mp4EmitPhase::Tkhd => self.emit_owned(build_tkhd(self.track(snapshot)?), maximum_bytes, Mp4EmitPhase::EdtsHeader),
            Mp4EmitPhase::EdtsHeader => {
                let track = self.track(snapshot)?;
                if track.metadata.edits.is_empty() { self.emit = Mp4EmitPhase::MdiaHeader; return Ok(Mp4EncodeAdvance::Progress); }
                let size = mp4_box_size(mp4_box_size(8 + track.metadata.edits.len() * 12)?)?;
                self.emit_owned(mp4_box_header(b"edts", size)?, maximum_bytes, Mp4EmitPhase::ElstHeader)
            }
            Mp4EmitPhase::ElstHeader => {
                let track = self.track(snapshot)?;
                let size = mp4_box_size(8 + track.metadata.edits.len() * 12)?;
                let mut bytes = mp4_box_header(b"elst", size)?;
                bytes.extend_from_slice(&[0; 4]);
                bytes.extend_from_slice(&(track.metadata.edits.len() as u32).to_be_bytes());
                self.item_index = 0;
                self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::Edit)
            }
            Mp4EmitPhase::Edit => {
                let Some(edit) = self.track(snapshot)?.metadata.edits.get(self.item_index) else { self.emit = Mp4EmitPhase::MdiaHeader; return Ok(Mp4EncodeAdvance::Progress); };
                let mut bytes = Vec::with_capacity(12);
                bytes.extend_from_slice(&(edit.segment_duration as u32).to_be_bytes());
                bytes.extend_from_slice(&(edit.media_time as i32).to_be_bytes());
                bytes.extend_from_slice(&edit.media_rate_integer.to_be_bytes());
                bytes.extend_from_slice(&edit.media_rate_fraction.to_be_bytes());
                self.item_index += 1;
                self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::Edit)
            }
            Mp4EmitPhase::MdiaHeader => {
                let track = self.track(snapshot)?;
                let edts = if track.metadata.edits.is_empty() { 0 } else { mp4_box_size(mp4_box_size(8 + track.metadata.edits.len() * 12)?)? };
                let size = self.plan()?.trak_bytes.checked_sub(8 + 92 + edts).ok_or("mp4.encode.mdia-size-underflow")?;
                self.emit_owned(mp4_box_header(b"mdia", size)?, maximum_bytes, Mp4EmitPhase::Mdhd)
            }
            Mp4EmitPhase::Mdhd => self.emit_owned(build_mdhd(self.track(snapshot)?), maximum_bytes, Mp4EmitPhase::HdlrHeader),
            Mp4EmitPhase::HdlrHeader => {
                let size = mp4_box_size(25 + self.track(snapshot)?.metadata.handler_name.len())?;
                self.emit_owned(mp4_box_header(b"hdlr", size)?, maximum_bytes, Mp4EmitPhase::HdlrFixed)
            }
            Mp4EmitPhase::HdlrFixed => {
                let mut bytes = vec![0; 8];
                bytes.extend_from_slice(b"vide");
                bytes.extend_from_slice(&[0; 12]);
                self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::HdlrName)
            }
            Mp4EmitPhase::HdlrName => {
                let name = self.track(snapshot)?.metadata.handler_name.as_bytes();
                self.emit_borrowed(name, maximum_bytes, Mp4EmitPhase::HdlrNull)
            }
            Mp4EmitPhase::HdlrNull => self.emit_owned(vec![0], maximum_bytes, Mp4EmitPhase::MinfHeader),
            Mp4EmitPhase::MinfHeader => {
                let hdlr = mp4_box_size(25 + self.track(snapshot)?.metadata.handler_name.len())?;
                let track = self.track(snapshot)?;
                let edts = if track.metadata.edits.is_empty() { 0 } else { mp4_box_size(mp4_box_size(8 + track.metadata.edits.len() * 12)?)? };
                let mdia = self.plan()?.trak_bytes - 8 - 92 - edts;
                let size = mdia.checked_sub(8 + 32 + hdlr).ok_or("mp4.encode.minf-size-underflow")?;
                self.emit_owned(mp4_box_header(b"minf", size)?, maximum_bytes, Mp4EmitPhase::Vmhd)
            }
            Mp4EmitPhase::Vmhd => self.emit_owned(build_vmhd(), maximum_bytes, Mp4EmitPhase::Dinf),
            Mp4EmitPhase::Dinf => self.emit_owned(build_dinf(), maximum_bytes, Mp4EmitPhase::StblHeader),
            Mp4EmitPhase::StblHeader => self.emit_owned(mp4_box_header(b"stbl", self.plan()?.stbl_bytes)?, maximum_bytes, Mp4EmitPhase::StsdHeader),
            Mp4EmitPhase::StsdHeader => {
                let track = self.track(snapshot)?;
                let codec = if track.codec.format.is_jpeg() { 0 } else { mp4_box_size(self.plan()?.codec_bytes)? };
                let entry = mp4_box_size(78 + codec + mp4_codec_extension_bytes(track))?;
                let mut bytes = mp4_box_header(b"stsd", mp4_box_size(8 + entry)?)?;
                bytes.extend_from_slice(&[0; 4]);
                bytes.extend_from_slice(&1u32.to_be_bytes());
                self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::SampleEntryHeader)
            }
            Mp4EmitPhase::SampleEntryHeader => {
                let track = self.track(snapshot)?;
                let codec = if track.codec.format.is_jpeg() { 0 } else { mp4_box_size(self.plan()?.codec_bytes)? };
                let size = mp4_box_size(78 + codec + mp4_codec_extension_bytes(track))?;
                let mut kind = [0; 4]; kind.copy_from_slice(track.codec.format.fourcc().as_bytes());
                self.emit_owned(mp4_box_header(&kind, size)?, maximum_bytes, Mp4EmitPhase::SampleEntryFixed)
            }
            Mp4EmitPhase::SampleEntryFixed => {
                let track = self.track(snapshot)?;
                let mut kind = [0; 4]; kind.copy_from_slice(track.codec.format.fourcc().as_bytes());
                let bytes = mp4_visual_sample_entry(&kind, track.width as u16, track.height as u16, &track.metadata.visual, &[])[8..].to_vec();
                self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::CodecHeader)
            }
            Mp4EmitPhase::CodecHeader => {
                let track = self.track(snapshot)?;
                if track.codec.format.is_jpeg() { self.emit = Mp4EmitPhase::CodecExtensions; return Ok(Mp4EncodeAdvance::Progress); }
                let kind = if track.codec.format.is_avc() { b"avcC" } else { b"hvcC" };
                self.emit_owned(mp4_box_header(kind, mp4_box_size(self.plan()?.codec_bytes)?)?, maximum_bytes, Mp4EmitPhase::CodecFixed)
            }
            Mp4EmitPhase::CodecFixed => {
                let track = self.track(snapshot)?;
                self.item_index = 0;
                self.sub_index = 0;
                if track.codec.format.is_avc() {
                    let (profile, compatibility, level) = track.codec.sps.first().and_then(|sps| sps.get(1..4)).map_or((66, 0, 30), |bytes| (bytes[0], bytes[1], bytes[2]));
                    self.emit_owned(vec![1, profile, compatibility, level, 0xfc | (track.codec.nal_length_size.saturating_sub(1) & 3), 0xe0 | (track.codec.sps.len() as u8 & 0x1f)], maximum_bytes, Mp4EmitPhase::CodecNalLength)
                } else {
                    let bytes = mp4_hevc_fixed(&track.codec);
                    self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::CodecHevcArray)
                }
            }
            Mp4EmitPhase::CodecNalLength => {
                let track = self.track(snapshot)?;
                let list = match self.sub_index { 0 => &track.codec.sps, 1 => &track.codec.pps, 2 => &track.codec.extension.as_ref().ok_or("mp4.encode.avc-extension-missing")?.sps_ext, _ => return Err("mp4.encode.avc-list-invalid".into()) };
                let Some(nal) = list.get(self.item_index) else {
                    self.item_index = 0;
                    self.emit = match self.sub_index { 0 => Mp4EmitPhase::CodecCount, 1 if track.codec.extension.is_some() => Mp4EmitPhase::CodecExtensionFixed, 1 | 2 => Mp4EmitPhase::CodecExtensions, _ => return Err("mp4.encode.avc-list-invalid".into()) };
                    return Ok(Mp4EncodeAdvance::Progress);
                };
                self.emit_owned((nal.len() as u16).to_be_bytes().to_vec(), maximum_bytes, Mp4EmitPhase::CodecNalData)
            }
            Mp4EmitPhase::CodecNalData => {
                let track = self.track(snapshot)?;
                let list = match self.sub_index { 0 => &track.codec.sps, 1 => &track.codec.pps, 2 => &track.codec.extension.as_ref().ok_or("mp4.encode.avc-extension-missing")?.sps_ext, _ => return Err("mp4.encode.avc-list-invalid".into()) };
                let nal = list.get(self.item_index).ok_or("mp4.encode.avc-nal-missing")?;
                if self.offset + maximum_bytes >= nal.len() { self.item_index += 1; }
                self.emit_borrowed(nal, maximum_bytes, Mp4EmitPhase::CodecNalLength)
            }
            Mp4EmitPhase::CodecCount => {
                self.sub_index = 1;
                self.item_index = 0;
                self.emit_owned(vec![self.track(snapshot)?.codec.pps.len() as u8], maximum_bytes, Mp4EmitPhase::CodecNalLength)
            }
            Mp4EmitPhase::CodecExtensionFixed => {
                let extension = self.track(snapshot)?.codec.extension.as_ref().ok_or("mp4.encode.avc-extension-missing")?;
                self.sub_index = 2;
                self.item_index = 0;
                self.emit_owned(vec![0xfc | (extension.chroma_format & 3), 0xf8 | (extension.bit_depth_luma_minus8 & 7), 0xf8 | (extension.bit_depth_chroma_minus8 & 7), extension.sps_ext.len() as u8], maximum_bytes, Mp4EmitPhase::CodecNalLength)
            }
            Mp4EmitPhase::CodecHevcArray => {
                let Some(config) = self.track(snapshot)?.codec.hevc.as_ref() else { self.emit = Mp4EmitPhase::CodecExtensions; return Ok(Mp4EncodeAdvance::Progress); };
                let Some(array) = config.arrays.get(self.item_index) else { self.emit = Mp4EmitPhase::CodecExtensions; return Ok(Mp4EncodeAdvance::Progress); };
                self.sub_index = 0;
                let mut bytes = vec![(u8::from(array.array_completeness) << 7) | (array.nal_unit_type & 0x3f)];
                bytes.extend_from_slice(&(array.nal_units.len() as u16).to_be_bytes());
                let next = if array.nal_units.is_empty() { self.item_index += 1; Mp4EmitPhase::CodecHevcArray } else { Mp4EmitPhase::CodecHevcNalLength };
                self.emit_owned(bytes, maximum_bytes, next)
            }
            Mp4EmitPhase::CodecHevcNalLength => {
                let array = self.track(snapshot)?.codec.hevc.as_ref().and_then(|config| config.arrays.get(self.item_index)).ok_or("mp4.encode.hevc-array-missing")?;
                let Some(nal) = array.nal_units.get(self.sub_index) else { self.item_index += 1; self.emit = Mp4EmitPhase::CodecHevcArray; return Ok(Mp4EncodeAdvance::Progress); };
                self.emit_owned((nal.len() as u16).to_be_bytes().to_vec(), maximum_bytes, Mp4EmitPhase::CodecHevcNalData)
            }
            Mp4EmitPhase::CodecHevcNalData => {
                let nal = self.track(snapshot)?.codec.hevc.as_ref().and_then(|config| config.arrays.get(self.item_index)).and_then(|array| array.nal_units.get(self.sub_index)).ok_or("mp4.encode.hevc-nal-missing")?;
                if self.offset + maximum_bytes >= nal.len() { self.sub_index += 1; }
                self.emit_borrowed(nal, maximum_bytes, Mp4EmitPhase::CodecHevcNalLength)
            }
            Mp4EmitPhase::CodecExtensions => {
                let bytes = build_codec_extensions(self.track(snapshot)?);
                if bytes.is_empty() { self.emit = Mp4EmitPhase::SttsHeader; return Ok(Mp4EncodeAdvance::Progress); }
                self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::SttsHeader)
            }
            Mp4EmitPhase::SttsHeader => {
                let mut bytes = mp4_box_header(b"stts", mp4_box_size(8 + self.plan()?.stts_runs * 8)?)?;
                bytes.extend_from_slice(&[0; 4]); bytes.extend_from_slice(&(self.plan()?.stts_runs as u32).to_be_bytes());
                self.item_index = 0; self.run_count = 0;
                self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::SttsRunMeasure)
            }
            Mp4EmitPhase::SttsRunMeasure => self.measure_emit_u32_run(snapshot, true),
            Mp4EmitPhase::SttsRunEmit => {
                let mut bytes = self.run_count.to_be_bytes().to_vec(); bytes.extend_from_slice(&self.run_u32.to_be_bytes()); self.run_count = 0;
                self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::SttsRunMeasure)
            }
            Mp4EmitPhase::StssHeader => {
                if self.plan()?.all_sync { self.emit = Mp4EmitPhase::CttsHeader; return Ok(Mp4EncodeAdvance::Progress); }
                let mut bytes = mp4_box_header(b"stss", mp4_box_size(8 + self.plan()?.sync_samples * 4)?)?;
                bytes.extend_from_slice(&[0; 4]); bytes.extend_from_slice(&(self.plan()?.sync_samples as u32).to_be_bytes()); self.item_index = 0;
                self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::StssEntry)
            }
            Mp4EmitPhase::StssEntry => {
                let track = self.track(snapshot)?;
                let Some(sample) = track.samples.get(self.item_index) else { self.emit = Mp4EmitPhase::CttsHeader; return Ok(Mp4EncodeAdvance::Progress); };
                self.item_index += 1;
                if !sample.sync { return Ok(Mp4EncodeAdvance::Progress); }
                self.emit_owned((self.item_index as u32).to_be_bytes().to_vec(), maximum_bytes, Mp4EmitPhase::StssEntry)
            }
            Mp4EmitPhase::CttsHeader => {
                if !self.plan()?.ctts_present { self.emit = Mp4EmitPhase::StscHeader; return Ok(Mp4EncodeAdvance::Progress); }
                let mut bytes = mp4_box_header(b"ctts", mp4_box_size(8 + self.plan()?.ctts_runs * 8)?)?;
                bytes.extend_from_slice(&[self.plan()?.ctts_version, 0, 0, 0]); bytes.extend_from_slice(&(self.plan()?.ctts_runs as u32).to_be_bytes()); self.item_index = 0; self.run_count = 0;
                self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::CttsRunMeasure)
            }
            Mp4EmitPhase::CttsRunMeasure => self.measure_emit_i32_run(snapshot),
            Mp4EmitPhase::CttsRunEmit => {
                let mut bytes = self.run_count.to_be_bytes().to_vec(); bytes.extend_from_slice(&(self.run_i32 as u32).to_be_bytes()); self.run_count = 0;
                self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::CttsRunMeasure)
            }
            Mp4EmitPhase::StscHeader => {
                let mut bytes = mp4_box_header(b"stsc", mp4_box_size(8 + self.plan()?.stsc_entries * 12)?)?;
                bytes.extend_from_slice(&[0; 4]); bytes.extend_from_slice(&(self.plan()?.stsc_entries as u32).to_be_bytes()); self.item_index = 0; self.previous_chunk_count = None;
                self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::StscEntry)
            }
            Mp4EmitPhase::StscEntry => {
                if self.item_index >= self.plan()?.chunk_count { self.emit = Mp4EmitPhase::StszHeader; return Ok(Mp4EncodeAdvance::Progress); }
                let count = self.chunk_count(snapshot, self.item_index)?;
                let first_chunk = self.item_index as u32 + 1;
                self.item_index += 1;
                if self.previous_chunk_count == Some(count) { return Ok(Mp4EncodeAdvance::Progress); }
                self.previous_chunk_count = Some(count);
                let mut bytes = first_chunk.to_be_bytes().to_vec(); bytes.extend_from_slice(&count.to_be_bytes()); bytes.extend_from_slice(&1u32.to_be_bytes());
                self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::StscEntry)
            }
            Mp4EmitPhase::StszHeader => {
                let track = self.track(snapshot)?;
                let uniform = self.plan()?.uniform_sample_size.filter(|_| !track.samples.is_empty());
                let size = mp4_box_size(12 + if uniform.is_some() { 0 } else { track.samples.len() * 4 })?;
                let mut bytes = mp4_box_header(b"stsz", size)?; bytes.extend_from_slice(&[0; 4]); bytes.extend_from_slice(&uniform.unwrap_or(0).to_be_bytes()); bytes.extend_from_slice(&(track.samples.len() as u32).to_be_bytes()); self.item_index = 0;
                let next = if uniform.is_some() { Mp4EmitPhase::StcoHeader } else { Mp4EmitPhase::StszEntry };
                self.emit_owned(bytes, maximum_bytes, next)
            }
            Mp4EmitPhase::StszEntry => {
                let Some(sample) = self.track(snapshot)?.samples.get(self.item_index) else { self.emit = Mp4EmitPhase::StcoHeader; return Ok(Mp4EncodeAdvance::Progress); };
                self.item_index += 1;
                self.emit_owned((sample.data.len() as u32).to_be_bytes().to_vec(), maximum_bytes, Mp4EmitPhase::StszEntry)
            }
            Mp4EmitPhase::StcoHeader => {
                let mut bytes = mp4_box_header(b"stco", mp4_box_size(8 + self.plan()?.chunk_count * 4)?)?; bytes.extend_from_slice(&[0; 4]); bytes.extend_from_slice(&(self.plan()?.chunk_count as u32).to_be_bytes());
                self.item_index = 0; self.stco_sample_index = 0; self.stco_sample_remaining = 0; self.stco_offset = u64::from(self.mdat_data_offset) + self.plans[..self.track_index].iter().map(|plan| plan.sample_bytes).sum::<u64>();
                self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::StcoEntry)
            }
            Mp4EmitPhase::StcoEntry => {
                if self.item_index >= self.plan()?.chunk_count { self.emit = Mp4EmitPhase::TrackDone; return Ok(Mp4EncodeAdvance::Progress); }
                let offset = u32::try_from(self.stco_offset).map_err(|_| "mp4.encode.media-offset-overflow")?;
                self.stco_sample_remaining = self.chunk_count(snapshot, self.item_index)? as usize;
                self.item_index += 1;
                self.emit_owned(offset.to_be_bytes().to_vec(), maximum_bytes, Mp4EmitPhase::StcoAdvance)
            }
            Mp4EmitPhase::StcoAdvance => {
                if self.stco_sample_remaining == 0 { self.emit = Mp4EmitPhase::StcoEntry; return Ok(Mp4EncodeAdvance::Progress); }
                let sample = self.track(snapshot)?.samples.get(self.stco_sample_index).ok_or("mp4.encode.chunk-sample-missing")?;
                self.stco_offset = self.stco_offset.checked_add(sample.data.len() as u64).ok_or("mp4.encode.media-offset-overflow")?;
                self.stco_sample_index += 1; self.stco_sample_remaining -= 1;
                Ok(Mp4EncodeAdvance::Progress)
            }
            Mp4EmitPhase::TrackDone => { self.track_index += 1; self.item_index = 0; self.emit = Mp4EmitPhase::TrackHeader; Ok(Mp4EncodeAdvance::Progress) }
            Mp4EmitPhase::UdtaHeader => {
                let size = mp4_udta_size(&snapshot.movie)?;
                if size == 0 { self.emit = Mp4EmitPhase::Free; return Ok(Mp4EncodeAdvance::Progress); }
                self.emit_owned(mp4_box_header(b"udta", size)?, maximum_bytes, Mp4EmitPhase::MetaHeader)
            }
            Mp4EmitPhase::MetaHeader => {
                let udta = mp4_udta_size(&snapshot.movie)?;
                self.emit_owned(mp4_box_header(b"meta", udta - 8)?, maximum_bytes, Mp4EmitPhase::MetaPrefix)
            }
            Mp4EmitPhase::MetaPrefix => {
                let mut bytes = vec![0; 4];
                let mut handler = vec![0; 8]; handler.extend_from_slice(b"mdir"); handler.extend_from_slice(b"appl"); handler.extend_from_slice(&[0; 8]); handler.push(0);
                bytes.extend(write_box(b"hdlr", &handler));
                self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::IlstHeader)
            }
            Mp4EmitPhase::IlstHeader => {
                let items = snapshot.movie.title.as_deref().map(mp4_metadata_item_size).transpose()?.unwrap_or(0) + snapshot.movie.encoder.as_deref().map(mp4_metadata_item_size).transpose()?.unwrap_or(0);
                self.emit_owned(mp4_box_header(b"ilst", mp4_box_size(items)?)?, maximum_bytes, if snapshot.movie.title.is_some() { Mp4EmitPhase::TitleHeader } else { Mp4EmitPhase::EncoderHeader })
            }
            Mp4EmitPhase::TitleHeader => {
                let title = snapshot.movie.title.as_deref().ok_or("mp4.encode.title-missing")?;
                self.emit_owned(mp4_metadata_headers(&[0xa9, b'n', b'a', b'm'], title)?, maximum_bytes, Mp4EmitPhase::TitleData)
            }
            Mp4EmitPhase::TitleData => self.emit_borrowed(snapshot.movie.title.as_deref().ok_or("mp4.encode.title-missing")?.as_bytes(), maximum_bytes, Mp4EmitPhase::EncoderHeader),
            Mp4EmitPhase::EncoderHeader => {
                let Some(encoder) = snapshot.movie.encoder.as_deref() else { self.emit = Mp4EmitPhase::Free; return Ok(Mp4EncodeAdvance::Progress); };
                self.emit_owned(mp4_metadata_headers(&[0xa9, b't', b'o', b'o'], encoder)?, maximum_bytes, Mp4EmitPhase::EncoderData)
            }
            Mp4EmitPhase::EncoderData => self.emit_borrowed(snapshot.movie.encoder.as_deref().ok_or("mp4.encode.encoder-missing")?.as_bytes(), maximum_bytes, Mp4EmitPhase::Free),
            Mp4EmitPhase::Free => self.emit_owned(write_box(b"free", &[]), maximum_bytes, Mp4EmitPhase::MdatHeader),
            Mp4EmitPhase::MdatHeader => {
                self.track_index = 0; self.item_index = 0;
                self.emit_owned(mp4_box_header(b"mdat", mp4_box_size(usize::try_from(self.mdat_bytes).map_err(|_| "mp4.encode.mdat-size-overflow")?)?)?, maximum_bytes, Mp4EmitPhase::MdatSample)
            }
            Mp4EmitPhase::MdatSample => {
                while self.track_index < snapshot.tracks.len() && self.item_index >= snapshot.tracks[self.track_index].samples.len() { self.track_index += 1; self.item_index = 0; }
                let Some(track) = snapshot.tracks.get(self.track_index) else { self.emit = Mp4EmitPhase::Complete; return Ok(Mp4EncodeAdvance::Progress); };
                let sample = track.samples.get(self.item_index).ok_or("mp4.encode.sample-missing")?;
                if self.offset + maximum_bytes >= sample.data.len() { self.item_index += 1; }
                self.emit_borrowed(&sample.data, maximum_bytes, Mp4EmitPhase::MdatSample)
            }
            Mp4EmitPhase::Complete => Ok(Mp4EncodeAdvance::Complete),
        }
    }

    fn track<'a>(&self, snapshot: &'a Mp4Snapshot) -> Result<&'a Mp4Track, String> { snapshot.tracks.get(self.track_index).ok_or_else(|| "mp4.encode.track-missing".into()) }
    fn plan(&self) -> Result<&Mp4TrackPlan, String> { self.plans.get(self.track_index).ok_or_else(|| "mp4.encode.track-plan-missing".into()) }
    fn chunk_count(&self, snapshot: &Mp4Snapshot, index: usize) -> Result<u32, String> {
        let plan = self.plan()?;
        if plan.retained_chunks { self.track(snapshot)?.chunk_sample_counts.get(index).copied().ok_or_else(|| "mp4.encode.chunk-missing".into()) } else if index == 0 { Ok(self.track(snapshot)?.samples.len() as u32) } else { Err("mp4.encode.chunk-missing".into()) }
    }

    fn measure_emit_u32_run(&mut self, snapshot: &Mp4Snapshot, duration: bool) -> Result<Mp4EncodeAdvance, String> {
        let samples = &self.track(snapshot)?.samples;
        if self.item_index >= samples.len() { if self.run_count > 0 { self.emit = Mp4EmitPhase::SttsRunEmit; } else { self.emit = Mp4EmitPhase::StssHeader; } return Ok(Mp4EncodeAdvance::Progress); }
        let value = if duration { samples[self.item_index].duration } else { 0 };
        if self.run_count == 0 { self.run_u32 = value; self.run_count = 1; self.item_index += 1; } else if self.run_u32 == value { self.run_count = self.run_count.checked_add(1).ok_or("mp4.encode.run-overflow")?; self.item_index += 1; } else { self.emit = Mp4EmitPhase::SttsRunEmit; }
        Ok(Mp4EncodeAdvance::Progress)
    }

    fn measure_emit_i32_run(&mut self, snapshot: &Mp4Snapshot) -> Result<Mp4EncodeAdvance, String> {
        let samples = &self.track(snapshot)?.samples;
        if self.item_index >= samples.len() { if self.run_count > 0 { self.emit = Mp4EmitPhase::CttsRunEmit; } else { self.emit = Mp4EmitPhase::StscHeader; } return Ok(Mp4EncodeAdvance::Progress); }
        let value = samples[self.item_index].cts_offset;
        if self.run_count == 0 { self.run_i32 = value; self.run_count = 1; self.item_index += 1; } else if self.run_i32 == value { self.run_count = self.run_count.checked_add(1).ok_or("mp4.encode.run-overflow")?; self.item_index += 1; } else { self.emit = Mp4EmitPhase::CttsRunEmit; }
        Ok(Mp4EncodeAdvance::Progress)
    }

    fn emit_owned(&mut self, bytes: Vec<u8>, maximum_bytes: usize, next: Mp4EmitPhase) -> Result<Mp4EncodeAdvance, String> { self.emit_borrowed(&bytes, maximum_bytes, next) }

    fn emit_borrowed(&mut self, bytes: &[u8], maximum_bytes: usize, next: Mp4EmitPhase) -> Result<Mp4EncodeAdvance, String> {
        if bytes.is_empty() { self.offset = 0; self.emit = next; return Ok(Mp4EncodeAdvance::Progress); }
        let end = self.offset.checked_add(maximum_bytes).unwrap_or(usize::MAX).min(bytes.len());
        let chunk = bytes.get(self.offset..end).ok_or("mp4.encode.offset-invalid")?.to_vec();
        self.offset = end;
        self.emitted_bytes = self.emitted_bytes.checked_add(chunk.len() as u64).ok_or("mp4.encode.output-size-overflow")?;
        if self.offset == bytes.len() { self.offset = 0; self.emit = next; }
        Ok(Mp4EncodeAdvance::Chunk(chunk))
    }
}

fn mp4_hevc_fixed(codec: &Mp4Codec) -> Vec<u8> {
    let fallback = Mp4HevcConfig::default();
    let config = codec.hevc.as_ref().unwrap_or(&fallback);
    let mut bytes = vec![1, ((config.general_profile_space & 3) << 6) | (u8::from(config.general_tier_flag) << 5) | (config.general_profile_idc & 0x1f)];
    bytes.extend_from_slice(&config.general_profile_compatibility_flags.to_be_bytes());
    bytes.extend_from_slice(&config.general_constraint_indicator_flags.to_be_bytes()[2..]);
    bytes.push(config.general_level_idc);
    bytes.extend_from_slice(&(0xf000 | (config.min_spatial_segmentation_idc & 0x0fff)).to_be_bytes());
    bytes.push(0xfc | (config.parallelism_type & 3)); bytes.push(0xfc | (config.chroma_format_idc & 3)); bytes.push(0xf8 | (config.bit_depth_luma_minus8 & 7)); bytes.push(0xf8 | (config.bit_depth_chroma_minus8 & 7));
    bytes.extend_from_slice(&config.avg_frame_rate.to_be_bytes());
    bytes.push(((config.constant_frame_rate & 3) << 6) | ((config.num_temporal_layers & 7) << 3) | (u8::from(config.temporal_id_nested) << 2) | (codec.nal_length_size.saturating_sub(1) & 3));
    bytes.push(config.arrays.len() as u8);
    bytes
}

fn mp4_metadata_headers(kind: &[u8; 4], value: &str) -> Result<Vec<u8>, String> {
    let data = mp4_box_size(8usize.checked_add(value.len()).ok_or("mp4.encode.metadata-size-overflow")?)?;
    let mut bytes = mp4_box_header(kind, mp4_box_size(data)?)?;
    bytes.extend(mp4_box_header(b"data", data)?);
    bytes.extend_from_slice(&[0, 0, 0, 1, 0, 0, 0, 0]);
    Ok(bytes)
}

pub mod playback {
    use super::{Mp4EncodeAdvance, Mp4EncodeCursor, Mp4Snapshot, STDIO_MP4_DOCUMENT_SCHEMA};
    use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
    use semio_framework_plugin::{ArtifactApp, ArtifactOwnedToolJobFactory, ArtifactReservedToolJob, ArtifactToolPublicationContract, ArtifactToolPublicationLane, Fault};
    use semio_s_artifact_stdio_contract::media_export::{IncrementalMediaAdvance, IncrementalMediaExportJob, IncrementalMediaExportSpec, PLAYBACK_CONTRACT, PLAYBACK_TOOL_ID};
    use std::marker::PhantomData;

    pub const MIME_TYPE: &str = "video/mp4";
    pub const MEDIA_SCHEMA: &str = "stdio.mp4";
    pub const PAYLOAD_SCHEMA: &str = "stdio.mp4.playback-export.v1";
    pub const MEDIA_TYPE: semio_framework_plugin::MediaType = semio_s_artifact_stdio_contract::media_export::PLAYBACK_MEDIA_TYPE;

    pub struct Mp4PlaybackExport;
    impl IncrementalMediaExportSpec for Mp4PlaybackExport {
        type Snapshot = Mp4Snapshot;
        type Cursor = Mp4EncodeCursor;
        const DOCUMENT_SCHEMA: &'static str = STDIO_MP4_DOCUMENT_SCHEMA;
        const MEDIA_SCHEMA: &'static str = MEDIA_SCHEMA;
        const MIME_TYPE: &'static str = MIME_TYPE;
        const PAYLOAD_SCHEMA: &'static str = PAYLOAD_SCHEMA;
        const STAGE: &'static str = "encode-mp4";
        const KIND_ID: &'static str = "s.stdio.mp4";
        const ARTIFACT_ID: &'static str = "stdio.mp4";
        const ARTIFACT_NAME: &'static str = "MP4 Video";
        const COMPONENT_KIND: &'static str = "video";
        fn cursor(snapshot: &Mp4Snapshot) -> Result<Mp4EncodeCursor, Fault> { Ok(Mp4EncodeCursor::new(snapshot)) }
        fn advance(cursor: &mut Mp4EncodeCursor, snapshot: &Mp4Snapshot, maximum_bytes: usize) -> Result<IncrementalMediaAdvance, Fault> {
            cursor.advance(snapshot, maximum_bytes).map(|advance| match advance {
                Mp4EncodeAdvance::Progress => IncrementalMediaAdvance::Progress,
                Mp4EncodeAdvance::Chunk(bytes) => IncrementalMediaAdvance::Chunk(bytes),
                Mp4EncodeAdvance::Complete => IncrementalMediaAdvance::Complete,
            }).map_err(Fault::from)
        }
    }
    pub type Mp4PlaybackExportJob = IncrementalMediaExportJob<Mp4PlaybackExport>;

    pub struct Mp4MediaExportJobFactory<A: ArtifactApp<Snapshot = Mp4Snapshot>> { keys: [ToolFactoryKey; 1], owner: PhantomData<fn() -> A> }
    impl<A: ArtifactApp<Snapshot = Mp4Snapshot>> Mp4MediaExportJobFactory<A> { pub fn new(controller: &str) -> Self { Self { keys: [ToolFactoryKey::new(controller, PLAYBACK_TOOL_ID)], owner: PhantomData } } }
    impl<A: ArtifactApp<Snapshot = Mp4Snapshot>> ToolJobFactory for Mp4MediaExportJobFactory<A> {
        type Payload = ArtifactReservedToolJob;
        type Job = ArtifactReservedToolJob;
        fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
        fn payload_schema_id(&self) -> &str { PAYLOAD_SCHEMA }
        fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
        fn execution_contract(&self) -> ToolExecutionContract { PLAYBACK_CONTRACT }
        fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> { Ok(payload) }
    }
    impl<A: ArtifactApp<Snapshot = Mp4Snapshot>> ArtifactOwnedToolJobFactory for Mp4MediaExportJobFactory<A> {
        type Owner = A;
        const TOOL_IDS: &'static [&'static str] = &[PLAYBACK_TOOL_ID];
        const DOCUMENT_SCHEMA: &'static str = STDIO_MP4_DOCUMENT_SCHEMA;
        const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: PLAYBACK_TOOL_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] }];
    }
}
//#endregion 🔖️Encode

#[cfg(test)]
#[path = "🧪️tests/🔬️codec/🦀️.rs"]
mod codec_tests;
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::isobmff::subsets::any::io::Mp4Composer as Mp4RawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<Mp4RawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::standards::isobmff::subsets::any::schema::diff::Mp4Diff;
    use crate::standards::isobmff::subsets::any::schema::mutations::{apply_mp4_mutation,Mp4Mutation};

    use crate::standards::isobmff::subsets::any::schema::snapshot::Mp4Snapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct Mp4BuilderConstruction {
        snapshot: Mp4Snapshot,
    }

    impl ArtifactBuilder for Mp4BuilderConstruction {
        type Snapshot = Mp4Snapshot;
        type Mutation = Mp4Mutation;
        type Diff = Mp4Diff;
        fn empty() -> Self {
            Self { snapshot: Mp4Snapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<Mp4Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Mp4Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_mp4_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <Mp4Diff as protocol::MutationDiff<Mp4Snapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            Ok(self.snapshot)
        }
    }
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::standards::isobmff::subsets::any::io;
    use crate::standards::isobmff::subsets::any::schema::snapshot::{Mp4Snapshot, STDIO_MP4_DOCUMENT_SCHEMA};
    use {semio_framework_plugin::Analysis,semio_framework_plugin::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_plugin::IoConfidence,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct Mp4Parts {
        pub snapshot: Option<Mp4Snapshot>,
    }

    pub struct Mp4AnalyzerAnalysis;

    impl ArtifactAnalysis for Mp4AnalyzerAnalysis {
        type Parts = Mp4Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.mp4", standard: StandardId("isobmff"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if io::sniff_real_bytes(bytes) {
                        return IoConfidence::High;
                    }
                    let marker = STDIO_MP4_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if io::sniff_real_bytes(text.as_bytes()) || text.contains(STDIO_MP4_DOCUMENT_SCHEMA) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Mp4Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Mp4Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Mp4Snapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec Mp4BuilderFacets {
        construction: Mp4BuilderConstruction,
        analysis: Mp4AnalyzerAnalysis,
        composition: crate::standards::isobmff::subsets::any::io::derived_composition::Mp4ComposerComposition,
    }
    builder: Mp4Builder,
    analyzer: Mp4Analyzer,
    composer: Mp4Composer,
);
