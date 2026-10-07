use super::*;

#[test]
fn text_edit_requires_an_explicit_text_value_and_allows_empty_documents() {
    assert!(html_command_from_action(HTML_KIT_ACTION_ID, None).is_err());
    let args = semio_framework_value::DslValue::object([("text".into(), semio_framework_value::DslValue::String(String::new()))]);
    assert_eq!(html_command_from_action(HTML_KIT_ACTION_ID, Some(&args)).expect("explicit empty text"), HtmlEditCommand::ReplaceText { text: String::new() });
}

#[test]
fn natural_file_route_exports_html5_and_reopens_through_one_mutation() {
    let source = b"<!doctype html><html><head><title>Natural Open Save</title></head><body><p>Edited body.</p></body></html>";
    let edited = crate::standards::v5::subsets::any::io::text::snapshot::parse_html_document(std::str::from_utf8(source).unwrap()).expect("HTML fixture");
    let bytes = <HtmlEditor as ArtifactEditor>::encode_natural_file(&edited).expect("HTML natural bytes");
    let oracle = semio_s_artifact_stdio_html_test_oracle::standards::v5::subsets::any::project_html_5;
    assert_eq!(oracle(&bytes).expect("html5ever reads exported HTML"), oracle(source).expect("html5ever reads expected HTML"));
    let reopened = <HtmlEditor as ArtifactEditor>::decode_natural_file(&bytes).expect("HTML natural bytes reopen");
    let Some(HtmlMutation::SetSnapshot(SetSnapshot { snapshot: opened })) = <HtmlEditor as ArtifactEditor>::whole_document_operation(reopened) else {
        panic!("natural HTML opens through one event-sourced snapshot mutation")
    };
    assert_eq!(opened, edited);
}

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_html_editor();
    assert_eq!(def.role, semio_framework::AppRole::Editor);
    assert_eq!(def.dialect, HTML_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<HtmlEditor as ArtifactEditor>::DIALECT, HTML_DIALECT);
}

/// 🎯️ LAW: the editor declares the artifact kind it edits (the artifact's own `artifact_kind()`), which is
/// what the hub's one open-target rule (`app_opens_kind`, `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs`)
/// pairs with this editor and the viewer of its dialect — so an HTML document can be created and opened as a hub document.
#[semio_framework_async_macros::async_test]
async fn the_editor_declares_the_artifact_kind_it_edits() {
    assert_eq!(create_html_editor().artifact_kinds, vec![crate::artifact_kind()]);
}

//#region 🎬️ExampleSwitchLaws
/// ⚖️ LAW: the example picker's verb is declared on the app itself, which is what
/// `appSwitchesExamples` (`🛠️ShellHelpers/🟦️.tsx`) reads before it offers the picker or announces a
/// boot example.
#[semio_framework_async_macros::async_test]
async fn the_editor_declares_the_example_pickers_verb() {
    let definition = create_html_editor();
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
    let registry = semio_framework_plugin::AppActionRegistry::from_definition(&create_html_editor());
    let mut app = semio_framework_plugin::VcsArtifactApp::<EditorApp<HtmlEditor>>::with_registry(EditorApp::<HtmlEditor>::default(), registry, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
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
    let example = html_example_snapshot(crate::examples::demo::ID);
    assert_ne!(example, HtmlSnapshot::default());
    assert_eq!(html_example_snapshot(""), HtmlSnapshot::default());
    assert_eq!(html_example_snapshot("no-such-example"), HtmlSnapshot::default());
}

/// ⚖️ LAW: the shells' `{action, args}` pair resolves into the typed command, for every spelling of
/// the id the navbar, the palette and a replayed shell command use.
#[semio_framework_async_macros::async_test]
async fn the_shell_action_pair_resolves_into_the_typed_command() {
    for key in ["exampleId", "example_id", "id", "value"] {
        let args = semio_framework_value::DslValue::object([(key.to_string(), semio_framework_value::DslValue::String("demo".into()))]);
        assert_eq!(html_command_from_action(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, Some(&args)).expect("declared verb"), HtmlEditCommand::SetActiveExample { example_id: "demo".into() });
    }
    assert!(html_command_from_action("noSuchVerb", None).is_err());
}
//#endregion 🎬️ExampleSwitchLaws

//#region 🪟️KitVerbLaws
type KitFixtureApp = semio_framework_plugin::VcsArtifactApp<EditorApp<HtmlEditor>>;

/// 🧪️ One registered fixture app holding `document`, loaded exactly as a host applies the example
/// switch's `Effect::LoadDocument`.
async fn kit_fixture_holding(document: &HtmlSnapshot) -> KitFixtureApp {
    use semio_framework_plugin::PluginApp;
    let mut app = semio_framework_plugin::artifact_app_laws::new_registered_app::<EditorApp<HtmlEditor>, _>(async { semio_framework_plugin::App { definition: create_html_editor(), examples: Vec::new() } }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(document, STDIO_HTML_DOCUMENT_SCHEMA) else { panic!("the example switch hands the host one whole document") };
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
    let target = html_example_snapshot(crate::examples::demo::ID);
    let mut app = kit_fixture_holding(&HtmlSnapshot::default()).await;
    dispatch_settled(&mut app, "textEdit", &[("text", &<HtmlSnapshot as store::ArtifactDsl>::print_dsl(&target))]).await.expect("replace-text settles");
    assert_eq!(app.snapshot().expect("html snapshot"), target);
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}

/// ⚖️ LAW: `replace-text` with text that is not this artifact's DSL is refused loudly with
/// `stdio.html.invalid-text` and leaves the document untouched — it used to answer an empty emit, which
/// read as an accepted edit that moved nothing.
#[semio_framework_async_macros::async_test]
async fn replace_text_refuses_text_that_is_not_the_artifacts_dsl() {
    let example = html_example_snapshot(crate::examples::demo::ID);
    let mut app = kit_fixture_holding(&example).await;
    let fault = dispatch_settled(&mut app, "textEdit", &[("text", "not a dsl document")]).await.expect_err("invalid text is refused");
    assert!(format!("{fault:?}").contains("stdio.html.invalid-text"), "{fault:?}");
    assert_eq!(app.snapshot().expect("html snapshot"), example);
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}
//#endregion 🪟️KitVerbLaws

//#region 🧮️NetLeafLaws
const NET_LEAVES: &str = include_str!("../../../🧫️fixtures/🧫️net-leaves/🔣️.json");

/// 🧾️ A leaf as the net-leaves corpus names it: kind and the node path it addresses (the parent path plus the child index for
/// an insert or a remove).
fn net_leaf_summary(leaf: &HtmlMutation) -> serde_json::Value {
    let child = |parent: &[usize], index: usize| parent.iter().copied().chain(std::iter::once(index)).collect::<Vec<_>>();
    let at = match leaf {
        HtmlMutation::InsertNode(leaf) => child(&leaf.parent, leaf.index),
        HtmlMutation::RemoveNode(leaf) => child(&leaf.parent, leaf.index),
        HtmlMutation::SetElementName(leaf) => leaf.path.clone(),
        HtmlMutation::SetAttribute(leaf) => leaf.path.clone(),
        HtmlMutation::SetText(leaf) => leaf.path.clone(),
        HtmlMutation::SetComment(leaf) => leaf.path.clone(),
        HtmlMutation::SetRawText(leaf) => leaf.path.clone(),
        HtmlMutation::SetDoctype(_) | HtmlMutation::SetSnapshot(_) => Vec::new(),
    };
    serde_json::json!({ "kind": protocol::SemanticMutation::<HtmlSnapshot>::semantics(leaf).kind, "at": at })
}

/// ⚖️ LAW (corpus `🧫️fixtures/🧫️net-leaves`): an applied HTML text is exactly the corpus's net node leaves; applied in order
/// they carry the committed document to exactly the applied text, and every leaf undoes with ONE row of its own, so the edit
/// reverts leaf by leaf back to the committed document.
#[test]
fn an_applied_text_is_its_net_node_leaves_and_they_reach_exactly_that_text() {
    let corpus: serde_json::Value = serde_json::from_str(NET_LEAVES).expect("net-leaves corpus");
    for case in corpus["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("id");
        let parse = |field: &str| <HtmlSnapshot as store::ArtifactDsl>::parse_dsl(case[field].as_str().expect("text")).unwrap_or_else(|error| panic!("{id}: {field} parses: {error:?}"));
        let (base, next) = (parse("before"), parse("after"));
        let leaves = html_net_mutations(&base, &next);
        assert_eq!(serde_json::Value::Array(leaves.iter().map(net_leaf_summary).collect()), case["leaves"], "{id}: the net leaves");
        let mut state = base.clone();
        let mut undo = Vec::new();
        for leaf in &leaves {
            let inverse = protocol::Mutation::inverse(leaf, &state).expect("valid retained mutation inverse fixture");
            assert_eq!(inverse.len(), 1, "{id}: {leaf:?} undoes with exactly one row");
            undo.extend(inverse);
            assert!(crate::standards::v5::subsets::any::schema::mutations::apply_html_mutation(&mut state, leaf).messages().iter().all(|message| message.level != semio_framework_diagnostic::Severity::Fatal), "{id}: {leaf:?} applies");
        }
        assert_eq!(state, next, "{id}: the net leaves reach exactly the applied text");
        for leaf in undo.iter().rev() {
            crate::standards::v5::subsets::any::schema::mutations::apply_html_mutation(&mut state, leaf);
        }
        assert_eq!(state, base, "{id}: undoing every leaf restores the committed document");
    }
}

/// ⚖️ LAW (audit T4): the ONLY whole-document `set-snapshot`s the net leaves emit are the named replace intents — another document
/// schema, or a root the node leaves cannot reach (another node kind) — and no net-leaves corpus change emits one.
#[test]
fn only_the_named_replace_intents_are_a_whole_document_set_snapshot() {
    let corpus: serde_json::Value = serde_json::from_str(NET_LEAVES).expect("net-leaves corpus");
    for case in corpus["cases"].as_array().expect("cases") {
        let parse = |field: &str| <HtmlSnapshot as store::ArtifactDsl>::parse_dsl(case[field].as_str().expect("text")).unwrap_or_else(|error| panic!("{}: {field} parses: {error:?}", case["id"]));
        assert!(html_net_mutations(&parse("before"), &parse("after")).iter().all(|leaf| !matches!(leaf, HtmlMutation::SetSnapshot(_))), "{}: a node edit is never a whole-document set-snapshot", case["id"]);
    }
    let base = HtmlSnapshot::default();
    let other = HtmlSnapshot { schema: "stdio.html.other-schema".into(), ..base.clone() };
    assert_eq!(html_net_mutations(&base, &other), vec![HtmlMutation::SetSnapshot(SetSnapshot { snapshot: other.clone() })], "another document schema replaces the document");
    let text_root = HtmlSnapshot { root: HtmlNode::Text { text: "plain".into() }, ..base.clone() };
    assert_eq!(html_net_mutations(&base, &text_root).last(), Some(&HtmlMutation::SetSnapshot(SetSnapshot { snapshot: text_root.clone() })), "a root of another kind replaces the document");
}

/// ⚖️ LAW (design §20.3): a document-details edit publishes the artifact's own net node leaves — exactly the corpus leaves of the
/// same change, never a whole `set-snapshot` for a node edit — with no description, so its row is labelled from its leaves.
#[test]
fn a_document_details_edit_is_its_net_node_leaves() {
    use semio_s_artifact_stdio_contract::editing::{snapshot_edit_source, SnapshotEditEvent, SnapshotEditingEditor};
    ::semio_framework_schema_registry::register_artifact_schema_descriptors(vec![crate::standards::v5::subsets::any::schema::html_artifact_schema_descriptor()]).expect("register html schema");
    let corpus: serde_json::Value = serde_json::from_str(NET_LEAVES).expect("net-leaves corpus");
    for case in corpus["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("id");
        let parse = |field: &str| <HtmlSnapshot as store::ArtifactDsl>::parse_dsl(case[field].as_str().expect("text")).unwrap_or_else(|error| panic!("{id}: {field} parses: {error:?}"));
        let (base, next) = (parse("before"), parse("after"));
        let event = SnapshotEditEvent::ReplaceSource { source: snapshot_edit_source(&next) };
        let emit = <HtmlEditor as SnapshotEditingEditor>::snapshot_edit_emit(&event, &base).unwrap_or_else(|fault| panic!("{id}: the details edit publishes: {fault:?}"));
        assert_eq!(serde_json::Value::Array(emit.artifact_mutations.iter().map(net_leaf_summary).collect()), case["leaves"], "{id}: the details edit is the net leaves");
        for leaf in &emit.artifact_mutations {
            let expected = match leaf {
                HtmlMutation::SetSnapshot(_) => ("Set snapshot", "Momentaufnahme setzen"),
                HtmlMutation::SetDoctype(_) => ("Set doctype", "Dokumenttyp setzen"),
                HtmlMutation::InsertNode(_) => ("Insert node", "Knoten einfügen"),
                HtmlMutation::RemoveNode(_) => ("Remove node", "Knoten entfernen"),
                HtmlMutation::SetElementName(_) => ("Set element name", "Elementname setzen"),
                HtmlMutation::SetAttribute(_) => ("Set attribute", "Attribut setzen"),
                HtmlMutation::SetText(_) => ("Set text", "Text setzen"),
                HtmlMutation::SetComment(_) => ("Set comment", "Kommentar setzen"),
                HtmlMutation::SetRawText(_) => ("Set raw text", "Rohtext setzen"),
            };
            let label = protocol::SemanticMutation::<HtmlSnapshot>::label(leaf);
            assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), expected.0, "{id}: English leaf label");
            assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), expected.1, "{id}: German leaf label");
        }
    }
}

/// ⚖️ LAW: the kit verb applies the HTML source the main window edits as ONE edit of its net leaves: one changed text is one
/// `set-text` row labelled from the leaf, the document reads exactly the applied text, and ONE undo restores it.
#[semio_framework_async_macros::async_test]
async fn one_applied_html_text_is_one_edit_of_its_net_leaves() {
    use semio_framework_plugin::PluginApp;
    let before = "<html><head><title>T</title></head><body><p>One</p></body></html>";
    let after = "<html><head><title>T</title></head><body><p>Uno</p></body></html>";
    let parse = |text: &str| <HtmlSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("html parses");
    let mut app = kit_fixture_holding(&parse(before)).await;
    let edits = app.edit_transactions().len();
    dispatch_settled(&mut app, "textEdit", &[("text", after)]).await.expect("the applied html settles");
    assert_eq!(app.snapshot().expect("html snapshot"), parse(after));
    assert_eq!(app.edit_transactions().len(), edits + 1, "one Apply, one edit");
    let rows: Vec<_> = app.history_snapshot().await.expect("history").upserts.into_iter().filter(|row| row.edit_id.is_some()).collect();
    let row = rows.iter().max_by_key(|row| row.seq).expect("the applied text's row");
    assert_eq!(row.mutations.len(), 1, "one changed text is one net leaf: {:?}", row.mutations);
    assert_eq!(
        row.mutations[0].label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En),
        protocol::SemanticMutation::<HtmlSnapshot>::label(&html_net_mutations(&parse(before), &parse(after))[0]).resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En)
    );
    semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut app, "undo", semio_framework_plugin::artifact_app_laws::meta("local").instance_id).await;
    assert_eq!(app.snapshot().expect("html snapshot"), parse(before), "one undo restores the committed document");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}
//#endregion 🧮️NetLeafLaws

semio_framework_plugin::history_edit_acceptance_law!("stdio", HtmlEditor, || semio_framework_plugin::App { definition: create_html_editor(), examples: Vec::new() }, "../..");
