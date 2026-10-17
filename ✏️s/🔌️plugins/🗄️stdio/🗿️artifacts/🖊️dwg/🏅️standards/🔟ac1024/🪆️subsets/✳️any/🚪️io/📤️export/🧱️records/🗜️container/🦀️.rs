//! 🗜️ The R2004 container writer: LZ77 compression, data and system pages, section descriptors and the canonical page layout.

use crate::standards::v_ac1024::subsets::any::io::export::records::bits::{push_u32, push_u64};

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
pub struct EncodedR2004Page {
    pub section_id: u32,
    pub page_number: i32,
    pub start_offset: u64,
    pub payload: Vec<u8>,
    pub address: u64,
    pub allocation_size: u32,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn align_r2004(value: usize) -> usize {
    (value + 0x1f) & !0x1f
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn r2004_crc32(data: &[u8]) -> u32 {
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
pub fn write_data_page(output: &mut Vec<u8>, page: &EncodedR2004Page) -> Result<(), String> {
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
pub struct R2004SectionDescriptor {
    pub name: &'static str,
    pub section_id: u32,
    pub semantic_size: u64,
    pub max_decompressed_size: u32,
    pub compression: u32,
    pub encryption: u32,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2004_section_info(descriptors: &[R2004SectionDescriptor], pages: &[EncodedR2004Page], reserved_name: &[u8; 64], application_history_scratch: &[u8]) -> Result<Vec<u8>, String> {
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
pub fn write_r2004_system_page(output: &mut Vec<u8>, page_type: u32, decoded: &[u8], physical_allocation: Option<usize>, fill_skip: usize) -> Result<usize, String> {
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
//#endregion 🔖️R2004Writer
//#endregion 🔖️SectionMapAndInfo
