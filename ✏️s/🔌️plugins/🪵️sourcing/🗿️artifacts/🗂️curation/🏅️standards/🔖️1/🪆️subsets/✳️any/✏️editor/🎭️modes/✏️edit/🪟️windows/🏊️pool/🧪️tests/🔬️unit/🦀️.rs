
use super::*;
use crate::editor::sourcing::SourcingCurationCommand;
use crate::editor::sourcing::commands::{curation_add, set_contributions, set_filter_query, sort_table, stock_from_catalogue};
use crate::editor::sourcing::modes::edit::windows::curated::SOURCING_CURATION_BODY_CURATED;
use crate::editor::sourcing::unit_tests::context::{dispatch, new_app, row_ids, settle, table_of};
use crate::{Filters, SortDirection, TableSort};

/// 🧩️ One fake host `sourcing.module` contribution carrying a single salvaged-oak kind under a
/// module id NO authored module serves — the exact `ProgramContributionEntry` envelope
/// `setContributions` receives from the demonstrator closure.
fn fake_contribution() -> String {
    let kind = crate::ObjectKind { id: CONTRIBUTED_KIND_ID.into(), name: "Salvaged Oak Beam".into(), module_id: "salvage".into(), typology_path: vec!["salvage".into()], availability: 4, geometry: Box::new(crate::GeometryRecipe::Box { width: 0.2, height: 0.2, depth: 3.0 }) };
    let entry = semio_framework::ProgramContributionEntry {
        plugin_id: "sourcing-module-salvage".into(),
        topic_contribution: Some(semio_framework::TopicContribution::new(
            "sourcing.module",
            semio_framework::DslValue::object([
                ("appId".to_string(), semio_framework::DslValue::String("sourcing-curation".to_string())),
                ("moduleId".to_string(), semio_framework::DslValue::String("salvage".to_string())),
                ("label".to_string(), semio_framework::DslValue::String("Salvage".to_string())),
                ("iconId".to_string(), semio_framework::DslValue::String("recycle".to_string())),
                ("typologyJson".to_string(), semio_framework::DslValue::String(semio_framework_os_kernel::json::to_json_string(&crate::schema::TypologyNode::new("salvage", "Salvage", vec![])))),
                ("kindsJson".to_string(), semio_framework::DslValue::String(semio_framework_os_kernel::json::to_json_string(&vec![kind]))),
            ]),
        )),
    };
    dsl::json::to_json_string(&vec![entry])
}

const CONTRIBUTED_KIND_ID: &str = "salvage-salvaged-oak";

/// 🧩️ One pack per AUTHORED module, exactly as the `sourcing-module-*` extension crates emit theirs:
/// `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/*/🦀️.rs` builds this payload from the schema's own
/// module, so this is the real host pack the demonstrator closure pushes, not a toy.
fn demonstrator_contributions() -> String {
    let entries: Vec<semio_framework::ProgramContributionEntry> = crate::schema::available_modules("[]")
        .into_iter()
        .map(|module| semio_framework::ProgramContributionEntry {
            plugin_id: format!("sourcing-module-{}", module.module_id),
            topic_contribution: Some(semio_framework::TopicContribution::new(
                "sourcing.module",
                semio_framework::DslValue::object([
                    ("appId".to_string(), semio_framework::DslValue::String("sourcing-curation".to_string())),
                    ("moduleId".to_string(), semio_framework::DslValue::String(module.module_id.clone())),
                    ("label".to_string(), semio_framework::DslValue::String(module.label.clone())),
                    ("iconId".to_string(), semio_framework::DslValue::String("beam".to_string())),
                    ("typologyJson".to_string(), semio_framework::DslValue::String(semio_framework_os_kernel::json::to_json_string(&module.typology))),
                    ("kindsJson".to_string(), semio_framework::DslValue::String(semio_framework_os_kernel::json::to_json_string(&module.kinds))),
                ]),
            )),
        })
        .collect();
    dsl::json::to_json_string(&entries)
}

#[semio_framework_async_macros::async_test]
async fn pool_render_respects_query_filter() {
    let document = crate::schema::default_document();
    let cfg = SourcingCurationConfig { filters: Filters { query: "glulam".into(), ..Default::default() }, ..Default::default() };
    let names: Vec<String> = pool_kinds(&document, &cfg).into_iter().map(|kind| kind.name).collect();
    assert!(names.iter().any(|name| name.contains("Glulam")));
    assert!(!names.iter().any(|name| name.contains("Hollow Core")));
}

#[semio_framework_async_macros::async_test]
async fn pool_row_carries_the_drag_payload_and_a_stepper_bounded_by_availability() {
    let document = crate::schema::default_document();
    let kind = crate::stock_of(&document).remove(0);
    let labels = crate::editor::sourcing::terminology::sourcing_curation_labels(&semio_framework_plugin::ViewModel::default());
    let row: serde_json::Value = serde_json::from_str(&protocol::json::to_json_string(&pool_row(&document, &kind, labels))).unwrap();
    assert_eq!(row["id"], kind.id.as_str());
    assert_eq!(row["_drag"]["objectId"], kind.id.as_str());
    assert_eq!(row["curated"]["kind"], "stepper");
    assert_eq!(row["curated"]["max"].as_f64().unwrap(), kind.availability as f64);
    assert_eq!(row["curated"]["action"]["action"], "curationSetCount");
    assert_eq!(row["curated"]["action"]["args"]["objectId"], kind.id.as_str());
}

/// 🧺️ LAW: the pool row exposes curated count only through the bounded stepper — no duplicate row buttons.
#[semio_framework_async_macros::async_test]
async fn pool_row_has_no_actions_column_and_only_a_curated_stepper() {
    let mut document = crate::schema::default_document();
    let kind = crate::stock_of(&document).remove(0);
    let labels = crate::editor::sourcing::terminology::sourcing_curation_labels(&semio_framework_plugin::ViewModel::default());
    let uncurated: serde_json::Value = serde_json::from_str(&protocol::json::to_json_string(&pool_row(&document, &kind, labels))).unwrap();
    assert!(uncurated.get("actions").is_none(), "the pool must not render a second +/- column");
    assert_eq!(uncurated["curated"]["kind"], "stepper");
    crate::schema::curation_delta(&mut document, &kind.id, 1);
    let curated: serde_json::Value = serde_json::from_str(&protocol::json::to_json_string(&pool_row(&document, &kind, labels))).unwrap();
    assert_eq!(curated["curated"]["value"].as_f64().unwrap(), 1.0);
}

/// ↕️ LAW: `Filters::sort` is shared with the curated table, so a sort naming a column the pool does
/// not have must leave the pool in document order rather than silently re-sorting it by name.
#[semio_framework_async_macros::async_test]
async fn pool_sort_applies_only_to_its_own_columns() {
    let document = crate::schema::default_document();
    let unsorted: Vec<String> = pool_kinds(&document, &SourcingCurationConfig::default()).into_iter().map(|kind| kind.id).collect();
    let foreign = SourcingCurationConfig { filters: Filters { sort: Some(TableSort { column_id: "count".into(), direction: SortDirection::Asc }), ..Default::default() }, ..Default::default() };
    assert_eq!(pool_kinds(&document, &foreign).into_iter().map(|kind| kind.id).collect::<Vec<_>>(), unsorted, "the curated table's own column must not reorder the pool");
    let descending = SourcingCurationConfig { filters: Filters { sort: Some(TableSort { column_id: "name".into(), direction: SortDirection::Desc }), ..Default::default() }, ..Default::default() };
    let names: Vec<String> = pool_kinds(&document, &descending).into_iter().map(|kind| kind.name).collect();
    let mut expected = names.clone();
    expected.sort_by(|a, b| b.cmp(a));
    assert_eq!(names, expected);
}

#[semio_framework_async_macros::async_test]
async fn pool_scene_names_columns_by_id_and_drops_onto_the_pool() {
    let document = crate::schema::default_document();
    let node = render(&document, &SourcingCurationConfig::default(), crate::editor::sourcing::terminology::sourcing_curation_labels(&semio_framework_plugin::ViewModel::default())).expect("bounded pool");
    assert_eq!(node.children.len(), 2, "filter row above the table");
    let semio_framework_plugin::Component::Surface(props) = &node.children.get(1).expect("table surface").component else { panic!("expected a table surface") };
    let scene: semio_framework_plugin::TableScene = semio_framework_ui_scene::decode(props).expect("table scene");
    assert!(node.children.get(0).expect("filter row").children.len() >= 6, "query, three modules, typology, availability and restock controls");
    let columns: serde_json::Value = serde_json::from_str(&scene.columns_json).unwrap();
    assert_eq!(columns[0]["id"], "name");
    assert_eq!(columns[0]["sortable"], true, "the name header is clickable to sort");
    assert_eq!(columns[4]["id"], "curated");
    assert_eq!(columns.as_array().expect("column records").len(), 5, "pool columns end at curated — no duplicate actions column");
    assert_eq!(scene.row_drag_mime.as_deref(), Some(crate::editor::sourcing::SOURCING_DRAG_MIME));
    assert!(scene.drop_action_json.expect("drop action").contains("dropOnPool"));
}

/// 🔍️ LAW: every filter control in the bar names a real command with the args the production bridge reads.
#[semio_framework_async_macros::async_test]
async fn the_filter_bar_binds_each_control_to_its_own_command() {
    let labels = crate::editor::sourcing::terminology::sourcing_curation_labels(&semio_framework_plugin::ViewModel::default());
    let bar = filter_bar(&SourcingCurationConfig::default(), labels).expect("bounded filter bar");
    let ids: Vec<String> = bar.children.iter().map(|child| child.key.as_str().to_string()).collect();
    for expected in ["sourcing-filter-query", "sourcing-filter-typology", "sourcing-filter-min-availability", "sourcing-pool-restock"] {
        assert!(ids.iter().any(|id| id == expected), "{expected} missing from {ids:?}");
    }
    assert!(ids.iter().any(|id| id.starts_with("sourcing-filter-module-")), "one toggle per available module in {ids:?}");
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_table_surface_and_body_key() {
    let def = definition();
    assert_eq!(def.body_key, SOURCING_CURATION_BODY_POOL);
    assert!(matches!(def.surface_kind, SurfaceKind::Table));
}

#[semio_framework_async_macros::async_test]
async fn renders_pool_table_scene() {
    let mut app = new_app().await;
    let rendered = semio_framework_plugin::PluginApp::render(&mut *app, SOURCING_CURATION_BODY_POOL, None, &semio_framework_plugin::ViewModel::default()).await.expect("render");
    assert!(rendered.root.children.iter().any(|child| matches!(child.component, semio_framework_plugin::Component::Surface(_))));
}

/// ⚖️ LAW: dispatching the search box's own command through the live app narrows the rendered pool.
#[semio_framework_async_macros::async_test]
async fn dispatching_the_search_box_narrows_the_rendered_pool() {
    let mut app = new_app().await;
    let before = row_ids(&mut app, SOURCING_CURATION_BODY_POOL).await;
    assert!(before.len() > 1, "the demo stock seeds several pool rows");
    dispatch(&mut app, SourcingCurationCommand::SetFilterQuery(set_filter_query::SetFilterQuery { value: "glulam".into() })).await;
    let after = row_ids(&mut app, SOURCING_CURATION_BODY_POOL).await;
    assert!(after.len() < before.len(), "the query must narrow {before:?} but rendered {after:?}");
    assert!(!after.is_empty(), "'glulam' still matches at least one kind");
    assert!(after.iter().all(|id| before.contains(id)));
}

/// ⚖️ LAW: clicking a sortable header dispatches `sortTable`, and the next render is reordered.
#[semio_framework_async_macros::async_test]
async fn dispatching_a_column_header_reorders_the_rendered_pool() {
    let mut app = new_app().await;
    let ascending_by_document = row_ids(&mut app, SOURCING_CURATION_BODY_POOL).await;
    dispatch(&mut app, SourcingCurationCommand::SortTable(sort_table::SortTable { column_id: "name".into(), direction: "desc".into() })).await;
    let descending = row_ids(&mut app, SOURCING_CURATION_BODY_POOL).await;
    assert_ne!(descending, ascending_by_document, "a descending name sort must reorder the pool");
    let mut sorted = descending.clone();
    sorted.sort();
    let mut roundtrip = ascending_by_document.clone();
    roundtrip.sort();
    assert_eq!(sorted, roundtrip, "sorting reorders rows, it never adds or drops any");
}

/// ⚖️ LAW: changing curated count through `curationAdd` updates the pool stepper and the curated table.
#[semio_framework_async_macros::async_test]
async fn curation_add_updates_the_pool_stepper_and_curated_table() {
    let mut app = new_app().await;
    let object_id = row_ids(&mut app, SOURCING_CURATION_BODY_POOL).await.remove(0);
    assert!(row_ids(&mut app, SOURCING_CURATION_BODY_CURATED).await.is_empty(), "the demo stock starts uncurated");
    dispatch(&mut app, SourcingCurationCommand::CurationAdd(curation_add::CurationAdd { object_id: object_id.clone() })).await;
    assert_eq!(row_ids(&mut app, SOURCING_CURATION_BODY_CURATED).await, vec![object_id.clone()]);
    let pool = table_of(&mut app, SOURCING_CURATION_BODY_POOL).await.1;
    let row = pool.iter().find(|row| row["id"] == object_id.as_str()).expect("the curated kind stays in the pool");
    assert_eq!(row["curated"]["value"].as_f64().unwrap(), 1.0);
    assert!(row.get("actions").is_none(), "the pool stepper is the sole curated control");
}

/// ⚖️ LAW: a host `sourcing.module` contribution reaches the pool — immediately as a module filter
/// toggle (session config), and as real pool ROWS once the restock button's `stockFromCatalogue`
/// merges it into the document. `stock` is VCS'd document content while contributions are session
/// config, so the restock step is required; this drives the PRODUCTION handler over a config that
/// already carries the contribution, which is what the host hands it.
#[semio_framework_async_macros::async_test]
async fn a_contributed_module_reaches_the_filter_bar_and_then_the_pool_rows() {
    let labels = crate::editor::sourcing::terminology::sourcing_curation_labels(&semio_framework_plugin::ViewModel::default());
    let cfg = SourcingCurationConfig { contributions_json: fake_contribution(), ..Default::default() };
    let toggles: Vec<String> = filter_bar(&cfg, labels).expect("bounded filter bar").children.iter().map(|child| child.key.as_str().to_string()).collect();
    assert!(toggles.iter().any(|id| id == "sourcing-filter-module-salvage"), "the contributed module gains its own filter toggle: {toggles:?}");
    let document = crate::schema::default_document();
    let before: Vec<String> = pool_kinds(&document, &SourcingCurationConfig::default()).into_iter().map(|kind| kind.id).collect();
    assert!(!before.iter().any(|id| id == CONTRIBUTED_KIND_ID));
    let history = semio_framework_plugin::HistoryView::empty();
    let emit = stock_from_catalogue::handle(&stock_from_catalogue::StockFromCatalogue {}, &semio_framework_plugin::ArtifactView::new(&document, &history), &semio_framework_plugin::ConfigView { snapshot: &cfg, window: None }).expect("restock");
    let restocked = emit
        .effects
        .into_iter()
        .find_map(|effect| match effect {
            semio_framework::kernel::Effect::LoadDocument { pack, .. } => Some(<CurationSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("restocked document")),
            _ => None,
        })
        .expect("restocking publishes a document load");
    let rows: Vec<String> = pool_kinds(&restocked, &cfg).into_iter().map(|kind| kind.id).collect();
    assert!(rows.iter().any(|id| id == CONTRIBUTED_KIND_ID), "the contributed kind must be a pool row after restocking: {rows:?}");
    assert!(before.iter().all(|id| rows.contains(id)), "restocking merges, it never drops the authored stock");
}

/// ⚖️ LAW: the REAL demonstrator pack — one topic contribution per authored module in a single
/// `ProgramContributionEntry[]` — crosses the LIVE `setContributions` lane and installs exactly the
/// modules it can act on. It is far past the 96-byte filter-text envelope that used to price (and
/// silently refuse) it. Because every entry re-contributes a module this crate already authors, the
/// installable share is empty: the app keeps the roster it can act on, never the host's whole pack,
/// and the module list stays at the authored four rather than doubling.
#[semio_framework_async_macros::async_test]
async fn the_real_sourcing_module_pack_installs_through_the_live_contributions_lane() {
    let json = demonstrator_contributions();
    assert!(json.len() > crate::editor::sourcing::component::SOURCING_CURATION_CONFIG_TEXT_BYTES, "the real pack is {} bytes — past the {}-byte filter-text envelope", json.len(), crate::editor::sourcing::component::SOURCING_CURATION_CONFIG_TEXT_BYTES);
    let installable = crate::schema::installable_contributions(&json, crate::editor::sourcing::component::SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES);
    assert_eq!(installable, "[]", "the shipped extensions re-contribute authored modules, so nothing new is installable");
    let installed: Vec<String> = crate::schema::available_modules(&json).into_iter().map(|module| module.module_id).collect();
    assert_eq!(installed, vec!["beams".to_string(), "windows".to_string(), "slabs".to_string(), "reuse".to_string()], "a re-contributed module never duplicates the authored one");
    let mutation = crate::editor::sourcing::config::SourcingCurationConfigMutation::SetContributions { json: installable };
    assert!(crate::editor::sourcing::component::sourcing_curation_config_mutation_footprint(&mutation).is_ok(), "the retained config preparation must admit the installable roster");
    let mut app = new_app().await;
    dispatch(&mut app, SourcingCurationCommand::SetContributions(set_contributions::SetContributions { json })).await;
    // 🧺️ EXACTLY the authored stock — asserted against the authored demo document itself rather
    // than a copied row count, so adding an authored module (`reuse` was the last one) can never
    // leave this law asserting a stale number again.
    let authored: Vec<String> = crate::schema::filtered_stock(&crate::schema::default_document(), &Filters::default()).into_iter().map(|kind| kind.id).collect();
    assert_eq!(row_ids(&mut app, SOURCING_CURATION_BODY_POOL).await, authored, "the pool still renders every authored kind, and only those, after the real pack crosses");
}

/// ⚖️ LAW: a module the app does NOT already author installs through the LIVE `setContributions`
/// dispatch and its kinds reach the Pool. The restock handler reads `cfg.snapshot.contributions_json`
/// off the retained config store, so a contributed kind in the published catalogue is proof that the
/// store really kept the pushed roster — the whole path the demonstrator depends on.
#[semio_framework_async_macros::async_test]
async fn dispatching_set_contributions_installs_the_module_into_the_live_catalogue() {
    let installable = crate::schema::installable_contributions(&fake_contribution(), crate::editor::sourcing::component::SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES);
    assert!(installable.contains("\"salvage\""), "a module id no authored module serves is installable: {installable}");
    assert!(installable.len() <= crate::editor::sourcing::component::SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES, "the installable roster fits its retained lane");
    let mut app = new_app().await;
    let before = row_ids(&mut app, SOURCING_CURATION_BODY_POOL).await;
    assert!(!before.iter().any(|id| id == CONTRIBUTED_KIND_ID), "the contributed kind is absent before the push");
    dispatch(&mut app, SourcingCurationCommand::SetContributions(set_contributions::SetContributions { json: fake_contribution() })).await;
    app.dispatch_typed(SourcingCurationCommand::StockFromCatalogue(stock_from_catalogue::StockFromCatalogue {}), &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("restock dispatch");
    let restocked = settle(&mut app)
        .await
        .into_iter()
        .find_map(|effect| match effect {
            semio_framework::kernel::Effect::LoadDocument { pack, .. } => Some(<CurationSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("restocked document")),
            _ => None,
        })
        .expect("restocking publishes a document load");
    let rows: Vec<String> = pool_kinds(&restocked, &SourcingCurationConfig::default()).into_iter().map(|kind| kind.id).collect();
    assert!(rows.iter().any(|id| id == CONTRIBUTED_KIND_ID), "the live retained config must carry the pushed pack: {rows:?}");
    assert!(before.iter().all(|id| rows.contains(id)), "installing a module merges, it never drops the authored stock");
}

/// ⚖️ LAW: dispatching the restock button's command through the live app publishes a document load
/// carrying the merged catalogue — the built-in modules, since this app starts with an empty
/// contributions lane.
#[semio_framework_async_macros::async_test]
async fn the_restock_button_publishes_a_merged_catalogue_document() {
    let mut app = new_app().await;
    let before = row_ids(&mut app, SOURCING_CURATION_BODY_POOL).await;
    app.dispatch_typed(SourcingCurationCommand::StockFromCatalogue(stock_from_catalogue::StockFromCatalogue {}), &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("restock dispatch");
    let restocked = settle(&mut app)
        .await
        .into_iter()
        .find_map(|effect| match effect {
            semio_framework::kernel::Effect::LoadDocument { pack, .. } => Some(<CurationSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("restocked document")),
            _ => None,
        })
        .expect("restocking publishes a document load");
    let rows: Vec<String> = pool_kinds(&restocked, &SourcingCurationConfig::default()).into_iter().map(|kind| kind.id).collect();
    assert_eq!(rows, before, "restocking the authored demo stock is idempotent");
}
