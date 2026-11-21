use semio_framework_artifact_playbook_playbook::flow_forms::PlaybookFlowFormProducer;
use super::*;
use semio_s_artifact_procedural_3d_editor_commands::add_generation;
use semio_s_artifact_procedural_3d_editor_commands::update_generation_values;
use crate::editor_domain::editor_laws::context::{app, dispatch, render as render_body, snapshot, Generation3dAppFixture};
use semio_s_artifact_procedural_3d_editor::Generation3dCommand;

#[semio_framework_async_macros::async_test]
async fn generate_form_hints_without_a_selected_generation() {
    let mut app = app().await;
    assert!(render_body(&mut app, GENERATION_3D_PLAY_BODY_GENERATE_FORM).await.contains("Add a generation"));
}

/// 📝️ The first slider question the bundled fixture's form spec exposes, plus the generation it
/// belongs to — the exact pair a user types into.
async fn first_slider_question(app: &mut Generation3dAppFixture) -> (String, String, f64) {
    dispatch(app, Generation3dCommand::AddGeneration(add_generation::AddGeneration {})).await;
    let read = snapshot(app);
    let generation_id = read.generation.generations.first().expect("addGeneration seeds one generation").id.clone();
    let spec = flow_host_snapshot_to_form_spec(&read.host_snapshot, &PlaybookFlowFormProducer);
    let question = spec
        .steps
        .iter()
        .flat_map(|step| step.blocks.iter())
        .find(|block| block.kind == "slider")
        .expect("the bundled fixture must expose at least one slider input to generate from");
    let question_id = question.id.clone();
    let current = read.generation.generations[0].values.get(&question_id).and_then(semio_framework_value::DslValue::as_f64).unwrap_or_default();
    (generation_id, question_id, current)
}

/// ⚖️ LAW: with a generation selected the Form window stops hinting and renders REAL controls, each
/// one bound to `updateGenerationValues` — the verb the window kind declares. Before this law the
/// only cited evidence for this window was "renders hint or real form chrome"
/// (`📓️audit-user-journey-gaps-2026-09-13.md` gap #8); nothing asserted the change binding exists.
#[semio_framework_async_macros::async_test]
async fn a_selected_generation_turns_the_form_into_bound_controls() {
    let mut app = app().await;
    let (generation_id, question_id, _) = first_slider_question(&mut app).await;
    let body = render_body(&mut app, GENERATION_3D_PLAY_BODY_GENERATE_FORM).await;
    assert!(!body.contains("Add a generation"), "a selected generation must retire the hint: {body}");
    assert!(body.contains("updateGenerationValues"), "every form control must dispatch updateGenerationValues: {body}");
    assert!(body.contains(&format!("generate.form.{question_id}")), "the slider question must render its own field: {body}");
    assert!(body.contains(&generation_id), "each control must carry the generation it edits: {body}");
}

/// ⚖️ LAW: editing a form value round-trips into the PATCHED fixture the generate preview evaluates.
/// `updateGenerationValues` writes the roster entry, and `generation_host_snapshot_for` — the one input the
/// generate preview's mesh is computed from (`flow_eval_tick::evaluate`) — carries the new number onto
/// the matching widget. That is the whole mechanism by which typing into the Form window changes the
/// preview mesh; the browser step in `🐍️generate-mode-probe.mjs` proves the mesh itself.
#[semio_framework_async_macros::async_test]
async fn editing_a_form_value_repatches_the_generate_preview_fixture() {
    let mut app = app().await;
    let (generation_id, question_id, before) = first_slider_question(&mut app).await;
    let next = before + 3.0;
    dispatch(
        &mut app,
        Generation3dCommand::UpdateGenerationValues(update_generation_values::UpdateGenerationValues {
            generation_id: Some(generation_id.clone()),
            question_id: question_id.clone(),
            value: semio_framework_value::DslValue::float(next),
        }),
    )
    .await;

    let read = snapshot(&app);
    let stored = read.generation.generations[0].values.get(&question_id).and_then(semio_framework_value::DslValue::as_f64);
    assert_eq!(stored, Some(next), "updateGenerationValues must persist the typed number on the generation");
    let patched = semio_s_artifact_procedural_3d_model::generation_host_snapshot_for(&read.host_snapshot, &read.generation, Some(generation_id.as_str()));
    let patched_value = patched
        .widgets
        .iter()
        .find_map(|widget| match widget {
            semio_framework_artifact_flow_flow::Widget::InputSlider { id, value, .. } if *id == question_id => Some(*value),
            _ => None,
        })
        .expect("the patched generate fixture must still carry the edited slider widget");
    patched.retire_cold();
    assert_eq!(patched_value, next, "the generate preview evaluates the PATCHED fixture, so the edited value has to reach it");

    let body = render_body(&mut app, GENERATION_3D_PLAY_BODY_GENERATE_FORM).await;
    assert!(body.contains(&next.to_string()), "the form must render back the value it just committed: {body}");
}

#[semio_framework_async_macros::async_test]
async fn generated_form_declares_a_named_viewport_and_uses_the_shared_field_keys() {
    use semio_framework_artifact_flow_flow::{FlowHostSnapshot, Widget};
    use semio_s_artifact_procedural_3d_terminology::Generation3dLabels;
    use semio_ui_contract::{Component, ContainerRole, LayoutSpec};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/form-panel.json")).unwrap();
    let mut app = app().await;
    let (generation_id, _, _) = first_slider_question(&mut app).await;
    let read = snapshot(&app);
    let mut host = FlowHostSnapshot::default();
    let fields = fixture["fields"].as_array().unwrap();
    for field in fields {
        let id = field["id"].as_str().unwrap();
        let label = field["label"].as_str().unwrap();
        host.widgets.push(if field["kind"] == "slider" {
            Widget::InputSlider { id: id.into(), label: label.into(), value: field["value"].as_f64().unwrap(), min: field["min"].as_f64().unwrap(), max: field["max"].as_f64().unwrap(), step: field["step"].as_f64().unwrap() }
        } else {
            Widget::Variable { id: id.into(), name: id.into(), schema: field["schema"].as_str().unwrap().into() }
        });
    }
    for row in fixture["locales"].as_array().unwrap() {
        let (locale, labels) = if row["id"] == "de" {
            (semio_ui_locale::Locale::De, &Generation3dLabels::NATIVE_DE)
        } else {
            (semio_ui_locale::Locale::En, &Generation3dLabels::NATIVE_EN)
        };
        let view = semio_plugin::ViewModel::new(locale, semio_ui_locale::Terminology::Native);
        let form = render(&host, &read.generation, Some(&generation_id), &view, labels).unwrap();
        let Component::Container(props) = &form.component else { panic!("form landmark") };
        assert_eq!(props.role, ContainerRole::Form);
        assert_eq!(props.label.as_ref().unwrap().0.as_str(), row["label"].as_str().unwrap());
        assert_eq!(form.key.as_str(), fixture["panelKey"].as_str().unwrap());
        assert!(matches!(&form.layout, LayoutSpec::Scroll(_)));
        assert_eq!(serde_json::to_value(&form.layout).unwrap(), fixture["scroll"]);
        let stack = &form.children[0];
        assert_eq!(stack.key.as_str(), fixture["fieldsKey"].as_str().unwrap());
        for field in fields {
            let key = format!("generate.form.{}", field["id"].as_str().unwrap());
            let definition = stack.children.iter().find(|node| node.key.as_str() == key).expect("field");
            let control = &definition.children[0];
            assert_eq!(control.key.as_str(), format!("{key}.control"));
            assert_eq!(control.accessibility.label.as_ref().unwrap().0.as_str(), field["label"].as_str().unwrap());
            assert_eq!(control.bindings.len(), 1);
        }
        semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::built_to_component_tree(form)).unwrap();
    }
    host.retire_cold();
}

#[semio_framework_async_macros::async_test]
async fn a_form_page_renders_only_its_original_question_addresses_inside_the_record_limit() {
    use semio_framework_artifact_flow_flow::{FlowHostSnapshot, Widget};
    use semio_s_artifact_procedural_3d_terminology::Generation3dLabels;
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../../../../🔌️plugins/🌀️procedural/🫀️foundation/🖼️semantic-ui/📝️form/pages.json")).unwrap();
    let case = corpus["cases"].as_array().unwrap().iter().find(|case| case["name"] == "long scalar form").unwrap();
    let count = case["fields"][0]["count"].as_u64().unwrap() as usize;
    let mut host = FlowHostSnapshot::default();
    for index in 0..count {
        host.widgets.push(Widget::InputSlider { id: format!("size-{index:03}"), label: format!("Size {index}"), value: 1.0, min: 0.0, max: 10.0, step: 1.0 });
    }
    let mut app = app().await;
    let (generation_id, _, _) = first_slider_question(&mut app).await;
    let read = snapshot(&app);
    let labels = &Generation3dLabels::NATIVE_EN;
    let window_id = "generate-form-roster".to_owned();
    let mut view = semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native);
    view.window_id = Some(window_id.clone());
    let mut all = Vec::new();
    let total = case["expect"]["total"].as_u64().unwrap() as usize;
    for requested in 0..total {
        view.tree_windows = vec![semio_plugin::TreeWindowRequest {
            body_key: GENERATION_3D_PLAY_BODY_GENERATE_FORM.into(), node_key: window_id.clone(), open: None, offset: requested as u32, rows: 1,
        }];
        let surface = render(&host, &read.generation, Some(&generation_id), &view, labels).unwrap();
        let navigation = &surface.children[0];
        assert_eq!(navigation.children[0].disabled, requested == 0);
        assert_eq!(navigation.children[2].disabled, requested + 1 == total);
        let action = &navigation.children[if requested == 0 { 2 } else { 0 }];
        let binding = serde_json::to_value(action.bindings.iter().next().unwrap()).unwrap();
        assert_eq!(binding["args"]["windowId"], window_id);
        assert_eq!(binding["action"]["name"], "setFormPage");
        let fields = &surface.children[1].children[0].children;
        assert!(7 + 2 * fields.len() <= semio_ui_contract::UI_DOCUMENT_NODES);
        if requested == 1 {
            assert_eq!(fields.len(), (case["expect"]["last"].as_u64().unwrap() - case["expect"]["first"].as_u64().unwrap()) as usize);
        }
        for node in fields {
            assert_eq!(node.children[0].bindings.len(), 1);
            all.push(node.key.as_str().to_owned());
        }
        semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::built_to_component_tree(surface)).unwrap();
    }
    assert_eq!(all, (0..count).map(|index| format!("generate.form.size-{index:03}")).collect::<Vec<_>>());
    host.retire_cold();
}

#[semio_framework_async_macros::async_test]
async fn authored_form_images_use_the_accessible_image_component() {
    use semio_framework_artifact_flow_flow::{FlowHostSnapshot, Widget};
    use semio_s_artifact_procedural_3d_terminology::Generation3dLabels;
    use semio_ui_contract::Component;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/form-image.json")).unwrap();
    let mut app = app().await;
    let (generation_id, _, _) = first_slider_question(&mut app).await;
    let read = snapshot(&app);
    for (locale, row) in fixture.as_object().unwrap() {
        let blocks = row["steps"][0]["blocks"].as_array().unwrap();
        let mut host = FlowHostSnapshot::default();
        for block in blocks {
            host.widgets.push(Widget::InputImage {
                id: block["id"].as_str().unwrap().into(),
                src: block["src"].as_str().unwrap_or_default().into(),
            });
        }
        let (locale, labels) = if locale == "de" {
            (semio_ui_locale::Locale::De, &Generation3dLabels::NATIVE_DE)
        } else {
            (semio_ui_locale::Locale::En, &Generation3dLabels::NATIVE_EN)
        };
        let view = semio_plugin::ViewModel::new(locale, semio_ui_locale::Terminology::Native);
        let form = render(&host, &read.generation, Some(&generation_id), &view, labels).unwrap();
        let controls: Vec<_> = form.children[0].children.iter().flat_map(|field| field.children.iter()).collect();
        let expected_key = format!("generate.form.{}.image", blocks[0]["id"].as_str().unwrap());
        let image = controls.iter().find(|node| node.key.as_str() == expected_key).expect("authored image");
        let Component::Image(props) = &image.component else { panic!("{locale}: expected an image component") };
        let source = blocks[0]["src"].as_str().unwrap();
        assert_eq!(source.len() as u64, row["sourceBytes"].as_u64().unwrap());
        if source.len() > semio_ui_contract::UI_TEXT_MAX_BYTES {
            assert_eq!(props.source_body_bytes, Some(source.len() as u32));
            let carrier = image.children.get(0).expect("one retained image source carrier");
            assert_eq!(image.children.len(), 1);
            assert_eq!(carrier.key.as_str(), props.src.as_str());
            assert_eq!(semio_plugin::artifact_app_laws::built_carrier_text(carrier), source);
        } else {
            assert_eq!(props.source_body_bytes, None);
            assert_eq!(props.src.as_str(), source);
            assert!(image.children.is_empty());
        }
        assert_eq!(props.alt.as_ref().unwrap().0.as_str(), blocks[0]["label"].as_str().unwrap());
        assert_eq!(image.accessibility.label.as_ref().unwrap().0.as_str(), blocks[0]["label"].as_str().unwrap());
        let missing = controls.iter().find(|node| node.key.as_str() == "generate.form.missing.image").expect("missing image placeholder");
        assert!(matches!(&missing.component, Component::Text(_)), "{locale}");
        assert!(format!("{missing:?}").contains(row["fallback"].as_str().unwrap()), "{locale}");
        semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::built_to_component_tree(form)).unwrap();
        host.retire_cold();
    }
}

#[semio_framework_async_macros::async_test]
async fn retained_form_images_budget_their_complete_payload_body() {
    use semio_framework_artifact_flow_flow::{FlowHostSnapshot, Widget};
    use semio_s_artifact_procedural_3d_terminology::Generation3dLabels;
    use semio_ui_contract::{BuiltNode, Component};
    fn nodes(node: &BuiltNode) -> usize {
        1 + node.children.iter().map(nodes).sum::<usize>()
    }

    let images: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/form-image.json")).unwrap();
    let source = images["en"]["steps"][0]["blocks"][0]["src"].as_str().unwrap();
    let cases: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../../../../🔌️plugins/🌀️procedural/🫀️foundation/🖼️semantic-ui/📝️form/pages.json")).unwrap();
    let case = cases["cases"].as_array().unwrap().iter().find(|case| case["name"] == "retained image sources").unwrap();
    let count = case["fields"][0]["count"].as_u64().unwrap() as usize;
    let split = case["expect"]["first"].as_u64().unwrap() as usize;
    assert_eq!(2 + semio_plugin::paged_text_carrier_nodes(source), case["fields"][0]["nodes"].as_u64().unwrap() as usize);
    let mut host = FlowHostSnapshot::default();
    for index in 0..count {
        host.widgets.push(Widget::InputImage { id: format!("guide-{index:03}"), src: source.into() });
    }
    let mut app = app().await;
    let (generation_id, _, _) = first_slider_question(&mut app).await;
    let read = snapshot(&app);
    let mut view = semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native);
    let window = "generate-form-images";
    view.window_id = Some(window.into());
    for (page, expected) in [(0, split), (1, count - split)] {
        view.tree_windows = vec![semio_plugin::TreeWindowRequest {
            body_key: GENERATION_3D_PLAY_BODY_GENERATE_FORM.into(), node_key: window.into(), open: None, offset: page, rows: 1,
        }];
        let form = render(&host, &read.generation, Some(&generation_id), &view, &Generation3dLabels::NATIVE_EN).unwrap();
        let fields = &form.children[1].children[0].children;
        assert_eq!(fields.len(), expected);
        assert!(fields.iter().all(|field| matches!(&field.children[0].component, Component::Image(props) if props.source_body_bytes.is_some())));
        assert!(nodes(&form) <= semio_ui_contract::UI_DOCUMENT_NODES);
        semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::built_to_component_tree(form)).unwrap();
    }
    host.retire_cold();
}
