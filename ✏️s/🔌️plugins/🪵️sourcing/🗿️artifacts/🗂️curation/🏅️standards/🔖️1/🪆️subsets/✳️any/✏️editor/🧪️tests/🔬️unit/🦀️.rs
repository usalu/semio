
use super::*;
use semio_framework_plugin::plugin_app_close_prelude::TypedOperationResultLane;

//#region 🧪️RetainedConfigOracle
#[semio_framework_async_macros::async_test]
async fn retained_example_load_publishes_authored_stock_and_closes_exact_owners() {
    let oracle: Vec<crate::ObjectKind> = dsl::json::from_json_str(include_str!("../../../📚️examples/🎬️demo/📦️expected-stock.json")).unwrap();
    for example_id in [DEMO_STOCK_EXAMPLE_ID, EMPTY_EXAMPLE_ID] {
        let mut app = new_app().await;
        app.bind_instance_id(7).await;
        app.dispatch_typed(SourcingCurationCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: example_id.into() }), &semio_framework_plugin::ActionMeta { actor: "fixture".into(), instance_id: 7, view_state: None }).await.unwrap();
        let mut document = None;
        let mut terminal = false;
        for _ in 0..100_000 {
            app.maintenance_step(1, 4_096).unwrap();
            app.advance_typed_operation_publication().await.unwrap();
            if let Some(page) = app.take_typed_operation_result_page(7) {
                assert_ne!(page.lane, TypedOperationResultLane::Fault, "{}", String::from_utf8_lossy(page.bytes()));
                terminal |= page.lane == TypedOperationResultLane::Terminal;
                assert!(app.acknowledge_typed_operation_result(page.token).unwrap());
            }
            if let Some(effect) = app.take_typed_operation_effect() {
                let semio_framework::kernel::Effect::LoadDocument { pack, spr } = effect else { panic!("example must publish a document load") };
                let history = protocol::os_spr::decode_history(&spr, &protocol::os_spr::DecodeOptions::default()).await.unwrap();
                assert_eq!(history.doc_id, "curation");
                assert_eq!(history.schema, SOURCING_CURATION_SCHEMA);
                assert!(history.edits.is_empty());
                document = Some(CurationSnapshot::decode_pack(&pack).unwrap());
            }
            app.take_typed_operation_event();
            app.take_typed_operation_ui_scope();
            if !app.has_pending_typed_operations() {
                break;
            }
            std::thread::yield_now();
        }
        assert!(terminal);
        let document = document.expect("retained example document effect");
        let stock = crate::stock_of(&document);
        if example_id == DEMO_STOCK_EXAMPLE_ID {
            assert_eq!(stock, oracle);
        } else {
            assert!(stock.is_empty());
        }
        for _ in 0..100_000 {
            if app.close_terminal_is_empty() {
                break;
            }
            app.close_step(1, 4_096).unwrap();
            std::thread::yield_now();
        }
        assert!(app.close_terminal_is_empty());
        eprintln!("[DEBUG] retained example {example_id} published {} authored rows and retired", stock.len());
    }
}

#[test]
fn retained_config_preparation_matches_the_json_oracle_and_rejects_maximum_plus_one() {
    let base = SourcingCurationConfig::default();
    let mut expected = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&base)).expect("JSON oracle base");
    expected["filters"]["query"] = serde_json::json!("timber");
    let (post, inverse, _) = prepare_sourcing_curation_config(&base, SourcingCurationConfigMutation::SetFilterQuery { value: "timber".into() }).expect("bounded config candidate");
    assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&post)).expect("JSON oracle post"), expected);
    assert!(matches!(&inverse[0], SourcingCurationConfigMutation::SetFilterQuery { value } if value == &base.filters.query));
    assert!(sourcing_curation_config_mutation_footprint(&SourcingCurationConfigMutation::SetFilterQuery { value: "x".repeat(SOURCING_CURATION_CONFIG_TEXT_BYTES) }).is_ok());
    assert!(sourcing_curation_config_mutation_footprint(&SourcingCurationConfigMutation::SetFilterQuery { value: "x".repeat(SOURCING_CURATION_CONFIG_TEXT_BYTES + 1) }).is_err());
    assert!(sourcing_curation_config_mutation_footprint(&SourcingCurationConfigMutation::SetFilterModules { module_ids: Vec::new() }).is_err());
    assert_eq!(SOURCING_CURATION_CONFIG_STORE_MAXIMUM_BYTES * 4 + 1_024, 4_096);
}
//#endregion 🧪️RetainedConfigOracle
use crate::editor::sourcing::testkit::{new_app, sourcing_manifest_for_testkit};
use semio_framework_plugin::testkit;
use semio_framework_plugin::{EditorApp, PluginApp};

#[semio_framework_async_macros::async_test]
async fn view_kind_config_only_commands_pass_kind_discipline() {
    // 🧬️ A registry-backed wrapper so the View-kind declarations actually get enforced.
    let mut app = new_app().await;
    let result = app.dispatch_typed(SourcingCurationCommand::SetFilterQuery(set_filter_query::SetFilterQuery { value: "glulam".into() }), &testkit::meta("local")).await.expect("filter query");
    assert!(result.mutations.is_empty(), "setFilterQuery is config-only, no document operations");
}

//#region 🔖️CommandSurface
/// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every
/// row's wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), 15, "every SourcingCurationCommand row must be covered by every_command()");
}

/// ⚖️ LAW: the PRODUCTION action bridge covers every declared row. `ArtifactEditor`'s default
/// `command_from_action` rejects app-owned actions outright, so an app that declares actions but
/// never overrides it ships a UI whose every control faults — which is exactly how this pane's
/// `setActiveExample` (and with it the whole `demo-stock` example) was dead in the demonstrator.
/// Asserted through `<SourcingCurationApp as ArtifactEditor>` — NOT the free function — because the
/// trait method is the seam the host actually calls.
#[semio_framework_async_macros::async_test]
async fn the_production_action_bridge_admits_every_declared_command() {
    for command in every_command() {
        let action = command.command_id();
        let built = <SourcingCurationApp as ArtifactEditor>::command_from_action(action, None).unwrap_or_else(|fault| panic!("action '{action}' is declared but the production bridge rejects it: {fault:?}"));
        assert_eq!(built.command_id(), action, "the bridge routed '{action}' to the wrong command");
    }
}

/// ⚖️ LAW: the framework's own conformance harness — it walks the actions this app's window kinds
/// actually RENDER, stages each one's declared args exactly as the host does, and knows which ids
/// are framework-injected (`undo`/`copy`/`recordTutorial`/…) and must be skipped. Strictly stronger
/// than enumerating the command rows: it catches an action that chrome declares but no command row
/// backs.
#[semio_framework_async_macros::async_test]
async fn every_rendered_action_bridges_through_the_framework_harness() {
    testkit::assert_declared_actions_bridge_to_commands::<EditorApp<SourcingCurationApp>>(sourcing_manifest_for_testkit).await;
}

/// ⚖️ LAW: the bridge reads the manifest's OWN arg names — `setActiveExample` declares a select
/// arg keyed `exampleId` (`🔖️Manifest`), and the payload field is `example_id`; the two
/// vocabularies are joined here and nowhere else.
#[semio_framework_async_macros::async_test]
async fn the_action_bridge_reads_the_declared_arg_names() {
    let built = <SourcingCurationApp as ArtifactEditor>::command_from_action("setActiveExample", Some(&protocol::DslValue::from(&serde_json::json!({ "exampleId": DEMO_STOCK_EXAMPLE_ID })))).expect("setActiveExample must convert");
    assert_eq!(built, SourcingCurationCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: DEMO_STOCK_EXAMPLE_ID.into() }));
    let filter = <SourcingCurationApp as ArtifactEditor>::command_from_action("setFilterModule", Some(&protocol::DslValue::from(&serde_json::json!({ "moduleId": "walls", "enabled": true })))).expect("setFilterModule must convert");
    assert_eq!(filter, SourcingCurationCommand::SetFilterModule(set_filter_module::SetFilterModule { module_id: "walls".into(), enabled: true }));
    assert!(<SourcingCurationApp as ArtifactEditor>::command_from_action("noSuchAction", None).is_err(), "an undeclared action must fault, not silently no-op");
}

/// ⚖️ LAW: text and binary are two projections of the same command, for every single row — the
/// permanent successor of the old `📡️protocol` crate's
/// `sourcing_curation_command_op_text_round_trips_every_variant`.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — copied
/// verbatim from each `app_commands!` row's `as "…"` literal (NOT a mechanical kebab-case of the
/// manifest action id: `setDocument`/`document-json`, `setActiveExample`/`active-example`, the whole
/// `setFilter*` family, and `setLocale`/`locale` all drop or rewrite the `set` prefix). This is what a
/// missing `#[dsl(keyword = ..)]` on a payload struct silently breaks (the record prints with no
/// keyword at all and no longer parses).
#[semio_framework_async_macros::async_test]
async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
    fn expected_wire_key(id: &str) -> &'static str {
        match id {
            "setDocument" => "document-json",
            "setActiveExample" => "active-example",
            "stockFromCatalogue" => "stock-from-catalogue",
            "curationAdd" => "curation-add",
            "curationSetCount" => "curation-set-count",
            "curationRemove" => "curation-remove",
            "dropOnPool" => "drop-on-pool",
            "dropOnCurated" => "drop-on-curated",
            "setFilterQuery" => "filter-query",
            "setFilterModule" => "filter-module",
            "setFilterTypology" => "filter-typology",
            "setFilterMinAvailability" => "filter-min-availability",
            "sortTable" => "sort-table",
            "setLocale" => "locale",
            "setContributions" => "contributions",
            other => panic!("expected_wire_key: unhandled command id {other}"),
        }
    }
    for command in every_command() {
        let id = command.command_id();
        let printed = protocol::OpText::print_op(&command);
        assert_eq!(printed.split(' ').next().unwrap_or_default(), expected_wire_key(id), "wire keyword drifted for command {id}: {printed:?}");
    }
}

/// ⚖️ The rows whose `Option` fields make `None`/`Some` distinct wire cases, pinned to the exact bytes
/// captured from the pre-merge `semio-s-app-sourcing-curation-protocol` crate
/// (`wire-baseline-before.txt` in this ticket's folder). A regression here is a real format break, not
/// a test-fixture mismatch.
#[semio_framework_async_macros::async_test]
async fn optional_field_rows_keep_their_pre_migration_bytes() {
    let cases: [(SourcingCurationCommand, &str, &str); 2] = [
        (
            SourcingCurationCommand::CurationSetCount(curation_set_count::CurationSetCount { object_id: "beam-glulam-gl24h".into(), delta: Some(1.0), value: None }),
            "curation-set-count curation-set-count object-id=beam-glulam-gl24h delta=1",
            "010401116265616d2d676c756c616d2d676c323468020006000105000000000000f03f",
        ),
        (
            SourcingCurationCommand::CurationSetCount(curation_set_count::CurationSetCount { object_id: "beam-glulam-gl24h".into(), delta: None, value: Some(4.0) }),
            "curation-set-count curation-set-count object-id=beam-glulam-gl24h value=4",
            "010401116265616d2d676c756c616d2d676c3234680200060002050000000000001040",
        ),
    ];
    for (command, text, hex) in cases {
        assert_eq!(protocol::OpText::print_op(&command), text);
        assert_eq!(protocol::OpBinary::encode_op(&command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>(), hex);
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order — mirrors the
/// pre-migration wire baseline captured into this ticket's `wire-baseline-before.txt`.
fn every_command() -> Vec<SourcingCurationCommand> {
    vec![
        SourcingCurationCommand::SetArtifactJson(set_artifact_json::SetArtifactJson { json: "{}".into() }),
        SourcingCurationCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "demo-stock".into() }),
        SourcingCurationCommand::StockFromCatalogue(stock_from_catalogue::StockFromCatalogue {}),
        SourcingCurationCommand::CurationAdd(curation_add::CurationAdd { object_id: "beam-glulam-gl24h".into() }),
        SourcingCurationCommand::CurationSetCount(curation_set_count::CurationSetCount { object_id: "beam-glulam-gl24h".into(), delta: Some(1.0), value: None }),
        SourcingCurationCommand::CurationRemove(curation_remove::CurationRemove { object_id: "beam-glulam-gl24h".into() }),
        SourcingCurationCommand::DropOnPool(drop_on_pool::DropOnPool { object_id: "beam-glulam-gl24h".into() }),
        SourcingCurationCommand::DropOnCurated(drop_on_curated::DropOnCurated { object_id: "beam-glulam-gl24h".into() }),
        SourcingCurationCommand::SetFilterQuery(set_filter_query::SetFilterQuery { value: "glulam".into() }),
        SourcingCurationCommand::SetFilterModule(set_filter_module::SetFilterModule { module_id: "beams".into(), enabled: true }),
        SourcingCurationCommand::SetFilterTypology(set_filter_typology::SetFilterTypology { path: "beams/steel".into() }),
        SourcingCurationCommand::SetFilterMinAvailability(set_filter_min_availability::SetFilterMinAvailability { delta: Some(1.0), value: None }),
        SourcingCurationCommand::SortTable(sort_table::SortTable { column_id: "availability".into(), direction: "desc".into() }),
        SourcingCurationCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
        SourcingCurationCommand::SetContributions(set_contributions::SetContributions { json: "[]".into() }),
    ]
}
//#endregion 🔖️CommandSurface

#[semio_framework_async_macros::async_test]
async fn app_definition_labels_resolve_german() {
    let def = &create_sourcing_curation_app();
    let (terminology, locale) = (semio_framework_plugin::Terminology::Native, semio_framework_plugin::Locale::De);
    assert_eq!(def.window_kinds.iter().find(|entry| entry.id == pool::SOURCING_CURATION_WINDOW_POOL).expect("pool window").label.resolve(terminology, locale), "Pool");
    assert_eq!(def.window_kinds.iter().find(|entry| entry.id == curated::SOURCING_CURATION_WINDOW_CURATED).expect("curated window").label.resolve(terminology, locale), "Kuratiert");
    assert_eq!(def.modes.iter().find(|entry| entry.id == edit::SOURCING_CURATION_MODE_CURATION).expect("curation mode").label.resolve(terminology, locale), "Kuratierung");
}

#[test]
fn host_contributions_resolve_to_the_event_sourced_config_lane() {
    let mutation = <SourcingCurationApp as ArtifactEditor>::host_configuration_mutation("setContributions", Some(&protocol::DslValue::from(&serde_json::json!({ "json": "[{\"id\":\"sourcing\"}]" }))))
        .expect("host configuration")
        .expect("sourcing contribution mutation");
    assert_eq!(mutation, SourcingCurationConfigMutation::SetContributions { json: "[{\"id\":\"sourcing\"}]".into() });
    assert_eq!(<SourcingCurationApp as ArtifactEditor>::host_configuration_mutation("setFilterQuery", None).expect("non-host action"), None);
}

#[test]
fn retained_factories_declare_every_publication_lane() {
    let bounded = <SourcingCurationBoundedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS;
    assert_eq!(bounded.iter().map(|contract| contract.tool_id).collect::<Vec<_>>(), SOURCING_CURATION_BOUNDED_TOOL_IDS);
    let lane_of = |tool_id: &str| bounded.iter().find(|contract| contract.tool_id == tool_id).expect("declared tool").lanes;
    for tool_id in ["setActiveExample", "setDocument", "stockFromCatalogue"] {
        assert_eq!(lane_of(tool_id), [ArtifactToolPublicationLane::HostOnly], "{tool_id} replaces the whole document through an effect");
    }
    for tool_id in ["curationAdd", "curationSetCount", "curationRemove", "dropOnPool", "dropOnCurated"] {
        assert_eq!(lane_of(tool_id), [ArtifactToolPublicationLane::Artifact], "{tool_id} publishes a curated-selection mutation");
    }
    for tool_id in ["setFilterQuery", "setFilterModule", "setFilterTypology", "setFilterMinAvailability", "sortTable", "setContributions"] {
        assert_eq!(lane_of(tool_id), [ArtifactToolPublicationLane::Config], "{tool_id} publishes view state only");
    }
}

#[semio_framework_async_macros::async_test]
async fn sourcing_curation_io_declares_the_catalog_out_port_alongside_the_implicit_document_ports() {
    let io = sourcing_curation_io();
    assert_eq!(io.document_schema, SOURCING_CURATION_SCHEMA);
    let ports = io.all_ports().await;
    assert_eq!(ports.len(), 3, "document:in, document:out, catalog:out");
    let catalog_out = ports.iter().find(|port| port.id == "catalog:out").expect("catalog:out port declared");
    assert_eq!(catalog_out.kind_id.as_deref(), Some("kit.catalog"));
    assert_eq!(catalog_out.media_type.class, MediaClass::Kit);
    assert_eq!(catalog_out.media_type.form, MediaForm::Type);
}

#[semio_framework_async_macros::async_test]
async fn sourcing_curation_io_and_catalog_export_round_trip() {
    let mut app = new_app().await;
    let media = semio_framework_plugin::resolve_ready(app.export_media("catalog:out")).expect("catalog export");
    assert_eq!(media.media_type.class, MediaClass::Kit);
    assert_eq!(media.media_type.form, MediaForm::Type);
    match media.payload {
        MediaPayload::Structured { schema, json } => {
            assert_eq!(schema, "kit.catalog");
            let fragment: serde_json::Value = serde_json::from_str(&json).unwrap();
            assert_eq!(fragment["objectKinds"].as_array().unwrap().len(), app.snapshot().expect("snapshot").stock_extra.len());
        }
        MediaPayload::Binary { .. } => panic!("expected a Structured payload"),
    }
}

/// 🧵️ Every UI-reachable command is on the retained bounded lane and nowhere else. `setLocale` is
/// deliberately absent — it is `ForbiddenFromUi`, dispatched only through the host configuration
/// route, and it is the ONLY command of this app that is not a retained tool.
#[test]
fn retained_route_catalog_covers_every_ui_reachable_command() {
    let mut routes = SOURCING_CURATION_BOUNDED_TOOL_IDS.to_vec();
    routes.sort_unstable();
    routes.dedup();
    assert_eq!(routes.len(), SOURCING_CURATION_BOUNDED_TOOL_IDS.len(), "no route is declared twice");
    assert_eq!(routes.len(), 14);
    assert!(!SOURCING_CURATION_BOUNDED_TOOL_IDS.contains(&"setLocale"));
}

#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let json = serde_json::to_string(&create_sourcing_curation_app()).expect("app definition json");
    for id in [pool::SOURCING_CURATION_WINDOW_POOL, curated::SOURCING_CURATION_WINDOW_CURATED, preview::SOURCING_CURATION_WINDOW_PREVIEW, grid::SOURCING_CURATION_WINDOW_GRID] {
        assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
    }
    assert!(json.contains(edit::SOURCING_CURATION_MODE_CURATION), "mode missing from the manifest");
    assert!(json.contains("catalogue.sourcing"), "artifact kind missing from the manifest");
}
