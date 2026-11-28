use crate::editor_unit_tests::context;
use semio_framework_test_fixtures::fixture_text;

use semio_plugin::{ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactToolPublicationLane, MediaClass, MediaForm, MediaPayload};
use crate::{CurationSnapshot, SOURCING_CURATION_SCHEMA};
use crate::editor::sourcing::config::{SetContributionsEdit, ToggleFilterModuleEdit, SetFilterQueryEdit, SourcingCurationConfig, SourcingCurationConfigMutation};
use crate::editor::sourcing::modes::edit::windows::{curated, grid, pool, preview};
use crate::editor::sourcing::commands::{curation_add, curation_remove, curation_set_count, drop_on_curated, drop_on_pool, load_document_json, set_active_example, set_contributions, set_filter_min_availability, set_filter_module, set_filter_query, set_filter_typology, sort_table, stock_from_catalogue};
use crate::editor::sourcing::modes::edit;
use crate::editor::sourcing::{SOURCING_CURATION_BOUNDED_TOOL_IDS, SOURCING_CURATION_CONFIG_GRANT_BYTES, SOURCING_CURATION_CONFIG_STORE_MAXIMUM_BYTES, SOURCING_CURATION_CONFIG_STORE_MAXIMUM_ITEMS, SOURCING_CURATION_RETAINED_RAW_BYTES, SourcingCurationApp, SourcingCurationBoundedCommandJobFactory, SourcingCurationCommand, create_sourcing_curation_app, prepare_sourcing_curation_config, sourcing_curation_config_mutation_retained_bytes};
use crate::editor::sourcing::editor_vocabulary::{sourcing_curation_io, DEMO_STOCK_EXAMPLE_ID, EMPTY_EXAMPLE_ID, SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES, SOURCING_CURATION_CONFIG_TEXT_BYTES};
use semio_plugin::plugin_app_close_prelude::TypedOperationResultLane;
use crate::editor_unit_tests::context::{dispatch, new_app, sourcing_manifest_for_tests};
use semio_plugin::artifact_app_laws;
use semio_plugin::{EditorApp, PluginApp};

#[semio_framework_async_macros::async_test]
async fn view_kind_config_only_commands_pass_kind_discipline() {
    let mut app = new_app().await;
    let result = dispatch(&mut app, SourcingCurationCommand::SetFilterQuery(set_filter_query::SetFilterQuery { value: "glulam".into() })).await;
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
    assert_eq!(ids.len(), 14, "every SourcingCurationCommand row must be covered by every_command()");
}

/// 🌉️ LAW: the PRODUCTION action bridge covers every declared row. `ArtifactEditor`'s default
/// `command_from_action` rejects app-owned actions outright, so an app that declares actions but
/// never overrides it ships a UI whose every control faults — which is exactly how this pane's
/// `setActiveExample` (and with it the whole `demo-stock` example) was dead in the demonstrator.
/// Asserted through `<SourcingCurationApp as ArtifactEditor>` — NOT the free function — because the
/// trait method is the seam the host actually calls.
#[semio_framework_async_macros::async_test]
async fn the_production_action_bridge_admits_every_declared_command() {
    for command in every_command() {
        let action = command.command_id();
        let built = <SourcingCurationApp as ArtifactEditor>::command_from_action(action, &mut None, &mut None).unwrap_or_else(|fault| panic!("action '{action}' is declared but the production bridge rejects it: {fault:?}"));
        assert_eq!(built.command_id(), action, "the bridge routed '{action}' to the wrong command");
    }
}

/// 🎞️ LAW: the framework's own conformance harness — it walks the actions this app's window kinds
/// actually RENDER, stages each one's declared args exactly as the host does, and knows which ids
/// are framework-injected (`undo`/`copy`/`recordTutorial`/…) and must be skipped. Strictly stronger
/// than enumerating the command rows: it catches an action that chrome declares but no command row
/// backs.
#[semio_framework_async_macros::async_test]
async fn every_rendered_action_bridges_through_the_framework_harness() {
    context::assert_declared_actions_bridge_to_commands().await;
}

/// 📛️ LAW: the bridge reads the manifest's OWN arg names — `setActiveExample` declares a select
/// arg keyed `exampleId` (`🔖️Manifest`), and the payload field is `example_id`; the two
/// vocabularies are joined here and nowhere else.
#[semio_framework_async_macros::async_test]
async fn the_action_bridge_reads_the_declared_arg_names() {
    let built = <SourcingCurationApp as ArtifactEditor>::command_from_action("setActiveExample", &mut Some((semio_framework_value::DslValue::from(&serde_json::json!({ "exampleId": DEMO_STOCK_EXAMPLE_ID }))).clone()), &mut None).expect("setActiveExample must convert");
    assert_eq!(built, SourcingCurationCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: DEMO_STOCK_EXAMPLE_ID.into() }));
    let filter = <SourcingCurationApp as ArtifactEditor>::command_from_action("setFilterModule", &mut Some((semio_framework_value::DslValue::from(&serde_json::json!({ "moduleId": "walls", "enabled": true }))).clone()), &mut None).expect("setFilterModule must convert");
    assert_eq!(filter, SourcingCurationCommand::SetFilterModule(set_filter_module::SetFilterModule { module_id: "walls".into(), enabled: true }));
    assert!(<SourcingCurationApp as ArtifactEditor>::command_from_action("noSuchAction", &mut None, &mut None).is_err(), "an undeclared action must fault, not silently no-op");
}

/// 📚️ LAW: the `setActiveExample` form offers the empty example next to every registered example, so the form and the navbar selector reach the same documents.
#[semio_framework_async_macros::async_test]
async fn set_active_example_offers_the_empty_example_and_every_registered_example() {
    let definition = create_sourcing_curation_app();
    let action = definition.actions.iter().find(|action| action.id == "setActiveExample").expect("setActiveExample is declared");
    let arg = action.args.iter().find(|arg| arg.id == "exampleId").expect("exampleId is declared");
    let semio_plugin::ArgSchema::String { options, .. } = &arg.schema else { panic!("exampleId is a select: {:?}", arg.schema) };
    let offered: Vec<&str> = options.iter().map(|option| option.value.as_str()).collect();
    let mut expected = vec![EMPTY_EXAMPLE_ID];
    expected.extend(crate::editor::sourcing::editor_vocabulary::examples().iter().map(|example| example.id()));
    assert_eq!(offered, expected);
}

/// 🔲️ LAW (S15 matrix, 2026-09-25): the rail stages `curationSetCount`'s `delta`/`value` through the argument schema
/// the manifest declares, and the bridge reads them as numbers. Declared as text, the staged `"1"` read as no delta,
/// the command resolved to a no-op and the rail journaled a row that changed nothing. The count args are numeric, and
/// the numeric staging reaches the command as the delta the user entered.
#[semio_framework_async_macros::async_test]
async fn the_rail_stages_numeric_curation_counts_into_one_curated_edit() {
    let definition = create_sourcing_curation_app();
    let action = definition.actions.iter().find(|action| action.id == "curationSetCount").expect("curationSetCount is declared on the rail");
    for id in ["delta", "value"] {
        let arg = action.args.iter().find(|arg| arg.id == id).unwrap_or_else(|| panic!("curationSetCount declares {id}"));
        assert!(matches!(arg.schema, semio_plugin::ArgSchema::Number { .. }), "curationSetCount.{id} must be a number argument, found {:?}", arg.schema);
    }
    let built = <SourcingCurationApp as ArtifactEditor>::command_from_action("curationSetCount", &mut Some((semio_framework_value::DslValue::from(&serde_json::json!({ "objectId": "beam-glulam-gl24h", "delta": 1 }))).clone()), &mut None).expect("curationSetCount must convert");
    assert_eq!(built, SourcingCurationCommand::CurationSetCount(curation_set_count::CurationSetCount { object_id: "beam-glulam-gl24h".into(), delta: Some(1.0), value: None }));
}

/// 📝️ LAW: text and binary are two projections of the same command, for every single row — the
/// permanent successor of the old `📡️protocol` crate's
/// `sourcing_curation_command_op_text_round_trips_every_variant`.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        semio_os_store_test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// 🎫️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — copied
/// verbatim from each `app_commands!` row's `as "…"` literal (NOT a mechanical kebab-case of the
/// manifest action id: `loadDocumentJson`/`document-json`, `setActiveExample`/`active-example`, the whole
/// missing `#[dsl(keyword = ..)]` on a payload struct silently breaks (the record prints with no
/// keyword at all and no longer parses).
#[semio_framework_async_macros::async_test]
async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
    fn expected_wire_key(id: &str) -> &'static str {
        match id {
            "loadDocumentJson" => "load-document-json",
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

/// ➖️ The rows whose `Option` fields make `None`/`Some` distinct wire cases, pinned to the exact bytes
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
        semio_os_store_test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order — mirrors the
/// pre-migration wire baseline captured into this ticket's `wire-baseline-before.txt`.
fn every_command() -> Vec<SourcingCurationCommand> {
    vec![
        SourcingCurationCommand::LoadDocumentJson(load_document_json::LoadDocumentJson { json: "{}".into() }),
        SourcingCurationCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: DEMO_STOCK_EXAMPLE_ID.into() }),
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
        SourcingCurationCommand::SetContributions(set_contributions::SetContributions { json: "[]".into() }),
    ]
}
//#endregion 🔖️CommandSurface

#[semio_framework_async_macros::async_test]
async fn app_definition_labels_resolve_german() {
    let def = &create_sourcing_curation_app();
    let (terminology, locale) = (semio_ui_locale::Terminology::Native, semio_ui_locale::Locale::De);
    assert_eq!(def.window_kinds.iter().find(|entry| entry.id == pool::SOURCING_CURATION_WINDOW_POOL).expect("pool window").label.resolve(terminology, locale), "Pool");
    assert_eq!(def.window_kinds.iter().find(|entry| entry.id == curated::SOURCING_CURATION_WINDOW_CURATED).expect("curated window").label.resolve(terminology, locale), "Kuratiert");
    assert_eq!(def.modes.iter().find(|entry| entry.id == edit::SOURCING_CURATION_MODE_CURATION).expect("curation mode").label.resolve(terminology, locale), "Kuratierung");
}


/// 🏠️ One host `sourcing.module` contribution for a module this crate does NOT author.
fn sourcing_reuse_contribution() -> String {
    let kind = crate::ObjectKind { id: "reuse-salvaged-oak".into(), name: "Salvaged Oak Beam".into(), module_id: "reuse".into(), typology_path: vec!["reuse".into()], availability: 4, geometry: Box::new(crate::GeometryRecipe::Box { width: 0.2, height: 0.2, depth: 3.0 }) };
    let entry = semio_framework::ProgramContributionEntry {
        plugin_id: "sourcing-module-reuse".into(),
        topic_contribution: Some(semio_framework::TopicContribution::new(
            crate::schema::SOURCING_MODULE_TOPIC,
            semio_framework::DslValue::object([
                ("appId".to_string(), semio_framework::DslValue::String("sourcing-curation".to_string())),
                ("moduleId".to_string(), semio_framework::DslValue::String("reuse".to_string())),
                ("label".to_string(), semio_framework::DslValue::String("Reuse".to_string())),
                ("iconId".to_string(), semio_framework::DslValue::String("recycle".to_string())),
                ("typologyJson".to_string(), semio_framework::DslValue::String(semio_framework_pack_json::to_json_string(&crate::schema::TypologyNode::new("reuse", "Reuse", vec![])))),
                ("kindsJson".to_string(), semio_framework::DslValue::String(semio_framework_pack_json::to_json_string(&vec![kind]))),
            ]),
        )),
    };
    semio_framework_pack_json::to_json_string(&vec![entry])
}

#[test]
fn retained_factories_declare_every_publication_lane() {
    let bounded = <SourcingCurationBoundedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS;
    assert_eq!(bounded.iter().map(|contract| contract.tool_id).collect::<Vec<_>>(), SOURCING_CURATION_BOUNDED_TOOL_IDS);
    let lane_of = |tool_id: &str| bounded.iter().find(|contract| contract.tool_id == tool_id).expect("declared tool").lanes;
    for tool_id in ["setActiveExample", "loadDocumentJson", "stockFromCatalogue"] {
        assert_eq!(lane_of(tool_id), [ArtifactToolPublicationLane::HostOnly], "{tool_id} replaces the whole document through an effect");
    }
    for tool_id in ["curationAdd", "curationSetCount", "curationRemove", "dropOnPool", "dropOnCurated"] {
        assert_eq!(lane_of(tool_id), [ArtifactToolPublicationLane::Artifact], "{tool_id} publishes a curated-selection mutation");
    }
    for tool_id in ["setFilterQuery", "setFilterModule", "setFilterTypology", "setFilterMinAvailability", "sortTable", "setContributions"] {
        assert_eq!(lane_of(tool_id), [ArtifactToolPublicationLane::Config], "{tool_id} publishes view state only");
    }
    assert_eq!(lane_of("setGridInstanceDisplay"), [ArtifactToolPublicationLane::WindowConfig], "grid display mode is per-window config");
}

#[semio_framework_async_macros::async_test]
async fn sourcing_curation_io_declares_the_catalog_out_port_alongside_the_implicit_document_ports() {
    let io = sourcing_curation_io();
    assert_eq!(io.artifact_schema, SOURCING_CURATION_SCHEMA);
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
    let media = context::export_media(&mut app, "catalog:out").await;
    assert_eq!(media.media_type.class, MediaClass::Kit);
    assert_eq!(media.media_type.form, MediaForm::Type);
    match media.payload {
        MediaPayload::Structured { schema, json } => {
            assert_eq!(schema, "kit.catalog");
            let fragment: serde_json::Value = serde_json::from_str(&json).unwrap();
            assert_eq!(fragment["objectKinds"].as_array().unwrap().len(), app.snapshot().expect("snapshot").stock_extra.len());
        }
        MediaPayload::Binary { .. } | MediaPayload::Intrinsic { .. } => panic!("expected a Structured payload"),
    }
}

/// 🗄️ deliberately absent — it is `ForbiddenFromUi`, dispatched only through the host configuration
/// route, and it is the ONLY command of this app that is not a retained tool.
#[test]
fn retained_route_catalog_covers_every_ui_reachable_command() {
    let mut routes = SOURCING_CURATION_BOUNDED_TOOL_IDS.to_vec();
    routes.sort_unstable();
    routes.dedup();
    assert_eq!(routes.len(), SOURCING_CURATION_BOUNDED_TOOL_IDS.len(), "no route is declared twice");
    assert_eq!(routes.len(), 15);
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


#[test]
fn retained_table_lanes_restore_complete_neutral_owner() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🚚️retained-table/🔣️.json")).unwrap();
    let scene = semio_plugin::TableScene::base(fixture["columnsJson"].as_str().unwrap(), fixture["rowsJson"].as_str().unwrap());
    let node = semio_plugin::scene_surface("neutral-retained-table", semio_plugin::plugin_app_close_prelude::SurfaceKind::Table, &scene, "Retained table", None).unwrap();
    assert_eq!(node.children.len(), 2);
    assert!(serde_json::to_string(&node).unwrap_err().to_string().contains("BuiltChildren requires retained page transport"));
    let semio_plugin::Component::Surface(props) = &node.component else { panic!("typed table surface") };
    let spine: semio_plugin::TableScene = semio_ui_scene::decode(props).unwrap();
    assert_eq!(props.doc_schema.as_str(), fixture["surfaceSchema"].as_str().unwrap());
    assert!(spine.columns_json.is_empty() && spine.rows_json.is_empty());
    assert_eq!(spine.lanes.len(), fixture["lanes"].as_array().unwrap().len());
    for (actual, expected) in spine.lanes.iter().zip(fixture["lanes"].as_array().unwrap()) {
        assert_eq!(actual.lane, expected["name"].as_str().unwrap());
        assert_eq!(u64::from(actual.bytes), expected["bytes"].as_u64().unwrap());
        assert_eq!(actual.hash, expected["hash"].as_str().unwrap());
    }
    let restored = context::table_scene_of(&node).expect("complete retained table");
    assert_eq!(restored.columns_json, scene.columns_json);
    assert_eq!(restored.rows_json, scene.rows_json);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&restored.columns_json).unwrap(), fixture["columns"]);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&restored.rows_json).unwrap(), fixture["rows"]);
}

/// 🖱️ A pick in the framework-owned `rows` domain owes a repaint to every window body that reads that selection: the Preview follows the pick, so its body is in the declared refresh scope, never left on its previous render until an unrelated edit dirties it.
#[test]
fn a_rows_pick_dirties_every_window_that_reads_the_rows_selection() {
    use semio_plugin::{interaction_declared_refresh_scope, interaction_window_bodies_by_domain, panel_leaf_body_keys, InteractionVerb};
    use semio_plugin::plugin_app_close_prelude::UiDirtyScope;
    let definition = crate::editor::sourcing::create_sourcing_curation_app();
    let by_domain = interaction_window_bodies_by_domain(&definition);
    let panels = panel_leaf_body_keys(&definition);
    let Some(UiDirtyScope::Partial { window_bodies, .. }) = interaction_declared_refresh_scope(InteractionVerb::Select, &["rows"], &by_domain, &panels) else { panic!("a rows pick answers a partial scope") };
    for window in &definition.window_kinds {
        assert!(window_bodies.contains(&window.body_key), "window body {} reads the rows selection but a rows pick would not repaint it", window.body_key);
    }
}
