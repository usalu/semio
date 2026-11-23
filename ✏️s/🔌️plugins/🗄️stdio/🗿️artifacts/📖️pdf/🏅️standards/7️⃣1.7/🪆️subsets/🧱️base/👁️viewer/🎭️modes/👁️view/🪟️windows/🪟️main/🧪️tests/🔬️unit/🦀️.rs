#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfPage;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
#[allow(unused_imports)]
use semio_plugin::app::{DocumentPage, DocumentView, DocumentWindowKit, WindowKit};
#[allow(unused_imports)]
use semio_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_plugin::WindowKindDefinition;
#[allow(unused_imports)]
use semio_ui_contract::BuiltNode;
use crate::viewer::pdf17::modes::view::windows::main::*;
use crate::standards::v1_7::subsets::base::io::export::demo_pdf17_snapshot;

#[test]
fn definition_declares_a_document_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[test]
fn render_lists_one_line_per_page_with_media_box_and_text() {
    let document = demo_pdf17_snapshot();
    assert_eq!(document.pages.len(), 1);
    let _arena = semio_plugin::artifact_app_laws::FixtureUiArena::admit();
    let node = render(&document, semio_ui_locale::Locale::En).expect("bounded PDF document fixture must render");
    let observed_children = node.children.len();
    let projection = semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::built_to_component_tree(node)).expect("project the rendered page window");
    assert_eq!(observed_children, 1);
    let page: semio_ui_scene::TextEditorScene = semio_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(&projection).expect("the page renders as a read-only text scene with its lanes");
    assert!(page.buffer.contains("MediaBox"));
}

#[test]
fn page_summary_speaks_both_languages_with_the_locale_decimal_mark() {
    let mut document = demo_pdf17_snapshot();
    document.pages[0].media_box = [0.0, 0.0, 612.5, 792.0];
    document.pages[0].crop_box = Some([1.5, 2.0, 600.0, 780.0]);
    let english = page_summary(0, &document.pages[0], semio_ui_locale::Locale::En);
    let german = page_summary(0, &document.pages[0], semio_ui_locale::Locale::De);
    assert!(english.starts_with("Page 1 -- MediaBox [0.0, 0.0, 612.5, 792.0], CropBox [1.5, 2.0, 600.0, 780.0]"), "{english}");
    assert!(german.starts_with("Seite 1 -- Medienrahmen [0,0; 0,0; 612,5; 792,0], Beschnittrahmen [1,5; 2,0; 600,0; 780,0]"), "{german}");
    document.pages[0].content.clear();
    assert!(page_summary(0, &document.pages[0], semio_ui_locale::Locale::En).contains("(no extracted or authored text)"));
    assert!(page_summary(0, &document.pages[0], semio_ui_locale::Locale::De).contains("(kein extrahierter oder verfasster Text)"));
}

#[test]
fn viewer_labels_are_complete_and_distinct_in_both_languages() {
    use semio_ui_locale::Locale;
    for locale in Locale::ALL {
        let labels = pdf_viewer_labels(locale);
        assert_eq!(labels.locale(), locale);
        assert!(labels.unknown_body.fill(&[("body", "x")]).to_string().contains('x'));
    }
    assert_ne!(pdf_viewer_labels(Locale::En).unknown_body.as_str(), pdf_viewer_labels(Locale::De).unknown_body.as_str());
}
