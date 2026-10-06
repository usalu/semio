//! \u{1f9ea}️ Own PDF 1.4 page drafts, mutation inverses and address admission.
use super::*;
use protocol::{MutationDiff, OpBinary};
const CORPUS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../\u{1f3c5}️standards/4️⃣1.4/\u{1fa86}️subsets/\u{1f9f1}️base/✏️editor/\u{1f9eb}️fixtures/\u{1f4c4}️resolved-page-domain/\u{1f523}️.json"));
fn fixture() -> crate::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot {
    use crate::standards::v1_4::subsets::base::schema::snapshot::{PdfSnapshot, PageDoc};
    let fixture: serde_json::Value = serde_json::from_str(CORPUS).unwrap();
    PdfSnapshot { schema: fixture["schema"].as_str().unwrap().into(), pages: fixture["pages"].as_array().unwrap().iter().map(|page| PageDoc { width: page["width"].as_f64().unwrap(), height: page["height"].as_f64().unwrap(), text: page["text"].as_str().unwrap().into() }).collect() }
}


#[test]
fn own14_page_view_retains_the_exact_publication_and_each_addressed_text() {
    let document = fixture();
    let revision = UiPublicationRevision(37);
    let view = editable_view(&document, revision).unwrap();
    assert_eq!(view.publication_revision, revision);
    assert_eq!(view.pages.len(), 2);
    for (index, (actual, expected)) in view.pages.iter().zip(&document.pages).enumerate() {
        assert_eq!(actual.page_index as usize, index);
        assert_eq!(actual.item_index, 0);
        assert_eq!(actual.text, expected.text);
    }
    for locale in [Locale::En, Locale::De] {
        let node = render_windowed(&document, revision, &TreeWindows::unhosted(), locale).unwrap();
        let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).unwrap();
        assert!(projection.contains("page-0-item-0"));
        assert!(projection.contains("page-1-item-0"));
    }
    eprintln!("[DEBUG] own14 two-page drafts keep real publication addresses and both locales");
}

#[test]
fn own14_page_text_admission_keeps_geometry_and_inverse() {
    let original = fixture();
    let revision = DocumentWindowKit::text_revision(&original.pages[0].text);
    let op = text_edit_mutation(&original, 0, 0, &revision, "Edited own14 text").unwrap().unwrap();
    assert_eq!(PdfMutation::decode_op(&op.encode_op().unwrap()).unwrap(), op);
    let outcome = op.diff(&original);
    assert!(outcome.messages().is_empty());
    let next = outcome.diff().apply(&original).unwrap();
    assert_eq!(next.pages[0].text, "Edited own14 text");
    assert_eq!(next.pages[0].width.to_bits(), original.pages[0].width.to_bits());
    assert_eq!(next.pages[0].height.to_bits(), original.pages[0].height.to_bits());
    assert_eq!(next.pages[1], original.pages[1]);
    assert_eq!(next.schema, original.schema);
    let mut restored = next;
    for inverse in op.inverse(&original).unwrap() { restored = inverse.diff(&restored).diff().apply(&restored).unwrap(); }
    assert_eq!(restored, original);
    assert!(text_edit_mutation(&original, 0, 0, &revision, &original.pages[0].text).unwrap().is_none());
    for (page, item, token) in [(2, 0, revision.as_str()), (0, 1, revision.as_str()), (0, 0, "stale")] {
        assert!(text_edit_mutation(&original, page, item, token, "edited").is_err());
    }
    assert_eq!(original, fixture());
    eprintln!("[DEBUG] own14 text admission rejects stale and absent targets and restores inverse");
}

#[test]
fn own14_page_structure_actions_only_emit_their_actual_mutation_kind() {
    let base = fixture();
    let args = DslValue::Object(vec![("page".into(), DslValue::float(1.0)), ("width".into(), DslValue::float(8.0)), ("height".into(), DslValue::float(9.0))]);
    let payload = edit_from_action("set-page-size", Some(&args)).unwrap();
    let emit = emit_page_edit(&base, "set-page-size", &payload).unwrap();
    assert_eq!(emit.artifact_mutations.len(), 1);
    let op = &emit.artifact_mutations[0];
    assert!(matches!(op, PdfMutation::ResizePage(ResizePage { index: 1, width: 8.0, height: 9.0 })));
    let next = op.diff(&base).diff().apply(&base).unwrap();
    assert_eq!(next.pages[0], base.pages[0]);
    assert_eq!(next.pages[1].text, base.pages[1].text);
    assert_eq!(next.pages[1].width.to_bits(), 8.0f64.to_bits());
    assert_eq!(next.pages[1].height.to_bits(), 9.0f64.to_bits());
    let mut restored = next;
    for inverse in op.inverse(&base).unwrap() { restored = inverse.diff(&restored).diff().apply(&restored).unwrap(); }
    assert_eq!(restored, base);
    assert!(emit_page_edit(&base, "remove-page", &payload).is_err());
    assert!(edit_from_action("insert-image", Some(&args)).is_err());
    assert!(edit_from_action("set-page-size", None).is_err());
    let wrong = DslValue::Object(vec![("page".into(), DslValue::float(1.0)), ("width".into(), DslValue::float(f64::NAN)), ("height".into(), DslValue::float(9.0))]);
    assert!(edit_from_action("set-page-size", Some(&wrong)).is_err());
    let out = PdfMutation::ResizePage(ResizePage { index: 2, width: 8.0, height: 9.0 }).print_op();
    assert!(emit_page_edit(&base, "set-page-size", &out).is_err());
    eprintln!("[DEBUG] own14 page-size command actual typed leaf and exact inverse");
}
