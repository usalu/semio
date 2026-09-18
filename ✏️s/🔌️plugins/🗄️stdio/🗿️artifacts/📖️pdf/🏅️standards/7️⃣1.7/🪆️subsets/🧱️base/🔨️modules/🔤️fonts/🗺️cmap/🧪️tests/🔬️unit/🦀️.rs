use super::*;

#[test]
fn to_unicode_round_trips_through_print_and_parse() {
    let map = PdfToUnicode { byte_width: 2, mappings: vec![PdfToUnicodeMapping::Char { code: 3, text: "A".into() }, PdfToUnicodeMapping::Char { code: 4, text: "ﬁ".into() }, PdfToUnicodeMapping::Range { low: 10, high: 20, text: "a".into() }] };
    let printed = print_to_unicode(&map);
    assert_eq!(parse_to_unicode(&printed), map);
    assert_eq!(to_unicode_lookup(&map, 12), Some("c".into()));
    assert_eq!(to_unicode_reverse(&map, "c"), Some(12));
    assert_eq!(to_unicode_reverse(&map, "ﬁ"), Some(4));
    assert_eq!(to_unicode_lookup(&map, 99), None);
}

#[test]
fn to_unicode_array_form_and_glyph_names_parse() {
    let body = b"1 begincodespacerange <00> <FF> endcodespacerange 1 beginbfrange <41> <43> [<0061> <0062> <0063>] endbfrange 1 beginbfchar <44> /eacute endbfchar";
    let map = parse_to_unicode(body);
    assert_eq!(map.byte_width, 1);
    assert_eq!(to_unicode_lookup(&map, 0x42), Some("b".into()));
    assert_eq!(to_unicode_lookup(&map, 0x44), Some("é".into()));
}

#[test]
fn embedded_cmap_round_trips_and_splits_mixed_widths() {
    let cmap = PdfEmbeddedCMap {
        name: "Custom".into(),
        vertical: false,
        codespace: vec![PdfCodespaceRange { byte_width: 1, low: 0x00, high: 0x80 }, PdfCodespaceRange { byte_width: 2, low: 0x8140, high: 0x9FFC }],
        mappings: vec![PdfCidMapping::Range { low: 0x20, high: 0x7E, cid: 1 }, PdfCidMapping::Char { code: 0x8140, cid: 633 }],
        use_cmap: None,
    };
    let printed = print_cmap(&cmap);
    assert_eq!(parse_cmap(&printed, "x"), cmap);
    let codec = CidCodec::embedded(&cmap);
    assert_eq!(codec.split(&[0x41, 0x81, 0x40, 0x42]), vec![(0x41, 1), (0x8140, 2), (0x42, 1)]);
    assert_eq!(codec.cid(0x41), 34);
    assert_eq!(codec.cid(0x8140), 633);
    assert_eq!(codec.code_for_cid(633), Some((0x8140, 2)));
    let identity = CidCodec::identity(false);
    assert_eq!(identity.split(&[0, 5, 1, 2]), vec![(5, 2), (258, 2)]);
    assert_eq!(identity.cid(258), 258);
}
