//! 📝️ Native PDF round trips for editor draft and resize controls.

use crate::editor::page::{apply_payload, edit_from_action, page_text_edit_mutations};
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfPage, PdfSnapshot};

fn native_page_edit_fixture() -> serde_json::Value {
    serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../../../../🧪️tests/🧫️fixtures/📝️native-page-edit/🔣️.json")).unwrap()
}

#[test]
fn native_page_edit_keeps_the_media_box_origin_when_resizing() {
    use crate::standards::v1_7::subsets::base::io::{export::encode_pdf, import::decode_pdf, mutation_bridge::apply_pdf_mutation};
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base as oracle;

    let fixture = native_page_edit_fixture();
    let case = &fixture["mediaBox"];
    let spec = |key: &str| semio_repo_test_host::parse_json(&case[key].to_string()).unwrap();
    let input = semio_framework_test_fixtures::fixture_bytes!("../../../../../../🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
    let input = oracle::oracle_apply_mutation(input, &spec("initial")).unwrap();
    let expected = oracle::oracle_apply_mutation(&input, &spec("expected")).unwrap();
    let mut snapshot = decode_pdf(&input).unwrap();
    let args = semio_framework_value::DslValue::from(case["resize"].clone());
    let edit = edit_from_action("set-page-size", Some(&args)).unwrap();
    for mutation in apply_payload(&snapshot, &edit.action, &edit.payload).unwrap() {
        apply_pdf_mutation(&mut snapshot, &mutation);
    }
    let output = encode_pdf(&snapshot).unwrap();
    assert_eq!(oracle::project_pdf_1_7(&output).unwrap(), oracle::project_pdf_1_7(&expected).unwrap());
}

#[test]
fn native_page_edit_places_the_first_draft_with_a_bound_font() {
    use crate::standards::v1_7::subsets::base::io::{export::encode_pdf, import::{decode_pdf, page_text}, mutation_bridge::apply_pdf_mutation};
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base as oracle;

    let fixture = native_page_edit_fixture();
    let case = &fixture["blankDraft"];
    let text = case["text"].as_str().unwrap();
    let mut snapshot = PdfSnapshot::default();
    snapshot.pages.push(PdfPage::new(100.0, 120.0));
    let mut snapshot = decode_pdf(&encode_pdf(&snapshot).unwrap()).unwrap();
    let revision = semio_plugin::app::DocumentWindowKit::text_revision("");
    for mutation in page_text_edit_mutations(&snapshot, 0, 0, &revision, text).unwrap() {
        apply_pdf_mutation(&mut snapshot, &mutation);
    }
    let bytes = encode_pdf(&snapshot).unwrap();
    let projected: serde_json::Value = serde_json::from_str(&oracle::project_pdf_1_7(&bytes).unwrap().to_string()).unwrap();
    assert_eq!(projected["pages"][0]["contentOperators"], case["expectedOperators"]);
    assert_eq!(projected["pages"][0]["text"], case["expectedLines"]);
    let reopened = decode_pdf(&bytes).unwrap();
    assert_eq!(page_text(&reopened, 0).unwrap(), text);
    assert_eq!(reopened.fonts.len(), 1);
}

#[test]
fn page_inspector_edits_and_clears_pdf_boundaries() {
    let _ui_arena = semio_plugin::artifact_app_laws::FixtureUiArena::admit();
    use crate::editor::page::{canvas_layers, render_inspector};
    use crate::standards::v1_7::subsets::base::io::{export::encode_pdf, import::decode_pdf, mutation_bridge::apply_pdf_mutation};
    use semio_framework_value::DslValue;
    use serde_json::{json, Value};

    let fixture = native_page_edit_fixture();
    let case = &fixture["pageBoundaries"];
    let mut snapshot = PdfSnapshot::default();
    snapshot.pages.push(PdfPage::new(case["initialSize"][0].as_f64().unwrap(), case["initialSize"][1].as_f64().unwrap()));
    let apply = |snapshot: &mut PdfSnapshot, action, args: Value| {
        let edit = edit_from_action(action, Some(&DslValue::from(args))).unwrap();
        for mutation in apply_payload(snapshot, &edit.action, &edit.payload).unwrap() {
            apply_pdf_mutation(snapshot, &mutation);
        }
    };
    let box_args = |definition: &Value| json!({"page": 1, "object": definition["object"], "field": "text", "value": definition["coordinates"]});
    let after_edit = |snapshot: &PdfSnapshot| decode_pdf(&encode_pdf(snapshot).unwrap()).unwrap();

    for definition in case["edits"].as_array().unwrap() {
        apply(&mut snapshot, "set-page-extra", box_args(definition));
    }
    apply(&mut snapshot, "set-page-user-unit", json!({"page": 1, "field": "x", "value": case["userUnit"]}));
    let mut snapshot = after_edit(&snapshot);
    let page = &snapshot.pages[0];
    let actual = [Some(page.media_box), page.crop_box, page.bleed_box, page.trim_box, page.art_box];
    for (actual, expected) in actual.into_iter().zip(case["edits"].as_array().unwrap()) {
        let expected = expected["value"].as_array().unwrap().iter().map(|value| value.as_f64().unwrap()).collect::<Vec<_>>();
        assert_eq!(actual.unwrap().as_slice(), expected);
    }
    assert_eq!(page.user_unit, case["userUnit"].as_f64());
    let view_size = |snapshot: &PdfSnapshot| {
        let scene: Value = serde_json::from_str(&canvas_layers(snapshot, None, semio_ui_locale::Locale::En)).unwrap();
        let corner = &scene[0]["segments"][2]["to"];
        [corner[0].as_f64().unwrap(), corner[1].as_f64().unwrap()]
    };
    let expected_size = |key: &str| [case[key][0].as_f64().unwrap(), case[key][1].as_f64().unwrap()];
    assert_eq!(view_size(&snapshot), expected_size("viewSize"));

    let panel = render_inspector(&snapshot, Some("p0:paper"), "s.stdio.pdf@1.7/*#editor", semio_plugin::UiPublicationRevision(17), semio_ui_locale::Locale::En).unwrap();
    let mut names = Vec::new();
    let mut shown = Vec::new();
    let policies = semio_plugin::artifact_app_laws::observe_and_retire_fixture_tree(semio_plugin::built_to_component_tree(panel), |panel| {
        super::collect_inspector(panel, &mut names, &mut shown);
        panel.children.iter().filter(|node| matches!(&node.component,
            semio_ui_contract::Component::Input(input) if input.commit.as_ref().is_some_and(|mode| mode.as_str() == "blurDecimalQuad"))).count()
    });
    assert_eq!(names.iter().filter(|name| *name == "set-page-extra").count(), 5);
    for definition in case["edits"].as_array().unwrap() {
        assert!(shown.iter().any(|value| value == definition["coordinates"].as_str().unwrap()));
    }
    assert_eq!(policies, 0);
    let movement = &case["decimalDrafts"]["move"];
    assert_eq!(snapshot.pages[0].trim_box.unwrap()[2], 145.0);
    let edit = edit_from_action("set-page-extra", Some(&DslValue::from(box_args(movement)))).unwrap();
    let edits = apply_payload(&snapshot, &edit.action, &edit.payload).unwrap();
    assert_eq!(edits.len(), 1);
    apply_pdf_mutation(&mut snapshot, &edits[0]);
    let moved = &after_edit(&snapshot).pages[0];
    let expected: [f64; 4] = serde_json::from_value(movement["value"].clone()).unwrap();
    assert_eq!(moved.trim_box, Some(expected));
    let malformed = edit_from_action("set-page-extra", Some(&DslValue::from(json!({"page": 1, "object": "box.crop", "field": "text", "value": "0 1 NaN 4"}))));
    assert!(malformed.map_or(true, |edit| apply_payload(&snapshot, &edit.action, &edit.payload).is_err()));
    apply(&mut snapshot, "set-page-extra", box_args(&case["clear"]));
    let cleared = after_edit(&snapshot);
    assert_eq!(cleared.pages[0].crop_box, None);
    assert_eq!(view_size(&cleared), expected_size("clearedViewSize"));
}
