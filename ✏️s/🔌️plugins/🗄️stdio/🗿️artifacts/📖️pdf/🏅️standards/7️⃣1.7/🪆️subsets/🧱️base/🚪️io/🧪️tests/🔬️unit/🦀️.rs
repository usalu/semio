use super::*;
use crate::standards::v1_7::subsets::base::modules::fonts::truetype::synthesize_truetype;

const THESIS: &[u8] = include_bytes!("../../../🖼️assets/🎓️bachelor-thesis/🎓️bachelor-thesis.pdf");
const REPORT_STRIP: &[u8] = include_bytes!("../../../🧫️fixtures/📊️report-strip/📊️report-strip.pdf");

fn typed_lanes(snapshot: &PdfSnapshot) -> PdfSnapshot {
    PdfSnapshot { objects: Vec::new(), trailer: Vec::new(), ..snapshot.clone() }
}

fn json_diff(path: &str, a: &serde_json::Value, b: &serde_json::Value, out: &mut Vec<String>) {
    use serde_json::Value;
    match (a, b) {
        (Value::Object(x), Value::Object(y)) => {
            for key in x.keys().chain(y.keys().filter(|k| !x.contains_key(*k))) {
                match (x.get(key), y.get(key)) {
                    (Some(av), Some(bv)) => json_diff(&format!("{path}.{key}"), av, bv, out),
                    (Some(av), None) => out.push(format!("{path}.{key}: only left = {}", truncate(&av.to_string()))),
                    (None, Some(bv)) => out.push(format!("{path}.{key}: only right = {}", truncate(&bv.to_string()))),
                    _ => {}
                }
            }
        }
        (Value::Array(x), Value::Array(y)) => {
            if x.len() != y.len() {
                out.push(format!("{path}: length {} vs {}", x.len(), y.len()));
            }
            for (index, (av, bv)) in x.iter().zip(y.iter()).enumerate() {
                json_diff(&format!("{path}[{index}]"), av, bv, out);
            }
        }
        _ => {
            if a != b {
                out.push(format!("{path}: {} vs {}", truncate(&a.to_string()), truncate(&b.to_string())));
            }
        }
    }
}

fn truncate(text: &str) -> String {
    text.chars().take(300).collect()
}

/// 🔍 Asserts two snapshots' typed lanes agree, reporting the first differences by path.
fn assert_same_lanes(left: &PdfSnapshot, right: &PdfSnapshot) {
    use pack::value::ToValue;
    let a: serde_json::Value = typed_lanes(left).to_value().into();
    let b: serde_json::Value = typed_lanes(right).to_value().into();
    let mut out = Vec::new();
    json_diff("$", &a, &b, &mut out);
    assert!(out.is_empty(), "typed lanes differ:\n{}", out.iter().take(12).cloned().collect::<Vec<_>>().join("\n"));
}

fn rich_document() -> PdfSnapshot {
    let program = synthesize_truetype(1000, &[('A' as u32, 600, vec![vec![(50, 0), (50, 700), (550, 700), (550, 0)]]), ('b' as u32, 550, vec![vec![(0, 0), (0, 500), (500, 500)]]), (' ' as u32, 250, Vec::new())]);
    let mut snapshot = text_document(&[(300.0, 400.0, "Hello Semio — a typed PDF")]);
    snapshot.declared_version = "1.7".into();
    snapshot.fonts.push(embedded_true_type_font("Synth", &program, "SynthSans", Some("Ab A")).unwrap());
    snapshot.images.push(PdfImage::rgb8("Im1", 2, 2, vec![255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 0]));
    let mut mask = PdfImage::gray8("Im1SMask", 2, 2, vec![255, 128, 64, 0]);
    mask.decode = vec![1.0, 0.0];
    snapshot.images.push(mask);
    snapshot.images[0].soft_mask = Some("Im1SMask".into());
    snapshot.images.push(PdfImage::jpeg("Photo", 1, 1, PdfColorSpace::DeviceRgb, vec![0xFF, 0xD8, 0xFF, 0xD9]));
    snapshot.forms.push(PdfFormXObject::new("Fm1", [0.0, 0.0, 50.0, 50.0], vec![PdfOp::SetFillRgb { r: 0.0, g: 0.5, b: 1.0 }, PdfOp::Rectangle { x: 0.0, y: 0.0, width: 50.0, height: 50.0 }, PdfOp::Fill]));
    snapshot.ext_g_states.push(PdfExtGState { id: "GS1".into(), fill_alpha: Some(0.5), stroke_alpha: Some(0.75), blend_mode: Some(vec!["Multiply".into()]), soft_mask: Some(PdfSoftMask::Luminosity { group: "Fm1".into(), backdrop: Some(vec![0.0]), transfer: None }), dash: Some((vec![3.0, 1.0], 0.5)), ..PdfExtGState::default() });
    snapshot.shadings.push(PdfShading { id: "Sh1".into(), color_space: PdfColorSpace::DeviceRgb, kind: PdfShadingKind::Axial { coords: [0.0, 0.0, 100.0, 0.0], domain: None, function: PdfFunction::Exponential { domain: vec![0.0, 1.0], range: None, c0: vec![1.0, 0.0, 0.0], c1: vec![0.0, 0.0, 1.0], n: 1.0 }, extend: [true, true] }, background: None, bbox: None, anti_alias: false, extra: Vec::new() });
    snapshot.shadings.push(PdfShading { id: "Sh2".into(), color_space: PdfColorSpace::DeviceGray, kind: PdfShadingKind::Radial { coords: [50.0, 50.0, 0.0, 50.0, 50.0, 40.0], domain: Some([0.0, 1.0]), function: PdfFunction::Stitching { domain: vec![0.0, 1.0], range: None, functions: vec![PdfFunction::Exponential { domain: vec![0.0, 1.0], range: None, c0: vec![0.0], c1: vec![1.0], n: 1.0 }, PdfFunction::Sampled { domain: vec![0.0, 1.0], range: vec![0.0, 1.0], size: vec![2], bits_per_sample: 8, order: None, encode: None, decode: None, samples: vec![255, 0] }], bounds: vec![0.5], encode: vec![0.0, 1.0, 0.0, 1.0] }, extend: [false, false] }, background: None, bbox: Some([0.0, 0.0, 100.0, 100.0]), anti_alias: true, extra: Vec::new() });
    snapshot.patterns.push(PdfPattern { id: "P1".into(), matrix: [1.0, 0.0, 0.0, 1.0, 10.0, 10.0], kind: PdfPatternKind::Shading { shading: "Sh1".into(), ext_g_state: Some("GS1".into()) }, extra: Vec::new() });
    snapshot.patterns.push(PdfPattern { id: "P2".into(), matrix: PDF_IDENTITY_MATRIX, kind: PdfPatternKind::Tiling { paint_type: 1, tiling_type: 1, bbox: [0.0, 0.0, 10.0, 10.0], x_step: 10.0, y_step: 10.0, content: vec![PdfOp::SetFillGray { gray: 0.5 }, PdfOp::Rectangle { x: 0.0, y: 0.0, width: 5.0, height: 5.0 }, PdfOp::Fill] }, extra: Vec::new() });
    snapshot.color_spaces.push(PdfNamedColorSpace { name: "CS0".into(), color_space: PdfColorSpace::Separation { name: "Spot".into(), alternate: Box::new(PdfColorSpace::DeviceCmyk), tint_transform: PdfFunction::Exponential { domain: vec![0.0, 1.0], range: None, c0: vec![0.0, 0.0, 0.0, 0.0], c1: vec![0.0, 1.0, 1.0, 0.0], n: 1.0 } } });
    snapshot.color_spaces.push(PdfNamedColorSpace { name: "CS1".into(), color_space: PdfColorSpace::Indexed { base: Box::new(PdfColorSpace::DeviceRgb), hival: 1, lookup: vec![0, 0, 0, 255, 255, 255] } });
    snapshot.properties.push(PdfNamedProperties { name: "MC0".into(), entries: vec![PdfDictEntry::new("MCID", PdfObject::Int(3))] });
    let page = &mut snapshot.pages[0];
    page.crop_box = Some([5.0, 5.0, 295.0, 395.0]);
    page.rotate = 90;
    page.user_unit = Some(2.0);
    page.content.extend(vec![
        PdfOp::Save,
        PdfOp::SetExtGState { name: "GS1".into() },
        PdfOp::Transform { matrix: [1.0, 0.0, 0.0, 1.0, 20.0, 20.0] },
        PdfOp::SetLineWidth { width: 2.5 },
        PdfOp::SetLineCap { cap: PdfLineCap::Round },
        PdfOp::SetLineJoin { join: PdfLineJoin::Bevel },
        PdfOp::SetDash { array: vec![4.0, 2.0], phase: 1.0 },
        PdfOp::MoveTo { x: 0.0, y: 0.0 },
        PdfOp::LineTo { x: 10.0, y: 0.0 },
        PdfOp::CurveTo { x1: 12.0, y1: 3.0, x2: 14.0, y2: 6.0, x3: 20.0, y3: 8.5 },
        PdfOp::CurveToInitial { x2: 25.0, y2: 9.0, x3: 30.0, y3: 10.0 },
        PdfOp::CurveToFinal { x1: 35.0, y1: 11.0, x3: 40.0, y3: 12.0 },
        PdfOp::ClosePath,
        PdfOp::SetStrokeRgb { r: 0.1, g: 0.2, b: 0.3 },
        PdfOp::SetFillCmyk { c: 0.0, m: 0.1, y: 0.2, k: 0.3 },
        PdfOp::FillStrokeEvenOdd,
        PdfOp::SetFillColorSpace { name: "Pattern".into() },
        PdfOp::SetFillColorN { components: Vec::new(), pattern: Some("P1".into()) },
        PdfOp::Rectangle { x: 0.0, y: 0.0, width: 100.0, height: 20.0 },
        PdfOp::Fill,
        PdfOp::SetFillColorSpace { name: "CS0".into() },
        PdfOp::SetFillColorN { components: vec![0.75], pattern: None },
        PdfOp::SetStrokeColorSpace { name: "CS1".into() },
        PdfOp::SetStrokeColor { components: vec![1.0] },
        PdfOp::Rectangle { x: 0.0, y: 30.0, width: 10.0, height: 10.0 },
        PdfOp::ClipEvenOdd,
        PdfOp::EndPath,
        PdfOp::PaintShading { name: "Sh2".into() },
        PdfOp::Transform { matrix: [40.0, 0.0, 0.0, 30.0, 100.0, 100.0] },
        PdfOp::PaintXObject { name: "Im1".into() },
        PdfOp::PaintXObject { name: "Fm1".into() },
        PdfOp::InlineImage { image: PdfInlineImage { width: 2, height: 1, bits_per_component: 8, color_space: Some(PdfColorSpace::DeviceGray), image_mask: false, decode: Vec::new(), interpolate: false, filters: Vec::new(), data: vec![0, 255], extra: Vec::new() } },
        PdfOp::BeginMarkedContentWithProperties { tag: "Span".into(), properties: PdfPropertyList::Named { name: "MC0".into() } },
        PdfOp::BeginText,
        PdfOp::SetFont { name: "Synth".into(), size: 14.0 },
        PdfOp::SetTextRenderingMode { mode: 2 },
        PdfOp::SetCharSpacing { spacing: 0.5 },
        PdfOp::SetWordSpacing { spacing: 1.0 },
        PdfOp::SetHorizontalScale { scale: 110.0 },
        PdfOp::SetTextRise { rise: 1.5 },
        PdfOp::SetTextMatrix { matrix: [1.0, 0.0, 0.0, 1.0, 50.0, 60.0] },
        PdfOp::ShowText { text: PdfTextString::text("Ab A") },
        PdfOp::ShowTextArray { items: vec![PdfTextArrayItem::Text { text: "A".into() }, PdfTextArrayItem::Adjust { amount: -120.0 }, PdfTextArrayItem::Text { text: "b".into() }] },
        PdfOp::NextLineShowTextSpaced { word_spacing: 2.0, char_spacing: 0.25, text: PdfTextString::text("A") },
        PdfOp::ShowText { text: PdfTextString::Codes { bytes: vec![0, 99] } },
        PdfOp::EndText,
        PdfOp::EndMarkedContent,
        PdfOp::Restore,
        PdfOp::Unknown { operator: "zz".into(), operands: vec![PdfObject::Int(1), PdfObject::name("X")] },
    ]);
    page.annotations.push(PdfAnnotation::link([10.0, 10.0, 100.0, 30.0], "https://semio-tech.com"));
    let mut note = PdfAnnotation::new([200.0, 300.0, 220.0, 320.0], PdfAnnotationKind::Text { open: true, icon: Some("Comment".into()), state: None, state_model: None });
    note.contents = Some("A note — with dash".into());
    note.color = vec![1.0, 1.0, 0.0];
    note.markup = Some(PdfMarkupAnnotation { title: Some("semio".into()), popup: Some(2), opacity: Some(0.8), creation_date: Some(PdfDate { year: 2026, month: 9, day: 18, hour: 1, minute: 2, second: 3, offset_minutes: Some(60) }), subject: Some("Subject".into()), ..PdfMarkupAnnotation::default() });
    note.appearance = Some(PdfAppearance { normal: PdfAppearanceEntry::Single { form: "Fm1".into() }, rollover: None, down: Some(PdfAppearanceEntry::States { states: vec![PdfAppearanceState { state: "On".into(), form: "Fm1".into() }] }) });
    note.appearance_state = Some("On".into());
    page.annotations.push(note);
    page.annotations.push(PdfAnnotation::new([200.0, 200.0, 300.0, 300.0], PdfAnnotationKind::Popup { parent: Some(1), open: false }));
    let mut highlight = PdfAnnotation::new([0.0, 0.0, 50.0, 10.0], PdfAnnotationKind::Highlight { quad_points: vec![0.0, 10.0, 50.0, 10.0, 0.0, 0.0, 50.0, 0.0] });
    highlight.markup = Some(PdfMarkupAnnotation { in_reply_to: Some(1), reply_type: Some("R".into()), ..PdfMarkupAnnotation::default() });
    highlight.border = Some(PdfBorderStyle { width: 2.0, style: Some("D".into()), dash: Some(vec![3.0]), radii: None });
    page.annotations.push(highlight);
    page.annotations.push(PdfAnnotation::new([0.0, 0.0, 10.0, 10.0], PdfAnnotationKind::Ink { paths: vec![vec![0.0, 0.0, 5.0, 5.0], vec![1.0, 1.0, 2.0, 2.0]] }));
    let mut widget = PdfAnnotation::new([0.0, 0.0, 30.0, 10.0], PdfAnnotationKind::Widget { field: Some("name".into()), highlight: Some("I".into()), characteristics: vec![PdfDictEntry::new("BG", PdfObject::numbers(&[1.0]))], action: None, additional_actions: Vec::new() });
    widget.flags = 4;
    page.annotations.push(widget);
    page.annotations.push(PdfAnnotation::new([0.0, 0.0, 10.0, 10.0], PdfAnnotationKind::FileAttachment { file: PdfFileSpecification::Embedded { file: "Att1".into() }, icon: Some("Paperclip".into()) }));
    snapshot.pages.push(PdfPage::new(200.0, 200.0));
    snapshot.outlines = vec![PdfOutlineItem { title: "Chapter — one".into(), destination: Some(PdfDestination::Page { page: 0, fit: PdfDestinationFit::Xyz { left: Some(0.0), top: Some(400.0), zoom: None } }), action: None, color: Some([1.0, 0.0, 0.0]), italic: true, bold: false, open: true, children: vec![PdfOutlineItem::to_page("Section", 1), PdfOutlineItem { title: "Web".into(), destination: None, action: Some(PdfAction::uri("https://example.org")), color: None, italic: false, bold: true, open: false, children: Vec::new(), extra: Vec::new() }], extra: Vec::new() }, PdfOutlineItem::to_page("Chapter two", 1)];
    snapshot.named_destinations.push(PdfNamedDestination { name: "start".into(), destination: PdfDestination::Page { page: 0, fit: PdfDestinationFit::FitHorizontal { top: Some(300.0) } } });
    snapshot.page_labels = vec![PdfPageLabelRange { start_index: 0, style: Some(PdfPageLabelStyle::RomanLower), prefix: None, start: 1 }, PdfPageLabelRange { start_index: 1, style: Some(PdfPageLabelStyle::Decimal), prefix: Some("A-".into()), start: 3 }];
    snapshot.embedded_files.push(PdfEmbeddedFile { id: "Att1".into(), file_name: "data.csv".into(), description: Some("Table".into()), mime_type: Some("text/csv".into()), data: b"a,b\n1,2\n".to_vec(), creation_date: None, modification_date: Some(PdfDate { year: 2026, month: 1, day: 2, hour: 0, minute: 0, second: 0, offset_minutes: None }), relationship: Some("Data".into()), listed: true });
    snapshot.output_intents.push(PdfOutputIntent { subtype: "GTS_PDFA1".into(), condition_identifier: "sRGB IEC61966-2.1".into(), condition: None, registry_name: Some("http://www.color.org".into()), info: Some("sRGB".into()), profile: None });
    snapshot.acro_form = Some(PdfAcroForm { fields: vec![PdfFormField { name: "name".into(), kind: PdfFormFieldKind::Text { value: Some("Ueli".into()), default_value: None, max_length: Some(20), rich_value: None }, flags: 0, alternate_name: Some("Your name".into()), mapping_name: None, default_appearance: Some("/F1 12 Tf 0 g".into()), quadding: Some(1), widgets: vec![[0, 5]], children: Vec::new(), additional_actions: Vec::new(), extra: Vec::new() }, PdfFormField { name: "group".into(), kind: PdfFormFieldKind::Container, flags: 0, alternate_name: None, mapping_name: None, default_appearance: None, quadding: None, widgets: Vec::new(), children: vec![PdfFormField { name: "choice".into(), kind: PdfFormFieldKind::Choice { values: vec!["b".into()], default_values: Vec::new(), options: vec![("a".into(), "Alpha".into()), ("b".into(), "b".into())], top_index: None }, flags: 1 << 17, alternate_name: None, mapping_name: None, default_appearance: None, quadding: None, widgets: Vec::new(), children: Vec::new(), additional_actions: Vec::new(), extra: Vec::new() }], additional_actions: Vec::new(), extra: Vec::new() }], need_appearances: true, signature_flags: 0, default_appearance: Some("/F1 0 Tf 0 g".into()), quadding: None, default_fonts: vec!["F1".into()], extra: Vec::new() });
    snapshot.optional_content = Some(PdfOptionalContent { groups: vec![PdfOptionalContentGroup { id: "OCG1".into(), name: "Layer 1".into(), intent: vec!["View".into()], usage: Vec::new() }], name: Some("Default".into()), base_state_off: false, on: vec!["OCG1".into()], off: Vec::new(), order: vec![PdfObject::name("OCG1")], extra: Vec::new() });
    snapshot.forms[0].optional_content = Some("OCG1".into());
    snapshot.page_layout = Some(PdfPageLayout::TwoColumnLeft);
    snapshot.page_mode = Some(PdfPageMode::UseOutlines);
    snapshot.viewer_preferences = Some(PdfViewerPreferences { hide_toolbar: true, display_doc_title: true, direction: Some("R2L".into()), print_scaling: Some("None".into()), num_copies: Some(2), print_page_range: vec![1, 2], ..PdfViewerPreferences::default() });
    snapshot.open_action = Some(PdfOpenAction::Destination { destination: PdfDestination::Page { page: 1, fit: PdfDestinationFit::Fit } });
    snapshot.language = Some("de-CH".into());
    snapshot.mark_info = Some(PdfMarkInfo { marked: true, user_properties: false, suspects: false });
    snapshot.metadata = Some("<?xpacket begin=\"\"?><x:xmpmeta xmlns:x=\"adobe:ns:meta/\"/><?xpacket end=\"w\"?>".into());
    snapshot.document_id = Some([vec![1u8; 16], vec![2u8; 16]]);
    snapshot.info = PdfInfo { title: Some("Rich — document".into()), author: Some("semio".into()), subject: None, keywords: Some("pdf, typed".into()), creator: Some("semio".into()), producer: Some("semio pdf".into()), creation_date: Some(PdfDate { year: 2026, month: 9, day: 18, hour: 20, minute: 0, second: 0, offset_minutes: Some(120) }), modification_date: None, trapped: Some("False".into()), extra: Vec::new() };
    snapshot.catalog_extra.push(PdfDictEntry::new("SemioMarker", PdfObject::Int(7)));
    snapshot
}

#[test]
fn text_document_round_trips_and_shows_its_text() {
    let seed = text_document(&[(200.0, 300.0, "Semio"), (100.0, 100.0, "")]);
    let bytes = encode_pdf(&seed).unwrap();
    assert!(bytes.starts_with(b"%PDF-1.7"));
    let decoded = decode_pdf(&bytes).unwrap();
    assert_same_lanes(&decoded, &seed);
    assert_eq!(decoded.page_text(0), "Semio");
    assert_eq!(decoded.pages.len(), 2);
    assert_eq!(decoded.fonts.len(), 1);
    let again = encode_pdf(&decoded).unwrap();
    assert_eq!(again, bytes, "a decoded document is its own fixed point");
}

#[test]
fn rich_document_round_trips_every_lane() {
    let seed = rich_document();
    let bytes = encode_pdf(&seed).unwrap();
    let decoded = decode_pdf(&bytes).unwrap();
    assert_same_lanes(&decoded, &seed);
    let again = encode_pdf(&decoded).unwrap();
    assert_eq!(again, bytes);
    assert_same_lanes(&decode_pdf(&again).unwrap(), &seed);
}

#[test]
fn xref_streams_and_object_streams_round_trip() {
    let seed = rich_document();
    let options = EncodeOptions { lower: LowerOptions::default(), write: WriteOptions { xref_stream: true, encryption: None } };
    let bytes = encode_pdf_with(&seed, &options).unwrap();
    assert!(bytes.windows(7).any(|w| w == b"/ObjStm"), "object streams present");
    let decoded = decode_pdf(&bytes).unwrap();
    assert_same_lanes(&decoded, &seed);
}

#[test]
fn encryption_round_trips_with_every_algorithm() {
    for algorithm in [PdfEncryptionAlgorithm::Rc4_40, PdfEncryptionAlgorithm::Rc4_128, PdfEncryptionAlgorithm::Aes128, PdfEncryptionAlgorithm::Aes256] {
        let mut seed = rich_document();
        seed.encryption = Some(PdfEncryption { algorithm, permissions: -1, user_password: "pw".into(), owner_password: Some("owner".into()), encrypt_metadata: true });
        let bytes = encode_pdf(&seed).unwrap();
        assert!(bytes.windows(8).any(|w| w == b"/Encrypt"));
        assert!(!bytes.windows(9).any(|w| w == b"semio pdf"), "{algorithm:?}: strings are not in clear");
        assert!(matches!(decode_pdf(&bytes), Err(PdfEngineError::Unsupported(_))));
        let decoded = decode_pdf_with_password(&bytes, "pw").unwrap();
        let mut expected = typed_lanes(&seed);
        expected.encryption = Some(PdfEncryption { algorithm, permissions: -1, user_password: "pw".into(), owner_password: None, encrypt_metadata: true });
        assert_same_lanes(&decoded, &expected);
        let owner = decode_pdf_with_password(&bytes, "owner").unwrap();
        assert_eq!(owner.pages, seed.pages);
    }
}

#[test]
fn retained_graph_survives_typed_edits_and_keeps_foreign_objects() {
    let mut seed = text_document(&[(200.0, 300.0, "Semio")]);
    seed.catalog_extra.push(PdfDictEntry::new("SemioExtra", PdfObject::Ref(ObjRef { num: 900, gen: 0 })));
    let bytes = encode_pdf(&seed).unwrap();
    let mut decoded = decode_pdf(&bytes).unwrap();
    let foreign = ObjRef { num: decoded.objects.iter().map(|o| o.id.num).max().unwrap() + 1, gen: 0 };
    decoded.objects.push(PdfIndirectObject { id: foreign, value: PdfObject::Dict(vec![PdfDictEntry::new("Kind", PdfObject::name("Foreign")), PdfDictEntry::new("Page", PdfObject::Ref(ObjRef { num: decoded.objects.iter().find(|o| o.value.dict_get("Type").and_then(PdfObject::as_name) == Some("Page")).unwrap().id.num, gen: 0 }))]) });
    decoded.catalog_extra = vec![PdfDictEntry::new("SemioExtra", PdfObject::Ref(foreign))];
    let mut page = PdfPage::new(100.0, 100.0);
    page.content = vec![PdfOp::BeginText, PdfOp::SetFont { name: "F1".into(), size: 10.0 }, PdfOp::MoveText { tx: 10.0, ty: 50.0 }, PdfOp::ShowText { text: PdfTextString::text("Second") }, PdfOp::EndText];
    decoded.pages.insert(0, page);
    let edited = encode_pdf(&decoded).unwrap();
    let reread = decode_pdf(&edited).unwrap();
    assert_eq!(reread.pages.len(), 2);
    assert_eq!(reread.page_text(0), "Second");
    assert_eq!(reread.page_text(1), "Semio");
    let kept = reread.objects.iter().find(|o| o.value.dict_get("Kind").and_then(PdfObject::as_name) == Some("Foreign")).expect("foreign object kept");
    let page_ref = kept.value.dict_get("Page").and_then(PdfObject::as_ref).unwrap();
    let pointed = reread.objects.iter().find(|o| o.id == page_ref).unwrap();
    assert_eq!(pointed.value.dict_get("Type").and_then(PdfObject::as_name), Some("Page"), "reference to the moved page was rewritten");
    assert_eq!(reread.catalog_extra, vec![PdfDictEntry::new("SemioExtra", PdfObject::Ref(kept.id))]);
    assert_eq!(encode_pdf(&reread).unwrap(), edited, "one regeneration reaches the fixed point");
}

#[test]
fn document_stream_matches_the_whole_document_write() {
    let seed = rich_document();
    let (mut stream, mut bytes) = DocumentStream::begin(&seed, seed.pages.len(), EncodeOptions::default()).unwrap();
    for page in &seed.pages {
        bytes.extend_from_slice(&stream.page(page).unwrap());
    }
    bytes.extend_from_slice(&stream.finish().unwrap());
    let decoded = decode_pdf(&bytes).unwrap();
    assert_same_lanes(&decoded, &seed);
}

#[test]
fn report_strip_fixture_reads_and_rewrites_stably() {
    let decoded = decode_pdf(REPORT_STRIP).unwrap();
    assert_eq!(decoded.pages.len(), 3);
    assert!(decoded.page_text(0).contains("Report strip page one"), "{}", decoded.page_text(0));
    assert_eq!(decoded.pages[1].crop_box, Some([10.0, 10.0, 585.0, 832.0]));
    assert_eq!(decoded.pages[2].rotate, 90);
    let bytes = encode_pdf(&decoded).unwrap();
    let reread = decode_pdf(&bytes).unwrap();
    assert_same_lanes(&reread, &decoded);
    assert_eq!(encode_pdf(&reread).unwrap(), bytes);
}

#[test]
fn bachelor_thesis_reads_with_real_text_and_rewrites_stably() {
    let decoded = decode_pdf(THESIS).unwrap();
    assert_eq!(decoded.pages.len(), 65);
    let text: String = (0..8).map(|index| decoded.page_text(index)).collect::<Vec<_>>().join("\n");
    assert!(text.contains("Abstract") && text.contains("construction industry"), "{}", text.chars().take(500).collect::<String>());
    assert!(!decoded.fonts.is_empty());
    assert!(decoded.fonts.iter().any(|font| matches!(font.kind, PdfFontKind::Type0 { .. }) || matches!(font.kind, PdfFontKind::Type1 { program: Some(_), .. })));
    let bytes = encode_pdf(&decoded).unwrap();
    let reread = decode_pdf(&bytes).unwrap();
    assert_eq!(reread.pages.len(), 65);
    assert_same_lanes(&reread, &decoded);
    assert_eq!(encode_pdf(&reread).unwrap(), bytes);
}

#[test]
fn text_layout_wraps_by_real_widths() {
    let font = PdfFont::standard("F1", "Helvetica");
    let layout = PdfTextLayout::new(&font, 12.0);
    assert_eq!(layout.width("AA"), Some(1334.0 * 12.0 / 1000.0));
    let lines = layout.wrap("the quick brown fox jumps over the lazy dog", 100.0);
    assert!(lines.len() >= 3);
    assert!(lines.iter().all(|line| layout.width(line).unwrap() <= 100.0));
    let (ops, count) = layout.show_paragraph("one\ntwo\nthree", 0.0, 100.0, 200.0, 30.0);
    assert_eq!(count, 2);
    assert!(ops.iter().any(|op| matches!(op, PdfOp::NextLine)));
}

#[test]
fn unresolved_resources_are_reported() {
    let snapshot = text_document(&[(100.0, 100.0, "x")]);
    let ops = vec![PdfOp::SetFont { name: "Nope".into(), size: 1.0 }, PdfOp::PaintXObject { name: "Im9".into() }];
    assert_eq!(unresolved_resources(&snapshot, &ops), vec!["font Nope".to_string(), "xobject Im9".to_string()]);
}
