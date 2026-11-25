//! 🧪️ Exercises one-unit progress, operator custody, and retirement on native page content.

use crate::standards::v1_7::subsets::base::io::foreign_artifacts::NativePdfArtifactResources;
use crate::standards::v1_7::subsets::base::modules::content::{extract_text, parse_content, ContentProgramCursor, ContentProgramEvent as Event, ContentProgramReceipt, NoFonts};
use crate::standards::v1_7::subsets::base::modules::lexer::CosGrant;
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfOp, PdfPropertyList, PdfTextArrayItem, PdfTextString};

const PAGE: &[u8] = include_bytes!("🧪️content-program.txt");

#[derive(Default)]
struct Account {
    allocation: usize,
    release: usize,
    acquired: usize,
    released: usize,
    denied: [bool; 2],
}

impl Account {
    fn grant(&self) -> CosGrant {
        CosGrant { work: 1, allocation_bytes: self.allocation, release_bytes: self.release }
    }

    fn record(&mut self, turn: ContentProgramReceipt) -> Event {
        assert!(turn.work <= 1);
        assert!(turn.allocated_bytes <= self.allocation);
        assert!(turn.released_bytes <= self.release);
        self.acquired += turn.allocated_bytes;
        self.released += turn.released_bytes;
        let event = turn.event.unwrap();
        match event {
            Event::NeedsAllocation { bytes } => {
                assert!(bytes > 0);
                self.denied[0] = true;
                self.allocation = bytes;
            }
            Event::NeedsRelease { bytes } => {
                assert!(bytes > 0);
                self.denied[1] = true;
                self.release = bytes;
            }
            _ => {
                self.allocation = 0;
                self.release = 0;
            }
        }
        event
    }

    fn close(&mut self, cursor: &mut ContentProgramCursor) {
        cursor.begin_close();
        assert_eq!(cursor.close_step(CosGrant { work: 0, allocation_bytes: 0, release_bytes: usize::MAX }).work, 0);
        for _ in 0..10_000 {
            if self.record(cursor.close_step(self.grant())) == Event::Closed { return; }
        }
        panic!("retained content was not retired");
    }
}

fn transferred_bytes(op: &PdfOp) -> usize {
    match op {
        PdfOp::SetFont { name, .. } | PdfOp::SetFillColorSpace { name } => name.capacity(),
        PdfOp::BeginMarkedContentWithProperties { tag, properties: PdfPropertyList::Named { name } } | PdfOp::MarkedContentPointWithProperties { tag, properties: PdfPropertyList::Named { name } } => tag.capacity() + name.capacity(),
        PdfOp::SetDash { array, .. } | PdfOp::SetFillColor { components: array } => array.capacity() * std::mem::size_of::<f64>(),
        PdfOp::ShowText { text } | PdfOp::NextLineShowText { text } | PdfOp::NextLineShowTextSpaced { text, .. } => match text {
            PdfTextString::Text { text } => text.capacity(),
            PdfTextString::Codes { codes } => codes.capacity() * std::mem::size_of::<u32>(),
        },
        PdfOp::ShowTextArray { items } => items.capacity() * std::mem::size_of::<PdfTextArrayItem>() +
            items.iter().map(|item| match item {
                PdfTextArrayItem::Text { text } => text.capacity(),
                PdfTextArrayItem::Codes { codes } => codes.capacity() * std::mem::size_of::<u32>(),
                PdfTextArrayItem::Adjust { .. } => 0,
            }).sum::<usize>(),
        _ => 0,
    }
}

#[test]
fn content_program_matches_the_shared_pdf_page_in_one_unit_turns() {
    let mut resources = NativePdfArtifactResources::from_objects(&[]);
    let reference = parse_content(PAGE, &NoFonts, &mut resources).unwrap();
    let mut cursor = ContentProgramCursor::new(0);
    let mut account = Account::default();
    let mut operators = Vec::new();
    let mut complete = false;
    for _ in 0..10_000 {
        let event = account.record(cursor.advance(PAGE, &NoFonts, account.grant(), false));
        match event {
            Event::OperatorReady => {
                let op = cursor.take_operator().unwrap();
                if matches!(op, PdfOp::Restore) {
                    assert_eq!(cursor.selected_font(), Some("F1"));
                }
                assert!(cursor.take_operator().is_none());
                operators.push(op);
            }
            Event::Complete => {
                complete = true;
                break;
            }
            Event::More | Event::NeedsAllocation { .. } | Event::NeedsRelease { .. } => {}
            other => panic!("unexpected content progress: {other:?}"),
        }
    }
    assert!(complete);
    assert_eq!(operators, reference);
    assert_eq!(extract_text(&operators, &NoFonts), "Alpha\nBeta\nGamma\nDelta");
    assert!(operators.iter().any(|op| matches!(op, PdfOp::SetDash { array, phase } if array == &[6., 2.] && *phase == 1.)));
    assert!(operators.iter().any(|op| matches!(op, PdfOp::Rectangle { x: 10., y: 20., width: 20., height: 15. })));
    assert!(operators.iter().any(|op| matches!(op, PdfOp::ShowText { text: PdfTextString::Text { text } } if text == "Alpha")));
    account.close(&mut cursor);
    let transferred = operators.iter().map(transferred_bytes).sum::<usize>();
    assert_eq!(account.acquired, account.released + transferred);
    assert_eq!(account.denied, [true, true]);
}

#[test]
fn content_program_cancellation_retires_lexical_and_semantic_custody() {
    for offset in 0..320 {
        let source = b"/F1 12 Tf q /F2 8 Tf (alpha\\001beta) Tj [1 2] 2 d [(ab) 3 (c\\001)] TJ Q";
        let mut cursor = ContentProgramCursor::new(0);
        let mut account = Account::default();
        let mut transferred = 0;
        for _ in 0..offset {
            match account.record(cursor.advance(source, &NoFonts, account.grant(), false)) {
                Event::OperatorReady => {
                    transferred += transferred_bytes(&cursor.take_operator().unwrap());
                }
                Event::Complete => break,
                Event::More | Event::NeedsAllocation { .. } | Event::NeedsRelease { .. } => {}
                other => panic!("unexpected content progress at {offset}: {other:?}"),
            }
        }
        let event = account.record(cursor.advance(source, &NoFonts, account.grant(), true));
        assert!(matches!(event, Event::Cancelled | Event::Complete));
        account.close(&mut cursor);
        assert_eq!(account.acquired, account.released + transferred, "stop at turn {offset}");
    }
}

#[test]
fn content_program_retains_an_unbounded_operator_without_publishing_it() {
    let source = b"q /Escaped#20Font 12 Tf [/Type /Example] BI /W 2 /H 2";
    let mut cursor = ContentProgramCursor::new(0);
    let mut account = Account::default();
    let mut transferred = 0;
    let mut deferred = false;
    for _ in 0..10_000 {
        match account.record(cursor.advance(source, &NoFonts, account.grant(), false)) {
            Event::OperatorReady => {
                if let PdfOp::SetFont { name, .. } = cursor.take_operator().unwrap() { transferred = name.capacity(); }
            }
            Event::Deferred { start, end } => {
                assert_eq!(&source[start..end], b"BI");
                assert!(cursor.take_operator().is_none());
                deferred = true;
                break;
            }
            Event::More | Event::NeedsAllocation { .. } | Event::NeedsRelease { .. } => {}
            other => panic!("unexpected content progress: {other:?}"),
        }
    }
    assert!(deferred);
    account.close(&mut cursor);
    assert_eq!(account.acquired, account.released + transferred);
}


#[test]
fn ready_text_array_stays_owned_until_taken_or_cancelled() {
    let source = b"[(a) -25 (b\\001)] TJ";
    let mut cursor = ContentProgramCursor::new(0);
    let mut account = Account::default();
    let mut ready = false;
    for _ in 0..1_000 {
        match account.record(cursor.advance(source, &NoFonts, account.grant(), false)) {
            Event::OperatorReady => {
                ready = true;
                break;
            }
            Event::More | Event::NeedsAllocation { .. } | Event::NeedsRelease { .. } => {}
            other => panic!("unexpected content progress: {other:?}"),
        }
    }
    assert!(ready);
    assert_eq!(account.record(cursor.advance(source, &NoFonts, account.grant(), true)), Event::Cancelled);
    assert!(cursor.take_operator().is_none());
    account.close(&mut cursor);
    assert_eq!(account.acquired, account.released);
}


#[test]
fn controlled_decimal_operands_match_reference_and_defer_excess_spelling() {
    use crate::standards::v1_7::subsets::base::modules::content::bounded_number;
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfDecimal, PdfObject};

    for source in ["-.03125", "1000.", "+.000001", "+12.125"] {
        let value = PdfObject::Real(PdfDecimal::parse(source).unwrap());
        assert_eq!(bounded_number(&value).unwrap().map(f64::to_bits), value.as_f64().map(f64::to_bits));
    }

    let source = format!("[.5 .{}] 0 d", "1".repeat(129));
    let mut cursor = ContentProgramCursor::new(0);
    let mut account = Account::default();
    for _ in 0..4096 {
        match account.record(cursor.advance(source.as_bytes(), &NoFonts, account.grant(), false)) {
            Event::Deferred { start, end } => {
                assert_eq!(&source[start..end], "d");
                assert!(cursor.take_operator().is_none());
                account.close(&mut cursor);
                assert_eq!(account.acquired, account.released);
                return;
            }
            Event::More | Event::NeedsAllocation { .. } | Event::NeedsRelease { .. } => {}
            other => panic!("unexpected content progress: {other:?}"),
        }
    }
    panic!("oversized decimal did not leave its operator deferred");
}


#[test]
fn standard_type1_program_preserves_roundtrip_text_and_alias_codes() {
    use crate::standards::v1_7::subsets::base::modules::{content::FontTable, fonts::{controlled_type1_text, FontCodec}};
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfBaseEncoding, PdfFont, PdfFontKind, PdfSimpleEncoding};

    struct Fonts(PdfBaseEncoding, FontCodec);
    impl FontTable for Fonts {
        fn font(&self, name: &str) -> Option<&FontCodec> { (name == "F1").then_some(&self.1) }
        fn controlled_type1(&self, name: &str) -> Option<PdfBaseEncoding> { (name == "F1").then_some(self.0) }
    }

    let source = b"BT /F1 12 Tf <636166e9> Tj [<80> -20 <a0>] TJ ET";
    let mut resources = NativePdfArtifactResources::from_objects(&[]);
    for encoding in [PdfBaseEncoding::Standard, PdfBaseEncoding::WinAnsi] {
        let font = PdfFont {
            id: "F1".into(),
            kind: PdfFontKind::Type1 {
                base_font: "Helvetica".into(), encoding: PdfSimpleEncoding { base: Some(encoding), differences: Vec::new() },
                first_char: 0, widths: Vec::new(), descriptor: None, program: None,
            },
            to_unicode: None, extra: Vec::new(),
        };
        let codec = FontCodec::new(&font, &resources).unwrap();
        for byte in 0..=u8::MAX {
            let reference = codec.decode_text(&[byte]).filter(|text| codec.encode(text).as_deref() == Some(&[byte]));
            assert_eq!(controlled_type1_text(encoding, byte), reference.as_deref(), "{encoding:?} {byte}");
        }
        let fonts = Fonts(encoding, codec);
        let expected = parse_content(source, &fonts, &mut resources).unwrap();
        let mut cursor = ContentProgramCursor::new(0);
        let mut account = Account::default();
        let mut operators = Vec::new();
        loop {
            match account.record(cursor.advance(source, &fonts, account.grant(), false)) {
                Event::OperatorReady => operators.push(cursor.take_operator().unwrap()),
                Event::Complete => break,
                Event::More | Event::NeedsAllocation { .. } | Event::NeedsRelease { .. } => {}
                event => panic!("unexpected Type1 text state {event:?}"),
            }
        }
        assert_eq!(operators, expected);
        if encoding == PdfBaseEncoding::WinAnsi {
            assert!(matches!(&operators[2], PdfOp::ShowText { text: PdfTextString::Text { text } } if text == "café"));
            assert!(matches!(&operators[3], PdfOp::ShowTextArray { items } if items == &[
                PdfTextArrayItem::Text { text: "€".into() }, PdfTextArrayItem::Adjust { amount: -20. },
                PdfTextArrayItem::Codes { codes: vec![160] },
            ]));
        }
        account.close(&mut cursor);
        assert_eq!(account.acquired, account.released + operators.iter().map(transferred_bytes).sum::<usize>());
    }
}

#[test]
fn simple_type1_differences_and_mac_roman_decode_like_the_reference_codec() {
    use crate::standards::v1_7::subsets::base::modules::fonts::{controlled_simple_text, FontCodec};
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfBaseEncoding, PdfEncodingDifference, PdfFont, PdfFontKind, PdfSimpleEncoding};

    let resources = NativePdfArtifactResources::from_objects(&[]);
    let cases: [(PdfBaseEncoding, &[(u32, &str)]); 6] = [
        (PdfBaseEncoding::MacRoman, &[]),
        (PdfBaseEncoding::Standard, &[(65, "eacute"), (66, "Euro")]),
        (PdfBaseEncoding::WinAnsi, &[(128, "bullet"), (233, "e")]),
        (PdfBaseEncoding::WinAnsi, &[(97, "b"), (98, "a")]),
        (PdfBaseEncoding::MacRoman, &[(36, "at"), (233, "udieresis")]),
        (PdfBaseEncoding::WinAnsi, &[(200, "eacute"), (201, "eacute")]),
    ];
    for (base, authored) in cases {
        let differences: Vec<PdfEncodingDifference> = authored.iter().map(|(code, glyph)| PdfEncodingDifference { code: *code, glyph: (*glyph).into() }).collect();
        let font = PdfFont {
            id: "F1".into(),
            kind: PdfFontKind::Type1 {
                base_font: "Helvetica".into(), encoding: PdfSimpleEncoding { base: Some(base), differences: differences.clone() },
                first_char: 0, widths: Vec::new(), descriptor: None, program: None,
            },
            to_unicode: None, extra: Vec::new(),
        };
        let codec = FontCodec::new(&font, &resources).unwrap();
        for byte in 0..=u8::MAX {
            let reference = codec.decode_text(&[byte]).filter(|text| codec.encode(text).as_deref() == Some(&[byte]));
            assert_eq!(controlled_simple_text(base, &differences, byte), reference.as_deref(), "{base:?} {authored:?} {byte}");
        }
    }
}

#[test]
fn codec_fonts_decode_text_and_codes_like_the_reference_parser_in_one_unit_turns() {
    use crate::standards::v1_7::subsets::base::modules::{content::FontTable, fonts::FontCodec};
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfBaseEncoding, PdfFont, PdfFontKind, PdfSimpleEncoding, PdfToUnicode, PdfToUnicodeMapping};

    struct Fonts(FontCodec);
    impl FontTable for Fonts {
        fn font(&self, name: &str) -> Option<&FontCodec> { (name == "F1").then_some(&self.0) }
    }

    let source = b"/OC /oc1 BDC /Span /MCID0 DP BT /F1 12 Tf (Hello) Tj [(W) 120 (orld) -50 <8081>] TJ (\\351t\\351) ' 2 0 (x\\001) \" /F2 9 Tf (plain) Tj ET EMC";
    let mut resources = NativePdfArtifactResources::from_objects(&[]);
    let font = PdfFont {
        id: "F1".into(),
        kind: PdfFontKind::TrueType {
            base_font: "Arial".into(), encoding: PdfSimpleEncoding { base: Some(PdfBaseEncoding::WinAnsi), differences: Vec::new() },
            first_char: 0, widths: Vec::new(), descriptor: None, program: None,
        },
        to_unicode: Some(PdfToUnicode { byte_width: 1, mappings: vec![PdfToUnicodeMapping::Char { code: 0x81, text: "Ω".into() }] }),
        extra: Vec::new(),
    };
    let fonts = Fonts(FontCodec::new(&font, &resources).unwrap());
    let expected = parse_content(source, &fonts, &mut resources).unwrap();
    let mut cursor = ContentProgramCursor::new(0);
    let mut account = Account::default();
    let mut operators = Vec::new();
    loop {
        match account.record(cursor.advance(source, &fonts, account.grant(), false)) {
            Event::OperatorReady => operators.push(cursor.take_operator().unwrap()),
            Event::Complete => break,
            Event::More | Event::NeedsAllocation { .. } | Event::NeedsRelease { .. } => {}
            event => panic!("unexpected codec text state {event:?}"),
        }
    }
    assert_eq!(operators, expected);
    assert!(operators.iter().any(|op| matches!(op, PdfOp::ShowText { text: PdfTextString::Text { text } } if text == "Hello")));
    assert!(operators.iter().any(|op| matches!(op, PdfOp::ShowTextArray { items } if items.iter().any(|item| matches!(item, PdfTextArrayItem::Text { text } if text.ends_with('Ω'))))));
    assert!(account.denied[0], "every decoded string was quoted before it was kept");
    assert!(matches!(&operators[0], PdfOp::BeginMarkedContentWithProperties { tag, properties: PdfPropertyList::Named { name } } if tag == "OC" && name == "oc1"), "a named property list stays on the controlled path");
    account.close(&mut cursor);
    assert_eq!(account.acquired, account.released + operators.iter().map(transferred_bytes).sum::<usize>());
}
