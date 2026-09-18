use super::*;

fn parse(text: &str) -> PdfObject {
    Lexer::new(text.as_bytes()).parse_object().expect("parses")
}

#[test]
fn parses_every_cos_object_kind() {
    assert_eq!(parse("null"), PdfObject::Null);
    assert_eq!(parse("true"), PdfObject::Bool(true));
    assert_eq!(parse("-42"), PdfObject::Int(-42));
    assert_eq!(parse("3.5"), PdfObject::Real(PdfDecimal::parse("3.5").unwrap()));
    assert_eq!(parse("/A#20B"), PdfObject::Name("A B".into()));
    assert_eq!(parse("(a\\)b\\101\\\nc)"), PdfObject::Str(b"a)bAc".to_vec()));
    assert_eq!(parse("<48 6>"), PdfObject::Str(vec![0x48, 0x60]));
    assert_eq!(parse("[1 /X (s) 2 0 R]"), PdfObject::Array(vec![PdfObject::Int(1), PdfObject::Name("X".into()), PdfObject::Str(b"s".to_vec()), PdfObject::Ref(ObjRef { num: 2, gen: 0 })]));
    assert_eq!(parse("<< /A 1 /B [2] >>"), PdfObject::Dict(vec![PdfDictEntry::new("A", PdfObject::Int(1)), PdfDictEntry::new("B", PdfObject::Array(vec![PdfObject::Int(2)]))]));
}

#[test]
fn stream_with_wrong_length_recovers_by_endstream_search() {
    let object = parse("<< /Length 99 >>\nstream\nabc\nendstream");
    assert_eq!(object, PdfObject::Stream { dict: vec![PdfDictEntry::new("Length", PdfObject::Int(99))], data: b"abc".to_vec(), filters: Vec::new() });
}

#[test]
fn content_tokens_yield_keywords() {
    let mut lexer = Lexer::new(b"1 0 0 RG /F1 12 Tf (x) Tj");
    let mut keywords = Vec::new();
    loop {
        match lexer.next_token().unwrap() {
            Token::Keyword(word) => keywords.push(word),
            Token::End => break,
            Token::Object(_) => {}
        }
    }
    assert_eq!(keywords, vec!["RG", "Tf", "Tj"]);
}

#[test]
fn writer_and_lexer_are_inverse() {
    let object = PdfObject::Dict(vec![
        PdfDictEntry::new("Real", PdfObject::number(0.5)),
        PdfDictEntry::new("Int", PdfObject::Int(-7)),
        PdfDictEntry::new("Name", PdfObject::name("With Space")),
        PdfDictEntry::new("Str", PdfObject::Str(vec![0, 255, b'('])),
        PdfDictEntry::new("Ref", PdfObject::Ref(ObjRef { num: 12, gen: 3 })),
        PdfDictEntry::new("Arr", PdfObject::Array(vec![PdfObject::Bool(false), PdfObject::Null])),
    ]);
    let bytes = object_bytes(&object);
    assert_eq!(Lexer::new(&bytes).parse_object().unwrap(), object);
}

#[test]
fn number_text_is_exponent_free_and_round_trips() {
    for value in [0.0, -0.0, 1.0, 0.1, 1e-7, 123456.789, -2.5e3] {
        let text = number_text(value);
        assert!(!text.contains('e'), "{text}");
        assert_eq!(text.parse::<f64>().unwrap(), if value == 0.0 { 0.0 } else { value });
    }
    assert_eq!(number_text(1e-7), "0.0000001");
    assert_eq!(number_text(-0.0), "0");
}

#[test]
fn brute_force_scan_finds_objects_without_xref() {
    let data = b"%PDF-1.7\n1 0 obj\n<< /Type /Catalog >>\nendobj\n2 0 obj\n42\nendobj\n";
    let found = brute_force_scan(data);
    assert_eq!(found.len(), 2);
    assert_eq!(parse_indirect_at(data, found[&2].1).unwrap().1, PdfObject::Int(42));
}
