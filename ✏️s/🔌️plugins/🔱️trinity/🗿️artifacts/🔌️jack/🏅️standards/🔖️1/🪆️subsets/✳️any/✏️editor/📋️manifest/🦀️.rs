//! 📋️ Jack manifest: the app manifest stitch of `create_trinity_jack_app`.

use super::*;

//#region 🔖️Manifest
/// 🎯️ `create_trinity_jack_app` → `Editor::builder(TRINITY_JACK_DIALECT)…build_definition()`
/// (contract §2.4). The old `.example("nakagin", …)`/`.workflow("trinity", …)` calls are DROPPED,
/// not ported — `EditorBuilder::build_definition()` returns a bare `AppDefinition`, discarding
/// `App.examples` entirely (SDK gap, see this packet's notes file; the subset's own
/// `📚️examples/🎬️demo` facet is the likely intended replacement mechanism).
///
/// ⌨️ `textEdit`/`textSelect` are the query editor's own gesture verbs — the text host sends each
/// keystroke's buffer and caret range — so they stay out of the palette and the Actions rail, where a
/// press carries neither (`textSelect` was refused `missing start`, `textEdit` would have emptied the
/// query). `patchNodes`' `nodeIds` is optional: left empty, the verb renames the selected nodes.
pub fn create_trinity_jack_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(TRINITY_JACK_DIALECT).document(["semio", "trinity", "jack"])
            .artifact_kind(ArtifactKindSpec {
                id: "graph.trinity".into(),
                label: LocalizedLabel::native("Trinity Graph", "Trinity-Graph"),
                source_format: "trinity.graph".into(),
                component_kind: "trinity".into(),
                dimension: "graph".into(),
                media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Graph, form: MediaForm::Trinity },
                schema: "trinity.graph".into(),
                export_formats: vec![],
                import_formats: vec![],
                    export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    })
            .icon_id("trinity")
            .mode_def(edit::definition())
            .tool(edit::tools::reorganize::definition())
            .default_mode_id(edit::TRINITY_JACK_MODE_EDIT)
            .window_kind(TRINITY_JACK_PLAY_WINDOW_GRAPH, LocalizedLabel::native("Nakagin Graph", "Nakagin-Graph"), TRINITY_JACK_PLAY_BODY_GRAPH, SemanticSurfaceKind::NodeGraph, "graph-dag")
            .window_kind(TRINITY_JACK_PLAY_WINDOW_EDITOR, LocalizedLabel::native("Jack Query", "Jack-Abfrage"), TRINITY_JACK_PLAY_BODY_EDITOR, SemanticSurfaceKind::TextEditor, "document-jack")
            .window_kind(TRINITY_JACK_PLAY_WINDOW_RESULTS, LocalizedLabel::native("Results", "Ergebnisse"), TRINITY_JACK_PLAY_BODY_RESULTS, SemanticSurfaceKind::Table, "table-2")
            .default_layout(edit::layout())
            .panel_tab(
                FRAMEWORK_PANEL_TAB_ARTIFACT_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"),
                PanelGroup::Workbench,
                TRINITY_JACK_PLAY_BODY_ARTIFACT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
                PanelGroup::Workbench,
                TRINITY_JACK_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
                PanelGroup::Details,
                TRINITY_JACK_PLAY_BODY_INSPECTION,
            )
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation).with_category("selection"))
            .action_destructive("deleteSelection")
            .mutation("patchNodes", LocalizedLabel::native("Patch Nodes", "Knoten aktualisieren"))
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("runQuery", LocalizedLabel::native("Run Jack Query", "Jack-Abfrage ausführen"), ActionKind::Mutation).with_category("methods"))
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("loadExampleQuery", LocalizedLabel::native("Load Example Query", "Beispielabfrage laden"), ActionKind::Mutation).with_category("open"))
            .action_with(semio_framework_plugin::ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left").with_category("mode"))
            .action_destructive("setActiveExample")
            // 🛠️ Dev-only whole-snapshot import — kept out of the command palette.
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::bounded_catalog("loadDocumentJson", LocalizedLabel::native("Load Document JSON", "Dokument-JSON laden"), ActionKind::Mutation) })
            .action_destructive("loadDocumentJson")
            .view_action("nodeGraphViewport", LocalizedLabel::native("Set Graph Viewport", "Graph-Ansicht festlegen"))
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new("textEdit", LocalizedLabel::native("Edit Jack Query", "Jack-Abfrage bearbeiten"), ActionKind::Mutation, "typography") })
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new("textSelect", LocalizedLabel::native("Select Jack Query Text", "Jack-Abfragetext auswählen"), ActionKind::View, "text-cursor") })
            .action_with(semio_framework_plugin::ActionDefinition::new("formatDocument", LocalizedLabel::native("Format Jack Query", "Jack-Abfrage formatieren"), ActionKind::Mutation, "typography").with_category("utilities"))
            .action_with(semio_framework_plugin::ActionDefinition::new("setLodMode", LocalizedLabel::native("Set LOD Mode", "LOD-Modus festlegen"), ActionKind::View, "layers"))
            .action_interactive_job("runQuery", InteractiveJobClassification::Migrated)
            .action_interactive_job("loadExampleQuery", InteractiveJobClassification::Migrated)
            .action_interactive_job("nodeGraphViewport", InteractiveJobClassification::Migrated)
            .action_interactive_job("textEdit", InteractiveJobClassification::Migrated)
            .action_interactive_job("textSelect", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodMode", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchNodes", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("loadDocumentJson", InteractiveJobClassification::Migrated)
            .action_interactive_job("formatDocument", InteractiveJobClassification::Migrated)
            // 🕹️ Domain "ast": jack's document nodes, transitive over each node's first incoming
            // connection (see `interaction_topology`). Selection/hover, marquee, modes and merges are
            // ALL framework-injected now — no app-declared setSelection/graphPointerDown verbs.
            .interaction(InteractionDefinition {
                id: "ast".into(),
                label: LocalizedLabel::native("Nodes", "Knoten"),
                granularities: vec![GranularityDefinition { id: "node".into(), label: LocalizedLabel::native("Node", "Knoten"), icon_id: "circle".into() }],
                hierarchy: HierarchyProvider::Topology,
                hover: HoverSpec { transitive: true, ..HoverSpec::default() },
                selection: SelectionSpec { modes: vec![SelectionMode::Multiple, SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: true, broadcast: true },
            })
            .window_kind_interactions(TRINITY_JACK_PLAY_WINDOW_GRAPH, vec![InteractionRef::new("ast")])
            // 📇️ Per-window action scoping: `build_definition` copies an unowned action onto EVERY window
            // kind. Viewport/LOD read the config of the graph window they were dispatched from, so they
            // belong to the graph; the text verbs edit the document's query where it is typed, so they
            // belong to the query editor. Document verbs, selection, history and the example picker stay
            // unscoped on purpose — they read the document, not a pane.
            .window_kind_action_refs(TRINITY_JACK_PLAY_WINDOW_EDITOR, vec!["textEdit".into(), "textSelect".into(), "formatDocument".into()])
            .window_kind_action_refs(TRINITY_JACK_PLAY_WINDOW_GRAPH, vec!["nodeGraphViewport".into(), "setLodMode".into()])
            // 📝️ Staged argument forms for the panel-visible preset loaders. The option list IS the
            // subset's registered example set — the navbar picker only ever dispatches a registered id.
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Fixture", "Fixtur"), vec![
                    ActionArgOption::new(crate::examples::demo::ID, crate::examples::demo::label()),
                ]).required(),
            ])
            .action_args("patchNodes", vec![
                ActionArgDef::entity_ids("nodeIds", LocalizedLabel::native("Nodes (empty: selection)", "Knoten (leer: Auswahl)"), "ast", "node"),
                ActionArgDef::select("field", LocalizedLabel::native("Field", "Feld"), vec![ActionArgOption::new("name", LocalizedLabel::native("Name", "Name"))]).required(),
                ActionArgDef::text("value", LocalizedLabel::native("Value", "Wert")).required(),
            ])
            .action_args("runQuery", vec![
                ActionArgDef::text("query", LocalizedLabel::native("Query", "Abfrage")),
                ActionArgDef::text("resultsWindowId", LocalizedLabel::native("Results window", "Ergebnisfenster")).required(),
            ])
            .action_args("loadExampleQuery", vec![
                ActionArgDef::select("query", LocalizedLabel::native("Example", "Beispiel"), vec![
                    ActionArgOption::new("MATCH (a:Piece) WHERE a.name = 't_f0_b_c0' OR a.name = 't_f0_b_c1' RETURN a.name", LocalizedLabel::native("Where Or", "Wo-Oder")),
                    ActionArgOption::new("MATCH (a:Piece)-[r:Connection]->(b:Piece) WHERE a.name = 'b' RETURN a, r, b", LocalizedLabel::native("Return Graph", "Graph zurückgeben")),
                    ActionArgOption::new("MATCH (a:Piece) WHERE a.name = 'b' SET a.label = 'demo-label'", LocalizedLabel::native("Set Label", "Label setzen")),
                    ActionArgOption::new("MATCH (a:Piece) WHERE a.name = 'b' SET a.x = 300, a.y = 120", LocalizedLabel::native("Set Position", "Position setzen")),
                    ActionArgOption::new("CREATE (n:Piece)", LocalizedLabel::native("Create Node", "Knoten erstellen")),
                    ActionArgOption::new("MATCH (a:Piece), (b:Piece) WHERE a.name = 'b' AND b.name != 'b' CREATE (a)-[:Connection]->(b)", LocalizedLabel::native("Create Edge", "Kante erstellen")),
                    ActionArgOption::new("MATCH (n:Piece) WHERE n.name = 'b' DELETE n", LocalizedLabel::native("Delete Leaf", "Blatt löschen")),
                    ActionArgOption::new("MERGE (x:Piece)-[:Connection]->(y:Piece)", LocalizedLabel::native("Merge Edge", "Kante zusammenführen")),
                ]).required(),
                ActionArgDef::text("resultsWindowId", LocalizedLabel::native("Results window", "Ergebnisfenster")).required(),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("mod+alt+s", "commitCheckpoint")
            .io(jack_io())
            .action_describe("setLodMode", LocalizedLabel::native("Sets the level of detail the graph window draws the component graph with; only that window's view changes.", "Legt die Detailstufe fest, mit der das Graphfenster den Bauteilgraphen zeichnet; nur die Ansicht dieses Fensters ändert sich."))
            .action_describe("formatDocument", LocalizedLabel::native("Reformats the document's Jack query as one undoable edit; the graph is not changed, and a query that does not parse is left as it is.", "Formatiert die Jack-Abfrage des Dokuments als ein rückgängig machbarer Schritt neu; der Graph ändert sich nicht, eine nicht lesbare Abfrage bleibt wie sie ist."))
            .action_describe("deleteSelection", LocalizedLabel::native("Deletes every selected node of the component graph together with every edge attached to them.", "Löscht alle ausgewählten Knoten des Bauteilgraphen samt aller daran angeschlossenen Kanten."))
            .action_describe("patchNodes", LocalizedLabel::native("Renames the given nodes, or the selected ones when no ids are given; name is the only field a Jack node allows.", "Benennt die angegebenen Knoten um, ohne Ids die ausgewählten; der Name ist das einzige Feld, das ein Jack-Knoten erlaubt."))
            .action_describe("runQuery", LocalizedLabel::native("Runs a Jack graph query (or the document's own query) against the component graph and shows the matches in the given results window; CREATE, SET or DELETE clauses change the graph.", "Führt eine Jack-Graphabfrage (oder die eigene Abfrage des Dokuments) auf dem Bauteilgraphen aus und zeigt die Treffer im angegebenen Ergebnisfenster; CREATE-, SET- oder DELETE-Klauseln ändern den Graphen."))
            .action_describe("loadExampleQuery", LocalizedLabel::native("Makes one of the bundled example queries the document's query, runs it and shows its matches in the given results window.", "Macht eine der mitgelieferten Beispielabfragen zur Abfrage des Dokuments, führt sie aus und zeigt ihre Treffer im angegebenen Ergebnisfenster."))
            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole document with a bundled fixture (the Nakagin capsule tower or the branch chain) and its example query, by example id.", "Ersetzt das gesamte Dokument durch eine mitgelieferte Fixture (den Nakagin Capsule Tower oder die Astkette) samt Beispielabfrage, anhand der Beispiel-Id."))
            .action_describe("loadDocumentJson", LocalizedLabel::native("Replaces the whole component graph with one parsed from the given document JSON; invalid JSON changes nothing.", "Ersetzt den gesamten Bauteilgraphen durch einen, der aus dem angegebenen Dokument-JSON gelesen wird; ungültiges JSON ändert nichts."))
            .action_audience("nodeGraphViewport", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_destructive("runQuery")
            .build_definition()
}
//#endregion 🔖️Manifest
