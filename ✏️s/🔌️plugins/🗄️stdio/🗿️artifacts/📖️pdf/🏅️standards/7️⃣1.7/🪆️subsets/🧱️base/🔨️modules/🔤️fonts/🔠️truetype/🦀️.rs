//! 🔠 TrueType / OpenType (sfnt) programs: the tables a PDF writer and reader need — `head`,
//! `hhea`, `hmtx`, `maxp`, `cmap` (formats 0, 4, 6, 12), `loca`/`glyf` (composite-aware),
//! `post` (glyph names), `OS/2`, `name`, `CFF ` presence — plus a glyph-id-preserving subsetter
//! (ISO 32000-1 §9.6.3, §9.9; the OpenType specification for the table layouts).

use std::collections::{BTreeMap, BTreeSet};

//#region 🔖️Reader
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn u16_at(data: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_be_bytes([*data.get(at)?, *data.get(at + 1)?]))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn i16_at(data: &[u8], at: usize) -> Option<i16> {
    u16_at(data, at).map(|value| value as i16)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn u32_at(data: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_be_bytes([*data.get(at)?, *data.get(at + 1)?, *data.get(at + 2)?, *data.get(at + 3)?]))
}

/// 🔠 A parsed sfnt program: table directory plus the decoded metrics every PDF font dictionary
/// derives from it.
#[derive(Clone, Debug)]
pub struct TrueTypeFont {
    pub data: Vec<u8>,
    pub tables: BTreeMap<String, (usize, usize)>,
    pub units_per_em: u16,
    pub num_glyphs: u16,
    pub bbox: [i16; 4],
    pub ascender: i16,
    pub descender: i16,
    pub line_gap: i16,
    pub cap_height: Option<i16>,
    pub x_height: Option<i16>,
    pub weight_class: Option<u16>,
    pub italic_angle: f64,
    pub fixed_pitch: bool,
    pub postscript_name: Option<String>,
    pub family_name: Option<String>,
    pub advances: Vec<u16>,
    pub unicode_map: BTreeMap<u32, u16>,
    pub mac_roman_map: BTreeMap<u8, u16>,
    pub symbol_map: BTreeMap<u32, u16>,
    pub glyph_names: Vec<String>,
    pub cff: bool,
    index_to_loc_long: bool,
}

impl TrueTypeFont {
    /// 📖 Parses `data` (a `.ttf`/`.otf`, or the first font of a `.ttc`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        let mut offset = 0usize;
        let tag = u32_at(data, 0).ok_or("truncated sfnt header")?;
        if tag == 0x74746366 {
            offset = u32_at(data, 12).ok_or("truncated ttc header")? as usize;
        }
        let num_tables = u16_at(data, offset + 4).ok_or("truncated table directory")? as usize;
        let mut tables = BTreeMap::new();
        for index in 0..num_tables {
            let record = offset + 12 + index * 16;
            let name = data.get(record..record + 4).ok_or("truncated table record")?;
            let table_offset = u32_at(data, record + 8).ok_or("truncated table record")? as usize;
            let length = u32_at(data, record + 12).ok_or("truncated table record")? as usize;
            if table_offset <= data.len() {
                tables.insert(String::from_utf8_lossy(name).into_owned(), (table_offset, length.min(data.len() - table_offset)));
            }
        }
        let table = |name: &str| -> Option<&[u8]> { tables.get(name).map(|(start, length)| &data[*start..*start + *length]) };
        let head = table("head").ok_or("missing head table")?;
        let units_per_em = u16_at(head, 18).unwrap_or(1000).max(16);
        let bbox = [i16_at(head, 36).unwrap_or(0), i16_at(head, 38).unwrap_or(0), i16_at(head, 40).unwrap_or(0), i16_at(head, 42).unwrap_or(0)];
        let index_to_loc_long = i16_at(head, 50).unwrap_or(0) != 0;
        let maxp = table("maxp").ok_or("missing maxp table")?;
        let num_glyphs = u16_at(maxp, 4).unwrap_or(0);
        let hhea = table("hhea").ok_or("missing hhea table")?;
        let ascender = i16_at(hhea, 4).unwrap_or(0);
        let descender = i16_at(hhea, 6).unwrap_or(0);
        let line_gap = i16_at(hhea, 8).unwrap_or(0);
        let num_h_metrics = u16_at(hhea, 34).unwrap_or(0) as usize;
        let hmtx = table("hmtx").unwrap_or(&[]);
        let mut advances = Vec::with_capacity(num_glyphs as usize);
        let mut last = 0u16;
        for glyph in 0..num_glyphs as usize {
            if glyph < num_h_metrics {
                last = u16_at(hmtx, glyph * 4).unwrap_or(last);
            }
            advances.push(last);
        }
        let (cap_height, x_height, weight_class) = match table("OS/2") {
            Some(os2) if os2.len() >= 90 => (i16_at(os2, 88), i16_at(os2, 86), u16_at(os2, 4)),
            Some(os2) => (None, None, u16_at(os2, 4)),
            None => (None, None, None),
        };
        let (italic_angle, fixed_pitch, glyph_names) = match table("post") {
            Some(post) => {
                let angle = u32_at(post, 4).map(|fixed| fixed as i32 as f64 / 65536.0).unwrap_or(0.0);
                let fixed = u32_at(post, 12).unwrap_or(0) != 0;
                (angle, fixed, Self::post_glyph_names(post, num_glyphs))
            }
            None => (0.0, false, Vec::new()),
        };
        let (postscript_name, family_name) = table("name").map(Self::names).unwrap_or((None, None));
        let mut unicode_map = BTreeMap::new();
        let mut mac_roman_map = BTreeMap::new();
        let mut symbol_map = BTreeMap::new();
        if let Some(cmap) = table("cmap") {
            let count = u16_at(cmap, 2).unwrap_or(0) as usize;
            let mut best_unicode: Option<(u8, usize)> = None;
            for index in 0..count {
                let record = 4 + index * 8;
                let platform = u16_at(cmap, record).unwrap_or(0);
                let encoding = u16_at(cmap, record + 2).unwrap_or(0);
                let sub_offset = u32_at(cmap, record + 4).unwrap_or(0) as usize;
                if sub_offset >= cmap.len() {
                    continue;
                }
                match (platform, encoding) {
                    (3, 10) | (0, 4) | (0, 6) => {
                        if best_unicode.is_none_or(|(rank, _)| rank < 3) {
                            best_unicode = Some((3, sub_offset));
                        }
                    }
                    (3, 1) | (0, 0..=3) => {
                        if best_unicode.is_none_or(|(rank, _)| rank < 2) {
                            best_unicode = Some((2, sub_offset));
                        }
                    }
                    (3, 0) => {
                        symbol_map = Self::parse_cmap_subtable(&cmap[sub_offset..]);
                    }
                    (1, 0) => {
                        for (code, glyph) in Self::parse_cmap_subtable(&cmap[sub_offset..]) {
                            if code < 256 {
                                mac_roman_map.insert(code as u8, glyph);
                            }
                        }
                    }
                    _ => {}
                }
            }
            if let Some((_, sub_offset)) = best_unicode {
                unicode_map = Self::parse_cmap_subtable(&cmap[sub_offset..]);
            }
        }
        let cff = tables.contains_key("CFF ");
        Ok(Self { data: data.to_vec(), tables, units_per_em, num_glyphs, bbox, ascender, descender, line_gap, cap_height, x_height, weight_class, italic_angle, fixed_pitch, postscript_name, family_name, advances, unicode_map, mac_roman_map, symbol_map, glyph_names, cff, index_to_loc_long })
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn names(name: &[u8]) -> (Option<String>, Option<String>) {
        let count = u16_at(name, 2).unwrap_or(0) as usize;
        let storage = u16_at(name, 4).unwrap_or(0) as usize;
        let mut postscript = None;
        let mut family = None;
        for index in 0..count {
            let record = 6 + index * 12;
            let platform = u16_at(name, record).unwrap_or(0);
            let name_id = u16_at(name, record + 6).unwrap_or(0);
            let length = u16_at(name, record + 8).unwrap_or(0) as usize;
            let offset = u16_at(name, record + 10).unwrap_or(0) as usize;
            let Some(bytes) = name.get(storage + offset..storage + offset + length) else { continue };
            let text = if platform == 3 || platform == 0 { String::from_utf16_lossy(&bytes.chunks(2).filter(|c| c.len() == 2).map(|c| u16::from_be_bytes([c[0], c[1]])).collect::<Vec<_>>()) } else { bytes.iter().map(|b| *b as char).collect() };
            match name_id {
                6 if postscript.is_none() => postscript = Some(text),
                1 if family.is_none() => family = Some(text),
                _ => {}
            }
        }
        (postscript, family)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn post_glyph_names(post: &[u8], num_glyphs: u16) -> Vec<String> {
        let version = u32_at(post, 0).unwrap_or(0);
        match version {
            0x00010000 => MAC_GLYPH_NAMES.iter().take(num_glyphs as usize).map(|name| name.to_string()).collect(),
            0x00020000 => {
                let count = u16_at(post, 32).unwrap_or(0) as usize;
                let mut indices = Vec::with_capacity(count);
                for glyph in 0..count {
                    indices.push(u16_at(post, 34 + glyph * 2).unwrap_or(0));
                }
                let mut names = Vec::new();
                let mut cursor = 34 + count * 2;
                while cursor < post.len() {
                    let length = post[cursor] as usize;
                    cursor += 1;
                    let end = (cursor + length).min(post.len());
                    names.push(String::from_utf8_lossy(&post[cursor..end]).into_owned());
                    cursor = end;
                }
                indices.iter().map(|index| if (*index as usize) < 258 { MAC_GLYPH_NAMES[*index as usize].to_string() } else { names.get(*index as usize - 258).cloned().unwrap_or_default() }).collect()
            }
            _ => Vec::new(),
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn parse_cmap_subtable(sub: &[u8]) -> BTreeMap<u32, u16> {
        let mut map = BTreeMap::new();
        match u16_at(sub, 0).unwrap_or(u16::MAX) {
            0 => {
                for code in 0..256usize {
                    if let Some(glyph) = sub.get(6 + code) {
                        if *glyph != 0 {
                            map.insert(code as u32, *glyph as u16);
                        }
                    }
                }
            }
            4 => {
                let seg_count = u16_at(sub, 6).unwrap_or(0) as usize / 2;
                let ends = 14;
                let starts = ends + seg_count * 2 + 2;
                let deltas = starts + seg_count * 2;
                let range_offsets = deltas + seg_count * 2;
                for segment in 0..seg_count {
                    let end = u16_at(sub, ends + segment * 2).unwrap_or(0) as u32;
                    let start = u16_at(sub, starts + segment * 2).unwrap_or(0) as u32;
                    let delta = u16_at(sub, deltas + segment * 2).unwrap_or(0);
                    let range_offset = u16_at(sub, range_offsets + segment * 2).unwrap_or(0) as usize;
                    if start > end || start == 0xFFFF {
                        continue;
                    }
                    for code in start..=end.min(0xFFFE) {
                        let glyph = if range_offset == 0 {
                            (code as u16).wrapping_add(delta)
                        } else {
                            let address = range_offsets + segment * 2 + range_offset + (code - start) as usize * 2;
                            match u16_at(sub, address) {
                                Some(0) | None => 0,
                                Some(glyph) => glyph.wrapping_add(delta),
                            }
                        };
                        if glyph != 0 {
                            map.insert(code, glyph);
                        }
                    }
                }
            }
            6 => {
                let first = u16_at(sub, 6).unwrap_or(0) as u32;
                let count = u16_at(sub, 8).unwrap_or(0) as usize;
                for index in 0..count {
                    if let Some(glyph) = u16_at(sub, 10 + index * 2) {
                        if glyph != 0 {
                            map.insert(first + index as u32, glyph);
                        }
                    }
                }
            }
            12 => {
                let groups = u32_at(sub, 12).unwrap_or(0) as usize;
                for group in 0..groups.min(100_000) {
                    let record = 16 + group * 12;
                    let start = u32_at(sub, record).unwrap_or(0);
                    let end = u32_at(sub, record + 4).unwrap_or(0);
                    let glyph = u32_at(sub, record + 8).unwrap_or(0);
                    if start > end || end - start > 0xFFFF {
                        continue;
                    }
                    for code in start..=end {
                        let gid = glyph + (code - start);
                        if gid != 0 && gid <= u16::MAX as u32 {
                            map.insert(code, gid as u16);
                        }
                    }
                }
            }
            _ => {}
        }
        map
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn table(&self, name: &str) -> Option<&[u8]> {
        self.tables.get(name).map(|(start, length)| &self.data[*start..*start + *length])
    }

    /// 🔢 Glyph id of a Unicode scalar (unicode cmap, then the (3,0) symbol cmap's F0xx range).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn glyph_for_char(&self, character: char) -> Option<u16> {
        let code = character as u32;
        self.unicode_map.get(&code).copied().or_else(|| self.symbol_map.get(&(0xF000 + (code & 0xFF))).copied().filter(|_| code < 256)).or_else(|| self.symbol_map.get(&code).copied())
    }

    /// 🔢 Glyph id of a glyph name (`post` table, then `gXX`/`glyphXX`/`uniXXXX` forms).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn glyph_for_name(&self, name: &str) -> Option<u16> {
        if let Some(index) = self.glyph_names.iter().position(|candidate| candidate == name) {
            return Some(index as u16);
        }
        for prefix in ["glyph", "g", "index"] {
            if let Some(rest) = name.strip_prefix(prefix) {
                if let Ok(index) = rest.parse::<u16>() {
                    return Some(index);
                }
            }
        }
        None
    }

    /// 📏 Advance width of a glyph in 1/1000 text-space units.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn advance_1000(&self, glyph: u16) -> f64 {
        let advance = self.advances.get(glyph as usize).or(self.advances.last()).copied().unwrap_or(0) as f64;
        advance * 1000.0 / self.units_per_em as f64
    }

    /// 📐 A font-unit value scaled to 1/1000 em.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn scale_1000(&self, value: i16) -> f64 {
        value as f64 * 1000.0 / self.units_per_em as f64
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn glyph_location(&self, glyph: u16) -> Option<(usize, usize)> {
        let loca = self.table("loca")?;
        let index = glyph as usize;
        let (start, end) = if self.index_to_loc_long { (u32_at(loca, index * 4)? as usize, u32_at(loca, index * 4 + 4)? as usize) } else { (u16_at(loca, index * 2)? as usize * 2, u16_at(loca, index * 2 + 2)? as usize * 2) };
        Some((start, end))
    }

    /// 🧩 Glyph ids a composite glyph references (empty for simple glyphs).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn component_glyphs(&self, glyph: u16) -> Vec<u16> {
        let mut out = Vec::new();
        let Some(glyf) = self.table("glyf") else { return out };
        let Some((start, end)) = self.glyph_location(glyph) else { return out };
        if end <= start || end > glyf.len() {
            return out;
        }
        let data = &glyf[start..end];
        if i16_at(data, 0).unwrap_or(0) >= 0 {
            return out;
        }
        let mut cursor = 10usize;
        loop {
            let Some(flags) = u16_at(data, cursor) else { break };
            let Some(component) = u16_at(data, cursor + 2) else { break };
            out.push(component);
            cursor += 4;
            cursor += if flags & 0x0001 != 0 { 4 } else { 2 };
            if flags & 0x0008 != 0 {
                cursor += 2;
            } else if flags & 0x0040 != 0 {
                cursor += 4;
            } else if flags & 0x0080 != 0 {
                cursor += 8;
            }
            if flags & 0x0020 == 0 {
                break;
            }
        }
        out
    }

    /// ✂️ A glyph-id-preserving subset: every glyph outside the closure of `glyphs` (components
    /// included) is emptied, unused tables dropped, and the result is a valid `FontFile2` whose
    /// glyph ids still match this font's — so `/CIDToGIDMap /Identity` and `post` names keep
    /// working. CFF-based programs cannot be subset this way and are returned whole.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn subset(&self, glyphs: &BTreeSet<u16>) -> Vec<u8> {
        let (Some(glyf), Some(_)) = (self.table("glyf"), self.table("loca")) else { return self.data.clone() };
        let mut keep: BTreeSet<u16> = BTreeSet::new();
        let mut pending: Vec<u16> = glyphs.iter().copied().chain(std::iter::once(0)).collect();
        while let Some(glyph) = pending.pop() {
            if glyph >= self.num_glyphs || !keep.insert(glyph) {
                continue;
            }
            pending.extend(self.component_glyphs(glyph));
        }
        let mut new_glyf = Vec::new();
        let mut offsets = Vec::with_capacity(self.num_glyphs as usize + 1);
        for glyph in 0..self.num_glyphs {
            offsets.push(new_glyf.len() as u32);
            if keep.contains(&glyph) {
                if let Some((start, end)) = self.glyph_location(glyph) {
                    if end > start && end <= glyf.len() {
                        new_glyf.extend_from_slice(&glyf[start..end]);
                        while new_glyf.len() % 4 != 0 {
                            new_glyf.push(0);
                        }
                    }
                }
            }
        }
        offsets.push(new_glyf.len() as u32);
        let long_loca = new_glyf.len() > 0x1FFFE;
        let mut loca = Vec::new();
        for offset in &offsets {
            if long_loca {
                loca.extend_from_slice(&offset.to_be_bytes());
            } else {
                loca.extend_from_slice(&((offset / 2) as u16).to_be_bytes());
            }
        }
        let mut head = self.table("head").map(<[u8]>::to_vec).unwrap_or_default();
        if head.len() >= 52 {
            head[50..52].copy_from_slice(&(long_loca as u16).to_be_bytes());
            head[8..12].copy_from_slice(&[0, 0, 0, 0]);
        }
        let mut tables: Vec<(&str, Vec<u8>)> = Vec::new();
        for name in ["cvt ", "fpgm", "prep"] {
            if let Some(table) = self.table(name) {
                tables.push((name, table.to_vec()));
            }
        }
        tables.push(("glyf", new_glyf));
        tables.push(("head", head));
        for name in ["hhea", "hmtx", "maxp", "OS/2", "cmap", "post"] {
            if let Some(table) = self.table(name) {
                tables.push((name, table.to_vec()));
            }
        }
        tables.push(("loca", loca));
        tables.sort_by(|a, b| a.0.cmp(b.0));
        build_sfnt(&tables)
    }

    /// 🧾 The whole program as a `FontFile2`-shaped sfnt (tables the writer needs, nothing else).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn all_glyphs(&self) -> BTreeSet<u16> {
        (0..self.num_glyphs).collect()
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn table_checksum(table: &[u8]) -> u32 {
    let mut sum = 0u32;
    for chunk in table.chunks(4) {
        let mut word = [0u8; 4];
        word[..chunk.len()].copy_from_slice(chunk);
        sum = sum.wrapping_add(u32::from_be_bytes(word));
    }
    sum
}

/// 🏗️ Assembles an sfnt from (tag, table) pairs sorted by tag, with a valid directory.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn build_sfnt(tables: &[(&str, Vec<u8>)]) -> Vec<u8> {
    let count = tables.len() as u16;
    let mut entry_selector = 0u16;
    while (2u16 << entry_selector) <= count {
        entry_selector += 1;
    }
    let search_range = (1u16 << entry_selector) * 16;
    let mut out = Vec::new();
    out.extend_from_slice(&0x00010000u32.to_be_bytes());
    out.extend_from_slice(&count.to_be_bytes());
    out.extend_from_slice(&search_range.to_be_bytes());
    out.extend_from_slice(&entry_selector.to_be_bytes());
    out.extend_from_slice(&(count * 16 - search_range).to_be_bytes());
    let mut offset = 12 + tables.len() * 16;
    let mut body = Vec::new();
    let mut head_offset = None;
    for (tag, table) in tables {
        let mut padded = table.clone();
        while padded.len() % 4 != 0 {
            padded.push(0);
        }
        out.extend_from_slice(tag.as_bytes());
        out.extend_from_slice(&table_checksum(&padded).to_be_bytes());
        out.extend_from_slice(&(offset as u32).to_be_bytes());
        out.extend_from_slice(&(table.len() as u32).to_be_bytes());
        if *tag == "head" {
            head_offset = Some(offset);
        }
        offset += padded.len();
        body.extend_from_slice(&padded);
    }
    out.extend_from_slice(&body);
    if let Some(head) = head_offset {
        let adjustment = 0xB1B0AFBAu32.wrapping_sub(table_checksum(&out));
        out[head + 8..head + 12].copy_from_slice(&adjustment.to_be_bytes());
    }
    out
}
//#endregion 🔖️Reader

//#region 🔖️MacGlyphNames
/// 🍎 The 258 standard Macintosh glyph names a `post` table format 1.0/2.0 indexes.
pub const MAC_GLYPH_NAMES: [&str; 258] = [
    ".notdef", ".null", "nonmarkingreturn", "space", "exclam", "quotedbl", "numbersign", "dollar", "percent", "ampersand", "quotesingle", "parenleft", "parenright", "asterisk", "plus", "comma", "hyphen", "period", "slash", "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "colon", "semicolon", "less", "equal", "greater", "question", "at", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W",
    "X", "Y", "Z", "bracketleft", "backslash", "bracketright", "asciicircum", "underscore", "grave", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "braceleft", "bar", "braceright", "asciitilde", "Adieresis", "Aring", "Ccedilla", "Eacute", "Ntilde", "Odieresis", "Udieresis", "aacute", "agrave", "acircumflex", "adieresis", "atilde", "aring", "ccedilla", "eacute", "egrave", "ecircumflex", "edieresis", "iacute",
    "igrave", "icircumflex", "idieresis", "ntilde", "oacute", "ograve", "ocircumflex", "odieresis", "otilde", "uacute", "ugrave", "ucircumflex", "udieresis", "dagger", "degree", "cent", "sterling", "section", "bullet", "paragraph", "germandbls", "registered", "copyright", "trademark", "acute", "dieresis", "notequal", "AE", "Oslash", "infinity", "plusminus", "lessequal", "greaterequal", "yen", "mu", "partialdiff", "summation", "product", "pi", "integral", "ordfeminine", "ordmasculine", "Omega", "ae", "oslash",
    "questiondown", "exclamdown", "logicalnot", "radical", "florin", "approxequal", "Delta", "guillemotleft", "guillemotright", "ellipsis", "nonbreakingspace", "Agrave", "Atilde", "Otilde", "OE", "oe", "endash", "emdash", "quotedblleft", "quotedblright", "quoteleft", "quoteright", "divide", "lozenge", "ydieresis", "Ydieresis", "fraction", "currency", "guilsinglleft", "guilsinglright", "fi", "fl", "daggerdbl", "periodcentered", "quotesinglbase", "quotedblbase", "perthousand", "Acircumflex", "Ecircumflex", "Aacute",
    "Edieresis", "Egrave", "Iacute", "Icircumflex", "Idieresis", "Igrave", "Oacute", "Ocircumflex", "apple", "Ograve", "Uacute", "Ucircumflex", "Ugrave", "dotlessi", "circumflex", "tilde", "macron", "breve", "dotaccent", "ring", "cedilla", "hungarumlaut", "ogonek", "caron", "Lslash", "lslash", "Scaron", "scaron", "Zcaron", "zcaron", "brokenbar", "Eth", "eth", "Yacute", "yacute", "Thorn", "thorn", "minus", "multiply", "onesuperior", "twosuperior", "threesuperior", "onehalf", "onequarter", "threequarters", "franc",
    "Gbreve", "gbreve", "Idotaccent", "Scedilla", "scedilla", "Cacute", "cacute", "Ccaron", "ccaron", "dcroat",
];
//#endregion 🔖️MacGlyphNames

//#region 🔖️Synthesis
/// 🧪 Builds a minimal, valid TrueType program from glyph outlines given as (unicode, advance,
/// contours of on-curve points in font units) — used by tests and by fixtures that need an
/// embeddable font without shipping one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn synthesize_truetype(units_per_em: u16, glyphs: &[(u32, u16, Vec<Vec<(i16, i16)>>)]) -> Vec<u8> {
    let mut glyf = Vec::new();
    let mut loca: Vec<u32> = vec![0];
    let mut advances: Vec<u16> = Vec::new();
    let mut cmap_entries: Vec<(u32, u16)> = Vec::new();
    let all: Vec<(u32, u16, Vec<Vec<(i16, i16)>>)> = std::iter::once((0u32, units_per_em / 2, Vec::new())).chain(glyphs.iter().cloned()).collect();
    for (index, (code, advance, contours)) in all.iter().enumerate() {
        advances.push(*advance);
        if index > 0 {
            cmap_entries.push((*code, index as u16));
        }
        if !contours.is_empty() {
            let points: Vec<(i16, i16)> = contours.iter().flatten().copied().collect();
            let (min_x, min_y, max_x, max_y) = points.iter().fold((i16::MAX, i16::MAX, i16::MIN, i16::MIN), |acc, p| (acc.0.min(p.0), acc.1.min(p.1), acc.2.max(p.0), acc.3.max(p.1)));
            glyf.extend_from_slice(&(contours.len() as i16).to_be_bytes());
            for value in [min_x, min_y, max_x, max_y] {
                glyf.extend_from_slice(&value.to_be_bytes());
            }
            let mut end = 0usize;
            for contour in contours {
                end += contour.len();
                glyf.extend_from_slice(&((end - 1) as u16).to_be_bytes());
            }
            glyf.extend_from_slice(&0u16.to_be_bytes());
            glyf.extend(std::iter::repeat_n(0x01u8, points.len()));
            let mut previous = (0i16, 0i16);
            let mut xs = Vec::new();
            let mut ys = Vec::new();
            for point in &points {
                xs.extend_from_slice(&(point.0 - previous.0).to_be_bytes());
                ys.extend_from_slice(&(point.1 - previous.1).to_be_bytes());
                previous = *point;
            }
            glyf.extend_from_slice(&xs);
            glyf.extend_from_slice(&ys);
            while glyf.len() % 4 != 0 {
                glyf.push(0);
            }
        }
        loca.push(glyf.len() as u32);
    }
    let num_glyphs = all.len() as u16;
    let mut head = Vec::new();
    head.extend_from_slice(&0x00010000u32.to_be_bytes());
    head.extend_from_slice(&0x00010000u32.to_be_bytes());
    head.extend_from_slice(&0u32.to_be_bytes());
    head.extend_from_slice(&0x5F0F3CF5u32.to_be_bytes());
    head.extend_from_slice(&0x000Bu16.to_be_bytes());
    head.extend_from_slice(&units_per_em.to_be_bytes());
    head.extend_from_slice(&[0u8; 16]);
    for value in [0i16, (-(units_per_em as i32) / 4) as i16, units_per_em as i16, units_per_em as i16] {
        head.extend_from_slice(&value.to_be_bytes());
    }
    head.extend_from_slice(&0u16.to_be_bytes());
    head.extend_from_slice(&8u16.to_be_bytes());
    head.extend_from_slice(&2i16.to_be_bytes());
    head.extend_from_slice(&1i16.to_be_bytes());
    head.extend_from_slice(&0i16.to_be_bytes());
    let mut hhea = Vec::new();
    hhea.extend_from_slice(&0x00010000u32.to_be_bytes());
    hhea.extend_from_slice(&((units_per_em as i32 * 4 / 5) as i16).to_be_bytes());
    hhea.extend_from_slice(&((-(units_per_em as i32) / 5) as i16).to_be_bytes());
    hhea.extend_from_slice(&0i16.to_be_bytes());
    hhea.extend_from_slice(&advances.iter().copied().max().unwrap_or(0).to_be_bytes());
    hhea.extend_from_slice(&[0u8; 20]);
    hhea.extend_from_slice(&0i16.to_be_bytes());
    hhea.extend_from_slice(&num_glyphs.to_be_bytes());
    let mut hmtx = Vec::new();
    for advance in &advances {
        hmtx.extend_from_slice(&advance.to_be_bytes());
        hmtx.extend_from_slice(&0i16.to_be_bytes());
    }
    let mut maxp = Vec::new();
    maxp.extend_from_slice(&0x00010000u32.to_be_bytes());
    maxp.extend_from_slice(&num_glyphs.to_be_bytes());
    maxp.extend_from_slice(&[0, 64, 0, 8, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let mut loca_bytes = Vec::new();
    for offset in &loca {
        loca_bytes.extend_from_slice(&offset.to_be_bytes());
    }
    let mut cmap = Vec::new();
    cmap.extend_from_slice(&0u16.to_be_bytes());
    cmap.extend_from_slice(&1u16.to_be_bytes());
    cmap.extend_from_slice(&3u16.to_be_bytes());
    cmap.extend_from_slice(&10u16.to_be_bytes());
    cmap.extend_from_slice(&12u32.to_be_bytes());
    cmap.extend_from_slice(&12u16.to_be_bytes());
    cmap.extend_from_slice(&0u16.to_be_bytes());
    cmap.extend_from_slice(&(16 + cmap_entries.len() as u32 * 12).to_be_bytes());
    cmap.extend_from_slice(&0u32.to_be_bytes());
    cmap.extend_from_slice(&(cmap_entries.len() as u32).to_be_bytes());
    for (code, glyph) in &cmap_entries {
        cmap.extend_from_slice(&code.to_be_bytes());
        cmap.extend_from_slice(&code.to_be_bytes());
        cmap.extend_from_slice(&(*glyph as u32).to_be_bytes());
    }
    let mut post = Vec::new();
    post.extend_from_slice(&0x00030000u32.to_be_bytes());
    post.extend_from_slice(&[0u8; 28]);
    let tables: Vec<(&str, Vec<u8>)> = vec![("cmap", cmap), ("glyf", glyf), ("head", head), ("hhea", hhea), ("hmtx", hmtx), ("loca", loca_bytes), ("maxp", maxp), ("post", post)];
    build_sfnt(&tables)
}
//#endregion 🔖️Synthesis

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
