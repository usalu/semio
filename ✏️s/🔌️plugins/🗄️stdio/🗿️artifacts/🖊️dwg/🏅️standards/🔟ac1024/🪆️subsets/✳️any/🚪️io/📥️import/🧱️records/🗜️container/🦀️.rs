//! 🗜️ R2004 and R2007 containers: file-header decryption, LZ77 and R2007 decompression, page and section directories.

use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::R2004_HEADER_LEN;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::decrypt_r2004_header;

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
const R2004_COPY_WINDOW: usize = 4096;

#[derive(Clone, Copy)]
enum R2004Stage {
    FirstOpcode,
    Opcode(u8),
    LiteralLength { length: usize, initial: bool },
    Literal(usize),
    NextOpcode,
    MatchLength { opcode: u8, length: usize },
    Offset { opcode: u8, length: usize },
    WideOffset { opcode: u8, length: usize, first: u8 },
    Match { remaining: usize, distance: usize, following: u8 },
    Following(u8),
    Complete,
}

/// 🗜️ R2004 LZ77 state for one immutable physical payload.
struct R2004Decoder {
    stage: R2004Stage,
    input: usize,
    maximum: usize,
    output: Vec<u8>,
    error: Option<String>,
}

impl R2004Decoder {
    fn new(maximum: usize) -> Self {
        Self { stage: R2004Stage::FirstOpcode, input: 0, maximum, output: Vec::new(), error: None }
    }

    #[cfg(test)]
    fn input_offset(&self) -> usize { self.input }
    fn output(&self) -> &[u8] { &self.output }

    fn step(&mut self, payload: &[u8], budget: usize) -> Result<bool, String> {
        if let Some(error) = &self.error {
            return Err(error.clone());
        }
        if matches!(self.stage, R2004Stage::Complete) {
            return Ok(true);
        }
        if budget == 0 {
            return Ok(false);
        }
        match self.advance(payload, budget.min(R2004_COPY_WINDOW)) {
            Ok(stage) => {
                self.stage = stage;
                Ok(matches!(stage, R2004Stage::Complete))
            }
            Err(error) => {
                self.error = Some(error.clone());
                Err(error)
            }
        }
    }

    fn read(&mut self, payload: &[u8]) -> Result<u8, String> {
        let byte = *payload.get(self.input).ok_or("dwg lz: source exhausted mid-opcode")?;
        self.input += 1;
        Ok(byte)
    }

    fn literal_length(&self, length: usize, initial: bool) -> Result<R2004Stage, String> {
        if length > self.maximum - self.output.len() {
            if initial { return Err("dwg lz: initial literal exceeds the output extent".into()); }
            return Ok(R2004Stage::Complete);
        }
        Ok(R2004Stage::Literal(length))
    }

    fn literal_opcode(&self, opcode: u8, initial: bool) -> Result<R2004Stage, String> {
        match opcode {
            0 => Ok(R2004Stage::LiteralLength { length: 18, initial }),
            1..=15 => self.literal_length(usize::from(opcode) + 3, initial),
            _ => Ok(R2004Stage::Opcode(opcode)),
        }
    }

    fn back_reference(&self, length: usize, distance: usize, following: u8) -> Result<R2004Stage, String> {
        if distance > self.output.len() || length > self.maximum - self.output.len() {
            return Err(format!("dwg lz: invalid backref length={length} distance={distance} position={}", self.output.len()));
        }
        Ok(R2004Stage::Match { remaining: length, distance, following })
    }

    fn advance(&mut self, payload: &[u8], budget: usize) -> Result<R2004Stage, String> {
        use R2004Stage::*;
        match self.stage {
            FirstOpcode => {
                let opcode = self.read(payload)?;
                self.literal_opcode(opcode, true)
            }
            NextOpcode => Ok(Opcode(self.read(payload)?)),
            Following(opcode) => {
                let trailing = usize::from(opcode & 3);
                if trailing != 0 { return self.literal_length(trailing, false); }
                let next = self.read(payload)?;
                self.literal_opcode(next, false)
            }
            LiteralLength { length, initial } => {
                let byte = self.read(payload)?;
                if byte == 0 && self.input < payload.len() {
                    Ok(LiteralLength { length: length.checked_add(255).ok_or("dwg lz: literal length overflow")?, initial })
                } else {
                    self.literal_length(length.checked_add(usize::from(byte)).ok_or("dwg lz: literal length overflow")?, initial)
                }
            }
            Literal(remaining) => {
                let count = remaining.min(budget);
                let bytes = payload.get(self.input..).and_then(|bytes| bytes.get(..count)).ok_or("dwg lz: truncated literal")?;
                self.output.try_reserve(count).map_err(|_| "dwg lz: output allocation failed")?;
                self.output.extend_from_slice(bytes);
                self.input += count;
                Ok(if count == remaining { NextOpcode } else { Literal(remaining - count) })
            }
            Opcode(opcode) => {
                if opcode == 0x11 || self.input == payload.len() || self.output.len() == self.maximum {
                    return Ok(Complete);
                }
                if opcode >= 0x40 {
                    return Ok(Offset { opcode, length: usize::from(opcode >> 4) - 1 });
                }
                if opcode < 0x10 {
                    return Err(format!("dwg lz: invalid match opcode {opcode:#x}"));
                }
                let mask = if opcode < 0x20 { 7 } else { 0x1f };
                let count = opcode & mask;
                if count == 0 { Ok(MatchLength { opcode, length: usize::from(mask) + 2 }) }
                else { Ok(Offset { opcode, length: usize::from(count) + 2 }) }
            }
            MatchLength { opcode, length } => {
                let byte = self.read(payload)?;
                let extended = byte == 0 && self.input < payload.len();
                let length = length.checked_add(if extended { 255 } else { usize::from(byte) }).ok_or("dwg lz: match length overflow")?;
                Ok(if extended { MatchLength { opcode, length } } else { Offset { opcode, length } })
            }
            Offset { opcode, length } => {
                let first = self.read(payload)?;
                if opcode < 0x40 { return Ok(WideOffset { opcode, length, first }); }
                let distance = (((usize::from(opcode) >> 2) & 3) | (usize::from(first) << 2)) + 1;
                self.back_reference(length, distance, opcode)
            }
            WideOffset { opcode, length, first } => {
                let second = self.read(payload)?;
                let offset = (usize::from(first) >> 2) | (usize::from(second) << 6);
                let distance = if opcode < 0x20 { (offset | ((usize::from(opcode) & 8) << 11)) + 0x4000 } else { offset + 1 };
                self.back_reference(length, distance, first)
            }
            Match { remaining, distance, following } => {
                let count = remaining.min(budget);
                self.output.try_reserve(count).map_err(|_| "dwg lz: output allocation failed")?;
                for _ in 0..count {
                    self.output.push(self.output[self.output.len() - distance]);
                }
                Ok(if count == remaining { Following(following) } else { Match { remaining: remaining - count, distance, following } })
            }
            Complete => Ok(Complete),
        }
    }
}

/// 🧱 Decompresses one R2004 stream with the physical page decoder.
pub fn decompress_r2004_section(comp: &[u8], decomp_size: usize) -> Result<Vec<u8>, String> {
    let mut decoder = R2004Decoder::new(decomp_size);
    while !decoder.step(comp, R2004_COPY_WINDOW)? {}
    Ok(decoder.output)
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
    let mut rows = R2004PageDirectoryRows::new();
    std::iter::from_fn(|| rows.next(dec, section_array_size)).collect()
}

struct R2004PageDirectoryRows {
    position: usize,
    address: u64,
}

impl R2004PageDirectoryRows {
    fn new() -> Self { Self { position: 0, address: 0x100 } }

    fn next(&mut self, data: &[u8], section_array_size: u32) -> Option<PageDirEntry> {
        let row = data.get(self.position..)?.first_chunk::<8>()?;
        let number = i32::from_le_bytes(row[..4].try_into().unwrap());
        let size = u32::from_le_bytes(row[4..].try_into().unwrap());
        self.position += 8;
        let entry = PageDirEntry { number, address: self.address };
        if number <= section_array_size as i32 {
            self.address = self.address.saturating_add(u64::from(size));
        }
        if number < 0 && data.len() - self.position >= 16 {
            self.position += 16;
        }
        Some(entry)
    }
}
//#region 🧱️R2004DirectoryDecode
/// 🧰 Transient page materialization used only while decoding named standard concepts.
#[derive(Debug, Clone, Default)]
pub struct DwgRawPage {
    pub page_number: i32,
    pub file_address: u64,
    pub start_offset: u64,
    /// 📥️ Empty iff this page's decompression failed; never retained by an artifact snapshot.
    pub decoded: Vec<u8>,
    pub error: Option<String>,
    r2007: Option<R2007PageLayout>,
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
struct R2004SectionRows {
    section: DwgRawSection,
    rows: std::ops::Range<usize>,
}

fn r2004_descriptor(data: &[u8], at: usize) -> Result<R2004SectionRows, String> {
    let rest = data.get(at..).ok_or("section info: descriptor begins outside the directory")?;
    let row = rest.first_chunk::<96>().ok_or("section info: descriptor is truncated")?;
    let word = |offset| u32::from_le_bytes(row[offset..offset + 4].try_into().unwrap());
    let count = word(8) as usize;
    if count > (rest.len() - row.len()) / 16 {
        return Err("section info: page roster exceeds decoded length".into());
    }
    let name = &row[32..];
    let length = name.iter().position(|&byte| byte == 0).unwrap_or(name.len());
    let section = DwgRawSection {
        name: String::from_utf8_lossy(&name[..length]).into_owned(),
        declared_size: u64::from_le_bytes(row[..8].try_into().unwrap()),
        compressed: word(20) == 2, max_decomp_size: word(12), pages: Vec::new(),
    };
    let first = at + row.len();
    Ok(R2004SectionRows { section, rows: first..first + count * 16 })
}

fn r2004_section_page_row(row: &[u8; 16]) -> (i32, u32, u64) {
    (i32::from_le_bytes(row[..4].try_into().unwrap()), u32::from_le_bytes(row[4..8].try_into().unwrap()), u64::from_le_bytes(row[8..].try_into().unwrap()))
}

fn r2004_directory_payload(bytes: &[u8], address: u64, signature: u32) -> Result<(&[u8], usize), String> {
    let address = usize::try_from(address).map_err(|_| "r2004: directory page address is too large")?;
    let page = bytes.get(address..).filter(|page| page.len() >= 20).ok_or_else(|| format!("r2004: directory header at {address:#x} is truncated"))?;
    let word = |offset| u32::from_le_bytes(page[offset..offset + 4].try_into().unwrap());
    let actual = word(0);
    if actual != signature {
        return Err(format!("r2004: directory signature mismatch: {actual:#x} != {signature:#x}"));
    }
    let payload = page[20..].get(..word(8) as usize).ok_or("r2004: directory payload is truncated")?;
    Ok((payload, word(4) as usize))
}

struct R2004DirectoryPage {
    address: u64,
    signature: u32,
    input_length: usize,
    decoder: R2004Decoder,
}

impl R2004DirectoryPage {
    fn open(file: &[u8], address: u64, signature: u32) -> Result<Self, String> {
        let (payload, bound) = r2004_directory_payload(file, address, signature)?;
        Ok(Self { address, signature, input_length: payload.len(), decoder: R2004Decoder::new(bound) })
    }

    fn fraction(&self) -> f32 { section_fraction(self.decoder.input, self.input_length) }

    fn step(&mut self, file: &[u8]) -> Result<Option<Vec<u8>>, String> {
        let (payload, _) = r2004_directory_payload(file, self.address, self.signature)?;
        if self.decoder.step(payload, R2004_COPY_WINDOW)? { Ok(Some(std::mem::take(&mut self.decoder.output))) }
        else { Ok(None) }
    }
}

enum R2004DirectoryStage {
    PageMap(R2004DirectoryPage),
    PageRows { data: Vec<u8>, rows: R2004PageDirectoryRows, count: u64 },
    SectionMap(R2004DirectoryPage),
    SectionRows { data: Vec<u8>, position: usize, remaining: usize, selected: Option<R2004SectionRows> },
    Complete,
}

struct R2004Directory {
    header: R2004FileHeader,
    select: fn(&str) -> bool,
    stage: R2004DirectoryStage,
    addresses: std::collections::HashMap<i32, u64>,
    sections: Vec<DwgRawSection>,
}

impl R2004Directory {
    fn open(bytes: &[u8], select: fn(&str) -> bool) -> Result<Self, String> {
        let header = bytes.get(0x80..0x80 + R2004_HEADER_LEN).ok_or("r2004: file is too short for its encrypted header")?;
        let header = parse_r2004_file_header(&decrypt_r2004_header(header))?;
        let address = header.section_map_address.checked_add(0x100).ok_or("r2004: section page map address overflows")?;
        let page = R2004DirectoryPage::open(bytes, address, 0x41630e3b)?;
        Ok(Self { header, select, stage: R2004DirectoryStage::PageMap(page), addresses: Default::default(), sections: Vec::new() })
    }

    fn fraction(&self) -> f32 {
        match &self.stage {
            R2004DirectoryStage::PageMap(page) => page.fraction() / 4.0,
            R2004DirectoryStage::PageRows { count, .. } => {
                let total = u64::from(self.header.numgaps) + u64::from(self.header.numsections);
                (1.0 + (*count as f32 / total.max(1) as f32).min(1.0)) / 4.0
            }
            R2004DirectoryStage::SectionMap(page) => (2.0 + page.fraction()) / 4.0,
            R2004DirectoryStage::SectionRows { data, position, selected, .. } => {
                let at = selected.as_ref().map_or(*position, |section| section.rows.start);
                (3.0 + section_fraction(at, data.len())) / 4.0
            }
            R2004DirectoryStage::Complete => 1.0,
        }
    }

    fn step(&mut self, file: &[u8]) -> Result<Option<Vec<DwgRawSection>>, String> {
        use R2004DirectoryStage::*;
        match &mut self.stage {
            PageMap(page) => {
                if let Some(data) = page.step(file)? {
                    self.stage = PageRows { data, rows: R2004PageDirectoryRows::new(), count: 0 };
                }
            }
            PageRows { data, rows, count } => {
                if let Some(entry) = rows.next(data, self.header.section_array_size) {
                    self.addresses.insert(entry.number, entry.address);
                    *count += 1;
                } else {
                    let expected = u64::from(self.header.numgaps) + u64::from(self.header.numsections);
                    if *count != expected { return Err(format!("r2004: page directory entry count {count} differs from {expected}")); }
                    let address = *self.addresses.get(&self.header.section_info_id).ok_or("r2004: section info not found in the page directory")?;
                    self.stage = SectionMap(R2004DirectoryPage::open(file, address, 0x4163003b)?);
                }
            }
            SectionMap(page) => {
                if let Some(data) = page.step(file)? {
                    if data.len() < 20 { return Err("section info: header shorter than 20 bytes".into()); }
                    let remaining = u32::from_le_bytes(data[..4].try_into().unwrap()) as usize;
                    if remaining > (data.len() - 20) / 96 { return Err("section info: descriptor roster exceeds decoded length".into()); }
                    self.stage = SectionRows { data, position: 20, remaining, selected: None };
                }
            }
            SectionRows { data, position, remaining, selected } => {
                if let Some(descriptor) = selected {
                    if descriptor.rows.is_empty() {
                        self.sections.push(selected.take().unwrap().section);
                    } else {
                        let start = descriptor.rows.start;
                        let (page_number, _, start_offset) = r2004_section_page_row(data[start..start + 16].try_into().unwrap());
                        let file_address = *self.addresses.get(&page_number).ok_or_else(|| format!("r2004: page {page_number} for section {} not in page directory", descriptor.section.name))?;
                        descriptor.section.pages.push(DwgRawPage { page_number, file_address, start_offset, ..Default::default() });
                        descriptor.rows.start += 16;
                    }
                } else if *remaining != 0 {
                    let descriptor = r2004_descriptor(data, *position)?;
                    *position = descriptor.rows.end;
                    *remaining -= 1;
                    if !descriptor.section.name.is_empty() && (self.select)(&descriptor.section.name) {
                        *selected = Some(descriptor);
                    }
                } else {
                    self.stage = Complete;
                    return Ok(Some(std::mem::take(&mut self.sections)));
                }
            }
            Complete => return Err("r2004: directory was already consumed".into()),
        }
        Ok(None)
    }
}

/// 🗺️ Resolves every physical section page using the same cursor as the drawing reader.
pub fn locate_r2004_sections(bytes: &[u8]) -> Result<Vec<DwgRawSection>, String> {
    let mut directory = R2004Directory::open(bytes, |_| true)?;
    loop {
        if let Some(sections) = directory.step(bytes)? { return Ok(sections); }
    }
}

/// 🗜️ D2: for every section D1 located, decrypts + decompresses (or, for `compressed == false`
/// sections, copies verbatim) each page's real content bytes. A single page's failure is
/// recorded on that page (`DwgRawPage::error`) and does not abort the other pages/sections --
/// the caller can tell exactly how much of D2 landed from the per-page `error` fields.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_r2004_sections(bytes: &[u8]) -> Result<Vec<DwgRawSection>, String> {
    Ok(decode_located_sections(bytes, locate_r2004_sections(bytes)?))
}

fn decode_located_sections(bytes: &[u8], mut sections: Vec<DwgRawSection>) -> Vec<DwgRawSection> {
    for section in &mut sections {
        for page in &mut section.pages {
            match read_section_page(bytes, page, section.compressed, section.max_decomp_size) {
                Ok(decoded) => page.decoded = decoded,
                Err(error) => page.error = Some(error),
            }
        }
    }
    sections
}

fn read_section_page(bytes: &[u8], page: &DwgRawPage, compressed: bool, max_decomp_size: u32) -> Result<Vec<u8>, String> {
    let address = usize::try_from(page.file_address).map_err(|_| "DWG page address is too large")?;
    let data = bytes.get(address..).ok_or("DWG page address is outside the file")?;
    if let Some(layout) = page.r2007 {
        if layout.stored {
            return data.get(..layout.size).map(<[u8]>::to_vec).ok_or_else(|| "R2007 stored page is truncated".into());
        }
        return r2007_page(bytes, address, layout.compressed, layout.size, 1, 251);
    }
    let (payload, bound) = r2004_data_payload(data, page.file_address, max_decomp_size)?;
    if compressed { decompress_r2004_section(payload, bound) } else { Ok(payload.to_vec()) }
}

fn r2004_data_payload(data: &[u8], file_address: u64, max_decomp_size: u32) -> Result<(&[u8], usize), String> {
    let header = data.first_chunk::<32>().ok_or("R2004 page header is truncated")?;
    let header = decrypt_page_header(header, file_address);
    if header.page_type != 0x4163043b {
        return Err(format!("R2004 page type {:#x} != 0x4163043b", header.page_type));
    }
    let payload = data[32..].get(..header.data_size as usize).ok_or("R2004 page payload is truncated")?;
    Ok((payload, max_decomp_size.max(header.page_size).max(1) as usize))
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
    let mut reader = R2007Deinterleave::new(length, length, factor, block_size)?;
    while !reader.step(encoded)? {}
    Ok(reader.output)
}

struct R2007Deinterleave {
    length: usize,
    keep: usize,
    factor: usize,
    block_size: usize,
    position: usize,
    output: Vec<u8>,
}

impl R2007Deinterleave {
    fn new(length: usize, keep: usize, factor: usize, block_size: usize) -> Result<Self, String> {
        if block_size == 0 || keep > length || factor.checked_mul(block_size).is_none_or(|size| size < length) {
            return Err("R2007 Reed-Solomon blocks are shorter than the declared data".into());
        }
        Ok(Self { length, keep, factor, block_size, position: 0, output: Vec::new() })
    }

    fn step(&mut self, encoded: &[u8]) -> Result<bool, String> {
        let count = (self.length - self.position).min(R2007_COPY_WINDOW);
        let end = self.position + count;
        let keep = end.min(self.keep);
        self.output.try_reserve(keep.saturating_sub(self.position)).map_err(|_| "R2007 deinterleaved allocation failed")?;
        while self.position < end {
            let block = self.position / self.block_size;
            let index = self.position % self.block_size;
            let byte = *encoded.get(block + index * self.factor).ok_or("R2007 Reed-Solomon block is truncated")?;
            if self.position < keep { self.output.push(byte); }
            self.position += 1;
        }
        Ok(self.position == self.length)
    }
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
const R2007_COPY_WINDOW: usize = 4096;

fn r2007_literal_block(chunk: &[u8]) -> [u8; 32] {
    let mut output = [0; 32];
    let layout = if chunk.len() == 32 { &[(24, 0, 8), (16, 8, 8), (8, 16, 8), (0, 24, 8)][..] }
    else { R2007_LITERAL_LAYOUT[chunk.len()] };
    for &(from, to, width) in layout {
        let segment = &chunk[from..from + width];
        let target = &mut output[to..to + width];
        match width {
            2 | 3 => target.iter_mut().zip(segment.iter().rev()).for_each(|(slot, byte)| *slot = *byte),
            16 => {
                target[..8].copy_from_slice(&segment[8..]);
                target[8..].copy_from_slice(&segment[..8]);
            }
            _ => target.copy_from_slice(segment),
        }
    }
    output
}

#[derive(Clone, Copy)]
enum R2007Stage {
    First,
    Run { opcode: usize, literal: usize },
    ExtendedLiteral,
    WideLiteral(usize),
    Literal(usize),
    AfterLiteral,
    Match(usize),
    Copy { remaining: usize, distance: usize, next: usize },
    Following(usize),
    Padding,
    Complete,
}

/// 🐚 Retains one AC21 compressed stream without materializing overlong output tails.
struct R2007Decoder {
    stage: R2007Stage,
    input: usize,
    logical_output: usize,
    expected: usize,
    output: Vec<u8>,
    error: Option<String>,
}

impl R2007Decoder {
    fn new(expected: usize) -> Self {
        Self { stage: R2007Stage::First, input: 0, logical_output: 0, expected, output: Vec::new(), error: None }
    }

    fn step(&mut self, source: &[u8]) -> Result<bool, String> {
        if let Some(error) = &self.error { return Err(error.clone()); }
        match self.advance(source) {
            Ok(stage) => {
                self.stage = stage;
                Ok(matches!(stage, R2007Stage::Complete))
            }
            Err(error) => {
                self.error = Some(error.clone());
                Err(error)
            }
        }
    }

    fn byte(&mut self, source: &[u8]) -> Result<usize, String> {
        let value = *source.get(self.input).ok_or("R2007 compressed stream underflow")?;
        self.input += 1;
        Ok(usize::from(value))
    }

    fn back_reference(&mut self, source: &[u8], opcode: usize) -> Result<R2007Stage, String> {
        let (remaining, distance, next) = match opcode >> 4 {
            0 => {
                let low = self.byte(source)?;
                let next = self.byte(source)?;
                ((opcode & 0x0f) + 0x13 + ((next >> 3) & 0x10), ((next & 0x78) << 5) + 1 + low, next)
            }
            1 => {
                let low = self.byte(source)?;
                let next = self.byte(source)?;
                ((opcode & 0x0f) + 3, ((next & 0xf8) << 5) + 1 + low, next)
            }
            2 => {
                let offset = self.byte(source)? | (self.byte(source)? << 8);
                if opcode & 8 == 0 {
                    let next = self.byte(source)?;
                    ((next & 0xf8) + (opcode & 7), offset, next)
                } else {
                    let high = self.byte(source)?;
                    let next = self.byte(source)?;
                    (((next & 0xf8) << 8) + (high << 3) + (opcode & 7) + 0x100, offset + 1, next)
                }
            }
            nibble => {
                let next = self.byte(source)?;
                (nibble, ((next & 0xf8) << 1) + (opcode & 0x0f) + 1, next)
            }
        };
        if distance == 0 && remaining > 0 { return Err("R2007 back-reference does not precede its output".into()); }
        if distance > self.logical_output { return Err("R2007 back-reference precedes the page".into()); }
        Ok(R2007Stage::Copy { remaining, distance, next })
    }

    fn retain(&mut self, count: usize) -> Result<usize, String> {
        self.logical_output = self.logical_output.checked_add(count).ok_or("R2007 logical output length overflows")?;
        let kept = count.min(self.expected - self.output.len());
        self.output.try_reserve(kept).map_err(|_| "R2007 output allocation failed")?;
        Ok(kept)
    }

    fn advance(&mut self, source: &[u8]) -> Result<R2007Stage, String> {
        use R2007Stage::*;
        match self.stage {
            First => {
                let opcode = self.byte(source)?;
                let literal = if opcode & 0xf0 == 0x20 {
                    let tail = source.get(self.input..).and_then(|source| source.first_chunk::<3>()).ok_or("R2007 compressed stream underflow")?;
                    self.input += 3;
                    usize::from(tail[2] & 7)
                } else { 0 };
                Ok(Run { opcode, literal })
            }
            Run { opcode, literal } => {
                if self.input >= source.len() { return Ok(Padding); }
                if literal != 0 { return Ok(Literal(literal)); }
                let length = opcode + 8;
                Ok(if length == 0x17 { ExtendedLiteral } else { Literal(length) })
            }
            ExtendedLiteral => {
                let extra = self.byte(source)?;
                Ok(if extra == 0xff { WideLiteral(0x17 + extra) } else { Literal(0x17 + extra) })
            }
            WideLiteral(length) => {
                let chunk = self.byte(source)? | (self.byte(source)? << 8);
                let length = length.checked_add(chunk).ok_or("R2007 literal length overflows")?;
                Ok(if chunk == 0xffff { WideLiteral(length) } else { Literal(length) })
            }
            Literal(remaining) => {
                let count = remaining.min(R2007_COPY_WINDOW);
                let bytes = source.get(self.input..).and_then(|bytes| bytes.get(..count)).ok_or("R2007 literal run exceeds its page")?;
                let kept = self.retain(count)?;
                for chunk in bytes.chunks(32).take(kept.div_ceil(32)) {
                    let width = chunk.len().min(self.expected - self.output.len());
                    self.output.extend_from_slice(&r2007_literal_block(chunk)[..width]);
                }
                self.input += count;
                Ok(if count == remaining { AfterLiteral } else { Literal(remaining - count) })
            }
            AfterLiteral => if self.input >= source.len() { Ok(Padding) } else { Ok(Match(self.byte(source)?)) },
            Match(opcode) => self.back_reference(source, opcode),
            Copy { remaining, distance, next } => {
                let count = remaining.min(R2007_COPY_WINDOW);
                let kept = self.retain(count)?;
                for _ in 0..kept { self.output.push(self.output[self.output.len() - distance]); }
                Ok(if count == remaining { Following(next) } else { Copy { remaining: remaining - count, distance, next } })
            }
            Following(next) => {
                if self.input >= source.len() { return Ok(Padding); }
                let literal = next & 7;
                if literal != 0 { return Ok(Run { opcode: next, literal }); }
                let opcode = self.byte(source)?;
                Ok(match opcode >> 4 {
                    0 => Run { opcode, literal: 0 },
                    15 => Match(opcode & 0x0f),
                    _ => Match(opcode),
                })
            }
            Padding => {
                let count = (self.expected - self.output.len()).min(R2007_COPY_WINDOW);
                self.output.try_reserve(count).map_err(|_| "R2007 output allocation failed")?;
                self.output.resize(self.output.len() + count, 0);
                Ok(if self.output.len() == self.expected { Complete } else { Padding })
            }
            Complete => Ok(Complete),
        }
    }
}

/// 🗜️ Restores a complete AC21 payload with the bounded page decoder.
fn decompress_r2007(source: &[u8], expected: usize) -> Result<Vec<u8>, String> {
    let mut decoder = R2007Decoder::new(expected);
    while !decoder.step(source)? {}
    Ok(decoder.output)
}
/// 📄️ One R2007 page: `factor` Reed–Solomon blocks of `block_size` data bytes at `address`, holding
/// `compressed` bytes that decompress to `size` (stored as-is when the two are equal).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn r2007_page(bytes: &[u8], address: usize, compressed: usize, size: usize, correction: usize, block_size: usize) -> Result<Vec<u8>, String> {
    let mut reader = R2007PageReader::new(address, compressed, size, correction, block_size)?;
    while !reader.step(bytes)? {}
    let R2007PageStage::Complete(output) = reader.stage else { unreachable!("completed AC21 page owns its bytes") };
    Ok(output)
}

enum R2007PageStage {
    Deinterleave(R2007Deinterleave),
    Decode { bytes: Vec<u8>, decoder: R2007Decoder },
    Complete(Vec<u8>),
}

struct R2007PageReader {
    address: usize,
    encoded_length: usize,
    compressed: usize,
    size: usize,
    stage: R2007PageStage,
}

impl R2007PageReader {
    fn new(address: usize, compressed: usize, size: usize, correction: usize, block_size: usize) -> Result<Self, String> {
        let aligned = compressed.checked_next_multiple_of(8).ok_or("R2007 compressed size overflows")?;
        let total = aligned.checked_mul(correction).filter(|total| *total > 0 && *total < 1 << 28).ok_or("R2007 page size is invalid")?;
        if block_size == 0 { return Err("R2007 page block size is invalid".into()); }
        let factor = total.div_ceil(block_size);
        let encoded_length = factor.checked_mul(255).ok_or("R2007 encoded page size overflows")?;
        let stage = R2007PageStage::Deinterleave(R2007Deinterleave::new(total, compressed, factor, block_size)?);
        Ok(Self { address, encoded_length, compressed, size, stage })
    }

    fn step(&mut self, file: &[u8]) -> Result<bool, String> {
        match &mut self.stage {
            R2007PageStage::Deinterleave(reader) => {
                let encoded = file.get(self.address..).ok_or("R2007 page address is out of bounds")?;
                if reader.step(&encoded[..encoded.len().min(self.encoded_length)])? {
                    let bytes = std::mem::take(&mut reader.output);
                    self.stage = if self.compressed == self.size { R2007PageStage::Complete(bytes) }
                    else { R2007PageStage::Decode { bytes, decoder: R2007Decoder::new(self.size) } };
                }
            }
            R2007PageStage::Decode { bytes, decoder } => {
                if decoder.step(bytes)? { self.stage = R2007PageStage::Complete(std::mem::take(&mut decoder.output)); }
            }
            R2007PageStage::Complete(_) => {}
        }
        Ok(matches!(self.stage, R2007PageStage::Complete(_)))
    }

    fn fraction(&self) -> f32 {
        match &self.stage {
            R2007PageStage::Deinterleave(reader) => section_fraction(reader.position, reader.length) * 0.5,
            R2007PageStage::Decode { decoder, .. } => 0.5 + 0.5 * section_fraction(decoder.output.len(), decoder.expected),
            R2007PageStage::Complete(_) => 1.0,
        }
    }
}
/// 🗂️ The named sections of an R2007 (AC1021) file: the Reed–Solomon file header yields the page
/// map and section map, whose pages resolve every section page. ODA §5.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_r2007_sections(bytes: &[u8]) -> Result<Vec<DwgRawSection>, String> {
    Ok(decode_located_sections(bytes, locate_r2007_selected_sections(bytes, |_| true)?))
}

#[derive(Debug, Clone, Copy)]
struct R2007PageLayout {
    stored: bool,
    size: usize,
    compressed: usize,
}

fn locate_r2007_selected_sections(bytes: &[u8], select: fn(&str) -> bool) -> Result<Vec<DwgRawSection>, String> {
    let mut directory = R2007Directory::open(bytes, select)?;
    loop {
        if let Some(sections) = directory.step(bytes)? { return Ok(sections); }
    }
}

fn r2007_u64(data: &[u8], offset: usize) -> Result<u64, String> {
    data.get(offset..).and_then(|data| data.first_chunk::<8>()).copied().map(u64::from_le_bytes)
        .ok_or_else(|| format!("R2007 field at {offset:#x} is truncated"))
}

fn r2007_host_size(value: u64) -> Result<usize, String> {
    usize::try_from(value).map_err(|_| "R2007 size exceeds the address space".into())
}

fn r2007_file_address(relative: usize) -> Result<usize, String> {
    R2007_DATA_PAGES_START.checked_add(relative).ok_or_else(|| "R2007 page address overflows".into())
}

struct R2007SectionRows {
    section: DwgRawSection,
    name: std::ops::Range<usize>,
    trim: usize,
    rows: std::ops::Range<usize>,
    encoding: u64,
}

impl R2007SectionRows {
    fn open(data: &[u8], position: usize) -> Result<Self, String> {
        let header = &data[position..position + 64];
        let name_length = r2007_host_size(r2007_u64(header, 32)?)?;
        let page_count = r2007_host_size(r2007_u64(header, 56)?)?;
        let encoding = r2007_u64(header, 48)?;
        let name_start = position + 64;
        let name_end = name_start.checked_add(name_length).filter(|end| *end <= data.len()).ok_or("R2007 section name is truncated")?;
        let rows_end = page_count.checked_mul(56).and_then(|bytes| name_end.checked_add(bytes)).filter(|end| *end <= data.len()).ok_or("R2007 section page roster is truncated")?;
        let section = DwgRawSection { name: String::new(), compressed: encoding == 4, declared_size: r2007_u64(header, 0)?, max_decomp_size: r2007_u64(header, 8)? as u32, pages: Vec::new() };
        Ok(Self { section, name: name_start..name_end - name_length % 2, trim: 0, rows: name_end..rows_end, encoding })
    }

    fn name_step(&mut self, bytes: &[u8]) -> Result<bool, String> {
        let mut count = self.name.len().min(R2007_COPY_WINDOW / 3 * 2);
        if count == 0 { return Ok(true); }
        if count < self.name.len() {
            let last = self.name.start + count - 2;
            let word = u16::from_le_bytes(bytes[last..last + 2].try_into().unwrap());
            if (0xd800..=0xdbff).contains(&word) { count -= 2; }
        }
        let chunk = &bytes[self.name.start..self.name.start + count];
        let words = chunk.as_chunks::<2>().0.iter().copied().map(u16::from_le_bytes);
        self.section.name.try_reserve(count / 2 * 3).map_err(|_| "R2007 section name allocation failed")?;
        for scalar in char::decode_utf16(words) {
            let scalar = scalar.unwrap_or(char::REPLACEMENT_CHARACTER);
            self.section.name.push(scalar);
            if scalar == '\0' { self.trim += 1; } else { self.trim = 0; }
        }
        self.name.start += count;
        if self.name.is_empty() { self.section.name.truncate(self.section.name.len() - self.trim); }
        Ok(self.name.is_empty())
    }

    fn page_step(&mut self, data: &[u8], pages: &std::collections::HashMap<u64, usize>) -> Result<(), String> {
        let record = &data[self.rows.start..self.rows.start + 56];
        let start_offset = r2007_u64(record, 0)?;
        let page_number = r2007_u64(record, 16)?;
        let size = r2007_host_size(r2007_u64(record, 24)?)?;
        let compressed = r2007_host_size(r2007_u64(record, 32)?)?;
        let address = *pages.get(&page_number).ok_or_else(|| format!("R2007 page {page_number} of {} is missing", self.section.name))?;
        let layout = R2007PageLayout { stored: self.encoding == 1, size, compressed };
        self.section.pages.push(DwgRawPage { page_number: page_number as i32, file_address: r2007_file_address(address)? as u64, start_offset, r2007: Some(layout), ..Default::default() });
        self.rows.start += 56;
        Ok(())
    }
}

enum R2007DirectoryStage {
    Header,
    HeaderDecode { source: Vec<u8>, decoder: R2007Decoder },
    PageMap(R2007PageReader),
    PageRows { data: Vec<u8>, position: usize, address: usize },
    SectionMap(R2007PageReader),
    SectionRows { data: Vec<u8>, position: usize, pending: Option<R2007SectionRows> },
    Complete,
}

struct R2007Directory {
    select: fn(&str) -> bool,
    stage: R2007DirectoryStage,
    metadata: Vec<u8>,
    pages: std::collections::HashMap<u64, usize>,
    sections: Vec<DwgRawSection>,
}

impl R2007Directory {
    fn open(file: &[u8], select: fn(&str) -> bool) -> Result<Self, String> {
        if file.len() < R2007_DATA_PAGES_START { return Err("R2007 file header is truncated".into()); }
        Ok(Self { select, stage: R2007DirectoryStage::Header, metadata: Vec::new(), pages: Default::default(), sections: Vec::new() })
    }

    fn page_map(&mut self, metadata: Vec<u8>) -> Result<R2007DirectoryStage, String> {
        let field = |index: usize| r2007_host_size(r2007_u64(&metadata, index * 8)?);
        let page = R2007PageReader::new(r2007_file_address(field(7)?)?, field(10)?, field(11)?, field(3)?, 239)?;
        self.metadata = metadata;
        Ok(R2007DirectoryStage::PageMap(page))
    }

    fn section_map(&self) -> Result<R2007DirectoryStage, String> {
        let field = |index: usize| r2007_host_size(r2007_u64(&self.metadata, index * 8)?);
        let address = *self.pages.get(&r2007_u64(&self.metadata, 24 * 8)?).ok_or("R2007 section map page is missing")?;
        Ok(R2007DirectoryStage::SectionMap(R2007PageReader::new(r2007_file_address(address)?, field(22)?, field(25)?, field(27)?, 239)?))
    }

    fn fraction(&self) -> f32 {
        let value = match &self.stage {
            R2007DirectoryStage::Header => 0.0,
            R2007DirectoryStage::HeaderDecode { decoder, .. } => section_fraction(decoder.output.len(), decoder.expected),
            R2007DirectoryStage::PageMap(reader) => 1.0 + reader.fraction(),
            R2007DirectoryStage::PageRows { data, position, .. } => 2.0 + section_fraction(*position, data.len()),
            R2007DirectoryStage::SectionMap(reader) => 3.0 + reader.fraction(),
            R2007DirectoryStage::SectionRows { data, position, pending } => {
                let at = pending.as_ref().map_or(*position, |rows| if rows.name.is_empty() { rows.rows.start } else { rows.name.start });
                4.0 + section_fraction(at, data.len())
            }
            R2007DirectoryStage::Complete => 5.0,
        };
        value / 5.0
    }

    fn step(&mut self, file: &[u8]) -> Result<Option<Vec<DwgRawSection>>, String> {
        use R2007DirectoryStage::*;
        match &mut self.stage {
            Header => {
                let header = r2007_deinterleave(&file[0x80..0x480], 3 * 239, 3, 239)?;
                let length = i32::from_le_bytes(header[24..28].try_into().unwrap());
                let payload = &header[32..];
                self.stage = if length < 0 { self.page_map(payload[..0x110].to_vec())? }
                else { HeaderDecode { source: payload.get(..length as usize).ok_or("R2007 header metadata is truncated")?.to_vec(), decoder: R2007Decoder::new(0x110) } };
            }
            HeaderDecode { source, decoder } => {
                if decoder.step(source)? {
                    let metadata = std::mem::take(&mut decoder.output);
                    self.stage = self.page_map(metadata)?;
                }
            }
            PageMap(reader) => {
                if reader.step(file)? {
                    let R2007PageStage::Complete(data) = &mut reader.stage else { unreachable!() };
                    self.stage = PageRows { data: std::mem::take(data), position: 0, address: 0 };
                }
            }
            PageRows { data, position, address } => {
                let next = data[*position..].first_chunk::<16>().map(|row| (i64::from_le_bytes(row[..8].try_into().unwrap()), i64::from_le_bytes(row[8..].try_into().unwrap())));
                if let Some((size, id)) = next.filter(|pair| *pair != (0, 0)) {
                    self.pages.insert(id.unsigned_abs(), *address);
                    *address = address.checked_add(usize::try_from(size).map_err(|_| "R2007 page size is invalid")?).ok_or("R2007 page directory address overflows")?;
                    *position += 16;
                } else { self.stage = self.section_map()?; }
            }
            SectionMap(reader) => {
                if reader.step(file)? {
                    let R2007PageStage::Complete(data) = &mut reader.stage else { unreachable!() };
                    self.stage = SectionRows { data: std::mem::take(data), position: 0, pending: None };
                }
            }
            SectionRows { data, position, pending } => {
                if let Some(rows) = pending {
                    if !rows.name.is_empty() {
                        rows.name_step(data)?;
                    } else if rows.section.name.is_empty() || !(self.select)(&rows.section.name) {
                        *pending = None;
                    } else if rows.rows.is_empty() {
                        self.sections.push(pending.take().unwrap().section);
                    } else {
                        rows.page_step(data, &self.pages)?;
                    }
                } else if data.len() - *position >= 64 {
                    let rows = R2007SectionRows::open(data, *position)?;
                    *position = rows.rows.end;
                    *pending = Some(rows);
                } else {
                    self.stage = Complete;
                    return Ok(Some(std::mem::take(&mut self.sections)));
                }
            }
            Complete => return Err("R2007 directory was already consumed".into()),
        }
        Ok(None)
    }
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
/// 🧭 Progress through the two DWG sections used by drawing projection.
pub enum DwgDrawingSectionStep {
    Directory,
    PageData,
    Page,
    Padding,
    Complete { handles: Vec<u8>, objects: Vec<u8> },
}

#[derive(Default)]
struct R2004DrawingPage {
    decoder: Option<R2004Decoder>,
    copied: usize,
    fraction: f32,
}

impl R2004DrawingPage {
    fn step(&mut self, file: &[u8], page: &DwgRawPage, section: &DwgRawSection, output: &mut Vec<u8>, start: usize, length_limit: usize) -> Result<bool, String> {
        let address = usize::try_from(page.file_address).map_err(|_| "DWG page address is too large")?;
        let data = file.get(address..).ok_or("DWG page address is outside the file")?;
        let (payload, bound) = r2004_data_payload(data, page.file_address, section.max_decomp_size)?;
        let decoded = if section.compressed {
            let decoder = self.decoder.get_or_insert_with(|| R2004Decoder::new(bound));
            if !decoder.step(payload, R2004_COPY_WINDOW)? {
                self.fraction = 0.5 * section_fraction(decoder.output().len(), bound);
                return Ok(false);
            }
            decoder.output()
        } else {
            payload
        };
        let length = length_limit.min(decoded.len());
        let end = self.copied + (length - self.copied).min(R2004_COPY_WINDOW);
        copy_page_bytes(output, start + self.copied, &decoded[self.copied..end])?;
        self.copied = end;
        let decompression = if section.compressed { 0.5 } else { 0.0 };
        self.fraction = decompression + (1.0 - decompression) * section_fraction(end, length);
        Ok(end == length)
    }
}

#[derive(Default)]
struct R2007DrawingPage {
    reader: Option<R2007PageReader>,
    copied: usize,
    fraction: f32,
}

impl R2007DrawingPage {
    fn step(&mut self, file: &[u8], page: &DwgRawPage, layout: R2007PageLayout, output: &mut Vec<u8>, start: usize, length_limit: usize) -> Result<bool, String> {
        let address = usize::try_from(page.file_address).map_err(|_| "DWG page address is too large")?;
        let (bytes, decoded) = if layout.stored {
            let raw = file.get(address..).and_then(|bytes| bytes.get(..layout.size)).ok_or("R2007 stored page is truncated")?;
            (raw, 0.0)
        } else {
            if self.reader.is_none() {
                self.reader = Some(R2007PageReader::new(address, layout.compressed, layout.size, 1, 251)?);
            }
            let reader = self.reader.as_mut().unwrap();
            if !matches!(reader.stage, R2007PageStage::Complete(_)) {
                reader.step(file)?;
                self.fraction = reader.fraction() * 0.5;
                return Ok(false);
            }
            let R2007PageStage::Complete(bytes) = &reader.stage else { unreachable!() };
            (bytes.as_slice(), 0.5)
        };
        let length = length_limit.min(bytes.len());
        let end = self.copied + (length - self.copied).min(R2007_COPY_WINDOW);
        copy_page_bytes(output, start + self.copied, &bytes[self.copied..end])?;
        self.copied = end;
        self.fraction = decoded + (1.0 - decoded) * section_fraction(end, length);
        Ok(end == length)
    }
}

fn copy_page_bytes(output: &mut Vec<u8>, start: usize, decoded: &[u8]) -> Result<(), String> {
    let overlap = decoded.len().min(output.len() - start);
    output.try_reserve(decoded.len() - overlap).map_err(|_| "DWG section allocation failed")?;
    output[start..start + overlap].copy_from_slice(&decoded[..overlap]);
    output.extend_from_slice(&decoded[overlap..]);
    Ok(())
}

enum DrawingDirectory {
    R2004(R2004Directory),
    R2007(R2007Directory),
}

impl DrawingDirectory {
    fn step(&mut self, bytes: &[u8]) -> Result<Option<Vec<DwgRawSection>>, String> {
        match self { Self::R2004(directory) => directory.step(bytes), Self::R2007(directory) => directory.step(bytes) }
    }

    fn fraction(&self) -> f32 {
        match self { Self::R2004(directory) => directory.fraction(), Self::R2007(directory) => directory.fraction() }
    }
}

/// 📑 Resolves drawing sections and restores their physical pages over successive turns.
pub struct DwgDrawingSections<Bytes> {
    bytes: Bytes,
    sections: [DwgRawSection; 2],
    output: [Vec<u8>; 2],
    section: usize,
    page: usize,
    r2004_page: R2004DrawingPage,
    r2007_page: R2007DrawingPage,
    directory: Option<DrawingDirectory>,
    r2007: bool,
}

impl<Bytes: AsRef<[u8]>> DwgDrawingSections<Bytes> {
    pub fn open(bytes: Bytes, r2007: bool) -> Result<Self, String> {
        let directory = if r2007 { DrawingDirectory::R2007(R2007Directory::open(bytes.as_ref(), drawing_section)?) }
        else { DrawingDirectory::R2004(R2004Directory::open(bytes.as_ref(), drawing_section)?) };
        Ok(Self { bytes, sections: Default::default(), output: Default::default(), section: 0, page: 0,
            r2004_page: Default::default(), r2007_page: Default::default(), directory: Some(directory), r2007 })
    }

    /// 📈 Reports how many selected physical pages have been restored.
    pub fn progress(&self) -> (usize, usize) {
        let done = self.sections.iter().take(self.section).map(|section| section.pages.len()).sum::<usize>() + self.page;
        let total = self.sections.iter().map(|section| section.pages.len()).sum();
        (done, total)
    }

    /// 📊 Reports progress through the directory and within the current physical page.
    pub fn fraction(&self) -> f32 {
        if let Some(directory) = &self.directory { return directory.fraction() / 4.0; }
        let (done, total) = self.progress();
        let partial = if self.r2007 { self.r2007_page.fraction } else { self.r2004_page.fraction };
        let data = ((done as f32 + partial) / total.max(1) as f32).min(1.0);
        0.25 + 0.75 * data
    }

    pub fn step(&mut self) -> Result<DwgDrawingSectionStep, String> {
        if let Some(directory) = &mut self.directory {
            if let Some(sections) = directory.step(self.bytes.as_ref())? {
                self.sections = select_drawing_sections(sections)?;
                self.directory = None;
            }
            return Ok(DwgDrawingSectionStep::Directory);
        }
        while let Some(section) = self.sections.get(self.section) {
            let output = &mut self.output[self.section];
            let declared = usize::try_from(section.declared_size).map_err(|_| "DWG section size exceeds the address space")?;
            let page = section.pages.get(self.page);
            let start = page.map_or(declared, |page| page.start_offset.min(section.declared_size) as usize);
            if start > output.len() {
                let padding = (start - output.len()).min(32 * 1024);
                output.try_reserve(padding).map_err(|_| "DWG section allocation failed")?;
                output.resize(output.len() + padding, 0);
                return Ok(DwgDrawingSectionStep::Padding);
            }
            if let Some(page) = page {
                let contextualize = |error| format!("section {} page {}: {error}", section.name, page.page_number);
                if let Some(layout) = page.r2007 {
                    if !self.r2007_page.step(self.bytes.as_ref(), page, layout, output, start, declared - start).map_err(contextualize)? {
                        return Ok(DwgDrawingSectionStep::PageData);
                    }
                    self.r2007_page = Default::default();
                } else {
                    if !self.r2004_page.step(self.bytes.as_ref(), page, section, output, start, declared - start).map_err(contextualize)? {
                        return Ok(DwgDrawingSectionStep::PageData);
                    }
                    self.r2004_page = Default::default();
                }
                self.page += 1;
                return Ok(DwgDrawingSectionStep::Page);
            }
            self.section += 1;
            self.page = 0;
        }
        if self.section == self.sections.len() {
            self.section += 1;
            let [handles, objects] = std::mem::take(&mut self.output);
            Ok(DwgDrawingSectionStep::Complete { handles, objects })
        } else {
            Err("DWG drawing sections were already consumed".into())
        }
    }
}
fn section_fraction(done: usize, total: usize) -> f32 {
    (done as f32 / total.max(1) as f32).min(1.0)
}

fn drawing_section(name: &str) -> bool { matches!(name, "AcDb:Handles" | "AcDb:AcDbObjects") }

fn select_drawing_sections(sections: Vec<DwgRawSection>) -> Result<[DwgRawSection; 2], String> {
    let mut located = sections.into_iter();
    let first = located.next().ok_or("DWG Handles and AcDbObjects sections are missing")?;
    let second = located.next().ok_or("DWG Handles or AcDbObjects section is missing")?;
    let sections = if first.name == "AcDb:Handles" { [first, second] } else { [second, first] };
    if sections[0].name != "AcDb:Handles" || sections[1].name != "AcDb:AcDbObjects" || located.next().is_some() {
        return Err("DWG requires one Handles section and one AcDbObjects section".into());
    }
    Ok(sections)
}
//#endregion DwgRead
//#endregion 🔖️DwgStructuralCodec
