use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::Editor;
use semio_s_artifact_stdio_contract::editing;
#[allow(unused_imports)]
use crate::editor::modes::edit;
#[allow(unused_imports)]
use crate::editor::modes::edit::windows::main;
#[allow(unused_imports)]
use crate::schema::mutations::SemioMutation;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSnapshot;
#[allow(unused_imports)]
use semio_framework_plugin::app::InteractionView;
#[allow(unused_imports)]
use semio_framework_plugin::{retained_command::ArtifactRetainedCommandInputs, retained_command::ArtifactRetainedCommandJob, retained_command::ArtifactRetainedCommandPayload, retained_command::BoundedArtifactCommandWork};
#[allow(unused_imports)]
use semio_framework_plugin::AppOperationContext;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactToolFactoryRegistry;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactToolPublicationContract;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactToolPublicationLane;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactView;
#[allow(unused_imports)]
use semio_framework_plugin::ConfigView;
#[allow(unused_imports)]
use semio_framework_artifact_reference::Dialect;
#[allow(unused_imports)]
use semio_framework_plugin::DraftView;
#[allow(unused_imports)]
use semio_framework_plugin::Emit;
#[allow(unused_imports)]
use semio_framework_plugin::Fault;
#[allow(unused_imports)]
use semio_framework_ui_locale::Label;
#[allow(unused_imports)]
use semio_framework_plugin::NoConfig;
#[allow(unused_imports)]
use semio_framework_plugin::NoConfigMutation;
#[allow(unused_imports)]
use semio_framework_plugin::NoDraft;
#[allow(unused_imports)]
use semio_framework_plugin::NoDraftMutation;
#[allow(unused_imports)]
use semio_framework_plugin::NoPresence;
#[allow(unused_imports)]
use semio_framework_plugin::NoPresenceMutation;
#[allow(unused_imports)]
use semio_framework_plugin::NoTransient;
#[allow(unused_imports)]
use semio_framework_plugin::NoTransientMutation;
#[allow(unused_imports)]
use semio_framework_artifact_reference::StandardId;
#[allow(unused_imports)]
use semio_framework_artifact_reference::SubsetId;
#[allow(unused_imports)]
use semio_framework_plugin::ToolExecutionContract;
#[allow(unused_imports)]
use semio_framework_plugin::ToolFactoryKey;
#[allow(unused_imports)]
use semio_framework_plugin::ToolJobFactory;
#[allow(unused_imports)]
use semio_framework_plugin::ToolJobFactoryError;
#[allow(unused_imports)]
use semio_framework_plugin::ToolOperationSpec;
#[allow(unused_imports)]
use semio_framework_plugin::EditorApp;
#[allow(unused_imports)]
use semio_framework_plugin::InteractiveJobClassification;
#[allow(unused_imports)]
use semio_framework_2d_compute::EngineHandles;
use crate::editor::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_semio_base_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, SEMIO_ANY_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<SemioAnyEditor as ArtifactEditor>::DIALECT, SEMIO_ANY_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<SemioAnyEditor, crate::viewer::SemioAnyViewer>().await;
}

semio_framework_plugin::history_edit_acceptance_law!("stdio/SemioAnyEditor", SemioAnyEditor, || semio_framework_plugin::App { definition: create_semio_base_editor(), examples: Vec::new() }, "../..");

mod edit_rules_laws {
    
    use semio_s_artifact_stdio_semio_base::editor::edit_plumbing::complete_in;
    use semio_s_artifact_stdio_semio_base::editor::edit_plumbing::edited;
    use semio_s_artifact_stdio_semio_base::editor::edit_plumbing::list_move;
    use semio_s_artifact_stdio_semio_base::editor::edit_plumbing::spread_item;
    use semio_s_artifact_stdio_semio_base::editor::edit_plumbing::Entries;
    use semio_framework_value::DslValue;
    use semio_s_artifact_stdio_contract::editing::{EditRules, SnapshotEditEvent};

    fn wildcards(path: &str) -> usize {
        path.split('/').filter(|segment| *segment == "*").count()
    }

    fn text(value: &str) -> DslValue {
        DslValue::String(value.to_string())
    }

    fn record(entries: Vec<(&str, DslValue)>) -> DslValue {
        DslValue::object(entries.into_iter().map(|(name, value)| (name.to_string(), value)))
    }

    macro_rules! tables {
        ($($subset:ident => $krate:ident),* $(,)?) => {
            vec![$((stringify!($subset), &$krate::editor::edit_rules::EDIT_RULES as &EditRules, $krate::schema::mutations::KINDS)),*]
        };
    }

    #[semio_framework_async_macros::async_test]
    async fn every_rule_names_a_kind_of_its_subset_and_one_selector_per_wildcard() {
        let all: Vec<(&str, &EditRules, &[&str])> = tables!(
            brep => semio_s_artifact_stdio_semio_brep, mesh => semio_s_artifact_stdio_semio_mesh, model => semio_s_artifact_stdio_semio_model, value => semio_s_artifact_stdio_semio_value, document => semio_s_artifact_stdio_semio_document, cad => semio_s_artifact_stdio_semio_cad,
            drawing => semio_s_artifact_stdio_semio_drawing, image => semio_s_artifact_stdio_semio_image, video => semio_s_artifact_stdio_semio_video, audio => semio_s_artifact_stdio_semio_audio, animation => semio_s_artifact_stdio_semio_animation,
            presentation => semio_s_artifact_stdio_semio_presentation, flow => semio_s_artifact_stdio_semio_flow, text => semio_s_artifact_stdio_semio_text, table => semio_s_artifact_stdio_semio_table, graph => semio_s_artifact_stdio_semio_graph,
            object => semio_s_artifact_stdio_semio_object, kit => semio_s_artifact_stdio_semio_kit,
        );
        for (subset, rules, kinds) in all {
            for rule in rules.entities {
                assert!(kinds.contains(&rule.kind) && rule.path.starts_with('/') && wildcards(rule.path) == rule.selectors.len(), "{subset}: entity rule {} -> {}", rule.path, rule.kind);
            }
            for rule in rules.inserts {
                assert!(kinds.contains(&rule.kind) && rule.path.starts_with('/') && wildcards(rule.path) == rule.selectors.len(), "{subset}: insert rule {} -> {}", rule.path, rule.kind);
            }
            for rule in rules.removes {
                assert!(kinds.contains(&rule.kind) && rule.path.starts_with('/') && wildcards(rule.path) == rule.selectors.len(), "{subset}: remove rule {} -> {}", rule.path, rule.kind);
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn a_run_content_edit_raises_one_edit_run_with_the_run_position() {
        let tree = record(vec![("runs", DslValue::Array(vec![record(vec![("content", text("a")), ("language", text("en")), ("marks", DslValue::Array(Vec::new()))])]))]);
        let event = SnapshotEditEvent::SetValue { path: "/runs/0/content".to_string(), value: text("b") };
        let plan = semio_s_artifact_stdio_semio_text::editor::edit_rules::EDIT_RULES.plan(&tree, &event).expect("plan").expect("a changed run");
        assert_eq!(plan.kind, "edit-run");
        assert_eq!(plan.entries, vec![("index".to_string(), DslValue::uint(0)), ("new_text".to_string(), text("b"))]);
    }

    #[semio_framework_async_macros::async_test]
    async fn an_edit_no_rule_names_is_refused() {
        let tree = record(vec![("runs", DslValue::Array(Vec::new()))]);
        let event = SnapshotEditEvent::SetValue { path: "/schema".to_string(), value: text("x") };
        assert!(semio_s_artifact_stdio_semio_text::editor::edit_rules::EDIT_RULES.plan(&tree, &event).is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn an_inserted_row_spreads_into_the_payload_and_a_setter_completes_what_it_keeps() {
        let mut entries: Entries = vec![("$item".to_string(), record(vec![("id", text("n1")), ("kind", text("k"))])), ("at".to_string(), DslValue::uint(2))];
        spread_item(&DslValue::Null, &mut entries).expect("spread");
        assert_eq!(entries, vec![("at".to_string(), DslValue::uint(2)), ("id".to_string(), text("n1")), ("kind".to_string(), text("k"))]);
        let tree = record(vec![("nodes", DslValue::Array(vec![record(vec![("width", DslValue::uint(3)), ("height", DslValue::uint(4))])]))]);
        let mut sized: Entries = vec![("index".to_string(), DslValue::uint(0)), ("width".to_string(), DslValue::uint(9))];
        complete_in(&tree, &mut sized, &[("nodes", "index")], &["width", "height"]).expect("complete");
        assert_eq!(sized.last(), Some(&("height".to_string(), DslValue::uint(4))));
    }

    #[semio_framework_async_macros::async_test]
    async fn an_edit_below_an_entity_is_applied_to_a_clone_of_that_entity_alone() {
        let entity = record(vec![("x", DslValue::Array(vec![DslValue::uint(1), DslValue::uint(2)]))]);
        let event = SnapshotEditEvent::InsertValue { path: "/p/x/1".to_string(), value: DslValue::uint(9) };
        let next = edited(&entity, &event, 1).expect("edited");
        assert_eq!(next, record(vec![("x", DslValue::Array(vec![DslValue::uint(1), DslValue::uint(9), DslValue::uint(2)]))]));
        assert_eq!(entity, record(vec![("x", DslValue::Array(vec![DslValue::uint(1), DslValue::uint(2)]))]));
    }

    #[semio_framework_async_macros::async_test]
    async fn only_a_move_inside_the_named_list_is_a_list_move() {
        let moved = SnapshotEditEvent::MoveValue { from: "/runs/0".to_string(), path: "/runs/2".to_string() };
        let elsewhere = SnapshotEditEvent::MoveValue { from: "/runs/0".to_string(), path: "/other/1".to_string() };
        assert_eq!(list_move(&moved, &["runs"]).expect("move"), Some((0, 2)));
        assert_eq!(list_move(&elsewhere, &["runs"]).expect("move"), None);
    }
}
