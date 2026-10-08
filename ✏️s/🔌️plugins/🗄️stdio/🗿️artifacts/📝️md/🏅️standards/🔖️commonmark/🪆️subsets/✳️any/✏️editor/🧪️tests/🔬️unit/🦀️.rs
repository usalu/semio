use super::*;

#[test]
fn text_edit_requires_an_explicit_text_value_and_allows_empty_documents() {
    assert!(md_command_from_action(MD_KIT_ACTION_ID, None).is_err());
    let args = semio_framework_value::DslValue::object([("text".into(), semio_framework_value::DslValue::String(String::new()))]);
    assert_eq!(md_command_from_action(MD_KIT_ACTION_ID, Some(&args)).expect("explicit empty text"), MdEditCommand::ReplaceText { text: String::new() });
}

#[test]
fn natural_file_route_exports_commonmark_and_reopens_the_same_document() {
    let edited = MdSnapshot::from_text("# Natural Open Save\n\nEdited body.\n");
    let bytes = <MdEditor as ArtifactEditor>::encode_natural_file(&edited).expect("Markdown natural bytes");
    let oracle = semio_s_artifact_stdio_md_test_oracle::standards::v_commonmark::subsets::any::project_md;
    assert_eq!(oracle(&bytes).expect("Comrak reads exported Markdown"), oracle(b"# Natural Open Save\n\nEdited body.\n").expect("Comrak reads expected Markdown"));
    let reopened = <MdEditor as ArtifactEditor>::decode_natural_file(&bytes).expect("Markdown natural bytes reopen");
    assert_eq!(reopened, edited);
}

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_md_editor();
    assert_eq!(def.role, semio_framework::AppRole::Editor);
    assert_eq!(def.dialect, MD_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<MdEditor as ArtifactEditor>::DIALECT, MD_DIALECT);
}

//#region 🎬️ExampleSwitchLaws
/// ⚖️ LAW: the example picker's verb is declared on the app itself, which is what
/// `appSwitchesExamples` (`🛠️ShellHelpers/🟦️.tsx`) reads before it offers the picker or announces a
/// boot example.
#[semio_framework_async_macros::async_test]
async fn the_editor_declares_the_example_pickers_verb() {
    let definition = create_md_editor();
    let action = definition.actions.iter().find(|action| action.id == semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID).expect("declared example verb");
    assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
    assert!(action.semantics.effects.destructive);
    assert!(action.args.iter().any(|arg| arg.id == "exampleId"));
}

/// ⚖️ LAW: the example switch's bounded-first-step proof joins its exact registered factory, and the
/// controller that owns it closes through its whole retained close protocol. A registry-backed
/// controller PANICS at construction (`interactive-job.catalog-authority` /
/// `interactive-job.catalog-incomplete`) whenever the retained roster, the publication contract, the
/// `Migrated` classification and `OpBinary::TOOL_JOB_IDS` disagree — so constructing one is the proof.
#[semio_framework_async_macros::async_test]
async fn the_example_switch_joins_its_exact_retained_factory() {
    use semio_framework_plugin::PluginApp;
    let registry = semio_framework_plugin::AppActionRegistry::from_definition(&create_md_editor());
    let mut app = semio_framework_plugin::VcsArtifactApp::<EditorApp<MdEditor>>::with_registry(EditorApp::<MdEditor>::default(), registry, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
    while !app.close_terminal_is_empty() {
        if matches!(app.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("registered fixture close"), semio_framework_plugin::PluginCloseStep::Complete) {
            break;
        }
    }
    assert!(app.close_terminal_is_empty(), "registered fixture reaches its exact terminal-empty witness");
}

/// ⚖️ LAW: the curated example parses into a document that differs from the genesis one, and an
/// unknown or empty id falls back to the genesis document.
#[semio_framework_async_macros::async_test]
async fn the_curated_example_carries_visible_content() {
    let example = md_example_snapshot(crate::examples::demo::ID);
    assert_ne!(example, MdSnapshot::default());
    assert_eq!(md_example_snapshot(""), MdSnapshot::default());
    assert_eq!(md_example_snapshot("no-such-example"), MdSnapshot::default());
}

/// ⚖️ LAW: the shells' `{action, args}` pair resolves into the typed command, for every spelling of
/// the id the navbar, the palette and a replayed shell command use.
#[semio_framework_async_macros::async_test]
async fn the_shell_action_pair_resolves_into_the_typed_command() {
    for key in ["exampleId", "example_id", "id", "value"] {
        let args = semio_framework_value::DslValue::object([(key.to_string(), semio_framework_value::DslValue::String("demo".into()))]);
        assert_eq!(md_command_from_action(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, Some(&args)).expect("declared verb"), MdEditCommand::SetActiveExample { example_id: "demo".into() });
    }
    assert!(md_command_from_action("noSuchVerb", None).is_err());
}
//#endregion 🎬️ExampleSwitchLaws

//#region 🪟️KitVerbLaws
type KitFixtureApp = semio_framework_plugin::VcsArtifactApp<EditorApp<MdEditor>>;

/// 🧪️ One registered fixture app holding `document`, loaded exactly as a host applies the example
/// switch's `Effect::LoadDocument`.
async fn kit_fixture_holding(document: &MdSnapshot) -> KitFixtureApp {
    use semio_framework_plugin::PluginApp;
    let mut app = semio_framework_plugin::artifact_app_laws::new_registered_app::<EditorApp<MdEditor>, _>(async { semio_framework_plugin::App { definition: create_md_editor(), examples: Vec::new() } }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(document, STDIO_MD_DOCUMENT_SCHEMA) else { panic!("the example switch hands the host one whole document") };
    semio_framework_plugin::artifact_app_laws::load_document(&mut app, &store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.expect("the host loads the example document");
    app
}

/// 🕹️ Dispatches `action` with text-staged `args`, exactly as a rail sends it, and settles it through
/// the host's bounded publication loop.
async fn dispatch_settled(app: &mut KitFixtureApp, action: &str, args: &[(&str, &str)]) -> Result<(), Fault> {
    use semio_framework_plugin::PluginApp;
    let meta = semio_framework_plugin::artifact_app_laws::meta("local");
    let args = semio_framework_value::DslValue::object(args.iter().map(|(key, value)| ((*key).to_string(), semio_framework_value::DslValue::String((*value).to_string()))).collect::<Vec<_>>());
    app.handle_action(action, Some(&args), &meta).await?;
    semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(app, meta.instance_id).await.map(|_| ())
}

/// ⚖️ LAW: `replace-text` — the verb the `TextWindowKit` mints for `🪟️main` — reaches the document through this
/// editor's exact retained factory. Unregistered, the reactor refused it inside `s` with
/// `interactive-job.missing-factory` (S15, session 11).
#[semio_framework_async_macros::async_test]
async fn the_kit_verb_edits_the_document_through_its_exact_retained_factory() {
    let target = md_example_snapshot(crate::examples::demo::ID);
    let mut app = kit_fixture_holding(&MdSnapshot::default()).await;
    dispatch_settled(&mut app, "textEdit", &[("text", &<MdSnapshot as store::ArtifactDsl>::print_dsl(&target))]).await.expect("replace-text settles");
    assert_eq!(app.snapshot().expect("md snapshot"), target);
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}
//#endregion 🪟️KitVerbLaws

//#region 🧮️NetLeafLaws
const NET_LEAVES: &str = include_str!("../../../🧫️fixtures/🧫️net-leaves/🔣️.json");

/// 🧾️ A leaf as the net-leaves corpus names it: kind, container path, index.
fn net_leaf_summary(leaf: &MdMutation) -> serde_json::Value {
    let step = |step: &MdPathStep| match step {
        MdPathStep::BlockQuote { index } => format!("quote:{index}"),
        MdPathStep::ListItem { index, item } => format!("item:{index}:{item}"),
    };
    let (path, index) = match leaf {
        MdMutation::InsertBlock(leaf) => (leaf.path.iter().map(step).collect::<Vec<_>>(), leaf.index),
        MdMutation::RemoveBlock(leaf) => (leaf.path.iter().map(step).collect(), leaf.index),
        MdMutation::ReplaceBlock(leaf) => (leaf.path.iter().map(step).collect(), leaf.index),
        MdMutation::SetInlines(leaf) => (leaf.path.iter().map(step).collect(), leaf.index),
    };
    serde_json::json!({ "kind": protocol::SemanticMutation::<MdSnapshot>::semantics(leaf).kind, "path": path, "index": index })
}

/// ⚖️ LAW (corpus `🧫️fixtures/🧫️net-leaves`): an applied markdown text is exactly the corpus's net block leaves; applied in
/// order they carry the committed document to exactly the applied text, and every leaf undoes with ONE row of its own, so the
/// edit reverts leaf by leaf back to the committed document.
#[test]
fn an_applied_text_is_its_net_block_leaves_and_they_reach_exactly_that_text() {
    let corpus: serde_json::Value = serde_json::from_str(NET_LEAVES).expect("net-leaves corpus");
    for case in corpus["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("id");
        let (base, next) = (MdSnapshot::from_text(case["before"].as_str().expect("before")), MdSnapshot::from_text(case["after"].as_str().expect("after")));
        let leaves = md_net_mutations(&base, &next);
        assert_eq!(serde_json::Value::Array(leaves.iter().map(net_leaf_summary).collect()), case["leaves"], "{id}: the net leaves");
        let mut state = base.clone();
        let mut undo = Vec::new();
        for leaf in &leaves {
            let inverse = protocol::Mutation::inverse(leaf, &state).expect("valid retained mutation inverse fixture");
            assert_eq!(inverse.len(), 1, "{id}: {leaf:?} undoes with exactly one row");
            undo.extend(inverse);
            assert!(crate::standards::v_commonmark::subsets::any::schema::mutations::apply_md_mutation(&mut state, leaf).messages().iter().all(|message| message.level != semio_framework_diagnostic::Severity::Fatal), "{id}: {leaf:?} applies");
        }
        assert_eq!(state, next, "{id}: the net leaves reach exactly the applied text");
        for leaf in undo.iter().rev() {
            crate::standards::v_commonmark::subsets::any::schema::mutations::apply_md_mutation(&mut state, leaf);
        }
        assert_eq!(state, base, "{id}: undoing every leaf restores the committed document");
    }
}

/// ⚖️ LAW: an applied envelope naming another document schema is refused by the exact replay instead of being published as a
/// whole-document replacement, and no net-leaves corpus change (every one keeps its schema) is refused.
#[test]
fn another_document_schema_is_refused_and_no_corpus_change_is() {
    let corpus: serde_json::Value = serde_json::from_str(NET_LEAVES).expect("net-leaves corpus");
    for case in corpus["cases"].as_array().expect("cases") {
        let (base, next) = (MdSnapshot::from_text(case["before"].as_str().expect("before")), MdSnapshot::from_text(case["after"].as_str().expect("after")));
        assert!(semio_s_artifact_stdio_contract::editing::net_leaves_exact(&base, &next, md_net_mutations).is_ok(), "{}: a block edit is addressed by its leaves", case["id"]);
    }
    let base = MdSnapshot::from_text("# Title\n\nBody.\n");
    let other = MdSnapshot { schema: "stdio.md.other-schema".into(), ..base.clone() };
    assert!(semio_s_artifact_stdio_contract::editing::net_leaves_exact(&base, &other, md_net_mutations).is_err(), "another document schema is unaddressed");
}

/// ⚖️ LAW (design §20.3): a document-details edit publishes the artifact's own net block leaves — exactly the corpus leaves of
/// the same change, with each history row labelled by its own leaf in every supported locale.
#[test]
fn a_document_details_edit_is_its_net_block_leaves() {
    use semio_s_artifact_stdio_contract::editing::{snapshot_edit_source, SnapshotEditEvent, SnapshotEditingEditor};
    ::semio_framework_schema_registry::register_artifact_schema_descriptors(vec![crate::standards::v_commonmark::subsets::any::schema::md_artifact_schema_descriptor()]).expect("register md schema");
    let corpus: serde_json::Value = serde_json::from_str(NET_LEAVES).expect("net-leaves corpus");
    for case in corpus["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("id");
        let (base, next) = (MdSnapshot::from_text(case["before"].as_str().expect("before")), MdSnapshot::from_text(case["after"].as_str().expect("after")));
        let event = SnapshotEditEvent::ReplaceSource { source: snapshot_edit_source(&next) };
        let emit = <MdEditor as SnapshotEditingEditor>::snapshot_edit_emit(&event, &base).unwrap_or_else(|fault| panic!("{id}: the details edit publishes: {fault:?}"));
        assert_eq!(serde_json::Value::Array(emit.artifact_mutations.iter().map(net_leaf_summary).collect()), case["leaves"], "{id}: the details edit is the net leaves");
        for leaf in &emit.artifact_mutations {
            let (en, de) = match leaf {
                MdMutation::InsertBlock(_) => ("Insert block", "Block einfügen"),
                MdMutation::RemoveBlock(_) => ("Remove block", "Block entfernen"),
                MdMutation::ReplaceBlock(_) => ("Replace block", "Block ersetzen"),
                MdMutation::SetInlines(_) => ("Set inlines", "Inline-Elemente setzen"),
            };
            assert_eq!(protocol::SemanticMutation::<MdSnapshot>::label(leaf), semio_framework_ui_locale::LocalizedLabel::native(en, de), "{id}: the row is labelled from its leaf in every supported locale");
        }
    }
}

/// ⚖️ LAW: the kit verb applies CommonMark source — the text the main window shows and edits — as ONE edit of its net leaves:
/// one changed paragraph is one `set-inlines` row labelled from the leaf, the document reads exactly the applied text, and ONE
/// undo restores the committed document.
#[semio_framework_async_macros::async_test]
async fn one_applied_markdown_text_is_one_edit_of_its_net_leaves() {
    use semio_framework_plugin::PluginApp;
    let before = "# Title\n\nFirst.\n\nSecond.\n";
    let after = "# Title\n\nFirst, edited.\n\nSecond.\n";
    let mut app = kit_fixture_holding(&MdSnapshot::from_text(before)).await;
    let edits = app.edit_transactions().len();
    dispatch_settled(&mut app, "textEdit", &[("text", after)]).await.expect("the applied markdown settles");
    assert_eq!(app.snapshot().expect("md snapshot"), MdSnapshot::from_text(after));
    assert_eq!(app.edit_transactions().len(), edits + 1, "one Apply, one edit");
    let rows: Vec<_> = app.history_snapshot().await.expect("history").upserts.into_iter().filter(|row| row.edit_id.is_some()).collect();
    let row = rows.iter().max_by_key(|row| row.seq).expect("the applied text's row");
    assert_eq!(row.mutations.len(), 1, "one changed paragraph is one net leaf: {:?}", row.mutations);
    assert_eq!(row.mutations[0].label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), "Set inlines");
    assert_eq!(row.mutations[0].label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), "Inline-Elemente setzen");
    semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut app, "undo", semio_framework_plugin::artifact_app_laws::meta("local").instance_id).await;
    assert_eq!(app.snapshot().expect("md snapshot"), MdSnapshot::from_text(before), "one undo restores the committed document");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}
//#endregion 🧮️NetLeafLaws

semio_framework_plugin::history_edit_acceptance_law!("stdio", MdEditor, || semio_framework_plugin::App { definition: create_md_editor(), examples: Vec::new() }, "../..");
