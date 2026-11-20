//! 🔤️ The PDF text string codec (§7.9.2.2): PDFDocEncoding table, decode and encode.

use std::collections::HashMap;
use std::sync::OnceLock;

//#region 🔤️TextString
/// 🔡 PDFDocEncoding (Annex D.2): code → Unicode scalar (text strings, not fonts).
pub const PDF_DOC_ENCODING: &[(u8, u32)] = &[(24, 728), (25, 711), (26, 710), (27, 729), (28, 733), (29, 731), (30, 730), (31, 732), (32, 32), (33, 33), (34, 34), (35, 35), (36, 36), (37, 37), (38, 38), (39, 39), (40, 40), (41, 41), (42, 42), (43, 43), (44, 44), (45, 45), (46, 46), (47, 47), (48, 48), (49, 49), (50, 50), (51, 51), (52, 52), (53, 53), (54, 54), (55, 55), (56, 56), (57, 57), (58, 58), (59, 59), (60, 60), (61, 61), (62, 62), (63, 63), (64, 64), (65, 65), (66, 66), (67, 67), (68, 68), (69, 69), (70, 70), (71, 71), (72, 72), (73, 73), (74, 74), (75, 75), (76, 76), (77, 77), (78, 78), (79, 79), (80, 80), (81, 81), (82, 82), (83, 83), (84, 84), (85, 85), (86, 86), (87, 87), (88, 88), (89, 89), (90, 90), (91, 91), (92, 92), (93, 93), (94, 94), (95, 95), (96, 96), (97, 97), (98, 98), (99, 99), (100, 100), (101, 101), (102, 102), (103, 103), (104, 104), (105, 105), (106, 106), (107, 107), (108, 108), (109, 109), (110, 110), (111, 111), (112, 112), (113, 113), (114, 114), (115, 115), (116, 116), (117, 117), (118, 118), (119, 119), (120, 120), (121, 121), (122, 122), (123, 123), (124, 124), (125, 125), (126, 126), (128, 8226), (129, 8224), (130, 8225), (131, 8230), (132, 8212), (133, 8211), (134, 402), (135, 8260), (136, 8249), (137, 8250), (138, 8722), (139, 8240), (140, 8222), (141, 8220), (142, 8221), (143, 8216), (144, 8217), (145, 8218), (146, 8482), (147, 64257), (148, 64258), (149, 321), (150, 338), (151, 352), (152, 376), (153, 381), (154, 305), (155, 322), (156, 339), (157, 353), (158, 382), (160, 8364), (161, 161), (162, 162), (163, 163), (164, 164), (165, 165), (166, 166), (167, 167), (168, 168), (169, 169), (170, 170), (171, 171), (172, 172), (174, 174), (175, 175), (176, 176), (177, 177), (178, 178), (179, 179), (180, 180), (181, 181), (182, 182), (183, 183), (184, 184), (185, 185), (186, 186), (187, 187), (188, 188), (189, 189), (190, 190), (191, 191), (192, 192), (193, 193), (194, 194), (195, 195), (196, 196), (197, 197), (198, 198), (199, 199), (200, 200), (201, 201), (202, 202), (203, 203), (204, 204), (205, 205), (206, 206), (207, 207), (208, 208), (209, 209), (210, 210), (211, 211), (212, 212), (213, 213), (214, 214), (215, 215), (216, 216), (217, 217), (218, 218), (219, 219), (220, 220), (221, 221), (222, 222), (223, 223), (224, 224), (225, 225), (226, 226), (227, 227), (228, 228), (229, 229), (230, 230), (231, 231), (232, 232), (233, 233), (234, 234), (235, 235), (236, 236), (237, 237), (238, 238), (239, 239), (240, 240), (241, 241), (242, 242), (243, 243), (244, 244), (245, 245), (246, 246), (247, 247), (248, 248), (249, 249), (250, 250), (251, 251), (252, 252), (253, 253), (254, 254), (255, 255)];

/// 🧭 Decodes the next scalar from a stable PDF string without copying or scanning the remaining bytes.
pub struct PdfTextScalarCursor { encoding: Encoding, offset: usize }

enum Encoding { PdfDoc, Utf16, Utf8 }

impl PdfTextScalarCursor {
    pub fn text(bytes: &[u8]) -> Self {
        if bytes.starts_with(&[0xFE, 0xFF]) { Self { encoding: Encoding::Utf16, offset: 2 } }
        else if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) { Self { encoding: Encoding::Utf8, offset: 3 } }
        else { Self { encoding: Encoding::PdfDoc, offset: 0 } }
    }

    pub fn uri() -> Self { Self { encoding: Encoding::Utf8, offset: 0 } }

    pub fn next(&mut self, source: &[u8]) -> Option<char> {
        let bytes = source.get(self.offset..)?;
        let (scalar, length) = match self.encoding {
            Encoding::PdfDoc => {
                let byte = bytes.first()?;
                let scalar = PDF_DOC_ENCODING.binary_search_by_key(byte, |&(code, _)| code)
                    .ok().and_then(|index| char::from_u32(PDF_DOC_ENCODING[index].1)).unwrap_or(char::REPLACEMENT_CHARACTER);
                (scalar, 1)
            }
            Encoding::Utf16 => decode_utf16_scalar(bytes)?,
            Encoding::Utf8 => decode_utf8_scalar(bytes)?,
        };
        self.offset += length;
        Some(scalar)
    }
}

fn decode_utf16_scalar(bytes: &[u8]) -> Option<(char, usize)> {
    let unit = u16::from_be_bytes(bytes.get(..2)?.try_into().ok()?);
    if let Some(scalar) = char::from_u32(unit as u32) { return Some((scalar, 2)); }
    if let (0xD800..=0xDBFF, Some(next)) = (unit, bytes.get(2..4)) {
        let low = u16::from_be_bytes([next[0], next[1]]);
        if (0xDC00..=0xDFFF).contains(&low) {
            let scalar = 0x10000 + (((unit - 0xD800) as u32) << 10) + (low - 0xDC00) as u32;
            return char::from_u32(scalar).map(|scalar| (scalar, 4));
        }
    }
    Some((char::REPLACEMENT_CHARACTER, 2))
}

fn decode_utf8_scalar(bytes: &[u8]) -> Option<(char, usize)> {
    let first = *bytes.first()?;
    let width = match first { 0x00..=0x7F => return Some((first as char, 1)), 0xC2..=0xDF => 2, 0xE0..=0xEF => 3, 0xF0..=0xF4 => 4, _ => 1 };
    let prefix = &bytes[..width.min(bytes.len())];
    match std::str::from_utf8(prefix) {
        Ok(text) => text.chars().next().map(|scalar| (scalar, width)),
        Err(error) => Some((char::REPLACEMENT_CHARACTER, error.error_len().unwrap_or(prefix.len()))),
    }
}

/// 🔤 Decodes a PDF text string (§7.9.2.2): UTF-16BE with BOM, UTF-8 with BOM, else PDFDoc.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_text_string(bytes: &[u8]) -> String {
    let mut cursor = PdfTextScalarCursor::text(bytes);
    std::iter::from_fn(|| cursor.next(bytes)).collect()
}

/// 🔤 Encodes a text string: PDFDoc when every character is representable, else UTF-16BE + BOM.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_text_string(text: &str) -> Vec<u8> {
    static REVERSE: OnceLock<HashMap<u32, u8>> = OnceLock::new();
    let reverse = REVERSE.get_or_init(|| PDF_DOC_ENCODING.iter().map(|(code, value)| (*value, *code)).collect());
    let mut out = Vec::with_capacity(text.len());
    for character in text.chars() {
        match reverse.get(&(character as u32)) {
            Some(code) if character != '\u{FFFD}' => out.push(*code),
            _ => {
                let mut utf16 = vec![0xFE, 0xFF];
                for unit in text.encode_utf16() {
                    utf16.extend_from_slice(&unit.to_be_bytes());
                }
                return utf16;
            }
        }
    }
    out
}
//#endregion 🔤️TextString

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_scalar_step_matches_authored_document_encoding() {
        let fixture = include_str!("../../🧪️tests/🧫️fixtures/🔤️text-strings/📑️metadata.txt");
        for line in fixture.lines().filter(|line| !line.starts_with('#')) {
            let (input, output) = line.split_once(' ').unwrap();
            let bytes = |value: &str| value.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect::<Vec<_>>();
            let input = bytes(input);
            let mut stepper = PdfTextScalarCursor::text(&input);
            let mut decoded = String::new();
            while let Some(scalar) = stepper.next(&input) { decoded.push(scalar); }
            assert_eq!(decoded.as_bytes(), bytes(output));
        }
    }

    #[test]
    fn utf8_scalar_recovery_matches_standard_loss_replacement() {
        for first in 0..=255u8 {
            for second in 0..=255u8 {
                let bytes = [first, second];
                let mut cursor = PdfTextScalarCursor::uri();
                let decoded: String = std::iter::from_fn(|| cursor.next(&bytes)).collect();
                assert_eq!(decoded, String::from_utf8_lossy(&bytes));
            }
        }
    }
}
