//! 🗜️ R2004 and R2007 containers: file-header decryption, LZ77 and R2007 decompression, page and section directories.

use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::R2004_HEADER_LEN;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::decrypt_r2004_header;

/// 🗂️ Section metadata with page identifiers, sizes, and offsets.
pub type DwgSectionInfo = (String, u64, u32, u32, u32, u32, Vec<(i32, u32, u64)>);
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
pub fn decode_r2007_sections(bytes: &[u8]) -> Result<Vec<DwgRawSection>, String> {
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
//#endregion DwgRead
//#endregion 🔖️DwgStructuralCodec
