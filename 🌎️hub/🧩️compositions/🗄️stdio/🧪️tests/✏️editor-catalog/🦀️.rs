//! ✏️ Every catalog editor declares, parses, and replays the complete editing contract.

use semio_framework_os_kernel::{ArtifactDsl, ArtifactPack, DslValue, Mutation, MutationDiff, OpBinary, OpText, ToValue};
use semio_framework_plugin::app::EditorSurfaceApp;
use semio_framework_plugin::{artifact_app_laws, AppActionRegistry, AppDefinition, ArtifactEditor, EditorApp, InteractiveJobClassification, PluginApp};
use semio_s_artifact_stdio_contract::editing::{SnapshotEditingEditor, SNAPSHOT_EDIT_ACTION_IDS};
use std::collections::BTreeSet;

/// 🧩️ Assembles every stdio package once: assembly publishes the artifact document schemas an editor's snapshot edits
/// resolve (`snapshot-edit.schema-unregistered` otherwise), exactly as a host that loaded the stdio packages has them.
fn stdio_packages_assembled() {
    static ASSEMBLED: std::sync::OnceLock<()> = std::sync::OnceLock::new();
    ASSEMBLED.get_or_init(|| {
        semio_hub_stdio::plugin().expect("stdio assembles");
        semio_hub_stdio_image::plugin().expect("stdio-image assembles");
        semio_hub_stdio_media::plugin().expect("stdio-media assembles");
        semio_hub_stdio_cad::plugin().expect("stdio-cad assembles");
        semio_hub_stdio_bim::plugin().expect("stdio-bim assembles");
        semio_hub_stdio_mesh::plugin().expect("stdio-mesh assembles");
        semio_hub_stdio_pdf::plugin().expect("stdio-pdf assembles");
        semio_hub_stdio_office::plugin().expect("stdio-office assembles");
        semio_hub_stdio_semio::plugin().expect("stdio-semio assembles");
        semio_hub_stdio_binary::plugin().expect("stdio-binary assembles");
    });
}

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/✏️editor-catalog/🔣️.json")).expect("neutral editor acceptance fixture")
}

fn snapshot_json<S: ToValue>(snapshot: &S) -> serde_json::Value {
    serde_json::from_str(&pack::json::to_json_string(&snapshot.to_value())).expect("independent snapshot oracle")
}

fn assert_detail_routes(root: &serde_json::Value, app_id: &str) {
    let mut frontier = vec![root];
    let mut controls = 0;
    while let Some(node) = frontier.pop() {
        for binding in node["bindings"].as_array().unwrap() {
            let action = &binding["action"];
            if SNAPSHOT_EDIT_ACTION_IDS.contains(&action["name"].as_str().unwrap()) {
                assert_eq!(action["scope"].as_str(), Some(app_id), "visible detail controls must address their exact subset editor");
                controls += 1;
            }
        }
        frontier.extend(node["children"].as_array().unwrap());
    }
    assert!(controls > 0, "{app_id} must render usable detail controls");
}

async fn assert_editor<E: ArtifactEditor + SnapshotEditingEditor>(definition: AppDefinition) {
    stdio_packages_assembled();
    let fixture = fixture();
    let details = fixture["detailsWindow"].as_str().unwrap();
    let details_kind = definition.window_kinds.iter().find(|kind| kind.id == details).unwrap_or_else(|| panic!("{} needs editable details", definition.id));
    for row in fixture["actions"].as_array().unwrap() {
        let id = row["id"].as_str().unwrap();
        let action = details_kind.actions.iter().find(|action| action.id == id).unwrap_or_else(|| panic!("{} details window is missing {id}", definition.id));
        assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "{} {id}", definition.id);
        assert!(E::Command::TOOL_JOB_IDS.contains(&id), "{} has no retained {id}", definition.id);
        let source = serde_json::to_string(&row["arguments"]).unwrap();
        let args: DslValue = pack::json::from_json_str(&source).expect("native action argument decoder");
        let native_json = pack::json::to_json_string(&args);
        assert_eq!(serde_json::from_str::<serde_json::Value>(&native_json).unwrap(), row["arguments"], "independent argument codec oracle {id}");
        let command = E::command_from_action(id, Some(&args)).unwrap_or_else(|error| panic!("{} cannot parse {id}: {error:?}", definition.id));
        assert_eq!(E::command_id(&command), id);
        let bytes = command.encode_op().expect("editor command encoding");
        let replay = E::Command::decode_op(&bytes).expect("editor command replay");
        assert_eq!(E::command_id(&replay), id);
        assert_eq!(replay.encode_op().unwrap(), bytes, "replay preserves every argument of {id}");
    }
    let registry = AppActionRegistry::from_definition(&definition);
    let mut app = EditorSurfaceApp::<E>::with_registry(EditorApp::<E>::default(), registry).await;
    let family = definition.id.strip_prefix("s.stdio.").unwrap().split('@').next().unwrap();
    let edits = &fixture["snapshotEdits"];
    let case = edits.get(&definition.id).or_else(|| edits.get(family)).unwrap_or_else(|| panic!("{} needs a meaningful edit fixture", definition.id));
    let base = E::initial_snapshot();
    let before = snapshot_json(&base);
    let schema_identity = |schema: &str| schema.trim_start_matches("s.").to_string();
    assert_eq!(before["schema"].as_str().map(schema_identity), Some(schema_identity(E::DOCUMENT_SCHEMA)), "{} edits the document schema of its own snapshot model", definition.id);
    for row in fixture["rejectedActions"].as_array().unwrap() {
        let arguments: DslValue = pack::json::from_json_str(&row["arguments"].to_string()).unwrap();
        let command = E::command_from_action(row["id"].as_str().unwrap(), Some(&arguments)).expect("invalid detail is still a well-formed command");
        assert!(artifact_app_laws::reduce_editor_command::<E>(&command, &base).is_err(), "{} direct command must protect schema identity", definition.id);
        assert_eq!(snapshot_json(&base), before, "{} refused direct command preserves its snapshot", definition.id);
    }
    let mut after = before.clone();
    let path = case["path"].as_str().unwrap();
    let unchanged_arguments = serde_json::json!({ "path": path, "value": before.pointer(path).expect("no-op fixture path") });
    let unchanged_arguments: DslValue = pack::json::from_json_str(&unchanged_arguments.to_string()).unwrap();
    let unchanged_command = E::command_from_action("setSnapshotValue", Some(&unchanged_arguments)).unwrap();
    let unchanged_event = E::snapshot_edit_event(&unchanged_command).unwrap();
    assert!(E::snapshot_edit_is_admitted(unchanged_event, &base), "{} must admit unchanged field values", definition.id);
    let unchanged = artifact_app_laws::reduce_editor_command::<E>(&unchanged_command, &base).expect("unchanged field is a successful no-op");
    assert!(unchanged.artifact_mutations.is_empty(), "{} unchanged field must not create a history event", definition.id);
    *after.pointer_mut(path).unwrap_or_else(|| panic!("{} has no editable fixture path {path}", definition.id)) = case["value"].clone();
    assert_ne!(before, after, "{} fixture must change a real detail", definition.id);
    let arguments = serde_json::json!({ "path": path, "value": case["value"] });
    let arguments: DslValue = pack::json::from_json_str(&arguments.to_string()).unwrap();
    let command = E::command_from_action("setSnapshotValue", Some(&arguments)).expect("typed detail command");
    let event = E::snapshot_edit_event(&command).expect("detail event is routed");
    assert!(E::snapshot_edit_is_admitted(event, &base), "{} initial document must admit a detail edit", definition.id);
    let emit = artifact_app_laws::reduce_editor_command::<E>(&command, &base).expect("direct detail reducer accepts valid fixture");
    assert!(!emit.artifact_mutations.is_empty(), "{} must publish an artifact event", definition.id);
    let mut projected = base.clone();
    let mut inverses = Vec::new();
    for mutation in emit.artifact_mutations {
        let binary = mutation.encode_op().expect("native mutation binary encoding");
        assert_eq!(E::Mutation::decode_op(&binary).expect("native mutation binary replay").encode_op().unwrap(), binary);
        assert_eq!(E::Mutation::parse_op(&mutation.print_op()).expect("native mutation text replay").encode_op().unwrap(), binary);
        inverses.push(mutation.inverse(&projected));
        projected = mutation.diff(&projected).diff().apply(&projected).expect("native detail diff applies");
    }
    assert_eq!(snapshot_json(&projected), after, "{} native event must preserve every other detail", definition.id);
    for mutations in inverses.into_iter().rev() {
        for mutation in mutations {
            projected = mutation.diff(&projected).diff().apply(&projected).expect("native inverse applies");
        }
    }
    assert_eq!(snapshot_json(&projected), before, "{} inverse restores the complete snapshot", definition.id);
    app.bind_instance_id(artifact_app_laws::meta("local").instance_id).await;
    for locale in [semio_framework_plugin::Locale::En, semio_framework_plugin::Locale::De] {
        let view = semio_framework_plugin::ViewModel {
            active_mode_id: Some(definition.default_mode_id.clone()),
            active_window_kind_id: Some(details.into()),
            window_id: Some("editor-catalog-details".into()),
            focused_window_id: Some("editor-catalog-details".into()),
            window_instances: vec![semio_framework::ViewWindowInstance { id: "editor-catalog-details".into(), window_kind_id: details.into() }],
            locale,
            ..Default::default()
        };
        let tree = app.render(semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY, None, &view).await.expect("real localized details render");
        let projection = artifact_app_laws::project_and_retire_fixture_tree(tree).expect("details have valid bounded UI structure");
        assert_detail_routes(&serde_json::from_str(&projection).unwrap(), &definition.id);
    }
    let arguments = match arguments { DslValue::Object(entries) => entries.into_iter().collect(), _ => unreachable!() };
    let invocation = semio_framework::manifest::ActionInvocation {
        address: semio_framework::manifest::ActionAddress {
            plugin_id: "stdio".into(),
            app_id: definition.id.clone(),
            mode_id: definition.default_mode_id.clone(),
            window_kind_id: details.into(),
            window_instance_id: "editor-catalog-details".into(),
            action_id: "setSnapshotValue".into(),
        },
        arguments,
    };
    app.handle_action_invocation(&invocation, Some(&definition.default_mode_id), &artifact_app_laws::meta("local")).await.expect("host-addressed detail action");
    artifact_app_laws::settle_registered_typed_operation(&mut app, 1).await.expect("detail action publishes");
    assert_eq!(snapshot_json(&app.snapshot().unwrap()), after, "{} addressed action changes persisted details", definition.id);
    artifact_app_laws::settle_history_verb(&mut app, "undo", 1).await;
    assert_eq!(snapshot_json(&app.snapshot().unwrap()), before, "{} undo restores every detail", definition.id);
    artifact_app_laws::settle_history_verb(&mut app, "redo", 1).await;
    assert_eq!(snapshot_json(&app.snapshot().unwrap()), after, "{} redo restores the complete edit", definition.id);
    let saved = app.snapshot().unwrap().encode_pack();
    let reopened = E::Snapshot::decode_pack(&saved).expect("saved artifact reopens");
    assert_eq!(snapshot_json(&reopened), after, "{} saved artifact preserves every edited detail", definition.id);
    let source = semio_s_artifact_stdio_contract::editing::snapshot_edit_source(&reopened);
    let source_reopened = semio_s_artifact_stdio_contract::editing::snapshot_from_edit_source::<E::Snapshot>(&source).expect("complete editable source reopens");
    assert_eq!(snapshot_json(&source_reopened), after, "{} editable source preserves every saved detail", definition.id);
    for _ in 0..65_536 {
        if app.close_terminal_is_empty() {
            break;
        }
        app.close_step(1, semio_framework_os_kernel::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("registered editor closes retained owners");
    }
    assert!(app.close_terminal_is_empty(), "{} closes every retained factory", definition.id);
}

#[test]
fn neutral_catalog_requires_each_edit_operation_once() {
    let fixture = fixture();
    let expected = SNAPSHOT_EDIT_ACTION_IDS.iter().copied().collect::<BTreeSet<_>>();
    let actual = fixture["actions"].as_array().unwrap().iter().map(|row| row["id"].as_str().unwrap()).collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
    assert_eq!(actual.len(), fixture["actions"].as_array().unwrap().len());
    assert_eq!(fixture["editorCount"].as_u64().unwrap() as usize, EDITOR_COUNT);
}

async fn assert_sqlite_snapshot_editor<E: ArtifactEditor>() {
    use semio_framework::io::io_mechanism::{io_entries, io_identify, io_route, io_run};
    use semio_framework::io_schema::{Confidence, IoFidelity, IoPayload, SQLITE_SNAPSHOT};
    stdio_packages_assembled();
    let native = E::DIALECT.into();
    let sqlite = SQLITE_SNAPSHOT.into();
    let entries = io_entries();
    if E::Snapshot::sqlite_snapshot_codec().is_none() {
        assert!(!entries.iter().any(|entry| (entry.from == native && entry.into == sqlite) || (entry.from == sqlite && entry.into == native)), "undeclared relational capability must publish no SQLite route");
        return;
    }
    assert!(entries.iter().any(|entry| entry.from == native && entry.into == sqlite && entry.fidelity == IoFidelity::Exact));
    assert!(entries.iter().any(|entry| entry.from == sqlite && entry.into == native && entry.fidelity == IoFidelity::Exact));
    let export = io_route(&native, &sqlite, 1).await.expect("shipped editor SQLite export route").value;
    let import = io_route(&sqlite, &native, 1).await.expect("shipped editor SQLite import route").value;
    let snapshot = E::initial_snapshot();
    let original = snapshot_json(&snapshot);
    for payload in [IoPayload::Binary(snapshot.encode_pack()), IoPayload::Text(snapshot.print_dsl())] {
        let database = io_run(&export, payload.clone()).await.expect("shipped editor SQLite export").value;
        let IoPayload::Binary(bytes) = &database else { panic!("SQLite snapshot file is binary") };
        assert!(bytes.starts_with(b"SQLite format 3\0"), "snapshot export must be a SQLite file");
        assert_eq!(io_identify(&database).await, vec![(sqlite.clone(), Confidence::High)]);
        let restored = io_run(&import, database).await.expect("shipped editor SQLite import").value;
        assert_eq!(restored, payload);
        let decoded = match restored {
            IoPayload::Binary(bytes) => E::Snapshot::decode_pack(&bytes).expect("restored native pack"),
            IoPayload::Text(text) => E::Snapshot::parse_dsl(&text).expect("restored native DSL"),
        };
        assert_eq!(snapshot_json(&decoded), original, "independent snapshot oracle for {}", native.to_coordinate());
    }
}

macro_rules! editor_catalog_laws {
    ($(($name:ident, $sqlite_name:ident, $editor:ty, $definition:path)),+ $(,)?) => {
        const EDITOR_COUNT: usize = [$(stringify!($editor)),+].len();
        $(#[semio_framework_async_macros::async_test] async fn $name() { assert_editor::<$editor>($definition()).await; })+
        $(#[semio_framework_async_macros::async_test] async fn $sqlite_name() { assert_sqlite_snapshot_editor::<$editor>().await; })+
    };
}

editor_catalog_laws! {
    (png_editor, sqlite_snapshot_png_editor, semio_s_artifact_stdio_png::editor::png::PngEditor, semio_s_artifact_stdio_png::editor::png::create_png_editor),
    (jpg_any_editor, sqlite_snapshot_jpg_any_editor, semio_s_artifact_stdio_jpg::editor::jpg_any::JpgAnyEditor, semio_s_artifact_stdio_jpg::editor::jpg_any::create_jpg_any_editor),
    (jpg_baseline_editor, sqlite_snapshot_jpg_baseline_editor, semio_s_artifact_stdio_jpg::editor::jpg_baseline::JpgBaselineEditor, semio_s_artifact_stdio_jpg::editor::jpg_baseline::create_jpg_baseline_editor),
    (bmp_editor, sqlite_snapshot_bmp_editor, semio_s_artifact_stdio_bmp::editor::bmp::BmpEditor, semio_s_artifact_stdio_bmp::editor::bmp::create_bmp_editor),
    (tiff_any_editor, sqlite_snapshot_tiff_any_editor, semio_s_artifact_stdio_tiff::editor::tiff_any::TiffAnyEditor, semio_s_artifact_stdio_tiff::editor::tiff_any::create_tiff_any_editor),
    (tiff_baseline_editor, sqlite_snapshot_tiff_baseline_editor, semio_s_artifact_stdio_tiff::editor::tiff_baseline::TiffBaselineEditor, semio_s_artifact_stdio_tiff::editor::tiff_baseline::create_tiff_baseline_editor),
    (gif87a_editor, sqlite_snapshot_gif87a_editor, semio_s_artifact_stdio_gif::editor::gif_87a::Gif87aEditor, semio_s_artifact_stdio_gif::editor::gif_87a::create_gif_87a_editor),
    (gif89a_editor, sqlite_snapshot_gif89a_editor, semio_s_artifact_stdio_gif::editor::gif_89a::Gif89aEditor, semio_s_artifact_stdio_gif::editor::gif_89a::create_gif_89a_editor),
    (svg_any_editor, sqlite_snapshot_svg_any_editor, semio_s_artifact_stdio_svg::editor::svg_any::SvgAnyEditor, semio_s_artifact_stdio_svg::editor::svg_any::create_svg_any_editor),
    (svg_basic_editor, sqlite_snapshot_svg_basic_editor, semio_s_artifact_stdio_svg::editor::svg_basic::SvgBasicEditor, semio_s_artifact_stdio_svg::editor::svg_basic::create_svg_basic_editor),
    (svg_tiny_editor, sqlite_snapshot_svg_tiny_editor, semio_s_artifact_stdio_svg::editor::svg_tiny::SvgTinyEditor, semio_s_artifact_stdio_svg::editor::svg_tiny::create_svg_tiny_editor),
    (mp4_editor, sqlite_snapshot_mp4_editor, semio_s_artifact_stdio_mp4::editor::mp4::Mp4Editor, semio_s_artifact_stdio_mp4::editor::mp4::create_mp4_editor),
    (mp3_editor, sqlite_snapshot_mp3_editor, semio_s_artifact_stdio_mp3::editor::mp3::Mp3Editor, semio_s_artifact_stdio_mp3::editor::mp3::create_mp3_editor),
    (wav_editor, sqlite_snapshot_wav_editor, semio_s_artifact_stdio_wav::editor::wav::WavEditor, semio_s_artifact_stdio_wav::editor::wav::create_wav_editor),
    (avi_editor, sqlite_snapshot_avi_editor, semio_s_artifact_stdio_avi::editor::avi::AviEditor, semio_s_artifact_stdio_avi::editor::avi::create_avi_editor),
    (html_editor, sqlite_snapshot_html_editor, semio_s_artifact_stdio_html::editor::html::HtmlEditor, semio_s_artifact_stdio_html::editor::html::create_html_editor),
    (md_editor, sqlite_snapshot_md_editor, semio_s_artifact_stdio_md::editor::md::MdEditor, semio_s_artifact_stdio_md::editor::md::create_md_editor),
    (semio_brep_editor, sqlite_snapshot_semio_brep_editor, semio_s_artifact_stdio_semio::editor::semio_brep::SemioBrepEditor, semio_s_artifact_stdio_semio::editor::semio_brep::create_semio_brep_editor),
    (semio_drawing_editor, sqlite_snapshot_semio_drawing_editor, semio_s_artifact_stdio_semio::editor::semio_drawing::SemioDrawingEditor, semio_s_artifact_stdio_semio::editor::semio_drawing::create_semio_drawing_editor),
    (semio_graph_editor, sqlite_snapshot_semio_graph_editor, semio_s_artifact_stdio_semio::editor::semio_graph::SemioGraphEditor, semio_s_artifact_stdio_semio::editor::semio_graph::create_semio_graph_editor),
    (semio_kit_editor, sqlite_snapshot_semio_kit_editor, semio_s_artifact_stdio_semio::editor::semio_kit::SemioKitEditor, semio_s_artifact_stdio_semio::editor::semio_kit::create_semio_kit_editor),
    (semio_mesh_editor, sqlite_snapshot_semio_mesh_editor, semio_s_artifact_stdio_semio::editor::semio_mesh::SemioMeshEditor, semio_s_artifact_stdio_semio::editor::semio_mesh::create_semio_mesh_editor),
    (semio_object_editor, sqlite_snapshot_semio_object_editor, semio_s_artifact_stdio_semio::editor::semio_object::SemioObjectEditor, semio_s_artifact_stdio_semio::editor::semio_object::create_semio_object_editor),
    (semio_table_editor, sqlite_snapshot_semio_table_editor, semio_s_artifact_stdio_semio::editor::semio_table::SemioTableEditor, semio_s_artifact_stdio_semio::editor::semio_table::create_semio_table_editor),
    (semio_text_editor, sqlite_snapshot_semio_text_editor, semio_s_artifact_stdio_semio::editor::semio_text::SemioTextEditor, semio_s_artifact_stdio_semio::editor::semio_text::create_semio_text_editor),
    (semio_animation_editor, sqlite_snapshot_semio_animation_editor, semio_s_artifact_stdio_semio::editor::semio_animation::SemioAnimationEditor, semio_s_artifact_stdio_semio::editor::semio_animation::create_semio_animation_editor),
    (semio_any_editor, sqlite_snapshot_semio_any_editor, semio_s_artifact_stdio_semio::editor::semio_base::SemioAnyEditor, semio_s_artifact_stdio_semio::editor::semio_base::create_semio_base_editor),
    (semio_audio_editor, sqlite_snapshot_semio_audio_editor, semio_s_artifact_stdio_semio::editor::semio_audio::SemioAudioEditor, semio_s_artifact_stdio_semio::editor::semio_audio::create_semio_audio_editor),
    (semio_cad_editor, sqlite_snapshot_semio_cad_editor, semio_s_artifact_stdio_semio::editor::semio_cad::SemioCadEditor, semio_s_artifact_stdio_semio::editor::semio_cad::create_semio_cad_editor),
    (semio_document_editor, sqlite_snapshot_semio_document_editor, semio_s_artifact_stdio_semio::editor::semio_document::SemioDocumentEditor, semio_s_artifact_stdio_semio::editor::semio_document::create_semio_document_editor),
    (semio_flow_editor, sqlite_snapshot_semio_flow_editor, semio_s_artifact_stdio_semio::editor::semio_flow::SemioFlowEditor, semio_s_artifact_stdio_semio::editor::semio_flow::create_semio_flow_editor),
    (semio_image_editor, sqlite_snapshot_semio_image_editor, semio_s_artifact_stdio_semio::editor::semio_image::SemioImageEditor, semio_s_artifact_stdio_semio::editor::semio_image::create_semio_image_editor),
    (semio_model_editor, sqlite_snapshot_semio_model_editor, semio_s_artifact_stdio_semio::editor::semio_model::SemioModelEditor, semio_s_artifact_stdio_semio::editor::semio_model::create_semio_model_editor),
    (semio_presentation_editor, sqlite_snapshot_semio_presentation_editor, semio_s_artifact_stdio_semio::editor::semio_presentation::SemioPresentationEditor, semio_s_artifact_stdio_semio::editor::semio_presentation::create_semio_presentation_editor),
    (semio_value_editor, sqlite_snapshot_semio_value_editor, semio_s_artifact_stdio_semio::editor::semio_value::SemioValueEditor, semio_s_artifact_stdio_semio::editor::semio_value::create_semio_value_editor),
    (semio_video_editor, sqlite_snapshot_semio_video_editor, semio_s_artifact_stdio_semio::editor::semio_video::SemioVideoEditor, semio_s_artifact_stdio_semio::editor::semio_video::create_semio_video_editor),
    (step_any_editor, sqlite_snapshot_step_any_editor, semio_s_artifact_stdio_step::editor::step_any::StepAnyEditor, semio_s_artifact_stdio_step::editor::step_any::create_step_any_editor),
    (step_cc1_editor, sqlite_snapshot_step_cc1_editor, semio_s_artifact_stdio_step::editor::step_cc1::StepCc1Editor, semio_s_artifact_stdio_step::editor::step_cc1::create_step_cc1_editor),
    (step_cc2_editor, sqlite_snapshot_step_cc2_editor, semio_s_artifact_stdio_step::editor::step_cc2::StepCc2Editor, semio_s_artifact_stdio_step::editor::step_cc2::create_step_cc2_editor),
    (step_cc3_editor, sqlite_snapshot_step_cc3_editor, semio_s_artifact_stdio_step::editor::step_cc3::StepCc3Editor, semio_s_artifact_stdio_step::editor::step_cc3::create_step_cc3_editor),
    (step_cc4_editor, sqlite_snapshot_step_cc4_editor, semio_s_artifact_stdio_step::editor::step_cc4::StepCc4Editor, semio_s_artifact_stdio_step::editor::step_cc4::create_step_cc4_editor),
    (step_cc5_editor, sqlite_snapshot_step_cc5_editor, semio_s_artifact_stdio_step::editor::step_cc5::StepCc5Editor, semio_s_artifact_stdio_step::editor::step_cc5::create_step_cc5_editor),
    (step_cc6_editor, sqlite_snapshot_step_cc6_editor, semio_s_artifact_stdio_step::editor::step_cc6::StepCc6Editor, semio_s_artifact_stdio_step::editor::step_cc6::create_step_cc6_editor),
    (ifc2x3_any_editor, sqlite_snapshot_ifc2x3_any_editor, semio_s_artifact_stdio_ifc::editor::ifc2x3_any::Ifc2x3AnyEditor, semio_s_artifact_stdio_ifc::editor::ifc2x3_any::create_ifc2x3_any_editor),
    (ifc2x3_cobie_editor, sqlite_snapshot_ifc2x3_cobie_editor, semio_s_artifact_stdio_ifc::editor::ifc2x3_cobie::Ifc2x3CobieEditor, semio_s_artifact_stdio_ifc::editor::ifc2x3_cobie::create_ifc2x3_cobie_editor),
    (ifc2x3_cv20_editor, sqlite_snapshot_ifc2x3_cv20_editor, semio_s_artifact_stdio_ifc::editor::ifc2x3_cv20::Ifc2x3Cv20Editor, semio_s_artifact_stdio_ifc::editor::ifc2x3_cv20::create_ifc2x3_cv20_editor),
    (ifc2x3_sav_editor, sqlite_snapshot_ifc2x3_sav_editor, semio_s_artifact_stdio_ifc::editor::ifc2x3_sav::Ifc2x3SavEditor, semio_s_artifact_stdio_ifc::editor::ifc2x3_sav::create_ifc2x3_sav_editor),
    (ifc4_any_editor, sqlite_snapshot_ifc4_any_editor, semio_s_artifact_stdio_ifc::editor::ifc4_any::Ifc4AnyEditor, semio_s_artifact_stdio_ifc::editor::ifc4_any::create_ifc4_any_editor),
    (dwg_ac1018_editor, sqlite_snapshot_dwg_ac1018_editor, semio_s_artifact_stdio_dwg::editor::dwg_ac1018::DwgAc1018Editor, semio_s_artifact_stdio_dwg::editor::dwg_ac1018::create_dwg_ac1018_editor),
    (dwg_ac1024_editor, sqlite_snapshot_dwg_ac1024_editor, semio_s_artifact_stdio_dwg::editor::dwg_ac1024::DwgAc1024Editor, semio_s_artifact_stdio_dwg::editor::dwg_ac1024::create_dwg_ac1024_editor),
    (dxf_any_editor, sqlite_snapshot_dxf_any_editor, semio_s_artifact_stdio_dxf::editor::dxf::DxfAnyEditor, semio_s_artifact_stdio_dxf::editor::dxf::create_dxf_any_editor),
    (gltf_any_editor, sqlite_snapshot_gltf_any_editor, semio_s_artifact_stdio_gltf::editor::gltf::GltfAnyEditor, semio_s_artifact_stdio_gltf::editor::gltf::create_gltf_any_editor),
    (obj_any_editor, sqlite_snapshot_obj_any_editor, semio_s_artifact_stdio_obj::editor::obj::ObjAnyEditor, semio_s_artifact_stdio_obj::editor::obj::create_obj_any_editor),
    (stl_any_editor, sqlite_snapshot_stl_any_editor, semio_s_artifact_stdio_stl::editor::stl::StlAnyEditor, semio_s_artifact_stdio_stl::editor::stl::create_stl_any_editor),
    (ply_any_editor, sqlite_snapshot_ply_any_editor, semio_s_artifact_stdio_ply::editor::ply::PlyAnyEditor, semio_s_artifact_stdio_ply::editor::ply::create_ply_any_editor),
    (las_any_editor, sqlite_snapshot_las_any_editor, semio_s_artifact_stdio_las::editor::las::LasAnyEditor, semio_s_artifact_stdio_las::editor::las::create_las_any_editor),
    (bcf_any_editor, sqlite_snapshot_bcf_any_editor, semio_s_artifact_stdio_bcf::editor::bcf::BcfAnyEditor, semio_s_artifact_stdio_bcf::editor::bcf::create_bcf_any_editor),
    (csv_editor, sqlite_snapshot_csv_editor, semio_s_artifact_stdio_csv::editor::csv::CsvEditor, semio_s_artifact_stdio_csv::editor::csv::create_csv_editor),
    (tsv_editor, sqlite_snapshot_tsv_editor, semio_s_artifact_stdio_tsv::editor::tsv::TsvEditor, semio_s_artifact_stdio_tsv::editor::tsv::create_tsv_editor),
    (txt_editor, sqlite_snapshot_txt_editor, semio_s_artifact_stdio_txt::editor::txt::TxtEditor, semio_s_artifact_stdio_txt::editor::txt::create_txt_editor),
    (json_any_editor, sqlite_snapshot_json_any_editor, semio_s_artifact_stdio_json::editor::json_any::JsonAnyEditor, semio_s_artifact_stdio_json::editor::json_any::create_json_editor),
    (json_i_json_editor, sqlite_snapshot_json_i_json_editor, semio_s_artifact_stdio_json::editor::json_i_json::JsonIJsonEditor, semio_s_artifact_stdio_json::editor::json_i_json::create_json_i_json_editor),
    (xml_any_editor, sqlite_snapshot_xml_any_editor, semio_s_artifact_stdio_xml::editor::xml_any::XmlAnyEditor, semio_s_artifact_stdio_xml::editor::xml_any::create_xml_editor),
    (xml_valid_editor, sqlite_snapshot_xml_valid_editor, semio_s_artifact_stdio_xml::editor::xml_valid::XmlValidEditor, semio_s_artifact_stdio_xml::editor::xml_valid::create_xml_valid_editor),
    (pdf14_a_editor, sqlite_snapshot_pdf14_a_editor, semio_s_artifact_stdio_pdf::editor::pdf14a::Pdf14AEditor, semio_s_artifact_stdio_pdf::editor::pdf14a::create_pdf14_a_editor),
    (pdf14_editor, sqlite_snapshot_pdf14_editor, semio_s_artifact_stdio_pdf::editor::pdf14::Pdf14Editor, semio_s_artifact_stdio_pdf::editor::pdf14::create_pdf14_editor),
    (pdf14_x_editor, sqlite_snapshot_pdf14_x_editor, semio_s_artifact_stdio_pdf::editor::pdf14x::Pdf14XEditor, semio_s_artifact_stdio_pdf::editor::pdf14x::create_pdf14_x_editor),
    (pdf17_a_editor, sqlite_snapshot_pdf17_a_editor, semio_s_artifact_stdio_pdf::editor::pdf17a::Pdf17AEditor, semio_s_artifact_stdio_pdf::editor::pdf17a::create_pdf17_a_editor),
    (pdf17_editor, sqlite_snapshot_pdf17_editor, semio_s_artifact_stdio_pdf::editor::pdf17::Pdf17Editor, semio_s_artifact_stdio_pdf::editor::pdf17::create_pdf17_editor),
    (pdf17_e_editor, sqlite_snapshot_pdf17_e_editor, semio_s_artifact_stdio_pdf::editor::pdf17e::Pdf17EEditor, semio_s_artifact_stdio_pdf::editor::pdf17e::create_pdf17_e_editor),
    (pdf17_h_editor, sqlite_snapshot_pdf17_h_editor, semio_s_artifact_stdio_pdf::editor::pdf17h::Pdf17HEditor, semio_s_artifact_stdio_pdf::editor::pdf17h::create_pdf17_h_editor),
    (pdf17_ua_editor, sqlite_snapshot_pdf17_ua_editor, semio_s_artifact_stdio_pdf::editor::pdf17ua::Pdf17UaEditor, semio_s_artifact_stdio_pdf::editor::pdf17ua::create_pdf17_ua_editor),
    (pdf17_vt_editor, sqlite_snapshot_pdf17_vt_editor, semio_s_artifact_stdio_pdf::editor::pdf17vt::Pdf17VtEditor, semio_s_artifact_stdio_pdf::editor::pdf17vt::create_pdf17_vt_editor),
    (pdf17_x_editor, sqlite_snapshot_pdf17_x_editor, semio_s_artifact_stdio_pdf::editor::pdf17x::Pdf17XEditor, semio_s_artifact_stdio_pdf::editor::pdf17x::create_pdf17_x_editor),
    (docx_editor, sqlite_snapshot_docx_editor, semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::base::DocxEditor, semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::base::create_docx_editor),
    (docx_strict_editor, sqlite_snapshot_docx_strict_editor, semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::strict::DocxStrictEditor, semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::strict::create_docx_strict_editor),
    (docx_transitional_editor, sqlite_snapshot_docx_transitional_editor, semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::transitional::DocxTransitionalEditor, semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::transitional::create_docx_transitional_editor),
    (pptx_editor, sqlite_snapshot_pptx_editor, semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::base::PptxEditor, semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::base::create_pptx_editor),
    (pptx_strict_editor, sqlite_snapshot_pptx_strict_editor, semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::strict::PptxStrictEditor, semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::strict::create_pptx_strict_editor),
    (pptx_transitional_editor, sqlite_snapshot_pptx_transitional_editor, semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::transitional::PptxTransitionalEditor, semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::transitional::create_pptx_transitional_editor),
    (xlsx_editor, sqlite_snapshot_xlsx_editor, semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::base::XlsxEditor, semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::base::create_xlsx_editor),
    (xlsx_strict_editor, sqlite_snapshot_xlsx_strict_editor, semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::strict::XlsxStrictEditor, semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::strict::create_xlsx_strict_editor),
    (xlsx_transitional_editor, sqlite_snapshot_xlsx_transitional_editor, semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::transitional::XlsxTransitionalEditor, semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::transitional::create_xlsx_transitional_editor),
    (epw_editor, sqlite_snapshot_epw_editor, semio_s_artifact_stdio_epw::editor::epw::EpwEditor, semio_s_artifact_stdio_epw::editor::epw::create_epw_editor),
    (zip_any_editor, sqlite_snapshot_zip_any_editor, semio_s_artifact_stdio_zip::editor::zip::base::ZipAnyEditor, semio_s_artifact_stdio_zip::editor::zip::base::create_zip_any_editor),
    (zip_iso21320_editor, sqlite_snapshot_zip_iso21320_editor, semio_s_artifact_stdio_zip::editor::zip::iso21320::ZipIso21320Editor, semio_s_artifact_stdio_zip::editor::zip::iso21320::create_zip_iso21320_editor),
    (deflate_editor, sqlite_snapshot_deflate_editor, semio_s_artifact_stdio_deflate::editor::deflate::DeflateEditor, semio_s_artifact_stdio_deflate::editor::deflate::create_deflate_editor),
    (binary_editor, sqlite_snapshot_binary_editor, semio_s_artifact_stdio_binary::editor::binary::BinaryEditor, semio_s_artifact_stdio_binary::editor::binary::create_binary_editor),
}
