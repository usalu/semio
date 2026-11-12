//! 📦️ The AC1024 file container: the 0x100-byte file header with the masked header data, section pages (32-byte masked page headers, compressed or stored), the section map, the section page map, and the second copy of the header data.
//! <https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf> (4 R2004 file format organization)

use crate::checksum::{adler, crc32, magic, mask_page_header};
use crate::lz77::compress;
use crate::sections::APPLICATION_VERSION;

const FILE_HEADER: usize = 0x100;
const DATA_PAGE: u32 = 0x4163_043B;
const SECTION_MAP: u32 = 0x4163_003B;
const PAGE_MAP: u32 = 0x4163_0E3B;
const SYSTEM_HEADER: usize = 0x14;
const CODE_PAGE_ANSI_1252: u16 = 30;
const PAGE_SIZE: usize = 0x7400;

/// 📚️ One named section of the file and how its pages are stored.
pub struct Section<'a> {
    pub name: &'a str,
    pub data: &'a [u8],
    pub compressed: bool,
    pub page: usize,
}

struct Described {
    id: u32,
    name: String,
    size: usize,
    page: usize,
    compressed: bool,
    pages: Vec<(u32, usize, usize)>,
}

fn pad(length: usize) -> usize {
    (0x20 - length % 0x20) % 0x20
}

fn push_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn data_page(file: &mut Vec<u8>, section: u32, offset: usize, payload: &[u8]) -> Result<usize, String> {
    let start = file.len();
    let size = 0x20 + payload.len() + pad(0x20 + payload.len());
    let data_checksum = adler(0, payload);
    let mut header = Vec::with_capacity(0x20);
    for value in [DATA_PAGE, section, payload.len() as u32, size as u32] {
        push_u32(&mut header, value);
    }
    header.extend_from_slice(&(offset as u64).to_le_bytes());
    push_u32(&mut header, 0);
    push_u32(&mut header, data_checksum);
    let header_checksum = adler(data_checksum, &header);
    header[0x18..0x1C].copy_from_slice(&header_checksum.to_le_bytes());
    mask_page_header(&mut header, u32::try_from(start).map_err(|_| "file larger than 4 GiB")?);
    file.extend_from_slice(&header);
    file.extend_from_slice(payload);
    file.extend_from_slice(&magic()[..pad(0x20 + payload.len())]);
    Ok(size)
}

fn system_page_size(raw: usize) -> Result<usize, String> {
    let stored = compress(&vec![0; raw])?.len();
    let used = SYSTEM_HEADER + stored + SYSTEM_HEADER;
    Ok(used + pad(used))
}

fn system_page(file: &mut Vec<u8>, kind: u32, raw: &[u8]) -> Result<usize, String> {
    let stored = compress(raw)?;
    let mut header = Vec::with_capacity(SYSTEM_HEADER);
    for value in [kind, raw.len() as u32, stored.len() as u32, 2, 0] {
        push_u32(&mut header, value);
    }
    let checksum = adler(adler(0, &header), &stored);
    header[0x10..0x14].copy_from_slice(&checksum.to_le_bytes());
    let start = file.len();
    file.extend_from_slice(&header);
    file.extend_from_slice(&stored);
    for value in [kind, 0, 0, 2, 0] {
        push_u32(file, value);
    }
    let size = file.len() - start;
    file.extend_from_slice(&magic()[..pad(size)]);
    Ok(size + pad(size))
}

struct Anchors {
    last_page: u32,
    page_map: u32,
    section_map: u32,
    page_map_address: u64,
    last_page_end: u64,
    second_header: u64,
}

fn file_header_data(anchors: &Anchors) -> Vec<u8> {
    let mut data = b"AcFssFcAJMB\0".to_vec();
    for value in [0, 0x6C, 4, 0, 0, 0, 1, anchors.last_page] {
        push_u32(&mut data, value);
    }
    data.extend_from_slice(&anchors.last_page_end.to_le_bytes());
    data.extend_from_slice(&anchors.second_header.to_le_bytes());
    for value in [0, anchors.last_page, 0x20, 0x80, 0x40, anchors.page_map] {
        push_u32(&mut data, value);
    }
    data.extend_from_slice(&anchors.page_map_address.to_le_bytes());
    for value in [anchors.section_map, anchors.last_page, 0, 0] {
        push_u32(&mut data, value);
    }
    let crc = crc32(&data);
    data[0x68..0x6C].copy_from_slice(&crc.to_le_bytes());
    data.iter().zip(magic().iter().cycle()).map(|(byte, key)| byte ^ key).collect()
}

/// 🧱️ Lays out the file: the 0x100-byte header, every section as pages in the order given, the section map and the page map, then the second header data. `summary` names the section whose address goes into the file header.
pub fn write(sections: &[Section], summary: &str) -> Result<Vec<u8>, String> {
    let mut file = vec![0u8; FILE_HEADER];
    let mut described: Vec<Described> = Vec::new();
    let mut page_sizes: Vec<usize> = Vec::new();
    let mut summary_address = 0u32;
    for (id, section) in sections.iter().enumerate() {
        let mut entry = Described { id: id as u32, name: section.name.to_string(), size: section.data.len(), page: section.page, compressed: section.compressed, pages: Vec::new() };
        let count = section.data.len().div_ceil(section.page).max(1);
        for index in 0..count {
            let from = index * section.page;
            let mut padded = section.data[from.min(section.data.len())..(from + section.page).min(section.data.len())].to_vec();
            padded.resize(section.page, 0);
            let payload = if section.compressed { compress(&padded)? } else { padded };
            if section.name == summary && index == 0 {
                summary_address = (file.len() + 0x20) as u32;
            }
            let size = data_page(&mut file, entry.id, index * section.page, &payload)?;
            page_sizes.push(size);
            entry.pages.push((page_sizes.len() as u32, payload.len(), index * section.page));
        }
        described.push(entry);
    }
    let data_pages = page_sizes.len() as u32;
    let (section_map_id, page_map_id) = (data_pages + 1, data_pages + 2);

    let mut map = Vec::new();
    for value in [described.len() as u32, 2, PAGE_SIZE as u32, 0, described.len() as u32] {
        push_u32(&mut map, value);
    }
    for entry in &described {
        map.extend_from_slice(&(entry.size as u64).to_le_bytes());
        for value in [entry.pages.len() as u32, entry.page as u32, 1, if entry.compressed { 2 } else { 1 }, entry.id, 0] {
            push_u32(&mut map, value);
        }
        let mut name = [0u8; 64];
        name[..entry.name.len()].copy_from_slice(entry.name.as_bytes());
        map.extend_from_slice(&name);
        for (number, stored, offset) in &entry.pages {
            push_u32(&mut map, *number);
            push_u32(&mut map, *stored as u32);
            map.extend_from_slice(&(*offset as u64).to_le_bytes());
        }
    }
    let section_map_size = system_page_size(map.len())?;
    let page_map_raw = 8 * page_map_id as usize;
    let page_map_size = system_page_size(page_map_raw)?;

    let mut page_map = Vec::with_capacity(page_map_raw);
    for (index, size) in page_sizes.iter().enumerate() {
        push_u32(&mut page_map, index as u32 + 1);
        push_u32(&mut page_map, *size as u32);
    }
    push_u32(&mut page_map, section_map_id);
    push_u32(&mut page_map, section_map_size as u32);
    push_u32(&mut page_map, page_map_id);
    push_u32(&mut page_map, page_map_size as u32);

    system_page(&mut file, SECTION_MAP, &map)?;
    let page_map_at = file.len();
    system_page(&mut file, PAGE_MAP, &page_map)?;

    let anchors = Anchors { last_page: page_map_id, page_map: page_map_id, section_map: section_map_id, page_map_address: (page_map_at - FILE_HEADER) as u64, last_page_end: (page_map_at + page_map_size - FILE_HEADER) as u64, second_header: file.len() as u64 };
    let data = file_header_data(&anchors);
    file.extend_from_slice(&data);

    let mut head = Vec::with_capacity(FILE_HEADER);
    head.extend_from_slice(b"AC1024");
    head.extend_from_slice(&[0; 5]);
    head.extend_from_slice(&[0, 3]);
    push_u32(&mut head, 0);
    head.extend_from_slice(&[APPLICATION_VERSION, 0]);
    head.extend_from_slice(&CODE_PAGE_ANSI_1252.to_le_bytes());
    head.extend_from_slice(&[0; 3]);
    push_u32(&mut head, 0);
    push_u32(&mut head, 0);
    push_u32(&mut head, summary_address);
    push_u32(&mut head, 0);
    push_u32(&mut head, 0x80);
    head.resize(0x80, 0);
    head.extend_from_slice(&data);
    head.extend_from_slice(&magic()[FILE_HEADER - 0x14..]);
    file[..FILE_HEADER].copy_from_slice(&head);
    Ok(file)
}
