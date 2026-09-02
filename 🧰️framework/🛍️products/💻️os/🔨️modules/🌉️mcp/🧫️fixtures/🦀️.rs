//! 🧫️ Catalog compiler test fixtures — packet `P2-catalog`. `manifest::PackageDescriptor`
//! construction has **zero real call sites anywhere in the repo yet** (its own doc comment:
//! "Nothing constructs or reads one yet in this packet" — `🛂️manifest/🦀️.rs` `🔖️PackageDescriptor`,
//! landed by the peer ticket's A2/A3, still additive-only). Depending on the real `🗒️note`/`📐️cad`
//! plugin crates as dev-dependencies to source one was considered and rejected: both crates live
//! under `✏️s/🔌️plugins/**`, territory the peer `MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME` ticket holds
//! exclusive through its A2/M0-M8 packets (`📌️important.md`'s collision matrix, "wait G3"), and
//! `cad`/`note` transitively pull in `semio-framework-plugin(-host)`, which `📓️terra-P3-report.md`
//! §7 already found does not compile workspace-wide right now (108 errors, the peer's in-flight WIT
//! rewrite). Pulling either crate in — even as a dev-dependency — would make THIS crate's own build
//! hostage to that unrelated, unfinished rewrite, exactly what `📓️terra-P1a-report.md` §5's
//! zero-plugin-host-dependency guarantee exists to prevent. Per this packet's brief §2.5 ("otherwise
//! construct them from the real `AppDefinition` builders in a test and say exactly how"): every
//! action id, `ActionKind`, and declared-arg list below is transcribed VERBATIM from
//! `📓️luna-actions-audit.md` §5 (cad's 41-row table, note's 36-row list — both audited directly
//! against the real plugin source, shasum-pinned there), constructed here through the real,
//! plugin-crate-independent `semio_framework::manifest::{ActionDefinition, ActionArgDef, AppDefinition,
//! …}` builder API — the same API every real plugin crate itself calls.

use crate::catalog::CatalogSource;
use crate::conformance::EvalCase;
use semio_framework::manifest::{
    self, ActionArgDef, ActionArgOption, ActionDefinition, ActionKind, AppDefinition, AppRole, ArgSchema, ContributionSet, ExecutionMode, ModeDefinition, Modes, PackageDescriptor, PackageHashes, PackageRole, UtilityDefinition, WindowKindDefinition,
    WindowKinds,
};
use semio_framework::{ArtifactDialect, IconName};
use semio_framework_ui::wgpu::{LocalizedLabel, SurfaceKind};

//#region 🔖️Helpers
fn action(id: &str, en: &str, de: &str, kind: ActionKind) -> ActionDefinition {
    ActionDefinition::bounded_catalog(id, LocalizedLabel::native(en, de), kind)
}

fn string_array_arg(id: &str, en: &str, de: &str) -> ActionArgDef {
    ActionArgDef {
        id: id.to_string(),
        label: LocalizedLabel::native(en, de),
        schema: ArgSchema::Array { items: Box::new(ArgSchema::String { options: Vec::new(), min_len: None, max_len: None, pattern: None, format: None }), min_items: None, max_items: None },
        presentation: None,
        required: false,
        default: None,
        description: None,
    }
}

fn number_arg(id: &str, en: &str, de: &str) -> ActionArgDef {
    ActionArgDef::number(id, LocalizedLabel::native(en, de)).default_value(0.0)
}

fn empty_hashes() -> PackageHashes {
    PackageHashes { wasm_sha256: String::new(), core_wasm_sha256: String::new(), descriptor_sha256: String::new() }
}

fn wrap_descriptor(manifest: manifest::PluginManifest) -> PackageDescriptor {
    PackageDescriptor {
        descriptor_version: 1,
        role: PackageRole::Plugin,
        manifest,
        activation_events: Vec::new(),
        capability_requests: Vec::new(),
        extension_points: Vec::new(),
        execution: ExecutionMode::Isolated,
        quotas: Default::default(),
        contributions: ContributionSet::default(),
        assets: Vec::new(),
        hashes: empty_hashes(),
    }
}
//#endregion 🔖️Helpers

//#region 🔖️CadFixture
/// 📐️ Every action id in `📓️luna-actions-audit.md` §5's "All 38 CAD Action IDs" table (41 rows as
/// literally enumerated there — the section header undercounts by 3 against its own body, noted
/// as-is), minus row 34 (`setActiveUtility`, framework-injected — triggered instead via
/// `app.utilities` below, never app-declared).
fn cad_actions() -> Vec<ActionDefinition> {
    vec![
        action("addObject", "Add Object", "Objekt hinzufügen", ActionKind::Mutation).use_when(["add a new object", "create an object", "ein objekt hinzufügen"]),
        action("patchObject", "Patch Object", "Objekt anpassen", ActionKind::Mutation),
        action("patchSelection", "Patch Selection", "Auswahl anpassen", ActionKind::Mutation),
        action("deleteObject", "Delete Object", "Objekt löschen", ActionKind::Mutation).use_when(["delete an object", "remove an object", "ein objekt löschen"]),
        action("duplicateObject", "Duplicate Object", "Objekt duplizieren", ActionKind::Mutation).use_when(["duplicate the object", "copy the object", "objekt duplizieren"]),
        action("addNode", "Add Node", "Knoten hinzufügen", ActionKind::Mutation).use_when(["add a node to the model", "einen knoten hinzufügen"]),
        action("renameNode", "Rename Node", "Knoten umbenennen", ActionKind::Mutation).use_when(["rename a node", "einen knoten umbenennen"]),
        action("translateSelection", "Translate Selection", "Auswahl verschieben", ActionKind::Mutation)
            .use_when(["move the selection", "translate the selected objects", "shift the selection", "die auswahl verschieben", "die ausgewählten objekte verschieben"])
            .with_args([string_array_arg("objectIds", "Object IDs", "Objekt-IDs"), number_arg("dx", "Delta X", "Delta X"), number_arg("dy", "Delta Y", "Delta Y"), number_arg("dz", "Delta Z", "Delta Z")]),
        action("rotateSelection", "Rotate Selection", "Auswahl drehen", ActionKind::Mutation).use_when(["rotate the selection", "turn the selected objects", "die auswahl drehen"]).with_args([
            string_array_arg("objectIds", "Object IDs", "Objekt-IDs"),
            number_arg("ax", "Axis X", "Achse X"),
            number_arg("ay", "Axis Y", "Achse Y"),
            number_arg("az", "Axis Z", "Achse Z"),
            number_arg("angle", "Angle", "Winkel"),
        ]),
        action("scaleSelection", "Scale Selection", "Auswahl skalieren", ActionKind::Mutation).use_when(["scale the selection", "resize the selected objects", "die auswahl skalieren"]).with_args([
            string_array_arg("objectIds", "Object IDs", "Objekt-IDs"),
            number_arg("sx", "Scale X", "Skalierung X"),
            number_arg("sy", "Scale Y", "Skalierung Y"),
            number_arg("sz", "Scale Z", "Skalierung Z"),
        ]),
        action("applyTransformation", "Apply Transformation", "Transformation anwenden", ActionKind::Mutation).use_when(["apply a transformation", "eine transformation anwenden"]),
        action("importCadFile", "Import CAD File", "CAD-Datei importieren", ActionKind::Mutation).use_when(["import a cad file", "eine cad-datei importieren"]),
        action("patchCadPlayReference", "Patch Play Reference", "Play-Referenz anpassen", ActionKind::Mutation),
        action("engagementSubmit", "Submit Engagement", "Eingabe abschließen", ActionKind::Mutation),
        action("focusModelDefinition", "Focus Model Definition", "Modelldefinition fokussieren", ActionKind::Mutation).use_when(["focus a model definition", "eine modelldefinition fokussieren"]).with_args([ActionArgDef::select(
            "modelDefinitionId",
            LocalizedLabel::native("Model Definition", "Modelldefinition"),
            vec![ActionArgOption::new("primary", LocalizedLabel::native("Primary", "Primär")), ActionArgOption::new("secondary", LocalizedLabel::native("Secondary", "Sekundär"))],
        )
        .required()]),
        action("setActiveExample", "Set Active Example", "Aktives Beispiel festlegen", ActionKind::Mutation).use_when(["load an example", "open a demo model", "ein beispiel laden"]).with_args([ActionArgDef::select(
            "exampleId",
            LocalizedLabel::native("Example", "Beispiel"),
            vec![ActionArgOption::new("empty", LocalizedLabel::native("Empty", "Leer")), ActionArgOption::new("demo", LocalizedLabel::native("Demo", "Demo")), ActionArgOption::new("capsule", LocalizedLabel::native("Capsule", "Kapsel"))],
        )
        .required()]),
        action("worldPointerDown", "World Pointer Down", "Zeiger gedrückt", ActionKind::View).in_palette(false),
        action("setCamera", "Set Camera", "Kamera festlegen", ActionKind::View).use_when(["change the camera view", "die kameraansicht ändern"]),
        action("setProjection", "Set Projection", "Projektion festlegen", ActionKind::View).use_when(["switch the projection", "die projektion wechseln"]),
        action("setProjectionParam", "Set Projection Parameter", "Projektionsparameter festlegen", ActionKind::View),
        action("setDislocateOption", "Set Dislocate Option", "Verlagerungsoption festlegen", ActionKind::View),
        action("setNodeSelection", "Set Node Selection", "Knotenauswahl festlegen", ActionKind::View).with_args([string_array_arg("nodeIds", "Node IDs", "Knoten-IDs")]),
        action("setReferenceSelection", "Set Reference Selection", "Referenzauswahl festlegen", ActionKind::View),
        action("referenceHover", "Reference Hover", "Referenz-Hover", ActionKind::View).in_palette(false),
        action("engagementInput", "Engagement Input", "Interaktionseingabe", ActionKind::View).in_palette(false),
        action("engagementPossibleSelect", "Engagement Possible Select", "Mögliche Auswahl", ActionKind::View).in_palette(false),
        action("engagementRepeatLast", "Repeat Last Engagement", "Letzte Interaktion wiederholen", ActionKind::View).in_palette(false),
        action("engagementAbort", "Abort Engagement", "Interaktion abbrechen", ActionKind::View).in_palette(false),
        action("worldPointerMove", "World Pointer Move", "Zeiger bewegt", ActionKind::View).in_palette(false),
        action("toggleSun", "Toggle Sun", "Sonne umschalten", ActionKind::View).use_when(["toggle the sun", "die sonne umschalten"]),
        action("setSunAzimuth", "Set Sun Azimuth", "Sonnenazimut festlegen", ActionKind::View),
        action("setSunElevation", "Set Sun Elevation", "Sonnenhöhe festlegen", ActionKind::View),
        action("setSunIntensity", "Set Sun Intensity", "Sonnenintensität festlegen", ActionKind::View).use_when(["change the sun intensity", "die sonnenintensität ändern"]),
        action("setLocale", "Set Locale", "Sprache festlegen", ActionKind::View),
        action("setTerminology", "Set Terminology", "Terminologie festlegen", ActionKind::View),
        action("setContributions", "Set Contributions", "Beiträge festlegen", ActionKind::View).in_palette(false),
        action("saveSelected", "Save Selected", "Auswahl speichern", ActionKind::Shell).use_when(["save the selected objects", "die ausgewählten objekte speichern"]),
        action("saveInPlay", "Save In Play", "Im Play speichern", ActionKind::Shell),
        action("saveCurrent", "Save Current", "Aktuelles speichern", ActionKind::Shell).use_when(["export the model", "save the current model", "das modell exportieren"]).with_args([ActionArgDef::select(
            "format",
            LocalizedLabel::native("Format", "Format"),
            vec![ActionArgOption::new("step", LocalizedLabel::native("STEP", "STEP")), ActionArgOption::new("obj", LocalizedLabel::native("OBJ", "OBJ")), ActionArgOption::new("stl", LocalizedLabel::native("STL", "STL"))],
        )
        .required()]),
        action("loadRawRequest", "Load Raw Request", "Rohdaten laden", ActionKind::Shell),
    ]
}

pub fn cad_app() -> AppDefinition {
    let dialect = ArtifactDialect { artifact_kind: "s.cad".to_string(), standard: "1".to_string(), subset: "any".to_string() };
    AppDefinition {
        id: "editor".to_string(),
        role: AppRole::Editor,
        dialect,
        label: LocalizedLabel::native("CAD", "CAD"),
        breadcrumb: vec!["CAD".to_string()],
        icon_id: None,
        controller_id: "cad".to_string(),
        modes: Modes::one(ModeDefinition { id: "edit".to_string(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: IconName::from("pencil"), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
        default_mode_id: "edit".to_string(),
        window_kinds: WindowKinds::one(WindowKindDefinition {
            id: "viewport".to_string(),
            label: LocalizedLabel::native("Viewport", "Ansicht"),
            body_key: "viewport".to_string(),
            surface_kind: SurfaceKind::World3d,
            icon_id: IconName::from("box"),
            options: Default::default(),
            actions: cad_actions(),
            utilities: Vec::new(),
            interactions: Vec::new(),
            params_schema: None,
            artifact_snapshot_schema: None,
            input_event_schema: None,
            output_schema: None,
            capabilities: Vec::new(),
        }),
        panel_tabs: Vec::new(),
        keybindings: Vec::new(),
        utilities: vec![UtilityDefinition::new("select", LocalizedLabel::native("Select", "Auswählen"), IconName::from("mouse-pointer"))],
        tools: Vec::new(),
        commands: Vec::new(),
        interactions: Vec::new(),
        named_layouts: Vec::new(),
        default_layout: None,
        terminologies: Vec::new(),
        terminology_breadcrumbs: Default::default(),
        introduction: None,
        tutorials: Vec::new(),
        dialogs: Vec::new(),
        media_inputs: Vec::new(),
        media_outputs: Vec::new(),
        artifact_kinds: Vec::new(),
        config: Default::default(),
        command_grammar: Default::default(),
        io: Default::default(),
    }
}

pub fn cad_descriptor() -> PackageDescriptor {
    wrap_descriptor(manifest::PluginManifest {
        plugin_id: "cad".to_string(),
        label: "CAD".to_string(),
        version: "0.1.0".to_string(),
        apps: vec![cad_app()],
        examples: Vec::new(),
        capabilities: Vec::new(),
        topic_contributions: Vec::new(),
        commands: Vec::new(),
        artifact_kinds: Vec::new(),
        dependencies: Vec::new(),
        contributions: Vec::new(),
    })
}
//#endregion 🔖️CadFixture

//#region 🔖️NoteFixture
/// 🗒️ Every action id in `📓️luna-actions-audit.md` §5's "All 36 Declared Action IDs" list for note —
/// minus row 33 (`setActiveUtility`, framework-injected). Per D2, note declares **zero** manifest
/// args on any action (engagement-driven UX) — the audit's own finding, preserved exactly here.
fn note_actions() -> Vec<ActionDefinition> {
    vec![
        action("setGridVisible", "Set Grid Visible", "Raster sichtbar", ActionKind::Mutation).use_when(["toggle the grid", "show the grid", "das raster umschalten"]),
        action("setGridSpacing", "Set Grid Spacing", "Rasterabstand festlegen", ActionKind::Mutation).use_when(["change the grid spacing", "den rasterabstand ändern"]),
        action("setGridSubdivisions", "Set Grid Subdivisions", "Rasterunterteilungen festlegen", ActionKind::Mutation),
        action("setGridOpacity", "Set Grid Opacity", "Rastertransparenz festlegen", ActionKind::Mutation),
        action("setSnapEnabled", "Set Snap Enabled", "Einrasten aktivieren", ActionKind::Mutation).use_when(["enable snapping", "turn on snap", "einrasten aktivieren"]),
        action("setSnapGridSpacing", "Set Snap Grid Spacing", "Einrastabstand festlegen", ActionKind::Mutation),
        action("setPencilWidth", "Set Pencil Width", "Stiftbreite festlegen", ActionKind::Mutation).use_when(["change the pencil width", "die stiftbreite ändern"]),
        action("setEraserRadius", "Set Eraser Radius", "Radiergummi-Radius festlegen", ActionKind::Mutation).use_when(["change the eraser size", "die radiergröße ändern"]),
        action("addBlock", "Add Block", "Block hinzufügen", ActionKind::Mutation).use_when(["add a block", "insert a new block", "einen block hinzufügen"]),
        action("moveBlock", "Move Block", "Block verschieben", ActionKind::Mutation).use_when(["move a block", "einen block verschieben"]),
        action("deleteBlock", "Delete Block", "Block löschen", ActionKind::Mutation).use_when(["delete a block", "einen block löschen"]),
        action("deleteSelection", "Delete Selection", "Auswahl löschen", ActionKind::Mutation).use_when(["delete the selection", "remove the selected items", "die auswahl löschen"]),
        action("duplicateBlock", "Duplicate Block", "Block duplizieren", ActionKind::Mutation).use_when(["duplicate a block", "einen block duplizieren"]),
        action("duplicateSelection", "Duplicate Selection", "Auswahl duplizieren", ActionKind::Mutation).use_when(["duplicate the selection", "copy the selected items", "die auswahl duplizieren"]),
        action("patchBlocks", "Patch Blocks", "Blöcke anpassen", ActionKind::Mutation),
        action("setActiveExample", "Set Active Example", "Aktives Beispiel festlegen", ActionKind::Mutation).use_when(["load a note example", "open a sketch template", "eine notizvorlage laden"]),
        action("setFixtureJson", "Set Fixture JSON", "Fixture-JSON festlegen", ActionKind::Mutation).in_palette(false),
        action("inkApplyEvents", "Apply Ink Events", "Zeichenereignisse anwenden", ActionKind::Mutation).in_palette(false),
        action("engagementSubmit", "Submit Engagement", "Eingabe abschließen", ActionKind::Mutation),
        action("nudgeSelection", "Nudge Selection", "Auswahl anstoßen", ActionKind::Mutation),
        action("nudgeSelectionUp", "Nudge Selection Up", "Auswahl nach oben schieben", ActionKind::Mutation).use_when(["nudge the selection up", "move the selection up a step", "die auswahl nach oben schieben"]),
        action("nudgeSelectionDown", "Nudge Selection Down", "Auswahl nach unten schieben", ActionKind::Mutation),
        action("nudgeSelectionLeft", "Nudge Selection Left", "Auswahl nach links schieben", ActionKind::Mutation),
        action("nudgeSelectionRight", "Nudge Selection Right", "Auswahl nach rechts schieben", ActionKind::Mutation),
        action("nudgeSelectionUpFast", "Nudge Selection Up Fast", "Auswahl schnell nach oben schieben", ActionKind::Mutation),
        action("nudgeSelectionDownFast", "Nudge Selection Down Fast", "Auswahl schnell nach unten schieben", ActionKind::Mutation),
        action("nudgeSelectionLeftFast", "Nudge Selection Left Fast", "Auswahl schnell nach links schieben", ActionKind::Mutation),
        action("nudgeSelectionRightFast", "Nudge Selection Right Fast", "Auswahl schnell nach rechts schieben", ActionKind::Mutation),
        action("engagementInput", "Engagement Input", "Interaktionseingabe", ActionKind::View).in_palette(false),
        action("navigatorEngagementInput", "Navigator Engagement Input", "Navigator-Interaktionseingabe", ActionKind::View).in_palette(false),
        action("setCamera", "Set Camera", "Kamera festlegen", ActionKind::View).use_when(["change the camera view", "die kameraansicht ändern"]),
        action("setCameraZoom", "Set Camera Zoom", "Kamerazoom festlegen", ActionKind::View),
        action("setLocale", "Set Locale", "Sprache festlegen", ActionKind::View),
        action("saveDownload", "Save Download", "Als Download speichern", ActionKind::Shell).use_when(["download the note", "save the note as a file", "die notiz herunterladen"]),
        action("loadRequest", "Load Request", "Ladevorgang anfordern", ActionKind::Shell).use_when(["open a note file", "load a file", "eine notizdatei öffnen"]),
    ]
}

pub fn note_app() -> AppDefinition {
    let dialect = ArtifactDialect { artifact_kind: "s.note".to_string(), standard: "1".to_string(), subset: "any".to_string() };
    AppDefinition {
        id: "editor".to_string(),
        role: AppRole::Editor,
        dialect,
        label: LocalizedLabel::native("Note", "Notiz"),
        breadcrumb: vec!["Note".to_string()],
        icon_id: None,
        controller_id: "note".to_string(),
        modes: Modes::one(ModeDefinition { id: "edit".to_string(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: IconName::from("pencil"), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
        default_mode_id: "edit".to_string(),
        window_kinds: WindowKinds::one(WindowKindDefinition {
            id: "canvas".to_string(),
            label: LocalizedLabel::native("Canvas", "Leinwand"),
            body_key: "canvas".to_string(),
            surface_kind: SurfaceKind::Canvas2d,
            icon_id: IconName::from("file"),
            options: Default::default(),
            actions: note_actions(),
            utilities: Vec::new(),
            interactions: Vec::new(),
            params_schema: None,
            artifact_snapshot_schema: None,
            input_event_schema: None,
            output_schema: None,
            capabilities: Vec::new(),
        }),
        panel_tabs: Vec::new(),
        keybindings: Vec::new(),
        utilities: vec![UtilityDefinition::new("pencil", LocalizedLabel::native("Pencil", "Stift"), IconName::from("pencil"))],
        tools: Vec::new(),
        commands: Vec::new(),
        interactions: Vec::new(),
        named_layouts: Vec::new(),
        default_layout: None,
        terminologies: Vec::new(),
        terminology_breadcrumbs: Default::default(),
        introduction: None,
        tutorials: Vec::new(),
        dialogs: Vec::new(),
        media_inputs: Vec::new(),
        media_outputs: Vec::new(),
        artifact_kinds: Vec::new(),
        config: Default::default(),
        command_grammar: Default::default(),
        io: Default::default(),
    }
}

pub fn note_descriptor() -> PackageDescriptor {
    wrap_descriptor(manifest::PluginManifest {
        plugin_id: "note".to_string(),
        label: "Note".to_string(),
        version: "0.1.0".to_string(),
        apps: vec![note_app()],
        examples: Vec::new(),
        capabilities: Vec::new(),
        topic_contributions: Vec::new(),
        commands: Vec::new(),
        artifact_kinds: Vec::new(),
        dependencies: Vec::new(),
        contributions: Vec::new(),
    })
}
//#endregion 🔖️NoteFixture

//#region 🔖️CollisionFixture
/// 🆔️ Two minimal synthetic plugins declaring the identical bare action id `deleteSelection` — the
/// D3 regression proof: a bare action id is never a capability id (`📓️luna-actions-audit.md` §7
/// lists `deleteSelection` as one of the 14 real cross-plugin collisions).
fn colliding_app(controller_id: &str) -> AppDefinition {
    let dialect = ArtifactDialect { artifact_kind: format!("s.{controller_id}"), standard: "1".to_string(), subset: "any".to_string() };
    AppDefinition {
        id: "surface".to_string(),
        role: AppRole::Editor,
        dialect,
        label: LocalizedLabel::native(controller_id, controller_id),
        breadcrumb: vec![controller_id.to_string()],
        icon_id: None,
        controller_id: controller_id.to_string(),
        modes: Modes::one(ModeDefinition { id: "edit".to_string(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: IconName::from("pencil"), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
        default_mode_id: "edit".to_string(),
        window_kinds: WindowKinds::one(WindowKindDefinition {
            id: "surface".to_string(),
            label: LocalizedLabel::native("Surface", "Fläche"),
            body_key: "surface".to_string(),
            surface_kind: SurfaceKind::Canvas2d,
            icon_id: IconName::from("file"),
            options: Default::default(),
            actions: vec![action("deleteSelection", "Delete Selection", "Auswahl löschen", ActionKind::Mutation)],
            utilities: Vec::new(),
            interactions: Vec::new(),
            params_schema: None,
            artifact_snapshot_schema: None,
            input_event_schema: None,
            output_schema: None,
            capabilities: Vec::new(),
        }),
        panel_tabs: Vec::new(),
        keybindings: Vec::new(),
        utilities: Vec::new(),
        tools: Vec::new(),
        commands: Vec::new(),
        interactions: Vec::new(),
        named_layouts: Vec::new(),
        default_layout: None,
        terminologies: Vec::new(),
        terminology_breadcrumbs: Default::default(),
        introduction: None,
        tutorials: Vec::new(),
        dialogs: Vec::new(),
        media_inputs: Vec::new(),
        media_outputs: Vec::new(),
        artifact_kinds: Vec::new(),
        config: Default::default(),
        command_grammar: Default::default(),
        io: Default::default(),
    }
}

pub fn colliding_action_id_source() -> CatalogSource {
    let a = wrap_descriptor(manifest::PluginManifest {
        plugin_id: "plugin-a".to_string(),
        label: "Plugin A".to_string(),
        version: "0.1.0".to_string(),
        apps: vec![colliding_app("plugin-a")],
        examples: Vec::new(),
        capabilities: Vec::new(),
        topic_contributions: Vec::new(),
        commands: Vec::new(),
        artifact_kinds: Vec::new(),
        dependencies: Vec::new(),
        contributions: Vec::new(),
    });
    let b = wrap_descriptor(manifest::PluginManifest {
        plugin_id: "plugin-b".to_string(),
        label: "Plugin B".to_string(),
        version: "0.1.0".to_string(),
        apps: vec![colliding_app("plugin-b")],
        examples: Vec::new(),
        capabilities: Vec::new(),
        topic_contributions: Vec::new(),
        commands: Vec::new(),
        artifact_kinds: Vec::new(),
        dependencies: Vec::new(),
        contributions: Vec::new(),
    });
    CatalogSource { descriptors: vec![a, b], os_commands: Vec::new(), shell: Vec::new(), gateway: Vec::new() }
}
//#endregion 🔖️CollisionFixture

//#region 🔖️CombinedSource
/// 📚️ The fixture `CatalogSource` every catalog/search/context/conformance test compiles against —
/// note + cad descriptors plus the crate's own real core-tool `CapabilityDefinition`s (see
/// `root::core_tool_capabilities`, mounted at the crate root so this module never needs to
/// reconstruct them).
pub fn note_and_cad_source() -> CatalogSource {
    CatalogSource { descriptors: vec![note_descriptor(), cad_descriptor()], os_commands: Vec::new(), shell: Vec::new(), gateway: crate::core_tool_capabilities() }
}
//#endregion 🔖️CombinedSource

//#region 🔖️Eval
const EVAL_JSON: &str = include_str!("🧪️eval/🔣️.json");

/// 📖️ Parses the embedded `🧪️eval/🔣️.json` fixture — `≥60` natural-language requests (English + German,
/// CLAUDE.md's en-first/de-second rule) each mapping to a real capability id compiled from
/// `note_and_cad_source()`.
pub fn eval_cases() -> Vec<EvalCase> {
    serde_json::from_str(EVAL_JSON).expect("🧪️eval/🔣️.json parses into Vec<EvalCase>")
}
//#endregion 🔖️Eval

//#region 🧪️Tests
#[cfg(test)]
mod quick {
    use super::*;

    #[test]
    fn cad_descriptor_declares_forty_non_framework_actions() {
        assert_eq!(cad_actions().len(), 40);
    }

    #[test]
    fn note_descriptor_declares_thirty_five_non_framework_actions() {
        assert_eq!(note_actions().len(), 35);
    }

    #[test]
    fn note_actions_declare_zero_manifest_args_per_d2() {
        assert!(note_actions().iter().all(|action| action.args.is_empty()));
    }

    #[test]
    fn eval_cases_has_at_least_sixty_entries_in_both_locales() {
        let cases = eval_cases();
        assert!(cases.len() >= 60, "expected >= 60 eval cases, got {}", cases.len());
        assert!(cases.iter().any(|case| case.locale == "en"));
        assert!(cases.iter().any(|case| case.locale == "de"));
    }
}
//#endregion 🧪️Tests
