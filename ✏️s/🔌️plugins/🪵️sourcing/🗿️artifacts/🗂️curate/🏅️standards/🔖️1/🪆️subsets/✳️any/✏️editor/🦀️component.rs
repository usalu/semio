//! 🛒️ Sourcing curate app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/✏️edit/🪟️windows/*`, labels in `🦀️terminology.rs`, view state in `🦀️config.rs`, shared
//! compute in the artifact's `🧬️schema`. This file is a routing table: `handle` → `SourcingCurateCommand::
//! dispatch`, `render` → body-key → node, and a `🔖️Manifest` region that calls one `definition()` per node.

use crate::editor::sourcing::config::{SourcingCurateConfig, SourcingCurateConfigMutation};
use crate::editor::sourcing::presence::{SourcingCuratePresence, SourcingCuratePresenceMutation};
use crate::editor::sourcing::modes::edit;
use crate::editor::sourcing::modes::edit::windows::{curated, grid, pool, preview};
use crate::editor::sourcing::terminology::sourcing_curate_labels;
use crate::artifacts::curate::op::SourcingMutation;
use crate::artifacts::curate::{CurateSnapshot, SOURCING_CURATE_SCHEMA};
use semio_framework_plugin::{NoDraft, NoDraftMutation, DraftView,
    ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, AppDefinition, ArtifactEditor, ArtifactKindSpec, CommandDefinition, ConfigView, ArtifactView, Dialect, Editor, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType,
    MergeMode, OsMediaCapability, SelectionMethod, SelectionMode, SelectionSpec, UiNode,
};
use semio_framework_plugin::app::InteractionView;
use store::EngineHandles;
use store::ArtifactPack;

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — the implicit document ports (keyed off
/// `SOURCING_CURATE_SCHEMA`, `MediaType{Kit,Kit}` matching the `"catalogue.sourcing"` `ArtifactKindSpec`)
/// plus the extra `catalog:out` output port: this app's `stock` (its `"catalogue.kinds"`-shaped rows)
/// mapped into the SAME `kit.catalog` JSON shape `block_3d::puzzle3d_catalog_fragment` produces, so
/// `s/plugin/puzzle`'s `kit:in` importer can consume either producer identically without knowing which
/// one it came from (see `crate::artifacts::curate::schema::inferences::sourcing_catalog_fragment`).
pub async fn sourcing_curate_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: SOURCING_CURATE_SCHEMA.into(),
        document_media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Kit },
        ports: vec![semio_framework_plugin::MediaPortSpec {
            id: "catalog:out".into(),
            label: "Catalog".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
            kind_id: Some("kit.catalog".into()),
            required: false,
            multiplicity: semio_framework::PortMultiplicity::Many,
        }],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "catalogue.sourcing".into(), name: "Sourcing Curation".into(), dimension: "data".into(), component_kind: "catalogue".into() },
    }
}
//#endregion 🔖️Io

//#region 🔖️Constants
/// 🎯️ Still used as a plain namespace tag for `ActionFactory`/`TableScene`/`WorldScene` addressing
/// below — NOT a trait const any more (contract §7.4: the real surface id is now derived from
/// `SOURCING_DIALECT` + `AppRole` via `surface_app_id`, never hand-written).
pub const SOURCING_CONTROLLER_ID: &str = "sourcing-curate";
pub const SOURCING_DRAG_MIME: &str = "application/x-semio-sourcing-object";
pub const DEMO_STOCK_EXAMPLE_ID: &str = "demo-stock";
pub const EMPTY_EXAMPLE_ID: &str = "empty-curation";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// builds its `on_change`/drop actions with.
pub async fn sourcing_action(action: &str, args: Option<serde_json::Value>) -> ActionDescriptor {
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
        "setLocale" as "locale" => set_locale::SetLocale,
        "setContributions" as "contributions" => set_contributions::SetContributions,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use crate::editor::sourcing::commands::{curate_add, curate_remove, curate_set_count, drop_on_curated, drop_on_pool};
use crate::editor::sourcing::commands::set_contributions;
use crate::editor::sourcing::commands::{set_active_example, set_artifact_json, stock_from_catalogue};
use crate::editor::sourcing::commands::{set_filter_min_availability, set_filter_module, set_filter_query, set_filter_typology, sort_table};
use crate::editor::sourcing::commands::set_locale;

/// 🎯️ Host action id + JSON args → the closed `SourcingCurateCommand` vocabulary — the production
/// bridge between the manifest's *declared* action surface (`🔖️Manifest`, camelCase arg names) and
/// the typed command channel that actually dispatches.
///
/// Chrome built by the `🎭️modes` nodes addresses this app by action id (`sourcing_action`), and
/// several of those rows carry `None` args because the host fills them at dispatch time from the
/// interaction itself (a text input's `value`, a slider's `delta`, a drop's `objectId`) — so every
/// field is read defensively and coerced, never assumed present.
///
/// Without this, `ArtifactApp::command_from_action`'s default rejects every app-owned action and the
/// pane cannot even load its own example. See `📐️cad`'s `cad_command_from_action` twin.
async fn sourcing_curate_command_from_action(action: &str, args: Option<&serde_json::Value>) -> Result<SourcingCurateCommand, Fault> {
    let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(serde_json::Value::as_str).map(str::to_string);
    let f64_field = |key: &str| args.and_then(|value| value.get(key)).and_then(serde_json::Value::as_f64);
    let bool_field = |key: &str| args.and_then(|value| value.get(key)).and_then(serde_json::Value::as_bool);
    let text_of = |key: &str| -> Option<String> {
        args.and_then(|value| value.get(key)).and_then(|value| match value {
            serde_json::Value::String(text) => Some(text.clone()),
            serde_json::Value::Bool(flag) => Some(flag.to_string()),
            serde_json::Value::Number(number) => Some(number.to_string()),
            _ => None,
        })
    };
    let json_field = |key: &str| -> String {
        match args.and_then(|value| value.get(key)) {
            Some(serde_json::Value::String(text)) => text.clone(),
            Some(other) => other.to_string(),
            None => args.map(serde_json::Value::to_string).unwrap_or_default(),
        }
    };
    let object_id = || str_field("objectId").unwrap_or_default();
    Ok(match action {
        "setDocument" => SourcingCurateCommand::SetArtifactJson(set_artifact_json::SetArtifactJson { json: json_field("json") }),
        "setActiveExample" => SourcingCurateCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: str_field("exampleId").unwrap_or_default() }),
        "stockFromCatalogue" => SourcingCurateCommand::StockFromCatalogue(stock_from_catalogue::StockFromCatalogue {}),
        "curateAdd" => SourcingCurateCommand::CurateAdd(curate_add::CurateAdd { object_id: object_id() }),
        "curateSetCount" => SourcingCurateCommand::CurateSetCount(curate_set_count::CurateSetCount { object_id: object_id(), delta: f64_field("delta"), value: f64_field("value") }),
        "curateRemove" => SourcingCurateCommand::CurateRemove(curate_remove::CurateRemove { object_id: object_id() }),
        "dropOnPool" => SourcingCurateCommand::DropOnPool(drop_on_pool::DropOnPool { object_id: object_id() }),
        "dropOnCurated" => SourcingCurateCommand::DropOnCurated(drop_on_curated::DropOnCurated { object_id: object_id() }),
        "setFilterQuery" => SourcingCurateCommand::SetFilterQuery(set_filter_query::SetFilterQuery { value: text_of("value").unwrap_or_default() }),
        "setFilterModule" => SourcingCurateCommand::SetFilterModule(set_filter_module::SetFilterModule { module_id: str_field("moduleId").unwrap_or_default(), enabled: bool_field("enabled").unwrap_or(false) }),
        "setFilterTypology" => SourcingCurateCommand::SetFilterTypology(set_filter_typology::SetFilterTypology { path: str_field("path").or_else(|| text_of("value")).unwrap_or_default() }),
        "setFilterMinAvailability" => SourcingCurateCommand::SetFilterMinAvailability(set_filter_min_availability::SetFilterMinAvailability { delta: f64_field("delta"), value: f64_field("value") }),
        "sortTable" => SourcingCurateCommand::SortTable(sort_table::SortTable { column_id: str_field("columnId").unwrap_or_default(), direction: str_field("direction").unwrap_or_default() }),
        "setLocale" => SourcingCurateCommand::SetLocale(set_locale::SetLocale { value: text_of("value").unwrap_or_default() }),
        "setContributions" => SourcingCurateCommand::SetContributions(set_contributions::SetContributions { json: json_field("json") }),
        other => {
            return Err(Fault::new(
                semio_framework_plugin::FaultOrigin::App,
                semio_framework_plugin::FaultCode::new("app.command.unsupported"),
                format!("action '{other}' is not a sourcing curate command"),
            ))
        }
    })
}
//#endregion 🔖️Commands

//#region 🔖️SourcingCurateApp
/// 🧪️ Unit struct — every former app-struct field lives in `crate::editor::sourcing::config::
/// SourcingCurateConfig`, written through `SourcingCurateConfigMutation`s.
#[derive(Default)]
pub struct SourcingCurateApp;

impl ArtifactEditor for SourcingCurateApp {
    type Snapshot = CurateSnapshot;
    type Mutation = SourcingMutation;
    type Config = SourcingCurateConfig;
    type ConfigMutation = SourcingCurateConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = SourcingCuratePresence;
    type PresenceMutation = SourcingCuratePresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = SourcingCurateCommand;

    const DIALECT: Dialect = crate::artifacts::curate::SOURCING_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = SOURCING_CURATE_SCHEMA;

    async fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::sourcing::config::schema::app_schema_descriptor())
    }

    async fn initial_snapshot() -> CurateSnapshot {
        crate::artifacts::curate::schema::default_document()
    }

    async fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(sourcing_curate_io())
    }

    /// 🎞️ `catalog:out` (see `crate::artifacts::curate::schema::inferences::sourcing_catalog_fragment`)
    /// plus the inherited `document:out` default (the pack of `doc.snapshot`, replicated inline —
    /// overriding `export_media` shadows the trait's provided body for every port on this app, not just
    /// the new one).
    async fn export_media(port: &str, doc: &ArtifactView<'_, CurateSnapshot>) -> Result<Media, MediaError> {
        match port {
            "catalog:out" => Ok(Media {
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                payload: MediaPayload::Structured { schema: "kit.catalog".into(), json: crate::artifacts::curate::schema::inferences::sourcing_catalog_fragment(doc.snapshot).to_string() },
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
    /// `Effect::LoadDocument` via `reset_document_effect`, outside undo history.
    async fn import_media(port: &str, media: &Media, _doc: &ArtifactView<'_, CurateSnapshot>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation, Self::DraftMutation>, MediaError> {
        if port != "document:in" {
            return Err(MediaError::NotImplemented);
        }
        let MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "document:in importer only accepts a Structured (base64 pack) payload".into()));
        };
        let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let snapshot = <CurateSnapshot as ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        Ok(Emit { effects: vec![reset_document_effect(&snapshot)], ..Default::default() })
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`. `setLocale` has no manifest declaration (host-pushed,
    /// not a user-facing action).
    async fn command_id(command: &SourcingCurateCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Production action bridge — see `sourcing_curate_command_from_action`. Overriding this is
    /// mandatory for any app that declares its own actions: the trait default only admits the
    /// framework-reserved ids and rejects everything else.
    async fn command_from_action(action: &str, args: Option<&serde_json::Value>) -> Result<SourcingCurateCommand, Fault> {
        sourcing_curate_command_from_action(action, args)
    }

    async fn handle(command: &SourcingCurateCommand, doc: &ArtifactView<'_, CurateSnapshot>, cfg: &ConfigView<'_, SourcingCurateConfig>, _interaction: &InteractionView<'_>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, CurateSnapshot>, cfg: &ConfigView<'_, SourcingCurateConfig>) -> UiNode {
        crate::artifacts::curate::schema::sync_sourcing_module_contributions(&cfg.snapshot.contributions_json);
        let snapshot = doc.snapshot;
        let config = cfg.snapshot;
        let labels = sourcing_curate_labels(config);
        match body_key {
            pool::SOURCING_CURATE_BODY_POOL => pool::render(snapshot, config, labels),
            curated::SOURCING_CURATE_BODY_CURATED => curated::render(snapshot, labels),
            // 🕹️ `render` carries no `InteractionView` (ArtifactApp's breaking pass only added it to
            // `handle`/`copy_fragment`/`cut_operations` — see ticket 26/08/14's w3b-summary.md) — the
            // preview window degrades to its "no selection" default until a future wave threads
            // interaction into render. Flagged as a discovered framework gap, not worked around here.
            preview::SOURCING_CURATE_BODY_PREVIEW => preview::render(snapshot, &[], labels),
            grid::SOURCING_CURATE_BODY_GRID => grid::render(snapshot, config),
            _ => semio_framework_plugin::ui_text(Label::data("")),
        }
    }
}
//#endregion 🔖️SourcingCurateApp

//#region 🔖️ResetDocument
/// 🌱️ Builds a `Effect::LoadDocument` that swaps the live document to `document` OUTSIDE undo
/// history — the sanctioned non-mutation path for a whole-document replace (JSON import, load-
/// example, bulk catalogue restock). Per `📓️taxonomy.md`, the former whole-snapshot-replace variant
/// is banned outright with NO replacement mutation: whole-document replace is not expressible as an in-history `Mutation` at
/// all. Every former "replace the whole document" gesture in this app (`import_media`'s
/// `"document:in"` above, `commands::document::{set_active_example, set_artifact_json,
/// stock_from_catalogue}`) builds this effect instead of an `Emit::mutations([...])`. The spr is a
/// fresh, edit-free op-log for `document` — a genesis envelope with no history to encode.
pub async fn reset_document_effect(document: &CurateSnapshot) -> semio_framework::kernel::Effect {
    let pack = <CurateSnapshot as ArtifactPack>::encode_pack(document);
    let envelope = store::create_document_envelope::<CurateSnapshot, SourcingMutation>(SOURCING_CURATE_SCHEMA, "curate", document.clone(), None);
    let spr = store::print_document_spr(&envelope).expect("curate document spr encode is infallible for a fresh, edit-free envelope");
    semio_framework::kernel::Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️ResetDocument

//#region 🔖️Manifest
/// 🙈️ An internal document operation kept out of the command palette — the curate/DnD arms that mutate
/// the persisted `CurateSnapshot` but are only ever dispatched from window chrome.
async fn hidden_operation(id: &str, label: impl Into<LocalizedLabel>) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog(id, label, ActionKind::Mutation) }
}

/// 🙈️👁️ The filter/sort/selection/world-pick arms emit ONLY `config_mutations`, so (unlike
/// `hidden_operation` above) they're declared `ActionKind::View`, letting `VcsArtifactApp`'s
/// kind-discipline check actually enforce "must not emit document operations".
async fn hidden_view_action(id: &str, label: impl Into<LocalizedLabel>) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog(id, label, ActionKind::View) }
}

/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
///
/// 🚧️ SDK GAP (contract §2.4): `EditorBuilder` has no `.example(...)`/`.workflow(...)` — the
/// pre-migration `App { definition, examples }` split means `.editor::<E>(def: AppDefinition)` only
/// ever registers an empty `examples` list. The two demo examples this app used to register here
/// (`DEMO_STOCK_EXAMPLE_ID`/`EMPTY_EXAMPLE_ID`) are dropped from `AppDefinition`, not ported — they
/// stay reachable through the `setActiveExample` action (`action_args` below) and this subset's own
/// `📚️examples` facet, just no longer wired into the manifest's `examples` list. See
/// `📓️w2-cad-report.md`'s "SDK gaps found" #4 for the same gap hit by the pilot packet.
pub async fn create_sourcing_curate_app() -> AppDefinition {
    Editor::builder(crate::artifacts::curate::SOURCING_DIALECT)
            .command(CommandDefinition { in_palette: false, ..CommandDefinition::new_catalog("setContributions", LocalizedLabel::native("Set Contributions", "Beiträge festlegen"), "host", ActionKind::View).with_args([ActionArgDef::text("json", LocalizedLabel::native("Contributions", "Beiträge"))]) })
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
            .mode_def(edit::definition())
            .default_mode_id(edit::SOURCING_CURATE_MODE_CURATE)
            .window_kind_def(pool::definition())
            .window_kind_def(curated::definition())
            .window_kind_def(preview::definition())
            .window_kind_def(grid::definition())
            .default_layout(edit::layout())
            // 🕹️ The "rows" interaction domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-
            // MECHANISM) replaces the deleted `selected_object_id` config field and the `select-row`/
            // `world-select` commands — a curation table picks exactly one row. The six framework verbs
            // (`interactionSelect`/`interactionHover`/`clearSelection`/`selectAll`/`setSelectionMode`/
            // `setInteractionGranularity`) auto-inject; the pool/curated tables and the grid's world3d
            // pick surface all carry it.
            .interaction(InteractionDefinition {
                id: "rows".into(),
                label: LocalizedLabel::native("Rows", "Zeilen"),
                granularities: vec![GranularityDefinition { id: "object".into(), label: LocalizedLabel::native("Object", "Objekt"), icon_id: "box".into() }],
                hierarchy: HierarchyProvider::Flat,
                hover: HoverSpec::default(),
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick],
                    merges: vec![MergeMode::Replace],
                    transitive: false,
                    broadcast: true,
                },
            })
            .window_kind_interactions(pool::SOURCING_CURATE_WINDOW_POOL, vec![InteractionRef::new("rows")])
            .window_kind_interactions(curated::SOURCING_CURATE_WINDOW_CURATED, vec![InteractionRef::new("rows")])
            .window_kind_interactions(grid::SOURCING_CURATE_WINDOW_GRID, vec![InteractionRef::new("rows")])
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
            .io(sourcing_curate_io())
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app as new_app_impl, new_app_with_registry as new_app_with_registry_impl};
    use semio_framework_plugin::{App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type SourcingApp = VcsArtifactApp<EditorApp<SourcingCurateApp>>;

    /// 🧪️ Framework testkit gap (contract §2.5, w0-f Gap 3 handoff): `new_app_with_registry` and
    /// `assert_declared_actions_bridge_to_commands` still take the pre-migration `fn() -> App` shape,
    /// not the `AppDefinition`-returning `create_sourcing_curate_app`. Local wrapper until that lands.
    pub(crate) async fn sourcing_manifest_for_testkit() -> App {
        App { definition: create_sourcing_curate_app(), examples: Vec::new() }
    }

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub async fn new_app() -> SourcingApp {
        new_app_impl::<EditorApp<SourcingCurateApp>>()
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub async fn new_app_with_registry() -> SourcingApp {
        new_app_with_registry_impl::<EditorApp<SourcingCurateApp>>(sourcing_manifest_for_testkit)
    }

    pub async fn dispatch(app: &mut SourcingApp, command: SourcingCurateCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub async fn render(app: &mut SourcingApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::sourcing::testkit::{new_app_with_registry, sourcing_manifest_for_testkit};
    use semio_framework_plugin::testkit;
    use semio_framework_plugin::{EditorApp, PluginApp};

    #[semio_framework_async_macros::async_test]
    async fn view_kind_config_only_commands_pass_kind_discipline() {
        // 🧬️ A registry-backed wrapper so the View-kind declarations actually get enforced.
        let mut app = new_app_with_registry();
        let result = app.dispatch_typed(SourcingCurateCommand::SetFilterQuery(set_filter_query::SetFilterQuery { value: "glulam".into() }), &testkit::meta("local")).expect("filter query");
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
        assert_eq!(ids.len(), 15, "every SourcingCurateCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: the PRODUCTION action bridge covers every declared row. `ArtifactEditor`'s default
    /// `command_from_action` rejects app-owned actions outright, so an app that declares actions but
    /// never overrides it ships a UI whose every control faults — which is exactly how this pane's
    /// `setActiveExample` (and with it the whole `demo-stock` example) was dead in the demonstrator.
    /// Asserted through `<SourcingCurateApp as ArtifactEditor>` — NOT the free function — because the
    /// trait method is the seam the host actually calls.
    #[semio_framework_async_macros::async_test]
    async fn the_production_action_bridge_admits_every_declared_command() {
        for command in every_command() {
            let action = command.command_id();
            let built = <SourcingCurateApp as ArtifactEditor>::command_from_action(action, None)
                .unwrap_or_else(|fault| panic!("action '{action}' is declared but the production bridge rejects it: {fault:?}"));
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
        testkit::assert_declared_actions_bridge_to_commands::<EditorApp<SourcingCurateApp>>(sourcing_manifest_for_testkit);
    }

    /// ⚖️ LAW: the bridge reads the manifest's OWN arg names — `setActiveExample` declares a select
    /// arg keyed `exampleId` (`🔖️Manifest`), and the payload field is `example_id`; the two
    /// vocabularies are joined here and nowhere else.
    #[semio_framework_async_macros::async_test]
    async fn the_action_bridge_reads_the_declared_arg_names() {
        let built = <SourcingCurateApp as ArtifactEditor>::command_from_action("setActiveExample", Some(&serde_json::json!({ "exampleId": DEMO_STOCK_EXAMPLE_ID })))
            .expect("setActiveExample must convert");
        assert_eq!(built, SourcingCurateCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: DEMO_STOCK_EXAMPLE_ID.into() }));
        let filter = <SourcingCurateApp as ArtifactEditor>::command_from_action("setFilterModule", Some(&serde_json::json!({ "moduleId": "walls", "enabled": true })))
            .expect("setFilterModule must convert");
        assert_eq!(filter, SourcingCurateCommand::SetFilterModule(set_filter_module::SetFilterModule { module_id: "walls".into(), enabled: true }));
        assert!(<SourcingCurateApp as ArtifactEditor>::command_from_action("noSuchAction", None).is_err(), "an undeclared action must fault, not silently no-op");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row — the
    /// permanent successor of the old `📡️protocol` crate's
    /// `sourcing_curate_command_op_text_round_trips_every_variant`.
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
        async fn expected_wire_key(id: &str) -> &'static str {
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
    #[semio_framework_async_macros::async_test]
    async fn optional_field_rows_keep_their_pre_migration_bytes() {
        let cases: [(SourcingCurateCommand, &str, &str); 2] = [
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
        ];
        for (command, text, hex) in cases {
            assert_eq!(protocol::OpText::print_op(&command), text);
            assert_eq!(protocol::OpBinary::encode_op(&command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>(), hex);
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order — mirrors the
    /// pre-migration wire baseline captured into this ticket's `wire-baseline-before.txt`.
    async fn every_command() -> Vec<SourcingCurateCommand> {
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
            SourcingCurateCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
            SourcingCurateCommand::SetContributions(set_contributions::SetContributions { json: "[]".into() }),
        ]
    }
    //#endregion 🔖️CommandSurface

    #[semio_framework_async_macros::async_test]
    async fn app_definition_labels_resolve_german() {
        let def = &create_sourcing_curate_app();
        let (terminology, locale) = (semio_framework_plugin::Terminology::Native, semio_framework_plugin::Locale::De);
        assert_eq!(def.window_kinds.iter().find(|entry| entry.id == pool::SOURCING_CURATE_WINDOW_POOL).expect("pool window").label.resolve(terminology, locale), "Pool");
        assert_eq!(def.window_kinds.iter().find(|entry| entry.id == curated::SOURCING_CURATE_WINDOW_CURATED).expect("curated window").label.resolve(terminology, locale), "Kuratiert");
        assert_eq!(def.modes.iter().find(|entry| entry.id == edit::SOURCING_CURATE_MODE_CURATE).expect("curate mode").label.resolve(terminology, locale), "Kuratieren");
    }

    #[semio_framework_async_macros::async_test]
    async fn sourcing_curate_io_declares_the_catalog_out_port_alongside_the_implicit_document_ports() {
        let io = sourcing_curate_io();
        assert_eq!(io.document_schema, SOURCING_CURATE_SCHEMA);
        let ports = io.all_ports();
        assert_eq!(ports.len(), 3, "document:in, document:out, catalog:out");
        let catalog_out = ports.iter().find(|port| port.id == "catalog:out").expect("catalog:out port declared");
        assert_eq!(catalog_out.kind_id.as_deref(), Some("kit.catalog"));
        assert_eq!(catalog_out.media_type.class, MediaClass::Kit);
        assert_eq!(catalog_out.media_type.form, MediaForm::Type);
    }

    #[semio_framework_async_macros::async_test]
    async fn sourcing_curate_io_and_catalog_export_round_trip() {
        let mut app = crate::editor::sourcing::testkit::new_app();
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

    #[semio_framework_async_macros::async_test]
    async fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_sourcing_curate_app()).expect("app definition json");
        for id in [pool::SOURCING_CURATE_WINDOW_POOL, curated::SOURCING_CURATE_WINDOW_CURATED, preview::SOURCING_CURATE_WINDOW_PREVIEW, grid::SOURCING_CURATE_WINDOW_GRID] {
            assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
        }
        assert!(json.contains(edit::SOURCING_CURATE_MODE_CURATE), "mode missing from the manifest");
        assert!(json.contains("catalogue.sourcing"), "artifact kind missing from the manifest");
    }
}
//#endregion 🧪️Tests
