//! 🧱️ Box builders of the MP4 codec: the sample entry, codec configuration, sample table, header and movie box builders. See [`super`].

use super::*;

//#region 🔖️Encode
/// 🐛 The 8 bytes of `reserved`+`data_reference_index` are followed by `pre_defined(2)` +
/// `reserved(2)` + `pre_defined[3](12)` = 16 bytes before `width`/`height` — matches
/// `parse_visual_sample_entry`'s read-side `skip(6+2+2+2+12)`.
pub(super) fn mp4_visual_sample_entry(codec_fourcc: &[u8; 4], width: u16, height: u16, visual: &Mp4VisualSampleEntry, extra: &[u8]) -> Vec<u8> {
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

pub(super) fn build_codec_extensions(track: &Mp4Track) -> Vec<u8> {
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
/// nicety: a natural-file open carries a whole caller-supplied document, and a caller that adds or drops
/// a sample cannot be expected to also restate a writer's chunking. Encoding a legal snapshot must
/// not depend on two fields being kept in step, so the encoder reconciles instead of aborting.
pub fn normalized_chunk_sample_counts(track: &Mp4Track) -> Vec<u32> {
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
pub(super) fn build_vmhd() -> Vec<u8> {
    write_box(b"vmhd", &[0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0])
}

pub(super) fn build_dinf() -> Vec<u8> {
    let url = write_box(b"url ", &[0, 0, 0, 1]);
    let mut dref_payload = vec![0u8; 4];
    dref_payload.extend_from_slice(&1u32.to_be_bytes());
    dref_payload.extend(url);
    write_box(b"dinf", &write_box(b"dref", &dref_payload))
}

pub(super) fn packed_language(language: &str) -> u16 {
    let mut chars = language.bytes().chain(std::iter::repeat(b'`')).take(3).map(|byte| u16::from(byte.saturating_sub(0x60)) & 0x1f);
    (chars.next().unwrap_or(0) << 10) | (chars.next().unwrap_or(0) << 5) | chars.next().unwrap_or(0)
}

pub(super) fn build_mdhd(track: &Mp4Track) -> Vec<u8> {
    let mut payload = vec![0u8; 4];
    payload.extend_from_slice(&(track.metadata.media_creation_time as u32).to_be_bytes());
    payload.extend_from_slice(&(track.metadata.media_modification_time as u32).to_be_bytes());
    payload.extend_from_slice(&track.timescale.to_be_bytes());
    payload.extend_from_slice(&(track.metadata.media_duration as u32).to_be_bytes());
    payload.extend_from_slice(&packed_language(&track.metadata.language).to_be_bytes());
    payload.extend_from_slice(&track.metadata.quality.to_be_bytes());
    write_box(b"mdhd", &payload)
}

pub(super) fn build_tkhd(track: &Mp4Track) -> Vec<u8> {
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

pub(super) fn build_mvhd(movie: &Mp4Movie) -> Vec<u8> {
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

pub(super) fn build_moov(snapshot: &Mp4Snapshot, mdat_data_offset: u32) -> Vec<u8> {
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
//#endregion 🔖️Encode
