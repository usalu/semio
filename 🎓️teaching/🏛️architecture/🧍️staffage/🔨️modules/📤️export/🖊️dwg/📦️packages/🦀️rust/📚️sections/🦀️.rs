//! 📚️ The sections of an AC1024 file other than the objects: header variables, classes, the handle map, and the small fixed sections (summary, application info, file dependencies, revision history, auxiliary header, free space, template).
//! <https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf> (5.8 Classes, 5.9 Header, 9 Header variables, 13 to 18, 21 to 23, 27)

use crate::bits::{aci, Bits, Ref, Streams, BY_BLOCK, BY_LAYER};
use crate::checksum::{crc16, CRC16_SEED};
use crate::objects::{handle as h, Database};

const HEADER_START: [u8; 16] = [0xCF, 0x7B, 0x1F, 0x23, 0xFD, 0xDE, 0x38, 0xA9, 0x5F, 0x7C, 0x68, 0xB8, 0x4E, 0x6D, 0x33, 0x5F];
const HEADER_END: [u8; 16] = [0x30, 0x84, 0xE0, 0xDC, 0x02, 0x21, 0xC7, 0x56, 0xA0, 0x83, 0x97, 0x47, 0xB1, 0x92, 0xCC, 0xA0];
const CLASSES_START: [u8; 16] = [0x8D, 0xA1, 0xC4, 0xB8, 0xC4, 0xA9, 0xF8, 0xC5, 0xC0, 0xDC, 0xF4, 0x5F, 0xE7, 0xCF, 0xB6, 0x8A];
const CLASSES_END: [u8; 16] = [0x72, 0x5E, 0x3B, 0x47, 0x3B, 0x56, 0x07, 0x3A, 0x3F, 0x23, 0x0B, 0xA0, 0x18, 0x30, 0x49, 0x75];
const PI_2: f64 = std::f64::consts::FRAC_PI_2;

/// 🧾️ The application version of AutoCAD 2010 (AC1024) in the file header and the auxiliary header.
pub const APPLICATION_VERSION: u8 = 29;

fn sentineled(start: &[u8; 16], end: &[u8; 16], data: &[u8]) -> Vec<u8> {
    let mut covered = (data.len() as u32).to_le_bytes().to_vec();
    covered.extend_from_slice(data);
    let crc = crc16(CRC16_SEED, &covered);
    let mut out = start.to_vec();
    out.extend_from_slice(&covered);
    out.extend_from_slice(&crc.to_le_bytes());
    out.extend_from_slice(end);
    out.extend_from_slice(&[0; 8]);
    out
}

fn section() -> Streams {
    let mut s = Streams::new();
    s.main.rl(0);
    s
}

/// 🌱️ A deterministic 128-bit identifier of `content` (two FNV-1a runs) as a GUID string.
pub fn guid(content: &[u8], salt: u64) -> String {
    let run = |seed: u64| content.iter().fold(seed, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01B3));
    let (a, b) = (run(0xCBF2_9CE4_8422_2325 ^ salt), run(0x9E37_79B9_7F4A_7C15 ^ salt));
    format!("{{{:08X}-{:04X}-{:04X}-{:04X}-{:012X}}}", (a >> 32) as u32, (a >> 16) as u16, a as u16, (b >> 48) as u16, b & 0xFFFF_FFFF_FFFF)
}

fn space(s: &mut Streams, extents: ([f64; 2], [f64; 2]), unbounded: bool) {
    let (low, high) = if unbounded { ([1e20; 3], [-1e20; 3]) } else { ([extents.0[0], extents.0[1], 0.0], [extents.1[0], extents.1[1], 0.0]) };
    s.main.bd3(0.0, 0.0, 0.0);
    s.main.bd3(low[0], low[1], low[2]);
    s.main.bd3(high[0], high[1], high[2]);
    s.main.rd2(0.0, 0.0);
    s.main.rd2(12.0, 9.0);
    s.main.bd(0.0);
    s.main.bd3(0.0, 0.0, 0.0);
    s.main.bd3(1.0, 0.0, 0.0);
    s.main.bd3(0.0, 1.0, 0.0);
    s.reference(Ref::HardPointer, 0);
    s.reference(Ref::HardPointer, 0);
    s.main.bs(0);
    s.reference(Ref::HardPointer, 0);
    (0..6).for_each(|_| s.main.bd3(0.0, 0.0, 0.0));
}

fn dimensions(s: &mut Streams) {
    let m = &mut s.main;
    [1.0, 0.18, 0.0625, 0.38, 0.18, 0.0, 0.0, 0.0, 0.0, 1.0, std::f64::consts::FRAC_PI_4].into_iter().for_each(|value| m.bd(value));
    m.bs(0);
    m.cmc(BY_BLOCK);
    [false, false, true, true, false, false].into_iter().for_each(|flag| m.bit(flag));
    [0, 0, 0, 0].into_iter().for_each(|value| m.bs(value));
    [0.18, 0.09, 0.0, 25.4, 1.0, 0.0, 1.0, 0.09, 0.0].into_iter().for_each(|value| m.bd(value));
    m.bit(false);
    m.bs(2);
    [false, false, false, false].into_iter().for_each(|flag| m.bit(flag));
    (0..3).for_each(|_| m.cmc(BY_BLOCK));
    [0, 4, 4, 2, 2, 0, 0, 2, 46, 0, 0].into_iter().for_each(|value| m.bs(value));
    m.bit(false);
    m.bit(false);
    [1, 0, 0, 0].into_iter().for_each(|value| m.bs(value));
    m.bit(false);
    m.bs(3);
    m.bit(false);
    m.bit(false);
    m.bd(0.0);
    m.bd(0.0);
    s.string("");
    s.string("");
    s.reference(Ref::HardPointer, h::STYLE_STANDARD);
    (0..7).for_each(|_| s.reference(Ref::HardPointer, 0));
    s.main.bs(-2);
    s.main.bs(-2);
}

/// 📋️ The AcDb:Header section: the drawing variables of a metric drawing whose insertion units are metres.
pub fn header(database: &Database, fingerprint: &str, version: &str) -> Vec<u8> {
    let mut s = section();
    [412_148_564_080.0, 1.0, 1.0, 1.0].into_iter().for_each(|value| s.main.bd(value));
    ["m", "", "", ""].into_iter().for_each(|text| s.string(text));
    s.main.bl(24);
    s.main.bl(0);
    [true, true, false, false, true, true, false, true, false].into_iter().for_each(|flag| s.main.bit(flag));
    s.main.bit(false);
    [true, false, false, false, false, true, true, false, true].into_iter().for_each(|flag| s.main.bit(flag));
    s.main.bit(false);
    s.main.bit(false);
    s.main.bs(1);
    [3020, 2, 4, 0, 0, 1, 0].into_iter().for_each(|value| s.main.bs(value));
    (0..3).for_each(|_| s.main.bl(0));
    (0..5).for_each(|_| s.main.bs(0));
    [8, 6, 6, 6, 6, 6, 6, 3, 70, 0, 64, 4, 0, 50].into_iter().for_each(|value| s.main.bs(value));
    [1.0, 2.5, 0.05, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 1.0].into_iter().for_each(|value| s.main.bd(value));
    s.string("");
    (0..4).for_each(|_| s.main.bl(0));
    (0..3).for_each(|_| s.main.bl(0));
    (0..4).for_each(|_| s.main.bl(0));
    s.main.cmc(BY_LAYER);
    s.main.handle(Ref::Absolute, database.next_handle);
    for target in [h::LAYER_0, h::STYLE_STANDARD, h::LTYPE_BYLAYER, 0, h::DIMSTYLE_STANDARD, h::MLINESTYLE_STANDARD] {
        s.reference(Ref::HardPointer, target);
    }
    s.main.bd(0.0);
    let extents = database.extents.unwrap_or(([0.0; 2], [0.0; 2]));
    space(&mut s, extents, true);
    space(&mut s, extents, database.extents.is_none());
    s.string("");
    s.string("");
    dimensions(&mut s);
    for target in [h::BLOCK_CONTROL, h::LAYER_CONTROL, h::STYLE_CONTROL, h::LTYPE_CONTROL, h::VIEW_CONTROL, h::UCS_CONTROL, h::VPORT_CONTROL, h::APPID_CONTROL, h::DIMSTYLE_CONTROL] {
        s.reference(Ref::HardOwner, target);
    }
    for target in [h::GROUP_DICTIONARY, h::MLINESTYLE_DICTIONARY] {
        s.reference(Ref::HardPointer, target);
    }
    s.reference(Ref::HardOwner, h::NAMED_OBJECTS);
    s.main.bs(1);
    s.main.bs(70);
    s.string("");
    s.string("");
    for target in [h::LAYOUT_DICTIONARY, h::PLOTSETTINGS_DICTIONARY, h::PLOTSTYLE_DICTIONARY, h::MATERIAL_DICTIONARY, h::COLOR_DICTIONARY, h::VISUALSTYLE_DICTIONARY] {
        s.reference(Ref::HardPointer, target);
    }
    s.main.bl(0x1D | 0x200 | 0x800 | 0x2000);
    s.main.bs(6);
    s.main.bs(0);
    s.string(fingerprint);
    s.string(version);
    [127, 0, 1, 2, 2, 0].into_iter().for_each(|value| s.main.rc(value));
    s.main.bs(257);
    s.main.bs(257);
    s.main.rc(0);
    s.main.rc(0);
    s.string("");
    for target in [h::PAPER_RECORD, h::MODEL_RECORD, h::LTYPE_BYLAYER, h::LTYPE_BYBLOCK, h::LTYPE_CONTINUOUS] {
        s.reference(Ref::HardPointer, target);
    }
    s.main.bit(false);
    s.main.bl(0);
    s.main.bl(0);
    s.main.bd(0.0);
    [2.0, 6.0, 2.0, 50.0, 0.0].into_iter().for_each(|value| s.main.bd(value));
    s.main.rc(0);
    s.main.rc(1);
    [0.25, 0.25, PI_2, PI_2, 0.0, 0.0].into_iter().for_each(|value| s.main.bd(value));
    s.main.bs(7);
    s.main.rc(1);
    [37.795, -122.394, 0.0].into_iter().for_each(|value| s.main.bd(value));
    s.main.bl(-8000);
    [0, 1, 0, 0].into_iter().for_each(|value| s.main.rc(value));
    s.main.bit(false);
    s.main.cmc(aci(257));
    (0..3).for_each(|_| s.reference(Ref::HardPointer, 0));
    s.main.rc(0);
    s.main.bd(0.0);
    (0..4).for_each(|_| s.main.bs(-1));
    s.main.bl(0);
    s.main.bl(0);
    s.main.bit(false);
    let (data, _) = s.merge(true);
    sentineled(&HEADER_START, &HEADER_END, &data)
}

const CLASSES: [(u16, &str, &str, u16, i32); 4] = [
    (500, "AcDbDictionaryWithDefault", "ACDBDICTIONARYWDFLT", 0x1F3, 1),
    (501, "AcDbPlaceHolder", "ACDBPLACEHOLDER", 0x1F3, 1),
    (502, "AcDbLayout", "LAYOUT", 0x1F3, 2),
    (503, "AcDbPolyline", "LWPOLYLINE", 0x1F2, 0),
];

/// 🏛️ The AcDb:Classes section: the four classes whose objects the writer emits by class number or by fixed type.
pub fn classes(polylines: usize) -> Vec<u8> {
    let mut s = section();
    s.main.bs(503);
    s.main.rc(0);
    s.main.rc(0);
    s.main.bit(true);
    for (number, cpp, dxf, item, count) in CLASSES {
        s.main.bs(number as i16);
        s.main.bs(0);
        s.string("ObjectDBX Classes");
        s.string(cpp);
        s.string(dxf);
        s.main.bit(false);
        s.main.bs(item as i16);
        s.main.bl(if number == 503 { polylines as i32 } else { count });
        s.main.bs(0);
        s.main.bs(0);
        s.main.bl(0);
        s.main.bl(0);
    }
    let (data, _) = s.merge(true);
    sentineled(&CLASSES_START, &CLASSES_END, &data)
}

/// 🗺️ The AcDb:Handles section: handle to object offset pairs in chunks of at most 2032 bytes, then the empty terminating chunk.
pub fn handles(records: &[(u64, u32)]) -> Vec<u8> {
    fn chunk(out: &mut Vec<u8>, body: &[u8]) {
        let start = out.len();
        out.extend_from_slice(&((body.len() + 2) as u16).to_be_bytes());
        out.extend_from_slice(body);
        let crc = crc16(CRC16_SEED, &out[start..]);
        out.extend_from_slice(&crc.to_be_bytes());
    }
    let mut sorted = records.to_vec();
    sorted.sort_by_key(|(handle, _)| *handle);
    let (mut out, mut body, mut last_handle, mut last_offset) = (Vec::new(), Vec::new(), 0u64, 0i64);
    for (handle, offset) in sorted {
        let mut pair = Vec::new();
        unsigned(&mut pair, handle - last_handle);
        signed(&mut pair, i64::from(offset) - last_offset);
        if body.len() + pair.len() > 2032 {
            chunk(&mut out, &body);
            body.clear();
            pair.clear();
            unsigned(&mut pair, handle);
            signed(&mut pair, i64::from(offset));
        }
        body.extend_from_slice(&pair);
        (last_handle, last_offset) = (handle, i64::from(offset));
    }
    if !body.is_empty() {
        chunk(&mut out, &body);
    }
    chunk(&mut out, &[]);
    out
}

fn unsigned(out: &mut Vec<u8>, mut value: u64) {
    while value >= 0x80 {
        out.push((value & 0x7F) as u8 | 0x80);
        value >>= 7;
    }
    out.push(value as u8);
}

fn signed(out: &mut Vec<u8>, value: i64) {
    let mut magnitude = value.unsigned_abs();
    while magnitude >= 0x40 {
        out.push((magnitude & 0x7F) as u8 | 0x80);
        magnitude >>= 7;
    }
    out.push(magnitude as u8 | if value < 0 { 0x40 } else { 0 });
}

fn unicode(out: &mut Vec<u8>, text: &str) {
    let units: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
    out.extend_from_slice(&(units.len() as u16).to_le_bytes());
    units.into_iter().for_each(|unit| out.extend_from_slice(&unit.to_le_bytes()));
}

/// 📝️ The AcDb:SummaryInfo section with empty properties and no dates.
pub fn summary() -> Vec<u8> {
    let mut out = Vec::new();
    (0..8).for_each(|_| unicode(&mut out, ""));
    out.extend_from_slice(&[0; 8 + 8 + 8 + 2 + 8]);
    out
}

/// 🧰️ The AcDb:AppInfo section naming this writer.
pub fn app_info() -> Vec<u8> {
    let mut out = 2u32.to_le_bytes().to_vec();
    unicode(&mut out, "AppInfoDataList");
    out.extend_from_slice(&3u32.to_le_bytes());
    for text in ["1.0.0", "Staffage archive DWG export", "<ProductInformation name=\"staffage-dwg\" build_version=\"1.0.0\" registry_version=\"1.0.0\" install_id_string=\"staffage\" registry_localeID=\"1033\"/>"] {
        out.extend_from_slice(&[0; 16]);
        unicode(&mut out, text);
    }
    out
}

/// 🔗️ The AcDb:FileDepList section without features or files.
pub fn file_dependencies() -> Vec<u8> {
    vec![0; 8]
}

/// 🕰️ The AcDb:RevHistory section as the ODA writes it for R2007 and later.
pub fn revision_history() -> Vec<u8> {
    [0u32, 0, 1, 0].iter().flat_map(|value| value.to_le_bytes()).collect()
}

/// 🧭️ The AcDb:AuxHeader section: version fields and one save.
pub fn aux_header(next_handle: u64) -> Vec<u8> {
    let mut s = Bits::new();
    s.raw(&[0xFF, 0x77, 0x01]);
    let version = u16::from(APPLICATION_VERSION);
    s.rs(version);
    s.rs(0);
    s.rl(1);
    s.rl(u32::MAX);
    s.rs(1);
    s.rs(0);
    s.rl(0);
    [version, 0, version, 0, 5, 0x893, 5, 0x893, 0, 1].into_iter().for_each(|value| s.rs(value));
    (0..5).for_each(|_| s.rl(0));
    (0..4).for_each(|_| s.rl(0));
    s.rl(if next_handle < 0x7FFF_FFFF { next_handle as u32 } else { u32::MAX });
    s.rl(0);
    s.rs(0);
    s.rs(1);
    (0..3).for_each(|_| s.rl(0));
    s.rl(1);
    (0..4).for_each(|_| s.rl(0));
    s.into_bytes()
}

/// 📦️ The AcDb:ObjFreeSpace section: the object count and the constants the ODA writes.
pub fn free_space(objects: usize) -> Vec<u8> {
    let mut out = 0u32.to_le_bytes().to_vec();
    out.extend_from_slice(&(objects as u32).to_le_bytes());
    out.extend_from_slice(&[0; 8]);
    out.extend_from_slice(&0u32.to_le_bytes());
    out.push(4);
    [0x32u32, 0, 0x64, 0, 0x200, 0, 0xFFFF_FFFF, 0].iter().for_each(|value| out.extend_from_slice(&value.to_le_bytes()));
    out
}

/// 📏️ The AcDb:Template section: no description and MEASUREMENT 1 (metric).
pub fn template() -> Vec<u8> {
    vec![0, 0, 1, 0]
}
