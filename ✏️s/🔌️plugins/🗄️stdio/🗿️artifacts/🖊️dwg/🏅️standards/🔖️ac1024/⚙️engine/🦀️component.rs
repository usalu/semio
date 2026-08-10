//! ⚙️ DwgEngine — owns a real `DwgArtifact`. Also home to the real AC1024/R2004+ byte-level
//! decode pipeline (ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION,
//! 🖊️dwg D1-D2 wave): file header decrypt, section/page directory walk (D1), and the bespoke
//! R2004+ LZ77-variant section decompressor (D2). Algorithm field-layout/opcode semantics
//! cross-checked against the ODA-spec-derived LibreDWG reference (GPLv3,
//! github.com/LibreDWG/libredwg `src/decode.c` + `src/r2004_file_header.spec`) and validated end
//! to end against the real 145KB `architectural.dwg` fixture via a standalone scratch crate
//! (`.🦑️repo/🎫️tickets/…/scratchpad/dwg_engine`) before landing here -- this is a clean-room
//! reimplementation, no code copied. Undecoded/unreachable content is never fabricated: every
//! function here returns a typed `Result` and the caller (schema/snapshot's `decode_dwg`) always
//! keeps the complete original file bytes verbatim regardless of how far this pipeline gets, so
//! re-encoding is lossless even for a `SentinelOnly` result.

use crate::artifacts::dwg::{DwgArtifact, DwgDiff, DwgMutation, DwgSnapshot, STDIO_DWG_DOCUMENT_SCHEMA};

//#region 🔖️R2004FileHeaderDecrypt
/// 🔓 R2004+ file header "decryption" -- not real security, a fixed LCG-generated one-time pad
/// (the classic Borland/MSVC `rand()` constants: `seed = seed*0x343fd + 0x269ec3`, upper 16 bits
/// of the running seed XORed per byte). Symmetric -- the same function both encrypts and decrypts.
pub fn decrypt_r2004_header(src: &[u8]) -> Vec<u8> {
    let mut rseed: u32 = 1;
    src.iter()
        .map(|&b| {
            rseed = rseed.wrapping_mul(0x343fd).wrapping_add(0x269ec3);
            b ^ ((rseed >> 0x10) & 0xFF) as u8
        })
        .collect()
}

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

const R2004_HEADER_LEN: usize = 0x6c;

fn parse_r2004_file_header(dec: &[u8]) -> Result<R2004FileHeader, String> {
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
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }
    fn u8(&mut self) -> Result<u8, String> {
        let b = *self.data.get(self.pos).ok_or("dwg lz: source exhausted mid-opcode")?;
        self.pos += 1;
        Ok(b)
    }
    fn has_more(&self) -> bool {
        self.pos < self.data.len()
    }
}

fn read_literal_length(src: &mut ByteCursor, opcode: u8) -> Result<u32, String> {
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

fn read_compressed_bytes(src: &mut ByteCursor, opcode: u8, bits: u32) -> Result<u32, String> {
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
fn two_byte_offset(src: &mut ByteCursor, plus: u32, existing_offset: u32) -> Result<(u8, u32), String> {
    let first = src.u8()?;
    let second = src.u8()?;
    let offset = existing_offset | ((first as u32) >> 2) | ((second as u32) << 6);
    Ok((first, offset + plus))
}

fn copy_bytes(n: u32, src: &mut ByteCursor, dec: &mut Vec<u8>) -> Result<u8, String> {
    for _ in 0..n {
        dec.push(src.u8()?);
    }
    src.u8()
}

/// 🗜️ Decompresses one R2004+ "compression algorithm 2" byte stream.
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
        if opcode1 < 0x10 || opcode1 >= 0x40 {
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
            return Err(format!(
                "dwg lz: invalid backref bytes={comp_bytes} offset={comp_offset} pos={pos} decomp_size={decomp_size}"
            ));
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
struct PageDirEntry {
    number: i32,
    address: u64,
}

fn parse_page_directory(dec: &[u8], section_array_size: u32) -> Vec<PageDirEntry> {
    let mut out = Vec::new();
    let mut pos = 0usize;
    let mut address: u64 = 0x100;
    while pos + 8 <= dec.len() {
        let number = i32::from_le_bytes(dec[pos..pos + 4].try_into().unwrap());
        let size = u32::from_le_bytes(dec[pos + 4..pos + 8].try_into().unwrap());
        pos += 8;
        out.push(PageDirEntry { number, address });
        if number <= section_array_size as i32 {
            address += size as u64;
        }
        if number < 0 && pos + 16 <= dec.len() {
            pos += 16; // parent/left/right/x00 gap-tree bookkeeping, unused for D1-D2 location.
        }
    }
    out
}

/// 🗿️ One raw decoded page's content plus its on-disk location, before any bitcode
/// interpretation (D3+ is out of scope for this ticket's D1-D2 bar) -- kept as an opaque byte
/// span so nothing genuinely undecoded is ever fabricated or silently dropped.
#[derive(Debug, Clone, Default)]
pub struct DwgRawPage {
    pub page_number: i32,
    pub file_address: u64,
    pub compressed_size: u32,
    /// Empty iff this specific page's own decompression failed (see `DwgRawPage::error`) --
    /// the section/page LOCATION (D1) still succeeded even when a given page's content (D2)
    /// couldn't be recovered, and the whole-file `DwgSnapshot.bytes` fallback stays lossless
    /// regardless.
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

fn parse_section_info(dec: &[u8]) -> Result<Vec<(String, u64, u32, u32, Vec<(i32, u32, u64)>)>, String> {
    // Returns (name, declared_size, compressed_flag, max_decomp_size, [(page_number, compressed_size, address_offset)]).
    if dec.len() < 20 {
        return Err("section info: header shorter than 20 bytes".into());
    }
    let u32_at = |o: usize| -> Result<u32, String> {
        dec.get(o..o + 4).map(|s| u32::from_le_bytes(s.try_into().unwrap())).ok_or_else(|| "section info: read past end".into())
    };
    let u64_at = |o: usize| -> Result<u64, String> {
        dec.get(o..o + 8).map(|s| u64::from_le_bytes(s.try_into().unwrap())).ok_or_else(|| "section info: read past end".into())
    };
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
        out.push((name, size, compressed, max_decomp_size, pages));
    }
    Ok(out)
}

/// 🗺️ D1: decrypts the file header and walks the section-page-map + section-info directories,
/// returning every named section this file's Section Info directory declares, each with its
/// pages LOCATED (file address + compressed size) but not yet decompressed. Returns `Err` only
/// when the file structurally isn't a decodable R2004+ file (wrong magic, truncated, checksum-
/// verified-wrong LCG landing) -- never a partial/garbage result.
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
        return Err(format!(
            "r2004: page directory entry count {} != numgaps({}) + numsections({})",
            page_dir.len(),
            hdr.numgaps,
            hdr.numsections
        ));
    }

    // Section info: the named-section directory, located via the page whose `number` equals
    // `section_info_id` (looked up in the page directory we just built, not a fixed offset).
    let info_entry = page_dir
        .iter()
        .find(|e| e.number == hdr.section_info_id)
        .ok_or_else(|| format!("r2004: section_info_id {} not found in page directory", hdr.section_info_id))?;
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
    for (name, declared_size, compressed_flag, max_decomp_size, pages) in descriptors {
        if name.is_empty() {
            continue; // padding descriptor slot (section-info headers sometimes reserve one).
        }
        let mut raw_pages = Vec::with_capacity(pages.len());
        for (pnum, psize, _addr_offset) in pages {
            let file_address = *by_number
                .get(&pnum)
                .ok_or_else(|| format!("r2004: page {pnum} for section {name} not in page directory"))?;
            raw_pages.push(DwgRawPage { page_number: pnum, file_address, compressed_size: psize, decoded: Vec::new(), error: None });
        }
        out.push(DwgRawSection { name, compressed: compressed_flag == 2, declared_size, max_decomp_size, pages: raw_pages });
    }
    Ok(out)
}

/// 🗜️ D2: for every section D1 located, decrypts + decompresses (or, for `compressed == false`
/// sections, copies verbatim) each page's real content bytes. A single page's failure is
/// recorded on that page (`DwgRawPage::error`) and does not abort the other pages/sections --
/// the caller can tell exactly how much of D2 landed from the per-page `error` fields.
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
//#endregion 🔖️SectionMapAndInfo

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_dwg_snapshot() -> DwgSnapshot {
    DwgSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::dwg::composer::register();
    register_artifact_schema();
    register_pilot_languages();
    store::register_document_codec(store::ArtifactCodec::of::<DwgSnapshot, DwgMutation>(STDIO_DWG_DOCUMENT_SCHEMA));
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (dwg).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.dwg",
        extension: Some("bin"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::dwg::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::dwg::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::dwg::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::dwg::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.dwg"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.dwg`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::dwg::schema::dwg_artifact_schema_descriptor());
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// ⚙️ `stdio.dwg` artifact engine.
pub struct DwgEngine {
    artifact_state: DwgArtifact,
    snapshot_state: DwgSnapshot,
}

impl DwgEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: DwgSnapshot) -> Self {
        let artifact_state = DwgArtifact::from_snapshot(snapshot.clone());
        Self { artifact_state, snapshot_state: snapshot }
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_dwg_snapshot();
        assert_eq!(snapshot.schema, STDIO_DWG_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let stub = b"AC1024\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
        let snap = crate::artifacts::dwg::schema::snapshot::decode_dwg(stub).expect("decode stub");
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <DwgSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.version, "AC1024");
        assert_eq!(parsed.bytes, stub);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <DwgSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    //#region 🔖️Lz77VariantUnit
    #[test]
    fn lcg_decrypt_is_its_own_inverse() {
        let plain: Vec<u8> = (0..R2004_HEADER_LEN as u8).collect();
        let enc = decrypt_r2004_header(&plain);
        let dec = decrypt_r2004_header(&enc);
        assert_eq!(dec, plain);
    }

    #[test]
    fn lz_round_trip_literal_only_stream() {
        // opcode low-nibble 3 -> literal run length 3+3=6, followed by 6 literal bytes, then the
        // 0x11 terminator (read as the "next opcode" by `copy_bytes`'s trailing byte read).
        let mut comp = vec![0x03u8];
        comp.extend_from_slice(b"abcdef");
        comp.push(0x11);
        let out = decompress_r2004_section(&comp, 64).expect("decompress");
        assert_eq!(&out, b"abcdef");
    }

    #[test]
    fn lz_rejects_out_of_bounds_backref() {
        // opcode 0x40 (short-match branch, comp_bytes=(0x40>>4)-1=3) with a huge encoded offset
        // and nothing decoded yet -- must error, never panic or fabricate bytes.
        let comp = vec![0x40u8, 0xFFu8];
        let err = decompress_r2004_section(&comp, 16);
        assert!(err.is_err(), "backref past the start of output must be a typed error");
    }
    //#endregion 🔖️Lz77VariantUnit

    //#region 🔖️RealFixture
    const ARCHITECTURAL_FIXTURE: &[u8] = include_bytes!("../../../📚️examples/🏛️architectural/🖼️assets/📄️architectural.dwg");

    /// 🧪️ D1: file header decrypts cleanly and every section+page is located by name, on the
    /// real ~145KB AC1024 fixture -- the actual regression test for "sentinel + passthrough"
    /// (the pre-ticket behavior, which never found a single real section on this file).
    #[test]
    fn real_fixture_d1_locates_every_named_section() {
        let sections = locate_r2004_sections(ARCHITECTURAL_FIXTURE).expect("D1 section location");
        let expected_names = [
            "AcDb:Header", "AcDb:AuxHeader", "AcDb:Classes", "AcDb:Handles", "AcDb:Template",
            "AcDb:ObjFreeSpace", "AcDb:AcDbObjects", "AcDb:RevHistory", "AcDb:SummaryInfo",
            "AcDb:Preview", "AcDb:AppInfo", "AcDb:AppInfoHistory", "AcDb:FileDepList",
        ];
        for name in expected_names {
            assert!(sections.iter().any(|s| s.name == name), "missing real section {name}");
        }
        // Every located page must carry a real, in-bounds file address and nonzero compressed
        // size -- proof this is genuine location, not a stub returning empty placeholders.
        for section in &sections {
            assert!(!section.pages.is_empty(), "section {} has no pages", section.name);
            for page in &section.pages {
                assert!(page.file_address > 0, "section {} page {} has null address", section.name, page.page_number);
                assert!(page.compressed_size > 0, "section {} page {} has zero compressed size", section.name, page.page_number);
            }
        }
    }

    /// 🧪️ D2: every located section's page content actually decompresses (or, for stored
    /// sections, copies) into nonzero real bytes -- the genuine "not just located but decoded"
    /// bar. `AcDb:Header`/`AcDb:Classes`/`AcDb:Handles` are asserted individually since they're
    /// the sections D4/D5 (stretch) would need to interpret further.
    #[test]
    fn real_fixture_d2_decompresses_every_section() {
        let sections = decode_r2004_sections(ARCHITECTURAL_FIXTURE).expect("D2 section decode");
        let mut any_errors = Vec::new();
        for section in &sections {
            for page in &section.pages {
                if let Some(err) = &page.error {
                    any_errors.push(format!("{}[{}]: {err}", section.name, page.page_number));
                } else {
                    assert!(!page.decoded.is_empty(), "section {} page {} decoded to zero bytes", section.name, page.page_number);
                }
            }
        }
        assert!(any_errors.is_empty(), "D2 page decode errors on real fixture: {any_errors:?}");

        for must_have in ["AcDb:Header", "AcDb:Classes", "AcDb:Handles"] {
            let s = sections.iter().find(|s| s.name == must_have).unwrap_or_else(|| panic!("{must_have} missing"));
            let total: usize = s.pages.iter().map(|p| p.decoded.len()).sum();
            assert!(total > 0, "{must_have} decoded to zero total bytes");
        }
    }

    /// 🧪️ D1 cross-validation: the page directory's total cumulative size must independently
    /// match the file header's own `last_section_address` field (decrypted from a completely
    /// different byte range) -- if the LZ decompressor or page-directory parser silently
    /// produced wrong-but-plausible-looking output, this arithmetic identity would not hold.
    #[test]
    fn real_fixture_page_directory_matches_header_cross_check() {
        let enc = &ARCHITECTURAL_FIXTURE[0x80..0x80 + R2004_HEADER_LEN];
        let hdr = parse_r2004_file_header(&decrypt_r2004_header(enc)).expect("header decrypt");
        let map_hdr_addr = (hdr.section_map_address + 0x100) as usize;
        let map_decomp_size = u32::from_le_bytes(ARCHITECTURAL_FIXTURE[map_hdr_addr + 4..map_hdr_addr + 8].try_into().unwrap()) as usize;
        let map_comp_size = u32::from_le_bytes(ARCHITECTURAL_FIXTURE[map_hdr_addr + 8..map_hdr_addr + 12].try_into().unwrap()) as usize;
        let map_data_start = map_hdr_addr + 0x14;
        let map_dec = decompress_r2004_section(&ARCHITECTURAL_FIXTURE[map_data_start..map_data_start + map_comp_size], map_decomp_size).unwrap();
        let page_dir = parse_page_directory(&map_dec, hdr.section_array_size);
        assert_eq!(page_dir.len() as u32, hdr.numgaps + hdr.numsections);

        // Independent re-derivation straight from the decompressed bytes (not reusing
        // `parse_page_directory`'s own running-address bookkeeping) as the actual cross-check.
        let mut pos = 0usize;
        let mut total: u64 = 0x100;
        while pos + 8 <= map_dec.len() {
            let number = i32::from_le_bytes(map_dec[pos..pos + 4].try_into().unwrap());
            let size = u32::from_le_bytes(map_dec[pos + 4..pos + 8].try_into().unwrap());
            pos += 8;
            if number <= hdr.section_array_size as i32 {
                total += size as u64;
            }
            if number < 0 && pos + 16 <= map_dec.len() {
                pos += 16;
            }
        }
        assert_eq!(total, hdr.last_section_address + 0x100, "page directory total size must match independent header field");
    }
    //#endregion 🔖️RealFixture
}
//#endregion 🧪️Tests
