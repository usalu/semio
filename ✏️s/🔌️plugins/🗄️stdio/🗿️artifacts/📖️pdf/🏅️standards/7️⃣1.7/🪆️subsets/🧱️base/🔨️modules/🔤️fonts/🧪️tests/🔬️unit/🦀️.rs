use super::*;
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfCidFont, PdfCidWidthRun, PdfEncodingDifference, PdfFontDescriptor};

#[test]
fn glyph_list_covers_names_programmatic_forms_and_ligatures() {
    assert_eq!(glyph_name_to_unicode("A").as_deref(), Some("A"));
    assert_eq!(glyph_name_to_unicode("germandbls").as_deref(), Some("ß"));
    assert_eq!(glyph_name_to_unicode("uni00E9").as_deref(), Some("é"));
    assert_eq!(glyph_name_to_unicode("u1F600").as_deref(), Some("😀"));
    assert_eq!(glyph_name_to_unicode("f_i").as_deref(), Some("fi"));
    assert_eq!(glyph_name_to_unicode("a.sc").as_deref(), Some("a"));
    assert_eq!(glyph_name_to_unicode("g123"), None);
    assert_eq!(unicode_to_glyph_name('é'), "eacute");
    assert_eq!(unicode_to_glyph_name('A'), "A");
    assert_eq!(unicode_to_glyph_name('\u{2022}'), "bullet");
}

#[test]
fn text_strings_round_trip_pdfdoc_and_utf16() {
    for text in ["Hello", "Straße – „quoted“", "日本語 😀"] {
        assert_eq!(decode_text_string(&encode_text_string(text)), text);
    }
    assert_eq!(encode_text_string("plain"), b"plain");
    assert_eq!(encode_text_string("é").len(), 1);
    assert_eq!(encode_text_string("日")[..2], [0xFE, 0xFF]);
}

#[test]
fn standard_fonts_have_afm_widths_and_aliases_fold() {
    let helvetica = standard_font("Helvetica").unwrap();
    assert_eq!(helvetica.width("A"), Some(667.0));
    assert_eq!(helvetica.width("space"), Some(278.0));
    assert_eq!(standard_font_name("ABCDEF+Arial-BoldMT"), Some("Helvetica-Bold"));
    assert_eq!(standard_font_name("TimesNewRomanPS-BoldItalicMT"), Some("Times-BoldItalic"));
    assert_eq!(standard_font_name("CourierNew"), Some("Courier"));
    assert_eq!(standard_font_name("Wingdings"), None);
    assert_eq!(standard_font("Courier").unwrap().width("W"), Some(600.0));
    assert!(standard_font("Symbol").unwrap().builtin_encoding.iter().any(|(code, name)| *code == 97 && *name == "alpha"));
}

#[test]
fn simple_font_codec_encodes_and_decodes_with_widths() {
    let font = PdfFont::standard("F1", "Helvetica");
    let codec = FontCodec::new(&font);
    assert!(!codec.is_composite());
    let bytes = codec.encode("Aé").unwrap();
    assert_eq!(bytes, vec![0x41, 0xE9]);
    let glyphs = codec.decode(&bytes);
    assert_eq!(glyphs[0].text.as_deref(), Some("A"));
    assert_eq!(glyphs[0].width, 667.0);
    assert_eq!(glyphs[1].text.as_deref(), Some("é"));
    assert_eq!(codec.decode_text(&bytes).as_deref(), Some("Aé"));
    assert_eq!(codec.text_width("AA"), Some(1334.0));
    assert_eq!(codec.encode("日"), None);
}

#[test]
fn differences_override_the_base_encoding() {
    let mut font = PdfFont::standard("F1", "Helvetica");
    if let PdfFontKind::Type1 { encoding, .. } = &mut font.kind {
        encoding.differences.push(PdfEncodingDifference { code: 0x41, glyph: "bullet".into() });
    }
    let codec = FontCodec::new(&font);
    assert_eq!(codec.decode_text(&[0x41]).as_deref(), Some("•"));
    assert_eq!(codec.encode("•"), Some(vec![0x41]));
}

#[test]
fn composite_font_codec_maps_through_to_unicode_and_cid_widths() {
    let font = PdfFont {
        id: "F0".into(),
        kind: PdfFontKind::Type0 { base_font: "Sans".into(), cmap: PdfCMap::identity_h(), descendant: PdfCidFont { true_type: true, base_font: "Sans".into(), system_info: Default::default(), descriptor: PdfFontDescriptor { font_name: "Sans".into(), ..Default::default() }, default_width: 500.0, widths: vec![PdfCidWidthRun { start_cid: 36, widths: vec![700.0, 710.0] }], default_vertical: None, vertical_metrics: Vec::new(), cid_to_gid: None, program: None, extra: Vec::new() } },
        to_unicode: Some(PdfToUnicode { byte_width: 2, mappings: vec![PdfToUnicodeMapping::Range { low: 36, high: 37, text: "A".into() }] }),
        extra: Vec::new(),
    };
    let codec = FontCodec::new(&font);
    assert!(codec.is_composite());
    assert_eq!(codec.encode("AB"), Some(vec![0, 36, 0, 37]));
    let glyphs = codec.decode(&[0, 36, 0, 37, 0, 99]);
    assert_eq!(glyphs.iter().map(|g| g.width).collect::<Vec<_>>(), vec![700.0, 710.0, 500.0]);
    assert_eq!(glyphs[1].text.as_deref(), Some("B"));
    assert_eq!(glyphs[2].text, None);
    assert_eq!(glyphs[0].glyph_id, Some(36));
}

#[test]
fn composite_font_without_to_unicode_encodes_through_the_embedded_cmap() {
    let program = truetype::synthesize_truetype(1000, &[('A' as u32, 600, vec![vec![(0, 0), (0, 700), (500, 700)]]), ('B' as u32, 650, Vec::new())]);
    let ttf = TrueTypeFont::parse(&program).unwrap();
    let font = PdfFont {
        id: "F0".into(),
        kind: PdfFontKind::Type0 { base_font: "Synth".into(), cmap: PdfCMap::identity_h(), descendant: PdfCidFont { true_type: true, base_font: "Synth".into(), system_info: Default::default(), descriptor: PdfFontDescriptor { font_name: "Synth".into(), ..Default::default() }, default_width: 1000.0, widths: vec![PdfCidWidthRun { start_cid: 1, widths: vec![600.0, 650.0] }], default_vertical: None, vertical_metrics: Vec::new(), cid_to_gid: None, program: Some(PdfFontProgram::TrueType { data: program }), extra: Vec::new() } },
        to_unicode: None,
        extra: Vec::new(),
    };
    let codec = FontCodec::new(&font);
    assert_eq!(codec.encode("BA"), Some(vec![0, ttf.glyph_for_char('B').unwrap() as u8, 0, 1]));
    assert_eq!(codec.decode_text(&[0, 1]).as_deref(), Some("A"));
    assert_eq!(codec.text_width("AB"), Some(1250.0));
}

#[test]
fn type1_cleartext_encoding_is_read() {
    let program = b"%!PS-AdobeFont-1.0\n/Encoding 256 array\n0 1 255 {1 index exch /.notdef put} for\ndup 65 /alpha put\ndup 66 /beta put\nreadonly def\ncurrentdict end\ncurrentfile eexec\n\x00\x01";
    let encoding = type1_builtin_encoding(program).unwrap();
    assert_eq!(encoding, vec![(65, "alpha".to_string()), (66, "beta".to_string())]);
}
