/// 🎫️ Controller and envelope laws of the surface fixture: the derived controller id, the manifest declaration, the canonical surface app id of actions and envelopes, the codec table pair and the refused contract mutating verbs.

/// 🪟️ The controller literal every proof row, tool key and manifest id in this fixture is stated
/// against IS the id `EditorApp` derives at runtime. A drift here would make the proof catalog
/// authoritative for a controller nothing dispatches to, which `validate_tool_job_rows` reports as
/// `interactive-job.catalog-controller` from inside a dispatch instead of here.
#[semio_framework_async_macros::async_test]
async fn surface_editor_controller_is_the_derived_id() {
    let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    assert_eq!(SURFACE_CONTROLLER_ID, semio_framework::surface_app_id(&SURFACE_TESTKIT_DIALECT.into(), semio_framework::AppRole::Editor));
    let mut app = new_registered_app::<EditorApp<SurfaceEditorFixture>, _>(surface_manifest(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
    assert_eq!(app.app_id().await, SURFACE_CONTROLLER_ID);
    close_registered_fixture_app(&mut app, crate::app_testing::artifact_app_laws::fixture_mounted_policy(),cx).expect("original fixture terminal close");

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}

/// 🔒️ The registry-less wrapper still fails CLOSED on the same editor: an owned factory never
/// substitutes for a manifest declaration, so a surface whose verb is undeclared is refused by name
/// before any factory is reached, and the document is untouched.
#[semio_framework_async_macros::async_test]
async fn editor_fixture_without_a_manifest_declaration_fails_closed() {
    let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let mut app = new_app::<EditorApp<SurfaceEditorFixture>>(protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
    let error = crate::app_testing::artifact_app_laws::dispatch_original_fixture_command(&mut app, &SurfaceEditorCommand::Increment, &meta("local"), crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity(), cx).await.expect_err("registry-less editor must fail closed");
    assert_eq!(error.primary_fault().expect("a diagnostic fault").code.0, "interactive-job.unknown-key");
    assert!(error.primary_fault().expect("a diagnostic fault").message.contains(SURFACE_TOOL_ID), "the refusal names the editor's own verb, not the generic placeholder: {}", error.primary_fault().expect("a diagnostic fault").message);
    assert_eq!(app.snapshot().unwrap().count, 0, "a fail-closed dispatch never reaches the reducer");
    close_registered_fixture_app(&mut app, crate::app_testing::artifact_app_laws::fixture_mounted_policy(),cx).expect("original fixture terminal close");

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}

/// 🐛️ Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS lane 4-G — WITH
/// TEETH: `ShellHost::encodeWindowActionInvocation` addresses every real click with the
/// surface's canonical id (`surface_app_id`, e.g. `s.space.home@1/*#editor`), never
/// `EditorApp::APP_ID`'s runtime-const placeholder (`"surface"`). Before the fix,
/// `handle_action_invocation`'s ownership check compared `address.app_id` against that
/// literal placeholder, so even the textbook-correct canonical id was rejected — every
/// button click across every `EditorApp`/`ViewerApp` surface in the product. This fixture's
/// registry is empty (`new_app`, contract-enforcement-less), so once ownership passes the
/// very next check (`registry.has_mode`) must fail instead — proving the rejection was
/// specifically the ownership line, not a coincidence of an otherwise-valid dispatch.
#[semio_framework_async_macros::async_test]
async fn handle_action_invocation_accepts_the_real_canonical_surface_app_id() {
    let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    use semio_framework::manifest::{ActionAddress, ActionInvocation};
    let mut app = new_app::<EditorApp<SurfaceEditorFixture>>(protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
    let real_id = semio_framework::surface_app_id(&SURFACE_TESTKIT_DIALECT.into(), semio_framework::AppRole::Editor);
    let invocation = ActionInvocation {
        address: ActionAddress { plugin_id: "test".into(), app_id: real_id.clone(), mode_id: "edit".into(), window_kind_id: "main".into(), window_instance_id: "main-instance".into(), action_id: "increment".into() },
        arguments: Default::default(),
    };
    let mut args=Some(semio_framework_value::DslValue::Object(invocation.arguments.into_iter().collect()));let mut fault=None;
    let error=crate::app_testing::artifact_app_laws::dispatch_addressed_fixture_on_original(&mut app,&invocation.address,&mut args,&mut fault,Some("edit"),&meta("local"),&mut crate::app_testing::artifact_app_laws::fixture_identity(),cx).await.expect_err("registry-less fixture declares no modes");
    let error=error.primary_fault().expect("mode ownership retains its complete original Fault");
    assert!(!error.message.contains("does not match"), "the real canonical app id must satisfy the ownership check, got: {}", error.message);
    assert!(error.message.contains("unknown action mode owner"), "expected ownership to pass and the mode lookup to fail instead, got: {}", error.message);
    assert_eq!(app.app_id().await, real_id, "PluginApp::app_id must report the real canonical id, not the APP_ID placeholder");
    close_registered_fixture_app(&mut app, crate::app_testing::artifact_app_laws::fixture_mounted_policy(),cx).expect("original fixture terminal close");

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}

/// 🪪️ Verifies `EditorApp` initializes its document, config, draft, and interaction
/// envelopes with the real canonical surface app id, not the `APP_ID` placeholder.
#[semio_framework_async_macros::async_test]
async fn editor_app_envelopes_carry_the_real_canonical_surface_app_id() {
    let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let mut app = new_app::<EditorApp<SurfaceEditorFixture>>(protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
    let real_id = semio_framework::surface_app_id(&SURFACE_TESTKIT_DIALECT.into(), semio_framework::AppRole::Editor);
    assert_eq!(app.stores.store.envelope().id, real_id);
    assert_eq!(app.stores.config_store.envelope().id, format!("{real_id}-config"));
    assert_eq!(app.stores.draft_store.envelope().id, format!("{real_id}-draft"));
    assert_eq!(app.stores.interaction_store.envelope().id, format!("{real_id}-interaction"));
    close_registered_fixture_app(&mut app, crate::app_testing::artifact_app_laws::fixture_mounted_policy(),cx).expect("original fixture terminal close");

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}

/// 🪪️ Verifies `ViewerApp` initializes its document, config, draft, and interaction
/// envelopes with the real canonical surface app id, not the `APP_ID` placeholder.
#[semio_framework_async_macros::async_test]
async fn viewer_app_envelopes_carry_the_real_canonical_surface_app_id() {
    let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let mut app = new_viewer::<SurfaceViewerFixture>(protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
    let real_id = semio_framework::surface_app_id(&SURFACE_TESTKIT_DIALECT.into(), semio_framework::AppRole::Viewer);
    assert_eq!(app.stores.store.envelope().id, real_id);
    assert_eq!(app.stores.config_store.envelope().id, format!("{real_id}-config"));
    assert_eq!(app.stores.draft_store.envelope().id, format!("{real_id}-draft"));
    assert_eq!(app.stores.interaction_store.envelope().id, format!("{real_id}-interaction"));
    close_registered_fixture_app(&mut app, crate::app_testing::artifact_app_laws::fixture_mounted_policy(),cx).expect("original fixture terminal close");

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}

/// 📌️ A hub Check In validates the pair it folded through the guest codec's `print-mirror`, and a zero-op `apply-ops`
/// batch passes a pair through: both read a POPULATED pair, whose unadopted envelope must be retired entry by entry —
/// dropping its owners aborted the guest on the history ledger's terminal-empty witness, so every writer and note Check In
/// was refused `codec-refused` (ticket 26/09/23, session 14).
#[semio_framework_async_macros::async_test]
async fn the_codec_table_mirrors_and_passes_through_a_populated_pair_without_aborting() {
    let table = crate::surface_contract::artifact_codec_table::<EditorApp<SurfaceEditorFixture>>();
    let genesis = (table.genesis)("artifact-5c0dec0de5c0dec0de5c0dec0de5c0de").await.expect("genesis pair of a server-minted artifact id");
    let op = protocol::OpBinary::encode_op(&SurfaceMutation::from(SetSurfaceCount { value: 7 })).expect("encode set-surface-count");
    let populated = (table.apply_ops)(&genesis.pack, &genesis.spr, &store::os_spr::encode_ops_vec(&[op]), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await.expect("one op lands one edit");
    let history = store::os_spr::decode_history(&populated.spr, &store::os_spr::DecodeOptions::default()).await.expect("populated history");
    assert_eq!(history.edits.len(), 1, "the pair carries one edit, so its history ledger is populated");
    let mirror = (table.print_mirror)(&populated.pack, &populated.spr).await.expect("a populated pair mirrors");
    assert!(mirror.ops.contains("set-surface-count"), "the mirror prints the pair's edit: {}", mirror.ops);
    let passed = (table.apply_ops)(&populated.pack, &populated.spr, &store::os_spr::encode_ops_vec(&[]), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await.expect("an empty batch passes a populated pair through");
    assert!(!passed.pack.is_empty() && !passed.spr.is_empty(), "an empty batch over a populated pair returns that pair");
}

/// 👁️🔒 Contract §2.3 clause 1/2 — WITH TEETH: dispatches the eight frozen mutating verbs
/// through the full `VcsArtifactApp<ViewerApp<V>>` runtime path (`handle_action` for the
/// seven string actions, `import_media` for the eighth) and asserts every one comes back
/// `Fault { origin: FaultOrigin::Framework, code: FaultCode::new("viewer.read-only"), .. }`.
#[semio_framework_async_macros::async_test]
async fn viewer_rejects_every_contract_mutating_verb() {
    let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let mut app = new_viewer::<SurfaceViewerFixture>(protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
    for verb in ["undo", "redo", "commitCheckpoint", "createAlternative", REVERT_TO_COMMAND_ACTION_ID, "cut", "paste"] {
        let error = app.handle_action(verb, None, &meta("local"), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await.err().unwrap_or_else(|| panic!("'{verb}' must be rejected on a viewer instance"));
        assert_eq!(error.origin, FaultOrigin::Framework, "'{verb}' rejection must carry FaultOrigin::Framework");
        assert_eq!(error.code.0, "viewer.read-only", "'{verb}' rejection must carry the frozen viewer.read-only code");
    }
    let media = Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "semio.testkit-surface/v1".into(), json: "{}".into() } };
    let error = app.import_media("any-port", media, &meta("local")).await.err().expect("'import' must be rejected on a viewer instance");
    assert_eq!(error.origin, FaultOrigin::Framework, "'import' rejection must carry FaultOrigin::Framework");
    assert_eq!(error.code.0, "viewer.read-only", "'import' rejection must carry the frozen viewer.read-only code");
    close_registered_fixture_app(&mut app, crate::app_testing::artifact_app_laws::fixture_mounted_policy(),cx).expect("original fixture terminal close");

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}
