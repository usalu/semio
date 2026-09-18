use super::*;

fn sample_font() -> Vec<u8> {
    let square = vec![vec![(50, 0), (50, 700), (550, 700), (550, 0)]];
    let bar = vec![vec![(0, 300), (0, 400), (600, 400), (600, 300)]];
    synthesize_truetype(1000, &[('A' as u32, 600, square), ('-' as u32, 600, bar), ('é' as u32, 500, Vec::new())])
}

#[test]
fn synthesized_font_parses_with_metrics_and_cmap() {
    let font = TrueTypeFont::parse(&sample_font()).unwrap();
    assert_eq!(font.num_glyphs, 4);
    assert_eq!(font.units_per_em, 1000);
    assert_eq!(font.glyph_for_char('A'), Some(1));
    assert_eq!(font.glyph_for_char('-'), Some(2));
    assert_eq!(font.glyph_for_char('é'), Some(3));
    assert_eq!(font.glyph_for_char('Z'), None);
    assert_eq!(font.advance_1000(1), 600.0);
    assert_eq!(font.advance_1000(3), 500.0);
    assert!(font.component_glyphs(1).is_empty());
}

#[test]
fn subset_keeps_glyph_ids_and_drops_outlines() {
    let font = TrueTypeFont::parse(&sample_font()).unwrap();
    let subset_bytes = font.subset(&[2u16].into_iter().collect());
    let subset = TrueTypeFont::parse(&subset_bytes).unwrap();
    assert_eq!(subset.num_glyphs, 4);
    assert_eq!(subset.glyph_for_char('-'), Some(2));
    assert_eq!(subset.glyph_location(1), Some((0, 0)).map(|_| subset.glyph_location(1).unwrap()));
    let (start, end) = subset.glyph_location(1).unwrap();
    assert_eq!(start, end, "glyph 1 was emptied");
    let (start, end) = subset.glyph_location(2).unwrap();
    assert!(end > start, "glyph 2 kept its outline");
    assert!(subset_bytes.len() < sample_font().len() + 64);
}

#[test]
fn table_checksums_are_directory_consistent() {
    let bytes = sample_font();
    let count = u16::from_be_bytes([bytes[4], bytes[5]]) as usize;
    for index in 0..count {
        let record = 12 + index * 16;
        let tag = std::str::from_utf8(&bytes[record..record + 4]).unwrap();
        let checksum = u32::from_be_bytes(bytes[record + 4..record + 8].try_into().unwrap());
        let offset = u32::from_be_bytes(bytes[record + 8..record + 12].try_into().unwrap()) as usize;
        let length = u32::from_be_bytes(bytes[record + 12..record + 16].try_into().unwrap()) as usize;
        let mut padded = bytes[offset..offset + length].to_vec();
        while padded.len() % 4 != 0 {
            padded.push(0);
        }
        if tag != "head" {
            assert_eq!(table_checksum(&padded), checksum, "{tag}");
        }
    }
}
