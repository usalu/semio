use crate::editor_unit_tests::context;
use crate::editor_unit_tests::context::{dispatch, new_app, sourcing_manifest_for_tests};
use semio_plugin::artifact_app_laws;
use semio_plugin::{EditorApp, PluginApp};
use semio_framework_test_fixtures::fixture_text;

use semio_plugin::{ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactToolPublicationLane, MediaClass, MediaForm, MediaPayload};
use crate::{CurationSnapshot, SOURCING_CURATION_SCHEMA};
use crate::editor::sourcing::config::{SetContributionsEdit, SetFilterModulesEdit, SetFilterQueryEdit, SourcingCurationConfig, SourcingCurationConfigMutation};
use crate::editor::sourcing::modes::edit::windows::{curated, grid, pool, preview};
use crate::editor::sourcing::commands::{curation_add, curation_remove, curation_set_count, drop_on_curated, drop_on_pool, load_document_json, set_active_example, set_contributions, set_filter_min_availability, set_filter_module, set_filter_query, set_filter_typology, sort_table, stock_from_catalogue};
use crate::editor::sourcing::modes::edit;
use crate::editor::sourcing::{SOURCING_CURATION_BOUNDED_TOOL_IDS, SOURCING_CURATION_CONFIG_GRANT_BYTES, SOURCING_CURATION_CONFIG_STORE_MAXIMUM_BYTES, SOURCING_CURATION_CONFIG_STORE_MAXIMUM_ITEMS, SOURCING_CURATION_RETAINED_RAW_BYTES, SourcingCurationApp, SourcingCurationBoundedCommandJobFactory, SourcingCurationCommand, create_sourcing_curation_app, prepare_sourcing_curation_config, sourcing_curation_config_mutation_retained_bytes};
use crate::editor::sourcing::editor_vocabulary::{sourcing_curation_io, DEMO_STOCK_EXAMPLE_ID, EMPTY_EXAMPLE_ID, SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES, SOURCING_CURATION_CONFIG_TEXT_BYTES};
use semio_plugin::plugin_app_close_prelude::TypedOperationResultLane;

//#region 🧪️RetainedConfigOracle
#[semio_framework_async_macros::async_test]
async fn retained_example_load_publishes_authored_stock_and_closes_exact_owners() {
    let oracle: Vec<crate::ObjectKind> = semio_framework_pack_json::from_json_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/📦️expected-stock.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for example_id in [DEMO_STOCK_EXAMPLE_ID, EMPTY_EXAMPLE_ID] {
        let mut app = new_app().await;
        app.bind_instance_id(7).await;
        let live = PluginApp::document_archive(&*app).await.expect("live document archive");
        let live_id = protocol::os_spr::decode_history(&live.parent_spr, &protocol::os_spr::DecodeOptions::default()).await.unwrap().doc_id;
        semio_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &SourcingCurationCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: example_id.into() }), &semio_plugin::ActionMeta { expected_document_revision:None, actor: "fixture".into(), instance_id: 7, view_state: None }, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.unwrap();
        let receipt = semio_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app.0, 7, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("complete instance7 example settlement");
        let terminal = receipt.lanes.contains(&TypedOperationResultLane::Terminal);
        let mut document = None;
        for effect in receipt.effects {
            let semio_framework::kernel::Effect::LoadDocument { pack, spr } = effect else { panic!("example must publish a document load") };
            let history = protocol::os_spr::decode_history(&spr, &protocol::os_spr::DecodeOptions::default()).await.unwrap();
            assert_eq!(history.doc_id, live_id, "the load names the live store, not the app-minted id");
            assert!(history.composition.as_ref().and_then(|composition| composition.dialect.as_ref()).is_some(), "the load carries the dialect archive hydration checks");
            assert_eq!(history.schema, SOURCING_CURATION_SCHEMA);
            assert!(history.edits.is_empty());
            document = Some(<CurationSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&pack).unwrap());
        }
        assert!(!app.has_pending_typed_operations(), "complete instance7 example settlement");
        assert!(terminal);
        let document = document.expect("retained example document effect");
        let stock = crate::stock_of(&document);
        if example_id == DEMO_STOCK_EXAMPLE_ID {
            assert_eq!(stock, oracle);
        } else {
            assert!(stock.is_empty());
        }
        app.close_fixture();
        assert!(app.close_terminal_is_empty());
    }
}

/// 🚪️ The browser host answers an example load by handing the published `LoadDocument` back through
/// the document archive door with an empty member roster; the load must settle `Ready` and publish the
/// authored stock rather than trap the instance.
#[semio_framework_async_macros::async_test]
async fn example_load_settles_through_the_host_document_archive_door() {
    let oracle: Vec<crate::ObjectKind> = semio_framework_pack_json::from_json_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/📦️expected-stock.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mut app = new_app().await;
    app.bind_instance_id(7).await;
    semio_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &SourcingCurationCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: DEMO_STOCK_EXAMPLE_ID.into() }), &semio_plugin::ActionMeta { expected_document_revision:None, actor: "fixture".into(), instance_id: 7, view_state: None }, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.unwrap();
    let receipt = semio_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app.0, 7, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("example settlement");
    let loaded = receipt.effects.into_iter().find_map(|effect| match effect { semio_framework::kernel::Effect::LoadDocument { pack, spr } => Some((pack, spr)), _ => None });
    let (parent_pack, parent_spr) = loaded.expect("example publishes a document load");
    semio_plugin::artifact_app_laws::admit_document_archive_load(&mut app.0, 91, protocol::DocumentArchivePack { parent_pack, parent_spr, members: Vec::new() }, app.1.cx()).expect("archive admission");
    semio_plugin::artifact_app_laws::settle_document_archive_load(&mut app.0, 91, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("archive load settles Ready");
    assert_eq!(crate::stock_of(&app.snapshot().expect("loaded snapshot")), oracle);
}

/// 💾️ What the shell persists is `document_archive()`; a curation's archive must carry its derived
/// catalog member from the first frame (boot genesis) so a reload reopens it through the roster rather
/// than re-deriving it — and the reloaded document keeps the edits made on top of the example.
#[semio_framework_async_macros::async_test]
async fn a_saved_curation_archive_carries_its_catalog_member_and_reloads_with_its_edits() {
    let mut app = new_app().await;
    let snapshot = app.snapshot().expect("boot snapshot");
    let catalog_id = snapshot.catalog.child_id.clone();
    assert_eq!(catalog_id, crate::catalog_child_handle(&crate::stock_of(&snapshot)).child_id, "the boot document declares the handle its own stock derives");
    let object_id = crate::stock_of(&snapshot)[0].id.clone();
    dispatch(&mut app, SourcingCurationCommand::CurationAdd(curation_add::CurationAdd { object_id: object_id.clone() })).await;
    let archive = PluginApp::document_archive(&*app).await.expect("document archive");
    assert_eq!(archive.members.len(), 1, "boot genesis opened exactly the catalog member");
    assert_eq!((archive.members[0].owner.slot.as_str(), archive.members[0].reference.artifact_id.as_str(), archive.members[0].reference.subset.as_str()), (crate::CATALOG_CHILD_SLOT, catalog_id.as_str(), "kit"));
    semio_plugin::artifact_app_laws::admit_document_archive_load(&mut app.0, 92, archive, app.1.cx()).expect("archive admission");
    semio_plugin::artifact_app_laws::settle_document_archive_load(&mut app.0, 92, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("archive reload settles Ready");
    let reloaded = app.snapshot().expect("reloaded snapshot");
    assert_eq!(reloaded.catalog.child_id, catalog_id);
    assert_eq!(reloaded.curated.iter().map(|item| (item.object_id.as_str(), item.count)).collect::<Vec<_>>(), vec![(object_id.as_str(), 1)]);
    assert!(app.child_store(crate::CATALOG_CHILD_SLOT, &catalog_id).await.is_some(), "the reopened catalog member is live after the reload");
}

#[test]
fn retained_config_preparation_matches_the_json_oracle_and_rejects_maximum_plus_one() {
    let base = SourcingCurationConfig::default();
    let mut expected = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&base)).expect("JSON oracle base");
    expected["filters"]["query"] = serde_json::json!("timber");
    let (post, inverse) = prepare_sourcing_curation_config(&base, &SourcingCurationConfigMutation::SetFilterQuery(SetFilterQueryEdit { value: "timber".into() })).expect("bounded config candidate");
    assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&post)).expect("JSON oracle post"), expected);
    assert!(matches!(&inverse[0], SourcingCurationConfigMutation::SetFilterQuery(SetFilterQueryEdit { value }) if value == &base.filters.query));
    assert!(sourcing_curation_config_mutation_retained_bytes(&SourcingCurationConfigMutation::SetFilterQuery(SetFilterQueryEdit { value: "x".repeat(SOURCING_CURATION_CONFIG_TEXT_BYTES) })).is_ok());
    assert!(sourcing_curation_config_mutation_retained_bytes(&SourcingCurationConfigMutation::SetFilterQuery(SetFilterQueryEdit { value: "x".repeat(SOURCING_CURATION_CONFIG_TEXT_BYTES + 1) })).is_err());
    assert!(sourcing_curation_config_mutation_retained_bytes(&SourcingCurationConfigMutation::SetFilterModules(SetFilterModulesEdit { module_ids: Vec::new() })).is_ok(), "clearing the module filter is a retained edit");
    assert!(sourcing_curation_config_mutation_retained_bytes(&SourcingCurationConfigMutation::SetFilterModules(SetFilterModulesEdit { module_ids: vec!["m".into(); SOURCING_CURATION_CONFIG_STORE_MAXIMUM_ITEMS + 1] })).is_err());
    assert_eq!(SOURCING_CURATION_CONFIG_STORE_MAXIMUM_BYTES, 768 + SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES, "the retained config store is the filter envelope plus its own contributions lane");
    assert_eq!(SOURCING_CURATION_CONFIG_GRANT_BYTES, 4_096, "one config turn must still fit the host's fixed 4 KiB typed-operation page grant");
}

/// ⚖️ LAW: the contributions lane is priced on its OWN envelope, not the 96-byte filter-text one —
/// a `sourcing.module` pack is a host-pushed capability blob, never typed-in filter text. Its
/// ceiling is exactly the wire envelope `setContributions` is already granted, so nothing can be
/// retained that could not have crossed the tool boundary.
#[test]
fn the_contributions_lane_is_priced_apart_from_the_filter_text_envelope() {
    assert!(SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES > SOURCING_CURATION_CONFIG_TEXT_BYTES);
    let inside = SourcingCurationConfigMutation::SetContributions(SetContributionsEdit { json: "x".repeat(SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES) });
    assert!(sourcing_curation_config_mutation_retained_bytes(&inside).is_ok(), "a pack at the lane ceiling is admitted");
    let over = SourcingCurationConfigMutation::SetContributions(SetContributionsEdit { json: "x".repeat(SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES + 1) });
    assert!(sourcing_curation_config_mutation_retained_bytes(&over).is_err(), "one byte past the lane is refused, never truncated");
    let base = SourcingCurationConfig { contributions_json: "x".repeat(SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES), ..Default::default() };
    let (post, inverse) = prepare_sourcing_curation_config(&base, &SourcingCurationConfigMutation::SetFilterQuery(SetFilterQueryEdit { value: "timber".into() })).expect("a filter edit over a full contributions lane stays bounded");
    assert_eq!(post.contributions_json.len(), SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES, "a filter edit never disturbs the retained pack");
    assert!(matches!(&inverse[0], SourcingCurationConfigMutation::SetFilterQuery(_)));
}
/// 🧩️ The pack the DEMONSTRATOR's aussuchen pane actually receives: the demonstrator consumes
/// `sourcing.module`, `cad.computer` and `process.machines`, so the host hands this app all three
/// topics in ONE crossing. Sizes are the REAL per-entry bulk strings of the shipped extensions,
/// measured off the built dev manifests 2026-09-16 (ticket 26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS).
fn demonstrator_contributions_pack() -> String {
    let entry = |plugin_id: &str, topic: &str, app_id: &str, bulk: usize| semio_framework::ProgramContributionEntry {
        plugin_id: plugin_id.to_string(),
        topic_contribution: Some(semio_framework::TopicContribution::new(
            topic,
            semio_framework::DslValue::object([
                ("appId".to_string(), semio_framework::DslValue::String(app_id.to_string())),
                ("moduleId".to_string(), semio_framework::DslValue::String(plugin_id.to_string())),
                ("bulkJson".to_string(), semio_framework::DslValue::String("m".repeat(bulk))),
            ]),
        )),
    };
    let entries = vec![
        entry("cad-extension-aec-building-structure", "cad.computer", "cad-play", 3_337),
        entry("cad-extension-aec-building", "cad.computer", "cad-play", 940),
        entry("cad-extension-aec-building-energy", "cad.computer", "cad-play", 635),
        entry("cad-extension-spatial-shape", "cad.computer", "cad-play", 176),
        entry("process-extension-wood", "process.machines", "process3d-play", 4_705),
        entry("process-extension-metal", "process.machines", "process3d-play", 4_011),
        entry("process-extension-robotic", "process.machines", "process3d-play", 4_010),
        entry("process-extension-concrete", "process.machines", "process3d-play", 3_311),
        entry("sourcing-module-beams", "sourcing.module", "sourcing-curation", 780),
        entry("sourcing-module-windows", "sourcing.module", "sourcing-curation", 669),
        entry("sourcing-module-slabs", "sourcing.module", "sourcing-curation", 589),
    ];
    semio_framework_pack_json::to_json_string(&entries)
}

/// 🎁️ LAW: the REAL demonstrator pack crosses this app's registered `setContributions` admission and
/// distils to a roster the retained contributions lane holds.
///
/// 🏁️ Before the per-app admission, sourcing priced `setContributions` on the 8 KiB gesture envelope
/// every retained tool shares, and the live push died with `typed command raw JSON exceeds its
/// registered retained-page admission` the moment the host stopped cutting capability packs to `[]`
/// (2026-09-16, ticket 26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS).
#[test]
fn the_real_demonstrator_pack_is_admitted_by_the_registered_contributions_wire() {
    let pack = demonstrator_contributions_pack();
    let wire = semio_framework_pack_json::to_json_string(&("setContributions", semio_framework::DslValue::object([("json".to_string(), semio_framework::DslValue::String(pack.clone()))])));
    println!("[STATS] sourcing demonstrator pack packChars={} wireChars={}", pack.len(), wire.len());
    assert!(pack.len() > SOURCING_CURATION_RETAINED_RAW_BYTES, "the real pack is past the gesture envelope");
    assert!(wire.len() <= semio_plugin::CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES, "the real pack's command wire ({} B) must fit the registered admission", wire.len());
    let distilled = crate::standards::v1::subsets::any::io::text::snapshot::installable_contributions(&pack, SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES);
    assert!(distilled.len() <= SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES, "what is RETAINED is the distilled roster, never the pack");
    assert!(sourcing_curation_config_mutation_retained_bytes(&SourcingCurationConfigMutation::SetContributions(SetContributionsEdit { json: distilled })).is_ok());
}

/// 🧩️ A pack whose distilled roster outgrows the seal's one-page buffer, so the canonical seal must run its second hashing pass.
fn two_pass_contributions_pack() -> String {
    let entries: Vec<_> = (0..6)
        .map(|index| semio_framework::ProgramContributionEntry {
            plugin_id: format!("sourcing-module-extra-{index}"),
            topic_contribution: Some(semio_framework::TopicContribution::new(
                "sourcing.module",
                semio_framework::DslValue::object([
                    ("appId".to_string(), semio_framework::DslValue::String(crate::standards::v1::subsets::any::schema::domain::SOURCING_CURATION_APP_ID.to_string())),
                    ("moduleId".to_string(), semio_framework::DslValue::String(format!("sourcing-module-extra-{index}"))),
                    ("label".to_string(), semio_framework::DslValue::String("m".repeat(160))),
                    ("typologyJson".to_string(), semio_framework::DslValue::String("[]".to_string())),
                    ("kindsJson".to_string(), semio_framework::DslValue::String("[]".to_string())),
                ]),
            )),
        })
        .collect();
    semio_framework_pack_json::to_json_string(&entries)
}

/// 🎁️ LAW: a roster past the seal's one-page buffer settles through the same typed `setContributions` operation.
#[semio_framework_async_macros::async_test]
async fn a_two_pass_roster_settles_through_the_typed_set_contributions_operation() {
    let mut app = new_app().await;
    let pack = two_pass_contributions_pack();
    let distilled = crate::standards::v1::subsets::any::io::text::snapshot::installable_contributions(&pack, SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES);
    assert!(distilled.len() > 1_024, "the roster must outgrow the seal page ({} B)", distilled.len());
    dispatch(&mut app, SourcingCurationCommand::SetContributions(set_contributions::SetContributions { json: pack })).await;
}

/// 🎁️ LAW: the real demonstrator pack, dispatched as the typed `setContributions` operation, settles instead of stalling.
#[semio_framework_async_macros::async_test]
async fn the_real_demonstrator_pack_settles_through_the_typed_set_contributions_operation() {
    let mut app = new_app().await;
    dispatch(&mut app, SourcingCurationCommand::SetContributions(set_contributions::SetContributions { json: demonstrator_contributions_pack() })).await;
}

/// 🌐️ LAW: the real demonstrator pack settles through the typed `setContributions` operation under the browser host's per-turn grant (depth 64, 520 KB copy, 16 MiB capacity, 1 MiB release, 64 items) instead of the native fixture stock.
#[semio_framework_async_macros::async_test]
async fn the_real_demonstrator_pack_settles_under_the_browser_grant() {
    let mut app = new_app().await;
    let _browser = browser_grant();
    dispatch(&mut app, SourcingCurationCommand::SetContributions(set_contributions::SetContributions { json: demonstrator_contributions_pack() })).await;
}
/// 🧹️ Drives the runtime's live-cleanup clock the way the browser host does after every actor turn: one item and one page of bytes per maintenance unit, every unit required not to fault, and the ladder required to reach `Complete` (an `AwaitingInput` wait is not idle) before `RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT` (256) consecutive zero-progress steps — the rule the browser actor applies in `runtime_live_cleanup_nonterminal_status` and faults with `plugin.internal.zero-progress`.
///
/// 🔗️ See `🎚️min-availability-fault.md` of ticket 26/09/01/SOURCING-END-TO-END, "Defect 4 fix".
fn drain_live_cleanup_to_idle(app: &mut crate::editor_unit_tests::context::SourcingTestApp) {
    use semio_plugin::{MountedOwnerIdentityV1, MountedOwnerPhaseV1, MountedOwnerTurnV1, PluginLifecycleStep};
    const ZERO_PROGRESS_LIMIT: usize = 256;
    let mut stalled = 0usize;
    loop {
        let cx = app.1.cx();
        let identity = MountedOwnerIdentityV1 { instance_id: 1, operation: cx.operation(), generation: cx.generation() };
        let phase = app.0.mounted_owner_maintenance_phase().expect("maintenance phase");
        let mut policy = semio_plugin::artifact_app_laws::fixture_mounted_policy();
        let loan = match phase { MountedOwnerPhaseV1::Preparation => &mut policy.preparation, MountedOwnerPhaseV1::Maintenance => &mut policy.maintenance, MountedOwnerPhaseV1::Close => &mut policy.close };
        loan.maximum_items = loan.maximum_items.min(1);
        loan.maximum_copy_bytes = loan.maximum_copy_bytes.min(4096);
        let Some(mut turn) = MountedOwnerTurnV1::admit(1, identity, phase, policy, cx).expect("maintenance admission") else { stalled += 1; assert!(stalled < ZERO_PROGRESS_LIMIT, "live cleanup admitted no turn for {ZERO_PROGRESS_LIMIT} consecutive steps"); continue };
        match app.0.mounted_owner_maintenance_step(&mut turn).expect("a live-cleanup maintenance unit never faults") {
            PluginLifecycleStep::Complete(_) => return,
            PluginLifecycleStep::Progress(receipt) if receipt != semio_framework_value::retained_clone::RetainedCloneProgress::default() => stalled = 0,
            step @ (PluginLifecycleStep::Progress(_) | PluginLifecycleStep::Blocked { .. } | PluginLifecycleStep::AwaitingInput { .. }) => { stalled += 1; assert!(stalled < ZERO_PROGRESS_LIMIT, "live cleanup made no progress for {ZERO_PROGRESS_LIMIT} consecutive steps, last {step:?} (browser: plugin.internal.zero-progress)"); }
        }
    }
}

/// 🎚️ LAW: the minimum-availability filter, typed or stepped, settles under the browser host's per-turn grant, applies, and its retired owners clean up without faulting the instance.
#[semio_framework_async_macros::async_test]
async fn the_min_availability_filter_settles_under_the_browser_grant_and_keeps_the_instance_alive() {
    let mut app = new_app().await;
    let _browser = browser_grant();
    for payload in [set_filter_min_availability::SetFilterMinAvailability { delta: None, value: Some(3.0) }, set_filter_min_availability::SetFilterMinAvailability { delta: Some(1.0), value: None }, set_filter_min_availability::SetFilterMinAvailability { delta: Some(1.0), value: None }] {
        dispatch(&mut app, SourcingCurationCommand::SetFilterMinAvailability(payload)).await;
        drain_live_cleanup_to_idle(&mut app);
    }
    dispatch(&mut app, SourcingCurationCommand::SetFilterQuery(set_filter_query::SetFilterQuery { value: "timber".into() })).await;
    drain_live_cleanup_to_idle(&mut app);
}

/// 🔢️ LAW: the minimum-availability input delivers its number over the wire — as a JSON number, a whole number, or the input's text — and every form applies, then cleans up without faulting the instance.
#[semio_framework_async_macros::async_test]
async fn the_min_availability_wire_payload_applies_in_every_number_form() {
    wire_forms_settle(&[r#"{"value":3}"#, r#"{"value":4.0}"#, r#"{"delta":1}"#, r#"{"delta":-1}"#, r#"{"value":"5"}"#]).await;
}

/// 🎚️ LAW: the browser delivers the number input as a continuous control — ticks with `commit:false` and a release with `commit:true` — and the press leaf of the Config lane settles after each.
#[semio_framework_async_macros::async_test]
async fn the_min_availability_gesture_forms_settle_after_every_tick_and_release() {
    wire_forms_settle(&[r#"{"value":6,"gesture":"input:1","commit":false}"#, r#"{"value":6,"gesture":"input:1","commit":true}"#, r#"{"value":7,"gesture":"input:2","commit":true}"#]).await;
}

/// 🎚️ LAW: the browser delivers the host abort of an open continuous control (blur) and the press leaf of the Config lane settles after it.
///
/// 🔗️ See `🎚️min-availability-fault.md` of ticket 26/09/01/SOURCING-END-TO-END, "Defect 3 fix".
#[semio_framework_async_macros::async_test]
async fn the_min_availability_gesture_forms_settle_after_every_tick_release_and_abort() {
    wire_forms_settle(&[r#"{"value":6,"gesture":"input:1","commit":false}"#, r#"{"value":6,"gesture":"input:1","commit":true}"#, r#"{"value":7,"gesture":"input:2","commit":true}"#, r#"{"gesture":"input:3","abort":"blur"}"#]).await;
}

async fn wire_forms_settle(forms: &[&str]) {
    use crate::editor_unit_tests::context::dispatch_wire;
    let mut app = new_app().await;
    let _browser = browser_grant();
    for arguments in forms {
        let outcome = dispatch_wire(&mut app, "setFilterMinAvailability", arguments).await;
        outcome.unwrap_or_else(|fault| panic!("min-availability wire payload {arguments} must settle: {fault:?}"));
        drain_live_cleanup_to_idle(&mut app);
    }
    dispatch(&mut app, SourcingCurationCommand::SetFilterQuery(set_filter_query::SetFilterQuery { value: "timber".into() })).await;
    drain_live_cleanup_to_idle(&mut app);
}

/// 🌐️ LAW: the exact browser sequence of the minimum-availability number input — a tick with the real ~55-char gesture id and `windowId`, the blur/Enter release of the same press seconds later with `commit:true`, then the next command (`sortTable`) — drains its cleanup ladder to idle, applies the Pool filter and sorts.
///
/// 🔗️ See `🎚️min-availability-fault.md` of ticket 26/09/01/SOURCING-END-TO-END, "Defect 4 fix".
#[semio_framework_async_macros::async_test]
async fn the_min_availability_browser_press_drains_its_cleanup_ladder_and_keeps_the_next_command_alive() {
    let mut app = browser_press_app().await;
    browser_press_wire(&mut app).await;
    assert_eq!(app.0.rendered_config().filters.min_availability, BROWSER_PRESS_MIN_AVAILABILITY, "the released press must filter the Pool");
    let browser = browser_grant();
    dispatch(&mut app, SourcingCurationCommand::SortTable(sort_table::SortTable { column_id: "availability".into(), direction: "desc".into() })).await;
    drain_live_cleanup_to_idle(&mut app);
    drop(browser);
    let config = app.0.rendered_config();
    assert_eq!(config.filters.sort.as_ref().map(|sort| sort.column_id.as_str()), Some("availability"), "sortTable must apply after the released press");
    assert_eq!(config.filters.min_availability, BROWSER_PRESS_MIN_AVAILABILITY, "the sort keeps the filter");
}

/// 🌐️ LAW: a browser tick followed by its release, with no follow-up command, drains the cleanup ladder to idle and the Pool filter applies.
///
/// 🔗️ See `🎚️min-availability-fault.md` of ticket 26/09/01/SOURCING-END-TO-END, "Defect 4 fix".
#[semio_framework_async_macros::async_test]
async fn the_min_availability_browser_press_drains_its_cleanup_ladder_without_a_follow_up_command() {
    let mut app = browser_press_app().await;
    browser_press_wire(&mut app).await;
    assert_eq!(app.0.rendered_config().filters.min_availability, BROWSER_PRESS_MIN_AVAILABILITY, "the released press must filter the Pool");
}

/// 🌐️ LAW: the browser sends a press's release while the tick's typed operation still publishes (the number input blurs 90 ms after the fill); both operations settle in order, the cleanup ladder drains and the release filters the Pool.
///
/// 🔗️ See `🎚️min-availability-fault.md` of ticket 26/09/01/SOURCING-END-TO-END, "Defect 5: scrub settlement stall".
#[semio_framework_async_macros::async_test]
async fn the_min_availability_release_sent_while_its_tick_still_publishes_settles_both() {
    use crate::editor_unit_tests::context::{dispatch_wire_unsettled, settle};
    let mut app = browser_press_app().await;
    let press = "sourcing-filter-min-availability:1795524369502:1";
    let browser = browser_grant();
    for commit in [false, true] {
        let arguments = format!(r#"{{"value":{BROWSER_PRESS_MIN_AVAILABILITY},"gesture":"{press}","commit":{commit},"windowId":"sourcing-pool"}}"#);
        dispatch_wire_unsettled(&mut app, "setFilterMinAvailability", &arguments).await.unwrap_or_else(|fault| panic!("{arguments}: {fault:?}"));
    }
    settle(&mut app).await;
    drain_live_cleanup_to_idle(&mut app);
    drop(browser);
    assert_eq!(app.0.rendered_config().filters.min_availability, BROWSER_PRESS_MIN_AVAILABILITY, "the released press must filter the Pool");
}

const BROWSER_PRESS_MIN_AVAILABILITY: u32 = 25;

async fn browser_press_app() -> crate::editor_unit_tests::context::SourcingTestApp {
    let mut app = new_app().await;
    let browser = browser_grant();
    dispatch(&mut app, SourcingCurationCommand::SetContributions(set_contributions::SetContributions { json: demonstrator_contributions_pack() })).await;
    drain_live_cleanup_to_idle(&mut app);
    drop(browser);
    app
}

const BROWSER_TURN_FUEL: u64 = 50_000_000;

/// 🌐️ The per-turn grant AND fuel the browser host issues: 64 items, 520 KB copy, 16 MiB capacity, 1 MiB release, depth 64 and 50,000,000 fuel (the native fixture grants 2,000,000).
fn browser_grant() -> impl Sized {
    let stock = semio_framework_value::RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 520 * 1024, maximum_capacity_bytes: 16 * 1024 * 1024, maximum_release_bytes: 1024 * 1024, maximum_depth: 64 };
    (semio_plugin::artifact_app_laws::arm_host_turn_grant(stock), semio_plugin::artifact_app_laws::arm_host_turn_fuel(BROWSER_TURN_FUEL))
}

async fn browser_press_wire(app: &mut crate::editor_unit_tests::context::SourcingTestApp) {
    use crate::editor_unit_tests::context::dispatch_wire;
    let press = "sourcing-filter-min-availability:1795524369502:1";
    for (commit, now_ms) in [(false, 1_000_000u64), (true, 1_004_650)] {
        app.0.set_tool_clock_ms(Some(now_ms));
        let arguments = format!(r#"{{"value":{BROWSER_PRESS_MIN_AVAILABILITY},"gesture":"{press}","commit":{commit},"windowId":"sourcing-pool"}}"#);
        let browser = browser_grant();
        dispatch_wire(app, "setFilterMinAvailability", &arguments).await.unwrap_or_else(|fault| panic!("{arguments}: {fault:?}"));
        drain_live_cleanup_to_idle(app);
        drop(browser);
    }
}


//#endregion 🧪️RetainedConfigOracle
