//! 📤️ DWG byte writer of `stdio.dwg` (ac1024/✳️any): the R2004 container, LZ77-variant section compression,
//! section/page maps and object bitcode encoded from the typed snapshot and the structural drawing model.
//! Depends on the model only; its round-trip laws read back through the sibling `import` module.

use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::binary::snapshot::dwg_version_sentinel;
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::binary::snapshot::parse_version_header_fields;
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::binary::snapshot::DWG_HEADER_BEGIN;
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::binary::snapshot::DWG_HEADER_END;
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::binary::snapshot::DWG_PREVIEW_BEGIN;
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::binary::snapshot::DWG_PREVIEW_END;
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::binary::snapshot::DWG_SENTINEL_CLASSES_BEGIN;
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::binary::snapshot::DwgBitReader;
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::binary::snapshot::R2004_HEADER_LEN;
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::binary::snapshot::constraint_node_id;
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::binary::snapshot::decrypt_r2004_header;
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::binary::snapshot::dwg_crc16;
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::binary::snapshot::evaluation_graph_indexes;
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::geometry::DwgColor;
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::geometry::DwgDrawing;
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::geometry::DwgEntity;
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry;
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::is_preamble_only_document;
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::unwritable_version;
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgExportError;
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgSnapshot;
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DWG_PREAMBLE_LEN;
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::STDIO_DWG_DOCUMENT_SCHEMA;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::geometry::dwg_drawing_to_mesh;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::geometry::dwg_drawing_to_paths;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::geometry::dwg_drawing_to_svg;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::geometry::dwg_geometry_to_path_segments;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::geometry::mesh_to_dwg_drawing;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::geometry::paths_to_dwg_drawing;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::geometry::DwgLayer;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::geometry::DwgPathSegment;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_read::standards::v_ac1024::subsets::any::io::import::decode_r2004_document_sections;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_read::standards::v_ac1024::subsets::any::io::import::decode_r2004_handle_map;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_read::standards::v_ac1024::subsets::any::io::import::decode_r2004_sections;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_read::standards::v_ac1024::subsets::any::io::import::decode_r2010_classes_section;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_read::standards::v_ac1024::subsets::any::io::import::decompress_r2004_section;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_read::standards::v_ac1024::subsets::any::io::import::dwg_from_bytes;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_read::standards::v_ac1024::subsets::any::io::import::locate_r2004_sections;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_read::standards::v_ac1024::subsets::any::io::import::parse_page_directory;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_read::standards::v_ac1024::subsets::any::io::import::parse_r2004_file_header;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_read::standards::v_ac1024::subsets::any::io::import::r2004_section_data;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_read::standards::v_ac1024::subsets::any::io::import::DwgRawSection;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_read::standards::v_ac1024::subsets::any::io::import::DWG_TYPE_LAYER;

/// 📦 Serialized objects with handle and byte-offset entries.
pub type DwgObjectFrame = (Vec<u8>, Vec<(u64, usize)>);
//#region 🔖️Lz77Variant
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_r2004_lz_length(output: &mut Vec<u8>, mut length: usize) {
    while length > 0xff {
        length -= 0xff;
        output.push(0);
    }
    output.push(length as u8);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_r2004_lz_opcode(output: &mut Vec<u8>, opcode: u8, length: usize, immediate: usize) {
    if length <= immediate {
        output.push(opcode | (length - 2) as u8);
    } else {
        output.push(opcode);
        write_r2004_lz_length(output, length - immediate);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_r2004_lz_literals(output: &mut Vec<u8>, source: &[u8], start: usize, length: usize) {
    if length == 0 {
        return;
    }
    if length > 3 {
        write_r2004_lz_opcode(output, 0, length - 1, 0x11);
    }
    output.extend_from_slice(&source[start..start + length]);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_r2004_lz_match(output: &mut Vec<u8>, distance: usize, length: usize, following_literals: usize) {
    let (mut first, second) = if length >= 0x0f || distance > 0x400 {
        let (opcode, encoded_distance) = if distance <= 0x4000 { (0x20, distance - 1) } else { (0x10 | (((distance - 0x4000) >> 11) & 8) as u8, distance - 0x4000) };
        write_r2004_lz_opcode(output, opcode, length, if distance <= 0x4000 { 0x21 } else { 0x09 });
        (((encoded_distance & 0xff) << 2) as u8, (encoded_distance >> 6) as u8)
    } else {
        let encoded_distance = distance - 1;
        ((((length + 1) << 4) | ((encoded_distance & 3) << 2)) as u8, (encoded_distance >> 2) as u8)
    };
    if following_literals < 4 {
        first |= following_literals as u8;
    }
    output.extend_from_slice(&[first, second]);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn r2004_lz_hash4(source: &[u8], position: usize) -> usize {
    let mut value = (source[position + 3] as usize) << 6;
    value ^= source[position + 2] as usize;
    value = (value << 5) ^ source[position + 1] as usize;
    value = (value << 5) ^ source[position] as usize;
    (value + (value >> 5)) & 0x7fff
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn r2004_lz_candidate(source: &[u8], position: usize, end: usize, table: &mut [usize]) -> (usize, usize) {
    let mut index = r2004_lz_hash4(source, position);
    let mut previous = table[index];
    let mut distance = position.wrapping_sub(previous);
    if previous != usize::MAX && distance <= 0xbfff {
        if distance > 0x400 && source[position + 3] != source[previous + 3] {
            index = (index & 0x7ff) ^ 0x401f;
            previous = table[index];
            distance = position.wrapping_sub(previous);
            if previous == usize::MAX || distance > 0xbfff || (distance > 0x400 && source[position + 3] != source[previous + 3]) {
                table[index] = position;
                return (0, distance);
            }
        }
        if source[position..position + 3] == source[previous..previous + 3] {
            let mut length = 3;
            while position + length < end && source[previous + length] == source[position + length] {
                length += 1;
            }
            table[index] = position;
            return (length, distance);
        }
    }
    table[index] = position;
    (0, distance)
}
/// 🗜️ Encodes the deterministic AC18 D2 token stream used by R2004-family pages.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compress_r2004_section(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() < 4 {
        return Err("R2004 compression needs at least four initial literal bytes".into());
    }
    let mut encoded = Vec::with_capacity(data.len());
    let mut table = vec![usize::MAX; 0x8000];
    let mut current_literal = 0usize;
    let mut position = 4usize;
    let mut pending = None;
    while position < data.len().saturating_sub(0x13) {
        let (length, distance) = r2004_lz_candidate(data, position, data.len(), &mut table);
        if length < 3 {
            position += 1;
            continue;
        }
        let literals = position - current_literal;
        if let Some((pending_length, pending_distance)) = pending {
            write_r2004_lz_match(&mut encoded, pending_distance, pending_length, literals);
        }
        write_r2004_lz_literals(&mut encoded, data, current_literal, literals);
        position += length;
        current_literal = position;
        pending = Some((length, distance));
    }
    let literals = data.len() - current_literal;
    if let Some((pending_length, pending_distance)) = pending {
        write_r2004_lz_match(&mut encoded, pending_distance, pending_length, literals);
    }
    write_r2004_lz_literals(&mut encoded, data, current_literal, literals);
    encoded.extend_from_slice(&[0x11, 0, 0]);
    Ok(encoded)
}
/// 🧮 R2004 system/data-page checksum from §4.2 of the Open Design specification.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn r2004_page_checksum(seed: u32, data: &[u8]) -> u32 {
    let mut sum1 = seed & 0xffff;
    let mut sum2 = seed >> 16;
    for chunk in data.chunks(0x15b0) {
        for byte in chunk {
            sum1 += *byte as u32;
            sum2 += sum1;
        }
        sum1 %= 0xfff1;
        sum2 %= 0xfff1;
    }
    (sum2 << 16) | sum1
}
//#endregion 🔖️Lz77Variant
//#region 🔖️SectionMapAndInfo
//#region 🔖️R2004Writer
#[derive(Clone)]
struct EncodedR2004Page {
    section_id: u32,
    page_number: i32,
    start_offset: u64,
    payload: Vec<u8>,
    address: u64,
    allocation_size: u32,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn align_r2004(value: usize) -> usize {
    (value + 0x1f) & !0x1f
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn push_u32(output: &mut Vec<u8>, value: u32) {
    output.extend_from_slice(&value.to_le_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn push_u16(output: &mut Vec<u8>, value: u16) {
    output.extend_from_slice(&value.to_le_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn push_u64(output: &mut Vec<u8>, value: u64) {
    output.extend_from_slice(&value.to_le_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn r2004_crc32(data: &[u8]) -> u32 {
    let mut crc = 0xffff_ffffu32;
    for byte in data {
        crc ^= *byte as u32;
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xedb8_8320u32 & (0u32.wrapping_sub(crc & 1)));
        }
    }
    !crc
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encrypt_data_page_header(mut header: [u8; 32], address: u64) -> [u8; 32] {
    let mask = 0x4164536bu32 ^ address as u32;
    for chunk in header.as_chunks_mut::<4>().0 {
        let value = u32::from_le_bytes(*chunk) ^ mask;
        chunk.copy_from_slice(&value.to_le_bytes());
    }
    header
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn extend_r2004_lcg_fill(output: &mut Vec<u8>, end: usize, mut seed: u32) {
    while output.len() < end {
        seed = seed.wrapping_mul(0x343fd).wrapping_add(0x269ec3);
        output.push((seed >> 16) as u8);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_data_page(output: &mut Vec<u8>, page: &EncodedR2004Page) -> Result<(), String> {
    if output.len() as u64 != page.address {
        return Err(format!("page {} address {} != output {}", page.page_number, page.address, output.len()));
    }
    let data_checksum = r2004_page_checksum(0, &page.payload);
    let mut header = [0u8; 32];
    header[0..4].copy_from_slice(&0x4163043bu32.to_le_bytes());
    header[4..8].copy_from_slice(&page.section_id.to_le_bytes());
    header[8..12].copy_from_slice(&(page.payload.len() as u32).to_le_bytes());
    header[12..16].copy_from_slice(&page.allocation_size.to_le_bytes());
    header[16..20].copy_from_slice(&(page.start_offset as u32).to_le_bytes());
    header[28..32].copy_from_slice(&data_checksum.to_le_bytes());
    let header_checksum = r2004_page_checksum(data_checksum, &header);
    header[24..28].copy_from_slice(&header_checksum.to_le_bytes());
    output.extend_from_slice(&encrypt_data_page_header(header, page.address));
    output.extend_from_slice(&page.payload);
    extend_r2004_lcg_fill(output, page.address as usize + page.allocation_size as usize, 1);
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn materialize_r2004_ordinary_pages_without_header(snapshot: &DwgSnapshot) -> Result<Vec<EncodedR2004Page>, String> {
    let (objects, pairs) = materialize_r2010_objects(&snapshot.drawing.objects)?;
    let handles = materialize_r2004_handles(&pairs)?;
    let sections = [
        (9u32, encode_summary_info(&snapshot.summary)?, Some(128usize)),
        (10, encode_indexed_preview(&snapshot.preview, 0x1c0)?, Some(87_040)),
        (11, encode_application_info(&snapshot.application)?, Some(768)),
        (12, encode_application_history(&snapshot.application_history)?, Some(1_408)),
        (13, encode_dependencies(&snapshot.dependencies)?, Some(768)),
        (8, encode_revision_history(&snapshot.revision_history)?, None),
        (7, objects, None),
        (6, encode_object_free_space(&snapshot.auxiliary_header.updated_at), None),
        (5, encode_template(&snapshot.template)?, None),
        (4, handles, None),
        (3, encode_r2010_classes_section(&snapshot.classes)?, None),
        (2, encode_auxiliary_header(&snapshot.auxiliary_header)?, None),
    ];
    let mut pages = Vec::new();
    let mut address = 0x100usize;
    let mut page_number = 1i32;
    for (section_id, content, stored_capacity) in sections {
        if let Some(capacity) = stored_capacity {
            if content.len() > capacity {
                return Err(format!("stored section {section_id} exceeds capacity {capacity}"));
            }
            let mut payload = content;
            payload.resize(capacity, 0);
            let allocation_size = (32 + capacity) as u32;
            pages.push(EncodedR2004Page { section_id, page_number, start_offset: 0, payload, address: address as u64, allocation_size });
            address += allocation_size as usize;
            page_number += 1;
            continue;
        }
        for (page_index, chunk) in content.chunks(0x7400).enumerate() {
            let mut decoded = vec![0; 0x7400];
            decoded[..chunk.len()].copy_from_slice(chunk);
            let payload = compress_r2004_section(&decoded)?;
            let allocation_size = align_r2004(32 + payload.len()) as u32;
            pages.push(EncodedR2004Page { section_id, page_number, start_offset: (page_index * 0x7400) as u64, payload, address: address as u64, allocation_size });
            address += allocation_size as usize;
            page_number += 1;
        }
    }
    Ok(pages)
}
struct R2004SectionDescriptor {
    name: &'static str,
    section_id: u32,
    semantic_size: u64,
    max_decompressed_size: u32,
    compression: u32,
    encryption: u32,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2004_section_info(descriptors: &[R2004SectionDescriptor], pages: &[EncodedR2004Page], reserved_name: &[u8; 64], application_history_scratch: &[u8]) -> Result<Vec<u8>, String> {
    let mut output = Vec::new();
    push_u32(&mut output, descriptors.len() as u32);
    push_u32(&mut output, 2);
    push_u32(&mut output, 0x7400);
    push_u32(&mut output, 0);
    push_u32(&mut output, descriptors.len() as u32);
    for descriptor in descriptors {
        let section_pages: Vec<&EncodedR2004Page> = pages.iter().filter(|page| page.section_id == descriptor.section_id).collect();
        push_u64(&mut output, descriptor.semantic_size);
        push_u32(&mut output, section_pages.len() as u32);
        push_u32(&mut output, descriptor.max_decompressed_size);
        push_u32(&mut output, 1);
        push_u32(&mut output, descriptor.compression);
        push_u32(&mut output, descriptor.section_id);
        push_u32(&mut output, descriptor.encryption);
        let name_position = output.len();
        let mut name = if descriptor.section_id == 0 { *reserved_name } else { [0u8; 64] };
        if descriptor.section_id != 0 {
            let available = application_history_scratch.len().saturating_sub(name_position).min(name.len());
            if available != 0 {
                name[..available].copy_from_slice(&application_history_scratch[name_position..name_position + available]);
            }
        }
        if descriptor.name.len() >= name.len() {
            return Err(format!("R2004 section name is too long: {}", descriptor.name));
        }
        name[..descriptor.name.len()].copy_from_slice(descriptor.name.as_bytes());
        name[descriptor.name.len()] = 0;
        match descriptor.section_id {
            13 => {
                name[17..22].copy_from_slice(&[0, 0, 0x11, 0, 0]);
                let text: Vec<u8> = "his file is a Trusted".encode_utf16().flat_map(u16::to_le_bytes).collect();
                name[22..].copy_from_slice(&text);
            }
            9 => {
                name[17] = 0;
                let text: Vec<u8> = "datetime>2008-12-04T23:".encode_utf16().flat_map(u16::to_le_bytes).collect();
                name[18..].copy_from_slice(&text);
            }
            6 => {
                name[18..].copy_from_slice(&[0xf6, 0xe8, 0x3b, 0x85, 0x9d, 0x44, 0xa0, 0, 0x22, 0, 0x3c, 0, 0x50, 0, 0x0d, 0, 0, 0, 0xa9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0x74, 0, 0]);
            }
            5 => {
                name[14..].copy_from_slice(&[0x6f, 0, 0x6e, 0, 0x3d, 0, 0x5c, 0, 0x22, 0, 0x44, 0, 0x2e, 0, 0x34, 0, 0x30, 0, 0x0e, 0, 0, 0, 0x81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x2d, 0x08, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0x74, 0, 0]);
            }
            4 => {
                name[13] = 0;
                name[14..].copy_from_slice(&[0x74, 0, 0x72, 0, 0x69, 0, 0x6e, 0, 0x67, 0, 0x3d, 0, 0x5c, 0, 0x22, 0, 0x41, 0, 0x0f, 0, 0, 0, 0x81, 0x07, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x0f, 0x20, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0x74, 0, 0]);
            }
            3 => {
                name[13..28].fill(0);
                name[28..].copy_from_slice(&[0xa4, 0xa3, 0x09, 0xeb, 0x10, 0, 0, 0, 0x35, 0x12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x7b, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0x74, 0, 0]);
            }
            2 => {
                name[15] = 0;
                name[16..].copy_from_slice(&[0x41, 0x63, 0x44, 0x62, 0x3a, 0x48, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x73, 0, 0, 0, 0, 0x11, 0, 0, 0, 0xcb, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80, 0x03, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0x74, 0, 0]);
            }
            1 => {
                name[12..16].fill(0);
                name[16..].copy_from_slice(&[
                    0x41, 0x63, 0x44, 0x62, 0x3a, 0x43, 0x6c, 0x61, 0x73, 0x73, 0x65, 0x73, 0, 0, 0, 0, 0x12, 0, 0, 0, 0xae, 0x03, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x41, 0x63, 0x44, 0x62, 0x3a, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0, 0x24, 0xab, 0x34,
                    0xeb,
                ]);
            }
            _ => {}
        }
        output.extend_from_slice(&name);
        for page in section_pages {
            push_u32(&mut output, page.page_number as u32);
            push_u32(&mut output, page.payload.len() as u32);
            push_u64(&mut output, page.start_offset);
        }
    }
    Ok(output)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_r2004_system_page(output: &mut Vec<u8>, page_type: u32, decoded: &[u8], physical_allocation: Option<usize>, fill_skip: usize) -> Result<usize, String> {
    let start = output.len();
    let payload = compress_r2004_section(decoded)?;
    let mut header = Vec::with_capacity(20);
    push_u32(&mut header, page_type);
    push_u32(&mut header, decoded.len() as u32);
    push_u32(&mut header, payload.len() as u32);
    push_u32(&mut header, 2);
    push_u32(&mut header, 0);
    let checksum = r2004_page_checksum(r2004_page_checksum(0, &header), &payload);
    header[16..20].copy_from_slice(&checksum.to_le_bytes());
    output.extend_from_slice(&header);
    output.extend_from_slice(&payload);
    push_u32(output, page_type);
    push_u32(output, 0);
    push_u32(output, 0);
    push_u32(output, 2);
    push_u32(output, 0);
    if let Some(allocation) = physical_allocation {
        let target = start.checked_add(allocation).ok_or("R2004 system allocation overflow")?;
        if output.len() > target {
            return Err(format!("R2004 system page exceeds allocation: {} > {target}", output.len()));
        }
        let fill = target - output.len();
        let mut generated = Vec::new();
        extend_r2004_lcg_fill(&mut generated, fill + fill_skip, 1);
        output.extend_from_slice(&generated[fill_skip..]);
    }
    Ok(payload.len())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn r2004_section_descriptors(snapshot: &DwgSnapshot, header_size: usize) -> Result<Vec<R2004SectionDescriptor>, String> {
    let (objects, pairs) = materialize_r2010_objects(&snapshot.drawing.objects)?;
    let handles = materialize_r2004_handles(&pairs)?;
    let sizes = [
        (13, "AcDb:FileDepList", encode_dependencies(&snapshot.dependencies)?.len(), 768, 1, 2),
        (12, "AcDb:AppInfoHistory", encode_application_history(&snapshot.application_history)?.len(), 1_408, 1, 0),
        (11, "AcDb:AppInfo", encode_application_info(&snapshot.application)?.len(), 768, 1, 0),
        (10, "AcDb:Preview", encode_indexed_preview(&snapshot.preview, 0x1c0)?.len(), 87_040, 1, 0),
        (9, "AcDb:SummaryInfo", encode_summary_info(&snapshot.summary)?.len(), 128, 1, 0),
        (8, "AcDb:RevHistory", encode_revision_history(&snapshot.revision_history)?.len(), 0x7400, 2, 0),
        (7, "AcDb:AcDbObjects", objects.len(), 0x7400, 2, 0),
        (6, "AcDb:ObjFreeSpace", encode_object_free_space(&snapshot.auxiliary_header.updated_at).len(), 0x7400, 2, 0),
        (5, "AcDb:Template", encode_template(&snapshot.template)?.len(), 0x7400, 2, 0),
        (4, "AcDb:Handles", handles.len(), 0x7400, 2, 0),
        (3, "AcDb:Classes", encode_r2010_classes_section(&snapshot.classes)?.len(), 0x7400, 2, 0),
        (2, "AcDb:AuxHeader", encode_auxiliary_header(&snapshot.auxiliary_header)?.len(), 0x7400, 2, 0),
        (1, "AcDb:Header", header_size, 0x7400, 2, 0),
    ];
    let mut descriptors = vec![R2004SectionDescriptor { name: "", section_id: 0, semantic_size: 0, max_decompressed_size: 0x7400, compression: 2, encryption: 0 }];
    descriptors.extend(sizes.into_iter().map(|(section_id, name, semantic_size, max_decompressed_size, compression, encryption)| R2004SectionDescriptor {
        name,
        section_id,
        semantic_size: semantic_size as u64,
        max_decompressed_size,
        compression,
        encryption,
    }));
    Ok(descriptors)
}
/// 🏗️ Materializes a canonical R2004-family directory from logical AC1024 section descriptors.
/// Section payloads are serialization products and are never retained in the artifact schema.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2004_canonical(snapshot: &DwgSnapshot) -> Result<Vec<u8>, String> {
    if snapshot.version.len() != 6 {
        return Err("version sentinel must contain six bytes".into());
    }
    let header = encode_r2010_header_section(&snapshot.header)?;
    let mut pages = materialize_r2004_ordinary_pages_without_header(snapshot)?;
    let mut decoded_header = vec![0; 0x7400];
    decoded_header[..header.len()].copy_from_slice(&header);
    let header_payload = compress_r2004_section(&decoded_header)?;
    let header_address = pages.last().map_or(0x100, |page| page.address + u64::from(page.allocation_size));
    let header_page_number = i32::try_from(pages.len() + 1).map_err(|_| "R2004 page count exceeds i32")?;
    pages.push(EncodedR2004Page { section_id: 1, page_number: header_page_number, start_offset: 0, allocation_size: align_r2004(32 + header_payload.len()) as u32, address: header_address, payload: header_payload });
    let section_info_page_number = header_page_number as u32 + 3;
    let section_map_page_number = header_page_number as u32 + 4;
    let descriptors = r2004_section_descriptors(snapshot, header.len())?;
    let application_history_scratch = encode_application_history(&snapshot.application_history)?;
    let page_map_with = |section_info_allocation: u32, section_map_allocation: u32| {
        let mut page_map = Vec::new();
        for page in &pages {
            push_u32(&mut page_map, page.page_number as u32);
            push_u32(&mut page_map, page.allocation_size);
        }
        push_u32(&mut page_map, section_info_page_number);
        push_u32(&mut page_map, section_info_allocation);
        push_u32(&mut page_map, section_map_page_number);
        push_u32(&mut page_map, section_map_allocation);
        page_map
    };
    let compressed_map = compress_r2004_section(&page_map_with(1_024, 1_600))?;
    let mut reserved_name = [0u8; 64];
    let reserved_source = compressed_map.get(52..).unwrap_or_default();
    let reserved_len = reserved_source.len().min(64);
    reserved_name[..reserved_len].copy_from_slice(&reserved_source[..reserved_len]);
    reserved_name[0] = 0;
    let section_info = encode_r2004_section_info(&descriptors, &pages, &reserved_name, &application_history_scratch)?;
    let section_info_allocation = align_r2004(40 + compress_r2004_section(&section_info)?.len()).max(1_024);
    let page_map = page_map_with(section_info_allocation as u32, 1_600);
    let section_map_allocation = align_r2004(40 + compress_r2004_section(&page_map)?.len()).max(1_600);
    let page_map = page_map_with(section_info_allocation as u32, section_map_allocation as u32);

    let mut output = vec![0u8; 0x100];
    output[0..6].copy_from_slice(snapshot.version.as_bytes());
    output[0x0b] = 2;
    output[0x0c] = 3;
    let preview_address = pages.iter().find(|page| page.section_id == 10).ok_or("Preview page missing")?.address as u32 + 32;
    output[0x0d..0x11].copy_from_slice(&preview_address.to_le_bytes());
    output[0x11] = 0x1d;
    output[0x12] = snapshot.maintenance_version;
    output[0x13..0x15].copy_from_slice(&snapshot.codepage.to_le_bytes());
    output[0x16] = 0x1d;
    output[0x17] = snapshot.maintenance_version;
    let summary_address = pages.iter().find(|page| page.section_id == 9).ok_or("SummaryInfo page missing")?.address as u32 + 32;
    output[0x20..0x24].copy_from_slice(&summary_address.to_le_bytes());
    output[0x28..0x2c].copy_from_slice(&0x80u32.to_le_bytes());
    let application_address = pages.iter().find(|page| page.section_id == 11).ok_or("AppInfo page missing")?.address as u32 + 32;
    output[0x2c..0x30].copy_from_slice(&application_address.to_le_bytes());
    let application_history_address = pages.iter().find(|page| page.section_id == 12).ok_or("AppInfoHistory page missing")?.address as u32 + 32;
    output[0x30..0x34].copy_from_slice(&application_history_address.to_le_bytes());
    for page in &pages {
        write_data_page(&mut output, page)?;
    }
    write_r2004_system_page(&mut output, 0x4163003b, &section_info, Some(section_info_allocation), 1)?;
    let page_map_address = output.len();
    write_r2004_system_page(&mut output, 0x41630e3b, &page_map, None, 0)?;
    let second_header_address = output.len() as u64;

    let mut file_header = vec![0u8; R2004_HEADER_LEN];
    file_header[0..12].copy_from_slice(b"AcFssFcAJMB\0");
    file_header[0x10..0x14].copy_from_slice(&0x6cu32.to_le_bytes());
    file_header[0x14..0x18].copy_from_slice(&4u32.to_le_bytes());
    file_header[0x24..0x28].copy_from_slice(&1u32.to_le_bytes());
    file_header[0x28..0x2c].copy_from_slice(&section_map_page_number.to_le_bytes());
    let last_section_address = page_map_address + section_map_allocation - 0x100;
    file_header[0x2c..0x34].copy_from_slice(&(last_section_address as u64).to_le_bytes());
    file_header[0x34..0x3c].copy_from_slice(&second_header_address.to_le_bytes());
    file_header[0x40..0x44].copy_from_slice(&(header_page_number as u32 + 2).to_le_bytes());
    file_header[0x44..0x48].copy_from_slice(&0x20u32.to_le_bytes());
    file_header[0x48..0x4c].copy_from_slice(&0x80u32.to_le_bytes());
    file_header[0x4c..0x50].copy_from_slice(&0x40u32.to_le_bytes());
    file_header[0x50..0x54].copy_from_slice(&section_map_page_number.to_le_bytes());
    file_header[0x54..0x5c].copy_from_slice(&((page_map_address - 0x100) as u64).to_le_bytes());
    file_header[0x5c..0x60].copy_from_slice(&section_info_page_number.to_le_bytes());
    file_header[0x60..0x64].copy_from_slice(&section_map_page_number.to_le_bytes());
    let crc = r2004_crc32(&file_header);
    file_header[0x68..0x6c].copy_from_slice(&crc.to_le_bytes());
    let encrypted_header = decrypt_r2004_header(&file_header);
    output[0x80..0xec].copy_from_slice(&encrypted_header);
    let magic = decrypt_r2004_header(&vec![0; 0x100]);
    output[0xec..0x100].copy_from_slice(&magic[0xec..0x100]);
    output.extend_from_slice(&encrypted_header);
    Ok(output)
}
/// 🧱️ Deterministically materializes AC1024 from logical drawing and section state.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2004_snapshot(snapshot: &DwgSnapshot) -> Result<Vec<u8>, String> {
    encode_r2004_canonical(snapshot)
}
//#endregion 🔖️R2004Writer
//#endregion 🔖️SectionMapAndInfo
//#region 🔖️DwgStructuralCodec
//#region DwgBits
struct DwgBitWriter {
    bytes: Vec<u8>,
    bit: u8,
}
impl DwgBitWriter {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn new() -> Self {
        Self { bytes: Vec::new(), bit: 0 }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_bit(&mut self, value: bool) {
        if self.bit == 0 {
            self.bytes.push(0);
        }
        if value {
            let last = self.bytes.len() - 1;
            self.bytes[last] |= 1 << (7 - self.bit);
        }
        self.bit = (self.bit + 1) % 8;
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_bits(&mut self, value: u64, count: u8) {
        for i in (0..count).rev() {
            self.write_bit((value >> i) & 1 != 0);
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_b(&mut self, value: bool) {
        self.write_bit(value);
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_bb(&mut self, value: u8) {
        self.write_bits(value as u64, 2);
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_rc(&mut self, value: u8) {
        self.write_bits(value as u64, 8);
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_rs(&mut self, value: u16) {
        self.write_rc((value & 0xFF) as u8);
        self.write_rc((value >> 8) as u8);
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_rl(&mut self, value: u32) {
        self.write_rs((value & 0xFFFF) as u16);
        self.write_rs((value >> 16) as u16);
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_rll(&mut self, value: u64) {
        self.write_rl((value & 0xFFFF_FFFF) as u32);
        self.write_rl((value >> 32) as u32);
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_rd(&mut self, value: f64) {
        let bits = value.to_bits();
        self.write_rl((bits & 0xFFFF_FFFF) as u32);
        self.write_rl((bits >> 32) as u32);
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_bs(&mut self, value: u16) {
        match value {
            0 => self.write_bb(2),
            256 => self.write_bb(3),
            v if v <= 0xFF => {
                self.write_bb(1);
                self.write_rc(v as u8);
            }
            v => {
                self.write_bb(0);
                self.write_rs(v);
            }
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_bl(&mut self, value: u32) {
        match value {
            0 => self.write_bb(2),
            v if v <= 0xFF => {
                self.write_bb(1);
                self.write_rc(v as u8);
            }
            v => {
                self.write_bb(0);
                self.write_rl(v);
            }
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_bd(&mut self, value: f64) {
        if value == 0.0 {
            self.write_bb(2);
        } else if value == 1.0 {
            self.write_bb(1);
        } else {
            self.write_bb(0);
            self.write_rd(value);
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_dd(&mut self, value: f64, default: f64) {
        let value_bytes = value.to_le_bytes();
        let default_bytes = default.to_le_bytes();
        if value_bytes == default_bytes {
            self.write_bb(0);
        } else if value_bytes[4..] == default_bytes[4..] {
            self.write_bb(1);
            for byte in &value_bytes[..4] {
                self.write_rc(*byte);
            }
        } else if value_bytes[6..] == default_bytes[6..] {
            self.write_bb(2);
            self.write_rc(value_bytes[4]);
            self.write_rc(value_bytes[5]);
            for byte in &value_bytes[..4] {
                self.write_rc(*byte);
            }
        } else {
            self.write_bb(3);
            self.write_rd(value);
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_bt(&mut self, value: f64) {
        if value == 0.0 {
            self.write_b(true);
        } else {
            self.write_b(false);
            self.write_bd(value);
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_2rd(&mut self, v: [f64; 2]) {
        self.write_rd(v[0]);
        self.write_rd(v[1]);
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_3bd(&mut self, v: [f64; 3]) {
        self.write_bd(v[0]);
        self.write_bd(v[1]);
        self.write_bd(v[2]);
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_3rd(&mut self, v: [f64; 3]) {
        self.write_rd(v[0]);
        self.write_rd(v[1]);
        self.write_rd(v[2]);
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_be(&mut self, normal: [f64; 3]) {
        if normal == [0.0, 0.0, 1.0] {
            self.write_b(true);
        } else {
            self.write_b(false);
            self.write_3bd(normal);
        }
    }


    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_tu(&mut self, text: &str) {
        let units: Vec<u16> = text.encode_utf16().collect();
        self.write_bs(units.len() as u16);
        for unit in units {
            self.write_rs(unit);
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn append_bits(&mut self, other: &DwgBitWriter) {
        let mut reader = DwgBitReader::new(&other.bytes);
        for _ in 0..other.bit_len() {
            self.write_bit(reader.read_bit().expect("writer-owned bitstream is complete"));
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_ms(&mut self, mut value: u32) {
        loop {
            let mut chunk = (value & 0x7FFF) as u16;
            value >>= 15;
            if value != 0 {
                chunk |= 0x8000;
                self.write_rs(chunk);
            } else {
                self.write_rs(chunk);
                break;
            }
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_umc(&mut self, mut value: u64) {
        loop {
            let mut byte = (value & 0x7f) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            self.write_rc(byte);
            if value == 0 {
                break;
            }
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_bot(&mut self, value: u16) {
        if value <= 0xff {
            self.write_bb(0);
            self.write_rc(value as u8);
        } else if (0x1f0..=0x2ef).contains(&value) {
            self.write_bb(1);
            self.write_rc((value - 0x1f0) as u8);
        } else {
            self.write_bb(2);
            self.write_rs(value);
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_handle(&mut self, code: u8, handle: u64) {
        let mut bytes = Vec::new();
        let mut v = handle;
        while v != 0 {
            bytes.insert(0, (v & 0xFF) as u8);
            v >>= 8;
        }
        self.write_rc((code << 4) | bytes.len() as u8);
        for b in bytes {
            self.write_rc(b);
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn pad_to_byte(&mut self) {
        while self.bit != 0 {
            self.write_bit(false);
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn bit_len(&self) -> usize {
        self.bytes.len() * 8 - if self.bit == 0 { 0 } else { 8 - self.bit as usize }
    }
}
//#endregion DwgBits
//#region DwgWrite
const DWG_SENTINEL_CLASSES_END: [u8; 16] = [0x72, 0x5E, 0x3B, 0x47, 0x3B, 0x56, 0x07, 0x3A, 0x3F, 0x23, 0x0B, 0xA0, 0x18, 0x30, 0x49, 0x75];
/// 📐️ Serializes a drawing as a new AutoCAD 2010 (AC1024) document: the drawing becomes the model
/// space of [`DwgSnapshot::from_drawing`], written by the same R2004-family container and R2010
/// object writer as every other AC1024 snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dwg_to_bytes(drawing: &DwgDrawing) -> Result<Vec<u8>, String> {
    encode_r2004_snapshot(&DwgSnapshot::from_drawing(drawing)?)
}
//#endregion DwgWrite
//#region DwgRead
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn push_utf16_z(output: &mut Vec<u8>, value: &str) -> Result<(), String> {
    let units = value.encode_utf16().collect::<Vec<_>>();
    let count = units.len().checked_add(1).ok_or("DWG UTF-16 string length overflow")?;
    push_u16(output, u16::try_from(count).map_err(|_| "DWG UTF-16 string exceeds u16 length")?);
    for unit in units {
        push_u16(output, unit);
    }
    push_u16(output, 0);
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn push_utf16_bytes(output: &mut Vec<u8>, value: &str) -> Result<(), String> {
    let units = value.encode_utf16().collect::<Vec<_>>();
    push_u32(output, u32::try_from(units.len().checked_mul(2).ok_or("DWG UTF-16 byte length overflow")?).map_err(|_| "DWG UTF-16 byte string exceeds u32 length")?);
    for unit in units {
        push_u16(output, unit);
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn push_bytes_z(output: &mut Vec<u8>, value: &str) -> Result<(), String> {
    let count = value.len().checked_add(1).ok_or("DWG byte string length overflow")?;
    push_u16(output, u16::try_from(count).map_err(|_| "DWG byte string exceeds u16 length")?);
    output.extend_from_slice(value.as_bytes());
    output.push(0);
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn checksum_bytes(value: &str) -> Result<[u8; 16], String> {
    if value.is_empty() {
        return Ok([0; 16]);
    }
    let compact = value.chars().filter(|character| *character != '-').collect::<String>();
    if compact.len() != 32 {
        return Err("DWG checksum identifier must contain 32 hexadecimal digits".into());
    }
    let mut bytes = [0; 16];
    for (index, byte) in bytes.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&compact[index * 2..index * 2 + 2], 16).map_err(|error| format!("invalid DWG checksum identifier: {error}"))?;
    }
    Ok(bytes)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_summary_info(summary: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgSummaryInfo) -> Result<Vec<u8>, String> {
    let mut output = Vec::new();
    for value in [&summary.title, &summary.subject, &summary.author, &summary.keywords, &summary.comments, &summary.last_saved_by, &summary.revision_number, &summary.hyperlink_base] {
        push_utf16_z(&mut output, value)?;
    }
    push_u64(&mut output, summary.total_editing_time);
    push_u32(&mut output, summary.created_at.days);
    push_u32(&mut output, summary.created_at.milliseconds);
    push_u32(&mut output, summary.modified_at.days);
    push_u32(&mut output, summary.modified_at.milliseconds);
    push_u16(&mut output, u16::try_from(summary.custom_properties.len()).map_err(|_| "DWG summary has too many custom properties")?);
    for property in &summary.custom_properties {
        push_utf16_z(&mut output, &property.key)?;
        push_utf16_z(&mut output, &property.value)?;
    }
    push_u32(&mut output, 0);
    push_u32(&mut output, 0);
    Ok(output)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_application_info(application: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgApplicationInfo) -> Result<Vec<u8>, String> {
    let mut output = Vec::new();
    push_u32(&mut output, 3);
    push_utf16_z(&mut output, &application.name)?;
    push_u32(&mut output, 3);
    output.extend_from_slice(&checksum_bytes(&application.version_checksum)?);
    push_utf16_z(&mut output, &application.version)?;
    output.extend_from_slice(&checksum_bytes(&application.comment_checksum)?);
    push_utf16_z(&mut output, &application.comment)?;
    output.extend_from_slice(&checksum_bytes(&application.product_checksum)?);
    push_utf16_z(&mut output, &application.product)?;
    if !application.application_version.is_empty() {
        push_bytes_z(&mut output, &application.application_version)?;
    }
    Ok(output)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_dependencies(dependencies: &[semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDependency]) -> Result<Vec<u8>, String> {
    let mut features = Vec::<String>::new();
    for dependency in dependencies {
        if !features.contains(&dependency.feature) {
            features.push(dependency.feature.clone());
        }
    }
    let mut output = Vec::new();
    push_u32(&mut output, u32::try_from(features.len()).map_err(|_| "DWG dependency feature count exceeds u32")?);
    for feature in &features {
        push_utf16_bytes(&mut output, feature)?;
    }
    push_u32(&mut output, u32::try_from(dependencies.len()).map_err(|_| "DWG dependency count exceeds u32")?);
    for dependency in dependencies {
        push_utf16_bytes(&mut output, &dependency.full_path)?;
        push_utf16_bytes(&mut output, &dependency.relative_path)?;
        push_utf16_bytes(&mut output, &dependency.fingerprint)?;
        push_utf16_bytes(&mut output, &dependency.version)?;
        push_u32(&mut output, features.iter().position(|feature| feature == &dependency.feature).ok_or("DWG dependency feature is missing")? as u32);
        push_u32(&mut output, dependency.timestamp);
        push_u32(&mut output, dependency.file_size);
        push_u16(&mut output, u16::from(dependency.affects_graphics));
        push_u32(&mut output, dependency.reference_count);
    }
    Ok(output)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_template(template: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgTemplate) -> Result<Vec<u8>, String> {
    let mut output = Vec::new();
    push_utf16_z(&mut output, &template.description)?;
    push_u16(
        &mut output,
        match template.measurement {
            semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgMeasurement::English => 0,
            semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgMeasurement::Metric => 1,
        },
    );
    Ok(output)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_auxiliary_header(value: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAuxiliaryHeader) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgCompatibilityProfile;
    if value.compatibility_profile != DwgCompatibilityProfile::Autocad2009 {
        return Err("unsupported auxiliary-header compatibility profile".into());
    }
    let mut output = vec![255, 119, 1];
    push_u16(&mut output, 29);
    push_u16(&mut output, 2);
    push_u32(&mut output, value.total_saves);
    output.extend_from_slice(&(-1i32).to_le_bytes());
    push_u16(&mut output, value.save_partition_one);
    push_u16(&mut output, value.save_partition_two);
    push_u32(&mut output, value.save_generation);
    for stamp in [&value.legacy_stamp_one, &value.legacy_stamp_two] {
        push_u16(&mut output, stamp.version);
        push_u16(&mut output, stamp.maintenance);
    }
    for value in [4u16, 1381, 261, 2600, 0, 1] {
        push_u16(&mut output, value);
    }
    for value in [0u32, 0, 0, 16_908_544, 65_538] {
        push_u32(&mut output, value);
    }
    for date in [&value.created_at, &value.updated_at] {
        push_u32(&mut output, date.days);
        push_u32(&mut output, date.milliseconds);
    }
    push_u64(&mut output, value.handle_seed);
    push_u16(&mut output, 0);
    push_u16(&mut output, value.terminal_save_generation);
    for terminal in [0u32, 0, 0, value.total_saves, 0, 0, 0, 0] {
        push_u32(&mut output, terminal);
    }
    if output.len() != 123 {
        return Err(format!("auxiliary header encoded to {} bytes", output.len()));
    }
    Ok(output)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_revision_history(value: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgRevisionHistory) -> Result<Vec<u8>, String> {
    let mut output = Vec::new();
    push_u32(&mut output, value.format_major);
    push_u32(&mut output, value.format_minor);
    push_u32(&mut output, u32::try_from(value.revisions.len()).map_err(|_| "revision history exceeds u32 count")?);
    for revision in &value.revisions {
        push_u32(&mut output, *revision);
    }
    Ok(output)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_indexed_preview(value: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgIndexedPreview, payload_address: u32) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgPreviewOrigin;
    if value.origin != DwgPreviewOrigin::BottomUp || value.palette.len() != 256 {
        return Err("AC1024 preview requires a bottom-up 256-color indexed bitmap".into());
    }
    let width = value.width as usize;
    let height = value.height as usize;
    if value.pixel_indices.len() != width.checked_mul(height).ok_or("preview dimensions overflow")? {
        return Err("preview pixel count disagrees with dimensions".into());
    }
    let stride = (width + 3) & !3;
    let image_size = stride.checked_mul(height).ok_or("preview image size overflow")?;
    let bitmap_size = 40 + 1024 + image_size;
    let overall_size = 1 + 18 + 80 + bitmap_size;
    let mut output = Vec::with_capacity(overall_size + 36);
    output.extend_from_slice(&DWG_PREVIEW_BEGIN);
    push_u32(&mut output, overall_size as u32);
    output.push(2);
    output.push(1);
    push_u32(&mut output, payload_address + 39);
    push_u32(&mut output, 80);
    output.push(2);
    push_u32(&mut output, payload_address + 119);
    push_u32(&mut output, bitmap_size as u32);
    output.extend_from_slice(&[0; 80]);
    push_u32(&mut output, 40);
    output.extend_from_slice(&(value.width as i32).to_le_bytes());
    output.extend_from_slice(&(value.height as i32).to_le_bytes());
    push_u16(&mut output, 1);
    push_u16(&mut output, 8);
    push_u32(&mut output, 0);
    push_u32(&mut output, image_size as u32);
    output.extend_from_slice(&0i32.to_le_bytes());
    output.extend_from_slice(&0i32.to_le_bytes());
    push_u32(&mut output, 256);
    push_u32(&mut output, 0);
    for color in &value.palette {
        output.extend_from_slice(&[color.blue, color.green, color.red, 0]);
    }
    for row in value.pixel_indices.chunks_exact(width.max(1)) {
        output.extend_from_slice(row);
        output.extend(std::iter::repeat_n(value.background_palette_index, stride - width));
    }
    output.extend_from_slice(&DWG_PREVIEW_END);
    Ok(output)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_digest128(value: &str) -> Result<[u8; 16], String> {
    if value.len() != 32 {
        return Err("128-bit identifier must contain 32 hex digits".into());
    }
    let mut output = [0u8; 16];
    for (index, pair) in value.as_bytes().as_chunks::<2>().0.iter().enumerate() {
        output[index] = u8::from_str_radix(std::str::from_utf8(pair).map_err(|_| "identifier is not UTF-8")?, 16).map_err(|_| "identifier contains non-hex digits")?;
    }
    Ok(output)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn render_application_properties(format_identifier: &str, properties: &[semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgApplicationProperty]) -> String {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgApplicationPropertyKind;
    let mut output = format!("<prop_set fmt_id=\"{{{format_identifier}}}\">");
    for property in properties {
        let tag = match property.kind {
            DwgApplicationPropertyKind::String => "string",
            DwgApplicationPropertyKind::DateTime => "datetime",
        };
        output.push_str(&format!("<prop id=\"{}\"><{tag}>{}</{tag}></prop>", property.id, property.value));
    }
    output.push_str("</prop_set>");
    output
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn render_product_information(value: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgProductInformation) -> String {
    format!(
        "\"<ProductInformation name =\\\"{}\\\" build_version=\\\"{}\\\" registry_version=\\\"{}\\\" install_id_string=\\\"{}\\\" registry_localeID=\\\"{}\\\"/>\"",
        value.name, value.build_version, value.registry_version, value.install_id, value.locale_id
    )
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_application_history(value: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgApplicationHistory) -> Result<Vec<u8>, String> {
    let mut output = Vec::new();
    output.extend_from_slice(&encode_digest128(&value.history_identifier_one)?);
    output.extend_from_slice(&encode_digest128(&value.history_identifier_two)?);
    push_u32(&mut output, value.class_version);
    push_utf16_z(&mut output, "AppInfoDataList")?;
    push_u32(&mut output, 4);
    for (digest, rendered) in [
        (&value.application_version_digest, value.application_version.clone()),
        (&value.trust_comment_digest, value.trust_comment.clone()),
        (&value.property_set_digest, render_application_properties(&value.property_format_identifier, &value.properties)),
        (&value.product_digest, render_product_information(&value.product)),
    ] {
        output.extend_from_slice(&encode_digest128(digest)?);
        push_utf16_z(&mut output, &rendered)?;
    }
    Ok(output)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_object_free_space(updated: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgJulianDate) -> Vec<u8> {
    let mut output = Vec::with_capacity(89);
    push_u64(&mut output, 0);
    push_u64(&mut output, 679);
    push_u32(&mut output, updated.days);
    push_u32(&mut output, updated.milliseconds);
    output.push(4);
    for (low, high) in [(50u64, 0u64), (100, 0), (512, 0), (0xffff_ffff, 0)] {
        push_u64(&mut output, low);
        push_u64(&mut output, high);
    }
    output
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn header_point3(value: &[f64], name: &str) -> Result<[f64; 3], String> {
    value.try_into().map_err(|_| format!("{name} must contain three coordinates"))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn header_point2(value: &[f64], name: &str) -> Result<[f64; 2], String> {
    value.try_into().map_err(|_| format!("{name} must contain two coordinates"))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_header_time(writer: &mut DwgBitWriter, value: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgJulianDate) {
    writer.write_bl(value.days);
    writer.write_bl(value.milliseconds);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_header_color(writer: &mut DwgBitWriter, index: u16, rgb: u32) {
    writer.write_bs(index);
    writer.write_bl(rgb);
    writer.write_rc(0);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_header_space(writer: &mut DwgBitWriter, value: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgHeaderSpaceGeometry, name: &str) -> Result<(), String> {
    writer.write_3bd(header_point3(&value.insertion_base, &format!("{name} insertion base"))?);
    writer.write_3bd(header_point3(&value.extents_minimum, &format!("{name} extents minimum"))?);
    writer.write_3bd(header_point3(&value.extents_maximum, &format!("{name} extents maximum"))?);
    writer.write_2rd(header_point2(&value.limits_minimum, &format!("{name} limits minimum"))?);
    writer.write_2rd(header_point2(&value.limits_maximum, &format!("{name} limits maximum"))?);
    writer.write_bd(value.elevation);
    writer.write_3bd(header_point3(&value.ucs_origin, &format!("{name} UCS origin"))?);
    writer.write_3bd(header_point3(&value.ucs_x_axis, &format!("{name} UCS X axis"))?);
    writer.write_3bd(header_point3(&value.ucs_y_axis, &format!("{name} UCS Y axis"))?);
    writer.write_bs(value.ucs_orthographic_view);
    for (point, suffix) in [(&value.ucs_origin_top, "top"), (&value.ucs_origin_bottom, "bottom"), (&value.ucs_origin_left, "left"), (&value.ucs_origin_right, "right"), (&value.ucs_origin_front, "front"), (&value.ucs_origin_back, "back")] {
        writer.write_3bd(header_point3(point, &format!("{name} UCS {suffix}"))?);
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_header_section(value: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgHeaderVariables) -> Result<Vec<u8>, String> {
    let mut main = DwgBitWriter::new();
    let u = &value.units;
    for number in [u.unit1_conversion, u.unit2_conversion, u.unit3_conversion, u.unit4_conversion] {
        main.write_bd(number);
    }
    main.write_bl(2_454_805);
    main.write_bl(60_784_745);
    let m = &value.modes;
    for flag in [m.dimension_associative, m.dimension_show, m.polyline_generation, m.orthographic_mode, m.regeneration_mode, m.fill_mode, m.quick_text_mode, m.paper_space_linetype_scale, m.limits_check] {
        main.write_b(flag);
    }
    main.write_b(false);
    for flag in [m.user_timer, m.sketch_polyline, m.angle_direction, m.spline_frame, m.mirror_text, m.world_view, m.tile_mode, m.paper_limits_check, m.visual_retain, m.display_silhouette, m.polyline_ellipse] {
        main.write_b(flag);
    }
    let i = &value.integers;
    main.write_bs(i.proxy_graphics);
    main.write_bs(i.tree_depth as u16);
    main.write_bs(i.linear_units);
    main.write_bs(i.linear_precision);
    main.write_bs(i.angular_units);
    main.write_bs(i.angular_precision);
    main.write_bs(i.attribute_mode);
    main.write_bs(i.point_display_mode);
    main.write_bl(0x30303030);
    main.write_bl(0x1d050900);
    main.write_bl(0x4d353930);
    for number in [i.user_integer1, i.user_integer2, i.user_integer3, i.user_integer4, i.user_integer5] {
        main.write_bs(number as u16);
    }
    for number in
        [i.spline_segments, i.surface_u, i.surface_v, i.surface_type, i.surface_tab1, i.surface_tab2, i.spline_type, i.shade_edge, i.shade_difference, i.unit_mode, i.maximum_active_viewports, i.isolines, i.multiline_justification, i.text_quality]
    {
        main.write_bs(number);
    }
    let s = &value.scalars;
    for number in [
        s.linetype_scale,
        s.text_size,
        s.trace_width,
        s.sketch_increment,
        s.fillet_radius,
        s.thickness,
        s.angle_base,
        s.point_display_size,
        s.polyline_width,
        s.user_real1,
        s.user_real2,
        s.user_real3,
        s.user_real4,
        s.user_real5,
        s.chamfer_a,
        s.chamfer_b,
        s.chamfer_c,
        s.chamfer_d,
        s.facet_resolution,
        s.multiline_scale,
        s.current_entity_linetype_scale,
    ] {
        main.write_bd(number);
    }
    write_header_time(&mut main, &value.time.created_at);
    write_header_time(&mut main, &value.time.updated_at);
    main.write_bl(0x44443434);
    main.write_bl(0x140d0102);
    main.write_bl(0x39343531);
    write_header_time(&mut main, &value.time.editing_duration);
    write_header_time(&mut main, &value.time.user_timer_duration);
    write_header_color(&mut main, s.current_entity_color_index, 0xc0000000);
    main.write_handle(0, value.relations.handle_seed);
    main.write_bd(s.paper_space_viewport_scale);
    write_header_space(&mut main, &value.paper_space, "paper space")?;
    write_header_space(&mut main, &value.model_space, "model space")?;
    let d = &value.dimensions;
    for number in [d.scale, d.arrow_size, d.extension_offset, d.line_increment, d.extension, d.rounding, d.line_extension, d.tolerance_plus, d.tolerance_minus, d.fixed_extension_length, d.jog_angle] {
        main.write_bd(number);
    }
    main.write_bs(d.text_fill);
    write_header_color(&mut main, d.text_fill_color_index, 0xc1000000);
    for flag in [d.tolerance, d.limits, d.text_inside_horizontal, d.text_outside_horizontal, d.suppress_extension1, d.suppress_extension2] {
        main.write_b(flag);
    }
    for number in [d.text_above, d.zero_suppression, d.angular_zero_suppression, d.arc_symbol] {
        main.write_bs(number);
    }
    for number in [d.text_height, d.center_mark, d.tick_size, d.alternate_scale, d.linear_factor, d.text_vertical_position, d.text_factor, d.gap, d.alternate_rounding] {
        main.write_bd(number);
    }
    main.write_b(d.alternate_units);
    main.write_bs(d.alternate_decimal_places);
    for flag in [d.text_outside_force_line, d.separate_arrows, d.text_inside, d.suppress_outside] {
        main.write_b(flag);
    }
    for color in [d.line_color_index, d.extension_color_index, d.text_color_index] {
        write_header_color(&mut main, color, 0xc1000000);
    }
    for number in [
        d.angular_decimal_places,
        d.decimal_places,
        d.tolerance_decimal_places,
        d.alternate_units_format,
        d.alternate_tolerance_decimal_places,
        d.angular_unit_format,
        d.fractional_format,
        d.linear_unit_format,
        d.decimal_separator,
        d.text_movement,
        d.justification,
    ] {
        main.write_bs(number);
    }
    for flag in [d.suppress_dimension1, d.suppress_dimension2] {
        main.write_b(flag);
    }
    for number in [d.tolerance_justification, d.tolerance_zero_suppression, d.alternate_zero_suppression, d.alternate_tolerance_zero_suppression] {
        main.write_bs(number);
    }
    main.write_b(d.user_positioned_text);
    main.write_bs(d.fit);
    main.write_b(d.fixed_extension_enabled);
    main.write_b(d.text_direction);
    main.write_bd(d.alternate_measurement_scale);
    main.write_bd(d.measurement_scale);
    main.write_bs(d.dimension_line_weight as u16);
    main.write_bs(d.extension_line_weight as u16);
    let p = &value.policy;
    main.write_bs(p.text_stack_alignment);
    main.write_bs(p.text_stack_size);
    if (p.current_entity_lineweight, p.end_caps, p.join_style, p.lineweight_display, p.external_reference_editing, p.extended_names, p.plot_style_mode, p.ole_startup) != (-1, 0, 0, false, true, true, true, false) {
        return Err("unsupported AC1024 packed drawing policy".into());
    }
    main.write_bl(0x2a1d);
    main.write_bs(p.insertion_units);
    main.write_bs(p.current_plot_style_type);
    for number in [p.sort_entities, p.index_control, p.hide_text, p.xclip_frame, p.dimension_association, p.halo_gap] {
        main.write_rc(number);
    }
    main.write_bs(p.obscured_color);
    main.write_bs(p.intersection_color);
    main.write_rc(p.obscured_linetype);
    main.write_rc(p.intersection_display);
    main.write_b(p.camera_display);
    main.write_bl(0);
    main.write_bl(10);
    main.write_bd(1.0);
    for number in [p.steps_per_second, p.step_size, p.dwf_3d_precision, p.lens_length, p.camera_height] {
        main.write_bd(number);
    }
    main.write_rc(p.solid_history);
    main.write_rc(p.show_history);
    for number in [p.polysolid_width, p.polysolid_height, p.loft_angle1, p.loft_angle2, p.loft_magnitude1, p.loft_magnitude2] {
        main.write_bd(number);
    }
    main.write_bs(p.loft_parameter);
    main.write_rc(p.loft_normals);
    for number in [p.latitude, p.longitude, p.north_direction] {
        main.write_bd(number);
    }
    main.write_bl(p.timezone as u32);
    for number in [p.light_glyph_display, p.tile_mode_light_sync, p.dwf_frame, p.dgn_frame] {
        main.write_rc(number);
    }
    if p.interfere_color_index != 256 {
        return Err("AC1024 INTERFERECOLOR must be the typed ByLayer color".into());
    }
    main.write_b(p.real_world_scale);
    write_header_color(&mut main, 0, 0xc3000001);
    main.write_rc(p.shadow_mode);
    main.write_bd(p.shadow_plane_location);

    let mut strings = DwgBitWriter::new();
    for text in [
        &u.unit1_name,
        &u.unit2_name,
        &u.unit3_name,
        &u.unit4_name,
        &value.strings.menu,
        &value.strings.dimension_postfix,
        &value.strings.dimension_alternate_postfix,
        &value.strings.dimension_alternate_measurement_zero_suffix,
        &value.strings.dimension_measurement_zero_suffix,
        &value.strings.hyperlink_base,
        &value.strings.stylesheet,
        &value.strings.fingerprint_guid,
        &value.strings.version_guid,
        &value.strings.project_name,
    ] {
        strings.write_tu(text);
    }
    let mut handles = DwgBitWriter::new();
    let r = &value.relations;
    let required = [r.current_layer, r.text_style, r.current_linetype, r.current_material, r.dimension_style, r.multiline_style];
    for handle in required {
        handles.write_handle(5, handle);
    }
    for handle in [r.paper_ucs_name, r.paper_ucs_orthographic_reference, r.paper_ucs_base, r.model_ucs_name, r.model_ucs_orthographic_reference, r.model_ucs_base] {
        handles.write_handle(5, handle.unwrap_or(0));
    }
    handles.write_handle(5, r.dimension_text_style);
    for handle in [r.dimension_leader_block, r.dimension_block, r.dimension_block1, r.dimension_block2, r.dimension_linetype, r.dimension_extension_linetype1, r.dimension_extension_linetype2] {
        handles.write_handle(5, handle.unwrap_or(0));
    }
    for handle in [r.block_control, r.layer_control, r.style_control, r.linetype_control, r.view_control, r.ucs_control, r.viewport_control, r.appid_control, r.dimension_style_control] {
        handles.write_handle(3, handle);
    }
    for handle in [r.group_dictionary, r.multiline_style_dictionary] {
        handles.write_handle(5, handle);
    }
    handles.write_handle(3, r.named_objects_dictionary);
    for handle in [
        r.layout_dictionary,
        r.plot_settings_dictionary,
        r.plot_style_name_dictionary,
        r.material_dictionary,
        r.color_dictionary,
        r.visual_style_dictionary,
        r.paper_space_block_record,
        r.model_space_block_record,
        r.by_layer_linetype,
        r.by_block_linetype,
        r.continuous_linetype,
    ] {
        handles.write_handle(5, handle);
    }
    for handle in [r.interfere_object_visual_style, r.interfere_viewport_visual_style, r.drag_visual_style] {
        handles.write_handle(5, handle.unwrap_or(0));
    }
    for value in [0xbfc4, 0x122d, 0xa23e, 0xb717] {
        handles.write_bs(value);
    }
    handles.write_bl(0);
    handles.write_bl(0);
    handles.write_b(true);
    handles.write_b(true);
    handles.write_b(true);
    handles.write_b(true);
    let string_bits = strings.bit_len();
    if string_bits > 0x7fff {
        return Err(format!("AC1024 Header string stream {string_bits} bits exceeds the compact size"));
    }
    main.append_bits(&strings);
    main.write_rs(string_bits as u16);
    main.write_b(true);
    let boundary = main.bit_len();
    main.append_bits(&handles);
    main.pad_to_byte();
    let mut output = Vec::with_capacity(main.bytes.len() + 42);
    output.extend_from_slice(&DWG_HEADER_BEGIN);
    push_u32(&mut output, u32::try_from(main.bytes.len() + 4).map_err(|_| "AC1024 Header exceeds RL size")?);
    push_u32(&mut output, u32::try_from(boundary + 32).map_err(|_| "AC1024 Header exceeds RL bit size")?);
    output.extend_from_slice(&main.bytes);
    let crc = dwg_crc16(0xc0c1, &output[16..]);
    push_u16(&mut output, crc);
    output.extend_from_slice(&DWG_HEADER_END);
    Ok(output)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_object_handle(writer: &mut DwgBitWriter, base: u64, target: Option<u64>) {
    match target {
        None | Some(0) => writer.write_handle(4, 0),
        Some(value) if value == base.saturating_add(1) => writer.write_handle(6, 0),
        Some(value) if value.saturating_add(1) == base => writer.write_handle(8, 0),
        Some(value) if value > base && value - base < value => writer.write_handle(10, value - base),
        Some(value) if value < base && base - value < value => writer.write_handle(12, base - value),
        Some(value) => writer.write_handle(4, value),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_xrecord_values(values: &[semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgXRecordValue]) -> Result<DwgBitWriter, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgXRecordValue;
    let mut data = DwgBitWriter::new();
    for value in values {
        value.validate()?;
        let group_code = match value {
            DwgXRecordValue::String { group_code, .. }
            | DwgXRecordValue::Real { group_code, .. }
            | DwgXRecordValue::Boolean { group_code, .. }
            | DwgXRecordValue::Integer8 { group_code, .. }
            | DwgXRecordValue::Integer16 { group_code, .. }
            | DwgXRecordValue::Integer32 { group_code, .. }
            | DwgXRecordValue::Integer64 { group_code, .. }
            | DwgXRecordValue::Point3d { group_code, .. }
            | DwgXRecordValue::Binary { group_code, .. }
            | DwgXRecordValue::Handle { group_code, .. }
            | DwgXRecordValue::ObjectId { group_code, .. } => *group_code,
        };
        data.write_rs(group_code as u16);
        match value {
            DwgXRecordValue::String { value, .. } => {
                let units = value.encode_utf16().collect::<Vec<_>>();
                data.write_rs(units.len() as u16);
                for unit in units {
                    data.write_rs(unit);
                }
            }
            DwgXRecordValue::Real { value, .. } => data.write_rd(*value),
            DwgXRecordValue::Boolean { value, .. } => data.write_rc(u8::from(*value)),
            DwgXRecordValue::Integer8 { value, .. } => data.write_rc(*value as u8),
            DwgXRecordValue::Integer16 { value, .. } => data.write_rs(*value as u16),
            DwgXRecordValue::Integer32 { value, .. } => data.write_rl(*value as u32),
            DwgXRecordValue::Integer64 { value, .. } => data.write_rll(*value as u64),
            DwgXRecordValue::Point3d { value, .. } => data.write_3rd(*value),
            DwgXRecordValue::Binary { octets, .. } => {
                data.write_rc(octets.len() as u8);
                for octet in octets {
                    data.write_rc(*octet);
                }
            }
            DwgXRecordValue::Handle { value, .. } => data.write_rll(*value),
            DwgXRecordValue::ObjectId { absolute_value, .. } => data.write_rll(*absolute_value),
        }
    }
    if data.bit != 0 {
        return Err("typed XRECORD values must occupy whole bytes".into());
    }
    Ok(data)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_eed(writer: &mut DwgBitWriter, _base: u64, records: &[semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgExtendedEntityData]) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgXRecordValue;
    for record in records {
        let mut values = DwgBitWriter::new();
        for value in &record.values {
            value.validate()?;
            match value {
                DwgXRecordValue::String { group_code: 1000, value } => {
                    values.write_rc(0);
                    let units = value.encode_utf16().collect::<Vec<_>>();
                    values.write_rs(units.len() as u16);
                    for unit in units {
                        values.write_rs(unit);
                    }
                }
                DwgXRecordValue::String { group_code: 1002, value } => {
                    values.write_rc(2);
                    values.write_rc(match value.as_str() {
                        "{" => 0,
                        "}" => 1,
                        _ => return Err("R2010 EED control string must be '{' or '}'".into()),
                    });
                }
                DwgXRecordValue::Handle { group_code: 1003, value } => {
                    values.write_rc(3);
                    values.write_rll(*value);
                }
                DwgXRecordValue::Binary { group_code: 1004, octets } => {
                    values.write_rc(4);
                    values.write_rc(octets.len() as u8);
                    for octet in octets {
                        values.write_rc(*octet);
                    }
                }
                DwgXRecordValue::Handle { group_code: 1005, value } => {
                    values.write_rc(5);
                    for shift in (0..64).step_by(8).rev() {
                        values.write_rc((value >> shift) as u8);
                    }
                }
                DwgXRecordValue::Point3d { group_code, value } if (1010..=1015).contains(group_code) => {
                    values.write_rc((*group_code - 1000) as u8);
                    values.write_3rd(*value);
                }
                DwgXRecordValue::Real { group_code, value } if (1040..=1042).contains(group_code) => {
                    values.write_rc((*group_code - 1000) as u8);
                    values.write_rd(*value);
                }
                DwgXRecordValue::Integer16 { group_code: 1070, value } => {
                    values.write_rc(70);
                    values.write_rs(*value as u16);
                }
                DwgXRecordValue::Integer32 { group_code: 1071, value } => {
                    values.write_rc(71);
                    values.write_rl(*value as u32);
                }
                _ => return Err(format!("group {} is not a standard R2010 EED value", value.group_code())),
            }
        }
        if values.bit != 0 || values.bytes.len() > usize::from(u16::MAX) {
            return Err("R2010 EED application record is not byte-sized".into());
        }
        writer.write_bs(values.bytes.len() as u16);
        writer.write_handle(5, record.application_handle);
        writer.append_bits(&values);
    }
    writer.write_bs(0);
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn finish_r2010_object_frame(data: &DwgBitWriter, mut handles: DwgBitWriter) -> Result<Vec<u8>, String> {
    while !(data.bit_len() + handles.bit_len()).is_multiple_of(8) {
        handles.write_b(true);
    }
    let handle_stream_bits = handles.bit_len();
    let mut payload = DwgBitWriter::new();
    payload.append_bits(data);
    payload.append_bits(&handles);
    if payload.bit != 0 {
        return Err("R2010 object frame is not byte-aligned".into());
    }
    let mut framed = DwgBitWriter::new();
    framed.write_ms(payload.bytes.len() as u32);
    framed.write_umc(handle_stream_bits as u64);
    framed.append_bits(&payload);
    if framed.bit != 0 {
        return Err("R2010 outer object frame is not byte-aligned".into());
    }
    let crc = dwg_crc16(0xC0C1, &framed.bytes);
    framed.bytes.extend_from_slice(&crc.to_le_bytes());
    Ok(framed.bytes)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_xrecord_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::XRecord(xrecord) = object.body.as_ref().ok_or_else(|| format!("XRECORD {:#x} body missing", object.handle))? else {
        return Err(format!("object {:#x} is not an XRECORD body", object.handle));
    };
    let xdata = encode_xrecord_values(&xrecord.values)?;
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(object.reactor_handles.len() as u32);
    data.write_b(object.extension_dictionary_handle.is_none());
    data.write_bl(xdata.bytes.len() as u32);
    data.append_bits(&xdata);
    data.write_bs(xrecord.cloning_flag);
    data.write_b(false);

    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    for reactor in &object.reactor_handles {
        handles.write_handle(4, *reactor);
    }
    if let Some(handle) = object.extension_dictionary_handle {
        handles.write_handle(4, handle);
    }
    for reference in &xrecord.object_id_handles {
        handles.write_handle(4, *reference);
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_dictionary_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::Dictionary(dictionary) = object.body.as_ref().ok_or_else(|| format!("dictionary {:#x} body missing", object.handle))? else {
        return Err(format!("object {:#x} is not a dictionary body", object.handle));
    };
    if dictionary.cloning_flag > 5 {
        return Err(format!("dictionary {:#x} cloning flag {} is invalid", object.handle, dictionary.cloning_flag));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(object.reactor_handles.len() as u32);
    data.write_b(object.extension_dictionary_handle.is_none());
    data.write_bl(dictionary.entries.len() as u32);
    data.write_bs(dictionary.cloning_flag);
    data.write_rc(u8::from(dictionary.hard_owner));
    let mut strings = DwgBitWriter::new();
    for entry in &dictionary.entries {
        strings.write_tu(&entry.name);
    }
    let string_bits = strings.bit_len();
    if string_bits == 0 {
        data.write_b(false);
    } else {
        data.append_bits(&strings);
        if string_bits > 0x7fff {
            return Err(format!("dictionary {:#x} string stream exceeds compact R2010 size", object.handle));
        }
        data.write_rs(string_bits as u16);
        data.write_b(true);
    }

    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    for reactor in &object.reactor_handles {
        handles.write_handle(4, *reactor);
    }
    if let Some(handle) = object.extension_dictionary_handle {
        handles.write_handle(4, handle);
    }
    for entry in &dictionary.entries {
        handles.write_handle(2, entry.handle);
    }
    if let Some(default_entry) = dictionary.default_entry_handle {
        handles.write_handle(5, default_entry);
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_table_control_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgTableControlBody;
    let semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::TableControl(control) = object.body.as_ref().ok_or_else(|| format!("table control {:#x} body missing", object.handle))? else {
        return Err(format!("object {:#x} is not a table control", object.handle));
    };
    let expected_type = match control {
        DwgTableControlBody::Block(_) => 48,
        DwgTableControlBody::Layer(_) => 50,
        DwgTableControlBody::TextStyle(_) => 52,
        DwgTableControlBody::Linetype(_) => 56,
        DwgTableControlBody::View(_) => 60,
        DwgTableControlBody::Ucs(_) => 62,
        DwgTableControlBody::Viewport(_) => 64,
        DwgTableControlBody::RegisteredApplication(_) => 66,
        DwgTableControlBody::DimensionStyle(_) => 68,
    };
    if object.type_code != expected_type {
        return Err(format!("table-control variant expects type {expected_type}, found {}", object.type_code));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(object.reactor_handles.len() as u32);
    data.write_b(object.extension_dictionary_handle.is_none());
    let entries = control.entry_handles();
    if matches!(object.type_code, 48 | 50 | 52 | 60) {
        data.write_bl(entries.len() as u32);
    } else {
        data.write_bs(entries.len() as u16);
    }
    if let DwgTableControlBody::DimensionStyle(value) = control {
        if value.additional_handles.len() > usize::from(u8::MAX) {
            return Err("DIMSTYLE additional handle count exceeds RC".into());
        }
        data.write_rc(value.additional_handles.len() as u8);
    }
    data.write_b(false);
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    for reactor in &object.reactor_handles {
        handles.write_handle(4, *reactor);
    }
    if let Some(xdic) = object.extension_dictionary_handle {
        handles.write_handle(3, xdic);
    }
    for entry in entries {
        handles.write_handle(2, entry.handle.unwrap_or_default());
    }
    match control {
        DwgTableControlBody::Block(value) => {
            handles.write_handle(3, value.model_space_handle.ok_or("BLOCK_CONTROL model-space handle missing")?);
            handles.write_handle(3, value.paper_space_handle.ok_or("BLOCK_CONTROL paper-space handle missing")?);
        }
        DwgTableControlBody::Linetype(value) => {
            handles.write_handle(3, value.by_block_handle);
            handles.write_handle(3, value.by_layer_handle);
        }
        DwgTableControlBody::DimensionStyle(value) => {
            for additional in &value.additional_handles {
                handles.write_handle(5, *additional);
            }
        }
        _ => {}
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_table_record_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgTableRecordBody;
    let body = match object.body.as_ref() {
        Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::TableRecord(value)) => value,
        _ => return Err(format!("table record {:#x} body missing", object.handle)),
    };
    let (common, expected_type) = match body {
        DwgTableRecordBody::RegisteredApplication(value) => (&value.common, 67),
        DwgTableRecordBody::TextStyle(value) => (&value.common, 53),
        DwgTableRecordBody::Layer(value) => (&value.common, 51),
        DwgTableRecordBody::Linetype(value) => (&value.common, 57),
        DwgTableRecordBody::BlockHeader(value) => (&value.common, 49),
        DwgTableRecordBody::Viewport(value) => (&value.common, 65),
        DwgTableRecordBody::DimensionStyle(value) => (&value.common, 69),
    };
    if object.type_code != expected_type {
        return Err(format!("table-record variant expects type {expected_type}, found {}", object.type_code));
    }
    if !matches!(common.xref_resolution, 0 | 256) {
        return Err(format!("table record {:#x} xref resolution {} is invalid", object.handle, common.xref_resolution));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(object.reactor_handles.len() as u32);
    data.write_b(object.extension_dictionary_handle.is_none());
    data.write_bs(common.xref_resolution);
    let mut strings = DwgBitWriter::new();
    if let DwgTableRecordBody::BlockHeader(value) = body {
        if value.anonymous {
            let mut characters = common.name.chars();
            let prefix = characters.by_ref().take(2).collect::<String>();
            if !prefix.starts_with('*') || prefix.chars().count() != 2 || !characters.as_str().chars().all(|character| character.is_ascii_digit()) {
                return Err(format!("anonymous block header {:#x} name {:?} is not a standard prefix plus numeric identity", object.handle, common.name));
            }
            strings.write_tu(&prefix);
        } else {
            strings.write_tu(&common.name);
        }
    } else {
        strings.write_tu(&common.name);
    }
    match body {
        DwgTableRecordBody::RegisteredApplication(value) => data.write_rc(value.group_71),
        DwgTableRecordBody::TextStyle(value) => {
            data.write_b(value.is_shape);
            data.write_b(value.is_vertical);
            data.write_bd(value.text_size);
            data.write_bd(value.width_factor);
            data.write_bd(value.oblique_angle);
            data.write_rc(value.generation);
            data.write_bd(value.last_height);
            strings.write_tu(&value.font_file);
            strings.write_tu(&value.big_font_file);
        }
        DwgTableRecordBody::Layer(value) => {
            if value.lineweight > 31 {
                return Err(format!("layer {:#x} lineweight {} exceeds five bits", object.handle, value.lineweight));
            }
            let flag0 = u16::from(value.frozen) | (u16::from(value.off) << 1) | (u16::from(value.frozen_in_new_viewports) << 2) | (u16::from(value.locked) << 3) | (u16::from(value.plottable) << 4) | (u16::from(value.lineweight) << 5);
            data.write_bs(flag0);
            data.write_bs(value.color.index);
            data.write_bl(encode_complex_color_value(&value.color.value));
            let color_flags = u8::from(value.color.name.is_some()) | (u8::from(value.color.book_name.is_some()) << 1);
            data.write_rc(color_flags);
            if let Some(name) = &value.color.name {
                strings.write_tu(name);
            }
            if let Some(book_name) = &value.color.book_name {
                strings.write_tu(book_name);
            }
        }
        DwgTableRecordBody::Linetype(value) => {
            if value.dashes.len() > usize::from(u8::MAX) {
                return Err(format!("linetype {:#x} has too many dashes", object.handle));
            }
            if value.dashes.iter().any(|dash| dash.text.is_some()) {
                return Err(format!("linetype {:#x} textual dash area is not yet supported", object.handle));
            }
            strings.write_tu(&value.description);
            data.write_bd(value.pattern_length);
            data.write_rc(value.alignment);
            data.write_rc(value.dashes.len() as u8);
            for dash in &value.dashes {
                data.write_bd(dash.length);
                data.write_bs(dash.complex_shape_code);
                data.write_rd(dash.x_offset);
                data.write_rd(dash.y_offset);
                data.write_bd(dash.scale);
                data.write_bd(dash.rotation);
                data.write_bs(dash.shape_flags);
            }
        }
        DwgTableRecordBody::BlockHeader(value) => {
            if (value.is_xref || value.xref_overlaid) && !value.owned_entity_handles.is_empty() {
                return Err(format!("block header {:#x} xref cannot own entities", object.handle));
            }
            if value.insert_backreference_handles.len() > 0xefffff {
                return Err(format!("block header {:#x} has too many insert backreferences", object.handle));
            }
            data.write_b(value.anonymous);
            data.write_b(value.has_attributes);
            data.write_b(value.is_xref);
            data.write_b(value.xref_overlaid);
            data.write_b(value.xref_loaded);
            if !value.is_xref && !value.xref_overlaid {
                data.write_bl(value.owned_entity_handles.len() as u32);
            }
            data.write_3bd(value.base_point);
            strings.write_tu(&value.xref_path);
            for _ in &value.insert_backreference_handles {
                data.write_rc(1);
            }
            data.write_rc(0);
            strings.write_tu(&value.description);
            data.write_bl(0);
            data.write_bs(value.insert_units);
            data.write_b(value.explodable);
            data.write_rc(value.block_scaling);
        }
        DwgTableRecordBody::Viewport(value) => {
            data.write_bd(value.view_height);
            data.write_bd(value.view_width);
            data.write_2rd(value.center);
            data.write_3bd(value.target);
            data.write_3bd(value.direction);
            data.write_bd(value.twist);
            data.write_bd(value.lens_length);
            data.write_bd(value.front_clipping);
            data.write_bd(value.back_clipping);
            for flag in value.view_mode {
                data.write_b(flag);
            }
            data.write_rc(value.render_mode);
            data.write_b(value.use_default_lights);
            data.write_rc(value.default_lighting_type);
            data.write_bd(value.brightness);
            data.write_bd(value.contrast);
            data.write_bs(value.ambient_color.index);
            data.write_bl(encode_complex_color_value(&value.ambient_color.value));
            let color_flags = u8::from(value.ambient_color.name.is_some()) | (u8::from(value.ambient_color.book_name.is_some()) << 1);
            data.write_rc(color_flags);
            if let Some(name) = &value.ambient_color.name {
                strings.write_tu(name);
            }
            if let Some(book_name) = &value.ambient_color.book_name {
                strings.write_tu(book_name);
            }
            data.write_2rd(value.lower_left);
            data.write_2rd(value.upper_right);
            data.write_b(value.ucs_follow);
            data.write_bs(value.circle_zoom);
            data.write_b(value.fast_zoom);
            data.write_bb(value.ucs_icon);
            data.write_b(value.grid_mode);
            data.write_2rd(value.grid_unit);
            data.write_b(value.snap_mode);
            data.write_b(value.snap_style);
            data.write_bs(value.snap_isopair);
            data.write_bd(value.snap_angle);
            data.write_2rd(value.snap_base);
            data.write_2rd(value.snap_unit);
            data.write_b(value.ucs_at_origin);
            data.write_b(value.ucs_viewport);
            data.write_3bd(value.ucs_origin);
            data.write_3bd(value.ucs_x_axis);
            data.write_3bd(value.ucs_y_axis);
            data.write_bd(value.ucs_elevation);
            data.write_bs(value.ucs_orthographic_view);
            data.write_bs(value.grid_flags);
            data.write_bs(value.grid_major);
        }
        DwgTableRecordBody::DimensionStyle(value) => {
            let write_color = |data: &mut DwgBitWriter, strings: &mut DwgBitWriter, color: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgComplexColor| {
                data.write_bs(color.index);
                data.write_bl(encode_complex_color_value(&color.value));
                data.write_rc(u8::from(color.name.is_some()) | (u8::from(color.book_name.is_some()) << 1));
                if let Some(name) = &color.name {
                    strings.write_tu(name);
                }
                if let Some(book) = &color.book_name {
                    strings.write_tu(book);
                }
            };
            strings.write_tu(&value.dimension_postfix);
            strings.write_tu(&value.alternate_postfix);
            let g = &value.geometry;
            data.write_bd(g.scale);
            data.write_bd(g.arrow_size);
            data.write_bd(g.extension_origin_offset);
            data.write_bd(g.dimension_line_increment);
            data.write_bd(g.extension_line_extension);
            data.write_bd(g.rounding);
            data.write_bd(g.dimension_line_extension);
            data.write_bd(g.plus_tolerance);
            data.write_bd(g.minus_tolerance);
            data.write_bd(g.fixed_extension_length);
            data.write_bd(g.jog_angle);
            data.write_bs(value.fill_mode);
            write_color(&mut data, &mut strings, &value.fill_color);
            let b = &value.behavior;
            data.write_b(b.tolerance);
            data.write_b(b.limits);
            data.write_b(b.text_inside_horizontal);
            data.write_b(b.text_outside_horizontal);
            data.write_b(b.suppress_extension_1);
            data.write_b(b.suppress_extension_2);
            data.write_bs(b.text_vertical_alignment);
            data.write_bs(b.zero_suppression);
            data.write_bs(b.angular_zero_suppression);
            data.write_bs(b.arc_symbol);
            let t = &value.text;
            data.write_bd(t.height);
            data.write_bd(t.center_mark_size);
            data.write_bd(t.tick_size);
            data.write_bd(t.alternate_scale);
            data.write_bd(t.linear_scale);
            data.write_bd(t.vertical_position);
            data.write_bd(t.tolerance_scale);
            data.write_bd(t.gap);
            data.write_bd(t.alternate_rounding);
            data.write_b(t.alternate_enabled);
            data.write_bs(t.alternate_decimals);
            data.write_b(t.text_outside_extensions);
            data.write_b(t.separate_arrowheads);
            data.write_b(t.force_text_inside);
            data.write_b(t.suppress_outside_extensions);
            write_color(&mut data, &mut strings, &t.dimension_line_color);
            write_color(&mut data, &mut strings, &t.extension_line_color);
            write_color(&mut data, &mut strings, &t.text_color);
            let u = &value.units;
            data.write_bs(u.alternate_decimal_places);
            data.write_bs(u.decimal_places);
            data.write_bs(u.tolerance_decimal_places);
            data.write_bs(u.alternate_units);
            data.write_bs(u.alternate_tolerance_decimal_places);
            data.write_bs(u.angular_units);
            data.write_bs(u.fraction_format);
            data.write_bs(u.linear_units);
            data.write_bs(u.decimal_separator);
            data.write_bs(u.text_movement);
            data.write_bs(u.text_horizontal_alignment);
            data.write_b(u.suppress_dimension_line_1);
            data.write_b(u.suppress_dimension_line_2);
            data.write_bs(u.tolerance_vertical_alignment);
            data.write_bs(u.tolerance_zero_suppression);
            data.write_bs(u.alternate_zero_suppression);
            data.write_bs(u.alternate_tolerance_zero_suppression);
            data.write_b(u.user_positioned_text);
            data.write_bs(u.arrow_text_fit);
            let r = &value.r2010;
            data.write_b(r.fixed_extension_enabled);
            data.write_b(r.text_direction);
            data.write_bd(r.alternate_measurement_factor);
            strings.write_tu(&r.alternate_measurement_suffix);
            data.write_bd(r.measurement_factor);
            strings.write_tu(&r.measurement_suffix);
            data.write_bs(r.dimension_lineweight);
            data.write_bs(r.extension_lineweight);
            data.write_b(r.flag);
        }
    }
    let string_bits = strings.bit_len();
    if string_bits > 0x7fff {
        return Err(format!("table record {:#x} strings exceed compact R2010 size", object.handle));
    }
    data.append_bits(&strings);
    data.write_rs(string_bits as u16);
    data.write_b(true);

    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    for reactor in &object.reactor_handles {
        handles.write_handle(4, *reactor);
    }
    if let Some(xdic) = object.extension_dictionary_handle {
        handles.write_handle(3, xdic);
    }
    handles.write_handle(5, common.xref_handle.unwrap_or_default());
    if let DwgTableRecordBody::Layer(value) = body {
        handles.write_handle(5, value.plot_style_handle.unwrap_or_default());
        handles.write_handle(5, value.material_handle.unwrap_or_default());
        handles.write_handle(5, value.linetype_handle.unwrap_or_default());
    }
    if let DwgTableRecordBody::Linetype(value) = body {
        for dash in &value.dashes {
            handles.write_handle(5, dash.style_handle.unwrap_or_default());
        }
    }
    if let DwgTableRecordBody::BlockHeader(value) = body {
        handles.write_handle(3, value.block_entity_handle);
        for owned in &value.owned_entity_handles {
            handles.write_handle(3, *owned);
        }
        handles.write_handle(3, value.end_block_entity_handle);
        for insert in &value.insert_backreference_handles {
            handles.write_handle(4, *insert);
        }
        handles.write_handle(5, value.layout_handle.unwrap_or_default());
    }
    if let DwgTableRecordBody::Viewport(value) = body {
        handles.write_handle(4, value.background_handle.unwrap_or_default());
        handles.write_handle(5, value.visual_style_handle.unwrap_or_default());
        handles.write_handle(3, value.sun_handle.unwrap_or_default());
        handles.write_handle(5, value.named_ucs_handle.unwrap_or_default());
        handles.write_handle(5, value.base_ucs_handle.unwrap_or_default());
    }
    if let DwgTableRecordBody::DimensionStyle(value) = body {
        handles.write_handle(5, value.text_style_handle.unwrap_or_default());
        handles.write_handle(5, value.leader_arrow_handle.unwrap_or_default());
        handles.write_handle(5, value.arrow_handle.unwrap_or_default());
        handles.write_handle(5, value.arrow_1_handle.unwrap_or_default());
        handles.write_handle(5, value.arrow_2_handle.unwrap_or_default());
        handles.write_handle(5, value.dimension_linetype_handle.unwrap_or_default());
        handles.write_handle(5, value.extension_1_linetype_handle.unwrap_or_default());
        handles.write_handle(5, value.extension_2_linetype_handle.unwrap_or_default());
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn entity_mode_bits(value: semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityMode) -> u8 {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityMode;
    match value {
        DwgEntityMode::ExplicitOwner => 0,
        DwgEntityMode::PaperSpace => 1,
        DwgEntityMode::ModelSpace => 2,
        DwgEntityMode::Reserved => 3,
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn entity_reference_bits(value: semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityReferenceMode) -> u8 {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityReferenceMode;
    match value {
        DwgEntityReferenceMode::ByLayer => 0,
        DwgEntityReferenceMode::ByBlock => 1,
        DwgEntityReferenceMode::Continuous => 2,
        DwgEntityReferenceMode::Explicit => 3,
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_entity_common_main(data: &mut DwgBitWriter, object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject, common: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityCommon) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityColorKind;
    data.write_b(false);
    data.write_bb(entity_mode_bits(common.mode));
    data.write_bl(object.reactor_handles.len() as u32);
    data.write_b(object.extension_dictionary_handle.is_none());
    let encoded_color = match common.color.kind {
        DwgEntityColorKind::ByLayer => 256,
        DwgEntityColorKind::ByBlock => 0,
        DwgEntityColorKind::Index => common.color.index,
        DwgEntityColorKind::TrueColor => return Err("R2010 true-color entity writing requires the complete typed ENC string stream".into()),
    };
    data.write_bs(encoded_color);
    data.write_bd(common.linetype_scale);
    data.write_bb(entity_reference_bits(common.linetype));
    data.write_bb(entity_reference_bits(common.plot_style));
    data.write_bb(entity_reference_bits(common.material));
    data.write_rc(common.shadow);
    data.write_b(common.full_visual_style_handle.is_some());
    data.write_b(common.face_visual_style_handle.is_some());
    data.write_b(common.edge_visual_style_handle.is_some());
    data.write_bs(common.invisible);
    data.write_rc(common.lineweight);
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_entity_common_handles(handles: &mut DwgBitWriter, object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject, common: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityCommon) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityColorKind;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityMode;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityReferenceMode;
    if common.color.kind == DwgEntityColorKind::TrueColor {
        handles.write_handle(5, common.color.color_handle.unwrap_or_default());
    }
    if common.mode == DwgEntityMode::ExplicitOwner {
        let owner = object.owner_handle.ok_or_else(|| format!("entity {:#x} explicit-owner mode has no owner", object.handle))?;
        if owner.saturating_add(1) == object.handle {
            handles.write_handle(8, 0);
        } else {
            let delta = object.handle.checked_sub(owner).ok_or_else(|| format!("entity {:#x} owner {owner:#x} is not lower", object.handle))?;
            handles.write_handle(12, delta);
        }
    } else if object.owner_handle.is_some() {
        return Err(format!("entity {:#x} non-owner mode has an owner handle", object.handle));
    }
    for reactor in &object.reactor_handles {
        handles.write_handle(4, *reactor);
    }
    if let Some(extension_dictionary) = object.extension_dictionary_handle {
        handles.write_handle(3, extension_dictionary);
    }
    handles.write_handle(5, common.layer_handle);
    if common.linetype == DwgEntityReferenceMode::Explicit {
        handles.write_handle(5, common.linetype_handle.unwrap_or_default());
    }
    if common.material == DwgEntityReferenceMode::Explicit {
        handles.write_handle(5, common.material_handle.unwrap_or_default());
    }
    if common.shadow == 3 {
        handles.write_handle(5, common.shadow_handle.unwrap_or_default());
    }
    if common.plot_style == DwgEntityReferenceMode::Explicit {
        handles.write_handle(5, common.plot_style_handle.unwrap_or_default());
    }
    for visual in [common.full_visual_style_handle, common.face_visual_style_handle, common.edge_visual_style_handle].into_iter().flatten() {
        handles.write_handle(5, visual);
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn logical_point3(values: &[f64], name: &str) -> Result<[f64; 3], String> {
    if values.len() != 3 || values.iter().any(|value| !value.is_finite()) {
        return Err(format!("{name} must contain exactly three finite coordinates"));
    }
    Ok([values[0], values[1], values[2]])
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn logical_point2(values: &[f64], name: &str) -> Result<[f64; 2], String> {
    if values.len() != 2 || values.iter().any(|value| !value.is_finite()) {
        return Err(format!("{name} must contain exactly two finite coordinates"));
    }
    Ok([values[0], values[1]])
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dimension_attachment_wire(value: semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDimensionTextAttachment) -> u16 {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDimensionTextAttachment::*;
    match value {
        TopCenter => 1,
        TopLeft => 2,
        TopRight => 3,
        MiddleCenter => 4,
        MiddleLeft => 5,
        MiddleRight => 6,
        BottomCenter => 7,
        BottomLeft => 8,
        BottomRight => 9,
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dimension_spacing_wire(value: semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDimensionLineSpacingStyle) -> u16 {
    match value {
        semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDimensionLineSpacingStyle::AtLeast => 1,
        semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDimensionLineSpacingStyle::Exact => 2,
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_block_begin_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject, name: &str) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::Entity(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityBody::BlockBegin(block))) = object.body.as_ref() else {
        return Err(format!("BLOCK {:#x} typed body missing", object.handle));
    };
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    encode_r2010_entity_common_main(&mut data, object, &block.common)?;
    let mut strings = DwgBitWriter::new();
    strings.write_tu(name);
    append_r2010_string_stream(&mut data, &strings, "BLOCK", object.handle)?;
    let mut handles = DwgBitWriter::new();
    encode_r2010_entity_common_handles(&mut handles, object, &block.common)?;
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_block_end_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::Entity(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityBody::BlockEnd(block))) = object.body.as_ref() else {
        return Err(format!("ENDBLK {:#x} typed body missing", object.handle));
    };
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    encode_r2010_entity_common_main(&mut data, object, &block.common)?;
    data.write_b(false);
    let mut handles = DwgBitWriter::new();
    encode_r2010_entity_common_handles(&mut handles, object, &block.common)?;
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_insert_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::Entity(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityBody::Insert(insert))) = object.body.as_ref() else {
        return Err(format!("INSERT {:#x} typed body missing", object.handle));
    };
    let insertion = logical_point3(&insert.insertion, "INSERT insertion")?;
    let scale = logical_point3(&insert.scale, "INSERT scale")?;
    let extrusion = logical_point3(&insert.extrusion, "INSERT extrusion")?;
    if scale.contains(&0.0) || !insert.rotation.is_finite() || extrusion == [0.0; 3] || insert.block_header_handle == 0 {
        return Err(format!("INSERT {:#x} transform or block-header relationship is invalid", object.handle));
    }
    let has_attributes = !insert.attribute_handles.is_empty();
    if has_attributes != insert.sequence_end_handle.is_some() {
        return Err(format!("INSERT {:#x} attributes and SEQEND must be present together", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    encode_r2010_entity_common_main(&mut data, object, &insert.common)?;
    data.write_3bd(insertion);
    if scale == [1.0; 3] {
        data.write_bb(3);
    } else if scale[0].to_bits() == scale[1].to_bits() && scale[0].to_bits() == scale[2].to_bits() {
        data.write_bb(2);
        data.write_rd(scale[0]);
    } else if scale[0] == 1.0 {
        data.write_bb(1);
        data.write_dd(scale[1], 1.0);
        data.write_dd(scale[2], 1.0);
    } else {
        data.write_bb(0);
        data.write_rd(scale[0]);
        data.write_dd(scale[1], scale[0]);
        data.write_dd(scale[2], scale[0]);
    }
    data.write_bd(insert.rotation);
    data.write_3bd(extrusion);
    data.write_b(has_attributes);
    if has_attributes {
        data.write_bl(insert.attribute_handles.len() as u32);
    }
    data.write_b(false);
    let mut handles = DwgBitWriter::new();
    encode_r2010_entity_common_handles(&mut handles, object, &insert.common)?;
    handles.write_handle(5, insert.block_header_handle);
    for attribute in &insert.attribute_handles {
        handles.write_handle(4, *attribute);
    }
    if let Some(sequence_end) = insert.sequence_end_handle {
        handles.write_handle(3, sequence_end);
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_dimension_linear_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::Entity(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityBody::DimensionLinear(linear))) = object.body.as_ref() else {
        return Err(format!("DIMENSION_LINEAR {:#x} typed body missing", object.handle));
    };
    let dimension = &linear.dimension;
    let extrusion = logical_point3(&dimension.extrusion, "dimension extrusion")?;
    let text_midpoint = logical_point2(&dimension.text_midpoint, "dimension text midpoint")?;
    let insertion_scale = logical_point3(&dimension.insertion_scale, "dimension insertion scale")?;
    let clone_point = logical_point2(&dimension.clone_insertion_point, "dimension clone insertion point")?;
    let extension_line_1 = logical_point3(&linear.extension_line_1, "dimension extension line 1")?;
    let extension_line_2 = logical_point3(&linear.extension_line_2, "dimension extension line 2")?;
    let definition_point = logical_point3(&linear.definition_point, "dimension definition point")?;
    if dimension.dimension_style_handle == 0
        || dimension.line_spacing_factor <= 0.0
        || insertion_scale.contains(&0.0)
        || [dimension.elevation, dimension.text_rotation, dimension.horizontal_direction, dimension.insertion_rotation, dimension.line_spacing_factor, dimension.actual_measurement, linear.oblique_angle, linear.dimension_rotation]
            .iter()
            .any(|value| !value.is_finite())
    {
        return Err(format!("DIMENSION_LINEAR {:#x} contains invalid semantic values", object.handle));
    }
    let flag = 0x08 | (if dimension.status.block_reference_is_exclusive { 0x20 | 0x02 } else { 0 }) | (if dimension.status.user_positioned_text { 0x80 } else { 0x01 });
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    encode_r2010_entity_common_main(&mut data, object, &dimension.common)?;
    data.write_rc(0);
    data.write_3bd(extrusion);
    data.write_2rd(text_midpoint);
    data.write_bd(dimension.elevation);
    data.write_rc(flag);
    data.write_bd(dimension.text_rotation);
    data.write_bd(dimension.horizontal_direction);
    data.write_3bd(insertion_scale);
    data.write_bd(dimension.insertion_rotation);
    data.write_bs(dimension_attachment_wire(dimension.attachment));
    data.write_bs(dimension_spacing_wire(dimension.line_spacing_style));
    data.write_bd(dimension.line_spacing_factor);
    data.write_bd(dimension.actual_measurement);
    data.write_b(false);
    data.write_b(dimension.flip_arrow_1);
    data.write_b(dimension.flip_arrow_2);
    data.write_2rd(clone_point);
    data.write_3bd(extension_line_1);
    data.write_3bd(extension_line_2);
    data.write_3bd(definition_point);
    data.write_bd(linear.oblique_angle);
    data.write_bd(linear.dimension_rotation);
    let mut strings = DwgBitWriter::new();
    strings.write_tu(&dimension.user_text);
    append_r2010_string_stream(&mut data, &strings, "DIMENSION_LINEAR", object.handle)?;
    let mut handles = DwgBitWriter::new();
    encode_r2010_entity_common_handles(&mut handles, object, &dimension.common)?;
    handles.write_handle(5, dimension.dimension_style_handle);
    handles.write_handle(5, dimension.dimension_block_handle.unwrap_or_default());
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn viewport_status_mask(flags: &[semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgViewportStatusFlag]) -> Result<u32, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgViewportStatusFlag::*;
    let mut mask = 0u32;
    for flag in flags {
        let bit = match flag {
            Perspective => 0,
            FrontClipping => 1,
            BackClipping => 2,
            UcsFollow => 3,
            FrontClipNotAtEye => 4,
            UcsIconVisible => 5,
            UcsIconAtOrigin => 6,
            FastZoom => 7,
            Snap => 8,
            Grid => 9,
            IsometricSnap => 10,
            HidePlot => 11,
            IsoPairTop => 12,
            IsoPairRight => 13,
            ZoomLock => 14,
            AlwaysEnabled => 15,
            NonRectangularClipping => 16,
            ViewportOff => 17,
            GridBeyondDrawingLimits => 18,
            AdaptiveGrid => 19,
            AdaptiveSubdivision => 20,
            GridFollowsWorkplane => 21,
        };
        if mask & (1 << bit) != 0 {
            return Err("VIEWPORT status contains a duplicate flag".into());
        }
        mask |= 1 << bit;
    }
    Ok(mask)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_viewport_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDefaultLightingType;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityBody;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgOrthographicView;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgShadePlotMode;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgViewportRenderMode;
    let Some(DwgLogicalObjectBody::Entity(DwgEntityBody::Viewport(viewport))) = object.body.as_ref() else {
        return Err(format!("VIEWPORT {:#x} body missing", object.handle));
    };
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    encode_r2010_entity_common_main(&mut data, object, &viewport.common)?;
    data.write_3bd(logical_point3(&viewport.center, "VIEWPORT center")?);
    data.write_bd(viewport.width);
    data.write_bd(viewport.height);
    data.write_3bd(logical_point3(&viewport.view_target, "VIEWPORT view target")?);
    data.write_3bd(logical_point3(&viewport.view_direction, "VIEWPORT view direction")?);
    for value in [viewport.twist_angle, viewport.view_height, viewport.lens_length, viewport.front_clip, viewport.back_clip, viewport.snap_angle] {
        data.write_bd(value);
    }
    data.write_2rd(logical_point2(&viewport.view_center, "VIEWPORT view center")?);
    data.write_2rd(logical_point2(&viewport.snap_base, "VIEWPORT snap base")?);
    data.write_2rd(logical_point2(&viewport.snap_unit, "VIEWPORT snap unit")?);
    data.write_2rd(logical_point2(&viewport.grid_unit, "VIEWPORT grid unit")?);
    data.write_bs(viewport.circle_zoom_percent);
    data.write_bs(viewport.grid_major);
    data.write_bl(u32::try_from(viewport.frozen_layer_handles.len()).map_err(|_| "VIEWPORT frozen-layer count exceeds u32")?);
    data.write_bl(viewport_status_mask(&viewport.status)?);
    data.write_rc(match viewport.render_mode {
        DwgViewportRenderMode::Optimized2d => 0,
        DwgViewportRenderMode::Wireframe => 1,
        DwgViewportRenderMode::HiddenLine => 2,
        DwgViewportRenderMode::FlatShaded => 3,
        DwgViewportRenderMode::GouraudShaded => 4,
        DwgViewportRenderMode::FlatShadedWithWireframe => 5,
        DwgViewportRenderMode::GouraudShadedWithWireframe => 6,
    });
    data.write_b(viewport.ucs_at_origin);
    data.write_b(viewport.ucs_per_viewport);
    data.write_3bd(logical_point3(&viewport.ucs_origin, "VIEWPORT UCS origin")?);
    data.write_3bd(logical_point3(&viewport.ucs_x_axis, "VIEWPORT UCS X axis")?);
    data.write_3bd(logical_point3(&viewport.ucs_y_axis, "VIEWPORT UCS Y axis")?);
    data.write_bd(viewport.ucs_elevation);
    data.write_bs(match viewport.orthographic_view {
        DwgOrthographicView::None => 0,
        DwgOrthographicView::Top => 1,
        DwgOrthographicView::Bottom => 2,
        DwgOrthographicView::Front => 3,
        DwgOrthographicView::Back => 4,
        DwgOrthographicView::Left => 5,
        DwgOrthographicView::Right => 6,
    });
    data.write_bs(match viewport.shade_plot_mode {
        DwgShadePlotMode::AsDisplayed => 0,
        DwgShadePlotMode::Wireframe => 1,
        DwgShadePlotMode::Hidden => 2,
        DwgShadePlotMode::Rendered => 3,
    });
    data.write_b(viewport.use_default_lights);
    data.write_rc(match viewport.default_lighting_type {
        DwgDefaultLightingType::OneDistantLight => 0,
        DwgDefaultLightingType::TwoDistantLights => 1,
    });
    data.write_bd(viewport.brightness);
    data.write_bd(viewport.contrast);
    data.write_bs(viewport.ambient_color.index);
    data.write_bl(encode_complex_color_value(&viewport.ambient_color.value));
    data.write_rc(u8::from(viewport.ambient_color.name.is_some()) | (u8::from(viewport.ambient_color.book_name.is_some()) << 1));
    let mut strings = DwgBitWriter::new();
    strings.write_tu(&viewport.style_sheet);
    if let Some(name) = &viewport.ambient_color.name {
        strings.write_tu(name);
    }
    if let Some(name) = &viewport.ambient_color.book_name {
        strings.write_tu(name);
    }
    append_r2010_string_stream(&mut data, &strings, "VIEWPORT", object.handle)?;
    let mut handles = DwgBitWriter::new();
    encode_r2010_entity_common_handles(&mut handles, object, &viewport.common)?;
    for layer in &viewport.frozen_layer_handles {
        handles.write_handle(5, *layer);
    }
    handles.write_handle(5, viewport.clip_boundary_handle.unwrap_or_default());
    handles.write_handle(5, viewport.named_ucs_handle.unwrap_or_default());
    handles.write_handle(5, viewport.base_ucs_handle.unwrap_or_default());
    handles.write_handle(4, viewport.background_handle.unwrap_or_default());
    handles.write_handle(5, viewport.visual_style_handle.unwrap_or_default());
    handles.write_handle(4, viewport.shade_plot_handle.unwrap_or_default());
    handles.write_handle(3, viewport.sun_handle.unwrap_or_default());
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_line_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::Entity(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityBody::Line(line))) = object.body.as_ref() else {
        return Err(format!("LINE {:#x} typed body missing", object.handle));
    };
    let start = logical_point3(&line.start, "LINE start")?;
    let end = logical_point3(&line.end, "LINE end")?;
    let extrusion = logical_point3(&line.extrusion, "LINE extrusion")?;
    if !line.thickness.is_finite() {
        return Err("LINE thickness must be finite".into());
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    encode_r2010_entity_common_main(&mut data, object, &line.common)?;
    let z_is_zero = start[2] == 0.0 && end[2] == 0.0;
    data.write_b(z_is_zero);
    data.write_rd(start[0]);
    data.write_dd(end[0], start[0]);
    data.write_rd(start[1]);
    data.write_dd(end[1], start[1]);
    if !z_is_zero {
        data.write_rd(start[2]);
        data.write_dd(end[2], start[2]);
    }
    data.write_bt(line.thickness);
    data.write_be(extrusion);
    data.write_b(false);
    let mut handles = DwgBitWriter::new();
    encode_r2010_entity_common_handles(&mut handles, object, &line.common)?;
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_arc_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::Entity(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityBody::Arc(arc))) = object.body.as_ref() else {
        return Err(format!("ARC {:#x} typed body missing", object.handle));
    };
    let center = logical_point3(&arc.center, "ARC center")?;
    let extrusion = logical_point3(&arc.extrusion, "ARC extrusion")?;
    if !arc.radius.is_finite() || arc.radius < 0.0 || !arc.thickness.is_finite() || !arc.start_angle.is_finite() || !arc.end_angle.is_finite() || extrusion == [0.0; 3] {
        return Err("ARC scalar values or extrusion are invalid".into());
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    encode_r2010_entity_common_main(&mut data, object, &arc.common)?;
    data.write_3bd(center);
    data.write_bd(arc.radius);
    data.write_bt(arc.thickness);
    data.write_be(extrusion);
    data.write_bd(arc.start_angle);
    data.write_bd(arc.end_angle);
    data.write_b(false);
    let mut handles = DwgBitWriter::new();
    encode_r2010_entity_common_handles(&mut handles, object, &arc.common)?;
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_lwpolyline_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::Entity(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityBody::LwPolyline(polyline))) = object.body.as_ref() else {
        return Err(format!("LWPOLYLINE {:#x} typed body missing", object.handle));
    };
    if polyline.vertices.is_empty() || polyline.vertices.len() > 20_000 {
        return Err(format!("LWPOLYLINE {:#x} vertex count is invalid", object.handle));
    }
    let extrusion = logical_point3(&polyline.extrusion, "LWPOLYLINE extrusion")?;
    let points = polyline
        .vertices
        .iter()
        .map(|vertex| if vertex.point.len() != 2 || vertex.point.iter().any(|value| !value.is_finite()) { Err("LWPOLYLINE vertex must have two finite coordinates".to_string()) } else { Ok([vertex.point[0], vertex.point[1]]) })
        .collect::<Result<Vec<_>, _>>()?;
    let has_bulges = polyline.vertices.iter().any(|vertex| vertex.bulge != 0.0);
    let has_ids = polyline.vertices.iter().any(|vertex| vertex.vertex_id.is_some());
    let has_widths = polyline.vertices.iter().any(|vertex| vertex.start_width.is_some() || vertex.end_width.is_some());
    if has_ids && polyline.vertices.iter().any(|vertex| vertex.vertex_id.is_none()) {
        return Err("LWPOLYLINE vertex IDs must cover every vertex".into());
    }
    if has_widths && polyline.vertices.iter().any(|vertex| vertex.start_width.is_none() || vertex.end_width.is_none()) {
        return Err("LWPOLYLINE widths must cover every vertex".into());
    }
    let mut flags = if polyline.closed { 512 } else { 0 };
    flags |= u16::from(polyline.constant_width.is_some()) * 4;
    flags |= u16::from(polyline.elevation != 0.0) * 8;
    flags |= u16::from(polyline.thickness != 0.0) * 2;
    flags |= u16::from(extrusion != [0.0, 0.0, 1.0]);
    flags |= u16::from(has_bulges) * 16;
    flags |= u16::from(has_widths) * 32;
    flags |= u16::from(has_ids) * 1024;
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    encode_r2010_entity_common_main(&mut data, object, &polyline.common)?;
    data.write_bs(flags);
    if let Some(width) = polyline.constant_width {
        data.write_bd(width);
    }
    if polyline.elevation != 0.0 {
        data.write_bd(polyline.elevation);
    }
    if polyline.thickness != 0.0 {
        data.write_bd(polyline.thickness);
    }
    if extrusion != [0.0, 0.0, 1.0] {
        data.write_3bd(extrusion);
    }
    data.write_bl(points.len() as u32);
    if has_bulges {
        data.write_bl(points.len() as u32);
    }
    if has_ids {
        data.write_bl(points.len() as u32);
    }
    if has_widths {
        data.write_bl(points.len() as u32);
    }
    data.write_2rd(points[0]);
    for index in 1..points.len() {
        data.write_dd(points[index][0], points[index - 1][0]);
        data.write_dd(points[index][1], points[index - 1][1]);
    }
    if has_bulges {
        for vertex in &polyline.vertices {
            data.write_bd(vertex.bulge);
        }
    }
    if has_ids {
        for vertex in &polyline.vertices {
            data.write_bl(vertex.vertex_id.unwrap());
        }
    }
    if has_widths {
        for vertex in &polyline.vertices {
            data.write_bd(vertex.start_width.unwrap());
            data.write_bd(vertex.end_width.unwrap());
        }
    }
    data.write_b(false);
    let mut handles = DwgBitWriter::new();
    encode_r2010_entity_common_handles(&mut handles, object, &polyline.common)?;
    finish_r2010_object_frame(&data, handles)
}
/// 🧱️ One R2010 entity frame: type, handle, EED and common data, then `body` writes the entity's own
/// main-stream fields (and strings), then the common handle references and `references`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_entity_frame(
    object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject,
    common: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityCommon,
    body: impl FnOnce(&mut DwgBitWriter, &mut DwgBitWriter) -> Result<(), String>,
    references: impl FnOnce(&mut DwgBitWriter),
) -> Result<Vec<u8>, String> {
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    encode_r2010_entity_common_main(&mut data, object, common)?;
    let mut strings = DwgBitWriter::new();
    body(&mut data, &mut strings)?;
    append_r2010_string_stream(&mut data, &strings, &object.class_name, object.handle)?;
    let mut handles = DwgBitWriter::new();
    encode_r2010_entity_common_handles(&mut handles, object, common)?;
    references(&mut handles);
    finish_r2010_object_frame(&data, handles)
}
/// ✏️ POINT, CIRCLE, ELLIPSE, TEXT, SPLINE, 3DFACE and the POLYLINE_3D / POLYLINE_PFACE families
/// (their VERTEX_3D, VERTEX_PFACE, VERTEX_PFACE_FACE and SEQEND sub-entities), field for field as
/// ODA §20.4 lays them out for R2010.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_entity_body_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityBody;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
    let Some(DwgLogicalObjectBody::Entity(body)) = object.body.as_ref() else { return Err(format!("{} {:#x} typed body missing", object.class_name, object.handle)) };
    let name = object.class_name.as_str();
    let finite = |values: &[f64]| values.iter().all(|value| value.is_finite());
    match body {
        DwgEntityBody::Point(point) => {
            let at = logical_point3(&point.point, "POINT position")?;
            let extrusion = logical_point3(&point.extrusion, "POINT extrusion")?;
            if !finite(&[point.thickness, point.x_axis_angle]) {
                return Err("POINT scalars must be finite".into());
            }
            encode_r2010_entity_frame(
                object,
                &point.common,
                |data, _| {
                    data.write_3bd(at);
                    data.write_bt(point.thickness);
                    data.write_be(extrusion);
                    data.write_bd(point.x_axis_angle);
                    Ok(())
                },
                |_| {},
            )
        }
        DwgEntityBody::Circle(circle) => {
            let center = logical_point3(&circle.center, "CIRCLE center")?;
            let extrusion = logical_point3(&circle.extrusion, "CIRCLE extrusion")?;
            if !finite(&[circle.radius, circle.thickness]) || circle.radius < 0.0 || extrusion == [0.0; 3] {
                return Err("CIRCLE radius, thickness or extrusion is invalid".into());
            }
            encode_r2010_entity_frame(
                object,
                &circle.common,
                |data, _| {
                    data.write_3bd(center);
                    data.write_bd(circle.radius);
                    data.write_bt(circle.thickness);
                    data.write_be(extrusion);
                    Ok(())
                },
                |_| {},
            )
        }
        DwgEntityBody::Ellipse(ellipse) => {
            let center = logical_point3(&ellipse.center, "ELLIPSE center")?;
            let major_axis = logical_point3(&ellipse.major_axis, "ELLIPSE major axis")?;
            let extrusion = logical_point3(&ellipse.extrusion, "ELLIPSE extrusion")?;
            if !finite(&[ellipse.axis_ratio, ellipse.start_parameter, ellipse.end_parameter]) || ellipse.axis_ratio <= 0.0 || ellipse.axis_ratio > 1.0 {
                return Err("ELLIPSE axis ratio or parameters are invalid".into());
            }
            encode_r2010_entity_frame(
                object,
                &ellipse.common,
                |data, _| {
                    data.write_3bd(center);
                    data.write_3bd(major_axis);
                    data.write_3bd(extrusion);
                    data.write_bd(ellipse.axis_ratio);
                    data.write_bd(ellipse.start_parameter);
                    data.write_bd(ellipse.end_parameter);
                    Ok(())
                },
                |_| {},
            )
        }
        DwgEntityBody::Text(text) => {
            let insertion = logical_point2(&text.insertion, "TEXT insertion")?;
            let alignment = text.alignment.as_deref().map(|value| logical_point2(value, "TEXT alignment")).transpose()?;
            let extrusion = logical_point3(&text.extrusion, "TEXT extrusion")?;
            if !finite(&[text.elevation, text.thickness, text.oblique_angle, text.rotation, text.height, text.width_factor]) || text.style_handle == 0 {
                return Err("TEXT scalars or style are invalid".into());
            }
            let flags = u8::from(text.elevation == 0.0)
                | (u8::from(alignment.is_none()) << 1)
                | (u8::from(text.oblique_angle == 0.0) << 2)
                | (u8::from(text.rotation == 0.0) << 3)
                | (u8::from(text.width_factor == 1.0) << 4)
                | (u8::from(text.generation == 0) << 5)
                | (u8::from(text.horizontal_alignment == 0) << 6)
                | (u8::from(text.vertical_alignment == 0) << 7);
            encode_r2010_entity_frame(
                object,
                &text.common,
                |data, strings| {
                    data.write_rc(flags);
                    if flags & 0x01 == 0 {
                        data.write_rd(text.elevation);
                    }
                    data.write_2rd(insertion);
                    if let Some(alignment) = alignment {
                        data.write_dd(alignment[0], insertion[0]);
                        data.write_dd(alignment[1], insertion[1]);
                    }
                    data.write_be(extrusion);
                    data.write_bt(text.thickness);
                    if flags & 0x04 == 0 {
                        data.write_rd(text.oblique_angle);
                    }
                    if flags & 0x08 == 0 {
                        data.write_rd(text.rotation);
                    }
                    data.write_rd(text.height);
                    if flags & 0x10 == 0 {
                        data.write_rd(text.width_factor);
                    }
                    strings.write_tu(&text.value);
                    for (bit, value) in [(0x20, text.generation), (0x40, text.horizontal_alignment), (0x80, text.vertical_alignment)] {
                        if flags & bit == 0 {
                            data.write_bs(value);
                        }
                    }
                    Ok(())
                },
                |handles| handles.write_handle(5, text.style_handle),
            )
        }
        DwgEntityBody::Spline(spline) => {
            let point_count = spline.control_points.len() / 3;
            if spline.control_points.len() % 3 != 0 || point_count == 0 || !(spline.weights.is_empty() || spline.weights.len() == point_count) || !finite(&spline.control_points) || !finite(&spline.knots) || !finite(&spline.weights) {
                return Err(format!("SPLINE {:#x} control points, knots or weights are invalid", object.handle));
            }
            encode_r2010_entity_frame(
                object,
                &spline.common,
                |data, _| {
                    data.write_bl(1);
                    data.write_bl(spline.degree);
                    data.write_b(spline.rational);
                    data.write_b(spline.closed);
                    data.write_b(spline.periodic);
                    data.write_bd(spline.knot_tolerance);
                    data.write_bd(spline.control_tolerance);
                    data.write_bl(spline.knots.len() as u32);
                    data.write_bl(point_count as u32);
                    data.write_b(!spline.weights.is_empty());
                    for knot in &spline.knots {
                        data.write_bd(*knot);
                    }
                    for (index, point) in spline.control_points.as_chunks::<3>().0.iter().enumerate() {
                        data.write_3bd(*point);
                        if let Some(weight) = spline.weights.get(index) {
                            data.write_bd(*weight);
                        }
                    }
                    Ok(())
                },
                |_| {},
            )
        }
        DwgEntityBody::Face3d(face) => {
            if face.corners.len() != 12 || !finite(&face.corners) {
                return Err("3DFACE needs four finite corners".into());
            }
            let corners = face.corners.as_chunks::<3>().0;
            let z_is_zero = corners.iter().all(|corner| corner[2] == 0.0);
            encode_r2010_entity_frame(
                object,
                &face.common,
                |data, _| {
                    data.write_b(face.invisible_edges == 0);
                    data.write_b(z_is_zero);
                    data.write_rd(corners[0][0]);
                    data.write_rd(corners[0][1]);
                    if !z_is_zero {
                        data.write_rd(corners[0][2]);
                    }
                    for index in 1..4 {
                        data.write_dd(corners[index][0], corners[index - 1][0]);
                        data.write_dd(corners[index][1], corners[index - 1][1]);
                        if !z_is_zero {
                            data.write_dd(corners[index][2], corners[index - 1][2]);
                        }
                    }
                    if face.invisible_edges != 0 {
                        data.write_bs(face.invisible_edges);
                    }
                    Ok(())
                },
                |_| {},
            )
        }
        DwgEntityBody::Polyline3d(polyline) => encode_r2010_entity_frame(
            object,
            &polyline.common,
            |data, _| {
                data.write_rc(polyline.curve_type);
                data.write_rc(polyline.flags);
                data.write_bl(polyline.vertex_handles.len() as u32);
                Ok(())
            },
            |handles| {
                for vertex in &polyline.vertex_handles {
                    handles.write_handle(3, *vertex);
                }
                handles.write_handle(3, polyline.sequence_end_handle);
            },
        ),
        DwgEntityBody::PolyfaceMesh(mesh) => encode_r2010_entity_frame(
            object,
            &mesh.common,
            |data, _| {
                data.write_bs(mesh.vertex_count);
                data.write_bs(mesh.face_count);
                data.write_bl(mesh.vertex_handles.len() as u32);
                Ok(())
            },
            |handles| {
                for vertex in &mesh.vertex_handles {
                    handles.write_handle(3, *vertex);
                }
                handles.write_handle(3, mesh.sequence_end_handle);
            },
        ),
        DwgEntityBody::Vertex(vertex) => {
            let point = logical_point3(&vertex.point, "VERTEX point")?;
            encode_r2010_entity_frame(
                object,
                &vertex.common,
                |data, _| {
                    data.write_rc(vertex.flags);
                    data.write_3bd(point);
                    Ok(())
                },
                |_| {},
            )
        }
        DwgEntityBody::PolyfaceFace(face) => {
            if face.indices.len() != 4 {
                return Err("VERTEX_PFACE_FACE needs four vertex indices".into());
            }
            encode_r2010_entity_frame(
                object,
                &face.common,
                |data, _| {
                    for index in &face.indices {
                        data.write_bs(*index as u16);
                    }
                    Ok(())
                },
                |_| {},
            )
        }
        DwgEntityBody::SequenceEnd(sequence_end) => encode_r2010_entity_frame(object, &sequence_end.common, |_, _| Ok(()), |_| {}),
        _ => Err(format!("{name} {:#x} is encoded by its own frame writer", object.handle)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn append_r2010_string_stream(data: &mut DwgBitWriter, strings: &DwgBitWriter, class_name: &str, handle: u64) -> Result<(), String> {
    let string_bits = strings.bit_len();
    if string_bits == 0 {
        data.write_b(false);
    } else {
        if string_bits > 0x3fff_ffff {
            return Err(format!("{class_name} {handle:#x} string stream size exceeds R2010 range"));
        }
        data.append_bits(strings);
        if string_bits <= 0x7fff {
            data.write_rs(string_bits as u16);
        } else {
            data.write_rs((string_bits >> 15) as u16);
            data.write_rs(((string_bits & 0x7fff) as u16) | 0x8000);
        }
        data.write_b(true);
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_visual_style_operation(data: &mut DwgBitWriter, operation: semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgVisualStylePropertyOperation) {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgVisualStylePropertyOperation;
    data.write_bs(match operation {
        DwgVisualStylePropertyOperation::Inherit => 0,
        DwgVisualStylePropertyOperation::Set => 1,
        DwgVisualStylePropertyOperation::Disable => 2,
        DwgVisualStylePropertyOperation::Enable => 3,
    });
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_visual_style_color(data: &mut DwgBitWriter, strings: &mut DwgBitWriter, property: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgVisualStyleProperty<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgComplexColor>) {
    data.write_bs(property.value.index);
    data.write_bl(encode_complex_color_value(&property.value.value));
    let flags = u8::from(property.value.name.is_some()) | (u8::from(property.value.book_name.is_some()) << 1);
    data.write_rc(flags);
    if let Some(name) = &property.value.name {
        strings.write_tu(name);
    }
    if let Some(book) = &property.value.book_name {
        strings.write_tu(book);
    }
    write_visual_style_operation(data, property.operation);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_visual_style_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
    let Some(DwgLogicalObjectBody::VisualStyle(style)) = object.body.as_ref() else { return Err(format!("VISUALSTYLE {:#x} body missing", object.handle)) };
    if style.style_type > 22 || matches!(style.style_type, 10 | 17..=19) || style.extension_lighting_model > 3 {
        return Err(format!("VISUALSTYLE {:#x} style or lighting type is invalid", object.handle));
    }
    let p = &style.properties;
    if p.face_lighting_model.value > 3
        || p.face_lighting_quality.value > 3
        || p.face_color_mode.value > 6
        || p.face_modifiers.value & !3 != 0
        || !p.face_opacity.value.is_finite()
        || !(0.0..=1.0).contains(&p.face_opacity.value)
        || !p.face_specular_amount.value.is_finite()
        || p.edge_model.value > 2
        || p.edge_styles.value & !15 != 0
        || !(1..=11).contains(&p.edge_obscured_line_pattern.value)
        || !(1..=11).contains(&p.edge_intersection_line_pattern.value)
        || !p.edge_crease_angle.value.is_finite()
        || !(-360.0..=360.0).contains(&p.edge_crease_angle.value)
        || p.edge_modifiers.value & !(1 | 2 | 4 | 8 | 16 | 64 | 128) != 0
        || !p.edge_opacity.value.is_finite()
        || !(0.0..=1.0).contains(&p.edge_opacity.value)
        || p.edge_isolines.value > 5000
        || p.display_settings.value & !15 != 0
        || !p.display_brightness.value.is_finite()
        || p.display_shadow_type.value > 3
    {
        return Err(format!("VISUALSTYLE {:#x} contains an invalid property value", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(object.reactor_handles.len() as u32);
    data.write_b(object.extension_dictionary_handle.is_none());
    data.write_bl(style.style_type);
    data.write_bs(style.extension_lighting_model);
    data.write_b(style.internal_only);
    let mut strings = DwgBitWriter::new();
    strings.write_tu(&style.description);
    macro_rules! bl {
        ($field:ident) => {{
            data.write_bl(p.$field.value);
            write_visual_style_operation(&mut data, p.$field.operation);
        }};
    }
    macro_rules! bs {
        ($field:ident) => {{
            data.write_bs(p.$field.value);
            write_visual_style_operation(&mut data, p.$field.operation);
        }};
    }
    macro_rules! bd {
        ($field:ident) => {{
            data.write_bd(p.$field.value);
            write_visual_style_operation(&mut data, p.$field.operation);
        }};
    }
    bl!(face_lighting_model);
    bl!(face_lighting_quality);
    bl!(face_color_mode);
    bs!(face_modifiers);
    bd!(face_opacity);
    bd!(face_specular_amount);
    write_visual_style_color(&mut data, &mut strings, &p.face_monochrome_color);
    bl!(edge_model);
    bl!(edge_styles);
    write_visual_style_color(&mut data, &mut strings, &p.edge_intersection_color);
    write_visual_style_color(&mut data, &mut strings, &p.edge_obscured_color);
    bl!(edge_obscured_line_pattern);
    bl!(edge_intersection_line_pattern);
    bd!(edge_crease_angle);
    bl!(edge_modifiers);
    write_visual_style_color(&mut data, &mut strings, &p.edge_color);
    bd!(edge_opacity);
    bl!(edge_width);
    bl!(edge_overhang);
    bl!(edge_jitter);
    write_visual_style_color(&mut data, &mut strings, &p.edge_silhouette_color);
    bl!(edge_silhouette_width);
    bl!(edge_halo_gap);
    bl!(edge_isolines);
    data.write_b(p.hidden_edge_precision.value);
    write_visual_style_operation(&mut data, p.hidden_edge_precision.operation);
    bl!(display_settings);
    bd!(display_brightness);
    bl!(display_shadow_type);
    append_r2010_string_stream(&mut data, &strings, "VISUALSTYLE", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    for reactor in &object.reactor_handles {
        handles.write_handle(4, *reactor);
    }
    if let Some(xdic) = object.extension_dictionary_handle {
        handles.write_handle(4, xdic);
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_associative_dependency_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::AssociativeDependency(dependency)) = object.body.as_ref() else {
        return Err(format!("ACDBASSOCDEPENDENCY {:#x} body missing", object.handle));
    };
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(object.reactor_handles.len() as u32);
    data.write_b(object.extension_dictionary_handle.is_none());
    data.write_bs(1);
    data.write_bl(0);
    data.write_b(dependency.is_read_dependency);
    data.write_b(dependency.is_write_dependency);
    data.write_b(dependency.is_attached_to_object);
    data.write_b(dependency.is_delegating_to_owning_action);
    data.write_bl(dependency.order as u32);
    data.write_b(dependency.name.is_some());
    data.write_bl(dependency.dependency_body_id as u32);
    let mut strings = DwgBitWriter::new();
    if let Some(name) = &dependency.name {
        strings.write_tu(name);
    }
    append_r2010_string_stream(&mut data, &strings, "ACDBASSOCDEPENDENCY", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    for reactor in &object.reactor_handles {
        handles.write_handle(4, *reactor);
    }
    if let Some(extension_dictionary) = object.extension_dictionary_handle {
        handles.write_handle(4, extension_dictionary);
    }
    handles.write_handle(4, dependency.dependent_on_object_handle);
    handles.write_handle(4, dependency.read_dependency_handle.unwrap_or_default());
    handles.write_handle(4, dependency.dependency_node_handle.unwrap_or_default());
    handles.write_handle(3, dependency.dependency_body_handle.unwrap_or_default());
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_associative_value_dependency_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::AssociativeValueDependency(value)) = object.body.as_ref() else {
        return Err(format!("ACDBASSOCVALUEDEPENDENCY {:#x} body missing", object.handle));
    };
    let dependency = &value.dependency;
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(object.reactor_handles.len() as u32);
    data.write_b(object.extension_dictionary_handle.is_none());
    data.write_bs(1);
    data.write_bl(0);
    data.write_b(dependency.is_read_dependency);
    data.write_b(dependency.is_write_dependency);
    data.write_b(dependency.is_attached_to_object);
    data.write_b(dependency.is_delegating_to_owning_action);
    data.write_bl(dependency.order as u32);
    data.write_b(dependency.name.is_some());
    data.write_bl(dependency.dependency_body_id as u32);
    data.write_bs(0);
    match value.cached_value {
        semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEvaluationVariant::Integer32(cached) => {
            data.write_bs(90);
            data.write_bl(cached as u32);
        }
    }
    let mut strings = DwgBitWriter::new();
    if let Some(name) = &dependency.name {
        strings.write_tu(name);
    }
    strings.write_tu(&value.value_name);
    append_r2010_string_stream(&mut data, &strings, "ACDBASSOCVALUEDEPENDENCY", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    for reactor in &object.reactor_handles {
        handles.write_handle(4, *reactor);
    }
    if let Some(extension_dictionary) = object.extension_dictionary_handle {
        handles.write_handle(4, extension_dictionary);
    }
    handles.write_handle(4, dependency.dependent_on_object_handle);
    handles.write_handle(4, dependency.read_dependency_handle.unwrap_or_default());
    handles.write_handle(4, dependency.dependency_node_handle.unwrap_or_default());
    handles.write_handle(3, dependency.dependency_body_handle.unwrap_or_default());
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_associative_geometry_dependency_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::AssociativeGeometryDependency(geometry)) = object.body.as_ref() else {
        return Err(format!("ACDBASSOCGEOMDEPENDENCY {:#x} body missing", object.handle));
    };
    let dependency = &geometry.dependency;
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(object.reactor_handles.len() as u32);
    data.write_b(object.extension_dictionary_handle.is_none());
    data.write_bs(1);
    data.write_bl(0);
    data.write_b(dependency.is_read_dependency);
    data.write_b(dependency.is_write_dependency);
    data.write_b(dependency.is_attached_to_object);
    data.write_b(dependency.is_delegating_to_owning_action);
    data.write_bl(dependency.order as u32);
    data.write_b(dependency.name.is_some());
    data.write_bl(dependency.dependency_body_id as u32);
    data.write_bs(0);
    data.write_b(geometry.enabled);
    data.write_b(geometry.dependent_on_compound_object);
    let mut strings = DwgBitWriter::new();
    if let Some(name) = &dependency.name {
        strings.write_tu(name);
    }
    strings.write_tu(&geometry.persistent_subentity_class_name);
    append_r2010_string_stream(&mut data, &strings, "ACDBASSOCGEOMDEPENDENCY", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    for reactor in &object.reactor_handles {
        handles.write_handle(4, *reactor);
    }
    if let Some(extension_dictionary) = object.extension_dictionary_handle {
        handles.write_handle(4, extension_dictionary);
    }
    handles.write_handle(4, dependency.dependent_on_object_handle);
    handles.write_handle(4, dependency.read_dependency_handle.unwrap_or_default());
    handles.write_handle(4, dependency.dependency_node_handle.unwrap_or_default());
    handles.write_handle(3, dependency.dependency_body_handle.unwrap_or_default());
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_block_grip_location_component_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEvaluationExpressionValue;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
    let Some(DwgLogicalObjectBody::BlockGripLocationComponent(grip)) = object.body.as_ref() else {
        return Err(format!("BLOCKGRIPLOCATIONCOMPONENT {:#x} body missing", object.handle));
    };
    let expression = &grip.evaluation_expression;
    if matches!(&expression.value, DwgEvaluationExpressionValue::PointGroup10(point) | DwgEvaluationExpressionValue::PointGroup11(point) if point.len() != 2) {
        return Err(format!("BLOCKGRIPLOCATIONCOMPONENT {:#x} point value must contain two coordinates", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(object.reactor_handles.len() as u32);
    data.write_b(object.extension_dictionary_handle.is_none());
    data.write_bl(expression.parent_id as u32);
    data.write_bl(expression.major_version);
    data.write_bl(expression.minor_version);
    match &expression.value {
        DwgEvaluationExpressionValue::Empty => data.write_bs((-9999i16) as u16),
        DwgEvaluationExpressionValue::Double(value) => {
            data.write_bs(40);
            data.write_bd(*value);
        }
        DwgEvaluationExpressionValue::PointGroup10(point) => {
            data.write_bs(10);
            data.write_2rd([point[0], point[1]]);
        }
        DwgEvaluationExpressionValue::PointGroup11(point) => {
            data.write_bs(11);
            data.write_2rd([point[0], point[1]]);
        }
        DwgEvaluationExpressionValue::String(_) => data.write_bs(1),
        DwgEvaluationExpressionValue::Integer32(value) => {
            data.write_bs(90);
            data.write_bl(*value as u32);
        }
        DwgEvaluationExpressionValue::ObjectReference(_) => data.write_bs(91),
        DwgEvaluationExpressionValue::Integer16(value) => {
            data.write_bs(70);
            data.write_bs(*value as u16);
        }
    }
    data.write_bl(expression.node_id);
    data.write_bl(grip.grip_type);
    let mut strings = DwgBitWriter::new();
    if let DwgEvaluationExpressionValue::String(value) = &expression.value {
        strings.write_tu(value);
    }
    strings.write_tu(&grip.grip_expression);
    append_r2010_string_stream(&mut data, &strings, "BLOCKGRIPLOCATIONCOMPONENT", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    for reactor in &object.reactor_handles {
        handles.write_handle(4, *reactor);
    }
    if let Some(extension_dictionary) = object.extension_dictionary_handle {
        handles.write_handle(4, extension_dictionary);
    }
    if let DwgEvaluationExpressionValue::ObjectReference(reference) = &expression.value {
        handles.write_handle(5, *reference);
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_dynamic_block_proxy_node_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEvaluationExpressionValue;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
    let Some(DwgLogicalObjectBody::DynamicBlockProxyNode(proxy)) = object.body.as_ref() else {
        return Err(format!("ACDB_DYNAMICBLOCKPROXYNODE {:#x} body missing", object.handle));
    };
    let expression = &proxy.evaluation_expression;
    if matches!(&expression.value, DwgEvaluationExpressionValue::PointGroup10(point) | DwgEvaluationExpressionValue::PointGroup11(point) if point.len() != 2 || point.iter().any(|value| !value.is_finite())) {
        return Err(format!("ACDB_DYNAMICBLOCKPROXYNODE {:#x} point value must contain two finite coordinates", object.handle));
    }
    if matches!(&expression.value, DwgEvaluationExpressionValue::ObjectReference(0)) {
        return Err(format!("ACDB_DYNAMICBLOCKPROXYNODE {:#x} object reference is null", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(object.reactor_handles.len() as u32);
    data.write_b(object.extension_dictionary_handle.is_none());
    data.write_bl(expression.parent_id as u32);
    data.write_bl(expression.major_version);
    data.write_bl(expression.minor_version);
    match &expression.value {
        DwgEvaluationExpressionValue::Empty => data.write_bs((-9999i16) as u16),
        DwgEvaluationExpressionValue::Double(value) => {
            data.write_bs(40);
            data.write_bd(*value);
        }
        DwgEvaluationExpressionValue::PointGroup10(point) => {
            data.write_bs(10);
            data.write_2rd([point[0], point[1]]);
        }
        DwgEvaluationExpressionValue::PointGroup11(point) => {
            data.write_bs(11);
            data.write_2rd([point[0], point[1]]);
        }
        DwgEvaluationExpressionValue::String(_) => data.write_bs(1),
        DwgEvaluationExpressionValue::Integer32(value) => {
            data.write_bs(90);
            data.write_bl(*value as u32);
        }
        DwgEvaluationExpressionValue::ObjectReference(_) => data.write_bs(91),
        DwgEvaluationExpressionValue::Integer16(value) => {
            data.write_bs(70);
            data.write_bs(*value as u16);
        }
    }
    data.write_bl(expression.node_id);
    let mut strings = DwgBitWriter::new();
    if let DwgEvaluationExpressionValue::String(value) = &expression.value {
        strings.write_tu(value);
    }
    append_r2010_string_stream(&mut data, &strings, "ACDB_DYNAMICBLOCKPROXYNODE", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    for reactor in &object.reactor_handles {
        handles.write_handle(4, *reactor);
    }
    if let Some(extension_dictionary) = object.extension_dictionary_handle {
        handles.write_handle(4, extension_dictionary);
    }
    if let DwgEvaluationExpressionValue::ObjectReference(reference) = &expression.value {
        handles.write_handle(5, *reference);
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_associative_variable_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEvaluationVariant;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
    let Some(DwgLogicalObjectBody::AssociativeVariable(variable)) = object.body.as_ref() else {
        return Err(format!("ACDBASSOCVARIABLE {:#x} body missing", object.handle));
    };
    if variable.action.maximum_dependency_index < 0 || variable.action.maximum_dependency_index as usize != variable.referenced_value_dependency_handles.len() {
        return Err(format!("ACDBASSOCVARIABLE {:#x} maximum dependency index must equal the referenced value-dependency count", object.handle));
    }
    if variable.mergeable != variable.mergeable_variable_name.is_some() {
        return Err(format!("ACDBASSOCVARIABLE {:#x} mergeable state and variable name disagree", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(object.reactor_handles.len() as u32);
    data.write_b(object.extension_dictionary_handle.is_none());
    data.write_bs(1);
    data.write_bl(0);
    data.write_bl(variable.action.action_index as u32);
    data.write_bl(variable.action.maximum_dependency_index as u32);
    data.write_bl(variable.action.dependencies.len() as u32);
    for dependency in &variable.action.dependencies {
        data.write_b(dependency.owned);
    }
    data.write_bl(2);
    match variable.evaluated_value {
        DwgEvaluationVariant::Integer32(value) => {
            data.write_bs(90);
            data.write_bl(value as u32);
        }
    }
    data.write_b(variable.mergeable);
    data.write_b(variable.must_merge);
    if variable.action.maximum_dependency_index > 0 {
        data.write_bl(variable.referenced_value_dependency_handles.len() as u32);
    }
    data.write_bs(0);
    let mut strings = DwgBitWriter::new();
    strings.write_tu(&variable.name);
    strings.write_tu(&variable.expression);
    strings.write_tu(&variable.evaluator_id);
    strings.write_tu(&variable.description);
    if let Some(name) = &variable.mergeable_variable_name {
        strings.write_tu(name);
    }
    append_r2010_string_stream(&mut data, &strings, "ACDBASSOCVARIABLE", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    for reactor in &object.reactor_handles {
        handles.write_handle(4, *reactor);
    }
    if let Some(extension_dictionary) = object.extension_dictionary_handle {
        handles.write_handle(4, extension_dictionary);
    }
    handles.write_handle(4, variable.action.owning_network_handle.unwrap_or_default());
    handles.write_handle(3, variable.action.action_body_handle.unwrap_or_default());
    for dependency in &variable.action.dependencies {
        handles.write_handle(if dependency.owned { 3 } else { 5 }, dependency.dependency_handle);
    }
    for dependency in &variable.referenced_value_dependency_handles {
        handles.write_handle(3, *dependency);
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_associative_dimension_dependency_body_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::AssociativeDimensionDependencyBody(body)) = object.body.as_ref() else {
        return Err(format!("ASSOCDIMDEPENDENCYBODY {:#x} body missing", object.handle));
    };
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(object.reactor_handles.len() as u32);
    data.write_b(object.extension_dictionary_handle.is_none());
    data.write_bs(1);
    data.write_bs(1);
    data.write_bs(1);
    let mut strings = DwgBitWriter::new();
    strings.write_tu(&body.name);
    append_r2010_string_stream(&mut data, &strings, "ASSOCDIMDEPENDENCYBODY", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    for reactor in &object.reactor_handles {
        handles.write_handle(4, *reactor);
    }
    if let Some(extension_dictionary) = object.extension_dictionary_handle {
        handles.write_handle(4, extension_dictionary);
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_block_parameter_dependency_body_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::BlockParameterDependencyBody(body)) = object.body.as_ref() else {
        return Err(format!("BLOCKPARAMDEPENDENCYBODY {:#x} body missing", object.handle));
    };
    if !object.extended_data.is_empty() || !object.reactor_handles.is_empty() || object.extension_dictionary_handle.is_some() {
        return Err(format!("BLOCKPARAMDEPENDENCYBODY {:#x} unsupported common state", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(0);
    data.write_b(true);
    data.write_bs(1);
    data.write_bs(1);
    data.write_bs(0);
    let mut strings = DwgBitWriter::new();
    strings.write_tu(&body.name);
    append_r2010_string_stream(&mut data, &strings, "BLOCKPARAMDEPENDENCYBODY", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_block_representation_data_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::BlockRepresentationData(body)) = object.body.as_ref() else {
        return Err(format!("ACDB_BLOCKREPRESENTATION_DATA {:#x} body missing", object.handle));
    };
    if !object.extended_data.is_empty() || object.extension_dictionary_handle.is_some() || object.owner_handle != Some(object.handle - 1) || object.reactor_handles.as_slice() != [object.handle - 1] || body.represented_block_header_handle == 0 {
        return Err(format!("ACDB_BLOCKREPRESENTATION_DATA {:#x} common or graph state is invalid", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(1);
    data.write_b(true);
    data.write_bs(1);
    data.write_b(false);
    let mut handles = DwgBitWriter::new();
    handles.write_handle(8, 0);
    handles.write_handle(4, object.reactor_handles[0]);
    handles.write_handle(5, body.represented_block_header_handle);
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_dynamic_block_purge_preventer_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::DynamicBlockPurgePreventer(body)) = object.body.as_ref() else {
        return Err(format!("ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION {:#x} body missing", object.handle));
    };
    if !object.extended_data.is_empty() || object.extension_dictionary_handle.is_some() || object.owner_handle.is_none() || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()] || body.protected_block_header_handle == 0 {
        return Err(format!("ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION {:#x} common or graph state is invalid", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(1);
    data.write_b(true);
    data.write_bs(1);
    data.write_b(false);
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    handles.write_handle(4, object.reactor_handles[0]);
    handles.write_handle(5, body.protected_block_header_handle);
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_evaluation_graph_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::EvaluationGraph(graph)) = object.body.as_ref() else {
        return Err(format!("ACAD_EVALUATION_GRAPH {:#x} body missing", object.handle));
    };
    if !object.extended_data.is_empty() || object.extension_dictionary_handle.is_some() || object.owner_handle.is_none() || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()] || graph.nodes.is_empty() {
        return Err(format!("ACAD_EVALUATION_GRAPH {:#x} common or graph state is invalid", object.handle));
    }
    let (node_relations, edge_relations, endpoints) = evaluation_graph_indexes(graph)?;
    let watermark = graph.nodes.iter().map(|node| node.id).max().ok_or("evaluation graph has no nodes")?;
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(1);
    data.write_b(true);
    data.write_bl(watermark);
    data.write_bl(watermark);
    data.write_bl(graph.nodes.len() as u32);
    for (index, node) in graph.nodes.iter().enumerate() {
        data.write_bl(index as u32);
        data.write_bl(32);
        data.write_bl(node.id);
        for relation in node_relations[index] {
            data.write_bl(relation as u32);
        }
    }
    data.write_bl(graph.edges.len() as u32);
    for (index, edge) in graph.edges.iter().enumerate() {
        data.write_bl(index as u32);
        data.write_bl(0);
        data.write_bl(edge.reference_count);
        data.write_bl(endpoints[index].0 as u32);
        data.write_bl(endpoints[index].1 as u32);
        for relation in edge_relations[index] {
            data.write_bl(relation as u32);
        }
    }
    data.write_b(false);
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    handles.write_handle(4, object.reactor_handles[0]);
    for node in &graph.nodes {
        handles.write_handle(3, node.expression_handle);
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_block_flip_parameter_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockParameterBaseLocation;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEvaluationExpressionValue;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
    let Some(DwgLogicalObjectBody::BlockFlipParameter(body)) = object.body.as_ref() else {
        return Err(format!("BLOCKFLIPPARAMETER {:#x} body missing", object.handle));
    };
    if body.properties.len() != 4
        || body.definition_base.len() != 3
        || body.definition_end.len() != 3
        || body.label_point.len() != 3
        || object.owner_handle.is_none()
        || !object.reactor_handles.is_empty()
        || object.extension_dictionary_handle.is_some()
    {
        return Err(format!("BLOCKFLIPPARAMETER {:#x} logical state is invalid", object.handle));
    }
    let expression = &body.evaluation_expression;
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(0);
    data.write_b(true);
    data.write_bl(expression.parent_id as u32);
    data.write_bl(expression.major_version);
    data.write_bl(expression.minor_version);
    match &expression.value {
        DwgEvaluationExpressionValue::Empty => data.write_bs((-9999i16) as u16),
        DwgEvaluationExpressionValue::Double(value) => {
            data.write_bs(40);
            data.write_bd(*value);
        }
        DwgEvaluationExpressionValue::PointGroup10(point) | DwgEvaluationExpressionValue::PointGroup11(point) if point.len() == 2 => {
            data.write_bs(if matches!(expression.value, DwgEvaluationExpressionValue::PointGroup10(_)) { 10 } else { 11 });
            data.write_2rd([point[0], point[1]]);
        }
        DwgEvaluationExpressionValue::String(_) => data.write_bs(1),
        DwgEvaluationExpressionValue::Integer32(value) => {
            data.write_bs(90);
            data.write_bl(*value as u32);
        }
        DwgEvaluationExpressionValue::ObjectReference(reference) if *reference != 0 => data.write_bs(91),
        DwgEvaluationExpressionValue::Integer16(value) => {
            data.write_bs(70);
            data.write_bs(*value as u16);
        }
        _ => return Err(format!("BLOCKFLIPPARAMETER {:#x} evaluation value is invalid", object.handle)),
    }
    data.write_bl(expression.node_id);
    data.write_bl(expression.major_version);
    data.write_bl(expression.minor_version);
    data.write_bl(0);
    data.write_b(body.show_properties);
    data.write_b(body.chain_actions);
    data.write_3bd([body.definition_base[0], body.definition_base[1], body.definition_base[2]]);
    data.write_3bd([body.definition_end[0], body.definition_end[1], body.definition_end[2]]);
    for property in &body.properties {
        data.write_bl(property.connections.len() as u32);
        for connection in &property.connections {
            data.write_bl(connection.code);
        }
    }
    data.write_bl(body.updated_flip.node_id);
    data.write_bl(0);
    data.write_bl(0);
    data.write_bl(0);
    data.write_bs(match body.base_location {
        DwgBlockParameterBaseLocation::StartPoint => 0,
        DwgBlockParameterBaseLocation::Midpoint => 1,
    });
    data.write_3bd([body.label_point[0], body.label_point[1], body.label_point[2]]);
    data.write_bl(body.updated_flip.node_id);
    let mut strings = DwgBitWriter::new();
    if let DwgEvaluationExpressionValue::String(value) = &expression.value {
        strings.write_tu(value);
    }
    strings.write_tu(&body.name);
    for property in &body.properties {
        for connection in &property.connections {
            strings.write_tu(&connection.name);
        }
    }
    strings.write_tu(&body.label);
    strings.write_tu(&body.description);
    strings.write_tu(&body.value_set.base_label);
    strings.write_tu(&body.value_set.flipped_label);
    strings.write_tu(&body.updated_flip.expression_name);
    append_r2010_string_stream(&mut data, &strings, "BLOCKFLIPPARAMETER", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    if let DwgEvaluationExpressionValue::ObjectReference(reference) = &expression.value {
        handles.write_handle(5, *reference);
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_block_visibility_parameter_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEvaluationExpressionValue;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgVisibilityEvaluationHistory;
    let Some(DwgLogicalObjectBody::BlockVisibilityParameter(body)) = object.body.as_ref() else {
        return Err(format!("BLOCKVISIBILITYPARAMETER {:#x} body missing", object.handle));
    };
    if body.definition_point.len() != 3
        || body.properties.len() != 2
        || body.eligible_entity_handles.contains(&0)
        || body.states.iter().any(|state| state.name.is_empty() || state.visible_entity_handles.iter().any(|handle| !body.eligible_entity_handles.contains(handle)))
        || object.owner_handle.is_none()
        || !object.reactor_handles.is_empty()
        || object.extension_dictionary_handle.is_some()
    {
        return Err(format!("BLOCKVISIBILITYPARAMETER {:#x} logical state is invalid", object.handle));
    }
    let expression = &body.evaluation_expression;
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(0);
    data.write_b(true);
    data.write_bl(expression.parent_id as u32);
    data.write_bl(expression.major_version);
    data.write_bl(expression.minor_version);
    match &expression.value {
        DwgEvaluationExpressionValue::Empty => data.write_bs((-9999i16) as u16),
        DwgEvaluationExpressionValue::Double(value) => {
            data.write_bs(40);
            data.write_bd(*value);
        }
        DwgEvaluationExpressionValue::PointGroup10(point) | DwgEvaluationExpressionValue::PointGroup11(point) if point.len() == 2 => {
            data.write_bs(if matches!(expression.value, DwgEvaluationExpressionValue::PointGroup10(_)) { 10 } else { 11 });
            data.write_2rd([point[0], point[1]]);
        }
        DwgEvaluationExpressionValue::String(_) => data.write_bs(1),
        DwgEvaluationExpressionValue::Integer32(value) => {
            data.write_bs(90);
            data.write_bl(*value as u32);
        }
        DwgEvaluationExpressionValue::ObjectReference(reference) if *reference != 0 => data.write_bs(91),
        DwgEvaluationExpressionValue::Integer16(value) => {
            data.write_bs(70);
            data.write_bs(*value as u16);
        }
        _ => return Err(format!("BLOCKVISIBILITYPARAMETER {:#x} evaluation value is invalid", object.handle)),
    }
    data.write_bl(expression.node_id);
    data.write_bl(expression.major_version);
    data.write_bl(expression.minor_version);
    data.write_bl(0);
    data.write_b(body.show_properties);
    data.write_b(body.chain_actions);
    data.write_3bd([body.definition_point[0], body.definition_point[1], body.definition_point[2]]);
    for property in &body.properties {
        data.write_bl(property.connections.len() as u32);
        for connection in &property.connections {
            data.write_bl(connection.code);
        }
    }
    data.write_bl(body.updated_visibility_node_id);
    data.write_b(body.initialized);
    data.write_b(matches!(body.evaluation_history, DwgVisibilityEvaluationHistory::Required));
    data.write_bl(body.eligible_entity_handles.len() as u32);
    data.write_bl(body.states.len() as u32);
    for state in &body.states {
        data.write_bl(state.visible_entity_handles.len() as u32);
        data.write_bl(state.controlled_expression_handles.len() as u32);
    }
    let mut strings = DwgBitWriter::new();
    if let DwgEvaluationExpressionValue::String(value) = &expression.value {
        strings.write_tu(value);
    }
    strings.write_tu(&body.element_name);
    for property in &body.properties {
        for connection in &property.connections {
            strings.write_tu(&connection.name);
        }
    }
    strings.write_tu(&body.name);
    strings.write_tu(&body.description);
    for state in &body.states {
        strings.write_tu(&state.name);
    }
    append_r2010_string_stream(&mut data, &strings, "BLOCKVISIBILITYPARAMETER", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    if let DwgEvaluationExpressionValue::ObjectReference(reference) = &expression.value {
        handles.write_handle(5, *reference);
    }
    for handle in &body.eligible_entity_handles {
        handles.write_handle(4, *handle);
    }
    for state in &body.states {
        for handle in &state.visible_entity_handles {
            handles.write_handle(4, *handle);
        }
        for handle in &state.controlled_expression_handles {
            handles.write_handle(4, *handle);
        }
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_placeholder_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
    if !matches!(object.body, Some(DwgLogicalObjectBody::Placeholder(_)))
        || !object.extended_data.is_empty()
        || object.owner_handle.is_none()
        || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()]
        || object.extension_dictionary_handle.is_some()
    {
        return Err(format!("ACDBPLACEHOLDER {:#x} logical state is invalid", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(1);
    data.write_b(true);
    data.write_b(false);
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    handles.write_handle(4, object.reactor_handles[0]);
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_dictionary_variable_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
    let Some(DwgLogicalObjectBody::DictionaryVariable(body)) = object.body.as_ref() else {
        return Err(format!("DICTIONARYVAR {:#x} body missing", object.handle));
    };
    if !object.extended_data.is_empty() || object.owner_handle.is_none() || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()] || object.extension_dictionary_handle.is_some() {
        return Err(format!("DICTIONARYVAR {:#x} common state is invalid", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(1);
    data.write_b(true);
    data.write_rc(0);
    let mut strings = DwgBitWriter::new();
    strings.write_tu(&body.value);
    append_r2010_string_stream(&mut data, &strings, "DICTIONARYVAR", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    handles.write_handle(4, object.reactor_handles[0]);
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_annotation_scale_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
    let Some(DwgLogicalObjectBody::AnnotationScale(body)) = object.body.as_ref() else {
        return Err(format!("SCALE {:#x} body missing", object.handle));
    };
    if body.name.is_empty()
        || !body.paper_units.is_finite()
        || body.paper_units <= 0.0
        || !body.drawing_units.is_finite()
        || body.drawing_units <= 0.0
        || body.is_unit_scale && (body.name != "1:1" || body.paper_units != 1.0 || body.drawing_units != 1.0)
        || !object.extended_data.is_empty()
        || object.owner_handle.is_none()
        || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()]
        || object.extension_dictionary_handle.is_some()
    {
        return Err(format!("SCALE {:#x} logical state is invalid", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(1);
    data.write_b(true);
    data.write_bs(0);
    data.write_bd(body.paper_units);
    data.write_bd(body.drawing_units);
    data.write_b(body.is_unit_scale);
    let mut strings = DwgBitWriter::new();
    strings.write_tu(&body.name);
    append_r2010_string_stream(&mut data, &strings, "SCALE", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    handles.write_handle(4, object.reactor_handles[0]);
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_sort_entities_table_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
    let Some(DwgLogicalObjectBody::SortEntitiesTable(body)) = object.body.as_ref() else {
        return Err(format!("SORTENTSTABLE {:#x} body missing", object.handle));
    };
    let entity_handles = body.entries.iter().map(|entry| entry.entity_handle).collect::<std::collections::BTreeSet<_>>();
    let sort_handles = body.entries.iter().map(|entry| entry.sort_handle).collect::<std::collections::BTreeSet<_>>();
    if body.block_header_handle == 0
        || body.entries.len() > 50_000
        || body.entries.iter().any(|entry| entry.entity_handle == 0)
        || entity_handles.len() != body.entries.len()
        || sort_handles.len() != body.entries.len()
        || !object.extended_data.is_empty()
        || object.owner_handle.is_none()
        || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()]
        || object.extension_dictionary_handle.is_some()
    {
        return Err(format!("SORTENTSTABLE {:#x} logical state is invalid", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(1);
    data.write_b(true);
    data.write_bl(body.entries.len() as u32);
    for entry in &body.entries {
        data.write_handle(0, entry.sort_handle);
    }
    data.write_b(false);
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    handles.write_handle(4, object.reactor_handles[0]);
    handles.write_handle(4, body.block_header_handle);
    for entry in &body.entries {
        handles.write_handle(4, entry.entity_handle);
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_table_style_color(data: &mut DwgBitWriter, color: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgComplexColor) -> Result<(), String> {
    if color.name.is_some() || color.book_name.is_some() {
        return Err("TABLESTYLE named colors are unsupported".into());
    }
    data.write_bs(color.index);
    data.write_bl(encode_complex_color_value(&color.value));
    data.write_rc(0);
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn table_style_borders(borders: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgCellBorders) -> [(u32, Option<&semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgCellBorder>); 6] {
    [(1, borders.top.as_ref()), (2, borders.horizontal_inside.as_ref()), (4, borders.bottom.as_ref()), (8, borders.left.as_ref()), (16, borders.vertical_inside.as_ref()), (32, borders.right.as_ref())]
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_cell_style(data: &mut DwgBitWriter, strings: &mut DwgBitWriter, handles: &mut DwgBitWriter, style: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgCellStyle) -> Result<(), String> {
    data.write_bl(5);
    data.write_bs(1);
    data.write_bl(style.property_override_flags);
    data.write_bl(style.merge_flags);
    write_table_style_color(data, &style.background_color)?;
    data.write_bl(style.content_layout);
    data.write_bl(style.content_format.property_override_flags);
    data.write_bl(style.content_format.property_flags);
    data.write_bl(style.content_format.value_data_type);
    data.write_bl(style.content_format.value_unit_type);
    strings.write_tu(&style.content_format.value_format_string);
    data.write_bd(style.content_format.rotation);
    data.write_bd(style.content_format.block_scale);
    data.write_bl(style.content_format.alignment);
    write_table_style_color(data, &style.content_format.content_color)?;
    handles.write_handle(5, style.content_format.text_style_handle.unwrap_or_default());
    data.write_bd(style.content_format.text_height);
    data.write_bs(1);
    data.write_bd(style.margins.vertical);
    data.write_bd(style.margins.horizontal);
    data.write_bd(style.margins.bottom);
    data.write_bd(style.margins.right);
    data.write_bd(style.margins.horizontal_spacing);
    data.write_bd(style.margins.vertical_spacing);
    let borders = table_style_borders(&style.borders);
    data.write_bl(borders.iter().filter(|(_, border)| border.is_some()).count() as u32);
    for (mask, border) in borders {
        let Some(border) = border else { continue };
        data.write_bl(mask);
        data.write_bl(border.override_flags);
        data.write_bl(border.border_type);
        write_table_style_color(data, &border.color)?;
        data.write_bl(border.lineweight as u32);
        handles.write_handle(5, border.linetype_handle.unwrap_or_default());
        data.write_bl(border.visible);
        data.write_bd(border.double_line_spacing);
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_table_style_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
    let Some(DwgLogicalObjectBody::TableStyle(body)) = object.body.as_ref() else {
        return Err(format!("TABLESTYLE {:#x} body missing", object.handle));
    };
    if object.owner_handle.is_none() || object.reactor_handles.len() != 1 || object.extension_dictionary_handle.is_none() || !object.extended_data.is_empty() {
        return Err(format!("TABLESTYLE {:#x} common state is invalid", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(1);
    data.write_b(false);
    data.write_rc(0);
    data.write_bl(0);
    data.write_bl(body.bit_flags);
    let mut strings = DwgBitWriter::new();
    strings.write_tu(&body.description);
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    handles.write_handle(4, object.reactor_handles[0]);
    handles.write_handle(3, object.extension_dictionary_handle.unwrap());
    handles.write_handle(3, body.template_style_handle.unwrap_or_default());
    for (style, id, cell_type, name) in [(&body.table, 4, 2, "Table"), (&body.title, 1, 1, "_TITLE"), (&body.header, 2, 1, "_HEADER"), (&body.data, 3, 2, "_DATA")] {
        if id != 4 {
            data.write_bl(id);
        }
        encode_r2010_cell_style(&mut data, &mut strings, &mut handles, style)?;
        data.write_bl(id);
        data.write_bl(cell_type);
        strings.write_tu(name);
        if id == 4 {
            data.write_bl(3);
        }
    }
    append_r2010_string_stream(&mut data, &strings, "TABLESTYLE", object.handle)?;
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_mline_style_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgMlineLinetype;
    let Some(DwgLogicalObjectBody::MlineStyle(body)) = object.body.as_ref() else { return Err(format!("MLINESTYLE {:#x} body missing", object.handle)) };
    if body.name.is_empty()
        || body.elements.is_empty()
        || body.elements.len() > u8::MAX as usize
        || body.elements.windows(2).any(|pair| pair[0].offset <= pair[1].offset)
        || object.owner_handle.is_none()
        || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()]
        || object.extension_dictionary_handle.is_some()
        || !object.extended_data.is_empty()
    {
        return Err(format!("MLINESTYLE {:#x} logical state is invalid", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(1);
    data.write_b(true);
    let mut flags = 0u16;
    flags |= u16::from(body.fill_enabled);
    flags |= u16::from(body.display_miters) << 1;
    flags |= u16::from(body.start_caps.square) << 4;
    flags |= u16::from(body.start_caps.inner_arcs) << 5;
    flags |= u16::from(body.start_caps.round_outer_arcs) << 6;
    flags |= u16::from(body.end_caps.square) << 8;
    flags |= u16::from(body.end_caps.inner_arcs) << 9;
    flags |= u16::from(body.end_caps.round_outer_arcs) << 10;
    data.write_bs(flags);
    write_table_style_color(&mut data, &body.fill_color)?;
    data.write_bd(body.start_angle);
    data.write_bd(body.end_angle);
    data.write_rc(body.elements.len() as u8);
    for element in &body.elements {
        data.write_bd(element.offset);
        write_table_style_color(&mut data, &element.color)?;
        data.write_bs(match element.linetype {
            DwgMlineLinetype::ByLayer => 32767,
            DwgMlineLinetype::ByBlock => 32766,
            DwgMlineLinetype::Continuous => 0,
        });
    }
    let mut strings = DwgBitWriter::new();
    strings.write_tu(&body.name);
    strings.write_tu(&body.description);
    append_r2010_string_stream(&mut data, &strings, "MLINESTYLE", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    handles.write_handle(4, object.reactor_handles[0]);
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_mleader_style_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::*;
    let Some(DwgLogicalObjectBody::MLeaderStyle(body)) = object.body.as_ref() else { return Err(format!("MLEADERSTYLE {:#x} body missing", object.handle)) };
    if body.leader.linetype_style_handle == 0
        || body.text.style_handle == 0
        || body.block.scale.len() != 3
        || body.block.scale.iter().any(|value| !value.is_finite())
        || object.owner_handle.is_none()
        || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()]
        || object.extension_dictionary_handle.is_some()
        || !object.extended_data.is_empty()
    {
        return Err(format!("MLEADERSTYLE {:#x} logical state is invalid", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(1);
    data.write_b(true);
    data.write_bs(2);
    data.write_bs(match body.content_type {
        DwgMLeaderContentType::None => 0,
        DwgMLeaderContentType::Block => 1,
        DwgMLeaderContentType::MText => 2,
    });
    data.write_bs(match body.draw_order {
        DwgMLeaderDrawOrder::LeaderFirst => 0,
        DwgMLeaderDrawOrder::ContentFirst => 1,
    });
    data.write_bs(match body.leader_order {
        DwgMLeaderLeaderOrder::HeadFirst => 0,
        DwgMLeaderLeaderOrder::TailFirst => 1,
    });
    data.write_bl(body.maximum_segment_points);
    data.write_bd(body.first_segment_angle);
    data.write_bd(body.second_segment_angle);
    data.write_bs(match body.leader.kind {
        DwgMLeaderKind::Invisible => 0,
        DwgMLeaderKind::Straight => 1,
        DwgMLeaderKind::Spline => 2,
    });
    write_table_style_color(&mut data, &body.leader.color)?;
    data.write_bl(body.leader.lineweight as u32);
    data.write_b(body.landing.enabled);
    data.write_bd(body.landing.gap);
    data.write_b(body.dogleg.enabled);
    data.write_bd(body.dogleg.length);
    data.write_bd(body.arrow.size);
    data.write_bs(body.text.left_attachment as u16);
    data.write_bs(body.text.right_attachment as u16);
    data.write_bs(body.text.angle as u16);
    data.write_bs(body.text.alignment as u16);
    write_table_style_color(&mut data, &body.text.color)?;
    data.write_bd(body.text.height);
    data.write_b(body.text.frame);
    data.write_b(body.text.always_left);
    data.write_bd(body.text.alignment_space);
    write_table_style_color(&mut data, &body.block.color)?;
    data.write_bd(body.block.scale[0]);
    data.write_bd(body.block.scale[1]);
    data.write_bd(body.block.scale[2]);
    data.write_b(body.block.use_scale);
    data.write_bd(body.block.rotation);
    data.write_b(body.block.use_rotation);
    data.write_bs(match body.block.connection {
        DwgMLeaderBlockConnection::Extents => 0,
        DwgMLeaderBlockConnection::BasePoint => 1,
    });
    data.write_bd(body.overall_scale);
    data.write_b(body.property_overrides_changed);
    data.write_b(body.annotative);
    data.write_bd(body.break_size);
    data.write_bs(match body.text.attachment_direction {
        DwgMLeaderAttachmentDirection::Horizontal => 0,
        DwgMLeaderAttachmentDirection::Vertical => 1,
    });
    data.write_bs(body.text.top_attachment as u16);
    data.write_bs(body.text.bottom_attachment as u16);
    let mut strings = DwgBitWriter::new();
    strings.write_tu(&body.description);
    strings.write_tu(&body.text.default_content);
    append_r2010_string_stream(&mut data, &strings, "MLEADERSTYLE", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    handles.write_handle(4, object.reactor_handles[0]);
    handles.write_handle(5, body.leader.linetype_style_handle);
    handles.write_handle(5, body.arrow.symbol_handle.unwrap_or_default());
    handles.write_handle(5, body.text.style_handle);
    handles.write_handle(5, body.block.content_handle.unwrap_or_default());
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_material_color(data: &mut DwgBitWriter, color: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgMaterialColor) {
    data.write_rc(u8::from(color.override_rgb.is_some()));
    data.write_bd(color.factor);
    if let Some(rgb) = color.override_rgb {
        data.write_bl(rgb);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_material_map(data: &mut DwgBitWriter, strings: &mut DwgBitWriter, map: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgMaterialMap) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgMaterialMapSource;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgMaterialProjection;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgMaterialTiling;
    if map.transform.len() != 16 || map.transform.iter().any(|value| !value.is_finite()) {
        return Err("MATERIAL mapper transform must contain sixteen finite values".into());
    }
    data.write_bd(map.blend_factor);
    data.write_rc(match map.projection {
        DwgMaterialProjection::Inherit => 0,
        DwgMaterialProjection::Planar => 1,
        DwgMaterialProjection::Box => 2,
        DwgMaterialProjection::Cylinder => 3,
        DwgMaterialProjection::Sphere => 4,
    });
    data.write_rc(match map.tiling {
        DwgMaterialTiling::Inherit => 0,
        DwgMaterialTiling::Tile => 1,
        DwgMaterialTiling::Crop => 2,
        DwgMaterialTiling::Clamp => 3,
        DwgMaterialTiling::Mirror => 4,
    });
    data.write_rc(if !map.scale_to_entity && !map.use_current_block_transform { 1 } else { u8::from(map.scale_to_entity) << 1 | u8::from(map.use_current_block_transform) << 2 });
    for value in &map.transform {
        data.write_bd(*value);
    }
    match map.source {
        DwgMaterialMapSource::None => {
            data.write_rc(1);
            strings.write_tu("");
        }
        DwgMaterialMapSource::CurrentScene => data.write_rc(0),
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_material_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
    let Some(DwgLogicalObjectBody::Material(body)) = object.body.as_ref() else { return Err(format!("MATERIAL {:#x} body missing", object.handle)) };
    if body.name.is_empty() || object.owner_handle.is_none() || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()] {
        return Err(format!("MATERIAL {:#x} logical state is invalid", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(1);
    data.write_b(object.extension_dictionary_handle.is_none());
    let mut strings = DwgBitWriter::new();
    strings.write_tu(&body.name);
    strings.write_tu(&body.description);
    write_material_color(&mut data, &body.ambient);
    write_material_color(&mut data, &body.diffuse);
    write_material_map(&mut data, &mut strings, &body.diffuse_map)?;
    write_material_color(&mut data, &body.specular);
    write_material_map(&mut data, &mut strings, &body.specular_map)?;
    data.write_bd(body.specular_gloss);
    write_material_map(&mut data, &mut strings, &body.reflection_map)?;
    data.write_bd(body.opacity);
    write_material_map(&mut data, &mut strings, &body.opacity_map)?;
    write_material_map(&mut data, &mut strings, &body.bump_map)?;
    data.write_bd(body.refraction_index);
    write_material_map(&mut data, &mut strings, &body.refraction_map)?;
    data.write_bd(body.translucence);
    data.write_bd(body.self_illumination);
    data.write_bd(body.reflectivity);
    data.write_bl(0);
    let channels = &body.enabled_channels;
    data.write_bl(u32::from(channels.diffuse) | u32::from(channels.specular) << 1 | u32::from(channels.reflection) << 2 | u32::from(channels.opacity) << 3 | u32::from(channels.bump) << 4 | u32::from(channels.refraction) << 5);
    data.write_bl(0);
    append_r2010_string_stream(&mut data, &strings, "MATERIAL", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    handles.write_handle(4, object.reactor_handles[0]);
    if let Some(handle) = object.extension_dictionary_handle {
        handles.write_handle(3, handle);
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_block_move_action_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockMoveCoordinateMode;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEvaluationExpressionValue;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
    let Some(DwgLogicalObjectBody::BlockMoveAction(body)) = object.body.as_ref() else { return Err(format!("BLOCKMOVEACTION {:#x} body missing", object.handle)) };
    let action = &body.action;
    let expression = &action.evaluation_expression;
    if !matches!(expression.value, DwgEvaluationExpressionValue::Empty)
        || action.display_location.len() != 3
        || action.display_location.iter().any(|value| !value.is_finite())
        || action.name.is_empty()
        || action.dependencies.len() != 2
        || action.action_node_ids.len() != 1
        || body.x_connection.name.is_empty()
        || body.y_connection.name.is_empty()
        || !body.distance_multiplier.is_finite()
        || body.distance_multiplier <= 0.0
        || !body.angle_offset.is_finite()
        || !matches!(body.coordinate_mode, DwgBlockMoveCoordinateMode::CartesianXy)
        || object.owner_handle.is_none()
        || !object.reactor_handles.is_empty()
        || object.extension_dictionary_handle.is_some()
    {
        return Err(format!("BLOCKMOVEACTION {:#x} logical state is invalid", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(0);
    data.write_b(true);
    data.write_bl(expression.parent_id as u32);
    data.write_bl(expression.major_version);
    data.write_bl(expression.minor_version);
    data.write_bs((-9999i16) as u16);
    data.write_bl(expression.node_id);
    data.write_bl(expression.major_version);
    data.write_bl(expression.minor_version);
    data.write_bl(0);
    data.write_3bd([action.display_location[0], action.display_location[1], action.display_location[2]]);
    data.write_bl(action.dependencies.len() as u32);
    data.write_bl(action.action_node_ids.len() as u32);
    for node_id in &action.action_node_ids {
        data.write_bl(*node_id);
    }
    data.write_bl(body.x_connection.node_id);
    data.write_bl(body.y_connection.node_id);
    data.write_bd(body.distance_multiplier);
    data.write_bd(body.angle_offset);
    data.write_rc(0);
    let mut strings = DwgBitWriter::new();
    strings.write_tu(&action.name);
    strings.write_tu(&body.x_connection.name);
    strings.write_tu(&body.y_connection.name);
    append_r2010_string_stream(&mut data, &strings, "BLOCKMOVEACTION", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    for dependency in &action.dependencies {
        handles.write_handle(4, dependency.object_handle);
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_assoc_network_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAssocNetworkMemberKind;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAssociativeActionStatus;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
    let Some(DwgLogicalObjectBody::AssocNetwork(network)) = object.body.as_ref() else { return Err(format!("ACDBASSOCNETWORK {:#x} body missing", object.handle)) };
    let action = &network.action;
    if !matches!(action.status, DwgAssociativeActionStatus::UpToDate)
        || action.maximum_dependency_index != 0
        || !action.dependencies.is_empty()
        || action.action_body_handle.is_some()
        || object.owner_handle.is_none()
        || object.reactor_handles.len() != 1
        || object.extension_dictionary_handle.is_some()
        || network.actions.iter().any(|member| member.handle == 0)
    {
        return Err(format!("ACDBASSOCNETWORK {:#x} logical state is invalid", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(1);
    data.write_b(true);
    data.write_bs(1);
    data.write_bl(0);
    data.write_bl(action.action_index as u32);
    data.write_bl(action.maximum_dependency_index as u32);
    data.write_bl(0);
    data.write_bs(0);
    data.write_bl(network.network_action_index as u32);
    data.write_bl(network.actions.len() as u32);
    for member in &network.actions {
        data.write_b(matches!(member.kind, DwgAssocNetworkMemberKind::Action));
    }
    data.write_bl(0);
    append_r2010_string_stream(&mut data, &DwgBitWriter::new(), "ACDBASSOCNETWORK", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    handles.write_handle(4, object.reactor_handles[0]);
    handles.write_handle(4, action.owning_network_handle.unwrap_or_default());
    handles.write_handle(3, 0);
    for member in &network.actions {
        handles.write_handle(if matches!(member.kind, DwgAssocNetworkMemberKind::Network) { 4 } else { 3 }, member.handle);
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn constraint_point3(values: &[f64], role: &str) -> Result<[f64; 3], String> {
    if values.len() != 3 || values.iter().any(|value| !value.is_finite()) {
        return Err(format!("{role} must contain three finite coordinates"));
    }
    Ok([values[0], values[1], values[2]])
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_constraint_node(node: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgConstraintNode, data: &mut DwgBitWriter, strings: &mut DwgBitWriter, handles: &mut DwgBitWriter) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgConstraintNode;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn core(data: &mut DwgBitWriter, node: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgConstraintNodeCore) -> Result<(), String> {
        if node.id < 0 || node.connected_node_ids.len() > 10_000 {
            return Err("constraint node ID or connection count is invalid".into());
        }
        data.write_bl(node.id as u32);
        data.write_bl(node.connected_node_ids.len() as u32);
        for connection in &node.connected_node_ids {
            data.write_bl(*connection);
        }
        Ok(())
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn geometric(data: &mut DwgBitWriter, value: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgGeometricConstraint) -> Result<(), String> {
        if !value.active {
            return Err("AC1024 geometric constraint must be active".into());
        }
        core(data, &value.node)?;
        data.write_bl(value.owner_node_id);
        data.write_b(value.implied);
        Ok(())
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn geometry(data: &mut DwgBitWriter, handles: &mut DwgBitWriter, value: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgConstraintGeometry) -> Result<(), String> {
        core(data, &value.node)?;
        handles.write_handle(4, value.geometry_dependency_handle.unwrap_or_default());
        data.write_bl(value.geometry_node_id);
        Ok(())
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn explicit(data: &mut DwgBitWriter, handles: &mut DwgBitWriter, value: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgExplicitConstraint) -> Result<(), String> {
        geometric(data, &value.geometric)?;
        if value.value_dependency_handle == 0 || value.dimension_dependency_handle == 0 {
            return Err("explicit constraint dependency handles must be nonnull".into());
        }
        handles.write_handle(5, value.value_dependency_handle);
        handles.write_handle(5, value.dimension_dependency_handle);
        Ok(())
    }
    let class = match node {
        DwgConstraintNode::ConstrainedImplicitPoint(value) => {
            geometry(data, handles, &value.geometry)?;
            if value.geometry.geometry_dependency_handle.is_some() {
                data.write_3bd(constraint_point3(value.point.as_deref().ok_or("dependent implicit point is missing its point")?, "implicit point")?);
            } else if value.point.is_some() {
                return Err("independent implicit point must not persist a conditional point".into());
            }
            data.write_rc(value.point_kind);
            data.write_bl(value.point_index as u32);
            data.write_bl(value.curve_node_id as u32);
            "AcConstrainedImplicitPoint"
        }
        DwgConstraintNode::PointCurveConstraint(value) => {
            geometric(data, value)?;
            "AcPointCurveConstraint"
        }
        DwgConstraintNode::ConstrainedBoundedLine(value) => {
            geometry(data, handles, &value.geometry)?;
            if value.bounded == value.ray {
                return Err("bounded-line ray and bounded states must be complementary".into());
            }
            data.write_3bd(constraint_point3(&value.origin, "bounded-line origin")?);
            data.write_3bd(constraint_point3(&value.direction, "bounded-line direction")?);
            data.write_b(value.ray);
            data.write_3bd(constraint_point3(&value.start_point, "bounded-line start")?);
            data.write_3bd(constraint_point3(&value.end_point, "bounded-line end")?);
            "AcConstrainedBoundedLine"
        }
        DwgConstraintNode::PointCoincidenceConstraint(value) => {
            geometric(data, value)?;
            "AcPointCoincidenceConstraint"
        }
        DwgConstraintNode::DistanceConstraint(value) => {
            explicit(data, handles, &value.explicit)?;
            data.write_rc(value.direction_kind);
            if value.direction_kind != 0 {
                data.write_3bd(constraint_point3(value.direction.as_deref().ok_or("directed distance is missing its direction")?, "distance direction")?);
            } else if value.direction.is_some() {
                return Err("undirected distance must not persist a conditional direction".into());
            }
            "AcDistanceConstraint"
        }
        DwgConstraintNode::PerpendicularConstraint(value) => {
            geometric(data, value)?;
            "AcPerpendicularConstraint"
        }
        DwgConstraintNode::HorizontalConstraint(value) => {
            geometric(data, &value.geometric)?;
            data.write_bl(value.datum_line_index as u32);
            "AcHorizontalConstraint"
        }
        DwgConstraintNode::ParallelConstraint(value) => {
            geometric(data, value)?;
            "AcParallelConstraint"
        }
        DwgConstraintNode::MidPointConstraint(value) => {
            geometric(data, value)?;
            "AcMidPointConstraint"
        }
        DwgConstraintNode::EqualLengthConstraint(value) => {
            geometric(data, value)?;
            "AcEqualLengthConstraint"
        }
        DwgConstraintNode::ColinearConstraint(value) => {
            geometric(data, value)?;
            "AcColinearConstraint"
        }
        DwgConstraintNode::ConstrainedDatumLine(value) => {
            geometry(data, handles, &value.geometry)?;
            data.write_3bd(constraint_point3(&value.origin, "datum-line origin")?);
            data.write_3bd(constraint_point3(&value.direction, "datum-line direction")?);
            "AcConstrainedDatumLine"
        }
        DwgConstraintNode::FixedConstraint(value) => {
            geometric(data, value)?;
            "AcFixedConstraint"
        }
        DwgConstraintNode::VerticalConstraint(value) => {
            geometric(data, &value.geometric)?;
            data.write_bl(value.datum_line_index as u32);
            "AcVerticalConstraint"
        }
    };
    strings.write_tu(class);
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_assoc_2d_constraint_group_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAssociativeActionStatus;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
    let Some(DwgLogicalObjectBody::Assoc2dConstraintGroup(group)) = object.body.as_ref() else { return Err(format!("ACDBASSOC2DCONSTRAINTGROUP {:#x} body missing", object.handle)) };
    if !matches!(group.action.status, DwgAssociativeActionStatus::UpToDate)
        || object.owner_handle.is_none()
        || !object.reactor_handles.is_empty()
        || object.extension_dictionary_handle.is_some()
        || group.action.action_body_handle.is_some()
        || group.work_plane.len() != 3
        || group.member_action_handles.contains(&0)
        || group.nodes.is_empty()
        || group.nodes.len() > 10_000
    {
        return Err(format!("ACDBASSOC2DCONSTRAINTGROUP {:#x} logical state is invalid", object.handle));
    }
    let work_plane = group.work_plane.iter().enumerate().map(|(index, point)| constraint_point3(point, &format!("work-plane point {index}"))).collect::<Result<Vec<_>, _>>()?;
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(0);
    data.write_b(true);
    data.write_bs(1);
    data.write_bl(0);
    data.write_bl(group.action.action_index as u32);
    data.write_bl(group.action.maximum_dependency_index as u32);
    data.write_bl(group.action.dependencies.len() as u32);
    for dependency in &group.action.dependencies {
        data.write_b(dependency.owned);
    }
    data.write_bl(0);
    data.write_b(group.do_not_check_newly_added_constraints);
    for point in work_plane {
        data.write_3bd(point);
    }
    data.write_bl(group.member_action_handles.len() as u32);
    data.write_bl(group.nodes.iter().map(constraint_node_id).max().and_then(|value| value.checked_add(1)).ok_or("constraint group node watermark is invalid")? as u32);
    data.write_bl(group.nodes.len() as u32);
    let mut strings = DwgBitWriter::new();
    let mut node_handles = DwgBitWriter::new();
    for node in &group.nodes {
        encode_r2010_constraint_node(node, &mut data, &mut strings, &mut node_handles)?;
    }
    append_r2010_string_stream(&mut data, &strings, "ACDBASSOC2DCONSTRAINTGROUP", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    handles.write_handle(4, group.action.owning_network_handle.unwrap_or_default());
    handles.write_handle(3, 0);
    for dependency in &group.action.dependencies {
        handles.write_handle(if dependency.owned { 3 } else { 4 }, dependency.dependency_handle);
    }
    handles.write_handle(3, 0);
    for member in &group.member_action_handles {
        handles.write_handle(3, *member);
    }
    handles.append_bits(&node_handles);
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_block_element(data: &mut DwgBitWriter, strings: &mut DwgBitWriter, element: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockElement, class_name: &str) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEvaluationExpressionValue;
    let expression = &element.evaluation_expression;
    if expression.parent_id != -1 || expression.major_version != 29 || expression.minor_version != 2 || !matches!(expression.value, DwgEvaluationExpressionValue::Empty) || element.name.is_empty() {
        return Err(format!("{class_name} block-element state is invalid"));
    }
    data.write_bl(expression.parent_id as u32);
    data.write_bl(expression.major_version);
    data.write_bl(expression.minor_version);
    data.write_bs((-9999i16) as u16);
    data.write_bl(expression.node_id);
    strings.write_tu(&element.name);
    data.write_bl(expression.major_version);
    data.write_bl(expression.minor_version);
    data.write_bl(0);
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_block_grip(data: &mut DwgBitWriter, strings: &mut DwgBitWriter, grip: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockGrip, x_role: &str, y_role: &str, class_name: &str) -> Result<(), String> {
    if grip.location.len() != 3 || grip.location.iter().any(|value| !value.is_finite()) || grip.updated_x.expression_name != x_role || grip.updated_y.expression_name != y_role {
        return Err(format!("{class_name} grip state is invalid"));
    }
    encode_r2010_block_element(data, strings, &grip.element, class_name)?;
    data.write_bl(grip.updated_x.node_id);
    data.write_bl(grip.updated_y.node_id);
    data.write_3bd([grip.location[0], grip.location[1], grip.location[2]]);
    data.write_b(grip.insertion_cycling);
    data.write_bl(grip.insertion_cycling_weight as u32);
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_dynamic_block_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockParameterBaseLocation;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
    if !object.extended_data.is_empty() || object.owner_handle.is_none() || !object.reactor_handles.is_empty() || object.extension_dictionary_handle.is_some() {
        return Err(format!("{} {:#x} common state is invalid", object.class_name, object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(0);
    data.write_b(true);
    let mut strings = DwgBitWriter::new();
    match object.body.as_ref() {
        Some(DwgLogicalObjectBody::BlockLinearParameter(body)) if object.type_code == 527 => {
            let parameter = &body.parameter;
            if parameter.definition_base.len() != 3
                || parameter.definition_end.len() != 3
                || parameter.properties.len() != 4
                || parameter.property_expression_references.iter().any(|reference| reference.property_index >= 4 || reference.node_id == 0)
                || parameter.property_expression_references.iter().enumerate().any(|(index, reference)| parameter.property_expression_references[..index].iter().any(|candidate| candidate.property_index == reference.property_index))
                || parameter.definition_base.iter().chain(&parameter.definition_end).any(|value| !value.is_finite())
                || body.allowed_values.is_empty()
                || body.allowed_values.len() > u16::MAX as usize
                || body.allowed_values.iter().any(|value| !value.is_finite())
                || !body.label_offset.is_finite()
            {
                return Err(format!("BLOCKLINEARPARAMETER {:#x} logical state is invalid", object.handle));
            }
            encode_r2010_block_element(&mut data, &mut strings, &parameter.element, "BLOCKLINEARPARAMETER")?;
            data.write_b(parameter.show_properties);
            data.write_b(parameter.chain_actions);
            data.write_3bd([parameter.definition_base[0], parameter.definition_base[1], parameter.definition_base[2]]);
            data.write_3bd([parameter.definition_end[0], parameter.definition_end[1], parameter.definition_end[2]]);
            for property in &parameter.properties {
                data.write_bl(property.connections.len() as u32);
                for connection in &property.connections {
                    data.write_bl(connection.code);
                    strings.write_tu(&connection.name);
                }
            }
            for property_index in 0..4 {
                data.write_bl(parameter.property_expression_references.iter().find(|reference| reference.property_index == property_index).map_or(0, |reference| reference.node_id));
            }
            data.write_bs(match parameter.base_location {
                DwgBlockParameterBaseLocation::StartPoint => 0,
                DwgBlockParameterBaseLocation::Midpoint => 1,
            });
            strings.write_tu(&body.distance_name);
            strings.write_tu(&body.distance_description);
            data.write_bd(body.label_offset);
            data.write_bl(8);
            data.write_bd(0.0);
            data.write_bd(0.0);
            data.write_bd(0.0);
            data.write_bs(body.allowed_values.len() as u16);
            for value in &body.allowed_values {
                data.write_bd(*value);
            }
        }
        Some(DwgLogicalObjectBody::BlockLinearGrip(body)) if object.type_code == 528 => {
            if body.orientation.len() != 3 || body.orientation.iter().any(|value| !value.is_finite()) || body.orientation.iter().all(|value| *value == 0.0) {
                return Err(format!("BLOCKLINEARGRIP {:#x} orientation is invalid", object.handle));
            }
            encode_r2010_block_grip(&mut data, &mut strings, &body.grip, "UpdatedEndX", "UpdatedEndY", "BLOCKLINEARGRIP")?;
            data.write_3bd([body.orientation[0], body.orientation[1], body.orientation[2]]);
        }
        Some(DwgLogicalObjectBody::BlockFlipGrip(body)) if object.type_code == 530 => {
            if body.updated_flip.expression_name != "UpdatedFlip" || body.orientation.len() != 3 || body.orientation.iter().any(|value| !value.is_finite()) || body.orientation.iter().all(|value| *value == 0.0) {
                return Err(format!("BLOCKFLIPGRIP {:#x} state is invalid", object.handle));
            }
            encode_r2010_block_grip(&mut data, &mut strings, &body.grip, "UpdatedBaseX", "UpdatedBaseY", "BLOCKFLIPGRIP")?;
            data.write_bl(body.updated_flip.node_id);
            data.write_3bd([body.orientation[0], body.orientation[1], body.orientation[2]]);
        }
        Some(DwgLogicalObjectBody::BlockVisibilityGrip(body)) if object.type_code == 532 => {
            encode_r2010_block_grip(&mut data, &mut strings, &body.grip, "UpdatedX", "UpdatedY", "BLOCKVISIBILITYGRIP")?;
        }
        _ => return Err(format!("{} {:#x} body missing or mismatched", object.class_name, object.handle)),
    }
    append_r2010_string_stream(&mut data, &strings, &object.class_name, object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_two_point_parameter(data: &mut DwgBitWriter, strings: &mut DwgBitWriter, parameter: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockTwoPointParameter, property_node_ids: [u32; 4], class_name: &str) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockParameterBaseLocation;
    if parameter.definition_base.len() != 3 || parameter.definition_end.len() != 3 || parameter.properties.len() != 4 || parameter.definition_base.iter().chain(&parameter.definition_end).any(|value| !value.is_finite()) {
        return Err(format!("{class_name} two-point parameter is invalid"));
    }
    encode_r2010_block_element(data, strings, &parameter.element, class_name)?;
    data.write_b(parameter.show_properties);
    data.write_b(parameter.chain_actions);
    data.write_3bd([parameter.definition_base[0], parameter.definition_base[1], parameter.definition_base[2]]);
    data.write_3bd([parameter.definition_end[0], parameter.definition_end[1], parameter.definition_end[2]]);
    for property in &parameter.properties {
        data.write_bl(property.connections.len() as u32);
        for connection in &property.connections {
            data.write_bl(connection.code);
            strings.write_tu(&connection.name);
        }
    }
    for node_id in property_node_ids {
        data.write_bl(node_id);
    }
    data.write_bs(match parameter.base_location {
        DwgBlockParameterBaseLocation::StartPoint => 0,
        DwgBlockParameterBaseLocation::Midpoint => 1,
    });
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_block_action(data: &mut DwgBitWriter, strings: &mut DwgBitWriter, handles: &mut DwgBitWriter, action: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockAction, class_name: &str) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockElement;
    if action.display_location.len() != 3 || action.display_location.iter().any(|value| !value.is_finite()) || action.name.is_empty() {
        return Err(format!("{class_name} action is invalid"));
    }
    encode_r2010_block_element(data, strings, &DwgBlockElement { evaluation_expression: action.evaluation_expression.clone(), name: action.name.clone() }, class_name)?;
    data.write_3bd([action.display_location[0], action.display_location[1], action.display_location[2]]);
    data.write_bl(action.dependencies.len() as u32);
    data.write_bl(action.action_node_ids.len() as u32);
    for node_id in &action.action_node_ids {
        data.write_bl(*node_id);
    }
    for dependency in &action.dependencies {
        if dependency.object_handle == 0 {
            return Err(format!("{class_name} dependency is null"));
        }
        handles.write_handle(4, dependency.object_handle);
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_r2010_action_connection(data: &mut DwgBitWriter, strings: &mut DwgBitWriter, connection: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockActionConnection) -> Result<(), String> {
    if connection.name.is_empty() {
        return Err("block-action connection name is empty".into());
    }
    data.write_bl(connection.node_id);
    strings.write_tu(&connection.name);
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_alignment_action_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockActionCoordinateMode;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockScaleMode;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
    if !object.extended_data.is_empty() || object.owner_handle.is_none() || !object.reactor_handles.is_empty() || object.extension_dictionary_handle.is_some() {
        return Err(format!("{} {:#x} common state is invalid", object.class_name, object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(0);
    data.write_b(true);
    let mut strings = DwgBitWriter::new();
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    match object.body.as_ref() {
        Some(DwgLogicalObjectBody::BlockAlignmentParameter(body)) if object.type_code == 533 => {
            if body.updated_grip_node_id == 0 || !body.parameter.property_expression_references.is_empty() {
                return Err(format!("BLOCKALIGNMENTPARAMETER {:#x} relation state is invalid", object.handle));
            }
            encode_r2010_two_point_parameter(&mut data, &mut strings, &body.parameter, [body.updated_grip_node_id, 0, 0, 0], "BLOCKALIGNMENTPARAMETER")?;
            data.write_b(body.align_perpendicular);
        }
        Some(DwgLogicalObjectBody::BlockAlignmentGrip(body)) if object.type_code == 534 => {
            if body.first_location_node_id == 0
                || body.second_location_node_id == 0
                || body.grip.updated_x.node_id != body.first_location_node_id
                || body.grip.updated_y.node_id != body.second_location_node_id
                || body.orientation.len() != 3
                || body.orientation.iter().any(|value| !value.is_finite())
                || body.orientation.iter().all(|value| *value == 0.0)
            {
                return Err(format!("BLOCKALIGNMENTGRIP {:#x} logical state is invalid", object.handle));
            }
            encode_r2010_block_grip(&mut data, &mut strings, &body.grip, "FirstLocation", "SecondLocation", "BLOCKALIGNMENTGRIP")?;
            data.write_3bd([body.orientation[0], body.orientation[1], body.orientation[2]]);
        }
        Some(DwgLogicalObjectBody::BlockStretchAction(body)) if object.type_code == 535 => {
            if body.points.is_empty()
                || body.points.iter().any(|point| point.len() != 2 || point.iter().any(|value| !value.is_finite()))
                || body.selections.iter().any(|selection| selection.object_handle == 0 || selection.vertex_indices.is_empty())
                || body.selectors.iter().any(|selector| selector.node_id == 0 || selector.point_indices.iter().any(|index| *index as usize >= body.points.len()))
                || !body.distance_multiplier.is_finite()
                || body.distance_multiplier <= 0.0
                || !body.angle_offset.is_finite()
                || !matches!(body.coordinate_mode, DwgBlockActionCoordinateMode::CartesianXy)
            {
                return Err(format!("BLOCKSTRETCHACTION {:#x} logical state is invalid", object.handle));
            }
            encode_r2010_block_action(&mut data, &mut strings, &mut handles, &body.action, "BLOCKSTRETCHACTION")?;
            write_r2010_action_connection(&mut data, &mut strings, &body.x_connection)?;
            write_r2010_action_connection(&mut data, &mut strings, &body.y_connection)?;
            data.write_bl(body.points.len() as u32);
            for point in &body.points {
                data.write_2rd([point[0], point[1]]);
            }
            data.write_bl(body.selections.len() as u32);
            for selection in &body.selections {
                data.write_bs(u16::try_from(selection.vertex_indices.len()).map_err(|_| "BLOCKSTRETCHACTION selection index count exceeds BS")?);
                for index in &selection.vertex_indices {
                    data.write_bl(*index);
                }
            }
            data.write_bl(body.selectors.len() as u32);
            for selector in &body.selectors {
                data.write_bl(selector.node_id);
                data.write_bs(u16::try_from(selector.point_indices.len()).map_err(|_| "BLOCKSTRETCHACTION selector index count exceeds BS")?);
                for index in &selector.point_indices {
                    data.write_bl(*index);
                }
            }
            data.write_bd(body.distance_multiplier);
            data.write_bd(body.angle_offset);
            data.write_rc(0);
            for selection in &body.selections {
                handles.write_handle(4, selection.object_handle);
            }
        }
        Some(DwgLogicalObjectBody::BlockScaleAction(body)) if object.type_code == 536 => {
            let base = &body.base;
            if base.offset.len() != 3 || base.base_point.len() != 3 || base.offset.iter().chain(&base.base_point).any(|value| !value.is_finite()) || !matches!(body.mode, DwgBlockScaleMode::Xy) {
                return Err(format!("BLOCKSCALEACTION {:#x} logical state is invalid", object.handle));
            }
            encode_r2010_block_action(&mut data, &mut strings, &mut handles, &base.action, "BLOCKSCALEACTION")?;
            data.write_3bd([base.offset[0], base.offset[1], base.offset[2]]);
            write_r2010_action_connection(&mut data, &mut strings, &base.x_base_connection)?;
            write_r2010_action_connection(&mut data, &mut strings, &base.y_base_connection)?;
            data.write_b(base.dependent);
            data.write_3bd([base.base_point[0], base.base_point[1], base.base_point[2]]);
            write_r2010_action_connection(&mut data, &mut strings, &body.uniform_scale_connection)?;
            write_r2010_action_connection(&mut data, &mut strings, &body.x_scale_connection)?;
            write_r2010_action_connection(&mut data, &mut strings, &body.y_scale_connection)?;
            data.write_rc(0);
        }
        Some(DwgLogicalObjectBody::BlockFlipAction(body)) if object.type_code == 537 => {
            encode_r2010_block_action(&mut data, &mut strings, &mut handles, &body.action, "BLOCKFLIPACTION")?;
            for connection in [&body.flip_connection, &body.updated_flip_connection, &body.updated_base_connection, &body.updated_end_connection] {
                write_r2010_action_connection(&mut data, &mut strings, connection)?;
            }
        }
        _ => return Err(format!("{} {:#x} body missing or mismatched", object.class_name, object.handle)),
    }
    append_r2010_string_stream(&mut data, &strings, &object.class_name, object.handle)?;
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_final_parameter_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
    if !object.extended_data.is_empty() || object.owner_handle.is_none() || object.extension_dictionary_handle.is_some() {
        return Err(format!("{} {:#x} common state is invalid", object.class_name, object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(object.reactor_handles.len() as u32);
    data.write_b(true);
    let mut strings = DwgBitWriter::new();
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    for reactor in &object.reactor_handles {
        handles.write_handle(4, *reactor);
    }
    match object.body.as_ref() {
        Some(DwgLogicalObjectBody::BlockBasePointParameter(body)) if object.type_code == 538 => {
            let parameter = &body.parameter;
            if !object.reactor_handles.is_empty()
                || parameter.definition_point.len() != 3
                || body.point.len() != 3
                || body.base_point.len() != 3
                || parameter.properties.len() != 2
                || parameter.definition_point.iter().chain(&body.point).chain(&body.base_point).any(|value| !value.is_finite())
            {
                return Err(format!("BLOCKBASEPOINTPARAMETER {:#x} logical state is invalid", object.handle));
            }
            encode_r2010_block_element(&mut data, &mut strings, &parameter.element, "BLOCKBASEPOINTPARAMETER")?;
            data.write_b(parameter.show_properties);
            data.write_b(parameter.chain_actions);
            data.write_3bd([parameter.definition_point[0], parameter.definition_point[1], parameter.definition_point[2]]);
            for property in &parameter.properties {
                data.write_bl(property.connections.len() as u32);
                for connection in &property.connections {
                    data.write_bl(connection.code);
                    strings.write_tu(&connection.name);
                }
            }
            data.write_bl(0);
            data.write_3bd([body.point[0], body.point[1], body.point[2]]);
            data.write_3bd([body.base_point[0], body.base_point[1], body.base_point[2]]);
        }
        Some(DwgLogicalObjectBody::BlockVerticalConstraintParameter(body) | DwgLogicalObjectBody::BlockHorizontalConstraintParameter(body)) if matches!(object.type_code, 546 | 548) => {
            if object.reactor_handles.len() != 3
                || body.displacement_grip_node_id == 0
                || body.dependency_handle == 0
                || body.expression_name.is_empty()
                || !body.value.is_finite()
                || body.allowed_values.values.is_empty()
                || body.allowed_values.values.len() > u16::MAX as usize
                || body.allowed_values.values.iter().any(|value| !value.is_finite())
                || !body.parameter.property_expression_references.is_empty()
            {
                return Err(format!("{} {:#x} logical state is invalid", object.class_name, object.handle));
            }
            encode_r2010_two_point_parameter(&mut data, &mut strings, &body.parameter, [0, body.displacement_grip_node_id, 0, 0], &object.class_name)?;
            handles.write_handle(4, body.dependency_handle);
            strings.write_tu(&body.expression_name);
            strings.write_tu(&body.expression_description);
            data.write_bd(body.value);
            data.write_bl(8);
            let delta = body.allowed_values.values.windows(2).next().map(|pair| pair[1] - pair[0]);
            let uniform = delta.filter(|step| *step != 0.0 && body.allowed_values.values.windows(2).all(|pair| pair[1] - pair[0] == *step));
            if let Some(step) = uniform {
                data.write_bd(body.allowed_values.values[0]);
                data.write_bd(*body.allowed_values.values.last().unwrap());
                data.write_bd(step);
            } else {
                data.write_bd(0.0);
                data.write_bd(0.0);
                data.write_bd(0.0);
            }
            data.write_bs(body.allowed_values.values.len() as u16);
            for value in &body.allowed_values.values {
                data.write_bd(*value);
            }
        }
        _ => return Err(format!("{} {:#x} body missing or mismatched", object.class_name, object.handle)),
    }
    append_r2010_string_stream(&mut data, &strings, &object.class_name, object.handle)?;
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_layout_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgOrthographicView;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgPlotArea;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgPlotPaperUnit;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgPlotRotation;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgShadePlot;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgShadePlotResolution;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgStandardScale;
    let Some(DwgLogicalObjectBody::Layout(layout)) = object.body.as_ref() else { return Err(format!("LAYOUT {:#x} body missing", object.handle)) };
    let dimensions = [
        (&layout.margins, 4),
        (&layout.paper_size, 2),
        (&layout.plot_origin, 2),
        (&layout.plot_window_lower_left, 2),
        (&layout.plot_window_upper_right, 2),
        (&layout.paper_image_origin, 2),
        (&layout.insertion_base, 3),
        (&layout.limits_minimum, 2),
        (&layout.limits_maximum, 2),
        (&layout.ucs_origin, 3),
        (&layout.ucs_x_axis, 3),
        (&layout.ucs_y_axis, 3),
        (&layout.extents_minimum, 3),
        (&layout.extents_maximum, 3),
    ];
    if object.type_code != 82
        || object.owner_handle.is_none()
        || object.reactor_handles.len() != 1
        || object.extension_dictionary_handle.is_none()
        || layout.block_header_handle == 0
        || dimensions.iter().any(|(values, length)| values.len() != *length || values.iter().any(|value| !value.is_finite()))
        || layout.viewport_handles.contains(&0)
        || layout.viewport_handles.iter().enumerate().any(|(index, handle)| layout.viewport_handles[..index].contains(handle))
        || !layout.paper_units.is_finite()
        || !layout.drawing_units.is_finite()
        || !layout.standard_scale_factor.is_finite()
        || !layout.ucs_elevation.is_finite()
    {
        return Err(format!("LAYOUT {:#x} logical state is invalid", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(1);
    data.write_b(false);
    let mut strings = DwgBitWriter::new();
    strings.write_tu(&layout.page_setup_name);
    strings.write_tu(&layout.printer_configuration);
    let flags = (u16::from(layout.plot_options.use_standard_scale) * 16)
        | (u16::from(layout.plot_options.plot_viewport_borders) * 32)
        | (u16::from(layout.plot_options.plot_with_lineweights) * 128)
        | (u16::from(layout.plot_options.draw_viewports_first) * 512)
        | (u16::from(layout.plot_options.model_type) * 1024)
        | (u16::from(layout.plot_options.update_paper) * 2048)
        | (u16::from(layout.plot_options.initializing) * 8192);
    data.write_bs(flags);
    for value in &layout.margins {
        data.write_bd(*value);
    }
    for value in &layout.paper_size {
        data.write_bd(*value);
    }
    strings.write_tu(&layout.canonical_media_name);
    for value in &layout.plot_origin {
        data.write_bd(*value);
    }
    data.write_bs(match layout.paper_unit {
        DwgPlotPaperUnit::Inches => 0,
    });
    data.write_bs(match layout.rotation {
        DwgPlotRotation::QuarterTurn => 1,
    });
    data.write_bs(match layout.plot_area {
        DwgPlotArea::Display => 0,
        DwgPlotArea::Layout => 5,
    });
    for value in &layout.plot_window_lower_left {
        data.write_bd(*value);
    }
    for value in &layout.plot_window_upper_right {
        data.write_bd(*value);
    }
    data.write_bd(layout.paper_units);
    data.write_bd(layout.drawing_units);
    strings.write_tu(&layout.stylesheet);
    data.write_bs(match layout.standard_scale {
        DwgStandardScale::Custom => 0,
        DwgStandardScale::OneToOne => 16,
    });
    data.write_bd(layout.standard_scale_factor);
    for value in &layout.paper_image_origin {
        data.write_bd(*value);
    }
    data.write_bs(match layout.shade_plot {
        DwgShadePlot::AsDisplayed => 0,
    });
    data.write_bs(match layout.shade_plot_resolution {
        DwgShadePlotResolution::Normal => 2,
    });
    data.write_bs(layout.shade_plot_dpi);
    strings.write_tu(&layout.name);
    data.write_bs(layout.tab_order);
    data.write_bs(u16::from(layout.options.paper_space_linetype_scaling));
    data.write_3bd([layout.insertion_base[0], layout.insertion_base[1], layout.insertion_base[2]]);
    data.write_2rd([layout.limits_minimum[0], layout.limits_minimum[1]]);
    data.write_2rd([layout.limits_maximum[0], layout.limits_maximum[1]]);
    data.write_3bd([layout.ucs_origin[0], layout.ucs_origin[1], layout.ucs_origin[2]]);
    data.write_3bd([layout.ucs_x_axis[0], layout.ucs_x_axis[1], layout.ucs_x_axis[2]]);
    data.write_3bd([layout.ucs_y_axis[0], layout.ucs_y_axis[1], layout.ucs_y_axis[2]]);
    data.write_bd(layout.ucs_elevation);
    data.write_bs(match layout.orthographic_view {
        DwgOrthographicView::None => 0,
        DwgOrthographicView::Top => 1,
        DwgOrthographicView::Bottom => 2,
        DwgOrthographicView::Front => 3,
        DwgOrthographicView::Back => 4,
        DwgOrthographicView::Left => 5,
        DwgOrthographicView::Right => 6,
    });
    data.write_3bd([layout.extents_minimum[0], layout.extents_minimum[1], layout.extents_minimum[2]]);
    data.write_3bd([layout.extents_maximum[0], layout.extents_maximum[1], layout.extents_maximum[2]]);
    data.write_bl(layout.viewport_handles.len() as u32);
    append_r2010_string_stream(&mut data, &strings, "LAYOUT", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    handles.write_handle(4, object.reactor_handles[0]);
    handles.write_handle(3, object.extension_dictionary_handle.unwrap());
    handles.write_handle(5, layout.plot_view_handle.unwrap_or_default());
    handles.write_handle(4, layout.visual_style_handle.unwrap_or_default());
    handles.write_handle(4, layout.block_header_handle);
    handles.write_handle(4, layout.active_viewport_handle.unwrap_or_default());
    handles.write_handle(5, layout.base_ucs_handle.unwrap_or_default());
    handles.write_handle(5, layout.named_ucs_handle.unwrap_or_default());
    for viewport in &layout.viewport_handles {
        handles.write_handle(4, *viewport);
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_object_frame(object: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject, block_names: &std::collections::BTreeMap<u64, String>) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityBody;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
    match object.body.as_ref().ok_or_else(|| format!("object {:#x} has no typed body", object.handle))? {
        DwgLogicalObjectBody::Dictionary(_) => encode_r2010_dictionary_frame(object),
        DwgLogicalObjectBody::TableControl(_) => encode_r2010_table_control_frame(object),
        DwgLogicalObjectBody::TableRecord(_) => encode_r2010_table_record_frame(object),
        DwgLogicalObjectBody::XRecord(_) => encode_r2010_xrecord_frame(object),
        DwgLogicalObjectBody::Entity(entity) => match entity {
            DwgEntityBody::Line(_) => encode_r2010_line_frame(object),
            DwgEntityBody::Arc(_) => encode_r2010_arc_frame(object),
            DwgEntityBody::LwPolyline(_) => encode_r2010_lwpolyline_frame(object),
            DwgEntityBody::BlockBegin(_) => encode_r2010_block_begin_frame(object, block_names.get(&object.handle).ok_or_else(|| format!("BLOCK {:#x} has no logical block-header name", object.handle))?),
            DwgEntityBody::BlockEnd(_) => encode_r2010_block_end_frame(object),
            DwgEntityBody::Insert(_) => encode_r2010_insert_frame(object),
            DwgEntityBody::DimensionLinear(_) => encode_r2010_dimension_linear_frame(object),
            DwgEntityBody::Viewport(_) => encode_r2010_viewport_frame(object),
            DwgEntityBody::Point(_) | DwgEntityBody::Circle(_) | DwgEntityBody::Ellipse(_) | DwgEntityBody::Text(_) | DwgEntityBody::Spline(_) | DwgEntityBody::Face3d(_) | DwgEntityBody::Polyline3d(_) | DwgEntityBody::PolyfaceMesh(_) | DwgEntityBody::Vertex(_) | DwgEntityBody::PolyfaceFace(_) | DwgEntityBody::SequenceEnd(_) => {
                encode_r2010_entity_body_frame(object)
            }
        },
        DwgLogicalObjectBody::AssociativeDependency(_) => encode_r2010_associative_dependency_frame(object),
        DwgLogicalObjectBody::AssociativeValueDependency(_) => encode_r2010_associative_value_dependency_frame(object),
        DwgLogicalObjectBody::AssociativeGeometryDependency(_) => encode_r2010_associative_geometry_dependency_frame(object),
        DwgLogicalObjectBody::BlockGripLocationComponent(_) => encode_r2010_block_grip_location_component_frame(object),
        DwgLogicalObjectBody::DynamicBlockProxyNode(_) => encode_r2010_dynamic_block_proxy_node_frame(object),
        DwgLogicalObjectBody::AssociativeVariable(_) => encode_r2010_associative_variable_frame(object),
        DwgLogicalObjectBody::AssociativeDimensionDependencyBody(_) => encode_r2010_associative_dimension_dependency_body_frame(object),
        DwgLogicalObjectBody::VisualStyle(_) => encode_r2010_visual_style_frame(object),
        DwgLogicalObjectBody::BlockParameterDependencyBody(_) => encode_r2010_block_parameter_dependency_body_frame(object),
        DwgLogicalObjectBody::BlockRepresentationData(_) => encode_r2010_block_representation_data_frame(object),
        DwgLogicalObjectBody::DynamicBlockPurgePreventer(_) => encode_r2010_dynamic_block_purge_preventer_frame(object),
        DwgLogicalObjectBody::EvaluationGraph(_) => encode_r2010_evaluation_graph_frame(object),
        DwgLogicalObjectBody::BlockFlipParameter(_) => encode_r2010_block_flip_parameter_frame(object),
        DwgLogicalObjectBody::BlockVisibilityParameter(_) => encode_r2010_block_visibility_parameter_frame(object),
        DwgLogicalObjectBody::Placeholder(_) => encode_r2010_placeholder_frame(object),
        DwgLogicalObjectBody::DictionaryVariable(_) => encode_r2010_dictionary_variable_frame(object),
        DwgLogicalObjectBody::AnnotationScale(_) => encode_r2010_annotation_scale_frame(object),
        DwgLogicalObjectBody::SortEntitiesTable(_) => encode_r2010_sort_entities_table_frame(object),
        DwgLogicalObjectBody::TableStyle(_) => encode_r2010_table_style_frame(object),
        DwgLogicalObjectBody::MlineStyle(_) => encode_r2010_mline_style_frame(object),
        DwgLogicalObjectBody::MLeaderStyle(_) => encode_r2010_mleader_style_frame(object),
        DwgLogicalObjectBody::Material(_) => encode_r2010_material_frame(object),
        DwgLogicalObjectBody::BlockMoveAction(_) => encode_r2010_block_move_action_frame(object),
        DwgLogicalObjectBody::AssocNetwork(_) => encode_r2010_assoc_network_frame(object),
        DwgLogicalObjectBody::Assoc2dConstraintGroup(_) => encode_r2010_assoc_2d_constraint_group_frame(object),
        DwgLogicalObjectBody::BlockLinearParameter(_) | DwgLogicalObjectBody::BlockLinearGrip(_) | DwgLogicalObjectBody::BlockFlipGrip(_) | DwgLogicalObjectBody::BlockVisibilityGrip(_) => encode_r2010_dynamic_block_frame(object),
        DwgLogicalObjectBody::BlockAlignmentParameter(_) | DwgLogicalObjectBody::BlockAlignmentGrip(_) | DwgLogicalObjectBody::BlockStretchAction(_) | DwgLogicalObjectBody::BlockScaleAction(_) | DwgLogicalObjectBody::BlockFlipAction(_) => {
            encode_r2010_alignment_action_frame(object)
        }
        DwgLogicalObjectBody::BlockBasePointParameter(_) | DwgLogicalObjectBody::BlockVerticalConstraintParameter(_) | DwgLogicalObjectBody::BlockHorizontalConstraintParameter(_) => encode_r2010_final_parameter_frame(object),
        DwgLogicalObjectBody::Layout(_) => encode_r2010_layout_frame(object),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_r2004_unsigned_modular_char(output: &mut Vec<u8>, mut value: u64) {
    while value >= 0x80 {
        output.push((value as u8 & 0x7f) | 0x80);
        value >>= 7;
    }
    output.push(value as u8);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_r2004_signed_modular_char(output: &mut Vec<u8>, value: i64) {
    let negative = value < 0;
    let mut magnitude = value.unsigned_abs();
    while magnitude >= 0x40 {
        output.push((magnitude as u8 & 0x7f) | 0x80);
        magnitude >>= 7;
    }
    output.push(magnitude as u8 | if negative { 0x40 } else { 0 });
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn finish_r2004_handle_block(output: &mut Vec<u8>, block: &mut Vec<u8>) -> Result<(), String> {
    let size = u16::try_from(block.len()).map_err(|_| "R2004 Handles block exceeds u16")?;
    block[0..2].copy_from_slice(&size.to_be_bytes());
    let crc = dwg_crc16(0xC0C1, block);
    output.extend_from_slice(block);
    output.extend_from_slice(&crc.to_be_bytes());
    block.clear();
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn materialize_r2004_handles(pairs: &[(u64, usize)]) -> Result<Vec<u8>, String> {
    let mut sorted = pairs.to_vec();
    sorted.sort_by_key(|(handle, _)| *handle);
    if sorted.iter().any(|(handle, _)| *handle == 0) || sorted.windows(2).any(|pair| pair[0].0 == pair[1].0) {
        return Err("R2004 Handles requires unique nonzero handles".into());
    }
    let mut output = Vec::new();
    let mut block = vec![0u8; 2];
    let mut last_handle = 0u64;
    let mut last_address = 0usize;
    for (handle, address) in sorted {
        write_r2004_unsigned_modular_char(&mut block, handle.checked_sub(last_handle).ok_or("R2004 Handles are not strictly increasing")?);
        write_r2004_signed_modular_char(&mut block, i64::try_from(address).map_err(|_| "R2004 object address exceeds i64")? - i64::try_from(last_address).map_err(|_| "R2004 prior object address exceeds i64")?);
        last_handle = handle;
        last_address = address;
        if block.len() > 2030 {
            finish_r2004_handle_block(&mut output, &mut block)?;
            block.extend_from_slice(&[0, 0]);
            last_handle = 0;
            last_address = 0;
        }
    }
    if block.len() > 2 {
        finish_r2004_handle_block(&mut output, &mut block)?;
    }
    let mut terminator = vec![0, 2];
    finish_r2004_handle_block(&mut output, &mut terminator)?;
    Ok(output)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn materialize_r2010_objects(objects: &[semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject]) -> Result<DwgObjectFrame, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgTableRecordBody;
    let mut seen = std::collections::BTreeSet::new();
    if objects.iter().any(|object| object.handle == 0 || !seen.insert(object.handle)) {
        return Err("AcDbObjects requires unique nonzero handles".into());
    }
    let block_names = objects
        .iter()
        .filter_map(|object| match object.body.as_ref() {
            Some(DwgLogicalObjectBody::TableRecord(DwgTableRecordBody::BlockHeader(header))) => Some((header.block_entity_handle, header.common.name.clone())),
            _ => None,
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut output = vec![0xca, 0x0d, 0x00, 0x00];
    let mut pairs = Vec::with_capacity(objects.len());
    for object in objects {
        pairs.push((object.handle, output.len()));
        output.extend_from_slice(&encode_r2010_object_frame(object, &block_names)?);
    }
    Ok((output, pairs))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_complex_color_value(value: &semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgComplexColorValue) -> u32 {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgComplexColorValue;
    match value {
        DwgComplexColorValue::ByLayer => 0xc000_0000,
        DwgComplexColorValue::ByBlock => 0xc100_0000,
        DwgComplexColorValue::ByColor { red, green, blue } => 0xc200_0000 | (u32::from(*red) << 16) | (u32::from(*green) << 8) | u32::from(*blue),
        DwgComplexColorValue::ByAci { index } => 0xc300_0000 | u32::from(*index),
        DwgComplexColorValue::ByPen { index } => 0xc400_0000 | u32::from(*index),
        DwgComplexColorValue::Foreground => 0xc500_0000,
        DwgComplexColorValue::LayerOff => 0xc600_0000,
        DwgComplexColorValue::LayerFrozen => 0xc700_0000,
        DwgComplexColorValue::None => 0xc800_0000,
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_classes_section(classes: &[semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgClass]) -> Result<Vec<u8>, String> {
    let mut data = DwgBitWriter::new();
    let maximum_class = classes.iter().map(|class| class.number).max().unwrap_or(499);
    data.write_bl(u32::from(maximum_class));
    data.write_b(true);
    let mut strings = DwgBitWriter::new();
    for class in classes {
        data.write_bs(class.number);
        data.write_bs(class.proxy_flags as u16);
        strings.write_tu(&class.application_name);
        strings.write_tu(&class.cpp_class_name);
        strings.write_tu(&class.dxf_name);
        data.write_b(class.was_zombie);
        data.write_bs(class.item_class_id);
        data.write_bl(class.object_count);
        data.write_bl(class.dwg_version);
        data.write_bl(class.maintenance_version);
        data.write_bl(class.reserved_values.first().copied().unwrap_or(0));
        data.write_bl(class.reserved_values.get(1).copied().unwrap_or(0));
    }
    let string_bits = strings.bit_len();
    data.append_bits(&strings);
    if string_bits <= 0x7fff {
        data.write_rs(string_bits as u16);
    } else {
        let high = string_bits >> 15;
        if high > u16::MAX as usize {
            return Err("R2010 class string stream exceeds extended size".into());
        }
        data.write_rs(high as u16);
        data.write_rs((string_bits as u16 & 0x7fff) | 0x8000);
    }
    data.write_b(true);
    let body_bits = data.bit_len();
    data.pad_to_byte();
    let total_bits = 32usize.checked_add(body_bits).ok_or("R2010 class section size overflow")?;
    let mut output = Vec::new();
    output.extend_from_slice(&DWG_SENTINEL_CLASSES_BEGIN);
    output.extend_from_slice(&(total_bits.div_ceil(8) as u32).to_le_bytes());
    output.extend_from_slice(&(total_bits as u32).to_le_bytes());
    output.extend_from_slice(&data.bytes);
    output.extend_from_slice(&dwg_crc16(0xC0C1, &output[16..]).to_le_bytes());
    output.extend_from_slice(&DWG_SENTINEL_CLASSES_END);
    output.extend_from_slice(&[0; 8]);
    Ok(output)
}
//#endregion DwgRead
//#endregion 🔖️DwgStructuralCodec
//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
include!("🧪️tests/🔬️standalone/🦀️.rs");
//#endregion 🧪️Tests
/// 📏️ Encodes layered planar polylines supplied by an artifact I/O conversion.
pub fn polylines_to_dwg_bytes<'a>(polylines: impl IntoIterator<Item = (&'a str, &'a [[f64; 2]], bool)>) -> Result<Vec<u8>, String> {
    let mut drawing = DwgDrawing::default();
    drawing.ensure_layer("0");
    for (name, vertices, closed) in polylines {
        let layer = drawing.ensure_layer(name);
        drawing.entities.push(DwgEntity { layer, color: DwgColor::ByLayer, geometry: DwgGeometry::LwPolyline { closed, elevation: 0.0, vertices: vertices.to_vec(), bulges: vec![0.0; vertices.len()] } });
    }
    dwg_to_bytes(&drawing)
}
#[cfg(test)]
#[path = "🧪️tests/🔬️polyline-io/🦀️.rs"]
mod polyline_io_tests;
#[cfg(test)]
#[path = "🧪️tests/🔮️acadrust-oracle/🦀️.rs"]
mod acadrust_oracle_tests;

//#region 📸️SnapshotEncode
/// 🖨️ The empty document's bytes: the preamble region, version stamp and preamble triple written in,
/// every other byte zero — byte for byte what the committed demo example already is.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_preamble_only_document(snapshot: &DwgSnapshot) -> Vec<u8> {
    let mut bytes = vec![0u8; DWG_PREAMBLE_LEN];
    bytes[0..6].copy_from_slice(snapshot.version.as_bytes());
    bytes[0x12] = snapshot.maintenance_version;
    bytes[0x13..0x15].copy_from_slice(&snapshot.codepage.to_le_bytes());
    bytes
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate_export_header(bytes: &[u8], snapshot: &DwgSnapshot) -> Result<(), DwgExportError> {
    let version = dwg_version_sentinel(bytes).map_err(DwgExportError::Writer)?;
    if snapshot.version.len() != 6 {
        return Err(DwgExportError::InvalidVersion("AC10xx sentinel must contain six ASCII bytes".into()));
    }
    if version != snapshot.version {
        return Err(DwgExportError::HeaderMismatch(format!("version {} != {}", version, snapshot.version)));
    }
    let (maintenance_version, codepage) = parse_version_header_fields(bytes).map_err(DwgExportError::Writer)?;
    if maintenance_version != snapshot.maintenance_version {
        return Err(DwgExportError::HeaderMismatch(format!("maintenance version {maintenance_version} != {}", snapshot.maintenance_version)));
    }
    if codepage != snapshot.codepage {
        return Err(DwgExportError::HeaderMismatch(format!("codepage {codepage} != {}", snapshot.codepage)));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_dwg(snapshot: &DwgSnapshot) -> Result<Vec<u8>, DwgExportError> {
    if snapshot.schema != STDIO_DWG_DOCUMENT_SCHEMA {
        return Err(DwgExportError::InvalidLogical("schema identity changed".into()));
    }
    if let Some((_, message)) = unwritable_version(snapshot) {
        return Err(DwgExportError::InvalidVersion(message));
    }
    // 🫙️ The empty document is written as the preamble and nothing else — the inverse of the read
    // above, and the only representation an R2004 container HAS for a snapshot that carries no
    // Header, no Classes and no object sections to build one from.
    let bytes = if is_preamble_only_document(snapshot) { encode_preamble_only_document(snapshot) } else { encode_r2004_snapshot(snapshot).map_err(DwgExportError::Writer)? };
    validate_export_header(&bytes, snapshot)?;
    Ok(bytes)
}
//#endregion 📸️SnapshotEncode
