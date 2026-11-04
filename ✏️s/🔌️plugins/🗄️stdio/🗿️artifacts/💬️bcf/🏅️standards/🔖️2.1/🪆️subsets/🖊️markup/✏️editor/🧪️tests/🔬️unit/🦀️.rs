use crate::{standards::v2_1::subsets::any::schema::mutations::set_topic_markup::SetTopicMarkup, standards::v2_1::subsets::any::schema::mutations::BcfMutation};
use crate::{standards::v2_1::subsets::any::schema::snapshot::BcfSnapshot, standards::v2_1::subsets::any::schema::snapshot::BcfTopic};
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_s_artifact_stdio_contract::editing;
use semio_s_artifact_stdio_contract::editing::SnapshotEditEvent;
use semio_s_artifact_stdio_contract::pack;
use crate::editor::bcf::{bcf_example_snapshot, bcf_command_from_action, bcf_emit};
use crate::apply_mutation;
use crate::editor::bcf::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_bcf_any_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, BCF_ANY_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<BcfAnyEditor as ArtifactEditor>::DIALECT, BCF_ANY_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<BcfAnyEditor, crate::viewer::bcf::BcfAnyViewer>().await;
}

#[test]
fn set_cell_requires_its_full_address_and_accepts_an_empty_value() {
    let args = semio_framework_value::DslValue::object([
        ("row".into(), semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(1))),
        ("column".into(), semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(2))),
        ("revision".into(), semio_framework_value::DslValue::String("rev".into())),
        ("value".into(), semio_framework_value::DslValue::String(String::new())),
    ]);
    assert_eq!(bcf_command_from_action("set-cell", Some(&args)).expect("complete cell address"), BcfAnyEditCommand::SetCell { row: 1, column: 2, revision: "rev".into(), value: String::new() });
    assert!(bcf_command_from_action("set-cell", None).is_err());
    let missing_value = semio_framework_value::DslValue::object([("row".into(), semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(1))), ("column".into(), semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(2)))]);
    assert!(bcf_command_from_action("set-cell", Some(&missing_value)).is_err());
}

#[semio_framework_async_macros::async_test]
async fn set_cell_reaches_the_document_through_its_exact_retained_factory() {
    use semio_framework_plugin::PluginApp;
    let source = bcf_example_snapshot(crate::examples::demo::ID);
    assert!(!source.topics.is_empty(), "the BCF example must contain an editable topic");
    let mut scope = semio_framework_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = semio_framework_plugin::with_authoring_identity!(|identity| semio_framework_plugin::artifact_app_laws::new_registered_app::<EditorApp<BcfAnyEditor>, _>(async { semio_framework_plugin::App { definition: create_bcf_any_editor(), examples: Vec::new() } }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity).await);
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&source, BCF_ANY_DOCUMENT_SCHEMA) else { panic!("the fixture load is a complete document effect") };
    semio_framework_plugin::with_authoring_identity!(|identity| semio_framework_plugin::artifact_app_laws::load_document(&mut app, &store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() }, &mut identity, scope.cx()).await).expect("load BCF fixture");
    let revision = semio_s_artifact_stdio_contract::window_kit_canonical_revision(app.test_document_revision());
    let args = semio_framework_value::DslValue::object([
        ("row".into(), semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(0))),
        ("column".into(), semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(1))),
        ("revision".into(), semio_framework_value::DslValue::String(revision)),
        ("value".into(), semio_framework_value::DslValue::String("Retained title".into())),
    ]);
    let meta = semio_framework_plugin::artifact_app_laws::meta("local");
    semio_framework_plugin::with_authoring_identity!(|identity| semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app, "set-cell", Some(args.clone()), &meta, &mut identity, scope.cx(), semio_s_artifact_stdio_contract::testing::mounted_policy().close).await).expect("dispatch set-cell");
    semio_framework_plugin::with_authoring_identity!(|identity| semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await).expect("settle set-cell");
    assert_eq!(app.snapshot().expect("BCF snapshot").topics[0].title, "Retained title");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app, semio_s_artifact_stdio_contract::testing::mounted_policy(), scope.cx()).expect("original caller paid app close");
}

#[test]
fn set_cell_uses_the_compact_topic_markup_leaf_for_modeled_fields() {
    let source = bcf_example_snapshot(crate::examples::demo::ID);
    let revision = semio_s_artifact_stdio_contract::window_kit_snapshot_revision(&source);
    let emitted = bcf_emit(&BcfAnyEditCommand::SetCell { row: 0, column: 1, revision, value: "Compact title".into() }, &source).expect("compact BCF title edit");
    assert!(matches!(
        emitted.artifact_mutations.as_slice(),
        [BcfMutation::SetTopicMarkup(crate::standards::v2_1::subsets::any::schema::mutations::set_topic_markup::SetTopicMarkup { guid, title: Some(title), description: None, status: None, priority: None, labels: None, creation_date: None, creation_author: None })]
        if guid == &source.topics[0].guid && title == "Compact title"
    ));
}

#[test]
fn set_cell_rejects_stale_revisions_and_addresses_without_mutation() {
    let source = bcf_example_snapshot(crate::examples::demo::ID);
    assert!(bcf_emit(&BcfAnyEditCommand::SetCell { row: 0, column: 0, revision: "stale".into(), value: "x".into() }, &source).is_err());
    let revision = semio_s_artifact_stdio_contract::window_kit_snapshot_revision(&source);
    assert!(bcf_emit(&BcfAnyEditCommand::SetCell { row: u32::MAX, column: 0, revision: revision.clone(), value: "x".into() }, &source).is_err());
    assert!(bcf_emit(&BcfAnyEditCommand::SetCell { row: 0, column: u32::MAX, revision, value: "x".into() }, &source).is_err());
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", BcfAnyEditor, || semio_framework_plugin::App { definition: create_bcf_any_editor(), examples: Vec::new() }, "../..");

#[semio_framework_async_macros::async_test]
async fn details_edits_resolve_to_the_kind_of_the_addressed_field() {
    
    use crate::standards::v2_1::subsets::any::schema::snapshot::{BcfComment, BcfTopic};
    let topic = BcfTopic { guid: "t1".into(), title: "old".into(), status: "Open".into(), comments: vec![BcfComment { guid: "c1".into(), date: "d".into(), author: "a".into(), text: "hello".into(), viewpoint_ref: None }], ..BcfTopic::default() };
    let base = BcfSnapshot { topics: vec![topic], ..BcfSnapshot::default() };
    let emit = |event: editing::SnapshotEditEvent| <BcfAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &base);
    let retitled = emit(editing::SnapshotEditEvent::SetValue { path: "/topics/0/title".into(), value: semio_framework_value::DslValue::String("new".into()) }).expect("a topic field edit resolves");
    let [mutation @ BcfMutation::SetTopicMarkup(_)] = retitled.artifact_mutations.as_slice() else { panic!("a topic field raises the topic-markup kind") };
    let mut state = base.clone();
    apply_mutation(&mut state, mutation);
    assert_eq!(state.topics[0].title, "new");
    let worded = emit(editing::SnapshotEditEvent::SetValue { path: "/topics/0/comments/0/text".into(), value: semio_framework_value::DslValue::String("bye".into()) }).expect("a comment field edit resolves");
    let [mutation @ BcfMutation::SetComment(_)] = worded.artifact_mutations.as_slice() else { panic!("a comment field raises the comment kind") };
    apply_mutation(&mut state, mutation);
    assert_eq!(state.topics[0].comments[0].text, "bye");
    let dropped = emit(editing::SnapshotEditEvent::RemoveValue { path: "/topics/0/comments/0".into() }).expect("a comment removal resolves");
    assert!(matches!(dropped.artifact_mutations.as_slice(), [BcfMutation::RemoveComment(_)]));
    assert_eq!(emit(editing::SnapshotEditEvent::SetValue { path: "/schema".into(), value: semio_framework_value::DslValue::String("other".into()) }).err().expect("no kind").code.0, "snapshot-edit.unsupported-path");
}

const NATURAL_MINIMAL: &[u8] = include_bytes!("../../../🧪️tests/🧫️fixtures/📂️natural-open/minimal.bcf");
const NATURAL_TRUNCATED: &[u8] = include_bytes!("../../../🧪️tests/🧫️fixtures/📂️natural-open/truncated.bcf");
const NATURAL_MISSING_VERSION: &[u8] = include_bytes!("../../../🧪️tests/🧫️fixtures/📂️natural-open/missing-version.bcf");

#[test]
fn natural_file_open_edit_save_reopen_keeps_comments_viewpoints_and_png() {
    let opened = <BcfAnyEditor as ArtifactEditor>::decode_natural_file(NATURAL_MINIMAL).expect("a real bcfzip opens");
    assert_eq!(opened.version, "2.1");
    assert_eq!(opened.topics.len(), 1);
    let topic = &opened.topics[0];
    assert_eq!(topic.guid, "topic-minimal-01");
    assert_eq!(topic.comments.len(), 1);
    assert_eq!(topic.viewpoints.len(), 1);
    let png = topic.viewpoints[0].snapshot.as_ref().expect("the viewpoint snapshot is retained");
    assert_eq!(&png[..8], b"\x89PNG\r\n\x1a\n");
    let command = BcfAnyEditCommand::SetCell { row: 0, column: 1, revision: semio_s_artifact_stdio_contract::window_kit_snapshot_revision(&opened), value: "Door swing blocks the corridor".into() };
    let emit = bcf_emit(&command, &opened).expect("title edit");
    let mut edited = opened.clone();
    for mutation in &emit.artifact_mutations {
        apply_mutation(&mut edited, mutation);
    }
    assert_eq!(edited.topics[0].title, "Door swing blocks the corridor");
    let saved = <BcfAnyEditor as ArtifactEditor>::encode_natural_file(&edited).expect("save");
    let reopened = <BcfAnyEditor as ArtifactEditor>::decode_natural_file(&saved).expect("reopen");
    assert_eq!(reopened, edited);
    assert_eq!(reopened.topics[0].comments, opened.topics[0].comments);
    assert_eq!(reopened.topics[0].viewpoints, opened.topics[0].viewpoints);
}

#[test]
fn natural_file_refuses_a_truncated_archive_and_a_missing_version_part() {
    let truncated = <BcfAnyEditor as ArtifactEditor>::decode_natural_file(NATURAL_TRUNCATED).expect_err("a truncated zip is refused");
    assert!(!truncated.to_string().is_empty());
    let missing = <BcfAnyEditor as ArtifactEditor>::decode_natural_file(NATURAL_MISSING_VERSION).expect_err("an archive without bcf.version is refused");
    assert!(missing.to_string().contains("bcf.version"), "{missing}");
}
