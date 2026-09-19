use super::*;
use crate::standards::v1_7::subsets::base::io::text_document;
use crate::standards::v1_7::subsets::base::schema::mutations::{binary, text};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::{OpBinary, OpText, SemanticMutation};

fn samples() -> Vec<PdfMutation> {
    let base = text_document(&[(200.0, 100.0, "Semio")]);
    vec![
        PdfMutation::InsertPage(InsertPage { index: 1, page: PdfPage::new(10.0, 20.0) }),
        PdfMutation::SetPageBox(set_page_box::SetPageBox { index: 0, kind: crate::standards::v1_7::subsets::base::schema::diff::PdfPageBox::Trim, rect: Some([1.0, 2.0, 3.0, 4.0]) }),
        PdfMutation::AppendPageContent(AppendPageContent { index: 0, content: vec![PdfOp::Save, PdfOp::Rectangle { x: 0.0, y: 0.0, width: 5.0, height: 5.0 }, PdfOp::Fill, PdfOp::Restore] }),
        PdfMutation::SetFont(set_font::SetFont { font: base.fonts[0].clone() }),
        PdfMutation::RemoveFont(remove_font::RemoveFont { id: "F1".into() }),
        PdfMutation::SetImage(set_image::SetImage { image: PdfImage::gray8("Im1", 2, 2, vec![0, 64, 128, 255]) }),
        PdfMutation::SetOutlines(set_outlines::SetOutlines { outlines: vec![PdfOutlineItem::to_page("Start", 0)] }),
        PdfMutation::SetLanguage(set_language::SetLanguage { language: Some("de-CH".into()) }),
        PdfMutation::SetEncryption(set_encryption::SetEncryption { encryption: None }),
        PdfMutation::SetCatalogEntry(set_catalog_entry::SetCatalogEntry { key: "Marker".into(), value: PdfObject::Int(7) }),
        PdfMutation::SetTrailerEntry(SetTrailerEntry { key: "Marker".into(), value: PdfObject::Bool(true) }),
    ]
}

#[test]
fn every_kind_is_declared_once_in_declaration_order() {
    let kinds = pdf_mutation_kinds();
    assert_eq!(kinds.len(), 60);
    assert_eq!(kinds.len(), binary::BINARY_TAG_REGISTRY.len());
    assert_eq!(kinds.len(), text::TEXT_OPCODE_REGISTRY.len());
    let mut seen = std::collections::HashSet::new();
    for (descriptor, (pascal, _, _)) in kinds.iter().zip(binary::BINARY_TAG_REGISTRY) {
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
        assert_eq!(bytes[0], store::pack_rt::OP_BINARY_FORMAT);
        assert_eq!(PdfMutation::decode_op(&bytes).unwrap(), mutation, "binary codec");
    }
}

#[test]
fn samples_apply_and_invert_on_a_real_document() {
    use protocol::Mutation;
    let base = text_document(&[(200.0, 100.0, "Semio")]);
    for mutation in samples() {
        let mut working = base.clone();
        let _outcome = apply_pdf_mutation(&mut working, &mutation);
        let mut restored = working.clone();
        for inverse in inverse_pdf_mutation(&mutation, &base) {
            apply_pdf_mutation(&mut restored, &inverse);
        }
        assert_eq!(restored, base, "inverse of {} lands back on the base", mutation.label());
    }
}
