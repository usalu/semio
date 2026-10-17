//! 📜️ The studio app manifest: play layout, modes, windows, panels, actions and the proof catalog.

use super::*;

//#region 🔖️SpaceManifest
async fn space_play_layout() -> WindowLayout {
    create_default_layout(
        &[
            semio_hub_space_foundation::S_PLAY_WINDOW_WORKFLOW.into(),
            crate::modes::main::windows::media_vfs::S_PLAY_WINDOW_MEDIA_VFS.into(),
            crate::modes::main::windows::compiled_dag::S_PLAY_WINDOW_COMPILED_DAG.into(),
        ],
        "row",
        Some(&[40.0, 30.0, 30.0]),
        Some(&["Workflow".into(), "Media VFS".into(), "Compiled DAG".into()]),
    )
}

pub async fn create_space_app() -> App {
    use crate::modes::main::windows::{compiled_dag, media_vfs, workflow};
    let builder = App::builder(S_PLAY_APP_ID, LocalizedLabel::native("Space", "Space")).await.document(["semio", "s", "studio"])
        .command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("setAppRegistrations", LocalizedLabel::native("Set App Registrations", "App-Registrierungen festlegen"), "host", ActionKind::View).with_args([ActionArgDef::text("json", LocalizedLabel::native("App Registrations", "App-Registrierungen"))]) }).await
        .icon_id("s").await
        .mode_def(crate::modes::main::definition().await).await
        .default_mode_id("main").await
        .window_kind_def(workflow::definition().await).await
        .window_kind_def(media_vfs::definition().await).await
        .window_kind_def(compiled_dag::definition().await).await
        .panel_tab_def(crate::panels::catalogue::definition().await).await
        .panel_tab_def(crate::panels::parameters::definition().await).await
        .panel_tab_def(crate::panels::inspection::definition().await).await
        .default_layout(space_play_layout().await).await
        .mutation("patchParameter", LocalizedLabel::native("Patch Parameter", "Parameter aktualisieren")).await
        .mutation("addParameter", LocalizedLabel::native("Add Parameter", "Parameter hinzufügen")).await
        .mutation("removeParameter", LocalizedLabel::native("Remove Parameter", "Parameter entfernen")).await
        .action_destructive("removeParameter").await
        .mutation("spawnApp", LocalizedLabel::native("Spawn App", "App erzeugen")).await
        .mutation("moveMediaNode", LocalizedLabel::native("Move Media Node", "Medienknoten verschieben")).await
        .mutation("connectMediaPorts", LocalizedLabel::native("Connect Media Ports", "Medien-Ports verbinden")).await
        .mutation("disconnectMediaEdge", LocalizedLabel::native("Disconnect Media Edge", "Medienverbindung trennen")).await
        .action_with(ActionDefinition::bounded_catalog("removeAppInstance", LocalizedLabel::native("Remove App Instance", "App-Instanz entfernen"), ActionKind::Mutation).with_category("selection")).await
        .action_destructive("removeAppInstance").await
        .mutation("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen")).await
        .action_destructive("deleteSelection").await
        .action_with(ActionDefinition::bounded_catalog("copyAppInstance", LocalizedLabel::native("Copy App Instance", "App-Instanz kopieren"), ActionKind::Mutation).with_category("transfer")).await
        .action_with(ActionDefinition::bounded_catalog("duplicateAppInstance", LocalizedLabel::native("Duplicate App Instance", "App-Instanz duplizieren"), ActionKind::Mutation).with_category("create")).await
        .action_with(ActionDefinition::bounded_catalog("pasteAppInstance", LocalizedLabel::native("Paste App Instance", "App-Instanz einfügen"), ActionKind::Mutation).with_category("transfer")).await
        .action_with(ActionDefinition::bounded_catalog("renameAppInstance", LocalizedLabel::native("Rename App Instance", "App-Instanz umbenennen"), ActionKind::Mutation).with_category("settings")).await
        .mutation("patchMediaNodes", LocalizedLabel::native("Patch Media Nodes", "Medienknoten aktualisieren")).await
        .mutation("patchAppInstances", LocalizedLabel::native("Patch App Instances", "App-Instanzen aktualisieren")).await
        .mutation("bindParameterField", LocalizedLabel::native("Bind Parameter Field", "Parameterfeld verknüpfen")).await
        .mutation("unbindParameterField", LocalizedLabel::native("Unbind Parameter Field", "Parameterfeld lösen")).await
        .action_with(ActionDefinition::bounded_catalog("reorganizeWorkflow", LocalizedLabel::native("Reorganize Workflow", "Workflow neu anordnen"), ActionKind::Mutation).with_category("transform")).await
        .mutation("workflowEngagementSubmit", LocalizedLabel::native("Workflow Engagement Submit", "Workflow-Eingabe bestätigen")).await
        .action_audience("workflowEngagementSubmit", semio_framework_plugin::CapabilityAudience::Input).await
        .mutation("compiledDagEngagementSubmit", LocalizedLabel::native("Compiled DAG Engagement Submit", "Kompilierter-DAG-Eingabe bestätigen")).await
        .action_audience("compiledDagEngagementSubmit", semio_framework_plugin::CapabilityAudience::Input).await
        .mutation("nodeGraphEdit", LocalizedLabel::native("Edit Workflow", "Workflow bearbeiten")).await
        // 🕹️ Selection/hover are the framework's `graph` interaction domain now (`.interaction(...)`
        // below) — the six framework verbs (`interactionSelect`/`interactionHover`/`clearSelection`/
        // `selectAll`/`setSelectionMode`/`setInteractionGranularity`) auto-inject.
        .action_with(ActionDefinition::new("setActivePanelTab", LocalizedLabel::native("Set Active Panel Tab", "Aktiven Panel-Tab festlegen"), ActionKind::View, "panel-left")).await
        .action_with(ActionDefinition::new("nodeGraphViewport", LocalizedLabel::native("Set Graph Viewport", "Graph-Ansichtsfenster festlegen"), ActionKind::View, "camera")).await
        .view_action("presenceHeartbeat", LocalizedLabel::native("Presence Heartbeat", "Anwesenheits-Heartbeat")).await
        .action_with(ActionDefinition::new("workflowEngagementInput", LocalizedLabel::native("Workflow Engagement Input", "Workflow-Eingabe"), ActionKind::View, "hand")).await
        .action_audience("workflowEngagementInput", semio_framework_plugin::CapabilityAudience::Input).await
        .action_with(ActionDefinition::new("compiledDagEngagementInput", LocalizedLabel::native("Compiled DAG Engagement Input", "Kompilierter-DAG-Eingabe"), ActionKind::View, "hand")).await
        .action_audience("compiledDagEngagementInput", semio_framework_plugin::CapabilityAudience::Input).await
        .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Shell, "panel-left")).await
        .action_with(ActionDefinition::new("exportMedia", LocalizedLabel::native("Export Media", "Medien exportieren"), ActionKind::Shell, "download")).await
        .action_with(ActionDefinition::new("importMedia", LocalizedLabel::native("Import Media", "Medien importieren"), ActionKind::Shell, "hard-drive")).await
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("importMediaPayload", LocalizedLabel::native("Import Media Payload", "Medien-Payload importieren"), ActionKind::Shell) }).await
        .action_with(ActionDefinition::new("exportStudioPack", LocalizedLabel::native("Export Studio Pack", "Studio-Paket exportieren"), ActionKind::Shell, "download")).await
        .action_with(ActionDefinition::new("exportStudioDsl", LocalizedLabel::native("Export Studio DSL", "Studio-DSL exportieren"), ActionKind::Shell, "download")).await
        .action_with(ActionDefinition::new("importSpacePack", LocalizedLabel::native("Import Studio Pack", "Studio-Paket importieren"), ActionKind::Shell, "hard-drive")).await
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("importSpacePackPayload", LocalizedLabel::native("Import Studio Pack Payload", "Studio-Paket-Payload importieren"), ActionKind::Shell) }).await
        .action_with(ActionDefinition::new("openSpace", LocalizedLabel::native("Open Studio", "Studio öffnen"), ActionKind::Shell, "folder-open")).await
        .action_with(ActionDefinition::new("openInstance", LocalizedLabel::native("Open Instance", "Instanz öffnen"), ActionKind::Shell, "folder-open").with_category("open")).await
        .shell_action("closeFocusedInstance", LocalizedLabel::native("Close Focused Instance", "Fokussierte Instanz schließen")).await
        .action_with(ActionDefinition::new("goHome", LocalizedLabel::native("Go Home", "Zur Startseite"), ActionKind::Shell, "home")).await
        .action_with(ActionDefinition::new("navigateVirtualFileSystemNode", LocalizedLabel::native("Navigate File System Node", "Dateisystemknoten navigieren"), ActionKind::Shell, "folder")).await
        .action_interactive_job("patchParameter", InteractiveJobClassification::Migrated).await
        .action_interactive_job("addParameter", InteractiveJobClassification::Migrated).await
        .action_interactive_job("removeParameter", InteractiveJobClassification::Migrated).await
        .action_interactive_job("spawnApp", InteractiveJobClassification::Migrated).await
        .action_interactive_job("moveMediaNode", InteractiveJobClassification::Migrated).await
        .action_interactive_job("connectMediaPorts", InteractiveJobClassification::Migrated).await
        .action_interactive_job("disconnectMediaEdge", InteractiveJobClassification::Migrated).await
        .action_interactive_job("removeAppInstance", InteractiveJobClassification::Migrated).await
        .action_interactive_job("deleteSelection", InteractiveJobClassification::Migrated).await
        .action_interactive_job("copyAppInstance", InteractiveJobClassification::Migrated).await
        .action_interactive_job("duplicateAppInstance", InteractiveJobClassification::Migrated).await
        .action_interactive_job("pasteAppInstance", InteractiveJobClassification::Migrated).await
        .action_interactive_job("renameAppInstance", InteractiveJobClassification::Migrated).await
        .action_interactive_job("patchMediaNodes", InteractiveJobClassification::Migrated).await
        .action_interactive_job("patchAppInstances", InteractiveJobClassification::Migrated).await
        .action_interactive_job("bindParameterField", InteractiveJobClassification::Migrated).await
        .action_interactive_job("unbindParameterField", InteractiveJobClassification::Migrated).await
        .action_interactive_job("reorganizeWorkflow", InteractiveJobClassification::Migrated).await
        .action_interactive_job("workflowEngagementSubmit", InteractiveJobClassification::Migrated).await
        .action_interactive_job("compiledDagEngagementSubmit", InteractiveJobClassification::Migrated).await
        .action_interactive_job("nodeGraphEdit", InteractiveJobClassification::Migrated).await
        .action_interactive_job("setActivePanelTab", InteractiveJobClassification::Migrated).await
        .action_interactive_job("nodeGraphViewport", InteractiveJobClassification::Migrated).await
        .action_interactive_job("presenceHeartbeat", InteractiveJobClassification::Migrated).await
        .action_interactive_job("workflowEngagementInput", InteractiveJobClassification::Migrated).await
        .action_interactive_job("compiledDagEngagementInput", InteractiveJobClassification::Migrated).await
        .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated).await
        .action_interactive_job("exportMedia", InteractiveJobClassification::Migrated).await
        .action_destructive("exportMedia").await
        .action_interactive_job("importMedia", InteractiveJobClassification::Migrated).await
        .action_interactive_job("importMediaPayload", InteractiveJobClassification::Migrated).await
        .action_interactive_job("exportStudioPack", InteractiveJobClassification::Migrated).await
        .action_destructive("exportStudioPack").await
        .action_interactive_job("exportStudioDsl", InteractiveJobClassification::Migrated).await
        .action_destructive("exportStudioDsl").await
        .action_interactive_job("importSpacePack", InteractiveJobClassification::Migrated).await
        .action_interactive_job("importSpacePackPayload", InteractiveJobClassification::Migrated).await
        .action_interactive_job("openSpace", InteractiveJobClassification::Migrated).await
        .action_interactive_job("openInstance", InteractiveJobClassification::Migrated).await
        .action_interactive_job("closeFocusedInstance", InteractiveJobClassification::Migrated).await
        .action_interactive_job("goHome", InteractiveJobClassification::Migrated).await
        .action_interactive_job("navigateVirtualFileSystemNode", InteractiveJobClassification::Migrated).await
        .action_interactive_job("setAppRegistrations", InteractiveJobClassification::Migrated).await
        // 📝️ Staged argument form for parameter creation (spawnApp stays context-driven; exportMedia requires an explicit artifact document).
        .action_args("exportMedia", vec![
            ActionArgDef::text("nodeId", LocalizedLabel::native("App Instance", "App-Instanz")),
            ActionArgDef::text("format", LocalizedLabel::native("Format", "Format")),
            ActionArgDef::text("documentJson", LocalizedLabel::native("Artifact Document", "Artefaktdokument")),
        ]).await
        .action_args("addParameter", vec![
            ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).default_value(&"Parameter"),
            ActionArgDef::select("type", LocalizedLabel::native("Type", "Typ"), vec![
                ActionArgOption::new("numeric", LocalizedLabel::native("Numeric", "Numerisch")),
                ActionArgOption::new("categorical", LocalizedLabel::native("Categorical", "Kategorisch")),
                ActionArgOption::new("toggle", LocalizedLabel::native("Toggle", "Schalter")),
                ActionArgOption::new("text", LocalizedLabel::native("Text", "Text")),
            ]).default_value(&"numeric"),
        ]).await
        // 📇️ Per-window action scoping — the Workflow (NodeGraph) window owns all graph/instance/
        // parameter editing plus the per-instance media import/export; the Media VFS
        // (VirtualFileSystem) window only navigates the media file tree; the read-only Compiled DAG
        // window only drives its own engagement. Navigation, panel-tab, presence, example and generic
        // node-graph view actions stay unscoped orphans and appear on every window.
        .window_kind_action_refs(semio_hub_space_foundation::S_PLAY_WINDOW_WORKFLOW, vec![
            "patchParameter".into(), "addParameter".into(), "removeParameter".into(),
            "spawnApp".into(), "moveMediaNode".into(), "connectMediaPorts".into(), "disconnectMediaEdge".into(),
            "removeAppInstance".into(), "deleteSelection".into(), "copyAppInstance".into(),
            "duplicateAppInstance".into(), "pasteAppInstance".into(), "renameAppInstance".into(),
            "patchMediaNodes".into(), "patchAppInstances".into(), "bindParameterField".into(),
            "unbindParameterField".into(), "reorganizeWorkflow".into(), "workflowEngagementSubmit".into(),
            "workflowEngagementInput".into(), "nodeGraphEdit".into(), "exportMedia".into(),
            "importMedia".into(), "importMediaPayload".into(),
        ]).await
        .window_kind_action_refs(media_vfs::S_PLAY_WINDOW_MEDIA_VFS, vec![
            "navigateVirtualFileSystemNode".into(),
        ]).await
        .window_kind_action_refs(compiled_dag::S_PLAY_WINDOW_COMPILED_DAG, vec![
            "compiledDagEngagementSubmit".into(), "compiledDagEngagementInput".into(),
        ]).await
        // 🕹️ First-class hover/selection (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
        // one domain over the workflow node graph, "instance"/"media-node" granularities (a node IS
        // the app instance now — see the kernel `🔁️workflow` crate's `🔖️InstanceIdentity` doc),
        // `HierarchyProvider::Topology` (see `SpaceApp::interaction_topology` above) — the node graph
        // itself is flat (no parent/child structure), so the topology exists purely so a deleted
        // node's id auto-prunes out of `graph`'s selection.
        .interaction(InteractionDefinition {
            id: S_PLAY_INTERACTION_DOMAIN.into(),
            label: LocalizedLabel::native("Graph", "Graph"),
            granularities: vec![
                GranularityDefinition { id: "instance".into(), label: LocalizedLabel::native("Instance", "Instanz"), icon_id: "box".into() },
                GranularityDefinition { id: "media-node".into(), label: LocalizedLabel::native("Media Node", "Medienknoten"), icon_id: "circle".into() },
            ],
            hierarchy: HierarchyProvider::Topology,
            hover: HoverSpec::default(),
            selection: SelectionSpec {
                modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                methods: vec![SelectionMethod::Pick],
                merges: vec![MergeMode::Replace],
                transitive: false,
                broadcast: true,
            },
        }).await
        .window_kind_interactions(semio_hub_space_foundation::S_PLAY_WINDOW_WORKFLOW, vec![InteractionRef::new(S_PLAY_INTERACTION_DOMAIN)]).await
        .keybinding("mod+z", "undo").await
        .keybinding("mod+shift+z", "redo").await
        .action_describe("patchParameter", LocalizedLabel::native("Sets one field (such as name, value or range) of one studio parameter by id; the value may be JSON or plain text.", "Setzt ein Feld (etwa Name, Wert oder Bereich) eines Studioparameters anhand seiner Id; der Wert darf JSON oder Klartext sein.")).await
        .action_describe("addParameter", LocalizedLabel::native("Adds a studio parameter with the given name and type (numeric, categorical, toggle or text) that app fields can be bound to.", "Fügt einen Studioparameter mit dem angegebenen Namen und Typ (Zahl, Auswahl, Schalter oder Text) hinzu, an den App-Felder gebunden werden können.")).await
        .action_describe("removeParameter", LocalizedLabel::native("Removes one studio parameter by id together with its field bindings.", "Entfernt einen Studioparameter anhand seiner Id samt seiner Feldbindungen.")).await
        .action_describe("spawnApp", LocalizedLabel::native("Adds an instance of a plugin app (plugin id and app id) to the studio workflow at x, y and makes it the active instance.", "Fügt dem Studio-Workflow an x, y eine Instanz einer Plugin-App (Plugin-Id und App-Id) hinzu und macht sie zur aktiven Instanz.")).await
        .action_describe("moveMediaNode", LocalizedLabel::native("Moves one app instance to the workflow canvas position x, y.", "Verschiebt eine App-Instanz an die Position x, y der Workflow-Fläche.")).await
        .action_describe("connectMediaPorts", LocalizedLabel::native("Connects a media output port of one app instance to a media input port of another, so its output flows into the second.", "Verbindet einen Medienausgang einer App-Instanz mit einem Medieneingang einer anderen, sodass deren Ausgabe in die zweite fließt.")).await
        .action_describe("disconnectMediaEdge", LocalizedLabel::native("Removes one media connection between two app instances by id.", "Entfernt eine Medienverbindung zwischen zwei App-Instanzen anhand ihrer Id.")).await
        .action_describe("removeAppInstance", LocalizedLabel::native("Removes one app instance (the given one, or the selected one) from the workflow with its connections.", "Entfernt eine App-Instanz (die angegebene oder die ausgewählte) samt ihrer Verbindungen aus dem Workflow.")).await
        .action_describe("deleteSelection", LocalizedLabel::native("Removes every selected app instance and connection from the studio workflow.", "Entfernt alle ausgewählten App-Instanzen und Verbindungen aus dem Studio-Workflow.")).await
        .action_describe("copyAppInstance", LocalizedLabel::native("Copies the selected app instances to the studio clipboard for a later paste; the workflow is not changed.", "Kopiert die ausgewählten App-Instanzen für ein späteres Einfügen in die Studio-Zwischenablage; der Workflow ändert sich nicht.")).await
        .action_describe("duplicateAppInstance", LocalizedLabel::native("Adds a copy of each selected app instance next to it, labelled with Copy.", "Fügt neben jeder ausgewählten App-Instanz eine mit Kopie beschriftete Kopie hinzu.")).await
        .action_describe("pasteAppInstance", LocalizedLabel::native("Adds a copy of every app instance on the studio clipboard to the workflow, offset from the original.", "Fügt dem Workflow eine versetzte Kopie jeder App-Instanz aus der Studio-Zwischenablage hinzu.")).await
        .action_describe("renameAppInstance", LocalizedLabel::native("Renames one app instance of the workflow.", "Benennt eine App-Instanz des Workflows um.")).await
        .action_describe("patchMediaNodes", LocalizedLabel::native("Sets one field (such as the position along an axis) on several app instances at once.", "Setzt ein Feld (etwa die Position entlang einer Achse) auf mehreren App-Instanzen zugleich.")).await
        .action_describe("patchAppInstances", LocalizedLabel::native("Sets the label of several app instances at once.", "Setzt die Beschriftung mehrerer App-Instanzen zugleich.")).await
        .action_describe("bindParameterField", LocalizedLabel::native("Binds one field of an app instance (by field path) to a studio parameter, so the parameter's value drives it; an empty parameter unbinds it.", "Bindet ein Feld einer App-Instanz (per Feldpfad) an einen Studioparameter, sodass dessen Wert es steuert; ein leerer Parameter löst die Bindung.")).await
        .action_describe("unbindParameterField", LocalizedLabel::native("Removes the parameter binding from one field of an app instance, so the field keeps its own value again.", "Entfernt die Parameterbindung von einem Feld einer App-Instanz, sodass das Feld wieder seinen eigenen Wert behält.")).await
        .action_describe("reorganizeWorkflow", LocalizedLabel::native("Lays out every app instance of the workflow automatically, overwriting their manual positions.", "Ordnet alle App-Instanzen des Workflows automatisch an und überschreibt ihre manuellen Positionen.")).await
        .action_describe("exportMedia", LocalizedLabel::native("Writes the media one app instance produces in the chosen format to a downloaded file on the user's machine.", "Schreibt die Medien, die eine App-Instanz erzeugt, im gewählten Format in eine heruntergeladene Datei auf dem Rechner des Nutzers.")).await
        .action_describe("importMedia", LocalizedLabel::native("Opens the host's file picker for a file of the given format; the chosen file is then imported into the given app instance.", "Öffnet die Dateiauswahl des Hosts für eine Datei des angegebenen Formats; die gewählte Datei wird dann in die angegebene App-Instanz importiert.")).await
        .action_describe("importMediaPayload", LocalizedLabel::native("Imports a picked file's content (data URL) into the app instance that Import Media was started for.", "Importiert den Inhalt einer gewählten Datei (Data-URL) in die App-Instanz, für die Medien importieren gestartet wurde.")).await
        .action_describe("setActivePanelTab", LocalizedLabel::native("Switches the studio's side panel to the given tab; only the view changes.", "Schaltet das Seitenpanel des Studios auf den angegebenen Reiter um; nur die Ansicht ändert sich.")).await
        .action_describe("setActiveExample", LocalizedLabel::native("Opens one of the bundled example studios by navigating the shell to it; the current studio is not changed.", "Öffnet eines der mitgelieferten Beispielstudios, indem die Shell dorthin navigiert; das aktuelle Studio ändert sich nicht.")).await
        .action_describe("exportStudioPack", LocalizedLabel::native("Writes the whole studio as a .pack file and an .ops event log to two downloaded files on the user's machine.", "Schreibt das gesamte Studio als .pack-Datei und .ops-Ereignisprotokoll in zwei heruntergeladene Dateien auf dem Rechner des Nutzers.")).await
        .action_describe("exportStudioDsl", LocalizedLabel::native("Writes the whole studio as DSL text to a downloaded file on the user's machine.", "Schreibt das gesamte Studio als DSL-Text in eine heruntergeladene Datei auf dem Rechner des Nutzers.")).await
        .action_describe("importSpacePack", LocalizedLabel::native("Opens the host's file picker for a studio .pack file; the chosen file is then imported.", "Öffnet die Dateiauswahl des Hosts für eine Studio-.pack-Datei; die gewählte Datei wird dann importiert.")).await
        .action_describe("importSpacePackPayload", LocalizedLabel::native("Imports a studio from the content (data URL) of a picked .pack file.", "Importiert ein Studio aus dem Inhalt (Data-URL) einer gewählten .pack-Datei.")).await
        .action_describe("openInstance", LocalizedLabel::native("Opens the given or selected app instance in its own window so its artifact can be viewed or edited.", "Öffnet die angegebene oder ausgewählte App-Instanz in einem eigenen Fenster, damit ihr Artefakt angesehen oder bearbeitet werden kann.")).await
        .action_describe("closeFocusedInstance", LocalizedLabel::native("Closes the app instance window that is currently focused; the workflow is not changed.", "Schließt das gerade fokussierte App-Instanz-Fenster; der Workflow ändert sich nicht.")).await
        .action_describe("openSpace", LocalizedLabel::native("Opens the studio with the given id, navigating the shell to it.", "Öffnet das Studio mit der angegebenen Id und navigiert die Shell dorthin.")).await
        .action_describe("navigateVirtualFileSystemNode", LocalizedLabel::native("Navigates the shell to the space with the given id.", "Navigiert die Shell zum Space mit der angegebenen Id.")).await
        .action_describe("goHome", LocalizedLabel::native("Navigates the shell back to the Home launcher.", "Navigiert die Shell zurück zum Home-Starter.")).await
        .action_audience("nodeGraphEdit", semio_framework_plugin::CapabilityAudience::Input).await
        .action_audience("nodeGraphViewport", semio_framework_plugin::CapabilityAudience::Chrome).await
        .action_audience("presenceHeartbeat", semio_framework_plugin::CapabilityAudience::Chrome).await
        .action_destructive("reorganizeWorkflow").await
        .keybinding("mod+s", "commitCheckpoint").await;
    let definition = builder.build_definition();
    let app = App { definition, examples: Vec::new() };
    let mut app = app.workflow("s", "S Studio", "studio").await;
    for (id, label) in S_STUDIO_EXAMPLES {
        // 🚧️ `OsWorkflowArtifactDocument` (= `BackboneDocument<WorkflowSnapshot, WorkflowMutation>`)
        // is deeply framework-owned (`ArtifactVcs`/`ArtifactCursor`/`Edit`/`Conflict`/...) and still
        // only derives `Serialize`, not `ToValue` — bridging the whole envelope is out of this
        // ticket's plugin-slice scope (tracked with the in-flight `BackboneDocument: ToValue`
        // framework work). The example payload only ever needed the bare `WorkflowSnapshot` anyway
        // (`parse_demo_space_document`'s own doc: "the fixture holds only the `WorkflowSnapshot`
        // payload"), which already derives `ToValue` — read the immutable genesis snapshot.
        let document = parse_demo_space_document().await;
        let document_value = semio_framework_value::ToValue::to_value(document.vcs.genesis.facts().snapshot());
        let json = semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&document_value));
        // 📊️ `label` is sourced from `S_STUDIO_EXAMPLES` — no per-locale split is available at the
        // source, so it is genuine runtime data here, not compile-checked native copy.
        app = app.example(*id, LocalizedLabel::data(*label), json, "file-text").await;
    }
    app
}
//#endregion 🔖️SpaceManifest
