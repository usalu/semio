use super::*;
use crate::schema::snapshot::{DocxBlock, DocxDocument};
use crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_docx;

fn descendant<'a>(node: &'a BuiltNode, key: &str) -> Option<&'a BuiltNode> {
    node.children.iter().find(|child| child.key.as_str() == key).or_else(|| node.children.iter().find_map(|child| descendant(child, key)))
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_document_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🧫️fixtures/🎨️run-formatting-action/🔣️.json")).expect("language-neutral formatting action fixture");
    let action = def.actions.iter().find(|action| action.id == fixture["action"].as_str().unwrap()).expect("formatting action");
    assert_eq!(action.label, LocalizedLabel::native(fixture["label"]["en"].as_str().unwrap(), fixture["label"]["de"].as_str().unwrap()));
    assert_eq!(action.args.iter().map(|argument| argument.id.as_str()).collect::<Vec<_>>(), fixture["fields"].as_array().unwrap().iter().map(|field| field["id"].as_str().unwrap()).collect::<Vec<_>>());
}

#[semio_framework_async_macros::async_test]
async fn render_exposes_localized_accessible_formatting_toggles_only_for_runs() {
    let document = build_minimal_docx(DocxDocument {
        body: vec![
            DocxBlock::Paragraph(crate::schema::snapshot::DocxParagraph { runs: vec![crate::schema::snapshot::DocxRun { text: "formatted".into(), italic: true, ..Default::default() }], ..Default::default() }),
            DocxBlock::Paragraph(crate::schema::snapshot::DocxParagraph::default()),
        ],
        styles: Vec::new(),
    });
    let address = docx_top_level_text_targets(&document, 0).unwrap().remove(0).address;
    for (locale, labels) in [(Locale::En, ["Bold (inherited)", "Italic (direct on)", "Underline (inherited)"]), (Locale::De, ["Fett (geerbt)", "Kursiv (direkt an)", "Unterstrichen (geerbt)"])] {
        let rendered = render_windowed(&document, &TreeWindows::unhosted(), locale, semio_framework_plugin::UiPublicationRevision::default()).expect("render formatting controls");
        let run = rendered.children.get(0).expect("run item");
        let toolbar = descendant(run, "run-formatting").expect("formatting toolbar");
        assert_eq!(toolbar.children.len(), 3);
        for (((control, label), expected_on), expected_next) in toolbar.children.iter().zip(labels).zip([false, true, false]).zip([(true, true, false), (false, false, false), (false, true, true)]) {
            let semio_framework_plugin::Component::Toggle(props) = &control.component else { panic!("formatting control is a toggle") };
            assert_eq!(props.text.as_ref().expect("accessible label").0.as_str(), label);
            assert_eq!(props.on, expected_on);
            let binding = control.bindings.get(0).expect("formatting action binding");
            assert_eq!(binding.action, ActionId::try_v1(BASE_CONTROLLER_ID, SET_RUN_FORMATTING_ACTION).unwrap());
            assert_eq!(serde_json::to_value(binding.args.as_ref().unwrap()).unwrap(), serde_json::to_value(formatting_arguments(&address, expected_next.0, expected_next.1, expected_next.2).unwrap()).unwrap());
        }
        let empty = rendered.children.get(1).expect("empty paragraph item");
        assert!(descendant(empty, "run-formatting").is_none());
    }
}

#[semio_framework_async_macros::async_test]
async fn formatting_accessibility_labels_distinguish_absent_and_explicit_off() {
    let document = build_minimal_docx(DocxDocument { body: vec![DocxBlock::paragraph("text")], styles: Vec::new() });
    let address = docx_top_level_text_targets(&document, 0).unwrap().remove(0).address;
    let inherited = formatting_toolbar(&address, Some(DocxRunFormatting::default()), BASE_CONTROLLER_ID, Locale::En).unwrap().unwrap();
    let off = formatting_toolbar(&address, Some(DocxRunFormatting { bold: Some(false), italic: Some(false), underline: Some(false) }), BASE_CONTROLLER_ID, Locale::En).unwrap().unwrap();
    let label = |toolbar: &BuiltNode| {
        let semio_framework_plugin::Component::Toggle(props) = &toolbar.children[0].component else { panic!("bold toggle") };
        props.text.as_ref().unwrap().0.as_str().to_owned()
    };
    assert_eq!(label(&inherited), "Bold (inherited)");
    assert_eq!(label(&off), "Bold (direct off)");
}

#[semio_framework_async_macros::async_test]
async fn hosted_collapsed_drafts_do_not_build_formatting_accessories_until_expanded() {
    use semio_framework_plugin::{TreeWindowRequest, ViewModel, TREE_WINDOW_PATH_SEPARATOR};

    let document = build_minimal_docx(DocxDocument { body: vec![DocxBlock::paragraph("formatted")], styles: Vec::new() });
    let view = |open| ViewModel {
        tree_windows: vec![
            TreeWindowRequest { body_key: BODY_KEY.into(), node_key: WINDOW_KIND_ID.into(), open: Some(true), offset: 0, rows: 1 },
            TreeWindowRequest { body_key: BODY_KEY.into(), node_key: format!("{WINDOW_KIND_ID}{TREE_WINDOW_PATH_SEPARATOR}page-0-item-0"), open: Some(open), offset: 0, rows: u32::from(open) },
        ],
        ..ViewModel::new(Locale::En, semio_framework_ui_locale::Terminology::Native)
    };
    let collapsed = view(false);
    let collapsed = render_windowed(&document, &TreeWindows::for_body(&collapsed, BODY_KEY), Locale::En, semio_framework_plugin::UiPublicationRevision::default()).unwrap();
    assert!(descendant(collapsed.children.get(0).unwrap(), "run-formatting").is_none());
    let expanded = view(true);
    let expanded = render_windowed(&document, &TreeWindows::for_body(&expanded, BODY_KEY), Locale::En, semio_framework_plugin::UiPublicationRevision::default()).unwrap();
    assert!(descendant(expanded.children.get(0).unwrap(), "run-formatting").is_some());
}

#[semio_framework_async_macros::async_test]
async fn render_emits_one_page_per_top_level_block() {
    let document = build_minimal_docx(DocxDocument { body: vec![DocxBlock::paragraph("first"), DocxBlock::paragraph("second")], styles: Vec::new() });
    let stack = render(&document, semio_framework_plugin::UiPublicationRevision(23)).expect("render");
    assert_eq!(stack.children.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn nested_table_text_remains_editable_in_the_document_window() {
    use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_from_text;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🧫️fixtures/🧭️table-run-projection/🔣️.json")).unwrap();
    let mut snapshot = crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_docx(crate::schema::snapshot::DocxDocument::default());
    let part_path = crate::standards::v_ecma_376::subsets::base::schema::inferences::document::main_document_path(&snapshot.opc).unwrap();
    snapshot.xml_part_mut(&part_path).unwrap().replace_document(xml_document_from_text(fixture["xml"].as_str().unwrap()).unwrap()).unwrap();
    let expected: Vec<Vec<String>> = serde_json::from_value(fixture["blocks"].clone()).unwrap();
    let pages = editable_pages(&snapshot).unwrap();
    assert_eq!(pages.iter().map(|page| page.text.as_str()).collect::<Vec<_>>(), expected.iter().flatten().map(String::as_str).collect::<Vec<_>>());
    assert!(pages.iter().all(|page| matches!(&page.arguments, Some(UiValue::Map(_)))));
    for locale in [Locale::En, Locale::De] {
        let rendered = render_windowed(&snapshot, &TreeWindows::unhosted(), locale, semio_framework_plugin::UiPublicationRevision::default()).unwrap();
        assert_eq!(rendered.children.len(), 5);
    }
}
