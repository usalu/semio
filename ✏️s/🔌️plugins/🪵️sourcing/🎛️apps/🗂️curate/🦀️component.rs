//! 🛒️ Sourcing curate app — the `ArtifactApp` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/🗂️curate/🪟️windows/*`, labels in `🦀️terminology.rs`, view state in `🦀️config.rs`, shared
//! compute in the artifact's `⚙️engine`. This file is a routing table: `handle` → `SourcingCurateCommand::
//! dispatch`, `render` → body-key → node, and a `🔖️Manifest` region that calls one `definition()` per node.

use crate::apps::curate::config::{SourcingCurateConfig, SourcingCurateConfigMutation};
use crate::apps::curate::presence::{SourcingCuratePresence, SourcingCuratePresenceMutation};
use crate::apps::curate::modes::curate;
use crate::apps::curate::modes::curate::windows::{curated, grid, pool, preview};
use crate::apps::curate::terminology::sourcing_curate_labels;
use crate::artifacts::curate::op::SourcingMutation;
use crate::artifacts::curate::{CurateSnapshot, SOURCING_CURATE_SCHEMA};
use semio_framework_plugin::{NoDraft, NoDraftMutation, DraftView, 
    ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, App, ArtifactKindSpec, ConfigView, ArtifactApp, ArtifactView, Emit, Fault, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType,
    OsMediaCapability, UiNode,
};
use store::EngineHandles;
use store::ArtifactPack;

//#region 🔖️Constants
pub const SOURCING_CURATE_APP_ID: &str = "sourcing-curate";
pub const SOURCING_CONTROLLER_ID: &str = "sourcing-curate";
pub const SOURCING_DRAG_MIME: &str = "application/x-semio-sourcing-object";
pub const DEMO_STOCK_EXAMPLE_ID: &str = "demo-stock";
pub const EMPTY_EXAMPLE_ID: &str = "empty-curation";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// builds its `on_change`/drop actions with.
pub fn sourcing_action(action: &str, args: Option<serde_json::Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(SOURCING_CONTROLLER_ID).action(action, args)
}
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `SourcingCurateApp::Command` — the SOLE dispatch surface for curate's own behavior, assembled
    /// from the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id
    /// (`command_id()`) and the `dsl` wire keyword (the kebab-case `#[dsl(key = ..)]` the codec uses) —
    /// they are genuinely different vocabularies, and `setLocale`/`locale` is the row that proves it.
    /// **Row order is the binary variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum SourcingCurateCommand for CurateSnapshot, SourcingMutation, SourcingCurateConfig, SourcingCurateConfigMutation {
        "setDocument" as "document-json" => set_artifact_json::SetArtifactJson,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "stockFromCatalogue" as "stock-from-catalogue" => stock_from_catalogue::StockFromCatalogue,
        "curateAdd" as "curate-add" => curate_add::CurateAdd,
        "curateSetCount" as "curate-set-count" => curate_set_count::CurateSetCount,
        "curateRemove" as "curate-remove" => curate_remove::CurateRemove,
        "dropOnPool" as "drop-on-pool" => drop_on_pool::DropOnPool,
        "dropOnCurated" as "drop-on-curated" => drop_on_curated::DropOnCurated,
        "setFilterQuery" as "filter-query" => set_filter_query::SetFilterQuery,
        "setFilterModule" as "filter-module" => set_filter_module::SetFilterModule,
        "setFilterTypology" as "filter-typology" => set_filter_typology::SetFilterTypology,
        "setFilterMinAvailability" as "filter-min-availability" => set_filter_min_availability::SetFilterMinAvailability,
        "sortTable" as "sort-table" => sort_table::SortTable,
        "selectRow" as "select-row" => select_row::SelectRow,
        "worldSelect" as "world-select" => world_select::WorldSelect,
        "setLocale" as "locale" => set_locale::SetLocale,
        "setContributions" as "contributions" => set_contributions::SetContributions,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use crate::apps::curate::commands::curation::{curate_add, curate_remove, curate_set_count, drop_on_curated, drop_on_pool};
use crate::apps::curate::commands::contribution::set_contributions;
use crate::apps::curate::commands::document::{set_active_example, set_artifact_json, stock_from_catalogue};
use crate::apps::curate::commands::filter::{set_filter_min_availability, set_filter_module, set_filter_query, set_filter_typology, sort_table};
use crate::apps::curate::commands::locale::set_locale;
use crate::apps::curate::commands::selection::{select_row, world_select};
//#endregion 🔖️Commands

//#region 🔖️SourcingCurateApp
/// 🧪️ Unit struct — every former app-struct field lives in `crate::apps::curate::config::
/// SourcingCurateConfig`, written through `SourcingCurateConfigMutation`s.
#[derive(Default)]
pub struct SourcingCurateApp;

impl ArtifactApp for SourcingCurateApp {
    type Snapshot = CurateSnapshot;
    type Mutation = SourcingMutation;
    type Config = SourcingCurateConfig;
    type ConfigMutation = SourcingCurateConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = SourcingCuratePresence;
    type PresenceMutation = SourcingCuratePresenceMutation;

    type Command = SourcingCurateCommand;

    const APP_ID: &'static str = SOURCING_CURATE_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = SOURCING_CURATE_SCHEMA;

    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::apps::curate::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> CurateSnapshot {
        crate::artifacts::curate::engine::default_document()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(crate::artifacts::curate::engine::sourcing_curate_io())
    }

    /// 🎞️ `catalog:out` (see `crate::artifacts::curate::engine::sourcing_catalog_fragment`) plus the
    /// inherited `document:out` default (the pack of `doc.snapshot`, replicated inline — overriding
    /// `export_media` shadows the trait's provided body for every port on this app, not just the new one).
    fn export_media(port: &str, doc: &ArtifactView<'_, CurateSnapshot>) -> Result<Media, MediaError> {
        match port {
            "catalog:out" => Ok(Media {
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                payload: MediaPayload::Structured { schema: "kit.catalog".into(), json: crate::artifacts::curate::engine::sourcing_catalog_fragment(doc.snapshot).to_string() },
            }),
            "document:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.document_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🧬️ Whole-document replace is banned from the `Mutation` enum outright (the former whole-
    /// snapshot-replace variant — see `📓️taxonomy.md`'s forbidden vocabulary), so this app does NOT
    /// override `whole_document_operation`
    /// (stays at the trait's own `None` default) and instead overrides `import_media` below to build a
    /// `HostEffect::LoadDocument` via `reset_document_effect`, outside undo history.
    fn import_media(port: &str, media: &Media, _doc: &ArtifactView<'_, CurateSnapshot>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation, Self::DraftMutation>, MediaError> {
        if port != "document:in" {
            return Err(MediaError::NotImplemented);
        }
        let MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "document:in importer only accepts a Structured (base64 pack) payload".into()));
        };
        let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let snapshot = <CurateSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        Ok(Emit { effects: vec![reset_document_effect(&snapshot)], ..Default::default() })
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`. `setLocale` has no manifest declaration (host-pushed,
    /// not a user-facing action).
    fn command_id(command: &SourcingCurateCommand) -> &'static str {
        command.command_id()
    }

    fn handle(command: &SourcingCurateCommand, doc: &ArtifactView<'_, CurateSnapshot>, cfg: &ConfigView<'_, SourcingCurateConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, CurateSnapshot>, cfg: &ConfigView<'_, SourcingCurateConfig>) -> UiNode {
        crate::artifacts::curate::engine::sync_sourcing_module_contributions(&cfg.snapshot.contributions_json);
        let snapshot = doc.snapshot;
        let config = cfg.snapshot;
        let labels = sourcing_curate_labels(config);
        match body_key {
            pool::SOURCING_CURATE_BODY_POOL => pool::render(snapshot, config, labels),
            curated::SOURCING_CURATE_BODY_CURATED => curated::render(snapshot, config, labels),
            preview::SOURCING_CURATE_BODY_PREVIEW => preview::render(snapshot, config, labels),
            grid::SOURCING_CURATE_BODY_GRID => grid::render(snapshot, config),
            _ => semio_framework_plugin::ui_text(Label::data("")),
        }
    }
}
//#endregion 🔖️SourcingCurateApp

//#region 🔖️ResetDocument
/// 🌱️ Builds a `HostEffect::LoadDocument` that swaps the live document to `document` OUTSIDE undo
/// history — the sanctioned non-mutation path for a whole-document replace (JSON import, load-
/// example, bulk catalogue restock). Per `📓️taxonomy.md`, the former whole-snapshot-replace variant
/// is banned outright with NO replacement mutation: whole-document replace is not expressible as an in-history `Mutation` at
/// all. Every former "replace the whole document" gesture in this app (`import_media`'s
/// `"document:in"` above, `commands::document::{set_active_example, set_artifact_json,
/// stock_from_catalogue}`) builds this effect instead of an `Emit::mutations([...])`. The spr is a
/// fresh, edit-free op-log for `document` — a genesis envelope with no history to encode.
pub fn reset_document_effect(document: &CurateSnapshot) -> semio_framework::kernel::HostEffect {
    let pack = <CurateSnapshot as store::ArtifactPack>::encode_pack(document);
    let envelope = store::create_document_envelope::<CurateSnapshot, SourcingMutation>(SOURCING_CURATE_SCHEMA, "curate", document.clone(), None);
    let spr = store::print_document_spr(&envelope).expect("curate document spr encode is infallible for a fresh, edit-free envelope");
    semio_framework::kernel::HostEffect::LoadDocument { pack, spr }
}
//#endregion 🔖️ResetDocument

//#region 🔖️Manifest
/// 🙈️ An internal document operation kept out of the command palette — the curate/DnD arms that mutate
/// the persisted `CurateSnapshot` but are only ever dispatched from window chrome.
fn hidden_operation(id: &str, label: impl Into<LocalizedLabel>) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog(id, label, ActionKind::Mutation) }
}

/// 🙈️👁️ The filter/sort/selection/world-pick arms emit ONLY `config_mutations`, so (unlike
/// `hidden_operation` above) they're declared `ActionKind::View`, letting `VcsArtifactApp`'s
/// kind-discipline check actually enforce "must not emit document operations".
fn hidden_view_action(id: &str, label: impl Into<LocalizedLabel>) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog(id, label, ActionKind::View) }
}

/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
pub fn create_sourcing_curate_app() -> App {
    App::from_builder(
        App::builder(SOURCING_CURATE_APP_ID, LocalizedLabel::native("Curate", "Kuratieren"))
            .document(["semio", "sourcing", "curate"])
            .artifact_kind(crate::artifacts::curate::artifact_kind())
            .artifact_kind(ArtifactKindSpec {
                id: "catalogue.kinds".into(),
                name: "Kind Catalogue".into(),
                source_format: "catalogue.kinds".into(),
                component_kind: "catalogue".into(),
                dimension: "data".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                schema: "catalogue.kinds".into(),
                export_formats: vec![],
                import_formats: vec![],
                    export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    })
            // 🔌️ WORKFLOWS-END-TO-END-TYPED-PORTS port recipe: the `catalog:out` port's declared kind —
            // harmless duplicate `ArtifactKindSpec` across producers (see `s/plugin/block`'s `3d` app,
            // which declares the SAME `kit.catalog` shape independently).
            .artifact_kind(ArtifactKindSpec {
                id: "kit.catalog".into(),
                name: "Kit Catalogue".into(),
                source_format: "kit.catalog".into(),
                component_kind: "catalogue".into(),
                dimension: "data".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                schema: "kit.catalog".into(),
                export_formats: vec![],
                import_formats: vec![],
                    export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    })
            .icon_id("library")
            .mode_def(curate::definition())
            .default_mode_id(curate::SOURCING_CURATE_MODE_CURATE)
            .window_kind_def(pool::definition())
            .window_kind_def(curated::definition())
            .window_kind_def(preview::definition())
            .window_kind_def(grid::definition())
            .default_layout(curate::layout())
            // 🔧️ Curation counts/stock edits are persisted in `CurateSnapshot`, so each arm emits a
            // whole-document `SetArtifact` operation and is declared as a Mutation, never a View.
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .mutation("stockFromCatalogue", LocalizedLabel::native("Stock From Catalogue", "Bestand aus Katalog"))
            .action_with(hidden_operation("setDocument", LocalizedLabel::native("Set Document", "Dokument festlegen")))
            .action_with(hidden_operation("curateAdd", LocalizedLabel::native("Curate Add", "Kuratierung hinzufügen")))
            .action_with(hidden_operation("curateSetCount", LocalizedLabel::native("Curate Set Count", "Kuratierte Anzahl festlegen")))
            .action_with(hidden_operation("curateRemove", LocalizedLabel::native("Curate Remove", "Kuratierung entfernen")))
            .action_with(hidden_operation("dropOnPool", LocalizedLabel::native("Drop On Pool", "Auf Pool ablegen")))
            .action_with(hidden_operation("dropOnCurated", LocalizedLabel::native("Drop On Curated", "Auf Kuratiert ablegen")))
            // 👁️ Filters/sort/selection — session-only `SourcingCurateConfig` view state, never the document.
            .action_with(hidden_view_action("setFilterQuery", LocalizedLabel::native("Set Filter Query", "Filterabfrage festlegen")))
            .action_with(hidden_view_action("setFilterModule", LocalizedLabel::native("Set Filter Module", "Filtermodul festlegen")))
            .action_with(hidden_view_action("setFilterTypology", LocalizedLabel::native("Set Filter Typology", "Filtertypologie festlegen")))
            .action_with(hidden_view_action("setFilterMinAvailability", LocalizedLabel::native("Set Filter Min Availability", "Mindestverfügbarkeit festlegen")))
            .action_with(hidden_view_action("sortTable", LocalizedLabel::native("Sort Table", "Tabelle sortieren")))
            .action_with(hidden_view_action("selectRow", LocalizedLabel::native("Select Row", "Zeile auswählen")))
            .action_with(hidden_view_action("worldSelect", LocalizedLabel::native("World Select", "Welt auswählen")))
            // 📝️ Staged argument form for the panel-visible example switch.
            .action_args(
                "setActiveExample",
                vec![ActionArgDef::select(
                    "exampleId",
                    LocalizedLabel::native("Example", "Beispiel"),
                    vec![ActionArgOption::new(DEMO_STOCK_EXAMPLE_ID, LocalizedLabel::native("Demo Stock", "Beispielbestand")), ActionArgOption::new(EMPTY_EXAMPLE_ID, LocalizedLabel::native("Empty Curation", "Leere Kuratierung"))],
                )
                .default_value(DEMO_STOCK_EXAMPLE_ID)],
            )
            // 🎯️ Typed channel surface — this app's typed commands are dispatched via
            // `SourcingCurateCommand`'s `OpBinary` codec directly (`setLocale` deliberately left
            // undeclared above, mirroring `flow_ui`: `VcsArtifactApp`'s kind-discipline check only runs
            // when the registry actually declares a command's id).
            .io(crate::artifacts::curate::engine::sourcing_curate_io()),
    )
    // 📄️ `AppDefinition::example` still wants document JSON (the manifest-wide example wire format);
    // the `.curate` text is only the on-disk source of truth, re-serialized here once.
    .example(DEMO_STOCK_EXAMPLE_ID, LocalizedLabel::native("Demo Stock", "Beispielbestand"), serde_json::to_string(&crate::artifacts::curate::engine::default_document()).unwrap_or_default(), "file-text")
    .example(EMPTY_EXAMPLE_ID, LocalizedLabel::native("Empty Curation", "Leere Kuratierung"), serde_json::to_string(&crate::artifacts::curate::engine::empty_document()).unwrap_or_default(), "file-text")
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app as new_app_impl, new_app_with_registry as new_app_with_registry_impl};
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type SourcingApp = VcsArtifactApp<SourcingCurateApp>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub fn new_app() -> SourcingApp {
        new_app_impl::<SourcingCurateApp>()
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub fn new_app_with_registry() -> SourcingApp {
        new_app_with_registry_impl::<SourcingCurateApp>(create_sourcing_curate_app)
    }

    pub fn dispatch(app: &mut SourcingApp, command: SourcingCurateCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut SourcingApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::curate::testkit::new_app_with_registry;
    use semio_framework_plugin::testkit;
    use semio_framework_plugin::PluginApp;

    #[test]
    fn view_kind_config_only_commands_pass_kind_discipline() {
        // 🧬️ A registry-backed wrapper so the View-kind declarations actually get enforced.
        let mut app = new_app_with_registry();
        let result = app.dispatch_typed(SourcingCurateCommand::SetFilterQuery(set_filter_query::SetFilterQuery { value: "glulam".into() }), &testkit::meta("local")).expect("filter query");
        assert!(result.mutations.is_empty(), "setFilterQuery is config-only, no document operations");
    }

    //#region 🔖️CommandSurface
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every
    /// row's wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
    #[test]
    fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 17, "every SourcingCurateCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row — the
    /// permanent successor of the old `📡️protocol` crate's
    /// `sourcing_curate_command_op_text_round_trips_every_variant`.
    #[test]
    fn every_command_round_trips_through_text_and_binary() {
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
    #[test]
    fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
        fn expected_wire_key(id: &str) -> &'static str {
            match id {
                "setDocument" => "document-json",
                "setActiveExample" => "active-example",
                "stockFromCatalogue" => "stock-from-catalogue",
                "curateAdd" => "curate-add",
                "curateSetCount" => "curate-set-count",
                "curateRemove" => "curate-remove",
                "dropOnPool" => "drop-on-pool",
                "dropOnCurated" => "drop-on-curated",
                "setFilterQuery" => "filter-query",
                "setFilterModule" => "filter-module",
                "setFilterTypology" => "filter-typology",
                "setFilterMinAvailability" => "filter-min-availability",
                "sortTable" => "sort-table",
                "selectRow" => "select-row",
                "worldSelect" => "world-select",
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
    /// captured from the pre-merge `semio-s-app-sourcing-curate-protocol` crate
    /// (`wire-baseline-before.txt` in this ticket's folder). A regression here is a real format break, not
    /// a test-fixture mismatch.
    #[test]
    fn optional_field_rows_keep_their_pre_migration_bytes() {
        let cases: [(SourcingCurateCommand, &str, &str); 4] = [
            (
                SourcingCurateCommand::CurateSetCount(curate_set_count::CurateSetCount { object_id: "beam-glulam-gl24h".into(), delta: Some(1.0), value: None }),
                "curate-set-count curate-set-count object-id=beam-glulam-gl24h delta=1",
                "010401116265616d2d676c756c616d2d676c323468020006000105000000000000f03f",
            ),
            (
                SourcingCurateCommand::CurateSetCount(curate_set_count::CurateSetCount { object_id: "beam-glulam-gl24h".into(), delta: None, value: Some(4.0) }),
                "curate-set-count curate-set-count object-id=beam-glulam-gl24h value=4",
                "010401116265616d2d676c756c616d2d676c3234680200060002050000000000001040",
            ),
            (SourcingCurateCommand::SelectRow(select_row::SelectRow { object_id: Some("beam-glulam-gl24h".into()) }), "select-row select-row object-id=beam-glulam-gl24h", "010d01116265616d2d676c756c616d2d676c32346801000600"),
            (SourcingCurateCommand::SelectRow(select_row::SelectRow { object_id: None }), "select-row select-row", "010d0000"),
        ];
        for (command, text, hex) in cases {
            assert_eq!(protocol::OpText::print_op(&command), text);
            assert_eq!(protocol::OpBinary::encode_op(&command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>(), hex);
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order — mirrors the
    /// pre-migration wire baseline captured into this ticket's `wire-baseline-before.txt`.
    fn every_command() -> Vec<SourcingCurateCommand> {
        vec![
            SourcingCurateCommand::SetArtifactJson(set_artifact_json::SetArtifactJson { json: "{}".into() }),
            SourcingCurateCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "demo-stock".into() }),
            SourcingCurateCommand::StockFromCatalogue(stock_from_catalogue::StockFromCatalogue {}),
            SourcingCurateCommand::CurateAdd(curate_add::CurateAdd { object_id: "beam-glulam-gl24h".into() }),
            SourcingCurateCommand::CurateSetCount(curate_set_count::CurateSetCount { object_id: "beam-glulam-gl24h".into(), delta: Some(1.0), value: None }),
            SourcingCurateCommand::CurateRemove(curate_remove::CurateRemove { object_id: "beam-glulam-gl24h".into() }),
            SourcingCurateCommand::DropOnPool(drop_on_pool::DropOnPool { object_id: "beam-glulam-gl24h".into() }),
            SourcingCurateCommand::DropOnCurated(drop_on_curated::DropOnCurated { object_id: "beam-glulam-gl24h".into() }),
            SourcingCurateCommand::SetFilterQuery(set_filter_query::SetFilterQuery { value: "glulam".into() }),
            SourcingCurateCommand::SetFilterModule(set_filter_module::SetFilterModule { module_id: "beams".into(), enabled: true }),
            SourcingCurateCommand::SetFilterTypology(set_filter_typology::SetFilterTypology { path: "beams/steel".into() }),
            SourcingCurateCommand::SetFilterMinAvailability(set_filter_min_availability::SetFilterMinAvailability { delta: Some(1.0), value: None }),
            SourcingCurateCommand::SortTable(sort_table::SortTable { column_id: "availability".into(), direction: "desc".into() }),
            SourcingCurateCommand::SelectRow(select_row::SelectRow { object_id: Some("beam-glulam-gl24h".into()) }),
            SourcingCurateCommand::WorldSelect(world_select::WorldSelect { ids: vec!["beam-glulam-gl24h".into(), "beam-kvh-c24".into()] }),
            SourcingCurateCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
            SourcingCurateCommand::SetContributions(set_contributions::SetContributions { json: "[]".into() }),
        ]
    }
    //#endregion 🔖️CommandSurface

    #[test]
    fn app_definition_labels_resolve_german() {
        let def = &create_sourcing_curate_app().definition;
        let (terminology, locale) = (semio_framework_plugin::Terminology::Native, semio_framework_plugin::Locale::De);
        assert_eq!(def.window_kinds.iter().find(|entry| entry.id == pool::SOURCING_CURATE_WINDOW_POOL).expect("pool window").label.resolve(terminology, locale), "Pool");
        assert_eq!(def.window_kinds.iter().find(|entry| entry.id == curated::SOURCING_CURATE_WINDOW_CURATED).expect("curated window").label.resolve(terminology, locale), "Kuratiert");
        assert_eq!(def.modes.iter().find(|entry| entry.id == curate::SOURCING_CURATE_MODE_CURATE).expect("curate mode").label.resolve(terminology, locale), "Kuratieren");
    }

    #[test]
    fn sourcing_curate_io_and_catalog_export_round_trip() {
        let mut app = crate::apps::curate::testkit::new_app();
        let media = app.export_media("catalog:out").expect("catalog export");
        assert_eq!(media.media_type.class, MediaClass::Kit);
        assert_eq!(media.media_type.form, MediaForm::Type);
        match media.payload {
            MediaPayload::Structured { schema, json } => {
                assert_eq!(schema, "kit.catalog");
                let fragment: serde_json::Value = serde_json::from_str(&json).unwrap();
                assert_eq!(fragment["objectKinds"].as_array().unwrap().len(), app.snapshot().expect("snapshot").stock.len());
            }
            MediaPayload::Binary { .. } => panic!("expected a Structured payload"),
        }
    }

    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_sourcing_curate_app().definition).expect("app definition json");
        for id in [pool::SOURCING_CURATE_WINDOW_POOL, curated::SOURCING_CURATE_WINDOW_CURATED, preview::SOURCING_CURATE_WINDOW_PREVIEW, grid::SOURCING_CURATE_WINDOW_GRID] {
            assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
        }
        assert!(json.contains(curate::SOURCING_CURATE_MODE_CURATE), "mode missing from the manifest");
        assert!(json.contains("catalogue.sourcing"), "artifact kind missing from the manifest");
    }
}
//#endregion 🧪️Tests
