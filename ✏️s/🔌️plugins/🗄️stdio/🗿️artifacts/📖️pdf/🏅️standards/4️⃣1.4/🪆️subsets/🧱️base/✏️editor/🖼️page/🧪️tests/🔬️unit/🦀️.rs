//! 🧪️ Own PDF 1.4 page text drafts, mutation inverses and address admission; page structure is tested with the canvas.
use crate::standards::v1_4::subsets::base::schema::snapshot::{PageDoc, PdfSnapshot};
use crate::standards::v1_4::subsets::base::schema::mutations::PdfMutation;
use semio_plugin::TreeWindows;
use semio_plugin::app::DocumentWindowKit;
use semio_framework_test_fixtures::fixture_text;
use semio_ui_contract::UiPublicationRevision;
use semio_ui_locale::Locale;
#[allow(unused_imports)]
use crate::standards::v1_4::subsets::base::schema::mutations::{InsertPage, RemovePage, MovePage, ReplacePageText};
#[allow(unused_imports)]
use protocol::{ColdMutation, Mutation, OpText};
#[allow(unused_imports)]
use semio_plugin::{Fault, FaultCode, FaultOrigin, ActionArgDef, ActionDefinition, ActionKind, WindowKindDefinition, PluginAssemblyError, UiAssemblyResult, Buildable, HasBase, UiText};
#[allow(unused_imports)]
use semio_plugin::app::{EditableDocumentPage, EditableDocumentView, WindowKit};
#[allow(unused_imports)]
use semio_plugin::plugin_app_close_prelude as ui;
#[allow(unused_imports)]
use semio_ui_contract::BuiltNode;
#[allow(unused_imports)]
use semio_ui_locale::LocalizedLabel;
use crate::editor::pdf14::page::{index};
use crate::editor::pdf14::page::*;
use protocol::{MutationDiff, OpBinary};
static CORPUS: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../🧱️base/✏️editor/🧪️tests/🧫️fixtures/📄️resolved-page-domain/🔣️.json"));
fn fixture() -> crate::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot {
    use crate::standards::v1_4::subsets::base::schema::snapshot::{PdfSnapshot, PageDoc};
    let fixture: serde_json::Value = serde_json::from_str((*CORPUS)).unwrap();
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
        let projection = semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::built_to_component_tree(node)).unwrap();
        assert!(projection.contains("page-0-item-0"));
        assert!(projection.contains("page-1-item-0"));
    }
}

#[test]
fn own14_page_text_admission_keeps_geometry_and_inverse() {
    let original = fixture();
    let revision = DocumentWindowKit::text_revision(&original.pages[0].text);
    let op = text_edit_mutation(&original, 0, 0, &revision, "Edited own14 text").unwrap().unwrap();
    assert_eq!(PdfMutation::decode_op(&op.encode_op().unwrap()).unwrap(), op);
    let outcome = op.diff(&original);
    assert!(outcome.messages().is_empty());
    let next = protocol::apply_diff_cold(outcome.diff(), &original).unwrap();
    assert_eq!(next.pages[0].text, "Edited own14 text");
    assert_eq!(next.pages[0].width.to_bits(), original.pages[0].width.to_bits());
    assert_eq!(next.pages[0].height.to_bits(), original.pages[0].height.to_bits());
    assert_eq!(next.pages[1], original.pages[1]);
    assert_eq!(next.schema, original.schema);
    let mut restored = next;
    for inverse in op.inverse(&original).unwrap() { restored = protocol::apply_diff_cold(inverse.diff(&restored).diff(), &restored).unwrap(); }
    assert_eq!(restored, original);
    assert!(text_edit_mutation(&original, 0, 0, &revision, &original.pages[0].text).unwrap().is_none());
    for (page, item, token) in [(2, 0, revision.as_str()), (0, 1, revision.as_str()), (0, 0, "stale")] {
        assert!(text_edit_mutation(&original, page, item, token, "edited").is_err());
    }
    assert_eq!(original, fixture());
}
