//! 📏️ Encoded sizes of the AVI codec: the chunk, list and payload totals and the header builders the cursor steps use. See [`super`].

use super::*;

//#region 🔖️Encode
pub(super) fn chunk_total(payload: usize) -> Result<usize, String> {
    u32::try_from(payload).map_err(|_| "avi.encode.chunk-too-large")?;
    8usize.checked_add(payload).and_then(|value| value.checked_add(payload % 2)).ok_or_else(|| "avi.encode.chunk-size-overflow".into())
}

pub(super) fn list_total(children: usize) -> Result<usize, String> {
    let payload = 4usize.checked_add(children).ok_or("avi.encode.list-size-overflow")?;
    chunk_total(payload)
}

pub(super) fn raw_payload_len(item: &RiffChunk) -> Result<usize, String> {
    if item.fourcc.starts_with("LIST:") { 4usize.checked_add(item.data.len()).ok_or_else(|| "avi.encode.list-size-overflow".into()) } else { Ok(item.data.len()) }
}

pub(super) fn raw_total(item: &RiffChunk) -> Result<usize, String> {
    chunk_total(raw_payload_len(item)?)
}

pub(super) fn chunk_header(fourcc: &[u8; 4], payload: usize) -> Result<[u8; 8], String> {
    let mut bytes = [0u8; 8];
    bytes[..4].copy_from_slice(fourcc);
    bytes[4..].copy_from_slice(&u32::try_from(payload).map_err(|_| "avi.encode.chunk-too-large")?.to_le_bytes());
    Ok(bytes)
}

pub(super) fn list_header(list_type: &[u8; 4], children: usize) -> Result<[u8; 12], String> {
    let payload = 4usize.checked_add(children).ok_or("avi.encode.list-size-overflow")?;
    let chunk = chunk_header(b"LIST", payload)?;
    let mut bytes = [0u8; 12];
    bytes[..8].copy_from_slice(&chunk);
    bytes[8..].copy_from_slice(list_type);
    Ok(bytes)
}

pub(super) fn riff_root_header(body: usize) -> Result<[u8; 12], String> {
    let mut bytes = [0u8; 12];
    bytes[..4].copy_from_slice(b"RIFF");
    bytes[4..8].copy_from_slice(&u32::try_from(body).map_err(|_| "avi.encode.riff-too-large")?.to_le_bytes());
    bytes[8..].copy_from_slice(b"AVI ");
    Ok(bytes)
}

pub(super) fn raw_header(item: &RiffChunk) -> Result<Vec<u8>, String> {
    if let Some(list_type) = item.fourcc.strip_prefix("LIST:") {
        Ok(list_header(&fourcc4(list_type), item.data.len())?.to_vec())
    } else {
        Ok(chunk_header(&fourcc4(&item.fourcc), item.data.len())?.to_vec())
    }
}

pub(super) fn avih_fixed(header: &AviMainHeader) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(56);
    for value in [header.micro_sec_per_frame, header.max_bytes_per_sec, header.padding_granularity, header.flags, header.total_frames, header.initial_frames, header.streams, header.suggested_buffer_size, header.width, header.height] {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    for index in 0..4 {
        bytes.extend_from_slice(&header.reserved.get(index).copied().unwrap_or(0).to_le_bytes());
    }
    bytes
}

pub(super) fn stream_header_payload_len(stream: &AviStream) -> Result<usize, String> {
    let width = match stream.strh.rc_frame_width { 0 => 0, 8 => 8, 16 => 16, _ => return Err("avi.encode.rc-frame-width-invalid".into()) };
    48usize.checked_add(width).and_then(|value| value.checked_add(stream.strh.strh_extra.len())).ok_or_else(|| "avi.encode.strh-size-overflow".into())
}

pub(super) fn stream_header_fixed(header: &AviStreamHeader) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::with_capacity(64);
    bytes.extend_from_slice(&fourcc4(&header.fcc_type));
    bytes.extend_from_slice(&fourcc4(&header.fcc_handler));
    bytes.extend_from_slice(&header.flags.to_le_bytes());
    bytes.extend_from_slice(&header.priority.to_le_bytes());
    bytes.extend_from_slice(&header.language.to_le_bytes());
    for value in [header.initial_frames, header.scale, header.rate, header.start, header.length, header.suggested_buffer_size] {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    bytes.extend_from_slice(&header.quality.to_le_bytes());
    bytes.extend_from_slice(&header.sample_size.to_le_bytes());
    match header.rc_frame_width {
        16 => for value in [header.rc_frame_left, header.rc_frame_top, header.rc_frame_right, header.rc_frame_bottom] { bytes.extend_from_slice(&value.to_le_bytes()); },
        8 => for value in [header.rc_frame_left, header.rc_frame_top, header.rc_frame_right, header.rc_frame_bottom] { bytes.extend_from_slice(&(value as i16).to_le_bytes()); },
        0 => {}
        _ => return Err("avi.encode.rc-frame-width-invalid".into()),
    }
    Ok(bytes)
}

pub(super) fn stream_format_payload_len(format: &AviStreamFormat) -> usize {
    match format {
        AviStreamFormat::BitmapInfo { .. } => 40,
        AviStreamFormat::WaveFormat { extra, .. } => 16usize.saturating_add(extra.len()),
        AviStreamFormat::Raw { data } => data.len(),
    }
}

pub(super) fn stream_format_fixed(format: &AviStreamFormat) -> Vec<u8> {
    match format {
        AviStreamFormat::BitmapInfo { size, width, height, planes, bit_count, compression, size_image, x_pels_per_meter, y_pels_per_meter, colors_used, colors_important } => {
            let mut bytes = Vec::with_capacity(40);
            bytes.extend_from_slice(&size.to_le_bytes());
            bytes.extend_from_slice(&width.to_le_bytes());
            bytes.extend_from_slice(&height.to_le_bytes());
            bytes.extend_from_slice(&planes.to_le_bytes());
            bytes.extend_from_slice(&bit_count.to_le_bytes());
            bytes.extend_from_slice(&fourcc4(compression));
            bytes.extend_from_slice(&size_image.to_le_bytes());
            bytes.extend_from_slice(&x_pels_per_meter.to_le_bytes());
            bytes.extend_from_slice(&y_pels_per_meter.to_le_bytes());
            bytes.extend_from_slice(&colors_used.to_le_bytes());
            bytes.extend_from_slice(&colors_important.to_le_bytes());
            bytes
        }
        AviStreamFormat::WaveFormat { format_tag, channels, samples_per_sec, avg_bytes_per_sec, block_align, bits_per_sample, .. } => {
            let mut bytes = Vec::with_capacity(16);
            bytes.extend_from_slice(&format_tag.to_le_bytes());
            bytes.extend_from_slice(&channels.to_le_bytes());
            bytes.extend_from_slice(&samples_per_sec.to_le_bytes());
            bytes.extend_from_slice(&avg_bytes_per_sec.to_le_bytes());
            bytes.extend_from_slice(&block_align.to_le_bytes());
            bytes.extend_from_slice(&bits_per_sample.to_le_bytes());
            bytes
        }
        AviStreamFormat::Raw { .. } => Vec::new(),
    }
}

pub(super) fn stream_format_extra(format: &AviStreamFormat) -> &[u8] {
    match format {
        AviStreamFormat::BitmapInfo { .. } => &[],
        AviStreamFormat::WaveFormat { extra, .. } => extra,
        AviStreamFormat::Raw { data } => data,
    }
}
//#endregion 🔖️Encode
