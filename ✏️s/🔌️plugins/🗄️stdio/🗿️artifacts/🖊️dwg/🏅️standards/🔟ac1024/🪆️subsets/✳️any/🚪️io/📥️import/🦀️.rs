//! 📥️ DWG byte reader of `stdio.dwg` (ac1024/✳️any): the real AC1024/R2004+ byte-level decode pipeline (ticket 26/08/10/
//! ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, 🖊️dwg D1-D2 wave): file header
//! decrypt, section/page directory walk (D1), and the bespoke R2004+ LZ77-variant section
//! decompressor (D2). Algorithm field-layout/opcode semantics cross-checked against the
//! ODA-spec-derived LibreDWG reference (GPLv3, github.com/LibreDWG/libredwg `src/decode.c` +
//! `src/r2004_file_header.spec`) and validated end to end against the real 145KB
//! `architectural.dwg` fixture via a standalone scratch crate before landing here -- clean-room
//! reimplementation, no code copied. Undecoded/unreachable content is never fabricated: every
//! function here returns a typed `Result`; native framing and page buffers remain transient and
//! only decoded logical standard concepts cross the deserialization boundary. Pure byte<->byte
//! algorithms with no `DwgSnapshot` dependency of their own — kept here per ticket 26/08/12/ENGINELESS-ARTIFACTS-
//! AND-APP-STATE-MACHINES rule 6 ("keep with the codec in 🚪️io/").
//!
//! Depends on the model only (`schema` and the binary-representation wire vocabulary); the writer lives in the
//! sibling `export` module and never reaches this one.

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
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::geometry::DwgDrawing;
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::geometry::DwgLayer;
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalDrawing;
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgSnapshot;
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DWG_PREAMBLE_LEN;
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::STDIO_DWG_DOCUMENT_SCHEMA;

/// 🗃️ Decoded object records and their common entity mode flags.
pub type DwgObjectRecords = (Vec<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject>, Vec<(u8, u8)>);
/// 🗂️ Section metadata with page identifiers, sizes, and offsets.
pub type DwgSectionInfo = (String, u64, u32, u32, u32, u32, Vec<(i32, u32, u64)>);
/// 🔗 Decoded common object relation handles.
pub type DwgObjectRelations = (Option<u64>, Vec<u64>, Option<u64>);
//#region 🔖️R2004FileHeaderDecrypt
/// 🗂️ The decrypted 0x6c-byte R2004+ file header record (offsets per `r2004_file_header.spec`).
/// `file_id_string` must read `"AcFssFcAJMB\0"` once correctly decrypted -- the one
/// self-validating invariant that proves the LCG pad above landed on the right bytes.
#[derive(Debug, Clone, Copy)]
pub struct R2004FileHeader {
    pub file_id_string: [u8; 12],
    pub last_section_address: u64,
    pub numgaps: u32,
    pub numsections: u32,
    pub section_map_id: i32,
    pub section_map_address: u64,
    pub section_info_id: i32,
    pub section_array_size: u32,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_r2004_file_header(dec: &[u8]) -> Result<R2004FileHeader, String> {
    if dec.len() < R2004_HEADER_LEN {
        return Err(format!("r2004 file header: need {R2004_HEADER_LEN} decrypted bytes, got {}", dec.len()));
    }
    let u32_at = |o: usize| u32::from_le_bytes(dec[o..o + 4].try_into().unwrap());
    let u64_at = |o: usize| u64::from_le_bytes(dec[o..o + 8].try_into().unwrap());
    let mut file_id_string = [0u8; 12];
    file_id_string.copy_from_slice(&dec[0..12]);
    if &file_id_string[0..11] != b"AcFssFcAJMB" {
        return Err("r2004 file header: file_ID_string mismatch (LCG decrypt landed wrong -- malformed/unsupported file)".into());
    }
    Ok(R2004FileHeader {
        file_id_string,
        last_section_address: u64_at(0x2c),
        numgaps: u32_at(0x3c),
        numsections: u32_at(0x40),
        section_map_id: u32_at(0x50) as i32,
        section_map_address: u64_at(0x54),
        section_info_id: u32_at(0x5c) as i32,
        section_array_size: u32_at(0x60),
    })
}
//#endregion 🔖️R2004FileHeaderDecrypt
//#region 🔖️Lz77Variant
/// 🗜️ R2004+ "compression algorithm 2": a bespoke byte-oriented LZ77 variant (NOT DEFLATE/zlib).
/// Opcode stream of interleaved back-reference and literal-copy runs, terminated by opcode byte
/// `0x11` or source exhaustion. `decomp_size` upper-bounds the output (real files use each
/// section's generous `max_decomp_size` allocation, not a tight fit -- the meaningful content
/// ends wherever the terminator naturally falls, which may be well short of `decomp_size`).
struct ByteCursor<'a> {
    data: &'a [u8],
    pos: usize,
}
impl<'a> ByteCursor<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn u8(&mut self) -> Result<u8, String> {
        let b = *self.data.get(self.pos).ok_or("dwg lz: source exhausted mid-opcode")?;
        self.pos += 1;
        Ok(b)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn has_more(&self) -> bool {
        self.pos < self.data.len()
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_literal_length(src: &mut ByteCursor<'_>, opcode: u8) -> Result<u32, String> {
    let mut lowbits = (opcode & 0xF) as u32;
    if lowbits == 0 {
        let mut lastbyte;
        loop {
            lastbyte = src.u8()?;
            if lastbyte != 0 || !src.has_more() {
                break;
            }
            lowbits += 0xFF;
        }
        lowbits += 0xF + lastbyte as u32;
    }
    Ok(lowbits + 3)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_compressed_bytes(src: &mut ByteCursor<'_>, opcode: u8, bits: u32) -> Result<u32, String> {
    let mut cb = (opcode as u32) & bits;
    if cb == 0 {
        let mut lastbyte;
        loop {
            lastbyte = src.u8()?;
            if lastbyte != 0 || !src.has_more() {
                break;
            }
            cb += 0xFF;
        }
        cb += lastbyte as u32 + bits;
    }
    Ok(cb + 2)
}
/// ⚠️ `existing_offset` carries bits the caller already set (opcode1 in 0x10-0x1F stashes
/// `(opcode1&8)<<11` before calling this) -- they must be OR'd into the combined offset BEFORE
/// `plus` is added, not added to the two new bytes alone and OR'd in afterward. Found via the
/// standalone-scratch-crate technique: the OR-then-add-separately variant silently desyncs the
/// decompressor on real (longer, more opcode-varied) section data while still round-tripping
/// shorter synthetic streams -- exactly the class of bug this technique exists to catch.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn two_byte_offset(src: &mut ByteCursor<'_>, plus: u32, existing_offset: u32) -> Result<(u8, u32), String> {
    let first = src.u8()?;
    let second = src.u8()?;
    let offset = existing_offset | ((first as u32) >> 2) | ((second as u32) << 6);
    Ok((first, offset + plus))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn copy_bytes(n: u32, src: &mut ByteCursor<'_>, dec: &mut Vec<u8>) -> Result<u8, String> {
    for _ in 0..n {
        dec.push(src.u8()?);
    }
    src.u8()
}
/// 🗜️ Decompresses one R2004+ "compression algorithm 2" byte stream.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decompress_r2004_section(comp: &[u8], decomp_size: usize) -> Result<Vec<u8>, String> {
    let mut src = ByteCursor::new(comp);
    let mut dec: Vec<u8> = Vec::with_capacity(decomp_size.min(1 << 20));

    let mut opcode1 = src.u8()?;
    if (opcode1 & 0xF0) == 0 {
        let lit_len = read_literal_length(&mut src, opcode1)?;
        opcode1 = copy_bytes(lit_len, &mut src, &mut dec)?;
    }

    while src.has_more() && dec.len() < decomp_size && opcode1 != 0x11 {
        let (comp_bytes, comp_offset): (u32, u32);
        if !(0x10..0x40).contains(&opcode1) {
            let cb = ((opcode1 as u32) >> 4).wrapping_sub(1);
            let opcode2 = src.u8()?;
            let co = ((((opcode1 as u32) >> 2) & 3) | ((opcode2 as u32) << 2)) + 1;
            comp_bytes = cb;
            comp_offset = co;
            // opcode1 intentionally unchanged here (matches reference semantics).
        } else if opcode1 < 0x20 {
            let cb = read_compressed_bytes(&mut src, opcode1, 7)?;
            let partial = ((opcode1 as u32) & 8) << 11;
            let (next_op, co) = two_byte_offset(&mut src, 0x4000, partial)?;
            opcode1 = next_op;
            comp_bytes = cb;
            comp_offset = co;
        } else {
            debug_assert!(opcode1 >= 0x20);
            let cb = read_compressed_bytes(&mut src, opcode1, 0x1f)?;
            let (next_op, co) = two_byte_offset(&mut src, 1, 0)?;
            opcode1 = next_op;
            comp_bytes = cb;
            comp_offset = co;
        }

        let pos = dec.len();
        let end = pos + comp_bytes as usize;
        if end > decomp_size || (comp_offset as usize) > pos {
            return Err(format!("dwg lz: invalid backref bytes={comp_bytes} offset={comp_offset} pos={pos} decomp_size={decomp_size}"));
        }
        for i in pos..end {
            let b = dec[i - comp_offset as usize];
            dec.push(b);
        }

        let mut lit_length = (opcode1 & 3) as u32;
        if lit_length == 0 {
            opcode1 = src.u8()?;
            if (opcode1 & 0xF0) == 0 {
                lit_length = read_literal_length(&mut src, opcode1)?;
            }
        }
        if lit_length > 0 && end + lit_length as usize <= decomp_size {
            opcode1 = copy_bytes(lit_length, &mut src, &mut dec)?;
        } else if lit_length > 0 {
            break;
        }
    }
    Ok(dec)
}
//#endregion 🔖️Lz77Variant
//#region 🔖️PageHeaderDecrypt
/// 🔓 Per-named-section-page 32-byte encrypted header: each little-endian u32 word XORed with
/// `0x4164536b ^ file_address`. `page_type` must equal `0x4163043b` once decrypted -- the
/// self-checking invariant every real reader validates before trusting the rest of the header.
struct PageHeader {
    page_type: u32,
    data_size: u32,
    page_size: u32,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decrypt_page_header(raw32: &[u8; 32], file_address: u64) -> PageHeader {
    let mask = 0x4164536bu32 ^ (file_address as u32);
    let mut words = [0u32; 8];
    for (k, word) in words.iter_mut().enumerate() {
        let w = u32::from_le_bytes(raw32[k * 4..k * 4 + 4].try_into().unwrap());
        *word = w ^ mask;
    }
    PageHeader { page_type: words[0], data_size: words[2], page_size: words[3] }
}
//#endregion 🔖️PageHeaderDecrypt
//#region 🔖️SectionMapAndInfo
/// 📄️ One physical page in the R2004+ page directory (`number` may legitimately skip values --
/// pages aren't guaranteed contiguous in number, only in cumulative `address`).
#[derive(Debug, Clone, Copy)]
pub struct PageDirEntry {
    pub number: i32,
    pub address: u64,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_page_directory(dec: &[u8], section_array_size: u32) -> Vec<PageDirEntry> {
    let mut out = Vec::new();
    let mut pos = 0usize;
    let mut address: u64 = 0x100;
    while pos + 8 <= dec.len() {
        let number = i32::from_le_bytes(dec[pos..pos + 4].try_into().unwrap());
        let size = u32::from_le_bytes(dec[pos + 4..pos + 8].try_into().unwrap());
        pos += 8;
        let entry_address = address;
        if number <= section_array_size as i32 {
            address += size as u64;
        }
        if number < 0 && pos + 16 <= dec.len() {
            pos += 16;
        }
        out.push(PageDirEntry { number, address: entry_address });
    }
    out
}
//#region 🧱️R2004DirectoryDecode
/// 🧰 Transient page materialization used only while decoding named standard concepts.
#[derive(Debug, Clone, Default)]
pub struct DwgRawPage {
    pub page_number: i32,
    pub file_address: u64,
    pub start_offset: u64,
    /// Empty iff this page's decompression failed; never retained by an artifact snapshot.
    pub decoded: Vec<u8>,
    pub error: Option<String>,
}
/// 🗂️ One named R2004+ section (`AcDb:Header`, `AcDb:Classes`, ...) as located via the section
/// info directory.
#[derive(Debug, Clone, Default)]
pub struct DwgRawSection {
    pub name: String,
    pub compressed: bool,
    pub declared_size: u64,
    /// 📏 The section's own generous per-page decompression buffer allocation (normally
    /// `0x7400`) -- the REAL bound real readers decompress each page into (never the tighter
    /// per-page `page_size` from the page header itself, which under-bounds real content and
    /// causes spurious "invalid backref" errors mid-stream; found via the standalone-scratch-
    /// crate technique after the first engine port used `page_size` and every compressed
    /// section on the real fixture failed).
    pub max_decomp_size: u32,
    pub pages: Vec<DwgRawPage>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_section_info(dec: &[u8]) -> Result<Vec<DwgSectionInfo>, String> {
    if dec.len() < 20 {
        return Err("section info: header shorter than 20 bytes".into());
    }
    let u32_at = |o: usize| -> Result<u32, String> { dec.get(o..o + 4).map(|s| u32::from_le_bytes(s.try_into().unwrap())).ok_or_else(|| "section info: read past end".into()) };
    let u64_at = |o: usize| -> Result<u64, String> { dec.get(o..o + 8).map(|s| u64::from_le_bytes(s.try_into().unwrap())).ok_or_else(|| "section info: read past end".into()) };
    let num_desc = u32_at(0)?;
    let mut pos = 20usize;
    let mut out = Vec::with_capacity(num_desc as usize);
    for _ in 0..num_desc {
        if pos + 96 > dec.len() {
            return Err(format!("section info: descriptor at {pos} exceeds decoded length {}", dec.len()));
        }
        let size = u64_at(pos)?;
        let num_sections = u32_at(pos + 8)?;
        let max_decomp_size = u32_at(pos + 12)?;
        let compressed = u32_at(pos + 20)?;
        let section_id = u32_at(pos + 24)?;
        let encrypted = u32_at(pos + 28)?;
        let name_raw = &dec[pos + 32..pos + 32 + 64];
        let end = name_raw.iter().position(|&b| b == 0).unwrap_or(64);
        let name = String::from_utf8_lossy(&name_raw[..end]).into_owned();
        pos += 96;
        let mut pages = Vec::with_capacity(num_sections as usize);
        for _ in 0..num_sections {
            if pos + 16 > dec.len() {
                return Err(format!("section info: page record at {pos} exceeds decoded length {}", dec.len()));
            }
            let pnum = i32::from_le_bytes(dec[pos..pos + 4].try_into().unwrap());
            let psize = u32_at(pos + 4)?;
            let paddr = u64_at(pos + 8)?;
            pages.push((pnum, psize, paddr));
            pos += 16;
        }
        out.push((name, size, compressed, max_decomp_size, section_id, encrypted, pages));
    }
    Ok(out)
}
/// 🗺️ D1: decrypts the file header and walks the section-page-map + section-info directories,
/// returning every named section this file's Section Info directory declares, each with its
/// pages LOCATED (file address + compressed size) but not yet decompressed. Returns `Err` only
/// when the file structurally isn't a decodable R2004+ file (wrong magic, truncated, checksum-
/// verified-wrong LCG landing) -- never a partial/garbage result.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn locate_r2004_sections(bytes: &[u8]) -> Result<Vec<DwgRawSection>, String> {
    if bytes.len() < 0x80 + R2004_HEADER_LEN {
        return Err(format!("r2004: file too short for encrypted header ({} bytes)", bytes.len()));
    }
    let enc = &bytes[0x80..0x80 + R2004_HEADER_LEN];
    let dec_hdr = decrypt_r2004_header(enc);
    let hdr = parse_r2004_file_header(&dec_hdr)?;

    // Section page map: located at `section_map_address + 0x100`, header is 5 plain (non-XORed)
    // RL fields (0x14 bytes): section_type (must be 0x41630e3b), decomp_size, comp_size,
    // compression_type, checksum.
    let map_hdr_addr = (hdr.section_map_address + 0x100) as usize;
    if map_hdr_addr + 0x14 > bytes.len() {
        return Err(format!("r2004: section page map header at {map_hdr_addr:#x} out of bounds"));
    }
    let map_section_type = u32::from_le_bytes(bytes[map_hdr_addr..map_hdr_addr + 4].try_into().unwrap());
    if map_section_type != 0x41630e3b {
        return Err(format!("r2004: section page map signature mismatch: {map_section_type:#x} != 0x41630e3b"));
    }
    let map_decomp_size = u32::from_le_bytes(bytes[map_hdr_addr + 4..map_hdr_addr + 8].try_into().unwrap()) as usize;
    let map_comp_size = u32::from_le_bytes(bytes[map_hdr_addr + 8..map_hdr_addr + 12].try_into().unwrap()) as usize;
    let map_data_start = map_hdr_addr + 0x14;
    if map_data_start + map_comp_size > bytes.len() {
        return Err("r2004: section page map compressed data out of bounds".into());
    }
    let map_dec = decompress_r2004_section(&bytes[map_data_start..map_data_start + map_comp_size], map_decomp_size)?;
    let page_dir = parse_page_directory(&map_dec, hdr.section_array_size);
    if page_dir.len() as u32 != hdr.numgaps + hdr.numsections {
        return Err(format!("r2004: page directory entry count {} != numgaps({}) + numsections({})", page_dir.len(), hdr.numgaps, hdr.numsections));
    }

    // Section info: the named-section directory, located via the page whose `number` equals
    // `section_info_id` (looked up in the page directory we just built, not a fixed offset).
    let info_entry = page_dir.iter().find(|e| e.number == hdr.section_info_id).ok_or_else(|| format!("r2004: section_info_id {} not found in page directory", hdr.section_info_id))?;
    let info_addr = info_entry.address as usize;
    if info_addr + 0x14 > bytes.len() {
        return Err(format!("r2004: section info header at {info_addr:#x} out of bounds"));
    }
    let info_section_type = u32::from_le_bytes(bytes[info_addr..info_addr + 4].try_into().unwrap());
    if info_section_type != 0x4163003b {
        return Err(format!("r2004: section info signature mismatch: {info_section_type:#x} != 0x4163003b"));
    }
    let info_decomp_size = u32::from_le_bytes(bytes[info_addr + 4..info_addr + 8].try_into().unwrap()) as usize;
    let info_comp_size = u32::from_le_bytes(bytes[info_addr + 8..info_addr + 12].try_into().unwrap()) as usize;
    let info_data_start = info_addr + 0x14;
    if info_data_start + info_comp_size > bytes.len() {
        return Err("r2004: section info compressed data out of bounds".into());
    }
    let info_dec = decompress_r2004_section(&bytes[info_data_start..info_data_start + info_comp_size], info_decomp_size)?;
    let descriptors = parse_section_info(&info_dec)?;

    let by_number: std::collections::HashMap<i32, u64> = page_dir.iter().map(|e| (e.number, e.address)).collect();
    let mut out = Vec::with_capacity(descriptors.len());
    for (name, declared_size, compressed_flag, max_decomp_size, _section_id, _encrypted, pages) in descriptors {
        if name.is_empty() {
            continue; // padding descriptor slot (section-info headers sometimes reserve one).
        }
        let mut raw_pages = Vec::with_capacity(pages.len());
        for (pnum, _psize, start_offset) in pages {
            let file_address = *by_number.get(&pnum).ok_or_else(|| format!("r2004: page {pnum} for section {name} not in page directory"))?;
            raw_pages.push(DwgRawPage { page_number: pnum, file_address, start_offset, decoded: Vec::new(), error: None });
        }
        out.push(DwgRawSection { name, compressed: compressed_flag == 2, declared_size, max_decomp_size, pages: raw_pages });
    }
    Ok(out)
}
/// 🗜️ D2: for every section D1 located, decrypts + decompresses (or, for `compressed == false`
/// sections, copies verbatim) each page's real content bytes. A single page's failure is
/// recorded on that page (`DwgRawPage::error`) and does not abort the other pages/sections --
/// the caller can tell exactly how much of D2 landed from the per-page `error` fields.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_r2004_sections(bytes: &[u8]) -> Result<Vec<DwgRawSection>, String> {
    let mut sections = locate_r2004_sections(bytes)?;
    for section in &mut sections {
        for page in &mut section.pages {
            let addr = page.file_address as usize;
            if addr + 32 > bytes.len() {
                page.error = Some(format!("page {} address {addr:#x} + 32 exceeds file length", page.page_number));
                continue;
            }
            let mut raw32 = [0u8; 32];
            raw32.copy_from_slice(&bytes[addr..addr + 32]);
            let ph = decrypt_page_header(&raw32, page.file_address);
            if ph.page_type != 0x4163043b {
                page.error = Some(format!("page {} page_type {:#x} != 0x4163043b", page.page_number, ph.page_type));
                continue;
            }
            let comp_start = addr + 32;
            let comp_end = comp_start + ph.data_size as usize;
            if comp_end > bytes.len() {
                page.error = Some(format!("page {} compressed span exceeds file length", page.page_number));
                continue;
            }
            let comp = &bytes[comp_start..comp_end];
            if section.compressed {
                // Bound by the SECTION's generous `max_decomp_size` allocation (normally
                // 0x7400), not the page header's own tighter `page_size` -- real content
                // routinely runs well past `page_size` before hitting the LZ terminator (see
                // `DwgRawSection::max_decomp_size` docs; this under-bound was a real bug caught
                // via the standalone-scratch-crate technique).
                let bound = section.max_decomp_size.max(ph.page_size).max(1) as usize;
                match decompress_r2004_section(comp, bound) {
                    Ok(d) => page.decoded = d,
                    Err(e) => page.error = Some(e),
                }
            } else {
                page.decoded = comp.to_vec();
            }
        }
    }
    Ok(sections)
}
//#endregion 🧱️R2004DirectoryDecode
//#region 🔖️R2007Container
/// 📜️ Byte offset of an R2007 (AC1021) file's data pages: the 0x80-byte preamble plus the 0x400-byte
/// Reed–Solomon block holding the compressed file header.
const R2007_DATA_PAGES_START: usize = 0x480;
/// 🧩️ Reed–Solomon (255, `block_size`) de-interleave: the codewords of `factor` blocks are stored
/// byte-interleaved; this keeps each block's data bytes and drops its parity (the reader trusts the
/// page and does not correct it). ODA §5.3.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn r2007_deinterleave(encoded: &[u8], length: usize, factor: usize, block_size: usize) -> Result<Vec<u8>, String> {
    let mut output = Vec::with_capacity(length);
    for block in 0..factor {
        let take = (length - output.len()).min(block_size);
        for index in 0..take {
            output.push(*encoded.get(block + index * factor).ok_or("R2007 Reed-Solomon block is truncated")?);
        }
    }
    if output.len() != length {
        return Err(format!("R2007 Reed-Solomon blocks carry {} of {length} bytes", output.len()));
    }
    Ok(output)
}
/// 🔀️ How R2007 literal runs shorter than 32 bytes are stored: segments of the source run, each
/// `(source offset, destination offset, width)`, where widths 2 and 3 are byte-reversed, 16 swaps its
/// two 8-byte halves and 1, 4 and 8 copy straight. A 32-byte run is this table's 24 + 8 layout.
const R2007_LITERAL_LAYOUT: [&[(usize, usize, usize)]; 32] = [
    &[],
    &[(0, 0, 1)],
    &[(0, 0, 2)],
    &[(0, 0, 3)],
    &[(0, 0, 4)],
    &[(4, 0, 1), (0, 1, 4)],
    &[(5, 0, 1), (1, 1, 4), (0, 5, 1)],
    &[(5, 0, 2), (1, 2, 4), (0, 6, 1)],
    &[(0, 0, 8)],
    &[(8, 0, 1), (0, 1, 8)],
    &[(9, 0, 1), (1, 1, 8), (0, 9, 1)],
    &[(9, 0, 2), (1, 2, 8), (0, 10, 1)],
    &[(8, 0, 4), (0, 4, 8)],
    &[(12, 0, 1), (8, 1, 4), (0, 5, 8)],
    &[(13, 0, 1), (9, 1, 4), (1, 5, 8), (0, 13, 1)],
    &[(13, 0, 2), (9, 2, 4), (1, 6, 8), (0, 14, 1)],
    &[(0, 0, 16)],
    &[(9, 0, 8), (8, 8, 1), (0, 9, 8)],
    &[(17, 0, 1), (1, 1, 16), (0, 17, 1)],
    &[(16, 0, 3), (0, 3, 16)],
    &[(16, 0, 4), (8, 4, 8), (0, 12, 8)],
    &[(20, 0, 1), (16, 1, 4), (8, 5, 8), (0, 13, 8)],
    &[(20, 0, 2), (16, 2, 4), (8, 6, 8), (0, 14, 8)],
    &[(20, 0, 3), (16, 3, 4), (8, 7, 8), (0, 15, 8)],
    &[(16, 0, 8), (0, 8, 16)],
    &[(17, 0, 8), (16, 8, 1), (0, 9, 16)],
    &[(25, 0, 1), (17, 1, 8), (16, 9, 1), (0, 10, 16)],
    &[(25, 0, 2), (17, 2, 8), (16, 10, 1), (0, 11, 16)],
    &[(24, 0, 4), (16, 4, 8), (8, 12, 8), (0, 20, 8)],
    &[(28, 0, 1), (24, 1, 4), (16, 5, 8), (8, 13, 8), (0, 21, 8)],
    &[(28, 0, 2), (24, 2, 4), (16, 6, 8), (8, 14, 8), (0, 22, 8)],
    &[(30, 0, 1), (26, 1, 4), (18, 5, 8), (10, 13, 8), (2, 21, 8), (0, 29, 2)],
];
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn r2007_copy_literal(source: &[u8], position: &mut usize, output: &mut Vec<u8>, length: usize) -> Result<(), String> {
    let run = source.get(*position..*position + length).ok_or("R2007 literal run exceeds its page")?;
    let mut place = |chunk: &[u8], layout: &[(usize, usize, usize)]| {
        let base = output.len();
        output.resize(base + chunk.len(), 0);
        for &(from, to, width) in layout {
            let segment = &chunk[from..from + width];
            let target = &mut output[base + to..base + to + width];
            match width {
                2 | 3 => target.iter_mut().zip(segment.iter().rev()).for_each(|(slot, byte)| *slot = *byte),
                16 => {
                    target[..8].copy_from_slice(&segment[8..]);
                    target[8..].copy_from_slice(&segment[..8]);
                }
                _ => target.copy_from_slice(segment),
            }
        }
    };
    let mut chunks = run.chunks_exact(32);
    for chunk in chunks.by_ref() {
        place(chunk, &[(24, 0, 8), (16, 8, 8), (8, 16, 8), (0, 24, 8)]);
    }
    let rest = chunks.remainder();
    place(rest, R2007_LITERAL_LAYOUT[rest.len()]);
    *position += length;
    Ok(())
}
/// 🗜️ R2007 "compression algorithm 2" (a byte-oriented LZ77 distinct from the R2004 one): literal
/// runs in the reordered layout above, back-references encoded by the opcode's high nibble. The
/// output is `expected` bytes long; a stream that ends early leaves the page's zero fill.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decompress_r2007(source: &[u8], expected: usize) -> Result<Vec<u8>, String> {
    let byte = |position: &mut usize| -> Result<usize, String> {
        let value = *source.get(*position).ok_or("R2007 compressed stream underflow")? as usize;
        *position += 1;
        Ok(value)
    };
    let back_reference = |opcode: usize, position: &mut usize| -> Result<(usize, usize, usize), String> {
        Ok(match opcode >> 4 {
            0 => {
                let low = byte(position)?;
                let next = byte(position)?;
                ((opcode & 0x0f) + 0x13 + ((next >> 3) & 0x10), ((next & 0x78) << 5) + 1 + low, next)
            }
            1 => {
                let low = byte(position)?;
                let next = byte(position)?;
                ((opcode & 0x0f) + 3, ((next & 0xf8) << 5) + 1 + low, next)
            }
            2 => {
                let offset = byte(position)? | (byte(position)? << 8);
                if opcode & 8 == 0 {
                    let next = byte(position)?;
                    ((next & 0xf8) + (opcode & 7), offset, next)
                } else {
                    let high = byte(position)?;
                    let next = byte(position)?;
                    (((next & 0xf8) << 8) + (high << 3) + (opcode & 7) + 0x100, offset + 1, next)
                }
            }
            nibble => {
                let next = byte(position)?;
                (nibble, ((next & 0xf8) << 1) + (opcode & 0x0f) + 1, next)
            }
        })
    };
    let mut output = Vec::with_capacity(expected);
    let mut position = 0usize;
    let mut opcode = byte(&mut position)?;
    let mut literal = 0usize;
    if opcode & 0xf0 == 0x20 {
        position += 3;
        literal = *source.get(position - 1).ok_or("R2007 compressed stream underflow")? as usize & 7;
    }
    while position < source.len() {
        if literal == 0 {
            literal = opcode + 8;
            if literal == 0x17 {
                let extra = byte(&mut position)?;
                literal += extra;
                if extra == 0xff {
                    loop {
                        let chunk = byte(&mut position)? | (byte(&mut position)? << 8);
                        literal += chunk;
                        if chunk != 0xffff {
                            break;
                        }
                    }
                }
            }
        }
        r2007_copy_literal(source, &mut position, &mut output, literal)?;
        if position >= source.len() {
            break;
        }
        opcode = byte(&mut position)?;
        loop {
            let (length, offset, next) = back_reference(opcode, &mut position)?;
            let start = output.len().checked_sub(offset).ok_or("R2007 back-reference precedes the page")?;
            for index in 0..length {
                output.push(output[start + index]);
            }
            opcode = next;
            literal = opcode & 7;
            if literal != 0 || position >= source.len() {
                break;
            }
            opcode = byte(&mut position)?;
            if opcode >> 4 == 0 {
                break;
            }
            if opcode >> 4 == 15 {
                opcode &= 0x0f;
            }
        }
    }
    output.resize(expected, 0);
    Ok(output)
}
/// 📄️ One R2007 page: `factor` Reed–Solomon blocks of `block_size` data bytes at `address`, holding
/// `compressed` bytes that decompress to `size` (stored as-is when the two are equal).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn r2007_page(bytes: &[u8], address: usize, compressed: usize, size: usize, correction: usize, block_size: usize) -> Result<Vec<u8>, String> {
    let total = ((compressed + 7) & !7).checked_mul(correction).filter(|total| *total > 0 && *total < 1 << 28).ok_or("R2007 page size is invalid")?;
    let factor = total.div_ceil(block_size);
    let encoded = bytes.get(address..).ok_or("R2007 page address is out of bounds")?;
    let encoded = &encoded[..encoded.len().min(factor * 255)];
    let data = r2007_deinterleave(encoded, total, factor, block_size)?;
    if compressed == size {
        return Ok(data[..size].to_vec());
    }
    decompress_r2007(&data[..compressed], size)
}
/// 🗂️ The named sections of an R2007 (AC1021) file: the Reed–Solomon file header yields the page
/// map and section map, whose pages resolve every section page. ODA §5.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_r2007_sections(bytes: &[u8]) -> Result<Vec<DwgRawSection>, String> {
    let header = r2007_deinterleave(bytes.get(0x80..0x480).ok_or("R2007 file header is truncated")?, 3 * 239, 3, 239)?;
    let u64_at = |data: &[u8], offset: usize| data.get(offset..offset + 8).map(|slice| u64::from_le_bytes(slice.try_into().unwrap())).ok_or_else(|| format!("R2007 field at {offset:#x} is truncated"));
    let compressed_length = i32::from_le_bytes(header[24..28].try_into().unwrap());
    let metadata = if compressed_length < 0 {
        header.get(32..32 + 0x110).ok_or("R2007 stored header metadata is truncated")?.to_vec()
    } else {
        decompress_r2007(header.get(32..32 + compressed_length as usize).ok_or("R2007 header metadata is truncated")?, 0x110)?
    };
    let field = |index: usize| u64_at(&metadata, index * 8).map(|value| value as usize);
    let page_map = r2007_page(bytes, R2007_DATA_PAGES_START + field(7)?, field(10)?, field(11)?, field(3)?, 239)?;
    let mut pages = std::collections::HashMap::new();
    let mut address = 0usize;
    for record in page_map.chunks_exact(16) {
        let size = i64::from_le_bytes(record[..8].try_into().unwrap());
        let id = i64::from_le_bytes(record[8..].try_into().unwrap());
        if size == 0 && id == 0 {
            break;
        }
        pages.insert(id.unsigned_abs(), address);
        address += usize::try_from(size).map_err(|_| "R2007 page size is negative")?;
    }
    let section_map_address = *pages.get(&(field(24)? as u64)).ok_or("R2007 section map page is missing")?;
    let section_map = r2007_page(bytes, R2007_DATA_PAGES_START + section_map_address, field(22)?, field(25)?, field(27)?, 239)?;
    let mut sections = Vec::new();
    let mut cursor = 0usize;
    while cursor + 64 <= section_map.len() {
        let data_size = u64_at(&section_map, cursor)?;
        let max_size = u64_at(&section_map, cursor + 8)?;
        let name_length = u64_at(&section_map, cursor + 32)? as usize;
        let encoding = u64_at(&section_map, cursor + 48)?;
        let page_count = u64_at(&section_map, cursor + 56)? as usize;
        cursor += 64;
        let name_bytes = section_map.get(cursor..cursor + name_length).ok_or("R2007 section name is truncated")?;
        let name = String::from_utf16_lossy(&name_bytes.chunks_exact(2).map(|pair| u16::from_le_bytes([pair[0], pair[1]])).collect::<Vec<_>>()).trim_end_matches('\0').to_string();
        cursor += name_length;
        let mut section = DwgRawSection { name, compressed: encoding == 4, declared_size: data_size, max_decomp_size: max_size as u32, pages: Vec::with_capacity(page_count) };
        for _ in 0..page_count {
            let start_offset = u64_at(&section_map, cursor)?;
            let page_number = u64_at(&section_map, cursor + 16)?;
            let size = u64_at(&section_map, cursor + 24)? as usize;
            let compressed = u64_at(&section_map, cursor + 32)? as usize;
            cursor += 56;
            let file_address = R2007_DATA_PAGES_START + *pages.get(&page_number).ok_or_else(|| format!("R2007 page {page_number} of {} is missing", section.name))?;
            let decoded = if encoding == 1 { bytes.get(file_address..file_address + size).map(<[u8]>::to_vec).ok_or("R2007 stored page is truncated".to_string()) } else { r2007_page(bytes, file_address, compressed, size, 1, 251) };
            let (decoded, error) = match decoded {
                Ok(decoded) => (decoded, None),
                Err(error) => (Vec::new(), Some(error)),
            };
            section.pages.push(DwgRawPage { page_number: page_number as i32, file_address: file_address as u64, start_offset, decoded, error });
        }
        if !section.name.is_empty() {
            sections.push(section);
        }
    }
    Ok(sections)
}
//#endregion 🔖️R2007Container
//#endregion 🔖️SectionMapAndInfo
//#region 🔖️DwgStructuralCodec
//#region DwgObjects
pub const DWG_TYPE_LAYER: u16 = 51;
const DWG_TYPE_BLOCK: u16 = 4;
const DWG_TYPE_ENDBLK: u16 = 5;
const DWG_TYPE_INSERT: u16 = 7;
const DWG_TYPE_LINE: u16 = 19;
const DWG_TYPE_DIMENSION_LINEAR: u16 = 21;
const DWG_TYPE_VIEWPORT: u16 = 34;
const DWG_TYPE_POINT: u16 = 27;
const DWG_TYPE_CIRCLE: u16 = 18;
const DWG_TYPE_ARC: u16 = 17;
const DWG_TYPE_ELLIPSE: u16 = 35;
const DWG_TYPE_LWPOLYLINE: u16 = 77;
const DWG_TYPE_SPLINE: u16 = 36;
const DWG_TYPE_TEXT: u16 = 1;
const DWG_TYPE_FACE3D: u16 = 28;
const DWG_TYPE_POLYLINE3D: u16 = 16;
const DWG_TYPE_POLYLINE_PFACE: u16 = 29;
const DWG_TYPE_SEQEND: u16 = 6;
const DWG_TYPE_VERTEX3D: u16 = 11;
const DWG_TYPE_VERTEX_PFACE: u16 = 13;
const DWG_TYPE_VERTEX_PFACE_FACE: u16 = 14;
/// 🗂️ The object-stream layouts the drawing reader distinguishes: R2004 (AC1018) frames carry a BS
/// type and an RL data size and keep strings inline; R2007 (AC1021) keeps the RL size but moves
/// strings into a trailing string stream; R2010 (AC1024) replaces the RL size by a handle-stream
/// size in the frame prefix and a bit-pair object type; R2013+ (AC1027, AC1032) add the
/// `has_ds_data` bit to every object's common data. ODA §20.1–§20.4.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum DwgObjectLayout {
    R2004,
    R2007,
    R2010,
    R2013,
}
impl DwgObjectLayout {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn of(version: &str) -> Result<Self, String> {
        match version {
            "AC1018" => Ok(Self::R2004),
            "AC1021" => Ok(Self::R2007),
            "AC1024" => Ok(Self::R2010),
            "AC1027" | "AC1032" => Ok(Self::R2013),
            other => Err(format!("unsupported DWG version {other:?}: this codec reads AC1018, AC1021, AC1024, AC1027 and AC1032")),
        }
    }
}
/// 📦️ One object record opened out of AcDbObjects: its data stream positioned after the identity
/// handle, its string stream (R2007+), its handle stream, and the bit where its main data ends.
struct DwgObjectRecordFrame<'a> {
    layout: DwgObjectLayout,
    handle: u64,
    type_code: u16,
    data: DwgBitReader<'a>,
    strings: Option<DwgBitReader<'a>>,
    handles: DwgBitReader<'a>,
    main_end_bit: usize,
    end_bit: usize,
}
impl<'a> DwgObjectRecordFrame<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn open(layout: DwgObjectLayout, object_data: &'a [u8], handle: u64, address: usize) -> Result<Self, String> {
        let frame = object_data.get(address..).ok_or_else(|| format!("object {handle:#x} address {address:#x} is out of bounds"))?;
        let mut sizer = DwgBitReader::new(frame);
        let payload_size = sizer.read_ms().map_err(|error| format!("object {handle:#x} size: {error}"))? as usize;
        let handle_stream_bits = if layout >= DwgObjectLayout::R2010 { Some(sizer.read_umc().map_err(|error| format!("object {handle:#x} handle-stream size: {error}"))? as usize) } else { None };
        sizer.pad_to_byte();
        let payload_end = sizer.byte_pos.checked_add(payload_size).ok_or_else(|| format!("object {handle:#x} payload overflow"))?;
        let payload = frame.get(sizer.byte_pos..payload_end).ok_or_else(|| format!("object {handle:#x} payload is truncated"))?;
        let stored_crc = frame.get(payload_end..payload_end + 2).map(|bytes| u16::from_le_bytes([bytes[0], bytes[1]])).ok_or_else(|| format!("object {handle:#x} CRC is truncated"))?;
        if dwg_crc16(0xC0C1, &frame[..payload_end]) != stored_crc {
            return Err(format!("object {handle:#x} CRC mismatch"));
        }
        let end_bit = payload_size * 8;
        let mut data = DwgBitReader::new(payload);
        let type_code = if layout >= DwgObjectLayout::R2010 { data.read_bot() } else { data.read_bs() }.map_err(|error| format!("object {handle:#x} type: {error}"))?;
        let handle_start = match handle_stream_bits {
            Some(bits) => end_bit.checked_sub(bits).ok_or_else(|| format!("object {handle:#x} handle stream exceeds its payload"))?,
            None => data.read_rl().map_err(|error| format!("object {handle:#x} data size: {error}"))? as usize,
        };
        if handle_start > end_bit {
            return Err(format!("object {handle:#x} data size exceeds its payload"));
        }
        let (_, encoded) = data.read_handle().map_err(|error| format!("object {handle:#x} identity: {error}"))?;
        if encoded != handle {
            return Err(format!("object map handle {handle:#x} does not match encoded handle {encoded:#x}"));
        }
        let (strings, main_end_bit) = if layout >= DwgObjectLayout::R2007 {
            let (reader, start) = r2010_string_stream(payload, handle_start).map_err(|error| format!("object {handle:#x} string stream: {error}"))?;
            (Some(reader), start)
        } else {
            (None, handle_start)
        };
        let handles = DwgBitReader::at_bit(payload, handle_start).map_err(|error| format!("object {handle:#x} handle stream: {error}"))?;
        Ok(Self { layout, handle, type_code, data, strings, handles, main_end_bit, end_bit })
    }

    /// 📝️ A text field: UTF-16 from the string stream (R2007+) or a code-page string inline (R2004).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn text(&mut self) -> Result<String, String> {
        match self.strings.as_mut() {
            Some(strings) => strings.read_tu(),
            None => self.data.read_tv(),
        }
    }

    /// ⏭️ Steps over extended entity data by its declared record sizes.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn skip_extended_data(&mut self) -> Result<(), String> {
        loop {
            let size = self.data.read_bs()? as usize;
            if size == 0 {
                return Ok(());
            }
            self.data.read_handle()?;
            self.data.skip_bits(size.checked_mul(8).ok_or("EED size overflow")?)?;
        }
    }

    /// 🧾️ Closes the main stream: every entity field was consumed exactly up to the string stream.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn finish_main(&self, class_name: &str) -> Result<(), String> {
        if self.data.bit_position() != self.main_end_bit {
            return Err(format!("{class_name} {:#x} main stream ends at bit {} instead of {}", self.handle, self.data.bit_position(), self.main_end_bit));
        }
        Ok(())
    }
}
/// 🧬️ The entity common data of `layout`, after the EED. A preview graphic is skipped when
/// `skip_graphic` (the drawing projection has nothing to keep it in) and refused otherwise.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_entity_common(data: &mut DwgBitReader<'_>, layout: DwgObjectLayout, skip_graphic: bool) -> Result<DwgDecodedEntityCommon, String> {
    if data.read_b()? {
        if !skip_graphic {
            return Err("entity graphic requires a typed semantic graphic model".into());
        }
        let byte_count = if layout >= DwgObjectLayout::R2010 { data.read_bll()? } else { u64::from(data.read_rl()?) };
        data.skip_bits(usize::try_from(byte_count).ok().and_then(|bytes| bytes.checked_mul(8)).ok_or("entity graphic length overflow")?)?;
    }
    decode_entity_common_fields(data, layout)
}
/// ✏️ Every drawing entity kind this codec models, decoded from `frame` after its EED: the typed
/// body plus the entity's owner, reactors and extension dictionary. `None` for any other type, and
/// for fit-point splines, which carry no control polygon to model. The one reading of these
/// layouts, shared by the drawing projection and the lossless AC1024 decoder. ODA §20.4.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_entity_body(frame: &mut DwgObjectRecordFrame<'_>, skip_graphic: bool) -> Result<Option<(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityBody, DwgObjectRelations)>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::*;
    let class_name = match frame.type_code {
        DWG_TYPE_LINE => "LINE",
        DWG_TYPE_ARC => "ARC",
        DWG_TYPE_CIRCLE => "CIRCLE",
        DWG_TYPE_POINT => "POINT",
        DWG_TYPE_ELLIPSE => "ELLIPSE",
        DWG_TYPE_TEXT => "TEXT",
        DWG_TYPE_SPLINE => "SPLINE",
        DWG_TYPE_FACE3D => "3DFACE",
        DWG_TYPE_LWPOLYLINE => "LWPOLYLINE",
        DWG_TYPE_POLYLINE3D => "POLYLINE_3D",
        DWG_TYPE_POLYLINE_PFACE => "POLYLINE_PFACE",
        DWG_TYPE_VERTEX3D => "VERTEX_3D",
        DWG_TYPE_VERTEX_PFACE => "VERTEX_PFACE",
        DWG_TYPE_VERTEX_PFACE_FACE => "VERTEX_PFACE_FACE",
        DWG_TYPE_SEQEND => "SEQEND",
        _ => return Ok(None),
    };
    let handle = frame.handle;
    let context = |error: String| format!("{class_name} {handle:#x}: {error}");
    let mut decoded = decode_entity_common(&mut frame.data, frame.layout, skip_graphic).map_err(context)?;
    let data = &mut frame.data;
    let mut owned_count = 0usize;
    let body = match frame.type_code {
        DWG_TYPE_LINE => {
            let z_is_zero = data.read_b()?;
            let start_x = data.read_rd()?;
            let end_x = data.read_dd(start_x)?;
            let start_y = data.read_rd()?;
            let end_y = data.read_dd(start_y)?;
            let (start_z, end_z) = if z_is_zero {
                (0.0, 0.0)
            } else {
                let start = data.read_rd()?;
                (start, data.read_dd(start)?)
            };
            DwgEntityBody::Line(DwgLineEntity { common: DwgEntityCommon::default(), start: vec![start_x, start_y, start_z], end: vec![end_x, end_y, end_z], thickness: data.read_bt()?, extrusion: data.read_be()?.to_vec() })
        }
        DWG_TYPE_ARC => DwgEntityBody::Arc(DwgArcEntity {
            common: DwgEntityCommon::default(),
            center: data.read_3bd()?.to_vec(),
            radius: data.read_bd()?,
            thickness: data.read_bt()?,
            extrusion: data.read_be()?.to_vec(),
            start_angle: data.read_bd()?,
            end_angle: data.read_bd()?,
        }),
        DWG_TYPE_CIRCLE => DwgEntityBody::Circle(DwgCircleEntity { common: DwgEntityCommon::default(), center: data.read_3bd()?.to_vec(), radius: data.read_bd()?, thickness: data.read_bt()?, extrusion: data.read_be()?.to_vec() }),
        DWG_TYPE_POINT => DwgEntityBody::Point(DwgPointEntity { common: DwgEntityCommon::default(), point: data.read_3bd()?.to_vec(), thickness: data.read_bt()?, extrusion: data.read_be()?.to_vec(), x_axis_angle: data.read_bd()? }),
        DWG_TYPE_ELLIPSE => DwgEntityBody::Ellipse(DwgEllipseEntity {
            common: DwgEntityCommon::default(),
            center: data.read_3bd()?.to_vec(),
            major_axis: data.read_3bd()?.to_vec(),
            extrusion: data.read_3bd()?.to_vec(),
            axis_ratio: data.read_bd()?,
            start_parameter: data.read_bd()?,
            end_parameter: data.read_bd()?,
        }),
        DWG_TYPE_TEXT => {
            let flags = data.read_rc()?;
            let elevation = if flags & 0x01 == 0 { data.read_rd()? } else { 0.0 };
            let insertion = data.read_2rd()?;
            let alignment = if flags & 0x02 == 0 { Some(vec![data.read_dd(insertion[0])?, data.read_dd(insertion[1])?]) } else { None };
            let extrusion = data.read_be()?.to_vec();
            let thickness = data.read_bt()?;
            let oblique_angle = if flags & 0x04 == 0 { data.read_rd()? } else { 0.0 };
            let rotation = if flags & 0x08 == 0 { data.read_rd()? } else { 0.0 };
            let height = data.read_rd()?;
            let width_factor = if flags & 0x10 == 0 { data.read_rd()? } else { 1.0 };
            let value = frame.text().map_err(context)?;
            let data = &mut frame.data;
            let generation = if flags & 0x20 == 0 { data.read_bs()? } else { 0 };
            let horizontal_alignment = if flags & 0x40 == 0 { data.read_bs()? } else { 0 };
            let vertical_alignment = if flags & 0x80 == 0 { data.read_bs()? } else { 0 };
            DwgEntityBody::Text(DwgTextEntity {
                common: DwgEntityCommon::default(),
                elevation,
                insertion: insertion.to_vec(),
                alignment,
                extrusion,
                thickness,
                oblique_angle,
                rotation,
                height,
                width_factor,
                value,
                generation,
                horizontal_alignment,
                vertical_alignment,
                style_handle: 0,
            })
        }
        DWG_TYPE_SPLINE => {
            let mut scenario = data.read_bl()?;
            if frame.layout >= DwgObjectLayout::R2013 {
                let flags = data.read_bl()?;
                let knot_parameter = data.read_bl()?;
                scenario = if knot_parameter == 15 || flags & 8 == 0 { 1 } else { 2 };
            }
            let degree = data.read_bl()?;
            if scenario != 1 {
                return Ok(None);
            }
            let rational = data.read_b()?;
            let closed = data.read_b()?;
            let periodic = data.read_b()?;
            let knot_tolerance = data.read_bd()?;
            let control_tolerance = data.read_bd()?;
            let knot_count = data.read_bl()? as usize;
            let point_count = data.read_bl()? as usize;
            let weighted = data.read_b()?;
            if knot_count > 1 << 20 || point_count > 1 << 20 {
                return Err(context(format!("{knot_count} knots / {point_count} control points exceed the codec limit")));
            }
            let knots = (0..knot_count).map(|_| data.read_bd()).collect::<Result<Vec<_>, _>>()?;
            let mut control_points = Vec::with_capacity(point_count * 3);
            let mut weights = Vec::new();
            for _ in 0..point_count {
                control_points.extend(data.read_3bd()?);
                if weighted {
                    weights.push(data.read_bd()?);
                }
            }
            DwgEntityBody::Spline(DwgSplineEntity { common: DwgEntityCommon::default(), degree, rational, closed, periodic, knot_tolerance, control_tolerance, knots, control_points, weights })
        }
        DWG_TYPE_FACE3D => {
            let has_no_flags = data.read_b()?;
            let z_is_zero = data.read_b()?;
            let mut corners = vec![data.read_rd()?, data.read_rd()?, if z_is_zero { 0.0 } else { data.read_rd()? }];
            for corner in 1..4 {
                let previous = (corner - 1) * 3;
                let x = data.read_dd(corners[previous])?;
                let y = data.read_dd(corners[previous + 1])?;
                let z = if z_is_zero { 0.0 } else { data.read_dd(corners[previous + 2])? };
                corners.extend([x, y, z]);
            }
            let invisible_edges = if has_no_flags { 0 } else { data.read_bs()? };
            DwgEntityBody::Face3d(DwgFace3dEntity { common: DwgEntityCommon::default(), corners, invisible_edges })
        }
        DWG_TYPE_LWPOLYLINE => {
            let flags = data.read_bs()?;
            let constant_width = if flags & 4 != 0 { Some(data.read_bd()?) } else { None };
            let elevation = if flags & 8 != 0 { data.read_bd()? } else { 0.0 };
            let thickness = if flags & 2 != 0 { data.read_bd()? } else { 0.0 };
            let extrusion = if flags & 1 != 0 { data.read_3bd()? } else { [0.0, 0.0, 1.0] };
            let vertex_count = data.read_bl()? as usize;
            if vertex_count == 0 || vertex_count > 1 << 20 {
                return Err(context(format!("vertex count {vertex_count} is invalid")));
            }
            let bulge_count = if flags & 16 != 0 { data.read_bl()? as usize } else { 0 };
            let vertex_id_count = if flags & 1024 != 0 { data.read_bl()? as usize } else { 0 };
            let width_count = if flags & 32 != 0 { data.read_bl()? as usize } else { 0 };
            for (name, count) in [("bulge", bulge_count), ("vertex ID", vertex_id_count), ("width", width_count)] {
                if count != 0 && count != vertex_count {
                    return Err(context(format!("{name} count {count} differs from {vertex_count} vertices")));
                }
            }
            let mut points = Vec::with_capacity(vertex_count);
            points.push(data.read_2rd()?);
            while points.len() < vertex_count {
                let previous: [f64; 2] = points[points.len() - 1];
                points.push([data.read_dd(previous[0])?, data.read_dd(previous[1])?]);
            }
            let bulges = (0..bulge_count).map(|_| data.read_bd()).collect::<Result<Vec<_>, _>>()?;
            let vertex_ids = (0..vertex_id_count).map(|_| data.read_bl()).collect::<Result<Vec<_>, _>>()?;
            let widths = (0..width_count).map(|_| Ok((data.read_bd()?, data.read_bd()?))).collect::<Result<Vec<_>, String>>()?;
            DwgEntityBody::LwPolyline(DwgLwPolylineEntity {
                common: DwgEntityCommon::default(),
                closed: flags & 512 != 0,
                constant_width,
                elevation,
                thickness,
                extrusion: extrusion.to_vec(),
                vertices: points
                    .into_iter()
                    .enumerate()
                    .map(|(index, point)| DwgLwPolylineVertex {
                        point: point.to_vec(),
                        bulge: bulges.get(index).copied().unwrap_or_default(),
                        vertex_id: vertex_ids.get(index).copied(),
                        start_width: widths.get(index).map(|width| width.0),
                        end_width: widths.get(index).map(|width| width.1),
                    })
                    .collect(),
            })
        }
        DWG_TYPE_POLYLINE3D => {
            let curve_type = data.read_rc()?;
            let flags = data.read_rc()?;
            owned_count = data.read_bl()? as usize;
            DwgEntityBody::Polyline3d(DwgPolyline3dEntity { common: DwgEntityCommon::default(), curve_type, flags, vertex_handles: Vec::new(), sequence_end_handle: 0 })
        }
        DWG_TYPE_POLYLINE_PFACE => {
            let vertex_count = data.read_bs()?;
            let face_count = data.read_bs()?;
            owned_count = data.read_bl()? as usize;
            DwgEntityBody::PolyfaceMesh(DwgPolyfaceMeshEntity { common: DwgEntityCommon::default(), vertex_count, face_count, vertex_handles: Vec::new(), sequence_end_handle: 0 })
        }
        DWG_TYPE_VERTEX3D | DWG_TYPE_VERTEX_PFACE => DwgEntityBody::Vertex(DwgVertexEntity { common: DwgEntityCommon::default(), flags: data.read_rc()?, point: data.read_3bd()?.to_vec() }),
        DWG_TYPE_VERTEX_PFACE_FACE => DwgEntityBody::PolyfaceFace(DwgPolyfaceFaceEntity { common: DwgEntityCommon::default(), indices: (0..4).map(|_| data.read_bs().map(|value| value as i16)).collect::<Result<_, _>>()? }),
        _ => DwgEntityBody::SequenceEnd(DwgSequenceEndEntity { common: DwgEntityCommon::default() }),
    };
    frame.finish_main(class_name)?;
    let relations = decode_r2010_entity_common_handles(&mut decoded, &mut frame.handles, handle).map_err(context)?;
    let mut body = body;
    match &mut body {
        DwgEntityBody::Text(text) => text.style_handle = read_object_handle(&mut frame.handles, handle).map_err(context)?.unwrap_or_default(),
        DwgEntityBody::Polyline3d(DwgPolyline3dEntity { vertex_handles, sequence_end_handle, .. }) | DwgEntityBody::PolyfaceMesh(DwgPolyfaceMeshEntity { vertex_handles, sequence_end_handle, .. }) => {
            if owned_count > 1 << 20 {
                return Err(context(format!("{owned_count} owned vertices exceed the codec limit")));
            }
            for index in 0..owned_count {
                vertex_handles.push(read_object_handle(&mut frame.handles, handle).map_err(context)?.ok_or_else(|| context(format!("owned vertex {index} is null")))?);
            }
            *sequence_end_handle = read_object_handle(&mut frame.handles, handle).map_err(context)?.ok_or_else(|| context("SEQEND handle is null".into()))?;
        }
        _ => {}
    }
    *body.common_mut() = decoded.logical;
    Ok(Some((body, relations)))
}
/// 🗂️ A LAYER record reduced to what the drawing projection keeps: its name and ACI colour (the
/// colour index of an `ByAci` complex colour, else the record's legacy index).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_layer_projection(frame: &mut DwgObjectRecordFrame<'_>) -> Result<DwgLayer, String> {
    frame.data.read_bl()?;
    frame.data.read_b()?;
    if frame.layout >= DwgObjectLayout::R2013 {
        frame.data.read_b()?;
    }
    let name = frame.text()?;
    frame.data.read_b()?;
    frame.data.read_bs()?;
    frame.data.read_b()?;
    frame.data.read_bs()?;
    let index = frame.data.read_bs()?;
    let rgb = frame.data.read_bl()?;
    let color = if rgb >> 24 == 0xc3 { (rgb & 0xff) as u8 } else { (index & 0xff) as u8 };
    Ok(DwgLayer { name, color })
}
/// 📐️ Reads the drawing of a DWG file of any version this codec knows ([`DwgObjectLayout::of`]):
/// every LAYER record and every modelled entity, projected through the same logical drawing the
/// lossless AC1024 decoder produces (so POLYLINE_3D / POLYLINE_PFACE gather their vertex records).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dwg_from_bytes(bytes: &[u8]) -> Result<DwgDrawing, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLayerTableRecord;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalDrawing;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalLayer;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgObjectCategory;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgTableRecordBody;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgTableRecordCommon;
    let version = String::from_utf8_lossy(bytes.get(0..6).ok_or("DWG file is shorter than its version sentinel")?).to_string();
    let layout = DwgObjectLayout::of(&version)?;
    let sections = if layout == DwgObjectLayout::R2007 { decode_r2007_sections(bytes)? } else { decode_r2004_sections(bytes)? };
    let handles = sections.iter().find(|section| section.name == "AcDb:Handles").ok_or("DWG Handles section missing")?;
    let objects = sections.iter().find(|section| section.name == "AcDb:AcDbObjects").ok_or("DWG AcDbObjects section missing")?;
    let handle_map = decode_r2004_handle_map(&r2004_section_data(handles)?)?;
    let object_data = r2004_section_data(objects)?;
    let mut layers = Vec::new();
    let mut logical_objects = Vec::new();
    for (handle, address) in handle_map {
        let mut frame = DwgObjectRecordFrame::open(layout, &object_data, handle, address)?;
        if frame.type_code != DWG_TYPE_LAYER && !matches!(frame.type_code, 1 | 6 | 11 | 13 | 14 | 16..=19 | 27..=29 | 35 | 36 | 77) {
            continue;
        }
        frame.skip_extended_data().map_err(|error| format!("object {handle:#x} extended data: {error}"))?;
        if frame.type_code == DWG_TYPE_LAYER {
            let layer = decode_layer_projection(&mut frame).map_err(|error| format!("LAYER {handle:#x}: {error}"))?;
            logical_objects.push(DwgLogicalObject {
                handle,
                type_code: DWG_TYPE_LAYER,
                class_name: "LAYER".into(),
                category: DwgObjectCategory::TableRecord,
                body: Some(DwgLogicalObjectBody::TableRecord(DwgTableRecordBody::Layer(DwgLayerTableRecord { common: DwgTableRecordCommon { name: layer.name.clone(), ..Default::default() }, ..Default::default() }))),
                ..Default::default()
            });
            layers.push(DwgLogicalLayer { name: layer.name, color: layer.color });
        } else if let Some((body, (owner_handle, _, _))) = decode_entity_body(&mut frame, true)? {
            logical_objects.push(DwgLogicalObject { handle, type_code: frame.type_code, category: DwgObjectCategory::Entity, owner_handle, body: Some(DwgLogicalObjectBody::Entity(body)), ..Default::default() });
        }
    }
    if layers.is_empty() {
        layers.push(DwgLogicalLayer { name: "0".into(), color: 7 });
    }
    let mut drawing = DwgLogicalDrawing { layers, objects: logical_objects, extmin: vec![0.0; 3], extmax: vec![0.0; 3] }.to_native()?;
    drawing.recompute_extents();
    Ok(drawing)
}
//#endregion DwgObjects
//#region DwgRead
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn r2004_section_data(section: &DwgRawSection) -> Result<Vec<u8>, String> {
    let mut data = vec![0; section.declared_size as usize];
    for page in &section.pages {
        if let Some(error) = &page.error {
            return Err(format!("section {} page {}: {error}", section.name, page.page_number));
        }
        let start = page.start_offset as usize;
        if start >= data.len() {
            continue;
        }
        let length = page.decoded.len().min(data.len() - start);
        data[start..start + length].copy_from_slice(&page.decoded[..length]);
    }
    Ok(data)
}
struct DwgSectionCursor<'a> {
    bytes: &'a [u8],
    position: usize,
}
impl<'a> DwgSectionCursor<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn take(&mut self, length: usize) -> Result<&'a [u8], String> {
        let end = self.position.checked_add(length).ok_or("DWG section cursor overflow")?;
        let bytes = self.bytes.get(self.position..end).ok_or_else(|| format!("DWG section value at {} needs {length} bytes, only {} remain", self.position, self.bytes.len().saturating_sub(self.position)))?;
        self.position = end;
        Ok(bytes)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn u16(&mut self) -> Result<u16, String> {
        Ok(u16::from_le_bytes(self.take(2)?.try_into().unwrap()))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn u8(&mut self) -> Result<u8, String> {
        Ok(self.take(1)?[0])
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn i32(&mut self) -> Result<i32, String> {
        Ok(i32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn u32(&mut self) -> Result<u32, String> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn u64(&mut self) -> Result<u64, String> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn utf16_z(&mut self) -> Result<String, String> {
        let count = usize::from(self.u16()?);
        let units = self.take(count.checked_mul(2).ok_or("DWG UTF-16 length overflow")?)?.as_chunks::<2>().0.iter().map(|unit| u16::from_le_bytes([unit[0], unit[1]])).take_while(|unit| *unit != 0).collect::<Vec<_>>();
        String::from_utf16(&units).map_err(|error| format!("invalid DWG UTF-16 string: {error}"))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn utf16_bytes(&mut self) -> Result<String, String> {
        let byte_count = self.u32()? as usize;
        if !byte_count.is_multiple_of(2) {
            return Err("DWG UTF-16 byte string has an odd length".into());
        }
        let units = self.take(byte_count)?.as_chunks::<2>().0.iter().map(|unit| u16::from_le_bytes([unit[0], unit[1]])).collect::<Vec<_>>();
        String::from_utf16(&units).map_err(|error| format!("invalid DWG UTF-16 byte string: {error}"))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn bytes_z(&mut self) -> Result<String, String> {
        let count = usize::from(self.u16()?);
        let bytes = self.take(count)?;
        Ok(String::from_utf8_lossy(bytes.strip_suffix(&[0]).unwrap_or(bytes)).into_owned())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn has_more(&self) -> bool {
        self.position < self.bytes.len()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn finish(self, section: &str) -> Result<(), String> {
        if self.position == self.bytes.len() {
            Ok(())
        } else {
            Err(format!("{section} has {} trailing bytes", self.bytes.len() - self.position))
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn checksum_text(bytes: &[u8]) -> String {
    bytes.iter().enumerate().map(|(index, byte)| format!("{}{:02x}", if [4, 6, 8, 10].contains(&index) { "-" } else { "" }, byte)).collect::<Vec<_>>().join("")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_summary_info(bytes: &[u8]) -> Result<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgSummaryInfo, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgCustomProperty;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgJulianDate;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgSummaryInfo;
    let mut cursor = DwgSectionCursor::new(bytes);
    let title = cursor.utf16_z()?;
    let subject = cursor.utf16_z()?;
    let author = cursor.utf16_z()?;
    let keywords = cursor.utf16_z()?;
    let comments = cursor.utf16_z()?;
    let last_saved_by = cursor.utf16_z()?;
    let revision_number = cursor.utf16_z()?;
    let hyperlink_base = cursor.utf16_z()?;
    let total_editing_time = cursor.u64()?;
    let created_at = DwgJulianDate { days: cursor.u32()?, milliseconds: cursor.u32()? };
    let modified_at = DwgJulianDate { days: cursor.u32()?, milliseconds: cursor.u32()? };
    let property_count = usize::from(cursor.u16()?);
    let mut custom_properties = Vec::with_capacity(property_count);
    for _ in 0..property_count {
        custom_properties.push(DwgCustomProperty { key: cursor.utf16_z()?, value: cursor.utf16_z()? });
    }
    let _reserved_one = cursor.u32()?;
    let _reserved_two = cursor.u32()?;
    Ok(DwgSummaryInfo { title, subject, author, keywords, comments, last_saved_by, revision_number, hyperlink_base, total_editing_time, created_at, modified_at, custom_properties })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_application_info(bytes: &[u8]) -> Result<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgApplicationInfo, String> {
    let mut cursor = DwgSectionCursor::new(bytes);
    let _format = cursor.u32()?;
    let name = cursor.utf16_z()?;
    let _field_count = cursor.u32()?;
    let version_checksum = checksum_text(cursor.take(16)?);
    let version = cursor.utf16_z()?;
    let comment_checksum = checksum_text(cursor.take(16)?);
    let comment = cursor.utf16_z()?;
    let product_checksum = checksum_text(cursor.take(16)?);
    let product = cursor.utf16_z()?;
    let application_version = if cursor.has_more() { cursor.bytes_z()? } else { String::new() };
    Ok(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgApplicationInfo { name, version_checksum, version, comment_checksum, comment, product_checksum, product, application_version })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_dependencies(bytes: &[u8]) -> Result<Vec<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDependency>, String> {
    let mut cursor = DwgSectionCursor::new(bytes);
    let feature_count = cursor.u32()? as usize;
    let mut features = Vec::with_capacity(feature_count);
    for _ in 0..feature_count {
        features.push(cursor.utf16_bytes()?);
    }
    let file_count = cursor.u32()? as usize;
    let mut dependencies = Vec::with_capacity(file_count);
    for _ in 0..file_count {
        let full_path = cursor.utf16_bytes()?;
        let relative_path = cursor.utf16_bytes()?;
        let fingerprint = cursor.utf16_bytes()?;
        let version = cursor.utf16_bytes()?;
        let feature_index = cursor.u32()? as usize;
        let timestamp = cursor.u32()?;
        let file_size = cursor.u32()?;
        let affects_graphics = cursor.u16()? != 0;
        let reference_count = cursor.u32()?;
        let feature = features.get(feature_index).cloned().ok_or("DWG dependency feature index is out of bounds")?;
        dependencies.push(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDependency { feature, full_path, relative_path, fingerprint, version, timestamp, file_size, affects_graphics, reference_count });
    }
    Ok(dependencies)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_template(bytes: &[u8]) -> Result<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgTemplate, String> {
    let mut cursor = DwgSectionCursor::new(bytes);
    let description = cursor.utf16_z()?;
    let measurement = match cursor.u16()? {
        0 => semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgMeasurement::English,
        1 => semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgMeasurement::Metric,
        value => return Err(format!("unsupported DWG measurement value {value}")),
    };
    Ok(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgTemplate { description, measurement })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_auxiliary_header(bytes: &[u8]) -> Result<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAuxiliaryHeader, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAuxiliaryHeader;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgCompatibilityProfile;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgJulianDate;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgVersionStamp;
    let mut cursor = DwgSectionCursor::new(bytes);
    if [cursor.u8()?, cursor.u8()?, cursor.u8()?] != [255, 119, 1] {
        return Err("unsupported auxiliary-header intro".into());
    }
    let version = cursor.u16()?;
    let maintenance = cursor.u16()?;
    if version != 29 || maintenance != 2 {
        return Err(format!("unsupported auxiliary-header target {version}.{maintenance}"));
    }
    let total_saves = cursor.u32()?;
    if cursor.i32()? != -1 {
        return Err("auxiliary-header minus-one marker changed".into());
    }
    let save_partition_one = cursor.u16()?;
    let save_partition_two = cursor.u16()?;
    let save_generation = cursor.u32()?;
    let legacy_stamp_one = DwgVersionStamp { version: cursor.u16()?, maintenance: cursor.u16()? };
    let legacy_stamp_two = DwgVersionStamp { version: cursor.u16()?, maintenance: cursor.u16()? };
    let profile_shorts = [cursor.u16()?, cursor.u16()?, cursor.u16()?, cursor.u16()?, cursor.u16()?, cursor.u16()?];
    let profile_longs = [cursor.u32()?, cursor.u32()?, cursor.u32()?, cursor.u32()?, cursor.u32()?];
    if profile_shorts != [4, 1381, 261, 2600, 0, 1] || profile_longs != [0, 0, 0, 16_908_544, 65_538] {
        return Err("unsupported auxiliary-header compatibility profile".into());
    }
    let created_at = DwgJulianDate { days: cursor.u32()?, milliseconds: cursor.u32()? };
    let updated_at = DwgJulianDate { days: cursor.u32()?, milliseconds: cursor.u32()? };
    let handle_seed = cursor.u64()?;
    if cursor.u16()? != 0 {
        return Err("auxiliary-header reserved handle marker changed".into());
    }
    let terminal_save_generation = cursor.u16()?;
    let terminal = [cursor.u32()?, cursor.u32()?, cursor.u32()?, cursor.u32()?, cursor.u32()?, cursor.u32()?, cursor.u32()?, cursor.u32()?];
    if terminal != [0, 0, 0, total_saves, 0, 0, 0, 0] {
        return Err("auxiliary-header terminal profile changed".into());
    }
    cursor.finish("AcDb:AuxHeader")?;
    Ok(DwgAuxiliaryHeader {
        total_saves,
        save_partition_one,
        save_partition_two,
        save_generation,
        legacy_stamp_one,
        legacy_stamp_two,
        compatibility_profile: DwgCompatibilityProfile::Autocad2009,
        created_at,
        updated_at,
        handle_seed,
        terminal_save_generation,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_revision_history(bytes: &[u8]) -> Result<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgRevisionHistory, String> {
    let mut cursor = DwgSectionCursor::new(bytes);
    let format_major = cursor.u32()?;
    let format_minor = cursor.u32()?;
    let count = cursor.u32()? as usize;
    let mut revisions = Vec::with_capacity(count);
    for _ in 0..count {
        revisions.push(cursor.u32()?);
    }
    cursor.finish("AcDb:RevHistory")?;
    Ok(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgRevisionHistory { format_major, format_minor, revisions })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_indexed_preview(bytes: &[u8]) -> Result<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgIndexedPreview, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgIndexedPreview;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgPreviewOrigin;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgRgba;
    let mut cursor = DwgSectionCursor::new(bytes);
    if cursor.take(16)? != DWG_PREVIEW_BEGIN {
        return Err("preview start sentinel changed".into());
    }
    if cursor.u32()? as usize != bytes.len() - 36 || cursor.u8()? != 2 {
        return Err("preview record envelope changed".into());
    }
    let (code_one, start_one, size_one) = (cursor.u8()?, cursor.u32()?, cursor.u32()?);
    let (code_two, start_two, size_two) = (cursor.u8()?, cursor.u32()?, cursor.u32()?);
    if (code_one, size_one, code_two) != (1, 80, 2) || start_two != start_one + 80 || size_two as usize != bytes.len().saturating_sub(16 + 4 + 1 + 18 + 80 + 16) {
        return Err("unsupported preview record table".into());
    }
    if cursor.take(80)?.iter().any(|byte| *byte != 0) {
        return Err("preview header record changed".into());
    }
    let header_size = cursor.u32()?;
    let width = cursor.i32()?;
    let height = cursor.i32()?;
    let planes = cursor.u16()?;
    let depth = cursor.u16()?;
    let compression = cursor.u32()?;
    let image_size = cursor.u32()?;
    let x_resolution = cursor.i32()?;
    let y_resolution = cursor.i32()?;
    let colors = cursor.u32()?;
    let important = cursor.u32()?;
    if header_size != 40 || width < 0 || height < 0 || planes != 1 || depth != 8 || compression != 0 || x_resolution != 0 || y_resolution != 0 || colors != 256 || important != 0 {
        return Err("unsupported indexed preview bitmap header".into());
    }
    let mut palette = Vec::with_capacity(256);
    for _ in 0..256 {
        let blue = cursor.u8()?;
        let green = cursor.u8()?;
        let red = cursor.u8()?;
        let alpha = cursor.u8()?;
        palette.push(DwgRgba { red, green, blue, alpha });
    }
    let width = width as usize;
    let height = height as usize;
    let stride = (width + 3) & !3;
    if image_size as usize != stride * height {
        return Err("preview image size disagrees with dimensions".into());
    }
    let mut pixel_indices = Vec::with_capacity(width * height);
    let mut background_palette_index = None;
    for _ in 0..height {
        pixel_indices.extend_from_slice(cursor.take(width)?);
        let padding = cursor.take(stride - width)?;
        if let Some(index) = padding.first().copied() {
            if padding.iter().any(|value| *value != index) {
                return Err("preview row padding is inconsistent".into());
            }
            if background_palette_index.get_or_insert(index) != &index {
                return Err("preview background palette index changed between rows".into());
            }
        }
    }
    if cursor.take(16)? != DWG_PREVIEW_END {
        return Err("preview end sentinel changed".into());
    }
    cursor.finish("AcDb:Preview")?;
    Ok(DwgIndexedPreview { width: width as u32, height: height as u32, origin: DwgPreviewOrigin::BottomUp, palette, pixel_indices, background_palette_index: background_palette_index.unwrap_or(0) })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_digest128(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
/// 🧾️ The inverse of [`render_application_properties`]: the property-set format identifier and its
/// `<prop id="…"><string|datetime>…</…></prop>` entries.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_application_properties(rendered: &str) -> Result<(String, Vec<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgApplicationProperty>), String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgApplicationProperty;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgApplicationPropertyKind;
    let body = rendered.strip_prefix("<prop_set fmt_id=\"{").and_then(|rest| rest.strip_suffix("</prop_set>")).ok_or("application property set is not a <prop_set>")?;
    let (format_identifier, mut rest) = body.split_once("}\">").ok_or("application property set has no format identifier")?;
    let mut properties = Vec::new();
    while !rest.is_empty() {
        let entry = rest.strip_prefix("<prop id=\"").ok_or("application property is not a <prop>")?;
        let (id, entry) = entry.split_once("\"><").ok_or("application property has no id")?;
        let (tag, entry) = entry.split_once('>').ok_or("application property has no value tag")?;
        let kind = match tag {
            "string" => DwgApplicationPropertyKind::String,
            "datetime" => DwgApplicationPropertyKind::DateTime,
            other => return Err(format!("application property value tag {other:?} is unknown")),
        };
        let (value, entry) = entry.split_once(&format!("</{tag}></prop>")).ok_or("application property is not closed")?;
        properties.push(DwgApplicationProperty { id: id.parse().map_err(|_| format!("application property id {id:?} is not a number"))?, kind, value: value.into() });
        rest = entry;
    }
    Ok((format_identifier.into(), properties))
}
/// 🧾️ The inverse of [`render_product_information`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_product_information(rendered: &str) -> Result<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgProductInformation, String> {
    let mut values = Vec::new();
    let mut rest = rendered.strip_prefix("\"<ProductInformation name =\\\"").and_then(|rest| rest.strip_suffix("\\\"/>\"")).ok_or("product information is not a <ProductInformation>")?;
    for key in ["build_version", "registry_version", "install_id_string", "registry_localeID"] {
        let (value, next) = rest.split_once(&format!("\\\" {key}=\\\"")).ok_or_else(|| format!("product information has no {key}"))?;
        values.push(value.to_string());
        rest = next;
    }
    values.push(rest.to_string());
    let [name, build_version, registry_version, install_id, locale_id]: [String; 5] = values.try_into().map_err(|_| "product information field count")?;
    Ok(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgProductInformation { name, build_version, registry_version, install_id, locale_id })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_application_history(bytes: &[u8]) -> Result<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgApplicationHistory, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgApplicationHistory;
    let mut cursor = DwgSectionCursor::new(bytes);
    let history_identifier_one = decode_digest128(cursor.take(16)?);
    let history_identifier_two = decode_digest128(cursor.take(16)?);
    let class_version = cursor.u32()?;
    if cursor.utf16_z()? != "AppInfoDataList" || cursor.u32()? != 4 {
        return Err("unsupported application-history list".into());
    }
    let application_version_digest = decode_digest128(cursor.take(16)?);
    let application_version = cursor.utf16_z()?;
    let trust_comment_digest = decode_digest128(cursor.take(16)?);
    let trust_comment = cursor.utf16_z()?;
    let property_set_digest = decode_digest128(cursor.take(16)?);
    let rendered_properties = cursor.utf16_z()?;
    let product_digest = decode_digest128(cursor.take(16)?);
    let rendered_product = cursor.utf16_z()?;
    cursor.finish("AcDb:AppInfoHistory")?;
    let (property_format_identifier, properties) = parse_application_properties(&rendered_properties)?;
    let product = parse_product_information(&rendered_product)?;
    Ok(DwgApplicationHistory {
        history_identifier_one,
        history_identifier_two,
        class_version,
        application_version_digest,
        application_version,
        trust_comment_digest,
        trust_comment,
        property_set_digest,
        property_format_identifier,
        properties,
        product_digest,
        product,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_header_time(reader: &mut DwgBitReader<'_>) -> Result<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgJulianDate, String> {
    Ok(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgJulianDate { days: reader.read_bl()?, milliseconds: reader.read_bl()? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_header_color(reader: &mut DwgBitReader<'_>, expected_rgb: u32, name: &str) -> Result<u16, String> {
    let index = reader.read_bs()?;
    let rgb = reader.read_bl()?;
    let flags = reader.read_rc()?;
    if rgb != expected_rgb || flags != 0 {
        return Err(format!("unsupported AC1024 Header color {name} index={index} rgb={rgb:#x} flags={flags:#x}"));
    }
    Ok(index)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_header_space(reader: &mut DwgBitReader<'_>) -> Result<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgHeaderSpaceGeometry, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgHeaderSpaceGeometry;
    Ok(DwgHeaderSpaceGeometry {
        insertion_base: reader.read_3bd().map_err(|error| format!("insertion base: {error}"))?.to_vec(),
        extents_minimum: reader.read_3bd().map_err(|error| format!("extents minimum: {error}"))?.to_vec(),
        extents_maximum: reader.read_3bd().map_err(|error| format!("extents maximum: {error}"))?.to_vec(),
        limits_minimum: reader.read_2rd()?.to_vec(),
        limits_maximum: reader.read_2rd()?.to_vec(),
        elevation: reader.read_bd().map_err(|error| format!("elevation: {error}"))?,
        ucs_origin: reader.read_3bd().map_err(|error| format!("UCS origin: {error}"))?.to_vec(),
        ucs_x_axis: reader.read_3bd().map_err(|error| format!("UCS X axis: {error}"))?.to_vec(),
        ucs_y_axis: reader.read_3bd().map_err(|error| format!("UCS Y axis: {error}"))?.to_vec(),
        ucs_orthographic_view: reader.read_bs()?,
        ucs_origin_top: reader.read_3bd().map_err(|error| format!("UCS top: {error}"))?.to_vec(),
        ucs_origin_bottom: reader.read_3bd().map_err(|error| format!("UCS bottom: {error}"))?.to_vec(),
        ucs_origin_left: reader.read_3bd().map_err(|error| format!("UCS left: {error}"))?.to_vec(),
        ucs_origin_right: reader.read_3bd().map_err(|error| format!("UCS right: {error}"))?.to_vec(),
        ucs_origin_front: reader.read_3bd().map_err(|error| format!("UCS front: {error}"))?.to_vec(),
        ucs_origin_back: reader.read_3bd().map_err(|error| format!("UCS back: {error}"))?.to_vec(),
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_header_relation(reader: &mut DwgBitReader<'_>, name: &str) -> Result<u64, String> {
    let (code, value) = reader.read_handle()?;
    let expected = match name {
        "block_control" | "layer_control" | "style_control" | "linetype_control" | "view_control" | "ucs_control" | "viewport_control" | "appid_control" | "dimension_style_control" | "named_objects_dictionary" => 3,
        _ => 5,
    };
    if code != expected {
        return Err(format!("AC1024 Header relation {name} uses handle code {code}, expected {expected}"));
    }
    Ok(value)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_optional_header_relation(reader: &mut DwgBitReader<'_>, name: &str) -> Result<Option<u64>, String> {
    Ok(Some(read_header_relation(reader, name)?).filter(|value| *value != 0))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_r2010_header_section(bytes: &[u8]) -> Result<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgHeaderVariables, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgHeaderVariables;
    if bytes.len() < 58 || bytes[..16] != DWG_HEADER_BEGIN {
        return Err("AC1024 Header begin sentinel is missing".into());
    }
    let size = u32::from_le_bytes(bytes[16..20].try_into().unwrap()) as usize;
    let bit_size = u32::from_le_bytes(bytes[20..24].try_into().unwrap()) as usize;
    let crc_end = size.checked_add(20).filter(|end| *end >= 24 && end + 18 <= bytes.len()).ok_or("AC1024 Header size exceeds its section")?;
    if bytes[crc_end + 2..crc_end + 18] != DWG_HEADER_END {
        return Err("AC1024 Header end sentinel is missing".into());
    }
    let expected_crc = u16::from_le_bytes(bytes[crc_end..crc_end + 2].try_into().unwrap());
    if dwg_crc16(0xc0c1, &bytes[16..crc_end]) != expected_crc {
        return Err("AC1024 Header CRC mismatch".into());
    }
    let stream = &bytes[24..crc_end];
    let boundary = bit_size.checked_sub(32).filter(|bits| *bits <= stream.len() * 8).ok_or("AC1024 Header bit size exceeds its stream")?;
    let mut main = DwgBitReader::new(stream);
    let mut value = DwgHeaderVariables::default();
    let u = &mut value.units;
    u.unit1_conversion = main.read_bd()?;
    u.unit2_conversion = main.read_bd()?;
    u.unit3_conversion = main.read_bd()?;
    u.unit4_conversion = main.read_bd()?;
    if main.read_bl()? != 2_454_805 || main.read_bl()? != 60_784_745 {
        return Err("unsupported AC1024 Header producer date profile".into());
    }
    let m = &mut value.modes;
    m.dimension_associative = main.read_b()?;
    m.dimension_show = main.read_b()?;
    m.polyline_generation = main.read_b()?;
    m.orthographic_mode = main.read_b()?;
    m.regeneration_mode = main.read_b()?;
    m.fill_mode = main.read_b()?;
    m.quick_text_mode = main.read_b()?;
    m.paper_space_linetype_scale = main.read_b()?;
    m.limits_check = main.read_b()?;
    if main.read_b()? {
        return Err("unsupported AC1024 Header limits feature marker".into());
    }
    m.user_timer = main.read_b()?;
    m.sketch_polyline = main.read_b()?;
    m.angle_direction = main.read_b()?;
    m.spline_frame = main.read_b()?;
    m.mirror_text = main.read_b()?;
    m.world_view = main.read_b()?;
    m.tile_mode = main.read_b()?;
    m.paper_limits_check = main.read_b()?;
    m.visual_retain = main.read_b()?;
    m.display_silhouette = main.read_b()?;
    m.polyline_ellipse = main.read_b()?;
    let i = &mut value.integers;
    i.proxy_graphics = main.read_bs()?;
    i.tree_depth = main.read_bs()? as i16;
    i.linear_units = main.read_bs()?;
    i.linear_precision = main.read_bs()?;
    i.angular_units = main.read_bs()?;
    i.angular_precision = main.read_bs()?;
    i.attribute_mode = main.read_bs()?;
    i.point_display_mode = main.read_bs()?;
    if [main.read_bl()?, main.read_bl()?, main.read_bl()?] != [0x30303030, 0x1d050900, 0x4d353930] {
        return Err("unsupported AC1024 Header producer generation".into());
    }
    i.user_integer1 = main.read_bs()? as i16;
    i.user_integer2 = main.read_bs()? as i16;
    i.user_integer3 = main.read_bs()? as i16;
    i.user_integer4 = main.read_bs()? as i16;
    i.user_integer5 = main.read_bs()? as i16;
    i.spline_segments = main.read_bs()?;
    i.surface_u = main.read_bs()?;
    i.surface_v = main.read_bs()?;
    i.surface_type = main.read_bs()?;
    i.surface_tab1 = main.read_bs()?;
    i.surface_tab2 = main.read_bs()?;
    i.spline_type = main.read_bs()?;
    i.shade_edge = main.read_bs()?;
    i.shade_difference = main.read_bs()?;
    i.unit_mode = main.read_bs()?;
    i.maximum_active_viewports = main.read_bs()?;
    i.isolines = main.read_bs()?;
    i.multiline_justification = main.read_bs()?;
    i.text_quality = main.read_bs()?;
    let s = &mut value.scalars;
    s.linetype_scale = main.read_bd()?;
    s.text_size = main.read_bd()?;
    s.trace_width = main.read_bd()?;
    s.sketch_increment = main.read_bd()?;
    s.fillet_radius = main.read_bd()?;
    s.thickness = main.read_bd()?;
    s.angle_base = main.read_bd()?;
    s.point_display_size = main.read_bd()?;
    s.polyline_width = main.read_bd()?;
    s.user_real1 = main.read_bd()?;
    s.user_real2 = main.read_bd()?;
    s.user_real3 = main.read_bd()?;
    s.user_real4 = main.read_bd()?;
    s.user_real5 = main.read_bd()?;
    s.chamfer_a = main.read_bd()?;
    s.chamfer_b = main.read_bd()?;
    s.chamfer_c = main.read_bd()?;
    s.chamfer_d = main.read_bd()?;
    s.facet_resolution = main.read_bd()?;
    s.multiline_scale = main.read_bd()?;
    s.current_entity_linetype_scale = main.read_bd()?;
    value.time.created_at = read_header_time(&mut main)?;
    value.time.updated_at = read_header_time(&mut main)?;
    if [main.read_bl()?, main.read_bl()?, main.read_bl()?] != [0x44443434, 0x140d0102, 0x39343531] {
        return Err("unsupported AC1024 Header time producer profile".into());
    }
    value.time.editing_duration = read_header_time(&mut main)?;
    value.time.user_timer_duration = read_header_time(&mut main)?;
    s.current_entity_color_index = read_header_color(&mut main, 0xc0000000, "CECOLOR")?;
    let (seed_code, seed) = main.read_handle()?;
    if seed_code != 0 {
        return Err(format!("AC1024 HANDSEED code {seed_code} != 0"));
    }
    value.relations.handle_seed = seed;
    s.paper_space_viewport_scale = main.read_bd()?;
    value.paper_space = read_header_space(&mut main).map_err(|error| format!("paper-space Header geometry: {error}"))?;
    value.model_space = read_header_space(&mut main).map_err(|error| format!("model-space Header geometry: {error}"))?;
    let d = &mut value.dimensions;
    d.scale = main.read_bd()?;
    d.arrow_size = main.read_bd()?;
    d.extension_offset = main.read_bd()?;
    d.line_increment = main.read_bd()?;
    d.extension = main.read_bd()?;
    d.rounding = main.read_bd()?;
    d.line_extension = main.read_bd()?;
    d.tolerance_plus = main.read_bd()?;
    d.tolerance_minus = main.read_bd()?;
    d.fixed_extension_length = main.read_bd()?;
    d.jog_angle = main.read_bd()?;
    d.text_fill = main.read_bs()?;
    d.text_fill_color_index = read_header_color(&mut main, 0xc1000000, "DIMTFILLCLR")?;
    d.tolerance = main.read_b()?;
    d.limits = main.read_b()?;
    d.text_inside_horizontal = main.read_b()?;
    d.text_outside_horizontal = main.read_b()?;
    d.suppress_extension1 = main.read_b()?;
    d.suppress_extension2 = main.read_b()?;
    d.text_above = main.read_bs()?;
    d.zero_suppression = main.read_bs()?;
    d.angular_zero_suppression = main.read_bs()?;
    d.arc_symbol = main.read_bs()?;
    d.text_height = main.read_bd()?;
    d.center_mark = main.read_bd()?;
    d.tick_size = main.read_bd()?;
    d.alternate_scale = main.read_bd()?;
    d.linear_factor = main.read_bd()?;
    d.text_vertical_position = main.read_bd()?;
    d.text_factor = main.read_bd()?;
    d.gap = main.read_bd()?;
    d.alternate_rounding = main.read_bd()?;
    d.alternate_units = main.read_b()?;
    d.alternate_decimal_places = main.read_bs()?;
    d.text_outside_force_line = main.read_b()?;
    d.separate_arrows = main.read_b()?;
    d.text_inside = main.read_b()?;
    d.suppress_outside = main.read_b()?;
    d.line_color_index = read_header_color(&mut main, 0xc1000000, "DIMCLRD")?;
    d.extension_color_index = read_header_color(&mut main, 0xc1000000, "DIMCLRE")?;
    d.text_color_index = read_header_color(&mut main, 0xc1000000, "DIMCLRT")?;
    d.angular_decimal_places = main.read_bs()?;
    d.decimal_places = main.read_bs()?;
    d.tolerance_decimal_places = main.read_bs()?;
    d.alternate_units_format = main.read_bs()?;
    d.alternate_tolerance_decimal_places = main.read_bs()?;
    d.angular_unit_format = main.read_bs()?;
    d.fractional_format = main.read_bs()?;
    d.linear_unit_format = main.read_bs()?;
    d.decimal_separator = main.read_bs()?;
    d.text_movement = main.read_bs()?;
    d.justification = main.read_bs()?;
    d.suppress_dimension1 = main.read_b()?;
    d.suppress_dimension2 = main.read_b()?;
    d.tolerance_justification = main.read_bs()?;
    d.tolerance_zero_suppression = main.read_bs()?;
    d.alternate_zero_suppression = main.read_bs()?;
    d.alternate_tolerance_zero_suppression = main.read_bs()?;
    d.user_positioned_text = main.read_b()?;
    d.fit = main.read_bs()?;
    d.fixed_extension_enabled = main.read_b()?;
    d.text_direction = main.read_b()?;
    d.alternate_measurement_scale = main.read_bd()?;
    d.measurement_scale = main.read_bd()?;
    d.dimension_line_weight = main.read_bs()? as i16;
    d.extension_line_weight = main.read_bs()? as i16;
    let p = &mut value.policy;
    p.text_stack_alignment = main.read_bs()?;
    p.text_stack_size = main.read_bs()?;
    if main.read_bl()? != 0x2a1d {
        return Err("unsupported AC1024 Header packed drawing policy".into());
    }
    p.current_entity_lineweight = -1;
    p.end_caps = 0;
    p.join_style = 0;
    p.lineweight_display = false;
    p.external_reference_editing = true;
    p.extended_names = true;
    p.plot_style_mode = true;
    p.ole_startup = false;
    p.insertion_units = main.read_bs()?;
    p.current_plot_style_type = main.read_bs()?;
    p.sort_entities = main.read_rc()?;
    p.index_control = main.read_rc()?;
    p.hide_text = main.read_rc()?;
    p.xclip_frame = main.read_rc()?;
    p.dimension_association = main.read_rc()?;
    p.halo_gap = main.read_rc()?;
    p.obscured_color = main.read_bs()?;
    p.intersection_color = main.read_bs()?;
    p.obscured_linetype = main.read_rc()?;
    p.intersection_display = main.read_rc()?;
    p.camera_display = main.read_b()?;
    if [main.read_bl()?, main.read_bl()?] != [0, 10] || main.read_bd()? != 1.0 {
        return Err("unsupported AC1024 Header render profile".into());
    }
    p.steps_per_second = main.read_bd()?;
    p.step_size = main.read_bd()?;
    p.dwf_3d_precision = main.read_bd()?;
    p.lens_length = main.read_bd()?;
    p.camera_height = main.read_bd()?;
    p.solid_history = main.read_rc()?;
    p.show_history = main.read_rc()?;
    p.polysolid_width = main.read_bd()?;
    p.polysolid_height = main.read_bd()?;
    p.loft_angle1 = main.read_bd()?;
    p.loft_angle2 = main.read_bd()?;
    p.loft_magnitude1 = main.read_bd()?;
    p.loft_magnitude2 = main.read_bd()?;
    p.loft_parameter = main.read_bs()?;
    p.loft_normals = main.read_rc()?;
    p.latitude = main.read_bd()?;
    p.longitude = main.read_bd()?;
    p.north_direction = main.read_bd()?;
    p.timezone = main.read_bl()? as i32;
    p.light_glyph_display = main.read_rc()?;
    p.tile_mode_light_sync = main.read_rc()?;
    p.dwf_frame = main.read_rc()?;
    p.dgn_frame = main.read_rc()?;
    p.real_world_scale = main.read_b()?;
    if read_header_color(&mut main, 0xc3000001, "INTERFERECOLOR")? != 0 {
        return Err("unsupported AC1024 INTERFERECOLOR index".into());
    }
    p.interfere_color_index = 256;
    p.shadow_mode = main.read_rc()?;
    p.shadow_plane_location = main.read_bd()?;
    let strings_start = main.bit_position();
    let mut strings = DwgBitReader::at_bit(stream, strings_start)?;
    u.unit1_name = strings.read_tu()?;
    u.unit2_name = strings.read_tu()?;
    u.unit3_name = strings.read_tu()?;
    u.unit4_name = strings.read_tu()?;
    value.strings.menu = strings.read_tu()?;
    value.strings.dimension_postfix = strings.read_tu()?;
    value.strings.dimension_alternate_postfix = strings.read_tu()?;
    value.strings.dimension_alternate_measurement_zero_suffix = strings.read_tu()?;
    value.strings.dimension_measurement_zero_suffix = strings.read_tu()?;
    value.strings.hyperlink_base = strings.read_tu()?;
    value.strings.stylesheet = strings.read_tu()?;
    value.strings.fingerprint_guid = strings.read_tu()?;
    value.strings.version_guid = strings.read_tu()?;
    value.strings.project_name = strings.read_tu()?;
    let string_bits = strings.bit_position() - strings_start;
    if usize::from(strings.read_rs()?) != string_bits || !strings.read_b()? || strings.bit_position() != boundary {
        return Err(format!("AC1024 Header string footer does not close the {string_bits}-bit string stream at bit {boundary}"));
    }
    let mut handles = DwgBitReader::at_bit(stream, boundary)?;
    let r = &mut value.relations;
    r.current_layer = read_header_relation(&mut handles, "CLAYER")?;
    r.text_style = read_header_relation(&mut handles, "TEXTSTYLE")?;
    r.current_linetype = read_header_relation(&mut handles, "CELTYPE")?;
    r.current_material = read_header_relation(&mut handles, "CMATERIAL")?;
    r.dimension_style = read_header_relation(&mut handles, "DIMSTYLE")?;
    r.multiline_style = read_header_relation(&mut handles, "CMLSTYLE")?;
    r.paper_ucs_name = read_optional_header_relation(&mut handles, "PUCSNAME")?;
    r.paper_ucs_orthographic_reference = read_optional_header_relation(&mut handles, "PUCSORTHOREF")?;
    r.paper_ucs_base = read_optional_header_relation(&mut handles, "PUCSBASE")?;
    r.model_ucs_name = read_optional_header_relation(&mut handles, "UCSNAME")?;
    r.model_ucs_orthographic_reference = read_optional_header_relation(&mut handles, "UCSORTHOREF")?;
    r.model_ucs_base = read_optional_header_relation(&mut handles, "UCSBASE")?;
    r.dimension_text_style = read_header_relation(&mut handles, "DIMTXSTY")?;
    r.dimension_leader_block = read_optional_header_relation(&mut handles, "DIMLDRBLK")?;
    r.dimension_block = read_optional_header_relation(&mut handles, "DIMBLK")?;
    r.dimension_block1 = read_optional_header_relation(&mut handles, "DIMBLK1")?;
    r.dimension_block2 = read_optional_header_relation(&mut handles, "DIMBLK2")?;
    r.dimension_linetype = read_optional_header_relation(&mut handles, "DIMLTYPE")?;
    r.dimension_extension_linetype1 = read_optional_header_relation(&mut handles, "DIMLTEX1")?;
    r.dimension_extension_linetype2 = read_optional_header_relation(&mut handles, "DIMLTEX2")?;
    macro_rules! relation {
        ($field:ident) => {
            r.$field = read_header_relation(&mut handles, stringify!($field))?;
        };
    }
    relation!(block_control);
    relation!(layer_control);
    relation!(style_control);
    relation!(linetype_control);
    relation!(view_control);
    relation!(ucs_control);
    relation!(viewport_control);
    relation!(appid_control);
    relation!(dimension_style_control);
    relation!(group_dictionary);
    relation!(multiline_style_dictionary);
    relation!(named_objects_dictionary);
    relation!(layout_dictionary);
    relation!(plot_settings_dictionary);
    relation!(plot_style_name_dictionary);
    relation!(material_dictionary);
    relation!(color_dictionary);
    relation!(visual_style_dictionary);
    relation!(paper_space_block_record);
    relation!(model_space_block_record);
    relation!(by_layer_linetype);
    relation!(by_block_linetype);
    relation!(continuous_linetype);
    r.interfere_object_visual_style = read_optional_header_relation(&mut handles, "INTERFEREOBJVS")?;
    r.interfere_viewport_visual_style = read_optional_header_relation(&mut handles, "INTERFEREVPVS")?;
    r.drag_visual_style = read_optional_header_relation(&mut handles, "DRAGVS")?;
    for expected in [0xbfc4, 0x122d, 0xa23e, 0xb717] {
        let actual = handles.read_bs()?;
        if actual != expected {
            return Err(format!("unsupported AC1024 Header producer compatibility value {actual:#x}"));
        }
    }
    if handles.read_bl()? != 0 || handles.read_bl()? != 0 || !handles.read_b()? || !(handles.read_b()? && handles.read_b()? && handles.read_b()?) || handles.bit_position() != stream.len() * 8 {
        return Err("unsupported AC1024 Header terminal framing".into());
    }
    Ok(value)
}
/// 📑️ The typed R2004 document sections a round trip reads back: header, dependencies, summary, application, template, revisions and preview.
pub struct DwgDocumentSections {
    pub header: semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgHeaderVariables,
    pub dependencies: Vec<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDependency>,
    pub summary: semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgSummaryInfo,
    pub application: semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgApplicationInfo,
    pub template: semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgTemplate,
    pub auxiliary_header: semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAuxiliaryHeader,
    pub revision_history: semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgRevisionHistory,
    pub preview: semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgIndexedPreview,
    pub application_history: semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgApplicationHistory,
}
/// 📑️ Decodes the R2004 document sections (summary, preview, application history) that the round-trip laws read back.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_r2004_document_sections(bytes: &[u8]) -> Result<DwgDocumentSections, String> {
    let sections = decode_r2004_sections(bytes)?;
    let data = |name: &str| -> Result<Vec<u8>, String> {
        let section = sections.iter().find(|section| section.name == name).ok_or_else(|| format!("R2004 {name} section missing"))?;
        r2004_section_data(section)
    };
    Ok(DwgDocumentSections {
        header: decode_r2010_header_section(&data("AcDb:Header")?).map_err(|error| format!("AcDb:Header: {error}"))?,
        dependencies: decode_dependencies(&data("AcDb:FileDepList")?).map_err(|error| format!("AcDb:FileDepList: {error}"))?,
        summary: decode_summary_info(&data("AcDb:SummaryInfo")?).map_err(|error| format!("AcDb:SummaryInfo: {error}"))?,
        application: decode_application_info(&data("AcDb:AppInfo")?).map_err(|error| format!("AcDb:AppInfo: {error}"))?,
        template: decode_template(&data("AcDb:Template")?).map_err(|error| format!("AcDb:Template: {error}"))?,
        auxiliary_header: decode_auxiliary_header(&data("AcDb:AuxHeader")?).map_err(|error| format!("AcDb:AuxHeader: {error}"))?,
        revision_history: decode_revision_history(&data("AcDb:RevHistory")?).map_err(|error| format!("AcDb:RevHistory: {error}"))?,
        preview: decode_indexed_preview(&data("AcDb:Preview")?).map_err(|error| format!("AcDb:Preview: {error}"))?,
        application_history: decode_application_history(&data("AcDb:AppInfoHistory")?).map_err(|error| format!("AcDb:AppInfoHistory: {error}"))?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_r2004_modular_char(bytes: &[u8], position: &mut usize, signed: bool) -> Result<i64, String> {
    let mut value = 0i64;
    let mut shift = 0u32;
    loop {
        let byte = *bytes.get(*position).ok_or("R2004 modular-char underflow")?;
        *position += 1;
        let negative = signed && byte & 0x80 == 0 && byte & 0x40 != 0;
        value |= ((byte & if negative { 0x3f } else { 0x7f }) as i64) << shift;
        if byte & 0x80 == 0 {
            if negative {
                value = -value;
            }
            return Ok(value);
        }
        shift += 7;
        if shift > 56 {
            return Err("R2004 modular-char overflow".into());
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_r2004_handle_map(bytes: &[u8]) -> Result<Vec<(u64, usize)>, String> {
    let mut position = 0usize;
    let mut entries = Vec::new();
    while position + 2 <= bytes.len() {
        let block_start = position;
        let block_size = u16::from_be_bytes([bytes[position], bytes[position + 1]]) as usize;
        position += 2;
        if block_size <= 2 {
            break;
        }
        let block_end = block_start.checked_add(block_size).ok_or("R2004 handle-map block overflow")?;
        if block_end > bytes.len() {
            return Err(format!("R2004 handle-map block {block_size} exceeds {} bytes", bytes.len() - block_start));
        }
        let mut handle = 0i64;
        let mut address = 0i64;
        while position < block_end {
            handle += read_r2004_modular_char(bytes, &mut position, false)?;
            address += read_r2004_modular_char(bytes, &mut position, true)?;
            if handle < 0 || address < 0 {
                return Err("R2004 handle-map produced a negative handle or object address".into());
            }
            entries.push((handle as u64, address as usize));
        }
        position = block_end.saturating_add(2).min(bytes.len());
    }
    Ok(entries)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_r2010_eed(reader: &mut DwgBitReader<'_>, _base: u64) -> Result<Vec<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgExtendedEntityData>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgExtendedEntityData;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgXRecordValue;
    let mut records = Vec::new();
    loop {
        let size = reader.read_bs()? as usize;
        if size == 0 {
            return Ok(records);
        }
        let (handle_code, handle_value) = reader.read_handle()?;
        if handle_code != 5 || handle_value == 0 {
            return Err(format!("R2010 EED application reference must be a non-null hard pointer, found code={handle_code} value={handle_value:#x}"));
        }
        let application_handle = handle_value;
        let end_bit = reader.bit_position().checked_add(size.checked_mul(8).ok_or("R2010 EED size overflow")?).ok_or("R2010 EED end overflow")?;
        if end_bit > reader.bytes.len().saturating_mul(8) {
            return Err("R2010 EED record exceeds object payload".into());
        }
        let mut values = Vec::new();
        while reader.bit_position() < end_bit {
            let code = reader.read_rc()?;
            let value = match code {
                0 => {
                    let length = reader.read_rs()? as usize;
                    let units = (0..length).map(|_| reader.read_rs()).collect::<Result<Vec<_>, _>>()?;
                    DwgXRecordValue::String { group_code: 1000, value: String::from_utf16(&units).map_err(|error| format!("invalid R2010 EED UTF-16 string: {error}"))? }
                }
                2 => {
                    let close = reader.read_rc()?;
                    let value = match close {
                        0 => "{",
                        1 => "}",
                        other => return Err(format!("R2010 EED control-string value {other} is invalid")),
                    };
                    DwgXRecordValue::String { group_code: 1002, value: value.into() }
                }
                3 => DwgXRecordValue::Handle { group_code: 1003, value: reader.read_rll()? },
                4 => {
                    let length = reader.read_rc()? as usize;
                    let octets = (0..length).map(|_| reader.read_rc()).collect::<Result<Vec<_>, _>>()?;
                    DwgXRecordValue::Binary { group_code: 1004, octets }
                }
                5 => {
                    let mut value = 0u64;
                    for _ in 0..8 {
                        value = (value << 8) | u64::from(reader.read_rc()?);
                    }
                    DwgXRecordValue::Handle { group_code: 1005, value }
                }
                10..=15 => DwgXRecordValue::Point3d { group_code: 1000 + i16::from(code), value: [reader.read_rd()?, reader.read_rd()?, reader.read_rd()?] },
                40..=42 => DwgXRecordValue::Real { group_code: 1000 + i16::from(code), value: reader.read_rd()? },
                70 => DwgXRecordValue::Integer16 { group_code: 1070, value: reader.read_rs()? as i16 },
                71 => DwgXRecordValue::Integer32 { group_code: 1071, value: reader.read_rl()? as i32 },
                1 => return Err("R2010 EED application-index item is redundant with its typed application handle".into()),
                other => return Err(format!("unknown R2010 EED item code {other}")),
            };
            value.validate()?;
            if reader.bit_position() > end_bit {
                return Err(format!("R2010 EED item code {code} exceeds its declared record size"));
            }
            values.push(value);
        }
        if reader.bit_position() != end_bit {
            return Err("R2010 EED record is not exactly consumed".into());
        }
        records.push(DwgExtendedEntityData { application_handle, values });
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn fixed_object_name(object_type: u16) -> &'static str {
    match object_type {
        1 => "TEXT",
        2 => "ATTRIB",
        3 => "ATTDEF",
        4 => "BLOCK",
        5 => "ENDBLK",
        6 => "SEQEND",
        7 => "INSERT",
        8 => "MINSERT",
        10 => "VERTEX_2D",
        11 => "VERTEX_3D",
        12 => "VERTEX_MESH",
        13 => "VERTEX_PFACE",
        14 => "VERTEX_PFACE_FACE",
        15 => "POLYLINE_2D",
        16 => "POLYLINE_3D",
        17 => "ARC",
        18 => "CIRCLE",
        19 => "LINE",
        20..=26 => "DIMENSION",
        27 => "POINT",
        28 => "3DFACE",
        29 => "POLYLINE_PFACE",
        30 => "POLYLINE_MESH",
        31 => "SOLID",
        32 => "TRACE",
        33 => "SHAPE",
        34 => "VIEWPORT",
        35 => "ELLIPSE",
        36 => "SPLINE",
        37 => "REGION",
        38 => "3DSOLID",
        39 => "BODY",
        40 => "RAY",
        41 => "XLINE",
        42 => "DICTIONARY",
        43 => "OLEFRAME",
        44 => "MTEXT",
        45 => "LEADER",
        46 => "TOLERANCE",
        47 => "MLINE",
        48 => "BLOCK_CONTROL",
        49 => "BLOCK_HEADER",
        50 => "LAYER_CONTROL",
        51 => "LAYER",
        52 => "STYLE_CONTROL",
        53 => "STYLE",
        56 => "LTYPE_CONTROL",
        57 => "LTYPE",
        60 => "VIEW_CONTROL",
        61 => "VIEW",
        62 => "UCS_CONTROL",
        63 => "UCS",
        64 => "VPORT_CONTROL",
        65 => "VPORT",
        66 => "APPID_CONTROL",
        67 => "APPID",
        68 => "DIMSTYLE_CONTROL",
        69 => "DIMSTYLE",
        70 => "VP_ENT_HDR_CONTROL",
        71 => "VP_ENT_HDR",
        72 => "GROUP",
        73 => "MLINESTYLE",
        74 => "OLE2FRAME",
        75 => "DUMMY",
        76 => "LONG_TRANSACTION",
        77 => "LWPOLYLINE",
        78 => "HATCH",
        79 => "XRECORD",
        80 => "ACDBPLACEHOLDER",
        81 => "VBA_PROJECT",
        82 => "LAYOUT",
        498 => "ACAD_PROXY_ENTITY",
        499 => "ACAD_PROXY_OBJECT",
        _ => "UNKNOWN",
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn object_category(object_type: u16) -> semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgObjectCategory {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgObjectCategory;
    match object_type {
        1..=41 | 43..=47 | 77 | 78 | 498 => DwgObjectCategory::Entity,
        48 | 50 | 52 | 56 | 60 | 62 | 64 | 66 | 68 | 70 => DwgObjectCategory::TableControl,
        49 | 51 | 53 | 57 | 61 | 63 | 65 | 67 | 69 | 71 => DwgObjectCategory::TableRecord,
        42 => DwgObjectCategory::Dictionary,
        500.. => DwgObjectCategory::Custom,
        _ => DwgObjectCategory::Object,
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn resolve_object_handle(base: u64, code: u8, value: u64) -> Option<u64> {
    let resolved = match code {
        6 => base.checked_add(1)?,
        8 => base.checked_sub(1)?,
        10 => base.checked_add(value)?,
        12 => base.checked_sub(value)?,
        _ => value,
    };
    (resolved != 0).then_some(resolved)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_object_handle(reader: &mut DwgBitReader<'_>, base: u64) -> Result<Option<u64>, String> {
    let (code, value) = reader.read_handle()?;
    Ok(resolve_object_handle(base, code, value))
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum XRecordStorageKind {
    String,
    Real,
    Boolean,
    Integer8,
    Integer16,
    Integer32,
    Integer64,
    Point3d,
    Binary,
    Handle,
    ObjectId,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xrecord_value_kind(group_code: i16) -> Option<XRecordStorageKind> {
    let code = i32::from(group_code);
    Some(match code {
        5 | 105 | 320..=329 | 390..=399 | 1003 | 1005 => XRecordStorageKind::Handle,
        0..=4 | 6..=9 | 100..=104 | 300..=309 | 410..=419 | 430..=439 | 470..=479 | 999 | 1000..=1002 => XRecordStorageKind::String,
        10..=37 | 110..=139 | 210..=269 | 1010..=1015 => XRecordStorageKind::Point3d,
        38..=59 | 140..=149 | 460..=469 | 1040..=1042 => XRecordStorageKind::Real,
        60..=79 | 170..=179 | 270..=279 | 370..=389 | 400..=409 | 1070 => XRecordStorageKind::Integer16,
        90..=99 | 420..=429 | 440..=459 | 1071 => XRecordStorageKind::Integer32,
        160..=169 => XRecordStorageKind::Integer64,
        280..=289 => XRecordStorageKind::Integer8,
        290..=299 => XRecordStorageKind::Boolean,
        310..=319 | 1004 => XRecordStorageKind::Binary,
        330..=369 => XRecordStorageKind::ObjectId,
        _ => return None,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_xrecord_values(data: &mut DwgBitReader<'_>, byte_count: usize, main_end_bit: usize) -> Result<Vec<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgXRecordValue>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgXRecordValue;
    let end_bit = data.bit_position().checked_add(byte_count.checked_mul(8).ok_or("XRECORD value size overflow")?).ok_or("XRECORD value boundary overflow")?;
    if end_bit > main_end_bit {
        return Err("XRECORD values exceed bounded class-main data".into());
    }
    let mut values = Vec::new();
    while data.bit_position() < end_bit {
        if end_bit.saturating_sub(data.bit_position()) < 16 {
            return Err("XRECORD group code is truncated".into());
        }
        let group_code = data.read_rs()? as i16;
        let kind = xrecord_value_kind(group_code).ok_or_else(|| format!("unsupported XRECORD group code {group_code}"))?;
        let value = match kind {
            XRecordStorageKind::String => {
                let length = data.read_rs()? as usize;
                if length > end_bit.saturating_sub(data.bit_position()) / 16 {
                    return Err(format!("XRECORD string for group {group_code} exceeds value boundary"));
                }
                let units = (0..length).map(|_| data.read_rs()).collect::<Result<Vec<_>, _>>()?;
                DwgXRecordValue::String { group_code, value: String::from_utf16(&units).map_err(|error| format!("invalid XRECORD UTF-16 string: {error}"))? }
            }
            XRecordStorageKind::Real => DwgXRecordValue::Real { group_code, value: data.read_rd()? },
            XRecordStorageKind::Boolean => {
                let stored = data.read_rc()?;
                if stored > 1 {
                    return Err(format!("XRECORD boolean group {group_code} has invalid value {stored}"));
                }
                DwgXRecordValue::Boolean { group_code, value: stored != 0 }
            }
            XRecordStorageKind::Integer8 => DwgXRecordValue::Integer8 { group_code, value: data.read_rc()? as i8 },
            XRecordStorageKind::Integer16 => DwgXRecordValue::Integer16 { group_code, value: data.read_rs()? as i16 },
            XRecordStorageKind::Integer32 => DwgXRecordValue::Integer32 { group_code, value: data.read_rl()? as i32 },
            XRecordStorageKind::Integer64 => DwgXRecordValue::Integer64 { group_code, value: data.read_rll()? as i64 },
            XRecordStorageKind::Point3d => DwgXRecordValue::Point3d { group_code, value: [data.read_rd()?, data.read_rd()?, data.read_rd()?] },
            XRecordStorageKind::Binary => {
                let length = data.read_rc()? as usize;
                if length > end_bit.saturating_sub(data.bit_position()) / 8 {
                    return Err(format!("XRECORD binary value for group {group_code} exceeds value boundary"));
                }
                DwgXRecordValue::Binary { group_code, octets: (0..length).map(|_| data.read_rc()).collect::<Result<Vec<_>, _>>()? }
            }
            XRecordStorageKind::Handle => DwgXRecordValue::Handle { group_code, value: data.read_rll()? },
            XRecordStorageKind::ObjectId => {
                let absolute_value = data.read_rll()?;
                DwgXRecordValue::ObjectId { group_code, absolute_value }
            }
        };
        if data.bit_position() > end_bit {
            return Err(format!("XRECORD value for group {group_code} exceeds declared size"));
        }
        values.push(value);
    }
    if data.bit_position() != end_bit {
        return Err("XRECORD values do not consume their declared size".into());
    }
    Ok(values)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dimension_attachment_logical(value: u16) -> Result<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDimensionTextAttachment, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDimensionTextAttachment::*;
    match value {
        1 => Ok(TopCenter),
        2 => Ok(TopLeft),
        3 => Ok(TopRight),
        4 => Ok(MiddleCenter),
        5 => Ok(MiddleLeft),
        6 => Ok(MiddleRight),
        7 => Ok(BottomCenter),
        8 => Ok(BottomLeft),
        9 => Ok(BottomRight),
        _ => Err(format!("dimension text attachment {value} is invalid")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dimension_spacing_logical(value: u16) -> Result<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDimensionLineSpacingStyle, String> {
    match value {
        1 => Ok(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDimensionLineSpacingStyle::AtLeast),
        2 => Ok(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDimensionLineSpacingStyle::Exact),
        _ => Err(format!("dimension line-spacing style {value} is invalid")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn viewport_status_flags(mask: u32) -> Result<Vec<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgViewportStatusFlag>, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgViewportStatusFlag::*;
    if mask & !0x003f_ffff != 0 {
        return Err(format!("VIEWPORT status contains unsupported bits {:#x}", mask & !0x003f_ffff));
    }
    let all = [
        Perspective,
        FrontClipping,
        BackClipping,
        UcsFollow,
        FrontClipNotAtEye,
        UcsIconVisible,
        UcsIconAtOrigin,
        FastZoom,
        Snap,
        Grid,
        IsometricSnap,
        HidePlot,
        IsoPairTop,
        IsoPairRight,
        ZoomLock,
        AlwaysEnabled,
        NonRectangularClipping,
        ViewportOff,
        GridBeyondDrawingLimits,
        AdaptiveGrid,
        AdaptiveSubdivision,
        GridFollowsWorkplane,
    ];
    Ok(all.into_iter().enumerate().filter_map(|(bit, flag)| (mask & (1 << bit) != 0).then_some(flag)).collect())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_visual_style_operation(data: &mut DwgBitReader<'_>) -> Result<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgVisualStylePropertyOperation, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgVisualStylePropertyOperation;
    match data.read_bs()? {
        0 => Ok(DwgVisualStylePropertyOperation::Inherit),
        1 => Ok(DwgVisualStylePropertyOperation::Set),
        2 => Ok(DwgVisualStylePropertyOperation::Disable),
        3 => Ok(DwgVisualStylePropertyOperation::Enable),
        value => Err(format!("visual-style property operation {value} is invalid")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_visual_style_color(data: &mut DwgBitReader<'_>) -> Result<(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgVisualStyleProperty<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgComplexColor>, u8), String> {
    let (value, flags) = read_r2010_cmc_main(data)?;
    let operation = read_visual_style_operation(data)?;
    Ok((semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgVisualStyleProperty { value, operation }, flags))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_r2010_block_element(data: &mut DwgBitReader<'_>, strings: &mut DwgBitReader<'_>, class_name: &str) -> Result<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockElement, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockElement;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEvaluationExpression;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEvaluationExpressionValue;
    let parent_id = data.read_bl()? as i32;
    let major_version = data.read_bl()?;
    let minor_version = data.read_bl()?;
    let value_code = data.read_bs()? as i16;
    let node_id = data.read_bl()?;
    let name = strings.read_tu()?;
    let repeated_major = data.read_bl()?;
    let repeated_minor = data.read_bl()?;
    let marker = data.read_bl()?;
    if parent_id != -1 || (major_version, minor_version, value_code, repeated_major, repeated_minor, marker) != (29, 2, -9999, 29, 2, 0) || name.is_empty() {
        return Err(format!("{class_name} block-element metadata is invalid"));
    }
    Ok(DwgBlockElement { evaluation_expression: DwgEvaluationExpression { parent_id, major_version, minor_version, value: DwgEvaluationExpressionValue::Empty, node_id }, name })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_r2010_block_grip(data: &mut DwgBitReader<'_>, strings: &mut DwgBitReader<'_>, x_role: &str, y_role: &str, class_name: &str) -> Result<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockGrip, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockGrip;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgNamedEvaluationNodeReference;
    let element = decode_r2010_block_element(data, strings, class_name)?;
    let updated_x = DwgNamedEvaluationNodeReference { node_id: data.read_bl()?, expression_name: x_role.into() };
    let updated_y = DwgNamedEvaluationNodeReference { node_id: data.read_bl()?, expression_name: y_role.into() };
    let location = data.read_3bd()?.to_vec();
    let insertion_cycling = data.read_b()?;
    let insertion_cycling_weight = data.read_bl()? as i32;
    if location.iter().any(|value| !value.is_finite()) {
        return Err(format!("{class_name} location is invalid"));
    }
    Ok(DwgBlockGrip { element, location, insertion_cycling, insertion_cycling_weight, updated_x, updated_y })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_r2010_two_point_parameter(data: &mut DwgBitReader<'_>, strings: &mut DwgBitReader<'_>, class_name: &str) -> Result<(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockTwoPointParameter, [u32; 4]), String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockParameterBaseLocation;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockParameterConnection;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockParameterProperty;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockTwoPointParameter;
    let element = decode_r2010_block_element(data, strings, class_name)?;
    let show_properties = data.read_b()?;
    let chain_actions = data.read_b()?;
    let definition_base = data.read_3bd()?.to_vec();
    let definition_end = data.read_3bd()?.to_vec();
    let mut properties = Vec::with_capacity(4);
    for _ in 0..4 {
        let count = data.read_bl()? as usize;
        if count > 10_000 {
            return Err(format!("{class_name} property connection count is invalid"));
        }
        let codes = (0..count).map(|_| data.read_bl()).collect::<Result<Vec<_>, _>>()?;
        let connections = codes.into_iter().map(|code| Ok(DwgBlockParameterConnection { code, name: strings.read_tu()? })).collect::<Result<Vec<_>, String>>()?;
        properties.push(DwgBlockParameterProperty { connections });
    }
    let property_node_ids = [data.read_bl()?, data.read_bl()?, data.read_bl()?, data.read_bl()?];
    let base_location = match data.read_bs()? {
        0 => DwgBlockParameterBaseLocation::StartPoint,
        1 => DwgBlockParameterBaseLocation::Midpoint,
        value => return Err(format!("{class_name} base location {value} is unsupported")),
    };
    Ok((DwgBlockTwoPointParameter { element, show_properties, chain_actions, definition_base, definition_end, properties, property_expression_references: Vec::new(), base_location }, property_node_ids))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_r2010_block_action(data: &mut DwgBitReader<'_>, strings: &mut DwgBitReader<'_>, handles: &mut DwgBitReader<'_>, base: u64, class_name: &str) -> Result<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockAction, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockAction;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockActionDependency;
    let element = decode_r2010_block_element(data, strings, class_name)?;
    let display_location = data.read_3bd()?.to_vec();
    let dependency_count = data.read_bl()? as usize;
    let action_node_count = data.read_bl()? as usize;
    if dependency_count > 10_000 || action_node_count > 10_000 {
        return Err(format!("{class_name} action count is invalid"));
    }
    let action_node_ids = (0..action_node_count).map(|_| data.read_bl()).collect::<Result<Vec<_>, _>>()?;
    let dependencies = (0..dependency_count).map(|_| read_object_handle(handles, base)?.ok_or_else(|| format!("{class_name} dependency is null")).map(|object_handle| DwgBlockActionDependency { object_handle })).collect::<Result<Vec<_>, String>>()?;
    Ok(DwgBlockAction { evaluation_expression: element.evaluation_expression, name: element.name, display_location, dependencies, action_node_ids })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_object_common_relations(data: &mut DwgBitReader<'_>, handles: &mut DwgBitReader<'_>, base: u64) -> Result<DwgObjectRelations, String> {
    let reactor_count = data.read_bl()? as usize;
    let extension_dictionary_missing = data.read_b()?;
    let owner = read_object_handle(handles, base)?;
    let reactors = (0..reactor_count).map(|_| read_object_handle(handles, base).map(|handle| handle.unwrap_or_default())).collect::<Result<Vec<_>, _>>()?.into_iter().filter(|handle| *handle != 0).collect();
    let extension_dictionary = if extension_dictionary_missing { None } else { read_object_handle(handles, base)? };
    Ok((owner, reactors, extension_dictionary))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_r2010_constraint_node(class: &str, data: &mut DwgBitReader<'_>, handles: &mut DwgBitReader<'_>, base: u64) -> Result<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgConstraintNode, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAxisConstraint;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgConstrainedBoundedLine;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgConstrainedDatumLine;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgConstrainedImplicitPoint;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgConstraintGeometry;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgConstraintNode;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgConstraintNodeCore;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDistanceConstraint;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgExplicitConstraint;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgGeometricConstraint;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn core(data: &mut DwgBitReader<'_>) -> Result<DwgConstraintNodeCore, String> {
        let id = data.read_bl()? as i32;
        if id < 0 {
            return Err(format!("constraint node ID {id} is invalid"));
        }
        let count = data.read_bl()? as usize;
        if count > 10_000 {
            return Err(format!("constraint node {id} connection count {count} is invalid"));
        }
        let connected_node_ids = (0..count).map(|_| data.read_bl()).collect::<Result<Vec<_>, _>>()?;
        Ok(DwgConstraintNodeCore { id, connected_node_ids })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn geometric(data: &mut DwgBitReader<'_>) -> Result<DwgGeometricConstraint, String> {
        Ok(DwgGeometricConstraint { node: core(data)?, owner_node_id: data.read_bl()?, implied: data.read_b()?, active: true })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn geometry(data: &mut DwgBitReader<'_>, handles: &mut DwgBitReader<'_>, base: u64) -> Result<DwgConstraintGeometry, String> {
        Ok(DwgConstraintGeometry { node: core(data)?, geometry_dependency_handle: read_object_handle(handles, base)?, geometry_node_id: data.read_bl()? })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn explicit(data: &mut DwgBitReader<'_>, handles: &mut DwgBitReader<'_>, base: u64) -> Result<DwgExplicitConstraint, String> {
        Ok(DwgExplicitConstraint {
            geometric: geometric(data)?,
            value_dependency_handle: read_object_handle(handles, base)?.ok_or("explicit constraint value dependency is null")?,
            dimension_dependency_handle: read_object_handle(handles, base)?.ok_or("explicit constraint dimension dependency is null")?,
        })
    }
    Ok(match class {
        "AcConstrainedImplicitPoint" => {
            let geometry = geometry(data, handles, base)?;
            let point = geometry.geometry_dependency_handle.is_some().then(|| data.read_3bd().map(|value| value.to_vec())).transpose()?;
            DwgConstraintNode::ConstrainedImplicitPoint(DwgConstrainedImplicitPoint { geometry, point, point_kind: data.read_rc()?, point_index: data.read_bl()? as i32, curve_node_id: data.read_bl()? as i32 })
        }
        "AcPointCurveConstraint" => DwgConstraintNode::PointCurveConstraint(geometric(data)?),
        "AcConstrainedBoundedLine" => {
            let geometry = geometry(data, handles, base)?;
            let origin = data.read_3bd()?.to_vec();
            let direction = data.read_3bd()?.to_vec();
            let ray = data.read_b()?;
            DwgConstraintNode::ConstrainedBoundedLine(DwgConstrainedBoundedLine { geometry, origin, direction, ray, bounded: !ray, start_point: data.read_3bd()?.to_vec(), end_point: data.read_3bd()?.to_vec() })
        }
        "AcPointCoincidenceConstraint" => DwgConstraintNode::PointCoincidenceConstraint(geometric(data)?),
        "AcDistanceConstraint" => {
            let explicit = explicit(data, handles, base)?;
            let direction_kind = data.read_rc()?;
            let direction = (direction_kind != 0).then(|| data.read_3bd().map(|value| value.to_vec())).transpose()?;
            DwgConstraintNode::DistanceConstraint(DwgDistanceConstraint { explicit, direction_kind, direction })
        }
        "AcPerpendicularConstraint" => DwgConstraintNode::PerpendicularConstraint(geometric(data)?),
        "AcHorizontalConstraint" => DwgConstraintNode::HorizontalConstraint(DwgAxisConstraint { geometric: geometric(data)?, datum_line_index: data.read_bl()? as i32 }),
        "AcParallelConstraint" => DwgConstraintNode::ParallelConstraint(geometric(data)?),
        "AcMidPointConstraint" => DwgConstraintNode::MidPointConstraint(geometric(data)?),
        "AcEqualLengthConstraint" => DwgConstraintNode::EqualLengthConstraint(geometric(data)?),
        "AcColinearConstraint" => DwgConstraintNode::ColinearConstraint(geometric(data)?),
        "AcConstrainedDatumLine" => {
            let geometry = geometry(data, handles, base)?;
            DwgConstraintNode::ConstrainedDatumLine(DwgConstrainedDatumLine { geometry, origin: data.read_3bd()?.to_vec(), direction: data.read_3bd()?.to_vec() })
        }
        "AcFixedConstraint" => DwgConstraintNode::FixedConstraint(geometric(data)?),
        "AcVerticalConstraint" => DwgConstraintNode::VerticalConstraint(DwgAxisConstraint { geometric: geometric(data)?, datum_line_index: data.read_bl()? as i32 }),
        other => return Err(format!("unsupported ACDBASSOC2DCONSTRAINTGROUP node class {other}")),
    })
}
struct DwgDecodedEntityCommon {
    logical: semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityCommon,
    reactor_count: usize,
    extension_dictionary_missing: bool,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn entity_mode(value: u8) -> semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityMode {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityMode;
    match value {
        0 => DwgEntityMode::ExplicitOwner,
        1 => DwgEntityMode::PaperSpace,
        2 => DwgEntityMode::ModelSpace,
        _ => DwgEntityMode::Reserved,
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn entity_reference_mode(value: u8) -> semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityReferenceMode {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityReferenceMode;
    match value {
        0 => DwgEntityReferenceMode::ByLayer,
        1 => DwgEntityReferenceMode::ByBlock,
        2 => DwgEntityReferenceMode::Continuous,
        _ => DwgEntityReferenceMode::Explicit,
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_r2010_entity_common_main(data: &mut DwgBitReader<'_>) -> Result<DwgDecodedEntityCommon, String> {
    if data.read_b()? {
        return Err("R2010 entity graphic requires a typed semantic graphic model".into());
    }
    decode_entity_common_fields(data, DwgObjectLayout::R2010)
}
/// 🧬️ Every common-entity field AFTER the graphic-present bit, in ODA R2010 order. The one and only
/// reading of that layout in this codec: `decode_r2010_entity_common_main` reaches it after refusing
/// a graphic it cannot model, and the drawing-geometry decoder after skipping one it does not need.
///
/// ⚠️ Two version-gated details that a reader written against the R2000 layout gets wrong, and did:
/// the `nolinks` bit is R13–R2000 ONLY and must NOT be consumed here, and R2010 adds three
/// visual-style presence bits after `shadow_flags` — each one gating a handle in the handle stream.
/// Off by those two bits, every subsequent field of a real AutoCAD entity decodes as noise.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_entity_common_fields(data: &mut DwgBitReader<'_>, layout: DwgObjectLayout) -> Result<DwgDecodedEntityCommon, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityColor;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityColorKind;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityCommon;
    let mode = data.read_bb()?;
    let reactor_count = data.read_bl()? as usize;
    let extension_dictionary_missing = data.read_b()?;
    if layout >= DwgObjectLayout::R2013 {
        data.read_b()?;
    }
    let encoded_color = data.read_bs()?;
    let transparency = if encoded_color & 0x2000 != 0 { Some(data.read_bl()?) } else { None };
    let mut rgb = 0;
    if encoded_color & 0x4000 == 0 && encoded_color & 0x8000 != 0 {
        rgb = data.read_bl()?;
    }
    let index = encoded_color & 0x01ff;
    let color = DwgEntityColor {
        kind: if encoded_color & 0xc000 != 0 {
            DwgEntityColorKind::TrueColor
        } else if index == 256 {
            DwgEntityColorKind::ByLayer
        } else if index == 0 {
            DwgEntityColorKind::ByBlock
        } else {
            DwgEntityColorKind::Index
        },
        index,
        rgb,
        transparency,
        name: None,
        book_name: None,
        color_handle: None,
    };
    let linetype_scale = data.read_bd()?;
    let linetype = entity_reference_mode(data.read_bb()?);
    let plot_style = entity_reference_mode(data.read_bb()?);
    let (material, shadow) = if layout >= DwgObjectLayout::R2007 { (entity_reference_mode(data.read_bb()?), data.read_rc()?) } else { (entity_reference_mode(0), 0) };
    let (full_visual, face_visual, edge_visual) = if layout >= DwgObjectLayout::R2010 { (data.read_b()?, data.read_b()?, data.read_b()?) } else { (false, false, false) };
    let invisible = data.read_bs()?;
    let lineweight = data.read_rc()?;
    Ok(DwgDecodedEntityCommon {
        logical: DwgEntityCommon {
            mode: entity_mode(mode),
            color,
            linetype_scale,
            linetype,
            plot_style,
            material,
            shadow,
            invisible,
            lineweight,
            full_visual_style_handle: full_visual.then_some(0),
            face_visual_style_handle: face_visual.then_some(0),
            edge_visual_style_handle: edge_visual.then_some(0),
            ..Default::default()
        },
        reactor_count,
        extension_dictionary_missing,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_r2010_entity_common_handles(decoded: &mut DwgDecodedEntityCommon, handles: &mut DwgBitReader<'_>, base: u64) -> Result<DwgObjectRelations, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityColorKind;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityMode;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityReferenceMode;
    if decoded.logical.color.kind == DwgEntityColorKind::TrueColor && decoded.logical.color.rgb == 0 {
        decoded.logical.color.color_handle = read_object_handle(handles, base)?;
    }
    let owner = if decoded.logical.mode == DwgEntityMode::ExplicitOwner { read_object_handle(handles, base)? } else { None };
    let mut reactors = Vec::with_capacity(decoded.reactor_count);
    for index in 0..decoded.reactor_count {
        reactors.push(read_object_handle(handles, base)?.ok_or_else(|| format!("entity reactor {index} is null"))?);
    }
    let extension_dictionary = if decoded.extension_dictionary_missing { None } else { read_object_handle(handles, base)? };
    decoded.logical.layer_handle = read_object_handle(handles, base)?.ok_or("entity layer handle is null")?;
    if decoded.logical.linetype == DwgEntityReferenceMode::Explicit {
        decoded.logical.linetype_handle = read_object_handle(handles, base)?;
    }
    if decoded.logical.material == DwgEntityReferenceMode::Explicit {
        decoded.logical.material_handle = read_object_handle(handles, base)?;
    }
    if decoded.logical.shadow == 3 {
        decoded.logical.shadow_handle = read_object_handle(handles, base)?;
    }
    if decoded.logical.plot_style == DwgEntityReferenceMode::Explicit {
        decoded.logical.plot_style_handle = read_object_handle(handles, base)?;
    }
    if decoded.logical.full_visual_style_handle.is_some() {
        decoded.logical.full_visual_style_handle = read_object_handle(handles, base)?;
    }
    if decoded.logical.face_visual_style_handle.is_some() {
        decoded.logical.face_visual_style_handle = read_object_handle(handles, base)?;
    }
    if decoded.logical.edge_visual_style_handle.is_some() {
        decoded.logical.edge_visual_style_handle = read_object_handle(handles, base)?;
    }
    Ok((owner, reactors, extension_dictionary))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_entity_terminal_fill(reader: &mut DwgBitReader<'_>, end_bit: usize, handle: u64, class_name: &str) -> Result<(), String> {
    let terminal_bits = end_bit.checked_sub(reader.bit_position()).ok_or_else(|| format!("{class_name} {handle:#x} handle stream exceeds its frame"))?;
    if terminal_bits > 7 {
        return Err(format!("{class_name} {handle:#x} has {terminal_bits} trailing handle bits"));
    }
    for _ in 0..terminal_bits {
        if !reader.read_b()? {
            return Err(format!("{class_name} {handle:#x} terminal handle fill contains zero"));
        }
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_r2010_cmc_main(data: &mut DwgBitReader<'_>) -> Result<(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgComplexColor, u8), String> {
    let index = data.read_bs()?;
    let value = decode_complex_color_value(data.read_bl()?)?;
    let flags = data.read_rc()?;
    if flags > 3 {
        return Err(format!("CMC flags {flags:#x} are invalid"));
    }
    Ok((semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgComplexColor { index, value, name: None, book_name: None }, flags))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_table_style_color(data: &mut DwgBitReader<'_>, handle: u64) -> Result<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgComplexColor, String> {
    let (color, flags) = read_r2010_cmc_main(data)?;
    if flags != 0 {
        return Err(format!("TABLESTYLE {handle:#x} named colors are unsupported"));
    }
    Ok(color)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_material_color(data: &mut DwgBitReader<'_>, handle: u64) -> Result<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgMaterialColor, String> {
    let source = data.read_rc()?;
    if source > 1 {
        return Err(format!("MATERIAL {handle:#x} color source {source} is unsupported"));
    }
    Ok(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgMaterialColor { factor: data.read_bd()?, override_rgb: if source == 1 { Some(data.read_bl()?) } else { None } })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_material_map(data: &mut DwgBitReader<'_>, strings: &mut DwgBitReader<'_>, handle: u64) -> Result<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgMaterialMap, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgMaterialMap;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgMaterialMapSource;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgMaterialProjection;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgMaterialTiling;
    let blend_factor = data.read_bd()?;
    let projection = match data.read_rc()? {
        0 => DwgMaterialProjection::Inherit,
        1 => DwgMaterialProjection::Planar,
        2 => DwgMaterialProjection::Box,
        3 => DwgMaterialProjection::Cylinder,
        4 => DwgMaterialProjection::Sphere,
        value => return Err(format!("MATERIAL {handle:#x} projection {value} is unsupported")),
    };
    let tiling = match data.read_rc()? {
        0 => DwgMaterialTiling::Inherit,
        1 => DwgMaterialTiling::Tile,
        2 => DwgMaterialTiling::Crop,
        3 => DwgMaterialTiling::Clamp,
        4 => DwgMaterialTiling::Mirror,
        value => return Err(format!("MATERIAL {handle:#x} tiling {value} is unsupported")),
    };
    let auto_transform = data.read_rc()?;
    let (scale_to_entity, use_current_block_transform) = if auto_transform == 1 { (false, false) } else { (auto_transform & 2 != 0, auto_transform & 4 != 0) };
    if auto_transform & !7 != 0 {
        return Err(format!("MATERIAL {handle:#x} auto-transform {auto_transform:#x} is unsupported"));
    }
    let transform = (0..16).map(|_| data.read_bd()).collect::<Result<Vec<_>, _>>()?;
    let source = match data.read_rc()? {
        0 => DwgMaterialMapSource::CurrentScene,
        1 => {
            let filename = strings.read_tu()?;
            if !filename.is_empty() {
                return Err(format!("MATERIAL {handle:#x} external texture file is not yet supported"));
            }
            DwgMaterialMapSource::None
        }
        value => return Err(format!("MATERIAL {handle:#x} map source {value} is unsupported")),
    };
    Ok(DwgMaterialMap { blend_factor, projection, tiling, scale_to_entity, use_current_block_transform, transform, source })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_r2010_cell_style(data: &mut DwgBitReader<'_>, strings: &mut DwgBitReader<'_>, handles: &mut DwgBitReader<'_>, base: u64) -> Result<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgCellStyle, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgCellBorder;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgCellBorders;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgCellContentFormat;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgCellMargins;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgCellStyle;
    if data.read_bl()? != 5 || data.read_bs()? != 1 {
        return Err(format!("TABLESTYLE {base:#x} cell type or data flag is unsupported"));
    }
    let property_override_flags = data.read_bl()?;
    let merge_flags = data.read_bl()?;
    let background_color = read_table_style_color(data, base)?;
    let content_layout = data.read_bl()?;
    let content_property_override_flags = data.read_bl()?;
    let content_property_flags = data.read_bl()?;
    let value_data_type = data.read_bl()?;
    let value_unit_type = data.read_bl()?;
    let value_format_string = strings.read_tu()?;
    let rotation = data.read_bd()?;
    let block_scale = data.read_bd()?;
    let alignment = data.read_bl()?;
    let content_color = read_table_style_color(data, base)?;
    let text_style_handle = read_object_handle(handles, base)?;
    let text_height = data.read_bd()?;
    if data.read_bs()? != 1 {
        return Err(format!("TABLESTYLE {base:#x} cell margins are missing"));
    }
    let margins = DwgCellMargins { vertical: data.read_bd()?, horizontal: data.read_bd()?, bottom: data.read_bd()?, right: data.read_bd()?, horizontal_spacing: data.read_bd()?, vertical_spacing: data.read_bd()? };
    let border_count = data.read_bl()? as usize;
    if border_count > 6 {
        return Err(format!("TABLESTYLE {base:#x} border count {border_count} is invalid"));
    }
    let mut borders = DwgCellBorders::default();
    for _ in 0..border_count {
        let mask = data.read_bl()?;
        let border = DwgCellBorder {
            override_flags: data.read_bl()?,
            border_type: data.read_bl()?,
            color: read_table_style_color(data, base)?,
            lineweight: data.read_bl()? as i32,
            linetype_handle: read_object_handle(handles, base)?,
            visible: data.read_bl()?,
            double_line_spacing: data.read_bd()?,
        };
        let slot = match mask {
            1 => &mut borders.top,
            2 => &mut borders.horizontal_inside,
            4 => &mut borders.bottom,
            8 => &mut borders.left,
            16 => &mut borders.vertical_inside,
            32 => &mut borders.right,
            _ => return Err(format!("TABLESTYLE {base:#x} border mask {mask} is invalid")),
        };
        if slot.replace(border).is_some() {
            return Err(format!("TABLESTYLE {base:#x} repeats border mask {mask}"));
        }
    }
    Ok(DwgCellStyle {
        property_override_flags,
        merge_flags,
        background_color,
        content_layout,
        content_format: DwgCellContentFormat {
            property_override_flags: content_property_override_flags,
            property_flags: content_property_flags,
            value_data_type,
            value_unit_type,
            value_format_string,
            rotation,
            block_scale,
            alignment,
            content_color,
            text_style_handle,
            text_height,
        },
        margins,
        borders,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_complex_color_value(word: u32) -> Result<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgComplexColorValue, String> {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgComplexColorValue;
    let value = word & 0x00ff_ffff;
    match word >> 24 {
        0xc0 => Ok(DwgComplexColorValue::ByLayer),
        0xc1 => Ok(DwgComplexColorValue::ByBlock),
        0xc2 => Ok(DwgComplexColorValue::ByColor { red: ((value >> 16) & 0xff) as u8, green: ((value >> 8) & 0xff) as u8, blue: (value & 0xff) as u8 }),
        0xc3 => Ok(DwgComplexColorValue::ByAci { index: u16::try_from(value).map_err(|_| format!("ACI color {value} exceeds u16"))? }),
        0xc4 => Ok(DwgComplexColorValue::ByPen { index: u8::try_from(value).map_err(|_| format!("pen color {value} exceeds u8"))? }),
        0xc5 => Ok(DwgComplexColorValue::Foreground),
        0xc6 => Ok(DwgComplexColorValue::LayerOff),
        0xc7 => Ok(DwgComplexColorValue::LayerFrozen),
        0xc8 if value == 0 => Ok(DwgComplexColorValue::None),
        method => Err(format!("complex-color method {method:#x} with value {value:#x} is unsupported")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_r2004_object_records(bytes: &[u8], classes: &[semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgClass]) -> Result<DwgObjectRecords, String> {
    let sections = decode_r2004_sections(bytes)?;
    let handles_section = sections.iter().find(|section| section.name == "AcDb:Handles").ok_or("R2004 Handles section missing")?;
    let objects_section = sections.iter().find(|section| section.name == "AcDb:AcDbObjects").ok_or("R2004 AcDbObjects section missing")?;
    let mut handle_map = decode_r2004_handle_map(&r2004_section_data(handles_section)?)?;
    handle_map.sort_by_key(|(_, address)| *address);
    let object_data = r2004_section_data(objects_section)?;
    let mut objects = Vec::with_capacity(handle_map.len());
    let mut xrecord_terminal_fills = Vec::new();
    let mut block_names = Vec::new();
    for (handle, address) in handle_map {
        let frame_bytes = object_data.get(address..).ok_or_else(|| format!("object {handle:#x} address {address} exceeds AcDbObjects"))?;
        let mut frame = DwgBitReader::new(frame_bytes);
        let payload_size = frame.read_ms().map(|value| value as usize).map_err(|error| format!("object {handle:#x} payload size: {error}"))?;
        let handle_stream_bits = frame.read_umc().map(|value| value as usize).map_err(|error| format!("object {handle:#x} handle-stream size: {error}"))?;
        frame.pad_to_byte();
        let payload_end = frame.byte_pos.checked_add(payload_size).ok_or_else(|| format!("object {handle:#x} payload end overflow"))?;
        let payload = frame_bytes.get(frame.byte_pos..payload_end).ok_or_else(|| format!("object {handle:#x} payload is truncated"))?;
        let stored_crc = frame_bytes.get(payload_end..payload_end + 2).map(|bytes| u16::from_le_bytes([bytes[0], bytes[1]])).ok_or_else(|| format!("object {handle:#x} CRC is truncated"))?;
        let computed_crc = dwg_crc16(0xC0C1, &frame_bytes[..payload_end]);
        if stored_crc != computed_crc {
            return Err(format!("object {handle:#x} CRC mismatch: stored {stored_crc:#06x}, computed {computed_crc:#06x}"));
        }
        let data_end_bit = payload_size.checked_mul(8).and_then(|bits| bits.checked_sub(handle_stream_bits)).ok_or_else(|| format!("object {handle:#x} handle-stream size exceeds payload"))?;
        let mut data = DwgBitReader::new(payload);
        let mut handle_reader = DwgBitReader::at_bit(payload, data_end_bit).map_err(|error| format!("object {handle:#x} handle-stream start: {error}"))?;
        let type_code = data.read_bot().map_err(|error| format!("object {handle:#x} type: {error}"))?;
        let (_, object_handle) = data.read_handle().map_err(|error| format!("object {handle:#x} self handle: {error}"))?;
        if object_handle != handle {
            return Err(format!("object {handle:#x} frame self handle is {object_handle:#x}"));
        }
        let fixed = fixed_object_name(type_code);
        let class_name = if fixed == "UNKNOWN" { classes.iter().find(|class| class.number == type_code).map_or_else(|| format!("CLASS_{type_code}"), |class| class.dxf_name.clone()) } else { fixed.to_string() };
        let category = object_category(type_code);
        let extended_data = decode_r2010_eed(&mut data, handle).map_err(|error| format!("object {handle:#x} EED: {error}"))?;
        let mut object = semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject { handle, type_code, class_name, category, extended_data, ..Default::default() };
        if category == semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgObjectCategory::Entity {
            if type_code == DWG_TYPE_BLOCK || type_code == DWG_TYPE_ENDBLK {
                let (mut strings, class_main_end) = r2010_string_stream(payload, data_end_bit).map_err(|error| format!("{} {handle:#x} string stream: {error}", if type_code == DWG_TYPE_BLOCK { "BLOCK" } else { "ENDBLK" }))?;
                let mut common = decode_r2010_entity_common_main(&mut data).map_err(|error| format!("{} {handle:#x} common data: {error}", if type_code == DWG_TYPE_BLOCK { "BLOCK" } else { "ENDBLK" }))?;
                if data.bit_position() != class_main_end {
                    return Err(format!("{} {handle:#x} main stream is not exactly consumed: {} != {class_main_end}", if type_code == DWG_TYPE_BLOCK { "BLOCK" } else { "ENDBLK" }, data.bit_position()));
                }
                if type_code == DWG_TYPE_BLOCK {
                    let name = strings.read_tu().map_err(|error| format!("BLOCK {handle:#x} name: {error}"))?;
                    let string_end = r2010_string_content_end_bit(payload, data_end_bit)?;
                    if strings.bit_position() != string_end {
                        return Err(format!("BLOCK {handle:#x} string stream is not exactly consumed: {} != {string_end}", strings.bit_position()));
                    }
                    block_names.push((handle, name));
                } else if class_main_end != data_end_bit.saturating_sub(1) {
                    return Err(format!("ENDBLK {handle:#x} unexpectedly declares a string stream"));
                }
                let (owner, reactors, extension_dictionary) =
                    decode_r2010_entity_common_handles(&mut common, &mut handle_reader, handle).map_err(|error| format!("{} {handle:#x} common handles: {error}", if type_code == DWG_TYPE_BLOCK { "BLOCK" } else { "ENDBLK" }))?;
                validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, if type_code == DWG_TYPE_BLOCK { "BLOCK" } else { "ENDBLK" })?;
                object.owner_handle = owner;
                object.reactor_handles = reactors;
                object.extension_dictionary_handle = extension_dictionary;
                object.body = Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::Entity(if type_code == DWG_TYPE_BLOCK {
                    semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityBody::BlockBegin(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockBeginEntity { common: common.logical })
                } else {
                    semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityBody::BlockEnd(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockEndEntity { common: common.logical })
                }));
            } else if type_code == DWG_TYPE_INSERT {
                let mut common = decode_r2010_entity_common_main(&mut data).map_err(|error| format!("INSERT {handle:#x} common data: {error}"))?;
                let insertion = data.read_3bd().map_err(|error| format!("INSERT {handle:#x} insertion: {error}"))?;
                let scale = match data.read_bb().map_err(|error| format!("INSERT {handle:#x} scale mode: {error}"))? {
                    0 => {
                        let x = data.read_rd()?;
                        [x, data.read_dd(x)?, data.read_dd(x)?]
                    }
                    1 => [1.0, data.read_dd(1.0)?, data.read_dd(1.0)?],
                    2 => {
                        let value = data.read_rd()?;
                        [value; 3]
                    }
                    3 => [1.0; 3],
                    _ => unreachable!(),
                };
                let rotation = data.read_bd().map_err(|error| format!("INSERT {handle:#x} rotation: {error}"))?;
                let extrusion = data.read_3bd().map_err(|error| format!("INSERT {handle:#x} extrusion: {error}"))?;
                let has_attributes = data.read_b().map_err(|error| format!("INSERT {handle:#x} attributes flag: {error}"))?;
                let attribute_count = if has_attributes { data.read_bl().map_err(|error| format!("INSERT {handle:#x} attribute count: {error}"))? as usize } else { 0 };
                if data.read_b().map_err(|error| format!("INSERT {handle:#x} string marker: {error}"))? || data.bit_position() != data_end_bit {
                    return Err(format!("INSERT {handle:#x} main/string stream is not exactly consumed"));
                }
                let (owner, reactors, extension_dictionary) = decode_r2010_entity_common_handles(&mut common, &mut handle_reader, handle)?;
                let block_header_handle = read_object_handle(&mut handle_reader, handle).map_err(|error| format!("INSERT {handle:#x} block header: {error}"))?.ok_or_else(|| format!("INSERT {handle:#x} block-header handle is null"))?;
                let mut attribute_handles = Vec::with_capacity(attribute_count);
                for index in 0..attribute_count {
                    attribute_handles.push(read_object_handle(&mut handle_reader, handle).map_err(|error| format!("INSERT {handle:#x} attribute {index}: {error}"))?.ok_or_else(|| format!("INSERT {handle:#x} attribute {index} is null"))?);
                }
                let sequence_end_handle = if has_attributes { read_object_handle(&mut handle_reader, handle).map_err(|error| format!("INSERT {handle:#x} SEQEND: {error}"))? } else { None };
                if has_attributes && sequence_end_handle.is_none() {
                    return Err(format!("INSERT {handle:#x} attributes have no SEQEND"));
                }
                validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "INSERT")?;
                object.owner_handle = owner;
                object.reactor_handles = reactors;
                object.extension_dictionary_handle = extension_dictionary;
                object.body = Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::Entity(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityBody::Insert(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgInsertEntity {
                    common: common.logical,
                    insertion: insertion.to_vec(),
                    scale: scale.to_vec(),
                    rotation,
                    extrusion: extrusion.to_vec(),
                    block_header_handle,
                    attribute_handles,
                    sequence_end_handle,
                })));
            } else if type_code == DWG_TYPE_DIMENSION_LINEAR {
                let (mut strings, class_main_end) = r2010_string_stream(payload, data_end_bit).map_err(|error| format!("DIMENSION_LINEAR {handle:#x} string stream: {error}"))?;
                let mut common = decode_r2010_entity_common_main(&mut data).map_err(|error| format!("DIMENSION_LINEAR {handle:#x} common data: {error}"))?;
                let class_version = data.read_rc()?;
                if class_version != 0 {
                    return Err(format!("DIMENSION_LINEAR {handle:#x} class version {class_version} is unsupported"));
                }
                let extrusion = data.read_3bd()?;
                let text_midpoint = data.read_2rd()?;
                let elevation = data.read_bd()?;
                let flag = data.read_rc()?;
                let block_reference_is_exclusive = flag & 0x20 != 0;
                let user_positioned_text = flag & 0x80 != 0;
                let expected_flag = 0x08 | (if block_reference_is_exclusive { 0x22 } else { 0 }) | (if user_positioned_text { 0x80 } else { 0x01 });
                if flag != expected_flag {
                    return Err(format!("DIMENSION_LINEAR {handle:#x} flag {flag:#x} disagrees with its semantic mirror bits; expected {expected_flag:#x}"));
                }
                let text_rotation = data.read_bd()?;
                let horizontal_direction = data.read_bd()?;
                let insertion_scale = data.read_3bd()?;
                let insertion_rotation = data.read_bd()?;
                let attachment = dimension_attachment_logical(data.read_bs()?)?;
                let line_spacing_style = dimension_spacing_logical(data.read_bs()?)?;
                let line_spacing_factor = data.read_bd()?;
                let actual_measurement = data.read_bd()?;
                if data.read_b()? {
                    return Err(format!("DIMENSION_LINEAR {handle:#x} reserved flag is set"));
                }
                let flip_arrow_1 = data.read_b()?;
                let flip_arrow_2 = data.read_b()?;
                let clone_insertion_point = data.read_2rd()?;
                let extension_line_1 = data.read_3bd()?;
                let extension_line_2 = data.read_3bd()?;
                let definition_point = data.read_3bd()?;
                let oblique_angle = data.read_bd()?;
                let dimension_rotation = data.read_bd()?;
                if data.bit_position() != class_main_end {
                    return Err(format!("DIMENSION_LINEAR {handle:#x} main stream is not exactly consumed: {} != {class_main_end}", data.bit_position()));
                }
                let user_text = strings.read_tu()?;
                let string_end = r2010_string_content_end_bit(payload, data_end_bit)?;
                if strings.bit_position() != string_end {
                    return Err(format!("DIMENSION_LINEAR {handle:#x} string stream is not exactly consumed: {} != {string_end}", strings.bit_position()));
                }
                let (owner, reactors, extension_dictionary) = decode_r2010_entity_common_handles(&mut common, &mut handle_reader, handle)?;
                let dimension_style_handle = read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("DIMENSION_LINEAR {handle:#x} dimension style is null"))?;
                let dimension_block_handle = read_object_handle(&mut handle_reader, handle)?;
                validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "DIMENSION_LINEAR")?;
                object.owner_handle = owner;
                object.reactor_handles = reactors;
                object.extension_dictionary_handle = extension_dictionary;
                object.body = Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::Entity(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityBody::DimensionLinear(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLinearDimensionEntity {
                    dimension: semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDimensionEntityCommon {
                        common: common.logical,
                        extrusion: extrusion.to_vec(),
                        text_midpoint: text_midpoint.to_vec(),
                        elevation,
                        status: semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDimensionStatus { block_reference_is_exclusive, user_positioned_text },
                        user_text,
                        text_rotation,
                        horizontal_direction,
                        insertion_scale: insertion_scale.to_vec(),
                        insertion_rotation,
                        attachment,
                        line_spacing_style,
                        line_spacing_factor,
                        actual_measurement,
                        flip_arrow_1,
                        flip_arrow_2,
                        clone_insertion_point: clone_insertion_point.to_vec(),
                        dimension_style_handle,
                        dimension_block_handle,
                    },
                    extension_line_1: extension_line_1.to_vec(),
                    extension_line_2: extension_line_2.to_vec(),
                    definition_point: definition_point.to_vec(),
                    oblique_angle,
                    dimension_rotation,
                })));
            } else if type_code == DWG_TYPE_VIEWPORT {
                use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgComplexColor;
                use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDefaultLightingType;
                use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEntityBody;
                use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
                use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgOrthographicView;
                use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgShadePlotMode;
                use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgViewportEntity;
                use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgViewportRenderMode;
                let (mut strings, class_main_end) = r2010_string_stream(payload, data_end_bit).map_err(|error| format!("VIEWPORT {handle:#x} string stream: {error}"))?;
                let mut common = decode_r2010_entity_common_main(&mut data).map_err(|error| format!("VIEWPORT {handle:#x} common data: {error}"))?;
                let center = data.read_3bd()?;
                let width = data.read_bd()?;
                let height = data.read_bd()?;
                let view_target = data.read_3bd()?;
                let view_direction = data.read_3bd()?;
                let twist_angle = data.read_bd()?;
                let view_height = data.read_bd()?;
                let lens_length = data.read_bd()?;
                let front_clip = data.read_bd()?;
                let back_clip = data.read_bd()?;
                let snap_angle = data.read_bd()?;
                let view_center = data.read_2rd()?;
                let snap_base = data.read_2rd()?;
                let snap_unit = data.read_2rd()?;
                let grid_unit = data.read_2rd()?;
                let circle_zoom_percent = data.read_bs()?;
                let grid_major = data.read_bs()?;
                let frozen_count = data.read_bl()? as usize;
                let status = viewport_status_flags(data.read_bl()?)?;
                let render_mode = match data.read_rc()? {
                    0 => DwgViewportRenderMode::Optimized2d,
                    1 => DwgViewportRenderMode::Wireframe,
                    2 => DwgViewportRenderMode::HiddenLine,
                    3 => DwgViewportRenderMode::FlatShaded,
                    4 => DwgViewportRenderMode::GouraudShaded,
                    5 => DwgViewportRenderMode::FlatShadedWithWireframe,
                    6 => DwgViewportRenderMode::GouraudShadedWithWireframe,
                    value => return Err(format!("VIEWPORT {handle:#x} render mode {value} is unsupported")),
                };
                let ucs_at_origin = data.read_b()?;
                let ucs_per_viewport = data.read_b()?;
                let ucs_origin = data.read_3bd()?;
                let ucs_x_axis = data.read_3bd()?;
                let ucs_y_axis = data.read_3bd()?;
                let ucs_elevation = data.read_bd()?;
                let orthographic_view = match data.read_bs()? {
                    0 => DwgOrthographicView::None,
                    1 => DwgOrthographicView::Top,
                    2 => DwgOrthographicView::Bottom,
                    3 => DwgOrthographicView::Front,
                    4 => DwgOrthographicView::Back,
                    5 => DwgOrthographicView::Left,
                    6 => DwgOrthographicView::Right,
                    value => return Err(format!("VIEWPORT {handle:#x} orthographic view {value} is unsupported")),
                };
                let shade_plot_mode = match data.read_bs()? {
                    0 => DwgShadePlotMode::AsDisplayed,
                    1 => DwgShadePlotMode::Wireframe,
                    2 => DwgShadePlotMode::Hidden,
                    3 => DwgShadePlotMode::Rendered,
                    value => return Err(format!("VIEWPORT {handle:#x} shade-plot mode {value} is unsupported")),
                };
                let use_default_lights = data.read_b()?;
                let default_lighting_type = match data.read_rc()? {
                    0 => DwgDefaultLightingType::OneDistantLight,
                    1 => DwgDefaultLightingType::TwoDistantLights,
                    value => return Err(format!("VIEWPORT {handle:#x} lighting type {value} is unsupported")),
                };
                let brightness = data.read_bd()?;
                let contrast = data.read_bd()?;
                let ambient_index = data.read_bs()?;
                let ambient_value = decode_complex_color_value(data.read_bl()?)?;
                let ambient_flags = data.read_rc()?;
                if ambient_flags & !3 != 0 {
                    return Err(format!("VIEWPORT {handle:#x} ambient color flags {ambient_flags:#x} are unsupported"));
                }
                if data.bit_position() != class_main_end {
                    return Err(format!("VIEWPORT {handle:#x} main stream is not exactly consumed: {} != {class_main_end}", data.bit_position()));
                }
                let style_sheet = strings.read_tu()?;
                let ambient_name = if ambient_flags & 1 != 0 { Some(strings.read_tu()?) } else { None };
                let ambient_book_name = if ambient_flags & 2 != 0 { Some(strings.read_tu()?) } else { None };
                let string_end = r2010_string_content_end_bit(payload, data_end_bit)?;
                if strings.bit_position() != string_end {
                    return Err(format!("VIEWPORT {handle:#x} string stream is not exactly consumed: {} != {string_end}", strings.bit_position()));
                }
                let (owner, reactors, extension_dictionary) = decode_r2010_entity_common_handles(&mut common, &mut handle_reader, handle)?;
                let mut frozen_layer_handles = Vec::with_capacity(frozen_count);
                for index in 0..frozen_count {
                    frozen_layer_handles.push(read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("VIEWPORT {handle:#x} frozen layer {index} is null"))?);
                }
                let clip_boundary_handle = read_object_handle(&mut handle_reader, handle)?;
                let named_ucs_handle = read_object_handle(&mut handle_reader, handle)?;
                let base_ucs_handle = read_object_handle(&mut handle_reader, handle)?;
                let background_handle = read_object_handle(&mut handle_reader, handle)?;
                let visual_style_handle = read_object_handle(&mut handle_reader, handle)?;
                let shade_plot_handle = read_object_handle(&mut handle_reader, handle)?;
                let sun_handle = read_object_handle(&mut handle_reader, handle)?;
                validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "VIEWPORT")?;
                object.owner_handle = owner;
                object.reactor_handles = reactors;
                object.extension_dictionary_handle = extension_dictionary;
                object.body = Some(DwgLogicalObjectBody::Entity(DwgEntityBody::Viewport(DwgViewportEntity {
                    common: common.logical,
                    center: center.to_vec(),
                    width,
                    height,
                    view_target: view_target.to_vec(),
                    view_direction: view_direction.to_vec(),
                    twist_angle,
                    view_height,
                    lens_length,
                    front_clip,
                    back_clip,
                    snap_angle,
                    view_center: view_center.to_vec(),
                    snap_base: snap_base.to_vec(),
                    snap_unit: snap_unit.to_vec(),
                    grid_unit: grid_unit.to_vec(),
                    circle_zoom_percent,
                    grid_major,
                    frozen_layer_handles,
                    status,
                    style_sheet,
                    render_mode,
                    ucs_at_origin,
                    ucs_per_viewport,
                    ucs_origin: ucs_origin.to_vec(),
                    ucs_x_axis: ucs_x_axis.to_vec(),
                    ucs_y_axis: ucs_y_axis.to_vec(),
                    ucs_elevation,
                    orthographic_view,
                    shade_plot_mode,
                    use_default_lights,
                    default_lighting_type,
                    brightness,
                    contrast,
                    ambient_color: DwgComplexColor { index: ambient_index, value: ambient_value, name: ambient_name, book_name: ambient_book_name },
                    clip_boundary_handle,
                    named_ucs_handle,
                    base_ucs_handle,
                    background_handle,
                    visual_style_handle,
                    shade_plot_handle,
                    sun_handle,
                })));
            } else {
                let mut frame = DwgObjectRecordFrame::open(DwgObjectLayout::R2010, &object_data, handle, address)?;
                frame.data = data;
                if let Some((body, (owner, reactors, extension_dictionary))) = decode_entity_body(&mut frame, false)? {
                    validate_entity_terminal_fill(&mut frame.handles, frame.end_bit, handle, &object.class_name)?;
                    object.owner_handle = owner;
                    object.reactor_handles = reactors;
                    object.extension_dictionary_handle = extension_dictionary;
                    object.body = Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::Entity(body));
                }
            }
            objects.push(object);
            continue;
        }

        let common_relations = decode_object_common_relations(&mut data, &mut handle_reader, handle);
        let (owner, reactors, extension_dictionary) = match common_relations {
            Ok(value) => value,
            Err(error)
                if type_code == 79
                    || object.class_name == "XRECORD"
                    || type_code == 506
                    || object.class_name == "VISUALSTYLE"
                    || type_code == 543
                    || object.class_name == "BLOCKPARAMDEPENDENCYBODY"
                    || type_code == 559
                    || object.class_name == "ACDB_BLOCKREPRESENTATION_DATA"
                    || type_code == 522
                    || object.class_name == "ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION"
                    || type_code == 517
                    || object.class_name == "ACAD_EVALUATION_GRAPH"
                    || type_code == 529
                    || object.class_name == "BLOCKFLIPPARAMETER"
                    || type_code == 531
                    || object.class_name == "BLOCKVISIBILITYPARAMETER"
                    || type_code == 80
                    || object.class_name == "ACDBPLACEHOLDER"
                    || type_code == 503
                    || object.class_name == "DICTIONARYVAR"
                    || type_code == 507
                    || object.class_name == "SCALE"
                    || type_code == 516
                    || object.class_name == "SORTENTSTABLE"
                    || type_code == 504
                    || object.class_name == "TABLESTYLE"
                    || type_code == 73
                    || object.class_name == "MLINESTYLE"
                    || type_code == 508
                    || object.class_name == "MLEADERSTYLE"
                    || type_code == 505
                    || object.class_name == "MATERIAL"
                    || type_code == 82
                    || object.class_name == "LAYOUT"
                    || type_code == 521
                    || object.class_name == "BLOCKMOVEACTION"
                    || type_code == 539
                    || object.class_name == "ACDBASSOCNETWORK"
                    || type_code == 540
                    || object.class_name == "ACDBASSOC2DCONSTRAINTGROUP"
                    || matches!(type_code, 527 | 528 | 530 | 532 | 533 | 534 | 535 | 536 | 537 | 538 | 546 | 548)
                    || matches!(
                        object.class_name.as_str(),
                        "BLOCKLINEARPARAMETER"
                            | "BLOCKLINEARGRIP"
                            | "BLOCKFLIPGRIP"
                            | "BLOCKVISIBILITYGRIP"
                            | "BLOCKALIGNMENTPARAMETER"
                            | "BLOCKALIGNMENTGRIP"
                            | "BLOCKSTRETCHACTION"
                            | "BLOCKSCALEACTION"
                            | "BLOCKFLIPACTION"
                            | "BLOCKBASEPOINTPARAMETER"
                            | "BLOCKVERTICALCONSTRAINTPARAMETER"
                            | "BLOCKHORIZONTALCONSTRAINTPARAMETER"
                    ) =>
            {
                return Err(format!("{} {handle:#x} common data: {error}", object.class_name));
            }
            Err(_) => {
                objects.push(object);
                continue;
            }
        };
        object.owner_handle = owner;
        object.reactor_handles = reactors;
        object.extension_dictionary_handle = extension_dictionary;
        let string_stream = r2010_string_stream(payload, data_end_bit).ok();
        let main_end_bit = string_stream.as_ref().map_or(data_end_bit, |(_, start)| *start);
        let mut strings = string_stream.map(|(reader, _)| reader);

        if type_code == 82 || object.class_name == "LAYOUT" {
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLayout;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLayoutOptions;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgOrthographicView;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgPlotArea;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgPlotOptions;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgPlotPaperUnit;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgPlotRotation;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgShadePlot;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgShadePlotResolution;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgStandardScale;
            let strings = strings.as_mut().ok_or_else(|| format!("LAYOUT {handle:#x} string stream missing"))?;
            let page_setup_name = strings.read_tu()?;
            let printer_configuration = strings.read_tu()?;
            let flags = data.read_bs()?;
            if flags & !0x2eb0 != 0 {
                return Err(format!("LAYOUT {handle:#x} plot options {flags:#x} are unsupported"));
            }
            let margins = (0..4).map(|_| data.read_bd()).collect::<Result<Vec<_>, _>>()?;
            let paper_size = (0..2).map(|_| data.read_bd()).collect::<Result<Vec<_>, _>>()?;
            let canonical_media_name = strings.read_tu()?;
            let plot_origin = (0..2).map(|_| data.read_bd()).collect::<Result<Vec<_>, _>>()?;
            let paper_unit = match data.read_bs()? {
                0 => DwgPlotPaperUnit::Inches,
                value => return Err(format!("LAYOUT {handle:#x} paper unit {value} is unsupported")),
            };
            let rotation = match data.read_bs()? {
                1 => DwgPlotRotation::QuarterTurn,
                value => return Err(format!("LAYOUT {handle:#x} rotation {value} is unsupported")),
            };
            let plot_area = match data.read_bs()? {
                0 => DwgPlotArea::Display,
                5 => DwgPlotArea::Layout,
                value => return Err(format!("LAYOUT {handle:#x} plot area {value} is unsupported")),
            };
            let plot_window_lower_left = (0..2).map(|_| data.read_bd()).collect::<Result<Vec<_>, _>>()?;
            let plot_window_upper_right = (0..2).map(|_| data.read_bd()).collect::<Result<Vec<_>, _>>()?;
            let plot_view_handle = read_object_handle(&mut handle_reader, handle)?;
            let paper_units = data.read_bd()?;
            let drawing_units = data.read_bd()?;
            let stylesheet = strings.read_tu()?;
            let standard_scale = match data.read_bs()? {
                0 => DwgStandardScale::Custom,
                16 => DwgStandardScale::OneToOne,
                value => return Err(format!("LAYOUT {handle:#x} standard scale {value} is unsupported")),
            };
            let standard_scale_factor = data.read_bd()?;
            let paper_image_origin = (0..2).map(|_| data.read_bd()).collect::<Result<Vec<_>, _>>()?;
            let shade_plot = match data.read_bs()? {
                0 => DwgShadePlot::AsDisplayed,
                value => return Err(format!("LAYOUT {handle:#x} shade plot {value} is unsupported")),
            };
            let shade_plot_resolution = match data.read_bs()? {
                2 => DwgShadePlotResolution::Normal,
                value => return Err(format!("LAYOUT {handle:#x} shade resolution {value} is unsupported")),
            };
            let shade_plot_dpi = data.read_bs()?;
            let visual_style_handle = read_object_handle(&mut handle_reader, handle)?;
            let name = strings.read_tu()?;
            let tab_order = data.read_bs()?;
            let layout_flags = data.read_bs()?;
            if layout_flags & !1 != 0 {
                return Err(format!("LAYOUT {handle:#x} options {layout_flags:#x} are unsupported"));
            }
            let insertion_base = data.read_3bd()?.to_vec();
            let limits_minimum = data.read_2rd()?.to_vec();
            let limits_maximum = data.read_2rd()?.to_vec();
            let ucs_origin = data.read_3bd()?.to_vec();
            let ucs_x_axis = data.read_3bd()?.to_vec();
            let ucs_y_axis = data.read_3bd()?.to_vec();
            let ucs_elevation = data.read_bd()?;
            let orthographic_view = match data.read_bs()? {
                0 => DwgOrthographicView::None,
                1 => DwgOrthographicView::Top,
                2 => DwgOrthographicView::Bottom,
                3 => DwgOrthographicView::Front,
                4 => DwgOrthographicView::Back,
                5 => DwgOrthographicView::Left,
                6 => DwgOrthographicView::Right,
                value => return Err(format!("LAYOUT {handle:#x} orthographic view {value} is unsupported")),
            };
            let extents_minimum = data.read_3bd()?.to_vec();
            let extents_maximum = data.read_3bd()?.to_vec();
            let viewport_count = data.read_bl()? as usize;
            if viewport_count > 10_000 {
                return Err(format!("LAYOUT {handle:#x} viewport count is invalid"));
            }
            let block_header_handle = read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("LAYOUT {handle:#x} block header is null"))?;
            let active_viewport_handle = read_object_handle(&mut handle_reader, handle)?;
            let base_ucs_handle = read_object_handle(&mut handle_reader, handle)?;
            let named_ucs_handle = read_object_handle(&mut handle_reader, handle)?;
            let viewport_handles = (0..viewport_count).map(|_| read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("LAYOUT {handle:#x} viewport is null"))).collect::<Result<Vec<_>, String>>()?;
            if data.bit_position() != main_end_bit || strings.bit_position() != r2010_string_content_end_bit(payload, data_end_bit)? {
                return Err(format!("LAYOUT {handle:#x} stream boundary is invalid"));
            }
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "LAYOUT")?;
            object.body = Some(DwgLogicalObjectBody::Layout(DwgLayout {
                page_setup_name,
                printer_configuration,
                canonical_media_name,
                stylesheet,
                name,
                plot_options: DwgPlotOptions {
                    use_standard_scale: flags & 16 != 0,
                    plot_viewport_borders: flags & 32 != 0,
                    plot_with_lineweights: flags & 128 != 0,
                    draw_viewports_first: flags & 512 != 0,
                    model_type: flags & 1024 != 0,
                    update_paper: flags & 2048 != 0,
                    initializing: flags & 8192 != 0,
                },
                margins,
                paper_size,
                plot_origin,
                paper_unit,
                rotation,
                plot_area,
                plot_window_lower_left,
                plot_window_upper_right,
                paper_units,
                drawing_units,
                standard_scale,
                standard_scale_factor,
                paper_image_origin,
                shade_plot,
                shade_plot_resolution,
                shade_plot_dpi,
                tab_order,
                options: DwgLayoutOptions { paper_space_linetype_scaling: layout_flags & 1 != 0 },
                insertion_base,
                limits_minimum,
                limits_maximum,
                ucs_origin,
                ucs_x_axis,
                ucs_y_axis,
                ucs_elevation,
                orthographic_view,
                extents_minimum,
                extents_maximum,
                plot_view_handle,
                visual_style_handle,
                block_header_handle,
                active_viewport_handle,
                base_ucs_handle,
                named_ucs_handle,
                viewport_handles,
            }));
        } else if type_code == 527 || object.class_name == "BLOCKLINEARPARAMETER" {
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockLinearParameter;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockParameterBaseLocation;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockParameterConnection;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockParameterProperty;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockTwoPointParameter;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgPropertyExpressionReference;
            let strings = strings.as_mut().ok_or_else(|| format!("BLOCKLINEARPARAMETER {handle:#x} string stream missing"))?;
            let element = decode_r2010_block_element(&mut data, strings, "BLOCKLINEARPARAMETER")?;
            let show_properties = data.read_b()?;
            let chain_actions = data.read_b()?;
            let definition_base = data.read_3bd()?.to_vec();
            let definition_end = data.read_3bd()?.to_vec();
            let mut properties = Vec::with_capacity(4);
            for _ in 0..4 {
                let count = data.read_bl()? as usize;
                if count > 10_000 {
                    return Err(format!("BLOCKLINEARPARAMETER {handle:#x} connection count is invalid"));
                }
                let codes = (0..count).map(|_| data.read_bl()).collect::<Result<Vec<_>, _>>()?;
                let connections = codes.into_iter().map(|code| Ok(DwgBlockParameterConnection { code, name: strings.read_tu()? })).collect::<Result<Vec<_>, String>>()?;
                properties.push(DwgBlockParameterProperty { connections });
            }
            let property_expression_references =
                (0..4).map(|property_index| data.read_bl().map(|node_id| (node_id != 0).then_some(DwgPropertyExpressionReference { property_index, node_id }))).collect::<Result<Vec<_>, _>>()?.into_iter().flatten().collect();
            let base_location = match data.read_bs()? {
                0 => DwgBlockParameterBaseLocation::StartPoint,
                1 => DwgBlockParameterBaseLocation::Midpoint,
                value => return Err(format!("BLOCKLINEARPARAMETER {handle:#x} base location {value} is unsupported")),
            };
            let distance_name = strings.read_tu()?;
            let distance_description = strings.read_tu()?;
            let label_offset = data.read_bd()?;
            let value_flags = data.read_bl()?;
            let minimum = data.read_bd()?;
            let maximum = data.read_bd()?;
            let increment = data.read_bd()?;
            let value_count = data.read_bs()? as usize;
            let allowed_values = (0..value_count).map(|_| data.read_bd()).collect::<Result<Vec<_>, _>>()?;
            if value_flags != 8
                || (minimum, maximum, increment) != (0.0, 0.0, 0.0)
                || allowed_values.is_empty()
                || definition_base.iter().chain(&definition_end).chain(&allowed_values).any(|value| !value.is_finite())
                || !label_offset.is_finite()
                || data.bit_position() != main_end_bit
                || strings.bit_position() != r2010_string_content_end_bit(payload, data_end_bit)?
            {
                return Err(format!("BLOCKLINEARPARAMETER {handle:#x} logical value or stream boundary is invalid"));
            }
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "BLOCKLINEARPARAMETER")?;
            object.body = Some(DwgLogicalObjectBody::BlockLinearParameter(DwgBlockLinearParameter {
                parameter: DwgBlockTwoPointParameter { element, show_properties, chain_actions, definition_base, definition_end, properties, property_expression_references, base_location },
                distance_name,
                distance_description,
                label_offset,
                allowed_values,
            }));
        } else if matches!(type_code, 528 | 530 | 532) || matches!(object.class_name.as_str(), "BLOCKLINEARGRIP" | "BLOCKFLIPGRIP" | "BLOCKVISIBILITYGRIP") {
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockFlipGrip;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockLinearGrip;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockVisibilityGrip;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgNamedEvaluationNodeReference;
            let strings = strings.as_mut().ok_or_else(|| format!("{} {handle:#x} string stream missing", object.class_name))?;
            object.body = Some(match type_code {
                528 => {
                    let grip = decode_r2010_block_grip(&mut data, strings, "UpdatedEndX", "UpdatedEndY", "BLOCKLINEARGRIP")?;
                    let orientation = data.read_3bd()?.to_vec();
                    if orientation.iter().any(|value| !value.is_finite()) || orientation.iter().all(|value| *value == 0.0) {
                        return Err(format!("BLOCKLINEARGRIP {handle:#x} orientation is invalid"));
                    }
                    DwgLogicalObjectBody::BlockLinearGrip(DwgBlockLinearGrip { grip, orientation })
                }
                530 => {
                    let grip = decode_r2010_block_grip(&mut data, strings, "UpdatedBaseX", "UpdatedBaseY", "BLOCKFLIPGRIP")?;
                    let updated_flip = DwgNamedEvaluationNodeReference { node_id: data.read_bl()?, expression_name: "UpdatedFlip".into() };
                    let orientation = data.read_3bd()?.to_vec();
                    if orientation.iter().any(|value| !value.is_finite()) || orientation.iter().all(|value| *value == 0.0) {
                        return Err(format!("BLOCKFLIPGRIP {handle:#x} orientation is invalid"));
                    }
                    DwgLogicalObjectBody::BlockFlipGrip(DwgBlockFlipGrip { grip, updated_flip, orientation })
                }
                532 => {
                    let grip = decode_r2010_block_grip(&mut data, strings, "UpdatedX", "UpdatedY", "BLOCKVISIBILITYGRIP")?;
                    DwgLogicalObjectBody::BlockVisibilityGrip(DwgBlockVisibilityGrip { grip })
                }
                _ => return Err(format!("{} {handle:#x} type code is invalid", object.class_name)),
            });
            if data.bit_position() != main_end_bit || strings.bit_position() != r2010_string_content_end_bit(payload, data_end_bit)? {
                return Err(format!("{} {handle:#x} stream boundary is invalid", object.class_name));
            }
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, &object.class_name)?;
        } else if matches!(type_code, 533..=537) || matches!(object.class_name.as_str(), "BLOCKALIGNMENTPARAMETER" | "BLOCKALIGNMENTGRIP" | "BLOCKSTRETCHACTION" | "BLOCKSCALEACTION" | "BLOCKFLIPACTION") {
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockActionConnection;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockActionCoordinateMode;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockActionWithBasePoint;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockAlignmentGrip;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockAlignmentParameter;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockFlipAction;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockScaleAction;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockScaleMode;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockStretchAction;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgStretchSelection;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgStretchSelector;
            let strings = strings.as_mut().ok_or_else(|| format!("{} {handle:#x} string stream missing", object.class_name))?;
            object.body = Some(match type_code {
                533 => {
                    let (parameter, property_node_ids) = decode_r2010_two_point_parameter(&mut data, strings, "BLOCKALIGNMENTPARAMETER")?;
                    let align_perpendicular = data.read_b()?;
                    if property_node_ids[0] == 0 || property_node_ids[1..] != [0, 0, 0] {
                        return Err(format!("BLOCKALIGNMENTPARAMETER {handle:#x} property relation is invalid"));
                    }
                    DwgLogicalObjectBody::BlockAlignmentParameter(DwgBlockAlignmentParameter { parameter, updated_grip_node_id: property_node_ids[0], align_perpendicular })
                }
                534 => {
                    let grip = decode_r2010_block_grip(&mut data, strings, "FirstLocation", "SecondLocation", "BLOCKALIGNMENTGRIP")?;
                    let orientation = data.read_3bd()?.to_vec();
                    if orientation.iter().any(|value| !value.is_finite()) || orientation.iter().all(|value| *value == 0.0) {
                        return Err(format!("BLOCKALIGNMENTGRIP {handle:#x} orientation is invalid"));
                    }
                    let first_location_node_id = grip.updated_x.node_id;
                    let second_location_node_id = grip.updated_y.node_id;
                    DwgLogicalObjectBody::BlockAlignmentGrip(DwgBlockAlignmentGrip { grip, first_location_node_id, second_location_node_id, orientation })
                }
                535 => {
                    let action = decode_r2010_block_action(&mut data, strings, &mut handle_reader, handle, "BLOCKSTRETCHACTION")?;
                    let x_node_id = data.read_bl()?;
                    let x_name = strings.read_tu()?;
                    let y_node_id = data.read_bl()?;
                    let y_name = strings.read_tu()?;
                    let point_count = data.read_bl()? as usize;
                    if point_count == 0 || point_count > 10_000 {
                        return Err(format!("BLOCKSTRETCHACTION {handle:#x} point count is invalid"));
                    }
                    let points = (0..point_count).map(|_| data.read_2rd().map(|point| point.to_vec())).collect::<Result<Vec<_>, _>>()?;
                    let selection_count = data.read_bl()? as usize;
                    if selection_count > 10_000 {
                        return Err(format!("BLOCKSTRETCHACTION {handle:#x} selection count is invalid"));
                    }
                    let selection_indices = (0..selection_count)
                        .map(|_| {
                            let count = data.read_bs()? as usize;
                            (0..count).map(|_| data.read_bl()).collect::<Result<Vec<_>, _>>()
                        })
                        .collect::<Result<Vec<_>, String>>()?;
                    let selector_count = data.read_bl()? as usize;
                    if selector_count > 10_000 {
                        return Err(format!("BLOCKSTRETCHACTION {handle:#x} selector count is invalid"));
                    }
                    let selectors = (0..selector_count)
                        .map(|_| {
                            let node_id = data.read_bl()?;
                            let count = data.read_bs()? as usize;
                            let point_indices = (0..count).map(|_| data.read_bl()).collect::<Result<Vec<_>, _>>()?;
                            Ok(DwgStretchSelector { node_id, point_indices })
                        })
                        .collect::<Result<Vec<_>, String>>()?;
                    let distance_multiplier = data.read_bd()?;
                    let angle_offset = data.read_bd()?;
                    if data.read_rc()? != 0 {
                        return Err(format!("BLOCKSTRETCHACTION {handle:#x} coordinate mode is unsupported"));
                    }
                    let selections = selection_indices
                        .into_iter()
                        .map(|vertex_indices| read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("BLOCKSTRETCHACTION {handle:#x} selection is null")).map(|object_handle| DwgStretchSelection { object_handle, vertex_indices }))
                        .collect::<Result<Vec<_>, String>>()?;
                    DwgLogicalObjectBody::BlockStretchAction(DwgBlockStretchAction {
                        action,
                        x_connection: DwgBlockActionConnection { node_id: x_node_id, name: x_name },
                        y_connection: DwgBlockActionConnection { node_id: y_node_id, name: y_name },
                        points,
                        selections,
                        selectors,
                        distance_multiplier,
                        angle_offset,
                        coordinate_mode: DwgBlockActionCoordinateMode::CartesianXy,
                    })
                }
                536 => {
                    let action = decode_r2010_block_action(&mut data, strings, &mut handle_reader, handle, "BLOCKSCALEACTION")?;
                    let offset = data.read_3bd()?.to_vec();
                    let x_base_connection = DwgBlockActionConnection { node_id: data.read_bl()?, name: strings.read_tu()? };
                    let y_base_connection = DwgBlockActionConnection { node_id: data.read_bl()?, name: strings.read_tu()? };
                    let dependent = data.read_b()?;
                    let base_point = data.read_3bd()?.to_vec();
                    let uniform_scale_connection = DwgBlockActionConnection { node_id: data.read_bl()?, name: strings.read_tu()? };
                    let x_scale_connection = DwgBlockActionConnection { node_id: data.read_bl()?, name: strings.read_tu()? };
                    let y_scale_connection = DwgBlockActionConnection { node_id: data.read_bl()?, name: strings.read_tu()? };
                    if data.read_rc()? != 0 {
                        return Err(format!("BLOCKSCALEACTION {handle:#x} scale mode is unsupported"));
                    }
                    DwgLogicalObjectBody::BlockScaleAction(DwgBlockScaleAction {
                        base: DwgBlockActionWithBasePoint { action, offset, x_base_connection, y_base_connection, dependent, base_point },
                        uniform_scale_connection,
                        x_scale_connection,
                        y_scale_connection,
                        mode: DwgBlockScaleMode::Xy,
                    })
                }
                537 => {
                    let action = decode_r2010_block_action(&mut data, strings, &mut handle_reader, handle, "BLOCKFLIPACTION")?;
                    let mut read_connection = || -> Result<DwgBlockActionConnection, String> { Ok(DwgBlockActionConnection { node_id: data.read_bl()?, name: strings.read_tu()? }) };
                    DwgLogicalObjectBody::BlockFlipAction(DwgBlockFlipAction {
                        action,
                        flip_connection: read_connection()?,
                        updated_flip_connection: read_connection()?,
                        updated_base_connection: read_connection()?,
                        updated_end_connection: read_connection()?,
                    })
                }
                _ => return Err(format!("{} {handle:#x} type code is invalid", object.class_name)),
            });
            if data.bit_position() != main_end_bit || strings.bit_position() != r2010_string_content_end_bit(payload, data_end_bit)? {
                return Err(format!("{} {handle:#x} stream boundary is invalid", object.class_name));
            }
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, &object.class_name)?;
        } else if matches!(type_code, 538 | 546 | 548) || matches!(object.class_name.as_str(), "BLOCKBASEPOINTPARAMETER" | "BLOCKVERTICALCONSTRAINTPARAMETER" | "BLOCKHORIZONTALCONSTRAINTPARAMETER") {
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockBasePointParameter;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockLinearConstraintParameter;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockOnePointParameter;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockParameterAllowedValues;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockParameterConnection;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockParameterProperty;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
            let strings = strings.as_mut().ok_or_else(|| format!("{} {handle:#x} string stream missing", object.class_name))?;
            object.body = Some(if type_code == 538 {
                let element = decode_r2010_block_element(&mut data, strings, "BLOCKBASEPOINTPARAMETER")?;
                let show_properties = data.read_b()?;
                let chain_actions = data.read_b()?;
                let definition_point = data.read_3bd()?.to_vec();
                let mut properties = Vec::with_capacity(2);
                for _ in 0..2 {
                    let count = data.read_bl()? as usize;
                    let codes = (0..count).map(|_| data.read_bl()).collect::<Result<Vec<_>, _>>()?;
                    properties.push(DwgBlockParameterProperty { connections: codes.into_iter().map(|code| Ok(DwgBlockParameterConnection { code, name: strings.read_tu()? })).collect::<Result<Vec<_>, String>>()? });
                }
                if data.read_bl()? != 0 {
                    return Err(format!("BLOCKBASEPOINTPARAMETER {handle:#x} property-info count is invalid"));
                }
                let point = data.read_3bd()?.to_vec();
                let base_point = data.read_3bd()?.to_vec();
                DwgLogicalObjectBody::BlockBasePointParameter(DwgBlockBasePointParameter { parameter: DwgBlockOnePointParameter { element, show_properties, chain_actions, definition_point, properties }, point, base_point })
            } else {
                let (parameter, property_node_ids) = decode_r2010_two_point_parameter(&mut data, strings, &object.class_name)?;
                if property_node_ids[1] == 0 || property_node_ids[0] != 0 || property_node_ids[2] != 0 || property_node_ids[3] != 0 {
                    return Err(format!("{} {handle:#x} property state is invalid", object.class_name));
                }
                let dependency_handle = read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("{} {handle:#x} dependency is null", object.class_name))?;
                let expression_name = strings.read_tu()?;
                let expression_description = strings.read_tu()?;
                let value = data.read_bd()?;
                let flags = data.read_bl()?;
                let minimum = data.read_bd()?;
                let maximum = data.read_bd()?;
                let increment = data.read_bd()?;
                let count = data.read_bs()? as usize;
                let values = (0..count).map(|_| data.read_bd()).collect::<Result<Vec<_>, _>>()?;
                let delta = values.windows(2).next().map(|pair| pair[1] - pair[0]);
                let derived = delta.filter(|step| *step != 0.0 && values.windows(2).all(|pair| pair[1] - pair[0] == *step));
                let expected = derived.map_or((0.0, 0.0, 0.0), |step| (values[0], *values.last().unwrap(), step));
                if flags != 8 || values.is_empty() || (minimum, maximum, increment) != expected {
                    return Err(format!("{} {handle:#x} allowed-value helpers are invalid", object.class_name));
                }
                let body = DwgBlockLinearConstraintParameter { parameter, displacement_grip_node_id: property_node_ids[1], dependency_handle, expression_name, expression_description, value, allowed_values: DwgBlockParameterAllowedValues { values } };
                if type_code == 546 {
                    DwgLogicalObjectBody::BlockVerticalConstraintParameter(body)
                } else {
                    DwgLogicalObjectBody::BlockHorizontalConstraintParameter(body)
                }
            });
            if data.bit_position() != main_end_bit || strings.bit_position() != r2010_string_content_end_bit(payload, data_end_bit)? {
                return Err(format!("{} {handle:#x} stream boundary is invalid", object.class_name));
            }
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, &object.class_name)?;
        } else if type_code == 539 || object.class_name == "ACDBASSOCNETWORK" {
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAssocNetwork;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAssocNetworkMember;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAssocNetworkMemberKind;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAssociativeAction;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAssociativeActionDependency;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAssociativeActionStatus;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
            let action_version = data.read_bs()?;
            let action_status = data.read_bl()?;
            if action_version != 1 || action_status != 0 {
                return Err(format!("ACDBASSOCNETWORK {handle:#x} action version/status is unsupported"));
            }
            let action_index = data.read_bl()? as i32;
            let maximum_dependency_index = data.read_bl()? as i32;
            let dependency_count = data.read_bl()? as usize;
            let dependency_ownership = (0..dependency_count).map(|_| data.read_b()).collect::<Result<Vec<_>, _>>()?;
            let network_version = data.read_bs()?;
            let network_action_index = data.read_bl()? as i32;
            let action_count = data.read_bl()? as usize;
            let action_kinds = (0..action_count).map(|_| data.read_b()).collect::<Result<Vec<_>, _>>()?;
            let owned_action_count = data.read_bl()? as usize;
            if network_version != 0 || owned_action_count != 0 || data.bit_position() != main_end_bit {
                return Err(format!("ACDBASSOCNETWORK {handle:#x} native derived state or main boundary is invalid"));
            }
            let owning_network_handle = read_object_handle(&mut handle_reader, handle)?;
            let action_body_handle = read_object_handle(&mut handle_reader, handle)?;
            let dependencies = dependency_ownership
                .into_iter()
                .map(|owned| read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("ACDBASSOCNETWORK {handle:#x} dependency is null")).map(|dependency_handle| DwgAssociativeActionDependency { owned, dependency_handle }))
                .collect::<Result<Vec<_>, String>>()?;
            let actions = action_kinds
                .into_iter()
                .map(|owned| {
                    read_object_handle(&mut handle_reader, handle)?
                        .ok_or_else(|| format!("ACDBASSOCNETWORK {handle:#x} member is null"))
                        .map(|member_handle| DwgAssocNetworkMember { handle: member_handle, kind: if owned { DwgAssocNetworkMemberKind::Action } else { DwgAssocNetworkMemberKind::Network } })
                })
                .collect::<Result<Vec<_>, String>>()?;
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "ACDBASSOCNETWORK")?;
            object.body = Some(DwgLogicalObjectBody::AssocNetwork(DwgAssocNetwork {
                action: DwgAssociativeAction { status: DwgAssociativeActionStatus::UpToDate, owning_network_handle, action_body_handle, action_index, maximum_dependency_index, dependencies },
                network_action_index,
                actions,
            }));
        } else if type_code == 540 || object.class_name == "ACDBASSOC2DCONSTRAINTGROUP" {
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAssoc2dConstraintGroup;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAssociativeAction;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAssociativeActionDependency;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAssociativeActionStatus;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
            let strings = strings.as_mut().ok_or_else(|| format!("ACDBASSOC2DCONSTRAINTGROUP {handle:#x} class stream missing"))?;
            let action_version = data.read_bs()?;
            let action_status = data.read_bl()?;
            if action_version != 1 || action_status != 0 {
                return Err(format!("ACDBASSOC2DCONSTRAINTGROUP {handle:#x} action version/status is unsupported"));
            }
            let action_index = data.read_bl()? as i32;
            let maximum_dependency_index = data.read_bl()? as i32;
            let dependency_count = data.read_bl()? as usize;
            if dependency_count > 10_000 {
                return Err(format!("ACDBASSOC2DCONSTRAINTGROUP {handle:#x} dependency count is invalid"));
            }
            let dependency_ownership = (0..dependency_count).map(|_| data.read_b()).collect::<Result<Vec<_>, _>>()?;
            if data.read_bl()? != 0 {
                return Err(format!("ACDBASSOC2DCONSTRAINTGROUP {handle:#x} group version is unsupported"));
            }
            let do_not_check_newly_added_constraints = data.read_b()?;
            let work_plane = (0..3).map(|_| data.read_3bd().map(|value| value.to_vec())).collect::<Result<Vec<_>, _>>()?;
            let member_count = data.read_bl()? as usize;
            if member_count > 10_000 {
                return Err(format!("ACDBASSOC2DCONSTRAINTGROUP {handle:#x} member count is invalid"));
            }
            let node_id_watermark = data.read_bl()? as i32;
            let node_count = data.read_bl()? as usize;
            if node_count == 0 || node_count > 10_000 {
                return Err(format!("ACDBASSOC2DCONSTRAINTGROUP {handle:#x} node count {node_count} is invalid"));
            }
            let owning_network_handle = read_object_handle(&mut handle_reader, handle)?;
            let action_body_handle = read_object_handle(&mut handle_reader, handle)?;
            let dependencies = dependency_ownership
                .into_iter()
                .map(|owned| read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("ACDBASSOC2DCONSTRAINTGROUP {handle:#x} dependency is null")).map(|dependency_handle| DwgAssociativeActionDependency { owned, dependency_handle }))
                .collect::<Result<Vec<_>, String>>()?;
            if read_object_handle(&mut handle_reader, handle)?.is_some() {
                return Err(format!("ACDBASSOC2DCONSTRAINTGROUP {handle:#x} compatibility dimension slot is nonnull"));
            }
            let member_action_handles = (0..member_count).map(|_| read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("ACDBASSOC2DCONSTRAINTGROUP {handle:#x} member action is null"))).collect::<Result<Vec<_>, String>>()?;
            let mut nodes = Vec::with_capacity(node_count);
            for index in 0..node_count {
                let class = strings.read_tu()?;
                let bit = data.bit_position();
                nodes.push(decode_r2010_constraint_node(&class, &mut data, &mut handle_reader, handle).map_err(|error| format!("ACDBASSOC2DCONSTRAINTGROUP {handle:#x} node {index} {class} at bit {bit}: {error}"))?);
            }
            let actual_node_id_watermark = nodes.iter().map(constraint_node_id).max().and_then(|value| value.checked_add(1));
            let string_end_bit = r2010_string_content_end_bit(payload, data_end_bit)?;
            if actual_node_id_watermark != Some(node_id_watermark) || data.bit_position() != main_end_bit || strings.bit_position() != string_end_bit {
                return Err(format!("ACDBASSOC2DCONSTRAINTGROUP {handle:#x} watermark {actual_node_id_watermark:?}/{node_id_watermark}, main {}/{main_end_bit}, strings {}/{string_end_bit}", data.bit_position(), strings.bit_position()));
            }
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "ACDBASSOC2DCONSTRAINTGROUP")?;
            object.body = Some(DwgLogicalObjectBody::Assoc2dConstraintGroup(DwgAssoc2dConstraintGroup {
                action: DwgAssociativeAction { status: DwgAssociativeActionStatus::UpToDate, owning_network_handle, action_body_handle, action_index, maximum_dependency_index, dependencies },
                do_not_check_newly_added_constraints,
                work_plane,
                member_action_handles,
                nodes,
            }));
        } else if type_code == 505 || object.class_name == "MATERIAL" {
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgMaterial;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgMaterialChannels;
            let strings = strings.as_mut().ok_or_else(|| format!("MATERIAL {handle:#x} string stream missing"))?;
            let name = strings.read_tu()?;
            let description = strings.read_tu()?;
            let ambient = read_material_color(&mut data, handle)?;
            let diffuse = read_material_color(&mut data, handle)?;
            let diffuse_map = read_material_map(&mut data, strings, handle)?;
            let specular = read_material_color(&mut data, handle)?;
            let specular_map = read_material_map(&mut data, strings, handle)?;
            let specular_gloss = data.read_bd()?;
            let reflection_map = read_material_map(&mut data, strings, handle)?;
            let opacity = data.read_bd()?;
            let opacity_map = read_material_map(&mut data, strings, handle)?;
            let bump_map = read_material_map(&mut data, strings, handle)?;
            let refraction_index = data.read_bd()?;
            let refraction_map = read_material_map(&mut data, strings, handle)?;
            let translucence = data.read_bd()?;
            let self_illumination = data.read_bd()?;
            let reflectivity = data.read_bd()?;
            if data.read_bl()? != 0 {
                return Err(format!("MATERIAL {handle:#x} illumination model is unsupported"));
            }
            let channels = data.read_bl()?;
            if channels & !63 != 0 || data.read_bl()? != 0 {
                return Err(format!("MATERIAL {handle:#x} channel or mode state is unsupported"));
            }
            if data.bit_position() != main_end_bit || strings.bit_position() != r2010_string_content_end_bit(payload, data_end_bit)? {
                return Err(format!("MATERIAL {handle:#x} main or string stream is not exactly consumed"));
            }
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "MATERIAL")?;
            object.body = Some(DwgLogicalObjectBody::Material(DwgMaterial {
                name,
                description,
                ambient,
                diffuse,
                diffuse_map,
                specular,
                specular_map,
                specular_gloss,
                reflection_map,
                opacity,
                opacity_map,
                bump_map,
                refraction_index,
                refraction_map,
                translucence,
                self_illumination,
                reflectivity,
                enabled_channels: DwgMaterialChannels { diffuse: channels & 1 != 0, specular: channels & 2 != 0, reflection: channels & 4 != 0, opacity: channels & 8 != 0, bump: channels & 16 != 0, refraction: channels & 32 != 0 },
            }));
        } else if type_code == 521 || object.class_name == "BLOCKMOVEACTION" {
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockAction;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockActionConnection;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockActionDependency;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockMoveAction;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockMoveCoordinateMode;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEvaluationExpression;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEvaluationExpressionValue;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
            let parent_id = data.read_bl()? as i32;
            let major_version = data.read_bl()?;
            let minor_version = data.read_bl()?;
            let value_code = data.read_bs()? as i16;
            if value_code != -9999 {
                return Err(format!("BLOCKMOVEACTION {handle:#x} evaluation value {value_code} is unsupported"));
            }
            let node_id = data.read_bl()?;
            let repeated_major = data.read_bl()?;
            let repeated_minor = data.read_bl()?;
            let application_marker = data.read_bl()?;
            if (repeated_major, repeated_minor, application_marker) != (major_version, minor_version, 0) {
                return Err(format!("BLOCKMOVEACTION {handle:#x} repeated block-element metadata is inconsistent"));
            }
            let display_location = data.read_3bd()?.to_vec();
            let dependency_count = data.read_bl()? as usize;
            let action_node_count = data.read_bl()? as usize;
            let action_node_ids = (0..action_node_count).map(|_| data.read_bl()).collect::<Result<Vec<_>, _>>()?;
            let x_code = data.read_bl()?;
            let y_code = data.read_bl()?;
            let distance_multiplier = data.read_bd()?;
            let angle_offset = data.read_bd()?;
            let coordinate_mode = match data.read_rc()? {
                0 => DwgBlockMoveCoordinateMode::CartesianXy,
                value => return Err(format!("BLOCKMOVEACTION {handle:#x} coordinate mode {value} is unsupported")),
            };
            if dependency_count != 2 || action_node_count != 1 || data.bit_position() != main_end_bit {
                return Err(format!("BLOCKMOVEACTION {handle:#x} semantic counts or main boundary are invalid"));
            }
            let strings = strings.as_mut().ok_or_else(|| format!("BLOCKMOVEACTION {handle:#x} string stream missing"))?;
            let name = strings.read_tu()?;
            let x_name = strings.read_tu()?;
            let y_name = strings.read_tu()?;
            if strings.bit_position() != r2010_string_content_end_bit(payload, data_end_bit)? {
                return Err(format!("BLOCKMOVEACTION {handle:#x} string stream is not exactly consumed"));
            }
            let dependencies = (0..dependency_count)
                .map(|_| read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("BLOCKMOVEACTION {handle:#x} dependency is null")).map(|object_handle| DwgBlockActionDependency { object_handle }))
                .collect::<Result<Vec<_>, String>>()?;
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "BLOCKMOVEACTION")?;
            object.body = Some(DwgLogicalObjectBody::BlockMoveAction(DwgBlockMoveAction {
                action: DwgBlockAction { evaluation_expression: DwgEvaluationExpression { parent_id, major_version, minor_version, value: DwgEvaluationExpressionValue::Empty, node_id }, name, display_location, dependencies, action_node_ids },
                x_connection: DwgBlockActionConnection { node_id: x_code, name: x_name },
                y_connection: DwgBlockActionConnection { node_id: y_code, name: y_name },
                distance_multiplier,
                angle_offset,
                coordinate_mode,
            }));
        } else if type_code == 73 || object.class_name == "MLINESTYLE" {
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgMlineCaps;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgMlineLinetype;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgMlineStyle;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgMlineStyleElement;
            let flags = data.read_bs()?;
            if flags & !0x0773 != 0 {
                return Err(format!("MLINESTYLE {handle:#x} flags {flags:#x} contain unknown concepts"));
            }
            let fill_color = read_table_style_color(&mut data, handle)?;
            let start_angle = data.read_bd()?;
            let end_angle = data.read_bd()?;
            let count = data.read_rc()? as usize;
            if count == 0 {
                return Err(format!("MLINESTYLE {handle:#x} has no elements"));
            }
            let mut elements = Vec::with_capacity(count);
            for _ in 0..count {
                let offset = data.read_bd()?;
                let color = read_table_style_color(&mut data, handle)?;
                let linetype = match data.read_bs()? {
                    32767 => DwgMlineLinetype::ByLayer,
                    32766 => DwgMlineLinetype::ByBlock,
                    0 => DwgMlineLinetype::Continuous,
                    value => return Err(format!("MLINESTYLE {handle:#x} linetype index {value} is unsupported")),
                };
                elements.push(DwgMlineStyleElement { offset, color, linetype });
            }
            if elements.windows(2).any(|pair| pair[0].offset <= pair[1].offset) || data.bit_position() != main_end_bit {
                return Err(format!("MLINESTYLE {handle:#x} main stream or element ordering is invalid"));
            }
            let strings = strings.as_mut().ok_or_else(|| format!("MLINESTYLE {handle:#x} string stream missing"))?;
            let name = strings.read_tu()?;
            let description = strings.read_tu()?;
            if strings.bit_position() != r2010_string_content_end_bit(payload, data_end_bit)? {
                return Err(format!("MLINESTYLE {handle:#x} string stream is not exactly consumed"));
            }
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "MLINESTYLE")?;
            object.body = Some(DwgLogicalObjectBody::MlineStyle(DwgMlineStyle {
                name,
                description,
                fill_enabled: flags & 1 != 0,
                display_miters: flags & 2 != 0,
                start_caps: DwgMlineCaps { square: flags & 0x10 != 0, inner_arcs: flags & 0x20 != 0, round_outer_arcs: flags & 0x40 != 0 },
                end_caps: DwgMlineCaps { square: flags & 0x100 != 0, inner_arcs: flags & 0x200 != 0, round_outer_arcs: flags & 0x400 != 0 },
                fill_color,
                start_angle,
                end_angle,
                elements,
            }));
        } else if type_code == 508 || object.class_name == "MLEADERSTYLE" {
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::*;
            if data.read_bs()? != 2 {
                return Err(format!("MLEADERSTYLE {handle:#x} class version is unsupported"));
            }
            let content_type = match data.read_bs()? {
                0 => DwgMLeaderContentType::None,
                1 => DwgMLeaderContentType::Block,
                2 => DwgMLeaderContentType::MText,
                value => return Err(format!("MLEADERSTYLE {handle:#x} content type {value} is invalid")),
            };
            let draw_order = match data.read_bs()? {
                0 => DwgMLeaderDrawOrder::LeaderFirst,
                1 => DwgMLeaderDrawOrder::ContentFirst,
                value => return Err(format!("MLEADERSTYLE {handle:#x} draw order {value} is invalid")),
            };
            let leader_order = match data.read_bs()? {
                0 => DwgMLeaderLeaderOrder::HeadFirst,
                1 => DwgMLeaderLeaderOrder::TailFirst,
                value => return Err(format!("MLEADERSTYLE {handle:#x} leader order {value} is invalid")),
            };
            let maximum_segment_points = data.read_bl()?;
            let first_segment_angle = data.read_bd()?;
            let second_segment_angle = data.read_bd()?;
            let kind = match data.read_bs()? {
                0 => DwgMLeaderKind::Invisible,
                1 => DwgMLeaderKind::Straight,
                2 => DwgMLeaderKind::Spline,
                value => return Err(format!("MLEADERSTYLE {handle:#x} leader kind {value} is invalid")),
            };
            let leader_color = read_table_style_color(&mut data, handle)?;
            let lineweight = data.read_bl()? as i32;
            let landing = DwgMLeaderLanding { enabled: data.read_b()?, gap: data.read_bd()? };
            let dogleg = DwgMLeaderDogleg { enabled: data.read_b()?, length: data.read_bd()? };
            let arrow_size = data.read_bd()?;
            let attachment = |value: u16| -> Result<DwgMLeaderTextAttachment, String> {
                Ok(match value {
                    0 => DwgMLeaderTextAttachment::TopOfTop,
                    1 => DwgMLeaderTextAttachment::MiddleOfTop,
                    2 => DwgMLeaderTextAttachment::Middle,
                    3 => DwgMLeaderTextAttachment::MiddleOfBottom,
                    4 => DwgMLeaderTextAttachment::BottomOfBottom,
                    5 => DwgMLeaderTextAttachment::BottomLine,
                    6 => DwgMLeaderTextAttachment::BottomOfTop,
                    7 => DwgMLeaderTextAttachment::BottomOfTopUnderline,
                    8 => DwgMLeaderTextAttachment::BottomOfTopNoUnderline,
                    9 => DwgMLeaderTextAttachment::Center,
                    _ => return Err(format!("MLEADERSTYLE attachment {value} is invalid")),
                })
            };
            let left_attachment = attachment(data.read_bs()?)?;
            let right_attachment = attachment(data.read_bs()?)?;
            let angle = match data.read_bs()? {
                0 => DwgMLeaderTextAngle::Horizontal,
                1 => DwgMLeaderTextAngle::Aligned,
                2 => DwgMLeaderTextAngle::AlwaysRightReading,
                value => return Err(format!("MLEADERSTYLE {handle:#x} text angle {value} is invalid")),
            };
            let alignment = match data.read_bs()? {
                0 => DwgMLeaderTextAlignment::Left,
                1 => DwgMLeaderTextAlignment::Center,
                2 => DwgMLeaderTextAlignment::Right,
                value => return Err(format!("MLEADERSTYLE {handle:#x} text alignment {value} is invalid")),
            };
            let text_color = read_table_style_color(&mut data, handle)?;
            let text_height = data.read_bd()?;
            let frame = data.read_b()?;
            let always_left = data.read_b()?;
            let alignment_space = data.read_bd()?;
            let block_color = read_table_style_color(&mut data, handle)?;
            let block_scale = vec![data.read_bd()?, data.read_bd()?, data.read_bd()?];
            let use_scale = data.read_b()?;
            let rotation = data.read_bd()?;
            let use_rotation = data.read_b()?;
            let connection = match data.read_bs()? {
                0 => DwgMLeaderBlockConnection::Extents,
                1 => DwgMLeaderBlockConnection::BasePoint,
                value => return Err(format!("MLEADERSTYLE {handle:#x} block connection {value} is invalid")),
            };
            let overall_scale = data.read_bd()?;
            let property_overrides_changed = data.read_b()?;
            let annotative = data.read_b()?;
            let break_size = data.read_bd()?;
            let attachment_direction = match data.read_bs()? {
                0 => DwgMLeaderAttachmentDirection::Horizontal,
                1 => DwgMLeaderAttachmentDirection::Vertical,
                value => return Err(format!("MLEADERSTYLE {handle:#x} attachment direction {value} is invalid")),
            };
            let top_attachment = attachment(data.read_bs()?)?;
            let bottom_attachment = attachment(data.read_bs()?)?;
            if data.bit_position() != main_end_bit {
                return Err(format!("MLEADERSTYLE {handle:#x} main stream is not exactly consumed: {} != {main_end_bit}", data.bit_position()));
            }
            let strings = strings.as_mut().ok_or_else(|| format!("MLEADERSTYLE {handle:#x} string stream missing"))?;
            let description = strings.read_tu()?;
            let default_content = strings.read_tu()?;
            if strings.bit_position() != r2010_string_content_end_bit(payload, data_end_bit)? {
                return Err(format!("MLEADERSTYLE {handle:#x} string stream is not exactly consumed"));
            }
            let linetype_style_handle = read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("MLEADERSTYLE {handle:#x} linetype style is null"))?;
            let symbol_handle = read_object_handle(&mut handle_reader, handle)?;
            let style_handle = read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("MLEADERSTYLE {handle:#x} text style is null"))?;
            let content_handle = read_object_handle(&mut handle_reader, handle)?;
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "MLEADERSTYLE")?;
            object.body = Some(DwgLogicalObjectBody::MLeaderStyle(DwgMLeaderStyle {
                content_type,
                draw_order,
                leader_order,
                maximum_segment_points,
                first_segment_angle,
                second_segment_angle,
                leader: DwgMLeaderLeaderStyle { kind, color: leader_color, linetype_style_handle, lineweight },
                landing,
                dogleg,
                description,
                arrow: DwgMLeaderArrow { symbol_handle, size: arrow_size },
                text: DwgMLeaderTextStyle {
                    default_content,
                    style_handle,
                    left_attachment,
                    right_attachment,
                    angle,
                    alignment,
                    color: text_color,
                    height: text_height,
                    frame,
                    always_left,
                    alignment_space,
                    attachment_direction,
                    top_attachment,
                    bottom_attachment,
                },
                block: DwgMLeaderBlockStyle { content_handle, color: block_color, scale: block_scale, use_scale, rotation, use_rotation, connection },
                overall_scale,
                property_overrides_changed,
                annotative,
                break_size,
            }));
        } else if type_code == 504 || object.class_name == "TABLESTYLE" {
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgTableStyle;
            if data.read_rc()? != 0 {
                return Err(format!("TABLESTYLE {handle:#x} native discriminator is unsupported"));
            }
            let strings = strings.as_mut().ok_or_else(|| format!("TABLESTYLE {handle:#x} string stream missing"))?;
            let description = strings.read_tu()?;
            if data.read_bl()? != 0 {
                return Err(format!("TABLESTYLE {handle:#x} native format version is unsupported"));
            }
            let bit_flags = data.read_bl()?;
            let template_style_handle = read_object_handle(&mut handle_reader, handle)?;
            let table = decode_r2010_cell_style(&mut data, strings, &mut handle_reader, handle)?;
            if data.read_bl()? != 4 || data.read_bl()? != 2 || strings.read_tu()? != "Table" || data.read_bl()? != 3 {
                return Err(format!("TABLESTYLE {handle:#x} base identity or override count is invalid"));
            }
            let mut overrides = Vec::with_capacity(3);
            for (expected_id, expected_type, expected_name) in [(1, 1, "_TITLE"), (2, 1, "_HEADER"), (3, 2, "_DATA")] {
                if data.read_bl()? != expected_id {
                    return Err(format!("TABLESTYLE {handle:#x} override selector is invalid"));
                }
                let style = decode_r2010_cell_style(&mut data, strings, &mut handle_reader, handle)?;
                if data.read_bl()? != expected_id || data.read_bl()? != expected_type || strings.read_tu()? != expected_name {
                    return Err(format!("TABLESTYLE {handle:#x} override identity is invalid"));
                }
                overrides.push(style);
            }
            if data.bit_position() != main_end_bit || strings.bit_position() != r2010_string_content_end_bit(payload, data_end_bit)? {
                return Err(format!("TABLESTYLE {handle:#x} main or string stream is not exactly consumed"));
            }
            if object.owner_handle.is_none() || object.reactor_handles.len() != 1 || object.extension_dictionary_handle.is_none() || !object.extended_data.is_empty() {
                return Err(format!("TABLESTYLE {handle:#x} common state is invalid"));
            }
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "TABLESTYLE")?;
            let mut overrides = overrides.into_iter();
            object.body = Some(DwgLogicalObjectBody::TableStyle(Box::new(DwgTableStyle { description, bit_flags, template_style_handle, table, title: overrides.next().unwrap(), header: overrides.next().unwrap(), data: overrides.next().unwrap() })));
        } else if type_code == 516 || object.class_name == "SORTENTSTABLE" {
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDrawOrderEntry;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgSortEntitiesTable;
            let count = data.read_bl()? as usize;
            if count > 50_000 {
                return Err(format!("SORTENTSTABLE {handle:#x} entry count {count} is invalid"));
            }
            let sort_handles = (0..count)
                .map(|_| {
                    let (code, value) = data.read_handle()?;
                    if code != 0 {
                        return Err(format!("SORTENTSTABLE {handle:#x} sort key uses handle code {code}"));
                    }
                    Ok(value)
                })
                .collect::<Result<Vec<_>, String>>()?;
            if data.bit_position() != main_end_bit || main_end_bit + 1 != data_end_bit {
                return Err(format!("SORTENTSTABLE {handle:#x} main/string stream is not exactly consumed"));
            }
            if object.owner_handle.is_none() || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()] || object.extension_dictionary_handle.is_some() || !object.extended_data.is_empty() {
                return Err(format!("SORTENTSTABLE {handle:#x} common state is invalid"));
            }
            let block_header_handle = read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("SORTENTSTABLE {handle:#x} block header is null"))?;
            let entity_handles = (0..count).map(|_| read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("SORTENTSTABLE {handle:#x} entity reference is null"))).collect::<Result<Vec<_>, String>>()?;
            let entries = entity_handles.into_iter().zip(sort_handles).map(|(entity_handle, sort_handle)| DwgDrawOrderEntry { entity_handle, sort_handle }).collect::<Vec<_>>();
            let unique_entities = entries.iter().map(|entry| entry.entity_handle).collect::<std::collections::BTreeSet<_>>();
            let unique_sorts = entries.iter().map(|entry| entry.sort_handle).collect::<std::collections::BTreeSet<_>>();
            if unique_entities.len() != entries.len() || unique_sorts.len() != entries.len() {
                return Err(format!("SORTENTSTABLE {handle:#x} contains duplicate entity or sort keys"));
            }
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "SORTENTSTABLE")?;
            object.body = Some(DwgLogicalObjectBody::SortEntitiesTable(DwgSortEntitiesTable { block_header_handle, entries }));
        } else if type_code == 507 || object.class_name == "SCALE" {
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAnnotationScale;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
            if data.read_bs()? != 0 {
                return Err(format!("SCALE {handle:#x} native format flag is unsupported"));
            }
            let paper_units = data.read_bd()?;
            let drawing_units = data.read_bd()?;
            let is_unit_scale = data.read_b()?;
            if data.bit_position() != main_end_bit {
                return Err(format!("SCALE {handle:#x} main stream is not exactly consumed"));
            }
            if object.owner_handle.is_none() || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()] || object.extension_dictionary_handle.is_some() || !object.extended_data.is_empty() {
                return Err(format!("SCALE {handle:#x} common state is invalid"));
            }
            let strings = strings.as_mut().ok_or_else(|| format!("SCALE {handle:#x} string stream missing"))?;
            let name = strings.read_tu()?;
            if strings.bit_position() != r2010_string_content_end_bit(payload, data_end_bit)?
                || name.is_empty()
                || !paper_units.is_finite()
                || paper_units <= 0.0
                || !drawing_units.is_finite()
                || drawing_units <= 0.0
                || is_unit_scale && (name != "1:1" || paper_units != 1.0 || drawing_units != 1.0)
            {
                return Err(format!("SCALE {handle:#x} logical value or string boundary is invalid"));
            }
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "SCALE")?;
            object.body = Some(DwgLogicalObjectBody::AnnotationScale(DwgAnnotationScale { name, paper_units, drawing_units, is_unit_scale }));
        } else if type_code == 80 || object.class_name == "ACDBPLACEHOLDER" {
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgPlaceholder;
            if data.bit_position() != main_end_bit
                || main_end_bit + 1 != data_end_bit
                || object.owner_handle.is_none()
                || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()]
                || object.extension_dictionary_handle.is_some()
                || !object.extended_data.is_empty()
            {
                return Err(format!("ACDBPLACEHOLDER {handle:#x} logical stream or common state is invalid"));
            }
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "ACDBPLACEHOLDER")?;
            object.body = Some(DwgLogicalObjectBody::Placeholder(DwgPlaceholder {}));
        } else if type_code == 503 || object.class_name == "DICTIONARYVAR" {
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDictionaryVariable;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
            if data.read_rc()? != 0 || data.bit_position() != main_end_bit {
                return Err(format!("DICTIONARYVAR {handle:#x} schema revision or main boundary is invalid"));
            }
            if object.owner_handle.is_none() || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()] || object.extension_dictionary_handle.is_some() || !object.extended_data.is_empty() {
                return Err(format!("DICTIONARYVAR {handle:#x} common state is invalid"));
            }
            let strings = strings.as_mut().ok_or_else(|| format!("DICTIONARYVAR {handle:#x} string stream missing"))?;
            let value = strings.read_tu()?;
            if strings.bit_position() != r2010_string_content_end_bit(payload, data_end_bit)? {
                return Err(format!("DICTIONARYVAR {handle:#x} string stream is not exactly consumed"));
            }
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "DICTIONARYVAR")?;
            object.body = Some(DwgLogicalObjectBody::DictionaryVariable(DwgDictionaryVariable { value }));
        } else if type_code == 531 || object.class_name == "BLOCKVISIBILITYPARAMETER" {
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockParameterConnection;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockParameterProperty;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockVisibilityParameter;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEvaluationExpression;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEvaluationExpressionValue;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgVisibilityEvaluationHistory;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgVisibilityState;
            let parent_id = data.read_bl()? as i32;
            let major_version = data.read_bl()?;
            let minor_version = data.read_bl()?;
            let value_code = data.read_bs()? as i16;
            let data_value = match value_code {
                -9999 => Some(DwgEvaluationExpressionValue::Empty),
                40 => Some(DwgEvaluationExpressionValue::Double(data.read_bd()?)),
                10 => Some(DwgEvaluationExpressionValue::PointGroup10(data.read_2rd()?.to_vec())),
                11 => Some(DwgEvaluationExpressionValue::PointGroup11(data.read_2rd()?.to_vec())),
                1 | 91 => None,
                90 => Some(DwgEvaluationExpressionValue::Integer32(data.read_bl()? as i32)),
                70 => Some(DwgEvaluationExpressionValue::Integer16(data.read_bs()? as i16)),
                _ => return Err(format!("BLOCKVISIBILITYPARAMETER {handle:#x} evaluation discriminator {value_code} is unsupported")),
            };
            let node_id = data.read_bl()?;
            if data.read_bl()? != major_version || data.read_bl()? != minor_version || data.read_bl()? != 0 {
                return Err(format!("BLOCKVISIBILITYPARAMETER {handle:#x} block-element version is invalid"));
            }
            let show_properties = data.read_b()?;
            let chain_actions = data.read_b()?;
            let definition_point = data.read_3bd()?.to_vec();
            let mut connection_codes = Vec::with_capacity(2);
            for _ in 0..2 {
                let count = data.read_bl()? as usize;
                connection_codes.push((0..count).map(|_| data.read_bl()).collect::<Result<Vec<_>, _>>()?);
            }
            let updated_visibility_node_id = data.read_bl()?;
            let initialized = data.read_b()?;
            let evaluation_history = if data.read_b()? { DwgVisibilityEvaluationHistory::Required } else { DwgVisibilityEvaluationHistory::Stateless };
            let eligible_count = data.read_bl()? as usize;
            let state_count = data.read_bl()? as usize;
            let state_counts = (0..state_count).map(|_| Ok((data.read_bl()? as usize, data.read_bl()? as usize))).collect::<Result<Vec<_>, String>>()?;
            if data.bit_position() != main_end_bit {
                return Err(format!("BLOCKVISIBILITYPARAMETER {handle:#x} main stream is not exactly consumed"));
            }
            if object.owner_handle.is_none() || !object.reactor_handles.is_empty() || object.extension_dictionary_handle.is_some() {
                return Err(format!("BLOCKVISIBILITYPARAMETER {handle:#x} common state is invalid"));
            }
            let strings = strings.as_mut().ok_or_else(|| format!("BLOCKVISIBILITYPARAMETER {handle:#x} string stream missing"))?;
            let string_value = if value_code == 1 { Some(strings.read_tu()?) } else { None };
            let element_name = strings.read_tu()?;
            let mut properties = Vec::with_capacity(2);
            for codes in connection_codes {
                let connections = codes.into_iter().map(|code| Ok(DwgBlockParameterConnection { code, name: strings.read_tu()? })).collect::<Result<Vec<_>, String>>()?;
                properties.push(DwgBlockParameterProperty { connections });
            }
            let name = strings.read_tu()?;
            let description = strings.read_tu()?;
            let state_names = (0..state_count).map(|_| strings.read_tu()).collect::<Result<Vec<_>, _>>()?;
            let string_end_bit = r2010_string_content_end_bit(payload, data_end_bit)?;
            if strings.bit_position() != string_end_bit {
                return Err(format!("BLOCKVISIBILITYPARAMETER {handle:#x} string stream is not exactly consumed"));
            }
            let value = match value_code {
                1 => DwgEvaluationExpressionValue::String(string_value.unwrap()),
                91 => DwgEvaluationExpressionValue::ObjectReference(read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("BLOCKVISIBILITYPARAMETER {handle:#x} evaluation object reference is null"))?),
                _ => data_value.unwrap(),
            };
            let eligible_entity_handles = (0..eligible_count).map(|_| read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("BLOCKVISIBILITYPARAMETER {handle:#x} eligible entity is null"))).collect::<Result<Vec<_>, String>>()?;
            let mut states = Vec::with_capacity(state_count);
            for ((visible_count, controlled_count), name) in state_counts.into_iter().zip(state_names) {
                let visible_entity_handles = (0..visible_count).map(|_| read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("BLOCKVISIBILITYPARAMETER {handle:#x} visible entity is null"))).collect::<Result<Vec<_>, String>>()?;
                let controlled_expression_handles =
                    (0..controlled_count).map(|_| read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("BLOCKVISIBILITYPARAMETER {handle:#x} controlled expression is null"))).collect::<Result<Vec<_>, String>>()?;
                states.push(DwgVisibilityState { name, visible_entity_handles, controlled_expression_handles });
            }
            if states.iter().any(|state| state.visible_entity_handles.iter().any(|member| !eligible_entity_handles.contains(member))) {
                return Err(format!("BLOCKVISIBILITYPARAMETER {handle:#x} state contains an ineligible entity"));
            }
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "BLOCKVISIBILITYPARAMETER")?;
            object.body = Some(DwgLogicalObjectBody::BlockVisibilityParameter(DwgBlockVisibilityParameter {
                evaluation_expression: DwgEvaluationExpression { parent_id, major_version, minor_version, value, node_id },
                element_name,
                show_properties,
                chain_actions,
                definition_point,
                properties,
                updated_visibility_node_id,
                initialized,
                name,
                description,
                evaluation_history,
                eligible_entity_handles,
                states,
            }));
        } else if type_code == 529 || object.class_name == "BLOCKFLIPPARAMETER" {
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockFlipParameter;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockFlipValueSet;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockParameterBaseLocation;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockParameterConnection;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockParameterProperty;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEvaluationExpression;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEvaluationExpressionValue;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgNamedEvaluationNodeReference;
            let parent_id = data.read_bl()? as i32;
            let major_version = data.read_bl()?;
            let minor_version = data.read_bl()?;
            let value_code = data.read_bs()? as i16;
            let data_value = match value_code {
                -9999 => Some(DwgEvaluationExpressionValue::Empty),
                40 => Some(DwgEvaluationExpressionValue::Double(data.read_bd()?)),
                10 => Some(DwgEvaluationExpressionValue::PointGroup10(data.read_2rd()?.to_vec())),
                11 => Some(DwgEvaluationExpressionValue::PointGroup11(data.read_2rd()?.to_vec())),
                1 | 91 => None,
                90 => Some(DwgEvaluationExpressionValue::Integer32(data.read_bl()? as i32)),
                70 => Some(DwgEvaluationExpressionValue::Integer16(data.read_bs()? as i16)),
                _ => return Err(format!("BLOCKFLIPPARAMETER {handle:#x} evaluation discriminator {value_code} is unsupported")),
            };
            let node_id = data.read_bl()?;
            if data.read_bl()? != major_version || data.read_bl()? != minor_version || data.read_bl()? != 0 {
                return Err(format!("BLOCKFLIPPARAMETER {handle:#x} block-element version is invalid"));
            }
            let show_properties = data.read_b()?;
            let chain_actions = data.read_b()?;
            let definition_base = data.read_3bd()?.to_vec();
            let definition_end = data.read_3bd()?.to_vec();
            let mut connection_codes = Vec::with_capacity(4);
            for _ in 0..4 {
                let count = data.read_bl()? as usize;
                connection_codes.push((0..count).map(|_| data.read_bl()).collect::<Result<Vec<_>, _>>()?);
            }
            let property_states = [data.read_bl()?, data.read_bl()?, data.read_bl()?, data.read_bl()?];
            let base_location = match data.read_bs()? {
                0 => DwgBlockParameterBaseLocation::StartPoint,
                1 => DwgBlockParameterBaseLocation::Midpoint,
                value => return Err(format!("BLOCKFLIPPARAMETER {handle:#x} base location {value} is unsupported")),
            };
            let label_point = data.read_3bd()?.to_vec();
            let updated_node_id = data.read_bl()?;
            if property_states != [updated_node_id, 0, 0, 0] || data.bit_position() != main_end_bit {
                return Err(format!("BLOCKFLIPPARAMETER {handle:#x} duplicated update state or main boundary is invalid"));
            }
            if object.owner_handle.is_none() || !object.reactor_handles.is_empty() || object.extension_dictionary_handle.is_some() {
                return Err(format!("BLOCKFLIPPARAMETER {handle:#x} common state is invalid"));
            }
            let strings = strings.as_mut().ok_or_else(|| format!("BLOCKFLIPPARAMETER {handle:#x} string stream missing"))?;
            let string_value = if value_code == 1 { Some(strings.read_tu()?) } else { None };
            let name = strings.read_tu()?;
            let mut properties = Vec::with_capacity(4);
            for codes in connection_codes {
                let connections = codes.into_iter().map(|code| Ok(DwgBlockParameterConnection { code, name: strings.read_tu()? })).collect::<Result<Vec<_>, String>>()?;
                properties.push(DwgBlockParameterProperty { connections });
            }
            let label = strings.read_tu()?;
            let description = strings.read_tu()?;
            let base_label = strings.read_tu()?;
            let flipped_label = strings.read_tu()?;
            let expression_name = strings.read_tu()?;
            let string_end_bit = r2010_string_content_end_bit(payload, data_end_bit)?;
            if strings.bit_position() != string_end_bit {
                return Err(format!("BLOCKFLIPPARAMETER {handle:#x} string stream is not exactly consumed"));
            }
            let value = match value_code {
                1 => DwgEvaluationExpressionValue::String(string_value.unwrap()),
                91 => DwgEvaluationExpressionValue::ObjectReference(read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("BLOCKFLIPPARAMETER {handle:#x} evaluation object reference is null"))?),
                _ => data_value.unwrap(),
            };
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "BLOCKFLIPPARAMETER")?;
            object.body = Some(DwgLogicalObjectBody::BlockFlipParameter(DwgBlockFlipParameter {
                evaluation_expression: DwgEvaluationExpression { parent_id, major_version, minor_version, value, node_id },
                name,
                show_properties,
                chain_actions,
                definition_base,
                definition_end,
                properties,
                base_location,
                label,
                description,
                value_set: DwgBlockFlipValueSet { base_label, flipped_label },
                label_point,
                updated_flip: DwgNamedEvaluationNodeReference { node_id: updated_node_id, expression_name },
            }));
        } else if type_code == 517 || object.class_name == "ACAD_EVALUATION_GRAPH" {
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEvaluationGraph;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEvaluationGraphEdge;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEvaluationGraphNode;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
            let watermark = data.read_bl()?;
            let watermark_copy = data.read_bl()?;
            if watermark != watermark_copy {
                return Err(format!("ACAD_EVALUATION_GRAPH {handle:#x} watermark copies disagree"));
            }
            let node_count = data.read_bl()? as usize;
            let mut nodes = Vec::with_capacity(node_count);
            let mut native_node_relations = Vec::with_capacity(node_count);
            for index in 0..node_count {
                if data.read_bl()? as usize != index || data.read_bl()? != 32 {
                    return Err(format!("ACAD_EVALUATION_GRAPH {handle:#x} node {index} native identity is invalid"));
                }
                let id = data.read_bl()?;
                let mut relations = [0i32; 4];
                for relation in &mut relations {
                    *relation = data.read_bl()? as i32;
                }
                nodes.push(DwgEvaluationGraphNode { id, expression_handle: 0 });
                native_node_relations.push(relations);
            }
            let edge_count = data.read_bl()? as usize;
            let mut edges = Vec::with_capacity(edge_count);
            let mut native_edge_relations = Vec::with_capacity(edge_count);
            let mut native_endpoints = Vec::with_capacity(edge_count);
            for index in 0..edge_count {
                if data.read_bl()? as usize != index || data.read_bl()? != 0 {
                    return Err(format!("ACAD_EVALUATION_GRAPH {handle:#x} edge {index} native identity is invalid"));
                }
                let reference_count = data.read_bl()?;
                let from = data.read_bl()? as usize;
                let to = data.read_bl()? as usize;
                let from_node_id = nodes.get(from).ok_or_else(|| format!("ACAD_EVALUATION_GRAPH {handle:#x} edge {index} source is invalid"))?.id;
                let to_node_id = nodes.get(to).ok_or_else(|| format!("ACAD_EVALUATION_GRAPH {handle:#x} edge {index} target is invalid"))?.id;
                let mut relations = [0i32; 5];
                for relation in &mut relations {
                    *relation = data.read_bl()? as i32;
                }
                edges.push(DwgEvaluationGraphEdge { from_node_id, to_node_id, reference_count, invertible: false, suppressed: false });
                native_edge_relations.push(relations);
                native_endpoints.push((from, to));
            }
            if data.bit_position() != main_end_bit || nodes.iter().map(|node| node.id).max() != Some(watermark) {
                return Err(format!("ACAD_EVALUATION_GRAPH {handle:#x} main stream or watermark is invalid"));
            }
            if !object.extended_data.is_empty() || object.extension_dictionary_handle.is_some() || object.owner_handle.is_none() || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()] {
                return Err(format!("ACAD_EVALUATION_GRAPH {handle:#x} common state is invalid"));
            }
            for node in &mut nodes {
                node.expression_handle = read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("ACAD_EVALUATION_GRAPH {handle:#x} expression handle is null"))?;
            }
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "ACAD_EVALUATION_GRAPH")?;
            let graph = DwgEvaluationGraph { nodes, edges };
            let (derived_node_relations, derived_edge_relations, derived_endpoints) = evaluation_graph_indexes(&graph)?;
            if native_node_relations != derived_node_relations || native_edge_relations != derived_edge_relations || native_endpoints != derived_endpoints {
                return Err(format!("ACAD_EVALUATION_GRAPH {handle:#x} native indexes do not match the semantic graph"));
            }
            object.body = Some(DwgLogicalObjectBody::EvaluationGraph(graph));
        } else if type_code == 522 || object.class_name == "ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION" {
            let marker = data.read_bs()?;
            if marker != 1 {
                return Err(format!("ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION {handle:#x} marker {marker} is unsupported"));
            }
            if data.bit_position() != main_end_bit {
                return Err(format!("ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION {handle:#x} main stream is not exactly consumed: {} != {main_end_bit}", data.bit_position()));
            }
            if !object.extended_data.is_empty() || object.extension_dictionary_handle.is_some() || object.owner_handle.is_none() || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()] {
                return Err(format!("ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION {handle:#x} common state is invalid"));
            }
            let protected_block_header_handle = read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION {handle:#x} protected block header is null"))?;
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION")?;
            object.body = Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::DynamicBlockPurgePreventer(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDynamicBlockPurgePreventer { protected_block_header_handle }));
        } else if type_code == 559 || object.class_name == "ACDB_BLOCKREPRESENTATION_DATA" {
            let marker = data.read_bs()?;
            if marker != 1 {
                return Err(format!("ACDB_BLOCKREPRESENTATION_DATA {handle:#x} marker {marker} is unsupported"));
            }
            if data.bit_position() != main_end_bit {
                return Err(format!("ACDB_BLOCKREPRESENTATION_DATA {handle:#x} main stream is not exactly consumed: {} != {main_end_bit}", data.bit_position()));
            }
            if !object.extended_data.is_empty() || object.extension_dictionary_handle.is_some() || object.owner_handle != Some(handle - 1) || object.reactor_handles.as_slice() != [handle - 1] {
                return Err(format!("ACDB_BLOCKREPRESENTATION_DATA {handle:#x} common state is invalid"));
            }
            let represented_block_header_handle = read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("ACDB_BLOCKREPRESENTATION_DATA {handle:#x} represented block header is null"))?;
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "ACDB_BLOCKREPRESENTATION_DATA")?;
            object.body = Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::BlockRepresentationData(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockRepresentationData { represented_block_header_handle }));
        } else if type_code == 506 || object.class_name == "VISUALSTYLE" {
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgVisualStyle;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgVisualStyleProperties;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgVisualStyleProperty;
            let style_type = data.read_bl()?;
            let extension_lighting_model = data.read_bs()?;
            let internal_only = data.read_b()?;
            macro_rules! blp {
                () => {{
                    let value = data.read_bl()?;
                    let operation = read_visual_style_operation(&mut data)?;
                    DwgVisualStyleProperty { value, operation }
                }};
            }
            macro_rules! bsp {
                () => {{
                    let value = data.read_bs()?;
                    let operation = read_visual_style_operation(&mut data)?;
                    DwgVisualStyleProperty { value, operation }
                }};
            }
            macro_rules! bdp {
                () => {{
                    let value = data.read_bd()?;
                    let operation = read_visual_style_operation(&mut data)?;
                    DwgVisualStyleProperty { value, operation }
                }};
            }
            let face_lighting_model = blp!();
            let face_lighting_quality = blp!();
            let face_color_mode = blp!();
            let face_modifiers = bsp!();
            let face_opacity = bdp!();
            let face_specular_amount = bdp!();
            let (face_monochrome_color, face_monochrome_flags) = read_visual_style_color(&mut data)?;
            let edge_model = blp!();
            let edge_styles = blp!();
            let (edge_intersection_color, edge_intersection_flags) = read_visual_style_color(&mut data)?;
            let (edge_obscured_color, edge_obscured_flags) = read_visual_style_color(&mut data)?;
            let edge_obscured_line_pattern = blp!();
            let edge_intersection_line_pattern = blp!();
            let edge_crease_angle = bdp!();
            let edge_modifiers = blp!();
            let (edge_color, edge_color_flags) = read_visual_style_color(&mut data)?;
            let edge_opacity = bdp!();
            let edge_width = blp!();
            let edge_overhang = blp!();
            let edge_jitter = blp!();
            let (edge_silhouette_color, edge_silhouette_flags) = read_visual_style_color(&mut data)?;
            let edge_silhouette_width = blp!();
            let edge_halo_gap = blp!();
            let edge_isolines = blp!();
            let hidden_edge_precision = DwgVisualStyleProperty { value: data.read_b()?, operation: read_visual_style_operation(&mut data)? };
            let display_settings = blp!();
            let display_brightness = bdp!();
            let display_shadow_type = blp!();
            if data.bit_position() != main_end_bit {
                return Err(format!("VISUALSTYLE {handle:#x} main stream is not exactly consumed: {} != {main_end_bit}", data.bit_position()));
            }
            let strings = strings.as_mut().ok_or_else(|| format!("VISUALSTYLE {handle:#x} string stream missing"))?;
            let description = strings.read_tu()?;
            let mut properties = DwgVisualStyleProperties {
                face_lighting_model,
                face_lighting_quality,
                face_color_mode,
                face_modifiers,
                face_opacity,
                face_specular_amount,
                face_monochrome_color,
                edge_model,
                edge_styles,
                edge_intersection_color,
                edge_obscured_color,
                edge_obscured_line_pattern,
                edge_intersection_line_pattern,
                edge_crease_angle,
                edge_modifiers,
                edge_color,
                edge_opacity,
                edge_width,
                edge_overhang,
                edge_jitter,
                edge_silhouette_color,
                edge_silhouette_width,
                edge_halo_gap,
                edge_isolines,
                hidden_edge_precision,
                display_settings,
                display_brightness,
                display_shadow_type,
            };
            let mut read_color_strings = |color: &mut semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgComplexColor, flags: u8| -> Result<(), String> {
                if flags & 1 != 0 {
                    color.name = Some(strings.read_tu()?);
                }
                if flags & 2 != 0 {
                    color.book_name = Some(strings.read_tu()?);
                }
                Ok(())
            };
            read_color_strings(&mut properties.face_monochrome_color.value, face_monochrome_flags)?;
            read_color_strings(&mut properties.edge_intersection_color.value, edge_intersection_flags)?;
            read_color_strings(&mut properties.edge_obscured_color.value, edge_obscured_flags)?;
            read_color_strings(&mut properties.edge_color.value, edge_color_flags)?;
            read_color_strings(&mut properties.edge_silhouette_color.value, edge_silhouette_flags)?;
            let string_end_bit = r2010_string_content_end_bit(payload, data_end_bit)?;
            if strings.bit_position() != string_end_bit {
                return Err(format!("VISUALSTYLE {handle:#x} string stream is not exactly consumed: {} != {string_end_bit}", strings.bit_position()));
            }
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "VISUALSTYLE")?;
            object.body = Some(DwgLogicalObjectBody::VisualStyle(DwgVisualStyle { description, style_type, extension_lighting_model, internal_only, properties }));
        } else if type_code == 543 || object.class_name == "BLOCKPARAMDEPENDENCYBODY" {
            let dependency_version = data.read_bs()?;
            let dimension_base_version = data.read_bs()?;
            let class_version = data.read_bs()?;
            if (dependency_version, dimension_base_version, class_version) != (1, 1, 0) {
                return Err(format!("BLOCKPARAMDEPENDENCYBODY {handle:#x} versions {dependency_version}/{dimension_base_version}/{class_version} are unsupported"));
            }
            if data.bit_position() != main_end_bit {
                return Err(format!("BLOCKPARAMDEPENDENCYBODY {handle:#x} main stream is not exactly consumed: {} != {main_end_bit}", data.bit_position()));
            }
            let strings = strings.as_mut().ok_or_else(|| format!("BLOCKPARAMDEPENDENCYBODY {handle:#x} string stream missing"))?;
            let name = strings.read_tu()?;
            let string_end_bit = r2010_string_content_end_bit(payload, data_end_bit)?;
            if strings.bit_position() != string_end_bit {
                return Err(format!("BLOCKPARAMDEPENDENCYBODY {handle:#x} string stream is not exactly consumed"));
            }
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "BLOCKPARAMDEPENDENCYBODY")?;
            object.body = Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::BlockParameterDependencyBody(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockParameterDependencyBody { name }));
        } else if type_code == 549 || object.class_name == "ASSOCDIMDEPENDENCYBODY" {
            let dependency_version = data.read_bs()?;
            let dimension_base_version = data.read_bs()?;
            let class_version = data.read_bs()?;
            if (dependency_version, dimension_base_version, class_version) != (1, 1, 1) {
                return Err(format!("ASSOCDIMDEPENDENCYBODY {handle:#x} versions {dependency_version}/{dimension_base_version}/{class_version} are unsupported"));
            }
            if data.bit_position() != main_end_bit {
                return Err(format!("ASSOCDIMDEPENDENCYBODY {handle:#x} main stream is not exactly consumed: {} != {main_end_bit}", data.bit_position()));
            }
            let strings = strings.as_mut().ok_or_else(|| format!("ASSOCDIMDEPENDENCYBODY {handle:#x} string stream missing"))?;
            let name = strings.read_tu()?;
            let string_end_bit = r2010_string_content_end_bit(payload, data_end_bit)?;
            if strings.bit_position() != string_end_bit {
                return Err(format!("ASSOCDIMDEPENDENCYBODY {handle:#x} string stream is not exactly consumed"));
            }
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "ASSOCDIMDEPENDENCYBODY")?;
            object.body = Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::AssociativeDimensionDependencyBody(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAssociativeDimensionDependencyBody { name }));
        } else if type_code == 545 || object.class_name == "ACDBASSOCVARIABLE" {
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAssociativeAction;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAssociativeActionDependency;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAssociativeActionStatus;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAssociativeVariable;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEvaluationVariant;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
            let action_version = data.read_bs()?;
            let action_status = data.read_bl()?;
            if action_version != 1 || action_status != 0 {
                return Err(format!("ACDBASSOCVARIABLE {handle:#x} action version/status {action_version}/{action_status} is unsupported"));
            }
            let action_index = data.read_bl()? as i32;
            let maximum_dependency_index = data.read_bl()? as i32;
            let dependency_count = data.read_bl()? as usize;
            let dependency_ownership = (0..dependency_count).map(|_| data.read_b()).collect::<Result<Vec<_>, _>>()?;
            let variable_version = data.read_bl()?;
            let value_code = data.read_bs()?;
            if variable_version != 2 || value_code != 90 {
                return Err(format!("ACDBASSOCVARIABLE {handle:#x} variable version/value code {variable_version}/{value_code} is unsupported"));
            }
            let evaluated_value = DwgEvaluationVariant::Integer32(data.read_bl()? as i32);
            let mergeable = data.read_b()?;
            let must_merge = data.read_b()?;
            let binding_count = if maximum_dependency_index > 0 { data.read_bl()? as usize } else { 0 };
            let binding_version = data.read_bs()?;
            if maximum_dependency_index < 0 || maximum_dependency_index as usize != binding_count || binding_version != 0 {
                return Err(format!("ACDBASSOCVARIABLE {handle:#x} binding index/count/version is inconsistent"));
            }
            if data.bit_position() != main_end_bit {
                return Err(format!("ACDBASSOCVARIABLE {handle:#x} main stream is not exactly consumed: {} != {main_end_bit}", data.bit_position()));
            }
            let strings = strings.as_mut().ok_or_else(|| format!("ACDBASSOCVARIABLE {handle:#x} string stream missing"))?;
            let name = strings.read_tu()?;
            let expression = strings.read_tu()?;
            let evaluator_id = strings.read_tu()?;
            let description = strings.read_tu()?;
            let mergeable_variable_name = if mergeable { Some(strings.read_tu()?) } else { None };
            let string_end_bit = r2010_string_content_end_bit(payload, data_end_bit)?;
            if strings.bit_position() != string_end_bit {
                return Err(format!("ACDBASSOCVARIABLE {handle:#x} string stream is not exactly consumed: {} != {string_end_bit}", strings.bit_position()));
            }
            let owning_network_handle = Some(read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("ACDBASSOCVARIABLE {handle:#x} owning network is null"))?);
            let action_body_handle = read_object_handle(&mut handle_reader, handle)?;
            let dependencies = dependency_ownership
                .into_iter()
                .map(|owned| read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("ACDBASSOCVARIABLE {handle:#x} action dependency is null")).map(|dependency_handle| DwgAssociativeActionDependency { owned, dependency_handle }))
                .collect::<Result<Vec<_>, String>>()?;
            let referenced_value_dependency_handles = (0..binding_count).map(|_| read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("ACDBASSOCVARIABLE {handle:#x} value dependency is null"))).collect::<Result<Vec<_>, String>>()?;
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "ACDBASSOCVARIABLE")?;
            object.body = Some(DwgLogicalObjectBody::AssociativeVariable(DwgAssociativeVariable {
                action: DwgAssociativeAction { status: DwgAssociativeActionStatus::UpToDate, owning_network_handle, action_body_handle, action_index, maximum_dependency_index, dependencies },
                name,
                expression,
                evaluator_id,
                description,
                evaluated_value,
                mergeable,
                mergeable_variable_name,
                must_merge,
                referenced_value_dependency_handles,
            }));
        } else if type_code == 547 || object.class_name == "ACDB_DYNAMICBLOCKPROXYNODE" {
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDynamicBlockProxyNode;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEvaluationExpression;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEvaluationExpressionValue;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
            let parent_id = data.read_bl()? as i32;
            let major_version = data.read_bl()?;
            let minor_version = data.read_bl()?;
            let value_code = data.read_bs()? as i16;
            let data_value = match value_code {
                -9999 => Some(DwgEvaluationExpressionValue::Empty),
                40 => Some(DwgEvaluationExpressionValue::Double(data.read_bd()?)),
                10 => Some(DwgEvaluationExpressionValue::PointGroup10(data.read_2rd()?.to_vec())),
                11 => Some(DwgEvaluationExpressionValue::PointGroup11(data.read_2rd()?.to_vec())),
                1 | 91 => None,
                90 => Some(DwgEvaluationExpressionValue::Integer32(data.read_bl()? as i32)),
                70 => Some(DwgEvaluationExpressionValue::Integer16(data.read_bs()? as i16)),
                _ => return Err(format!("ACDB_DYNAMICBLOCKPROXYNODE {handle:#x} value discriminator {value_code} is unsupported")),
            };
            let node_id = data.read_bl()?;
            if data.bit_position() != main_end_bit {
                return Err(format!("ACDB_DYNAMICBLOCKPROXYNODE {handle:#x} main stream is not exactly consumed: {} != {main_end_bit}", data.bit_position()));
            }
            let value = match value_code {
                1 => {
                    let strings = strings.as_mut().ok_or_else(|| format!("ACDB_DYNAMICBLOCKPROXYNODE {handle:#x} string stream missing"))?;
                    let value = DwgEvaluationExpressionValue::String(strings.read_tu()?);
                    let string_end_bit = r2010_string_content_end_bit(payload, data_end_bit)?;
                    if strings.bit_position() != string_end_bit {
                        return Err(format!("ACDB_DYNAMICBLOCKPROXYNODE {handle:#x} string stream is not exactly consumed"));
                    }
                    value
                }
                91 => DwgEvaluationExpressionValue::ObjectReference(read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("ACDB_DYNAMICBLOCKPROXYNODE {handle:#x} object reference is null"))?),
                _ => data_value.unwrap(),
            };
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "ACDB_DYNAMICBLOCKPROXYNODE")?;
            object.body = Some(DwgLogicalObjectBody::DynamicBlockProxyNode(DwgDynamicBlockProxyNode { evaluation_expression: DwgEvaluationExpression { parent_id, major_version, minor_version, value, node_id } }));
        } else if type_code == 520 || object.class_name == "ACDB_BLOCKGRIPLOCATIONCOMPONENT" {
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockGripLocationComponent;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEvaluationExpression;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEvaluationExpressionValue;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
            let parent_id = data.read_bl()? as i32;
            let major_version = data.read_bl()?;
            let minor_version = data.read_bl()?;
            let value_code = data.read_bs()? as i16;
            let data_value = match value_code {
                -9999 => Some(DwgEvaluationExpressionValue::Empty),
                40 => Some(DwgEvaluationExpressionValue::Double(data.read_bd()?)),
                10 => Some(DwgEvaluationExpressionValue::PointGroup10(data.read_2rd()?.to_vec())),
                11 => Some(DwgEvaluationExpressionValue::PointGroup11(data.read_2rd()?.to_vec())),
                1 | 91 => None,
                90 => Some(DwgEvaluationExpressionValue::Integer32(data.read_bl()? as i32)),
                70 => Some(DwgEvaluationExpressionValue::Integer16(data.read_bs()? as i16)),
                _ => return Err(format!("BLOCKGRIPLOCATIONCOMPONENT {handle:#x} value discriminator {value_code} is unsupported")),
            };
            let node_id = data.read_bl()?;
            let grip_type = data.read_bl()?;
            if data.bit_position() != main_end_bit {
                return Err(format!("BLOCKGRIPLOCATIONCOMPONENT {handle:#x} main stream is not exactly consumed: {} != {main_end_bit}", data.bit_position()));
            }
            let strings = strings.as_mut().ok_or_else(|| format!("BLOCKGRIPLOCATIONCOMPONENT {handle:#x} string stream missing"))?;
            let string_value = if value_code == 1 { Some(strings.read_tu()?) } else { None };
            let grip_expression = strings.read_tu()?;
            let string_end_bit = r2010_string_content_end_bit(payload, data_end_bit)?;
            if strings.bit_position() != string_end_bit {
                return Err(format!("BLOCKGRIPLOCATIONCOMPONENT {handle:#x} string stream is not exactly consumed: {} != {string_end_bit}", strings.bit_position()));
            }
            let value = match value_code {
                1 => DwgEvaluationExpressionValue::String(string_value.unwrap()),
                91 => DwgEvaluationExpressionValue::ObjectReference(read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("BLOCKGRIPLOCATIONCOMPONENT {handle:#x} object reference is null"))?),
                _ => data_value.unwrap(),
            };
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "BLOCKGRIPLOCATIONCOMPONENT")?;
            object.body =
                Some(DwgLogicalObjectBody::BlockGripLocationComponent(DwgBlockGripLocationComponent { evaluation_expression: DwgEvaluationExpression { parent_id, major_version, minor_version, value, node_id }, grip_type, grip_expression }));
        } else if type_code == 544 || object.class_name == "ACDBASSOCGEOMDEPENDENCY" {
            let class_version = data.read_bs()?;
            let status = data.read_bl()?;
            if class_version != 1 || status != 0 {
                return Err(format!("ACDBASSOCGEOMDEPENDENCY {handle:#x} has unsupported dependency version/status {class_version}/{status}"));
            }
            let is_read_dependency = data.read_b()?;
            let is_write_dependency = data.read_b()?;
            let is_attached_to_object = data.read_b()?;
            let is_delegating_to_owning_action = data.read_b()?;
            let order = data.read_bl()? as i32;
            let has_name = data.read_b()?;
            let dependency_body_id = data.read_bl()? as i32;
            let geometry_version = data.read_bs()?;
            let enabled = data.read_b()?;
            let dependent_on_compound_object = data.read_b()?;
            if geometry_version != 0 {
                return Err(format!("ACDBASSOCGEOMDEPENDENCY {handle:#x} geometry version {geometry_version} is unsupported"));
            }
            if data.bit_position() != main_end_bit {
                return Err(format!("ACDBASSOCGEOMDEPENDENCY {handle:#x} main stream is not exactly consumed: {} != {main_end_bit}", data.bit_position()));
            }
            let strings = strings.as_mut().ok_or_else(|| format!("ACDBASSOCGEOMDEPENDENCY {handle:#x} string stream missing"))?;
            let name = if has_name { Some(strings.read_tu()?) } else { None };
            let persistent_subentity_class_name = strings.read_tu()?;
            let string_end_bit = r2010_string_content_end_bit(payload, data_end_bit)?;
            if strings.bit_position() != string_end_bit {
                return Err(format!("ACDBASSOCGEOMDEPENDENCY {handle:#x} string stream is not exactly consumed: {} != {string_end_bit}", strings.bit_position()));
            }
            let dependent_on_object_handle = read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("ACDBASSOCGEOMDEPENDENCY {handle:#x} dependent-on object is null"))?;
            let read_dependency_handle = read_object_handle(&mut handle_reader, handle)?;
            let dependency_node_handle = read_object_handle(&mut handle_reader, handle)?;
            let dependency_body_handle = read_object_handle(&mut handle_reader, handle)?;
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "ACDBASSOCGEOMDEPENDENCY")?;
            object.body = Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::AssociativeGeometryDependency(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAssociativeGeometryDependency {
                dependency: semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAssociativeDependency {
                    status: semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAssociativeDependencyStatus::UpToDate,
                    is_read_dependency,
                    is_write_dependency,
                    is_attached_to_object,
                    is_delegating_to_owning_action,
                    order,
                    dependent_on_object_handle,
                    name,
                    read_dependency_handle,
                    dependency_node_handle,
                    dependency_body_handle,
                    dependency_body_id,
                },
                enabled,
                persistent_subentity_class_name,
                dependent_on_compound_object,
            }));
        } else if type_code == 541 || object.class_name == "ACDBASSOCVALUEDEPENDENCY" {
            let class_version = data.read_bs()?;
            let status = data.read_bl()?;
            if class_version != 1 || status != 0 {
                return Err(format!("ACDBASSOCVALUEDEPENDENCY {handle:#x} has unsupported dependency version/status {class_version}/{status}"));
            }
            let is_read_dependency = data.read_b()?;
            let is_write_dependency = data.read_b()?;
            let is_attached_to_object = data.read_b()?;
            let is_delegating_to_owning_action = data.read_b()?;
            let order = data.read_bl()? as i32;
            let has_name = data.read_b()?;
            let dependency_body_id = data.read_bl()? as i32;
            let value_dependency_version = data.read_bs()?;
            let cached_value_code = data.read_bs()? as i16;
            if value_dependency_version != 0 || cached_value_code != 90 {
                return Err(format!("ACDBASSOCVALUEDEPENDENCY {handle:#x} has unsupported value version/code {value_dependency_version}/{cached_value_code}"));
            }
            let cached_value = semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgEvaluationVariant::Integer32(data.read_bl()? as i32);
            if data.bit_position() != main_end_bit {
                return Err(format!("ACDBASSOCVALUEDEPENDENCY {handle:#x} main stream is not exactly consumed: {} != {main_end_bit}", data.bit_position()));
            }
            let strings = strings.as_mut().ok_or_else(|| format!("ACDBASSOCVALUEDEPENDENCY {handle:#x} string stream missing"))?;
            let name = if has_name { Some(strings.read_tu()?) } else { None };
            let value_name = strings.read_tu()?;
            let string_end_bit = r2010_string_content_end_bit(payload, data_end_bit)?;
            if strings.bit_position() != string_end_bit {
                return Err(format!("ACDBASSOCVALUEDEPENDENCY {handle:#x} string stream is not exactly consumed: {} != {string_end_bit}", strings.bit_position()));
            }
            let dependent_on_object_handle = read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("ACDBASSOCVALUEDEPENDENCY {handle:#x} dependent-on object is null"))?;
            let read_dependency_handle = read_object_handle(&mut handle_reader, handle)?;
            let dependency_node_handle = read_object_handle(&mut handle_reader, handle)?;
            let dependency_body_handle = read_object_handle(&mut handle_reader, handle)?;
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "ACDBASSOCVALUEDEPENDENCY")?;
            object.body = Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::AssociativeValueDependency(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAssociativeValueDependency {
                dependency: semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAssociativeDependency {
                    status: semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAssociativeDependencyStatus::UpToDate,
                    is_read_dependency,
                    is_write_dependency,
                    is_attached_to_object,
                    is_delegating_to_owning_action,
                    order,
                    dependent_on_object_handle,
                    name,
                    read_dependency_handle,
                    dependency_node_handle,
                    dependency_body_handle,
                    dependency_body_id,
                },
                cached_value,
                value_name,
            }));
        } else if type_code == 542 || object.class_name == "ACDBASSOCDEPENDENCY" {
            let class_version = data.read_bs().map_err(|error| format!("ACDBASSOCDEPENDENCY {handle:#x} class version: {error}"))?;
            if class_version != 1 {
                return Err(format!("ACDBASSOCDEPENDENCY {handle:#x} class version {class_version} is unsupported"));
            }
            let status = data.read_bl().map_err(|error| format!("ACDBASSOCDEPENDENCY {handle:#x} status: {error}"))?;
            if status != 0 {
                return Err(format!("ACDBASSOCDEPENDENCY {handle:#x} status {status} is unsupported"));
            }
            let is_read_dependency = data.read_b()?;
            let is_write_dependency = data.read_b()?;
            let is_attached_to_object = data.read_b()?;
            let is_delegating_to_owning_action = data.read_b()?;
            let order = data.read_bl()? as i32;
            let has_name = data.read_b()?;
            let dependency_body_id = data.read_bl()? as i32;
            if data.bit_position() != main_end_bit {
                return Err(format!("ACDBASSOCDEPENDENCY {handle:#x} main stream is not exactly consumed: {} != {main_end_bit}", data.bit_position()));
            }
            let name = if has_name { Some(strings.as_mut().ok_or_else(|| format!("ACDBASSOCDEPENDENCY {handle:#x} string stream missing"))?.read_tu()?) } else { None };
            if strings.as_ref().is_some_and(|reader| reader.bit_position() != main_end_bit) {
                return Err(format!("ACDBASSOCDEPENDENCY {handle:#x} string stream is not exactly consumed"));
            }
            let dependent_on_object_handle = read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("ACDBASSOCDEPENDENCY {handle:#x} dependent-on object is null"))?;
            let read_dependency_handle = read_object_handle(&mut handle_reader, handle)?;
            let dependency_node_handle = read_object_handle(&mut handle_reader, handle)?;
            let dependency_body_handle = read_object_handle(&mut handle_reader, handle)?;
            validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "ACDBASSOCDEPENDENCY")?;
            object.body = Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::AssociativeDependency(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAssociativeDependency {
                status: semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgAssociativeDependencyStatus::UpToDate,
                is_read_dependency,
                is_write_dependency,
                is_attached_to_object,
                is_delegating_to_owning_action,
                order,
                dependent_on_object_handle,
                name,
                read_dependency_handle,
                dependency_node_handle,
                dependency_body_handle,
                dependency_body_id,
            }));
        } else if type_code == 42 || object.class_name == "ACDBDICTIONARYWDFLT" {
            let item_count = data.read_bl().map_err(|error| format!("dictionary {handle:#x} item count: {error}"))? as usize;
            let cloning_flag = data.read_bs().map_err(|error| format!("dictionary {handle:#x} cloning flag: {error}"))?;
            let hard_owner = data.read_rc().map_err(|error| format!("dictionary {handle:#x} hard-owner flag: {error}"))? != 0;
            let names = (0..item_count).map(|_| strings.as_mut().ok_or("dictionary string stream missing")?.read_tu().map_err(|error| format!("dictionary {handle:#x} entry name: {error}"))).collect::<Result<Vec<_>, String>>()?;
            let mut entries = Vec::with_capacity(item_count);
            for name in names {
                let reference = read_object_handle(&mut handle_reader, handle).map_err(|error| format!("dictionary {handle:#x} entry {name} handle: {error}"))?.ok_or_else(|| format!("dictionary {handle:#x} entry {name} has a null handle"))?;
                entries.push(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgNamedReference { name, handle: reference });
            }
            let default_entry_handle = if object.class_name == "ACDBDICTIONARYWDFLT" { read_object_handle(&mut handle_reader, handle).map_err(|error| format!("dictionary {handle:#x} default entry handle: {error}"))? } else { None };
            object.body = Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::Dictionary(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDictionaryBody { entries, cloning_flag, hard_owner, default_entry_handle }));
        } else if matches!(type_code, 48 | 50 | 52 | 56 | 60 | 62 | 64 | 66 | 68 | 70) {
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockTableControl;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDimensionStyleTableControl;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLinetypeTableControl;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgTableControlBody;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgTableControlEntries;
            use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgTableControlEntry;
            let entry_count = match type_code {
                48 | 50 | 52 | 60 => data.read_bl().map(|value| value as usize),
                _ => data.read_bs().map(|value| value as usize),
            }
            .map_err(|error| format!("table control {handle:#x} entry count: {error}"))?;
            let additional_count = if type_code == 68 { data.read_rc().map_err(|error| format!("DIMSTYLE control {handle:#x} additional count: {error}"))? as usize } else { 0 };
            let mut entry_handles = Vec::with_capacity(entry_count);
            for index in 0..entry_count {
                entry_handles.push(DwgTableControlEntry { handle: read_object_handle(&mut handle_reader, handle).map_err(|error| format!("table control {handle:#x} entry {index}: {error}"))? });
            }
            let body = match type_code {
                48 => DwgTableControlBody::Block(DwgBlockTableControl {
                    entry_handles,
                    model_space_handle: Some(read_object_handle(&mut handle_reader, handle)?.ok_or("BLOCK_CONTROL model-space handle is null")?),
                    paper_space_handle: Some(read_object_handle(&mut handle_reader, handle)?.ok_or("BLOCK_CONTROL paper-space handle is null")?),
                }),
                50 => DwgTableControlBody::Layer(DwgTableControlEntries { entry_handles }),
                52 => DwgTableControlBody::TextStyle(DwgTableControlEntries { entry_handles }),
                56 => DwgTableControlBody::Linetype(DwgLinetypeTableControl {
                    entry_handles,
                    by_block_handle: read_object_handle(&mut handle_reader, handle)?.ok_or("LTYPE_CONTROL ByBlock handle is null")?,
                    by_layer_handle: read_object_handle(&mut handle_reader, handle)?.ok_or("LTYPE_CONTROL ByLayer handle is null")?,
                }),
                60 => DwgTableControlBody::View(DwgTableControlEntries { entry_handles }),
                62 => DwgTableControlBody::Ucs(DwgTableControlEntries { entry_handles }),
                64 => DwgTableControlBody::Viewport(DwgTableControlEntries { entry_handles }),
                66 => DwgTableControlBody::RegisteredApplication(DwgTableControlEntries { entry_handles }),
                68 => {
                    let mut additional_handles = Vec::with_capacity(additional_count);
                    for index in 0..additional_count {
                        additional_handles.push(
                            read_object_handle(&mut handle_reader, handle)
                                .map_err(|error| format!("DIMSTYLE control {handle:#x} additional handle {index}: {error}"))?
                                .ok_or_else(|| format!("DIMSTYLE control {handle:#x} additional handle {index} is null"))?,
                        );
                    }
                    DwgTableControlBody::DimensionStyle(DwgDimensionStyleTableControl { entry_handles, additional_handles })
                }
                _ => return Err(format!("unsupported table control type {type_code}")),
            };
            object.body = Some(DwgLogicalObjectBody::TableControl(body));
        } else if matches!(type_code, 49 | 51 | 53 | 57 | 65 | 67 | 69) {
            let xref_resolution = data.read_bs().map_err(|error| format!("table record {handle:#x} xref resolution: {error}"))?;
            if !matches!(xref_resolution, 0 | 256) {
                return Err(format!("table record {handle:#x} xref resolution {xref_resolution} is invalid"));
            }
            let text_style = if type_code == 53 {
                Some((
                    data.read_b().map_err(|error| format!("text style {handle:#x} shape flag: {error}"))?,
                    data.read_b().map_err(|error| format!("text style {handle:#x} vertical flag: {error}"))?,
                    data.read_bd().map_err(|error| format!("text style {handle:#x} text size: {error}"))?,
                    data.read_bd().map_err(|error| format!("text style {handle:#x} width factor: {error}"))?,
                    data.read_bd().map_err(|error| format!("text style {handle:#x} oblique angle: {error}"))?,
                    data.read_rc().map_err(|error| format!("text style {handle:#x} generation: {error}"))?,
                    data.read_bd().map_err(|error| format!("text style {handle:#x} last height: {error}"))?,
                ))
            } else {
                None
            };
            let viewport = if type_code == 65 {
                let view_height = data.read_bd().map_err(|error| format!("viewport table {handle:#x} height: {error}"))?;
                let view_width = data.read_bd().map_err(|error| format!("viewport table {handle:#x} width: {error}"))?;
                let center = data.read_2rd().map_err(|error| format!("viewport table {handle:#x} center: {error}"))?;
                let target = data.read_3bd().map_err(|error| format!("viewport table {handle:#x} target: {error}"))?;
                let direction = data.read_3bd().map_err(|error| format!("viewport table {handle:#x} direction: {error}"))?;
                let twist = data.read_bd().map_err(|error| format!("viewport table {handle:#x} twist: {error}"))?;
                let lens_length = data.read_bd().map_err(|error| format!("viewport table {handle:#x} lens: {error}"))?;
                let front_clipping = data.read_bd().map_err(|error| format!("viewport table {handle:#x} front clipping: {error}"))?;
                let back_clipping = data.read_bd().map_err(|error| format!("viewport table {handle:#x} back clipping: {error}"))?;
                let view_mode = [data.read_b()?, data.read_b()?, data.read_b()?, data.read_b()?];
                let render_mode = data.read_rc()?;
                let use_default_lights = data.read_b()?;
                let default_lighting_type = data.read_rc()?;
                let brightness = data.read_bd()?;
                let contrast = data.read_bd()?;
                let ambient_index = data.read_bs()?;
                let ambient_rgb = data.read_bl()?;
                let ambient_flags = data.read_rc()?;
                if ambient_flags > 3 {
                    return Err(format!("viewport table {handle:#x} ambient color flags {ambient_flags:#x} are invalid"));
                }
                Some((
                    semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgViewportTableRecord {
                        common: Default::default(),
                        view_height,
                        view_width,
                        center,
                        target,
                        direction,
                        twist,
                        lens_length,
                        front_clipping,
                        back_clipping,
                        view_mode,
                        render_mode,
                        use_default_lights,
                        default_lighting_type,
                        brightness,
                        contrast,
                        ambient_color: semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgComplexColor { index: ambient_index, value: decode_complex_color_value(ambient_rgb)?, name: None, book_name: None },
                        lower_left: data.read_2rd()?,
                        upper_right: data.read_2rd()?,
                        ucs_follow: data.read_b()?,
                        circle_zoom: data.read_bs()?,
                        fast_zoom: data.read_b()?,
                        ucs_icon: data.read_bb()?,
                        grid_mode: data.read_b()?,
                        grid_unit: data.read_2rd()?,
                        snap_mode: data.read_b()?,
                        snap_style: data.read_b()?,
                        snap_isopair: data.read_bs()?,
                        snap_angle: data.read_bd()?,
                        snap_base: data.read_2rd()?,
                        snap_unit: data.read_2rd()?,
                        ucs_at_origin: data.read_b()?,
                        ucs_viewport: data.read_b()?,
                        ucs_origin: data.read_3bd()?,
                        ucs_x_axis: data.read_3bd()?,
                        ucs_y_axis: data.read_3bd()?,
                        ucs_elevation: data.read_bd()?,
                        ucs_orthographic_view: data.read_bs()?,
                        grid_flags: data.read_bs()?,
                        grid_major: data.read_bs()?,
                        background_handle: None,
                        visual_style_handle: None,
                        sun_handle: None,
                        named_ucs_handle: None,
                        base_ucs_handle: None,
                    },
                    ambient_flags,
                ))
            } else {
                None
            };
            let dimension_style = if type_code == 69 {
                use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDimensionBehavior;
                use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDimensionGeometry;
                use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDimensionR2010;
                use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDimensionStyleTableRecord;
                use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDimensionText;
                use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgDimensionUnits;
                let geometry = DwgDimensionGeometry {
                    scale: data.read_bd()?,
                    arrow_size: data.read_bd()?,
                    extension_origin_offset: data.read_bd()?,
                    dimension_line_increment: data.read_bd()?,
                    extension_line_extension: data.read_bd()?,
                    rounding: data.read_bd()?,
                    dimension_line_extension: data.read_bd()?,
                    plus_tolerance: data.read_bd()?,
                    minus_tolerance: data.read_bd()?,
                    fixed_extension_length: data.read_bd()?,
                    jog_angle: data.read_bd()?,
                };
                let fill_mode = data.read_bs()?;
                let (fill_color, fill_flags) = read_r2010_cmc_main(&mut data)?;
                let behavior = DwgDimensionBehavior {
                    tolerance: data.read_b()?,
                    limits: data.read_b()?,
                    text_inside_horizontal: data.read_b()?,
                    text_outside_horizontal: data.read_b()?,
                    suppress_extension_1: data.read_b()?,
                    suppress_extension_2: data.read_b()?,
                    text_vertical_alignment: data.read_bs()?,
                    zero_suppression: data.read_bs()?,
                    angular_zero_suppression: data.read_bs()?,
                    arc_symbol: data.read_bs()?,
                };
                let mut text = DwgDimensionText {
                    height: data.read_bd()?,
                    center_mark_size: data.read_bd()?,
                    tick_size: data.read_bd()?,
                    alternate_scale: data.read_bd()?,
                    linear_scale: data.read_bd()?,
                    vertical_position: data.read_bd()?,
                    tolerance_scale: data.read_bd()?,
                    gap: data.read_bd()?,
                    alternate_rounding: data.read_bd()?,
                    alternate_enabled: data.read_b()?,
                    alternate_decimals: data.read_bs()?,
                    text_outside_extensions: data.read_b()?,
                    separate_arrowheads: data.read_b()?,
                    force_text_inside: data.read_b()?,
                    suppress_outside_extensions: data.read_b()?,
                    ..Default::default()
                };
                let (c1, c1f) = read_r2010_cmc_main(&mut data)?;
                let (c2, c2f) = read_r2010_cmc_main(&mut data)?;
                let (c3, c3f) = read_r2010_cmc_main(&mut data)?;
                text.dimension_line_color = c1;
                text.extension_line_color = c2;
                text.text_color = c3;
                let units = DwgDimensionUnits {
                    alternate_decimal_places: data.read_bs()?,
                    decimal_places: data.read_bs()?,
                    tolerance_decimal_places: data.read_bs()?,
                    alternate_units: data.read_bs()?,
                    alternate_tolerance_decimal_places: data.read_bs()?,
                    angular_units: data.read_bs()?,
                    fraction_format: data.read_bs()?,
                    linear_units: data.read_bs()?,
                    decimal_separator: data.read_bs()?,
                    text_movement: data.read_bs()?,
                    text_horizontal_alignment: data.read_bs()?,
                    suppress_dimension_line_1: data.read_b()?,
                    suppress_dimension_line_2: data.read_b()?,
                    tolerance_vertical_alignment: data.read_bs()?,
                    tolerance_zero_suppression: data.read_bs()?,
                    alternate_zero_suppression: data.read_bs()?,
                    alternate_tolerance_zero_suppression: data.read_bs()?,
                    user_positioned_text: data.read_b()?,
                    arrow_text_fit: data.read_bs()?,
                };
                let r2010 = DwgDimensionR2010 {
                    fixed_extension_enabled: data.read_b()?,
                    text_direction: data.read_b()?,
                    alternate_measurement_factor: data.read_bd()?,
                    alternate_measurement_suffix: String::new(),
                    measurement_factor: data.read_bd()?,
                    measurement_suffix: String::new(),
                    dimension_lineweight: data.read_bs()?,
                    extension_lineweight: data.read_bs()?,
                    flag: data.read_b()?,
                };
                Some((
                    DwgDimensionStyleTableRecord {
                        common: Default::default(),
                        dimension_postfix: String::new(),
                        alternate_postfix: String::new(),
                        geometry,
                        fill_mode,
                        fill_color,
                        behavior,
                        text,
                        units,
                        r2010,
                        text_style_handle: None,
                        leader_arrow_handle: None,
                        arrow_handle: None,
                        arrow_1_handle: None,
                        arrow_2_handle: None,
                        dimension_linetype_handle: None,
                        extension_1_linetype_handle: None,
                        extension_2_linetype_handle: None,
                    },
                    [fill_flags, c1f, c2f, c3f],
                ))
            } else {
                None
            };
            let block_header = if type_code == 49 {
                let anonymous = data.read_b().map_err(|error| format!("block header {handle:#x} anonymous flag: {error}"))?;
                let has_attributes = data.read_b().map_err(|error| format!("block header {handle:#x} attribute flag: {error}"))?;
                let is_xref = data.read_b().map_err(|error| format!("block header {handle:#x} xref flag: {error}"))?;
                let xref_overlaid = data.read_b().map_err(|error| format!("block header {handle:#x} overlay flag: {error}"))?;
                let xref_loaded = data.read_b().map_err(|error| format!("block header {handle:#x} loaded flag: {error}"))?;
                let owned_count = if !is_xref && !xref_overlaid { data.read_bl().map_err(|error| format!("block header {handle:#x} owned count: {error}"))? as usize } else { 0 };
                let base_point = data.read_3bd().map_err(|error| format!("block header {handle:#x} base point: {error}"))?;
                let mut insert_count = 0usize;
                loop {
                    let marker = data.read_rc().map_err(|error| format!("block header {handle:#x} insert marker: {error}"))?;
                    if marker == 0 {
                        break;
                    }
                    if marker != 1 || insert_count >= 0xefffff {
                        return Err(format!("block header {handle:#x} insert marker {marker} is invalid"));
                    }
                    insert_count += 1;
                }
                let preview_size = data.read_bl().map_err(|error| format!("block header {handle:#x} preview size: {error}"))? as usize;
                if preview_size != 0 {
                    return Err(format!("block header {handle:#x} has unsupported semantic preview of {preview_size} bytes"));
                }
                let insert_units = data.read_bs().map_err(|error| format!("block header {handle:#x} insert units: {error}"))?;
                let explodable = data.read_b().map_err(|error| format!("block header {handle:#x} explodable flag: {error}"))?;
                let block_scaling = data.read_rc().map_err(|error| format!("block header {handle:#x} block scaling: {error}"))?;
                Some((anonymous, has_attributes, is_xref, xref_overlaid, xref_loaded, owned_count, base_point, insert_count, insert_units, explodable, block_scaling))
            } else {
                None
            };
            let linetype = if type_code == 57 {
                let pattern_length = data.read_bd().map_err(|error| format!("linetype {handle:#x} pattern length: {error}"))?;
                let alignment = data.read_rc().map_err(|error| format!("linetype {handle:#x} alignment: {error}"))?;
                let dash_count = data.read_rc().map_err(|error| format!("linetype {handle:#x} dash count: {error}"))? as usize;
                let mut dashes = Vec::with_capacity(dash_count);
                for index in 0..dash_count {
                    dashes.push((
                        data.read_bd().map_err(|error| format!("linetype {handle:#x} dash {index} length: {error}"))?,
                        data.read_bs().map_err(|error| format!("linetype {handle:#x} dash {index} shape code: {error}"))?,
                        data.read_rd().map_err(|error| format!("linetype {handle:#x} dash {index} X offset: {error}"))?,
                        data.read_rd().map_err(|error| format!("linetype {handle:#x} dash {index} Y offset: {error}"))?,
                        data.read_bd().map_err(|error| format!("linetype {handle:#x} dash {index} scale: {error}"))?,
                        data.read_bd().map_err(|error| format!("linetype {handle:#x} dash {index} rotation: {error}"))?,
                        data.read_bs().map_err(|error| format!("linetype {handle:#x} dash {index} shape flags: {error}"))?,
                    ));
                }
                Some((pattern_length, alignment, dashes))
            } else {
                None
            };
            let layer = if type_code == 51 {
                let flag0 = data.read_bs().map_err(|error| format!("layer {handle:#x} flags: {error}"))?;
                let color_index = data.read_bs().map_err(|error| format!("layer {handle:#x} color index: {error}"))?;
                let color_rgb = data.read_bl().map_err(|error| format!("layer {handle:#x} color RGB: {error}"))?;
                let color_flags = data.read_rc().map_err(|error| format!("layer {handle:#x} color flags: {error}"))?;
                if color_flags > 3 {
                    return Err(format!("layer {handle:#x} color flags {color_flags:#x} are invalid"));
                }
                Some((flag0, color_index, color_rgb, color_flags))
            } else {
                None
            };
            let group_71 = if type_code == 67 { Some(data.read_rc().map_err(|error| format!("registered-application {handle:#x} group-71 marker: {error}"))?) } else { None };
            if data.bit_position() != main_end_bit {
                return Err(format!("table record {handle:#x} main stream is not exactly consumed: {} != {main_end_bit}", data.bit_position()));
            }
            let strings = strings.as_mut().ok_or_else(|| format!("table record {handle:#x} string stream missing"))?;
            let name = strings.read_tu().map_err(|error| format!("table record {handle:#x} name: {error}"))?;
            let xref_handle = read_object_handle(&mut handle_reader, handle).map_err(|error| format!("table record {handle:#x} xref handle: {error}"))?;
            let common = semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgTableRecordCommon { name, xref_resolution, xref_handle };
            let body = if let Some((mut value, color_flags)) = dimension_style {
                value.common = common;
                value.dimension_postfix = strings.read_tu()?;
                value.alternate_postfix = strings.read_tu()?;
                let mut read_color_strings = |color: &mut semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgComplexColor, flags: u8| -> Result<(), String> {
                    if flags & 1 != 0 {
                        color.name = Some(strings.read_tu()?);
                    }
                    if flags & 2 != 0 {
                        color.book_name = Some(strings.read_tu()?);
                    }
                    Ok(())
                };
                read_color_strings(&mut value.fill_color, color_flags[0])?;
                read_color_strings(&mut value.text.dimension_line_color, color_flags[1])?;
                read_color_strings(&mut value.text.extension_line_color, color_flags[2])?;
                read_color_strings(&mut value.text.text_color, color_flags[3])?;
                value.r2010.alternate_measurement_suffix = strings.read_tu()?;
                value.r2010.measurement_suffix = strings.read_tu()?;
                value.text_style_handle = read_object_handle(&mut handle_reader, handle)?;
                value.leader_arrow_handle = read_object_handle(&mut handle_reader, handle)?;
                value.arrow_handle = read_object_handle(&mut handle_reader, handle)?;
                value.arrow_1_handle = read_object_handle(&mut handle_reader, handle)?;
                value.arrow_2_handle = read_object_handle(&mut handle_reader, handle)?;
                value.dimension_linetype_handle = read_object_handle(&mut handle_reader, handle)?;
                value.extension_1_linetype_handle = read_object_handle(&mut handle_reader, handle)?;
                value.extension_2_linetype_handle = read_object_handle(&mut handle_reader, handle)?;
                semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgTableRecordBody::DimensionStyle(value)
            } else if let Some((mut value, ambient_flags)) = viewport {
                value.common = common;
                if ambient_flags & 1 != 0 {
                    value.ambient_color.name = Some(strings.read_tu().map_err(|error| format!("viewport table {handle:#x} ambient color name: {error}"))?);
                }
                if ambient_flags & 2 != 0 {
                    value.ambient_color.book_name = Some(strings.read_tu().map_err(|error| format!("viewport table {handle:#x} ambient color book: {error}"))?);
                }
                value.background_handle = read_object_handle(&mut handle_reader, handle)?;
                value.visual_style_handle = read_object_handle(&mut handle_reader, handle)?;
                value.sun_handle = read_object_handle(&mut handle_reader, handle)?;
                value.named_ucs_handle = read_object_handle(&mut handle_reader, handle)?;
                value.base_ucs_handle = read_object_handle(&mut handle_reader, handle)?;
                semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgTableRecordBody::Viewport(value)
            } else if let Some((anonymous, has_attributes, is_xref, xref_overlaid, xref_loaded, owned_count, base_point, insert_count, insert_units, explodable, block_scaling)) = block_header {
                let xref_path = strings.read_tu().map_err(|error| format!("block header {handle:#x} xref path: {error}"))?;
                let description = strings.read_tu().map_err(|error| format!("block header {handle:#x} description: {error}"))?;
                let block_entity_handle = read_object_handle(&mut handle_reader, handle).map_err(|error| format!("block header {handle:#x} block entity: {error}"))?.ok_or_else(|| format!("block header {handle:#x} block entity is null"))?;
                let mut owned_entity_handles = Vec::with_capacity(owned_count);
                for index in 0..owned_count {
                    owned_entity_handles
                        .push(read_object_handle(&mut handle_reader, handle).map_err(|error| format!("block header {handle:#x} owned entity {index}: {error}"))?.ok_or_else(|| format!("block header {handle:#x} owned entity {index} is null"))?);
                }
                let end_block_entity_handle =
                    read_object_handle(&mut handle_reader, handle).map_err(|error| format!("block header {handle:#x} end-block entity: {error}"))?.ok_or_else(|| format!("block header {handle:#x} end-block entity is null"))?;
                let mut insert_backreference_handles = Vec::with_capacity(insert_count);
                for index in 0..insert_count {
                    insert_backreference_handles.push(
                        read_object_handle(&mut handle_reader, handle)
                            .map_err(|error| format!("block header {handle:#x} insert backreference {index}: {error}"))?
                            .ok_or_else(|| format!("block header {handle:#x} insert backreference {index} is null"))?,
                    );
                }
                let layout_handle = read_object_handle(&mut handle_reader, handle).map_err(|error| format!("block header {handle:#x} layout: {error}"))?;
                semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgTableRecordBody::BlockHeader(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgBlockHeaderTableRecord {
                    common,
                    anonymous,
                    has_attributes,
                    is_xref,
                    xref_overlaid,
                    xref_loaded,
                    owned_entity_handles,
                    base_point,
                    xref_path,
                    insert_backreference_handles,
                    description,
                    insert_units,
                    explodable,
                    block_scaling,
                    block_entity_handle,
                    end_block_entity_handle,
                    layout_handle,
                })
            } else if let Some((pattern_length, alignment, dash_values)) = linetype {
                let description = strings.read_tu().map_err(|error| format!("linetype {handle:#x} description: {error}"))?;
                let mut dashes = Vec::with_capacity(dash_values.len());
                for (index, (length, complex_shape_code, x_offset, y_offset, scale, rotation, shape_flags)) in dash_values.into_iter().enumerate() {
                    let style_handle = read_object_handle(&mut handle_reader, handle).map_err(|error| format!("linetype {handle:#x} dash {index} style: {error}"))?;
                    dashes.push(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLinetypeDash { length, complex_shape_code, style_handle, x_offset, y_offset, scale, rotation, shape_flags, text: None });
                }
                semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgTableRecordBody::Linetype(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLinetypeTableRecord { common, description, pattern_length, alignment, dashes })
            } else if let Some((flag0, color_index, color_rgb, color_flags)) = layer {
                let color_name = if color_flags & 1 != 0 { Some(strings.read_tu().map_err(|error| format!("layer {handle:#x} color name: {error}"))?) } else { None };
                let color_book_name = if color_flags & 2 != 0 { Some(strings.read_tu().map_err(|error| format!("layer {handle:#x} color book name: {error}"))?) } else { None };
                let plot_style_handle = read_object_handle(&mut handle_reader, handle).map_err(|error| format!("layer {handle:#x} plot-style handle: {error}"))?;
                let material_handle = read_object_handle(&mut handle_reader, handle).map_err(|error| format!("layer {handle:#x} material handle: {error}"))?;
                let linetype_handle = read_object_handle(&mut handle_reader, handle).map_err(|error| format!("layer {handle:#x} linetype handle: {error}"))?;
                semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgTableRecordBody::Layer(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLayerTableRecord {
                    common,
                    frozen: flag0 & 1 != 0,
                    off: flag0 & 2 != 0,
                    frozen_in_new_viewports: flag0 & 4 != 0,
                    locked: flag0 & 8 != 0,
                    plottable: flag0 & 16 != 0,
                    lineweight: ((flag0 & 0x03e0) >> 5) as u8,
                    color: semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgComplexColor { index: color_index, value: decode_complex_color_value(color_rgb)?, name: color_name, book_name: color_book_name },
                    plot_style_handle,
                    material_handle,
                    linetype_handle,
                })
            } else if let Some((is_shape, is_vertical, text_size, width_factor, oblique_angle, generation, last_height)) = text_style {
                let font_file = strings.read_tu().map_err(|error| format!("text style {handle:#x} font file: {error}"))?;
                let big_font_file = strings.read_tu().map_err(|error| format!("text style {handle:#x} big-font file: {error}"))?;
                semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgTableRecordBody::TextStyle(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgTextStyleTableRecord { common, is_shape, is_vertical, text_size, width_factor, oblique_angle, generation, last_height, font_file, big_font_file })
            } else {
                semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgTableRecordBody::RegisteredApplication(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgRegisteredApplicationTableRecord { common, group_71: group_71.unwrap() })
            };
            object.body = Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::TableRecord(body));
        } else if type_code == 79 || object.class_name == "XRECORD" {
            let data_byte_count = data.read_bl().map_err(|error| format!("XRECORD {handle:#x} value byte count: {error}"))? as usize;
            let values = decode_xrecord_values(&mut data, data_byte_count, main_end_bit).map_err(|error| format!("XRECORD {handle:#x}: {error}"))?;
            let cloning_flag = data.read_bs().map_err(|error| format!("XRECORD {handle:#x} cloning flag: {error}"))?;
            if data.bit_position() != main_end_bit {
                return Err(format!("XRECORD {handle:#x} class-main stream is not exactly consumed: {} != {main_end_bit}", data.bit_position()));
            }
            let handle_end_bit = payload_size.saturating_mul(8);
            let mut object_id_handles = Vec::new();
            while handle_end_bit.saturating_sub(handle_reader.bit_position()) >= 8 {
                let checkpoint = handle_reader.clone();
                match read_object_handle(&mut handle_reader, handle) {
                    Ok(Some(reference)) => object_id_handles.push(reference),
                    Ok(None) => break,
                    Err(_) => {
                        handle_reader = checkpoint;
                        break;
                    }
                }
            }
            let terminal_padding = handle_end_bit.saturating_sub(handle_reader.bit_position());
            if terminal_padding > 7 {
                return Err(format!("XRECORD {handle:#x} has {terminal_padding} trailing handle-stream bits"));
            }
            let mut padding_value = 0u8;
            for _ in 0..terminal_padding {
                padding_value = (padding_value << 1) | u8::from(handle_reader.read_b().map_err(|error| format!("XRECORD {handle:#x} terminal fill: {error}"))?);
            }
            let expected_padding = (1u8 << terminal_padding).saturating_sub(1);
            if padding_value != expected_padding {
                return Err(format!("XRECORD {handle:#x} terminal handle-stream fill is {padding_value:#x}, expected {expected_padding:#x}"));
            }
            xrecord_terminal_fills.push((terminal_padding as u8, padding_value));
            object.body = Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::XRecord(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgXRecordBody { values, object_id_handles, cloning_flag }));
        }
        objects.push(object);
    }
    for (block_handle, native_name) in block_names {
        let headers = objects
            .iter()
            .enumerate()
            .filter_map(|(index, object)| match object.body.as_ref() {
                Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::TableRecord(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgTableRecordBody::BlockHeader(header))) if header.block_entity_handle == block_handle => Some(index),
                _ => None,
            })
            .collect::<Vec<_>>();
        if headers.len() != 1 {
            return Err(format!("BLOCK {block_handle:#x} has {} block-header relationships", headers.len()));
        }
        let header_index = headers[0];
        let marker_owner = objects.iter().find(|object| object.handle == block_handle).and_then(|object| object.owner_handle);
        let header_handle = objects[header_index].handle;
        let ordinary_index = objects.iter().find_map(|object| match object.body.as_ref() {
            Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::TableControl(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgTableControlBody::Block(control))) => control.entry_handles.iter().position(|entry| entry.handle == Some(header_handle)),
            _ => None,
        });
        let Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObjectBody::TableRecord(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgTableRecordBody::BlockHeader(header))) = objects[header_index].body.as_mut() else { unreachable!() };
        if let Some(owner) = marker_owner {
            if owner != header_handle {
                return Err(format!("BLOCK {block_handle:#x} owner {owner:#x} differs from block header {header_handle:#x}"));
            }
        }
        if header.anonymous {
            let suffix = native_name.strip_prefix(&header.common.name).ok_or_else(|| format!("anonymous BLOCK {block_handle:#x} name {native_name:?} does not extend prefix {:?}", header.common.name))?;
            if suffix.is_empty() || !suffix.chars().all(|character| character.is_ascii_digit()) {
                return Err(format!("anonymous BLOCK {block_handle:#x} name {native_name:?} has a nonnumeric identity"));
            }
            let suffix_index = suffix.parse::<usize>().map_err(|error| format!("anonymous BLOCK {block_handle:#x} identity: {error}"))?;
            if ordinary_index != Some(suffix_index) {
                return Err(format!("anonymous BLOCK {block_handle:#x} identity {suffix_index} differs from ordinary block-control index {ordinary_index:?}"));
            }
            header.common.name = native_name;
        } else if header.common.name != native_name {
            return Err(format!("BLOCK {block_handle:#x} native name {native_name:?} differs from block-header name {:?}", header.common.name));
        }
    }
    Ok((objects, xrecord_terminal_fills))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_r2004_object_identities(bytes: &[u8], classes: &[semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgClass]) -> Result<Vec<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgLogicalObject>, String> {
    decode_r2004_object_records(bytes, classes).map(|(objects, _)| objects)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn r2010_string_stream(bytes: &[u8], end_bit: usize) -> Result<(DwgBitReader<'_>, usize), String> {
    if end_bit < 17 || end_bit > bytes.len().saturating_mul(8) {
        return Err("R2010 string-stream end is out of bounds".into());
    }
    let mut present = DwgBitReader::at_bit(bytes, end_bit - 1)?;
    if !present.read_b()? {
        return Ok((DwgBitReader::at_bit(bytes, end_bit - 1)?, end_bit - 1));
    }
    let mut size_reader = DwgBitReader::at_bit(bytes, end_bit - 17)?;
    let low = size_reader.read_rs()?;
    let (size_bits, header_bits) = if low & 0x8000 == 0 {
        (usize::from(low), 17usize)
    } else {
        if end_bit < 33 {
            return Err("R2010 extended string-stream size is truncated".into());
        }
        let mut high_reader = DwgBitReader::at_bit(bytes, end_bit - 33)?;
        ((usize::from(low & 0x7fff)) | (usize::from(high_reader.read_rs()?) << 15), 33usize)
    };
    let start = end_bit.checked_sub(header_bits).and_then(|value| value.checked_sub(size_bits)).ok_or("R2010 string-stream size exceeds class data")?;
    Ok((DwgBitReader::at_bit(bytes, start)?, start))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn r2010_string_content_end_bit(bytes: &[u8], end_bit: usize) -> Result<usize, String> {
    let (_, start) = r2010_string_stream(bytes, end_bit)?;
    if start == end_bit.saturating_sub(1) {
        return Ok(start);
    }
    let mut size_reader = DwgBitReader::at_bit(bytes, end_bit.checked_sub(17).ok_or("R2010 string footer is truncated")?)?;
    let low = size_reader.read_rs()?;
    Ok(end_bit - if low & 0x8000 == 0 { 17 } else { 33 })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_r2010_classes_section(bytes: &[u8]) -> Result<Vec<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgClass>, String> {
    if bytes.len() < 24 || bytes[..16] != DWG_SENTINEL_CLASSES_BEGIN {
        return Err("R2010 classes section sentinel is missing".into());
    }
    let total_bits = u32::from_le_bytes(bytes[20..24].try_into().unwrap()) as usize;
    let end_bit = 20usize.checked_mul(8).and_then(|value| value.checked_add(total_bits)).ok_or("R2010 classes end overflow")?;
    let (mut strings, data_end_bit) = r2010_string_stream(bytes, end_bit)?;
    let mut data = DwgBitReader::at_bit(bytes, 24 * 8)?;
    let maximum_class = data.read_bl()? as u16;
    if !data.read_b()? {
        return Err("R2010 classes section standard marker is unset".into());
    }
    let expected = maximum_class.saturating_sub(499) as usize;
    let mut classes = Vec::with_capacity(expected);
    while classes.len() < expected && data.bit_position() < data_end_bit {
        let number = data.read_bs()?;
        let proxy_flags = u32::from(data.read_bs()?);
        let application_name = strings.read_tu()?;
        let cpp_class_name = strings.read_tu()?;
        let dxf_name = strings.read_tu()?;
        let was_zombie = data.read_b()?;
        let item_class_id = data.read_bs()?;
        let object_count = data.read_bl()?;
        let dwg_version = data.read_bl()?;
        let maintenance_version = data.read_bl()?;
        let reserved_values = vec![data.read_bl()?, data.read_bl()?];
        classes.push(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgClass { number, proxy_flags, application_name, cpp_class_name, dxf_name, was_zombie, item_class_id, object_count, dwg_version, maintenance_version, reserved_values });
    }
    Ok(classes)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_r2004_classes(bytes: &[u8]) -> Result<Vec<semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgClass>, String> {
    let sections = decode_r2004_sections(bytes)?;
    let classes = sections.iter().find(|section| section.name == "AcDb:Classes").ok_or("R2004 Classes section missing")?;
    decode_r2010_classes_section(&r2004_section_data(classes)?)
}
//#endregion DwgRead
//#endregion 🔖️DwgStructuralCodec

//#region 📸️SnapshotDecode
/// 🗺️ Materializes section pages only while deserializing and projects their standard objects.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_drawing(bytes: &[u8]) -> Result<DwgLogicalDrawing, String> {
    DwgLogicalDrawing::from_native(&dwg_from_bytes(bytes)?)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_dwg(bytes: &[u8]) -> Result<DwgSnapshot, String> {
    let version = dwg_version_sentinel(bytes)?;
    let (maintenance_version, codepage) = parse_version_header_fields(bytes)?;
    // 🫙️ The empty document. A stream that stops at the end of the preamble carries no section map,
    // no page map and no encrypted file header, so there is nothing behind it to decode — and the
    // R2004 reader's "file too short for encrypted header" is the wrong answer to give about a
    // document this artifact itself writes and commits as an example.
    if bytes.len() == DWG_PREAMBLE_LEN {
        return Ok(DwgSnapshot { schema: STDIO_DWG_DOCUMENT_SCHEMA.into(), version, maintenance_version, codepage, ..Default::default() });
    }
    if version != "AC1024" {
        return DwgSnapshot::from_drawing(&dwg_from_bytes(bytes)?);
    }
    let mut drawing = decode_drawing(bytes)?;
    let classes = decode_r2004_classes(bytes)?;
    drawing.objects = decode_r2004_object_identities(bytes, &classes)?;
    let document = decode_r2004_document_sections(bytes)?;
    Ok(DwgSnapshot {
        schema: STDIO_DWG_DOCUMENT_SCHEMA.into(),
        version,
        maintenance_version,
        codepage,
        drawing,
        header: document.header,
        classes,
        dependencies: document.dependencies,
        summary: document.summary,
        application: document.application,
        template: document.template,
        auxiliary_header: document.auxiliary_header,
        revision_history: document.revision_history,
        preview: document.preview,
        application_history: document.application_history,
    })
}
//#endregion 📸️SnapshotDecode
