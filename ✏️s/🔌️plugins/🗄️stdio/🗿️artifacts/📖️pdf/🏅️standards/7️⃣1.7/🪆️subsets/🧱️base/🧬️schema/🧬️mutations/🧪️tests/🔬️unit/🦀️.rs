use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfPage, PdfSnapshot};
use semio_framework_value::{DslValue, ToValue};
use semio_s_artifact_stdio_contract::editing::SnapshotEditEvent;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::PdfDiff;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::snapshot::ObjRef;
#[allow(unused_imports)]
use semio_framework_value::FromValue;
#[allow(unused_imports)]
use semio_plugin::{Fault, FaultCode, FaultOrigin};
#[allow(unused_imports)]
use semio_s_artifact_stdio_contract::editing::{EditRules, EntityRule, InsertRule, RemoveRule, Selector};
use crate::standards::v1_7::subsets::base::schema::mutations::*;
use crate::standards::v1_7::subsets::base::io::mutation_bridge::apply_pdf_mutation;
use crate::standards::v1_7::subsets::base::io::export::text_document;
use crate::standards::v1_7::subsets::base::io::text::mutations as text;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::{OpBinary, OpText, SemanticMutation};

fn samples() -> Vec<PdfMutation> {
    let base = text_document(&[(200.0, 100.0, "Semio")]);
    vec![
        PdfMutation::InsertPage(InsertPage { index: 1, page: PdfPage::new(10.0, 20.0) }),
        PdfMutation::SetPageBox(set_page_box::SetPageBox { index: 0, kind: crate::standards::v1_7::subsets::base::schema::diff::PdfPageBox::Trim, rect: Some([1.0, 2.0, 3.0, 4.0]) }),
        PdfMutation::AppendPageContent(AppendPageContent { index: 0, content: vec![PdfOp::Save, PdfOp::Rectangle { x: 0.0, y: 0.0, width: 5.0, height: 5.0 }, PdfOp::Fill, PdfOp::Restore] }),
        PdfMutation::SetFont(set_font::SetFont { font: base.fonts[0].clone(), index: None }),
        PdfMutation::RemoveFont(remove_font::RemoveFont { id: "F1".into() }),
        PdfMutation::SetImage(set_image::SetImage { image: PdfImage::gray8("Im1", 2, 2, vec![0, 64, 128, 255]), index: None }),
        PdfMutation::SetOutlines(set_outlines::SetOutlines { outlines: vec![PdfOutlineItem::to_page("Start", 0)] }),
        PdfMutation::SetLanguage(set_language::SetLanguage { language: Some("de-CH".into()) }),
        PdfMutation::SetEncryption(set_encryption::SetEncryption { encryption: None }),
        PdfMutation::SetCatalogEntry(set_catalog_entry::SetCatalogEntry { key: "Marker".into(), value: PdfObject::Int(7), index: None }),
        PdfMutation::SetTrailerEntry(SetTrailerEntry { key: "Marker".into(), value: PdfObject::Bool(true), index: None }),
        PdfMutation::ReplacePage(replace_page::ReplacePage { index: 0, page: PdfPage::new(10.0, 20.0) }),
    ]
}

#[test]
fn every_kind_is_declared_once_in_declaration_order() {
    let kinds = pdf_mutation_kinds();
    assert_eq!(kinds.len(), 61);
    assert_eq!(kinds.len(), text::TEXT_OPCODE_REGISTRY.len());
    let mut seen = std::collections::HashSet::new();
    for (descriptor, (pascal, _)) in kinds.iter().zip(text::TEXT_OPCODE_REGISTRY) {
        assert!(seen.insert(descriptor.kind), "duplicate kind {}", descriptor.kind);
        let expected = pascal.chars().fold(String::new(), |mut out, c| {
            if c.is_ascii_uppercase() && !out.is_empty() {
                out.push('-');
            }
            out.push(c.to_ascii_lowercase());
            out
        });
        assert_eq!(descriptor.kind, expected, "registry order drifts from the aggregate");
    }
    assert_eq!(PdfMutation::kinds().first().map(|k| k.kind), Some("insert-page"));
}

#[test]
fn samples_round_trip_through_both_op_codecs() {
    for mutation in samples() {
        let line = mutation.print_op();
        assert!(!line.contains('\n'));
        assert_eq!(PdfMutation::parse_op(&line).unwrap(), mutation, "text codec");
        let bytes = mutation.encode_op().unwrap();
        assert_eq!(PdfMutation::decode_op(&bytes).unwrap(), mutation, "pack codec");
    }
}

#[test]
fn pdf17_operations_round_trip_through_the_pack_document_with_exact_float_words() {
    let words = [0x8000_0000_0000_0000_u64, 0x0000_0000_0000_0001, 1.0_f64.to_bits(), 612.0_f64.to_bits(), 0x7fef_ffff_ffff_ffff, 0.1_f64.to_bits()];
    let mut count = 0;
    for word in words {
        let value = f64::from_bits(word);
        let operations = vec![
            PdfMutation::InsertPage(InsertPage { index: 0, page: PdfPage::new(value, 792.0) }),
            PdfMutation::SetPageBox(set_page_box::SetPageBox { index: 0, kind: crate::standards::v1_7::subsets::base::schema::diff::PdfPageBox::Trim, rect: Some([value, value, 612.0, 792.0]) }),
            PdfMutation::AppendPageContent(AppendPageContent { index: 0, content: vec![PdfOp::Rectangle { x: value, y: 0.0, width: 5.0, height: value }] }),
        ];
        for operation in operations {
            let bytes = operation.encode_op().unwrap();
            assert_eq!(&bytes[..8], &[0x89, 0x53, 0x50, 0x4B, 0x0D, 0x0A, 0x1A, 0x0A]);
            let decoded = PdfMutation::decode_op(&bytes).unwrap();
            assert_eq!(decoded.encode_op().unwrap(), bytes);
            match (&operation, &decoded) {
                (PdfMutation::SetPageBox(left), PdfMutation::SetPageBox(right)) => {
                    let (left, right) = (left.rect.unwrap(), right.rect.unwrap());
                    assert_eq!(left.map(f64::to_bits), right.map(f64::to_bits));
                }
                (PdfMutation::AppendPageContent(left), PdfMutation::AppendPageContent(right)) => {
                    let (PdfOp::Rectangle { x: left_x, height: left_height, .. }, PdfOp::Rectangle { x: right_x, height: right_height, .. }) = (&left.content[0], &right.content[0]) else { panic!("rectangle expected") };
                    assert_eq!((left_x.to_bits(), left_height.to_bits()), (right_x.to_bits(), right_height.to_bits()));
                }
                _ => assert_eq!(operation, decoded),
            }
            count += 1;
        }
    }
    assert_eq!(count, 18);
}

#[test]
fn samples_apply_and_invert_on_a_real_document() {
    use protocol::Mutation;
    let base = text_document(&[(200.0, 100.0, "Semio")]);
    for mutation in samples() {
        let mut working = base.clone();
        let _outcome = apply_pdf_mutation(&mut working, &mutation);
        let mut restored = working.clone();
        for inverse in semio_s_artifact_stdio_contract::mutation_inverse(&mutation, &base).expect("valid retained mutation inverse fixture") {
            apply_pdf_mutation(&mut restored, &inverse);
        }
        assert_eq!(restored, base, "inverse of {} lands back on the base", mutation.label().resolve(semio_ui_locale::Terminology::Native, semio_ui_locale::Locale::En));
    }
}

fn resolved(event: &semio_s_artifact_stdio_contract::editing::SnapshotEditEvent, base: &PdfSnapshot) -> Result<Vec<PdfMutation>, String> {
    if let Some(leaves) = special_edit(event, base).map_err(|fault| format!("{fault:?}"))? {
        return Ok(leaves);
    }
    EDIT_RULES.resolve::<PdfSnapshot, PdfMutation>(base, event).map_err(|error| error.to_string())
}

#[test]
fn a_details_edit_dispatches_the_concrete_kind_of_the_field_it_names() {
    use semio_framework_value::ToValue;
    use semio_s_artifact_stdio_contract::editing::SnapshotEditEvent;
    let space = |name: &str| PdfNamedColorSpace { name: name.to_string(), color_space: PdfColorSpace::DeviceRgb };
    let mut base = text_document(&[(200.0, 100.0, "one"), (300.0, 100.0, "two"), (400.0, 100.0, "three")]);
    base.color_spaces = vec![space("A"), space("B"), space("C")];
    let set = |path: &str, value: semio_framework_value::DslValue| resolved(&SnapshotEditEvent::SetValue { path: path.to_string(), value }, &base);
    assert!(matches!(set("/language", Some("de-CH".to_string()).to_value()).unwrap().as_slice(), [PdfMutation::SetLanguage(SetLanguage { language: Some(_) })]));
    assert!(matches!(set("/info/title", "net".to_string().to_value()).unwrap().as_slice(), [PdfMutation::SetInfo(_)]));
    assert!(matches!(set("/pages/1/rotate", 90i64.to_value()).unwrap().as_slice(), [PdfMutation::ReplacePage(ReplacePage { index: 1, .. })]));
    assert!(set("/language", None::<String>.to_value()).unwrap().is_empty(), "an unchanged lane needs no leaf");
    assert!(matches!(set("/colorSpaces/1", space("B2").to_value()).unwrap().as_slice(), [PdfMutation::RemoveColorSpace(RemoveColorSpace { name }), PdfMutation::SetColorSpace(SetColorSpace { index: Some(1), .. })] if name == "B"), "a renamed row keeps its position");
    assert!(matches!(set("/colorSpaces/1/name", "B2".to_string().to_value()).unwrap().as_slice(), [PdfMutation::RemoveColorSpace(_), PdfMutation::SetColorSpace(SetColorSpace { index: Some(1), .. })]));
    assert!(matches!(set("/colorSpaces/1", space("B").to_value()).unwrap().as_slice(), []), "an unchanged row needs no leaf");
    let remove = |path: &str| resolved(&SnapshotEditEvent::RemoveValue { path: path.to_string() }, &base).unwrap();
    assert!(matches!(remove("/pages/1").as_slice(), [PdfMutation::RemovePage(RemovePage { index: 1 })]));
    assert!(matches!(remove("/colorSpaces/1").as_slice(), [PdfMutation::RemoveColorSpace(RemoveColorSpace { name })] if name == "B"));
    let insert = resolved(&SnapshotEditEvent::InsertValue { path: "/colorSpaces/1".to_string(), value: PdfNamedColorSpace { name: "Z".to_string(), color_space: PdfColorSpace::DeviceGray }.to_value() }, &base).unwrap();
    assert!(matches!(insert.as_slice(), [PdfMutation::SetColorSpace(SetColorSpace { index: Some(1), .. })]));
    let moved = resolved(&SnapshotEditEvent::MoveValue { from: "/pages/1".to_string(), path: "/pages/2".to_string() }, &base).unwrap();
    assert!(matches!(moved.as_slice(), [PdfMutation::MovePage(MovePage { from: 1, to: 2 })]));
    let entry = resolved(&SnapshotEditEvent::InsertValue { path: "/trailer/-".to_string(), value: PdfDictEntry { key: "K".to_string(), value: PdfObject::Int(1) }.to_value() }, &base).unwrap();
    assert!(matches!(entry.as_slice(), [PdfMutation::SetTrailerEntry(SetTrailerEntry { key, index: Some(0), .. })] if key == "K"));
    assert!(set("/schema", "other".to_string().to_value()).is_err(), "the identity markers are not editable");
    assert!(set("/colorSpaces", Vec::<PdfNamedColorSpace>::new().to_value()).is_err(), "a list lane is edited item by item");
}
