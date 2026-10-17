//! 🎞️ Box parsing and decode of the MP4 codec: the sniff, the sample table parsing (stts, ctts, stsc, stsz, stco, co64, stss) with the run-length helpers, the track, media and movie header boxes, the codec configuration boxes and the movie decode. See [`super`].

use super::*;

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
    pub(super) fn len(&self) -> usize {
        match self {
            Self::Uniform { count, .. } => *count as usize,
            Self::PerSample(v) => v.len(),
        }
    }
    pub(super) fn get(&self, i: usize) -> Result<u32, String> {
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
pub(super) fn run_length_encode_u32(values: &[u32]) -> Vec<(u32, u32)> {
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

pub(super) fn run_length_encode_i64(values: &[i64]) -> Vec<(u32, i64)> {
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
pub(super) fn build_hvcc(config: &Mp4HevcConfig, nal_length_size: u8) -> Vec<u8> {
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
