//! 📋️ Manifest: the hidden operation and view action builders and `create_sourcing_curation_app`.

use super::*;
use crate::editor::sourcing::base::{DEMO_STOCK_EXAMPLE_ID, SOURCING_OBJECT_GRANULARITY, SOURCING_ROWS_DOMAIN, sourcing_curation_io};

//#region 🔖️Manifest
/// 🙈️ An internal document operation kept out of the command palette — the curation/DnD arms that mutate
/// the persisted `CurationSnapshot` but are only ever dispatched from window chrome.
fn hidden_operation(id: &str, label: impl Into<LocalizedLabel>) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(id, label, ActionKind::Mutation) }
}

/// 🙈️👁️ The filter/sort/selection/world-pick arms emit ONLY `config_mutations`, so (unlike
/// `hidden_operation` above) they're declared `ActionKind::View`, letting `VcsArtifactApp`'s
/// kind-discipline check actually enforce "must not emit document operations".
fn hidden_view_action(id: &str, label: impl Into<LocalizedLabel>) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(id, label, ActionKind::View) }
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
pub fn create_sourcing_curation_app() -> AppDefinition {
    Editor::builder(crate::SOURCING_DIALECT)
            .command({
                let mut definition = CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("setContributions", LocalizedLabel::native("Set Contributions", "Beiträge festlegen"), "host", ActionKind::View).with_args([ActionArgDef::text("json", LocalizedLabel::native("Contributions", "Beiträge"))]) };
                definition.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
                definition
            })
            .action_interactive_job("setContributions", InteractiveJobClassification::Migrated)
            .document(["semio", "sourcing", "curation"])
            .terminology("reuse")
            .terminology_document("reuse", ["Entwerfen mit Bestand", "Aussuchen"])
            .artifact_kind(crate::artifact_kind())
            .artifact_kind(ArtifactKindSpec {
                id: "catalogue.kinds".into(),
                label: LocalizedLabel::native("Kind Catalogue", "Typenkatalog"),
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
                label: LocalizedLabel::native("Kit Catalogue", "Bausatzkatalog"),
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
            .default_mode_id(edit::SOURCING_CURATION_MODE_CURATION)
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
                id: SOURCING_ROWS_DOMAIN.into(),
                label: LocalizedLabel::native("Rows", "Zeilen"),
                granularities: vec![GranularityDefinition { id: SOURCING_OBJECT_GRANULARITY.into(), label: LocalizedLabel::native("Object", "Objekt"), icon_id: "box".into() }],
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
            .window_kind_interactions(pool::SOURCING_CURATION_WINDOW_POOL, vec![InteractionRef::new("rows")])
            .window_kind_interactions(curated::SOURCING_CURATION_WINDOW_CURATED, vec![InteractionRef::new("rows")])
            .window_kind_interactions(grid::SOURCING_CURATION_WINDOW_GRID, vec![InteractionRef::new("rows")])
            // 🔧️ Curation counts/stock edits are persisted in `CurationSnapshot`, so each arm emits a
            // whole-document `SetArtifact` operation and is declared as a Mutation, never a View.
            .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
            .action_destructive("setActiveExample")
            .mutation("stockFromCatalogue", LocalizedLabel::native("Stock From Catalogue", "Bestand aus Katalog"))
            .action_with(hidden_operation("loadDocumentJson", LocalizedLabel::native("Load Document JSON", "Dokument-JSON laden")))
            .action_destructive("loadDocumentJson")
            .action_with(hidden_operation("curationAdd", LocalizedLabel::native("Curation Add", "Kuratierung hinzufügen")))
            // 🔧️ Palette-reachable Artifact-lane mutation: the curated-table stepper and DnD arms stay
            // window chrome, but outcome 1 needs one Actions-rail row that journals a store edit
            // (ticket 26/09/18 S14). `stockFromCatalogue` remains HostOnly and cannot move `#s-checkin`.
            .action_with(ActionDefinition::bounded_catalog("curationSetCount", LocalizedLabel::native("Curation Set Count", "Kuratierte Anzahl festlegen"), ActionKind::Mutation))
            .action_with(hidden_operation("curationRemove", LocalizedLabel::native("Curation Remove", "Kuratierung entfernen")))
            .action_destructive("curationRemove")
            .action_with(hidden_operation("dropOnPool", LocalizedLabel::native("Drop On Pool", "Auf Pool ablegen")))
            .action_audience("dropOnPool", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(hidden_operation("dropOnCurated", LocalizedLabel::native("Drop On Curated", "Auf Kuratiert ablegen")))
            .action_audience("dropOnCurated", semio_framework_plugin::CapabilityAudience::Input)
            // 👁️ Filters/sort/selection — session-only `SourcingCurationConfig` view state, never the document.
            .action_with(hidden_view_action("setFilterQuery", LocalizedLabel::native("Set Filter Query", "Filterabfrage festlegen")))
            .action_with(hidden_view_action("setFilterModule", LocalizedLabel::native("Set Filter Module", "Filtermodul festlegen")))
            .action_with(hidden_view_action("setFilterTypology", LocalizedLabel::native("Set Filter Typology", "Filtertypologie festlegen")))
            .action_with(hidden_view_action("setFilterMinAvailability", LocalizedLabel::native("Set Filter Min Availability", "Mindestverfügbarkeit festlegen")))
            .action_with(hidden_view_action("sortTable", LocalizedLabel::native("Sort Table", "Tabelle sortieren")))
            .view_action("setGridInstanceDisplay", LocalizedLabel::native("Set Grid Instance Display", "Raster-Instanzdarstellung festlegen"))
            // 📝️ Staged argument form for the panel-visible example switch.
            .action_args(
                "setActiveExample",
                vec![ActionArgDef::select(
                    "exampleId",
                    LocalizedLabel::native("Example", "Beispiel"),
                    crate::editor::sourcing::base::examples().iter().map(|example| ActionArgOption::new(example.id(), example.label().clone())).collect(),
                )
                .default_value(&DEMO_STOCK_EXAMPLE_ID)],
            )
            .action_args(
                "curationSetCount",
                vec![
                    ActionArgDef::text("objectId", LocalizedLabel::native("Object", "Objekt")).required(),
                    ActionArgDef::number("delta", LocalizedLabel::native("Delta", "Delta")),
                    ActionArgDef::number("value", LocalizedLabel::native("Value", "Wert")),
                ],
            )
            // 🎯️ Typed channel surface — this app's typed commands are dispatched via
            // undeclared above, mirroring `flow_ui`: `VcsArtifactApp`'s kind-discipline check only runs
            // when the registry actually declares a command's id).
            .io(sourcing_curation_io())
            .action_interactive_job("loadDocumentJson", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("stockFromCatalogue", InteractiveJobClassification::Migrated)
            .action_interactive_job("curationAdd", InteractiveJobClassification::Migrated)
            .action_interactive_job("curationSetCount", InteractiveJobClassification::Migrated)
            .action_interactive_job("curationRemove", InteractiveJobClassification::Migrated)
            .action_interactive_job("dropOnPool", InteractiveJobClassification::Migrated)
            .action_interactive_job("dropOnCurated", InteractiveJobClassification::Migrated)
            .action_interactive_job("setFilterQuery", InteractiveJobClassification::Migrated)
            .action_interactive_job("setFilterModule", InteractiveJobClassification::Migrated)
            .action_interactive_job("setFilterTypology", InteractiveJobClassification::Migrated)
            .action_interactive_job("setFilterMinAvailability", InteractiveJobClassification::Migrated)
            .action_interactive_job("sortTable", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridInstanceDisplay", InteractiveJobClassification::Migrated)
            .action_args(
                "setGridInstanceDisplay",
                vec![ActionArgDef::select(
                    "value",
                    LocalizedLabel::native("Instance display", "Instanzdarstellung"),
                    vec![
                        ActionArgOption::new("line-behind", LocalizedLabel::native("Line behind grid", "Linie hinter Raster")),
                        ActionArgOption::new("representative", LocalizedLabel::native("Representative only", "Nur Vertreter")),
                        ActionArgOption::new("representative-with-count", LocalizedLabel::native("Representative and count", "Vertreter und Anzahl")),
                    ],
                )],
            )
            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole curation with one of the bundled example stocks or the empty curation, by example id.", "Ersetzt die gesamte Kuratierung durch einen der mitgelieferten Beispielbestände oder die leere Kuratierung, anhand der Beispiel-Id."))
            .action_describe("stockFromCatalogue", LocalizedLabel::native("Adds every object kind the installed sourcing modules (such as beams, slabs and windows) offer to the stock, keeping the existing stock and curated counts.", "Fügt dem Bestand alle Objektarten hinzu, die die installierten Beschaffungsmodule (etwa Träger, Decken und Fenster) anbieten; vorhandener Bestand und kuratierte Anzahlen bleiben erhalten."))
            .action_describe("loadDocumentJson", LocalizedLabel::native("Loads the whole curation, outside history, from one decoded from the given JSON document; oversized or schema-mismatched JSON is refused.", "Lädt die gesamte Kuratierung außerhalb des Verlaufs aus dem angegebenen JSON-Dokument; zu großes oder schemafremdes JSON wird abgelehnt."))
            .action_describe("curationAdd", LocalizedLabel::native("Adds one more of the given stock object to the curated selection, at most up to its available quantity.", "Nimmt ein weiteres Exemplar des angegebenen Bestandsobjekts in die Kuratierung auf, höchstens bis zur verfügbaren Menge."))
            .action_describe("curationSetCount", LocalizedLabel::native("Sets how many of one stock object are curated, by a relative delta or an absolute value clamped to its availability; zero removes it from the curation.", "Legt fest, wie viele Exemplare eines Bestandsobjekts kuratiert sind, per relativer Änderung oder absolutem Wert bis zur Verfügbarkeit; null entfernt es aus der Kuratierung."))
            .action_describe("curationRemove", LocalizedLabel::native("Removes one stock object from the curated selection entirely; the stock itself is unchanged.", "Entfernt ein Bestandsobjekt vollständig aus der Kuratierung; der Bestand selbst bleibt unverändert."))
            .action_describe("setGridInstanceDisplay", LocalizedLabel::native("Chooses how object instances are shown in the curation grid window; only that window's view changes.", "Wählt, wie Objektinstanzen im Kuratierungs-Rasterfenster dargestellt werden; nur die Ansicht dieses Fensters ändert sich."))
            .build_definition()
}
//#endregion 🔖️Manifest
