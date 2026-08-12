//! 🖥️ Draw play app — the `ArtifactApp` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the window
//! render in `🎭️modes/✏️edit/🪟️windows/🖼️canvas`, panel trees in `📌️panels/*`, labels in
//! `🦀️terminology.rs`, view state in `🦀️config.rs`, shared compute in the artifact's `⚙️engine`.
//! This file is a routing table: `handle` → `DrawCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one `definition()` per node.

use crate::apps::draw::commands::canvas::DrawSession;
use crate::apps::draw::commands::{canvas, document, layer, view};
use crate::apps::draw::config::{DrawConfig, DrawConfigMutation};
use crate::apps::draw::presence::{DrawPresence, DrawPresenceMutation};
use crate::apps::draw::modes::edit;
use crate::apps::draw::modes::edit::windows::canvas as canvas_window;
use crate::apps::draw::panels::{catalogue as catalogue_panel, layers as layers_panel, properties as properties_panel};
use crate::apps::draw::terminology::DrawPlayLabels;
use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::{DrawSnapshot, DRAW_DOCUMENT_SCHEMA};
use semio_framework_plugin::{NoDraft, NoDraftMutation, DraftView, 
    ActionDescriptor, ActionKind, App, ConfigView, ArtifactApp, ArtifactView, Emit, Fault, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, SurfaceKind, UtilityCategory, UtilityDefinition, WindowEngagement,
    WindowEngagementInput, WindowEngagementStatus,
};
use store::EngineHandles;
use serde_json::Value;
use store::ArtifactPack;

pub use catalogue_panel::DRAW_PLAY_BODY_CATALOGUE;
pub use canvas_window::{DRAW_PLAY_BODY_COMPOSITE, DRAW_PLAY_WINDOW_CANVAS};
pub use layers_panel::{DRAW_LAYER_KIND_DRAG_MIME, DRAW_PLAY_BODY_LAYERS};
pub use properties_panel::DRAW_PLAY_BODY_PROPERTIES;

//#region 🔖️Constants
pub const DRAW_PLAY_APP_ID: &str = "draw-play";
pub const DRAW_PLAY_CONTROLLER_ID: &str = "draw-play";
/// 🧰️ The utility the canvas returns to after committing a shape/draft/trace (first UtilityRef default).
pub const DRAW_DEFAULT_UTILITY: &str = "selectDirect";
pub const DRAW_PLAY_EXAMPLE_DEFAULT_ID: &str = "semio";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`) builds its `on_change`/item actions with.
pub fn draw_play_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(DRAW_PLAY_CONTROLLER_ID).action(action, args)
}

/// 🛠️ An internal (non-palette) action declaration — the pointer/gesture/inspector-bound vocabulary
/// that is dispatched by the canvas/panels, never surfaced as a standalone command palette entry.
fn draw_internal_action(id: &str, label: impl Into<LocalizedLabel>, kind: ActionKind) -> semio_framework_plugin::ActionDefinition {
    semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new_catalog(id, label, kind) }
}

/// 🧰️ One canvas utility declaration (id/label/icon reused verbatim from the retired `utilities()` impl).
fn draw_utility(id: &str, label: impl Into<LocalizedLabel>, icon: &str, group: &str, category: UtilityCategory) -> UtilityDefinition {
    UtilityDefinition { group: Some(group.into()), category: Some(category), ..UtilityDefinition::new(id, label, icon) }
}
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `DrawPlayApp::Command` — the SOLE dispatch surface for draw's own behavior, covering every
    /// action `create_draw_app` declares. Field shapes mirror each action's real `args` object.
    /// **Row order is the binary variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum DrawCommand for DrawSnapshot, DrawMutation, DrawConfig, DrawConfigMutation, ctx = DrawSession {
        "setSnapshot" as "set-snapshot" => set_snapshot::SetSnapshot,
        "commitDocument" as "commit-document" => commit_document::CommitDocument,
        "setFixtureJson" as "fixture-json" => set_fixture_json::SetFixtureJson,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "setSelectedOpacity" as "selected-opacity" => set_selected_opacity::SetSelectedOpacity,
        "engagementSubmit" as "engagement-submit" => engagement_submit::EngagementSubmit,
        "addLayer" as "add-layer" => add_layer::AddLayer,
        "dropLayerKind" as "drop-layer-kind" => drop_layer_kind::DropLayerKind,
        "moveLayer" as "move-layer" => move_layer::MoveLayer,
        "deleteLayer" as "delete-layer" => delete_layer::DeleteLayer,
        "duplicateLayer" as "duplicate-layer" => duplicate_layer::DuplicateLayer,
        "toggleLayerVisible" as "toggle-layer-visible" => toggle_layer_visible::ToggleLayerVisible,
        "combineBoolean" as "combine-boolean" => combine_boolean::CombineBoolean,
        "patchLayer" as "patch-layer" => patch_layer::PatchLayer,
        "patchLayers" as "patch-layers" => patch_layers::PatchLayers,
        "setActiveUtility" as "active-utility" => set_active_utility::SetActiveUtility,
        "setCamera" as "camera" => set_camera::SetCamera,
        "setCameraZoom" as "camera-zoom" => set_camera_zoom::SetCameraZoom,
        "setSelection" as "set-selection" => set_selection::SetSelection,
        "setHover" as "set-hover" => set_hover::SetHover,
        "selectAll" as "select-all" => select_all::SelectAll,
        "clearSelection" as "clear-selection" => clear_selection::ClearSelection,
        "engagementInput" as "engagement-input" => engagement_input::EngagementInput,
        "setLocale" as "locale" => set_locale::SetLocale,
        "canvasPointerDown" as "canvas-pointer-down" => canvas_pointer_down::CanvasPointerDown,
        "canvasPointerMove" as "canvas-pointer-move" => canvas_pointer_move::CanvasPointerMove,
        "canvasPointerUp" as "canvas-pointer-up" => canvas_pointer_up::CanvasPointerUp,
        "canvasDoubleClick" as "canvas-double-click" => canvas_double_click::CanvasDoubleClick,
        "canvasCommitDraft" as "canvas-commit-draft" => canvas_commit_draft::CanvasCommitDraft,
        "canvasEscape" as "canvas-escape" => canvas_escape::CanvasEscape,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use document::{commit_document, set_active_example, set_snapshot, set_fixture_json};
use layer::{add_layer, combine_boolean, delete_layer, drop_layer_kind, duplicate_layer, move_layer, patch_layer, patch_layers, set_selected_opacity, toggle_layer_visible};
use view::{clear_selection, engagement_input, engagement_submit, select_all, set_active_utility, set_camera, set_camera_zoom, set_hover, set_locale, set_selection};
use canvas::{canvas_commit_draft, canvas_double_click, canvas_escape, canvas_pointer_down, canvas_pointer_move, canvas_pointer_up};
//#endregion 🔖️Commands

//#region 🔖️DrawPlayApp
/// 🧪️ Unit struct apart from `session`: every former `DrawInteractionState`/`ViewModel`-derived field
/// lives in [`DrawConfig`], written through [`DrawConfigMutation`]s. `session` holds the one piece of
/// state that is neither document nor view-config — the live gesture statechart — threaded into every
/// command handler as the `app_commands!` dispatch context.
#[derive(Default)]
pub struct DrawPlayApp;

impl ArtifactApp for DrawPlayApp {
    type Snapshot = DrawSnapshot;
    type Mutation = DrawMutation;
    type Config = DrawConfig;
    type ConfigMutation = DrawConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = DrawPresence;
    type PresenceMutation = DrawPresenceMutation;

    type Command = DrawCommand;

    const APP_ID: &'static str = DRAW_PLAY_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = DRAW_DOCUMENT_SCHEMA;

    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::apps::draw::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> DrawSnapshot {
        crate::artifacts::draw::engine::default_draw_document("empty", None)
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(crate::artifacts::draw::engine::draw_io())
    }

    /// 🎞️ `vector:out` (see `crate::artifacts::draw::engine::draw_vector_media`) plus the inherited
    /// `document:out` default (the pack of `doc.snapshot`, replicated inline — overriding
    /// `export_media` shadows the trait's provided body for every port on this app, not just the new one).
    fn export_media(port: &str, doc: &ArtifactView<'_, DrawSnapshot>) -> Result<Media, MediaError> {
        match port {
            "vector:out" => crate::artifacts::draw::engine::draw_vector_media(doc.snapshot),
            "document:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.document_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    // 🖼️ No override: whole-document replacement has no `Mutation` vehicle any more (banned
    // vocabulary — see `🧬️mutations/🦀️component.rs`'s module doc). The default `None` disables the
    // generic `import_media("document:in")` port for draw; explicit whole-document load/replace
    // stays reachable through the `set_snapshot`/`commit_document`/`set_fixture_json`/
    // `set_active_example` commands, which now emit `HostEffect::LoadDocument` (the sanctioned
    // non-history reset path) instead.

    /// 🏷️ `app_commands!`'s generated `command_id()`.
    fn command_id(command: &DrawCommand) -> &'static str {
        command.command_id()
    }

    fn handle(command: &DrawCommand, doc: &ArtifactView<'_, DrawSnapshot>, cfg: &ConfigView<'_, DrawConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<DrawMutation, DrawConfigMutation, Self::DraftMutation>, Fault> {
        thread_local! {
            static DRAW_SESSION: std::cell::RefCell<DrawSession> = std::cell::RefCell::new(DrawSession::default());
        }
        DRAW_SESSION.with(|session| {
            let mut session = session.borrow_mut();
            command.dispatch(doc, cfg, &mut session)
        })
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, DrawSnapshot>, cfg: &ConfigView<'_, DrawConfig>) -> semio_framework_plugin::UiNode {
        let document = doc.snapshot;
        let config = cfg.snapshot;
        let labels = semio_framework_plugin::resolve_labels_for_locale::<DrawPlayLabels>(&config.locale);
        let active_utility = config.active_utility_id.as_str();
        let session = DrawSession::default();
        match body_key {
            DRAW_PLAY_BODY_COMPOSITE => canvas_window::render(document, config, &session.gesture, active_utility),
            DRAW_PLAY_BODY_LAYERS => layers_panel::render(document, config, labels),
            DRAW_PLAY_BODY_CATALOGUE => catalogue_panel::render(document, config, labels),
            DRAW_PLAY_BODY_PROPERTIES => properties_panel::render(document, config, labels, active_utility),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️DrawPlayApp

//#region 🔖️Manifest
pub fn create_draw_app() -> App {
    let engagement = WindowEngagement {
        session_active: Some(false),
        options: None,
        input: Some(WindowEngagementInput {
            id: Some("draw-canvas-engagement".into()),
            value: Some(String::new()),
            placeholder: Some("Layer name".into()),
            on_change: Some(draw_play_action("engagementInput", None)),
            on_submit: Some(draw_play_action("engagementSubmit", None)),
            disabled: None,
            on_repeat_last: None,
            on_abort: None,
        }),
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus { id: "draw-layer-count".into(), text: "0 layers · 0 selected".into() }]),
        possible_engagements: None,
    };
    App::from_builder(
        App::builder(DRAW_PLAY_APP_ID, LocalizedLabel::native("Draw", "Zeichnen")).document(["semio", "draw"])
            .artifact_kind(crate::artifacts::draw::artifact_kind())
            .icon_id("draw")
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .default_mode_id("edit")
            .window_kind_with_engagement(DRAW_PLAY_WINDOW_CANVAS, LocalizedLabel::native("Canvas", "Leinwand"), DRAW_PLAY_BODY_COMPOSITE, SurfaceKind::Canvas2d, engagement, "pen-tool")
            .panel_tab_def(layers_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(properties_panel::definition())
            // ✏️ Palette-visible content operations.
            .mutation("addLayer", LocalizedLabel::native("Add Layer", "Ebene hinzufügen"))
            .mutation("combineBoolean", LocalizedLabel::native("Combine Boolean", "Boolean kombinieren"))
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            // 🔧️ Internal content operations — inspector/layer-panel/import-bound, not palette commands.
            .action_with(draw_internal_action("setSnapshot", LocalizedLabel::native("Set Document", "Dokument festlegen"), ActionKind::Mutation))
            .action_with(draw_internal_action("commitDocument", LocalizedLabel::native("Commit Document", "Dokument übernehmen"), ActionKind::Mutation))
            .action_with(draw_internal_action("setFixtureJson", LocalizedLabel::native("Set Fixture Json", "Fixture-JSON festlegen"), ActionKind::Mutation))
            .action_with(draw_internal_action("setSelectedOpacity", LocalizedLabel::native("Set Selected Opacity", "Deckkraft der Auswahl festlegen"), ActionKind::Mutation))
            .action_with(draw_internal_action("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"), ActionKind::Mutation))
            .action_with(draw_internal_action("dropLayerKind", LocalizedLabel::native("Drop Layer Kind", "Ebenenart ablegen"), ActionKind::Mutation))
            .action_with(draw_internal_action("moveLayer", LocalizedLabel::native("Move Layer", "Ebene verschieben"), ActionKind::Mutation))
            .action_with(draw_internal_action("deleteLayer", LocalizedLabel::native("Delete Layer", "Ebene löschen"), ActionKind::Mutation))
            .action_with(draw_internal_action("duplicateLayer", LocalizedLabel::native("Duplicate Layer", "Ebene duplizieren"), ActionKind::Mutation))
            .action_with(draw_internal_action("toggleLayerVisible", LocalizedLabel::native("Toggle Layer Visible", "Ebenensichtbarkeit umschalten"), ActionKind::Mutation))
            .action_with(draw_internal_action("patchLayer", LocalizedLabel::native("Patch Layer", "Ebene aktualisieren"), ActionKind::Mutation))
            .action_with(draw_internal_action("patchLayers", LocalizedLabel::native("Patch Layers", "Ebenen aktualisieren"), ActionKind::Mutation))
            // 🖱️ Internal pointer/gesture vocabulary — commit-time handlers emit operations, the rest are pure View.
            .action_with(draw_internal_action("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"), ActionKind::Mutation))
            .action_with(draw_internal_action("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Leinwand-Zeiger losgelassen"), ActionKind::Mutation))
            .action_with(draw_internal_action("canvasDoubleClick", LocalizedLabel::native("Canvas Double Click", "Leinwand-Doppelklick"), ActionKind::Mutation))
            .action_with(draw_internal_action("canvasCommitDraft", LocalizedLabel::native("Canvas Commit Draft", "Leinwand-Entwurf übernehmen"), ActionKind::Mutation))
            .action_with(draw_internal_action("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Leinwand-Zeiger bewegen"), ActionKind::View))
            .action_with(draw_internal_action("canvasEscape", LocalizedLabel::native("Canvas Escape", "Leinwand abbrechen"), ActionKind::View))
            // 👁️ Ephemeral view state.
            .view_action("selectAll", LocalizedLabel::native("Select All", "Alles auswählen"))
            .view_action("clearSelection", LocalizedLabel::native("Clear Selection", "Auswahl aufheben"))
            .action_with(draw_internal_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"), ActionKind::View))
            .action_with(draw_internal_action("setHover", LocalizedLabel::native("Set Hover", "Überfahren festlegen"), ActionKind::View))
            .action_with(draw_internal_action("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View))
            .action_with(draw_internal_action("setLocale", LocalizedLabel::native("Set Locale", "Sprache festlegen"), ActionKind::View))
            // 📷️ Camera — session-only runtime pose, never a document operation.
            .action_with(draw_internal_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View))
            .action_with(draw_internal_action("setCameraZoom", LocalizedLabel::native("Set Camera Zoom", "Kamerazoom festlegen"), ActionKind::View))
            // 🧰️ Canvas utilities — one exclusive set per window, active utility host-owned (never a document operation).
            .utility(draw_utility("selectMarquee", LocalizedLabel::native("Marquee Select", "Rahmenauswahl"), "square-dashed", "Select", UtilityCategory::Selection))
            .utility(draw_utility("selectLasso", LocalizedLabel::native("Lasso Select", "Lasso-Auswahl"), "lasso", "Select", UtilityCategory::Selection))
            .utility(draw_utility("selectDirect", LocalizedLabel::native("Direct Select", "Direktauswahl"), "mouse-pointer-2", "Select", UtilityCategory::Selection))
            .utility(draw_utility("pen", LocalizedLabel::native("Pen", "Stift"), "pen-tool", "Draw", UtilityCategory::Utilities))
            .utility(draw_utility("shapeRect", LocalizedLabel::native("Rectangle", "Rechteck"), "rectangle-tool", "Draw", UtilityCategory::Utilities))
            .utility(draw_utility("shapeEllipse", LocalizedLabel::native("Ellipse", "Ellipse"), "circle", "Draw", UtilityCategory::Utilities))
            .utility(draw_utility("shapeLine", LocalizedLabel::native("Line", "Linie"), "minus", "Draw", UtilityCategory::Utilities))
            .utility(draw_utility("shapePolygon", LocalizedLabel::native("Polygon", "Polygon"), "hexagon", "Draw", UtilityCategory::Utilities))
            .utility(draw_utility("booleanCombine", LocalizedLabel::native("Boolean", "Boolean"), "combine", "Combine", UtilityCategory::Utilities))
            .utility(draw_utility("trace", LocalizedLabel::native("Trace", "Nachzeichnen"), "scan-line", "Combine", UtilityCategory::Utilities))
            .utility(draw_utility("transformMove", LocalizedLabel::native("Pan", "Verschieben"), "move", "View", UtilityCategory::Utilities))
            .window_kind_utilities(DRAW_PLAY_WINDOW_CANVAS, vec![
                "selectMarquee".into(), "selectLasso".into(), "selectDirect".into(),
                "pen".into(), "shapeRect".into(), "shapeEllipse".into(), "shapeLine".into(), "shapePolygon".into(),
                "booleanCombine".into(), "trace".into(), "transformMove".into(),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("mod+a", "selectAll")
            .keybinding("escape", "canvasEscape")
            .keybinding("enter", "canvasCommitDraft")
            .default_layout(edit::layout()),
    )
    .example(DRAW_PLAY_EXAMPLE_DEFAULT_ID, LocalizedLabel::native("Semio", "Semio"), crate::artifacts::draw::engine::semio_draw_example_json(), "sparkles")
    .workflow("draw", "Draw", "2d.drawing")
}
//#endregion 🔖️Manifest

//#region 🔗️StandaloneLinkage
/// 🪶️ Satisfies the plugin runtime when this app is linked as its own WASM module.
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
#[linkage = "weak"]
pub extern "C" fn semio_plugin_bundle_installer_link_shim() {}
//#endregion 🔗️StandaloneLinkage

//#region 🔖️WasmBridge
/// 🌉️ Generic `ArtifactStore` aliases used only by the WASM bridge below.
pub type DrawEnvelope = store::ArtifactEnvelope<DrawSnapshot, DrawMutation>;
pub type DrawStore = store::ArtifactStore<DrawSnapshot, DrawMutation>;

#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use store::create_document_envelope;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct DrawSnapshotVcs {
        store: RefCell<DrawStore>,
    }

    #[wasm_bindgen]
    impl DrawSnapshotVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<DrawSnapshotVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: DrawEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    DrawStore::new(envelope)
                }
                None => DrawStore::new(create_document_envelope(DRAW_DOCUMENT_SCHEMA, "draw", crate::artifacts::draw::engine::empty_draw_snapshot(), None)),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchText)]
        pub fn dispatch_text(&self, command_text: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_text(command_text).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = dispatchBinary)]
        pub fn dispatch_binary(&self, command_bytes: &[u8]) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_binary(command_bytes).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn snapshot_json(&self) -> Result<String, JsValue> {
            self.store.borrow().snapshot_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = envelopeJson)]
        pub fn envelope_json(&self) -> Result<String, JsValue> {
            self.store.borrow().envelope_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = generation)]
        pub fn generation(&self) -> u32 {
            self.store.borrow().generation() as u32
        }
    }
}
//#endregion 🔖️WasmBridge

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::VcsArtifactApp;

    pub type DrawApp = VcsArtifactApp<DrawPlayApp>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub fn draw_app() -> DrawApp {
        new_app::<DrawPlayApp>()
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub fn draw_app_with_registry() -> DrawApp {
        new_app_with_registry::<DrawPlayApp>(create_draw_app)
    }

    /// 🧰️ Sets the config's host-owned active utility to `utility`.
    pub fn set_utility(app: &mut DrawApp, utility: &str) {
        app.dispatch_typed(DrawCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: utility.into() }), &meta("local")).expect("set active utility");
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::draw::engine::{default_draw_document, layer_id, semio_draw_example_json};
    use crate::artifacts::draw::DrawLayerNode;
    use semio_framework_plugin::kernel::HostEffect;
    use semio_framework_plugin::{testkit as fw_testkit, PluginApp, ViewModel, SET_ACTIVE_UTILITY_ACTION_ID};
    use testkit::{draw_app, draw_app_with_registry, set_utility, DrawApp};

    fn first_layer_id(app: &DrawApp) -> String {
        layer_id(&app.snapshot().expect("materialize projection").layers[0]).to_string()
    }

    fn last_layer_id(app: &DrawApp) -> String {
        let projection = app.snapshot().expect("materialize projection");
        layer_id(projection.layers.last().expect("layer")).to_string()
    }

    #[test]
    fn renders_canvas_scene_with_segments() {
        let mut app = draw_app();
        let example_json = semio_draw_example_json();
        let node = app.render(DRAW_PLAY_BODY_COMPOSITE, Some(example_json.as_str()), &ViewModel::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
        let value = serde_json::to_value(&node).unwrap();
        let layers_json = value.pointer("/canvas2d/layersJson").and_then(|v| v.as_str()).expect("layersJson string");
        assert!(layers_json.contains("segments"));
        let records: Vec<Value> = serde_json::from_str(layers_json).unwrap();
        assert!(records.iter().any(|record| record.get("role").and_then(|value| value.as_str()) == Some("meta")));
        assert!(records.iter().any(|record| record.get("id").and_then(|value| value.as_str()) == Some("artboard:frame")), "canvas must show the document artboard frame");
        assert!(
            records.iter().any(|record| { record.get("id").and_then(|value| value.as_str()) == Some("artboard:dimensions") && record.pointer("/text/content").and_then(|value| value.as_str()).is_some_and(|label| label.contains('×')) }),
            "canvas must show document dimension label"
        );
        assert!(layers_json.contains("200 × 200"), "example artboard dimensions must be visible");
    }

    #[test]
    fn default_document_exposes_artboard_dimensions_on_canvas() {
        let mut app = draw_app();
        let node = app.render(DRAW_PLAY_BODY_COMPOSITE, None, &ViewModel::default()).expect("render");
        let value = serde_json::to_value(&node).unwrap();
        let layers_json = value.pointer("/canvas2d/layersJson").and_then(|v| v.as_str()).expect("layersJson string");
        assert!(layers_json.contains("1024 × 1024"), "blank documents show default artboard dimensions");
    }

    #[test]
    fn layers_panel_lists_default_layer() {
        let mut app = draw_app();
        let node = app.render(DRAW_PLAY_BODY_LAYERS, None, &ViewModel::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("draw-play-layers.add.path"));
        assert!(json.contains("Layer 1"));
    }

    #[test]
    fn catalogue_panel_lists_boolean_operations() {
        let mut app = draw_app();
        let node = app.render(DRAW_PLAY_BODY_CATALOGUE, None, &ViewModel::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("draw-play-catalogue.path"));
        assert!(json.contains("Boolean union"));
    }

    #[test]
    fn add_layer_action_emits_op_and_appends_path() {
        let mut app = draw_app();
        let before = app.snapshot().unwrap().layers.len();
        let result = app.dispatch_typed(DrawCommand::AddLayer(add_layer::AddLayer { kind: "shape:rect".into() }), &fw_testkit::meta("local")).expect("add layer");
        assert_eq!(result.mutations.len(), 1);
        let projection = app.snapshot().unwrap();
        assert_eq!(projection.layers.len(), before + 1);
        assert!(projection.layers.iter().any(|layer| matches!(layer, DrawLayerNode::Shape(shape) if shape.shape_kind == "rect")));
    }

    #[test]
    fn patch_layers_opacity_emits_granular_operation() {
        let mut app = draw_app();
        let id = first_layer_id(&app);
        let result = app.dispatch_typed(DrawCommand::PatchLayers(patch_layers::PatchLayers { layer_ids: vec![id], field: "opacity".into(), value: "0.5".into() }), &fw_testkit::meta("local")).expect("patch");
        assert_eq!(result.mutations.len(), 1);
        let projection = app.snapshot().unwrap();
        assert!((crate::artifacts::draw::engine::layer_base(&projection.layers[0]).opacity - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn patch_layer_name_emits_op_and_changes_projection() {
        let mut app = draw_app();
        let id = first_layer_id(&app);
        let result = app.dispatch_typed(DrawCommand::PatchLayer(patch_layer::PatchLayer { layer_id: id, field: "name".into(), value: "Renamed".into() }), &fw_testkit::meta("local")).expect("patch");
        assert_eq!(result.mutations.len(), 1);
        assert_eq!(crate::artifacts::draw::engine::layer_base(&app.snapshot().unwrap().layers[0]).name, "Renamed");
    }

    #[test]
    fn set_selection_view_action_emits_no_ops_and_drives_inspector() {
        let mut app = draw_app();
        let id = first_layer_id(&app);
        let result = app.dispatch_typed(DrawCommand::SetSelection(set_selection::SetSelection { ids: vec![id] }), &fw_testkit::meta("local")).expect("select");
        assert!(result.mutations.is_empty(), "selection is ephemeral view state, not a document operation");
        let node = app.render(DRAW_PLAY_BODY_PROPERTIES, None, &ViewModel::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Orientation"));
        assert!(json.contains("Position X"));
    }

    #[test]
    fn set_active_utility_clears_scratch_and_emits_no_history_entry() {
        let mut app = draw_app_with_registry();
        set_utility(&mut app, "shapeRect");
        app.dispatch_typed(DrawCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { x: 10.0, y: 10.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false }), &fw_testkit::meta("local")).expect("down");
        let before = app.snapshot().unwrap();
        let result = app.dispatch_typed(DrawCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "pen".into() }), &fw_testkit::meta("local")).expect("switch utility");
        assert!(result.mutations.is_empty(), "utility switching never emits document operations");
        assert_eq!(app.snapshot().unwrap(), before, "utility switching does not mutate the document");
        let up = app.dispatch_typed(DrawCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { x: 40.0, y: 40.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false }), &fw_testkit::meta("local")).expect("up");
        assert!(up.mutations.is_empty(), "the in-progress shape draft was cleared on utility switch");
    }

    #[test]
    fn combine_boolean_creates_boolean_layer() {
        let mut app = draw_app();
        let first_id = first_layer_id(&app);
        app.dispatch_typed(DrawCommand::AddLayer(add_layer::AddLayer { kind: "shape:rect".into() }), &fw_testkit::meta("local")).expect("add rect");
        let second_id = last_layer_id(&app);
        let result = app.dispatch_typed(DrawCommand::CombineBoolean(combine_boolean::CombineBoolean { operation: "union".into(), ids: vec![first_id, second_id] }), &fw_testkit::meta("local")).expect("combine");
        assert_eq!(result.mutations.len(), 1);
        assert!(app.snapshot().unwrap().layers.iter().any(|layer| matches!(layer, DrawLayerNode::Boolean(_))));
    }

    #[test]
    fn canvas_point_to_world_matches_host_formula() {
        let camera = crate::artifacts::draw::DrawCamera { x: 100.0, y: 50.0, zoom: 2.0 };
        let (world_x, world_y) = canvas::canvas_point_to_world(&camera, 420.0, 310.0, 800.0, 600.0);
        assert!((world_x - 110.0).abs() < 1e-9);
        assert!((world_y - 55.0).abs() < 1e-9);
    }

    #[test]
    fn shape_rect_drag_commits_one_layer_and_requests_utility_reset() {
        let mut app = draw_app_with_registry();
        set_utility(&mut app, "shapeRect");
        app.dispatch_typed(DrawCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { x: 500.0, y: 400.0, width: 1000.0, height: 800.0, shift: false, ctrl: false, meta: false }), &fw_testkit::meta("local")).expect("down");
        app.dispatch_typed(DrawCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 600.0, y: 500.0, width: 1000.0, height: 800.0 }), &fw_testkit::meta("local")).expect("move");
        let result = app
            .dispatch_typed(DrawCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { x: 600.0, y: 500.0, width: 1000.0, height: 800.0, shift: false, ctrl: false, meta: false }), &fw_testkit::meta("local"))
            .expect("up");
        assert_eq!(result.mutations.len(), 1, "a shape drag commits as one edit adding exactly the layer");
        let projection = app.snapshot().unwrap();
        assert!(projection.layers.iter().any(|layer| matches!(layer, DrawLayerNode::Shape(shape) if shape.shape_kind == "rect")));
        assert!(
            matches!(
                result.requested_effects.as_slice(),
                [HostEffect::SetActiveUtility { window_id, utility_id }] if window_id == DRAW_PLAY_WINDOW_CANVAS && utility_id == "selectDirect"
            ),
            "the canvas returns to select-direct via a host effect, not a document operation"
        );
    }

    #[test]
    fn pen_draft_commits_path_layer_on_enter() {
        let mut app = draw_app();
        set_utility(&mut app, "pen");
        app.dispatch_typed(DrawCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { x: 400.0, y: 300.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false }), &fw_testkit::meta("local")).expect("p1");
        app.dispatch_typed(DrawCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { x: 500.0, y: 300.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false }), &fw_testkit::meta("local")).expect("p2");
        let result = app.dispatch_typed(DrawCommand::CanvasCommitDraft(canvas_commit_draft::CanvasCommitDraft {}), &fw_testkit::meta("local")).expect("commit");
        assert_eq!(result.mutations.len(), 1, "the draft commits as exactly one AddLayer edit");
        let projection = app.snapshot().unwrap();
        assert!(projection.layers.iter().any(|layer| matches!(layer, DrawLayerNode::Path(path) if !path.segments.is_empty())));
        assert!(matches!(result.requested_effects.as_slice(), [HostEffect::SetActiveUtility { utility_id, .. }] if utility_id == "selectDirect"));
    }

    #[test]
    fn canvas_escape_cancels_draft_without_committing() {
        let mut app = draw_app();
        let before = app.snapshot().unwrap().layers.len();
        set_utility(&mut app, "pen");
        app.dispatch_typed(DrawCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { x: 400.0, y: 300.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false }), &fw_testkit::meta("local")).expect("p1");
        let result = app.dispatch_typed(DrawCommand::CanvasEscape(canvas_escape::CanvasEscape {}), &fw_testkit::meta("local")).expect("escape");
        assert!(result.mutations.is_empty());
        assert_eq!(app.snapshot().unwrap().layers.len(), before);
    }

    #[test]
    fn marquee_select_covers_contained_layer_only() {
        // 🔖 Built through dispatched commands (`add-layer` + `patch-layer` transform fields), never
        // a whole-document swap — `SetSnapshot` is banned vocabulary now (see
        // `🧬️mutations/🦀️component.rs`'s module doc); this exercises the same real semantic
        // `create-layer`/`update-layer-transform` mutations a live editor session would emit.
        let mut app = draw_app();
        set_utility(&mut app, "selectMarquee");
        let initial_id = layer_id(&app.snapshot().unwrap().layers[0]).to_string();
        app.dispatch_typed(DrawCommand::DeleteLayer(delete_layer::DeleteLayer { layer_id: initial_id }), &fw_testkit::meta("local")).expect("clear default layer");

        app.dispatch_typed(DrawCommand::AddLayer(add_layer::AddLayer { kind: "shape:rect".into() }), &fw_testkit::meta("local")).expect("add rect");
        let rect_a_id = layer_id(app.snapshot().unwrap().layers.last().unwrap()).to_string();
        for (field, value) in [("transformX", "10"), ("transformY", "10"), ("transformScaleX", "0.15625"), ("transformScaleY", "0.208333")] {
            app.dispatch_typed(DrawCommand::PatchLayer(patch_layer::PatchLayer { layer_id: rect_a_id.clone(), field: field.into(), value: value.into() }), &fw_testkit::meta("local")).expect("position rect a");
        }

        app.dispatch_typed(DrawCommand::AddLayer(add_layer::AddLayer { kind: "shape:ellipse".into() }), &fw_testkit::meta("local")).expect("add ellipse");
        let ellipse_b_id = layer_id(app.snapshot().unwrap().layers.last().unwrap()).to_string();
        for (field, value) in [("transformX", "200"), ("transformY", "200")] {
            app.dispatch_typed(DrawCommand::PatchLayer(patch_layer::PatchLayer { layer_id: ellipse_b_id.clone(), field: field.into(), value: value.into() }), &fw_testkit::meta("local")).expect("position ellipse b");
        }

        app.dispatch_typed(DrawCommand::SetCamera(set_camera::SetCamera { camera: crate::artifacts::draw::DrawCamera { x: 0.0, y: 0.0, zoom: 1.0 } }), &fw_testkit::meta("local")).expect("camera");
        app.dispatch_typed(DrawCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { x: 400.0, y: 300.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false }), &fw_testkit::meta("local")).expect("down");
        app.dispatch_typed(DrawCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 460.0, y: 360.0, width: 800.0, height: 600.0 }), &fw_testkit::meta("local")).expect("move");
        app.dispatch_typed(DrawCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { x: 460.0, y: 360.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false }), &fw_testkit::meta("local")).expect("up");
        let node = app.render(DRAW_PLAY_BODY_COMPOSITE, None, &ViewModel::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains(&format!("overlay:sel:{rect_a_id}")), "the contained rect is selected");
        assert!(!json.contains(&format!("overlay:sel:{ellipse_b_id}")), "the outside ellipse is not selected");
    }

    #[test]
    fn set_camera_writes_runtime_and_emits_no_operations() {
        let mut app = draw_app();
        let before = app.snapshot().expect("projection");
        let result = app.dispatch_typed(DrawCommand::SetCamera(set_camera::SetCamera { camera: crate::artifacts::draw::DrawCamera { x: 5.0, y: 5.0, zoom: 2.0 } }), &fw_testkit::meta("local")).expect("camera");
        assert!(result.mutations.is_empty(), "camera is a view action and emits no operations");
        assert_eq!(app.snapshot().expect("projection"), before, "camera never mutates the document");
        let json = serde_json::to_string(&app.render(DRAW_PLAY_BODY_COMPOSITE, None, &ViewModel::default()).expect("render")).unwrap();
        assert!(json.contains(r#""zoom":2.0"#), "composite scene camera reflects runtime state: {json}");
        assert!(json.contains(r#""cameraX":5.0"#), "composite scene camera reflects runtime state: {json}");
    }

    #[test]
    fn set_camera_zoom_updates_zoom_and_keeps_pan_via_runtime() {
        let mut app = draw_app();
        app.dispatch_typed(DrawCommand::SetCamera(set_camera::SetCamera { camera: crate::artifacts::draw::DrawCamera { x: 4.0, y: 5.0, zoom: 1.0 } }), &fw_testkit::meta("local")).expect("set camera");
        let result = app.dispatch_typed(DrawCommand::SetCameraZoom(set_camera_zoom::SetCameraZoom { value: 3.0 }), &fw_testkit::meta("local")).expect("set camera zoom");
        assert!(result.mutations.is_empty(), "camera zoom is a view action and emits no operations");
        let json = serde_json::to_string(&app.render(DRAW_PLAY_BODY_COMPOSITE, None, &ViewModel::default()).expect("render")).unwrap();
        assert!(json.contains(r#""zoom":3.0"#), "zoom updated: {json}");
        assert!(json.contains(r#""cameraX":4.0"#), "pan preserved across zoom-only update: {json}");
    }

    #[test]
    fn add_layer_undo_round_trip_through_wrapper() {
        let mut app = draw_app();
        let before = app.snapshot().unwrap().layers.len();
        fw_testkit::assert_undo_redo_round_trip(&mut app, DrawCommand::AddLayer(add_layer::AddLayer { kind: "path".into() }), |app| app.snapshot().unwrap().layers.len(), before, before + 1);
    }

    #[test]
    fn utility_registry_declares_all_canvas_utilities_scoped_to_the_window() {
        let definition = create_draw_app().definition;
        let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
        assert_eq!(utility_ids, ["selectMarquee", "selectLasso", "selectDirect", "pen", "shapeRect", "shapeEllipse", "shapeLine", "shapePolygon", "booleanCombine", "trace", "transformMove"],);
        let selects: Vec<&str> = definition.utilities.iter().filter(|utility| utility.category == Some(UtilityCategory::Selection)).map(|utility| utility.id.as_str()).collect();
        assert_eq!(selects, ["selectMarquee", "selectLasso", "selectDirect"]);
        let scene = definition.window_kinds.iter().find(|window| window.id == DRAW_PLAY_WINDOW_CANVAS).expect("canvas window");
        assert_eq!(scene.utilities.len(), definition.utilities.len(), "every utility is scoped to the canvas window kind");
        assert!(definition.actions.iter().any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID && matches!(action.kind, ActionKind::View)));
        assert!(!definition.actions.iter().any(|action| action.id == "setActiveUtility" && !matches!(action.kind, ActionKind::View)));
    }

    #[test]
    fn render_canvas_emits_selection_overlay() {
        let mut app = draw_app();
        app.dispatch_typed(DrawCommand::AddLayer(add_layer::AddLayer { kind: "shape:rect".into() }), &fw_testkit::meta("local")).expect("add");
        let id = last_layer_id(&app);
        app.dispatch_typed(DrawCommand::SetSelection(set_selection::SetSelection { ids: vec![id.clone()] }), &fw_testkit::meta("local")).expect("select");
        let node = app.render(DRAW_PLAY_BODY_COMPOSITE, None, &ViewModel::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains(&format!("overlay:sel:{id}")));
    }

    #[test]
    fn draw_labels_resolve_native_by_default() {
        let mut app = draw_app();
        let node = app.render(DRAW_PLAY_BODY_LAYERS, None, &ViewModel::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Add Path"));
        assert!(json.contains("Add Rectangle"));
        assert!(!json.contains("Pfad hinzufügen"));
    }

    #[test]
    fn draw_labels_translate_panels_in_german() {
        let mut app = draw_app();
        app.dispatch_typed(DrawCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }), &fw_testkit::meta("local")).expect("set locale");
        let layers_node = app.render(DRAW_PLAY_BODY_LAYERS, None, &ViewModel::default()).expect("render");
        let layers_json = serde_json::to_string(&layers_node).unwrap();
        assert!(layers_json.contains("Pfad hinzufügen"));
        assert!(layers_json.contains("Rechteck hinzufügen"));
        assert!(!layers_json.contains("Add Path"));
        let catalogue_node = app.render(DRAW_PLAY_BODY_CATALOGUE, None, &ViewModel::default()).expect("render");
        let catalogue_json = serde_json::to_string(&catalogue_node).unwrap();
        assert!(catalogue_json.contains("\"Ellipse\""));
        assert!(catalogue_json.contains("Nachzeichnung"));
    }

    #[test]
    fn draw_io_declares_vector_out_and_export_media_covers_both_ports() {
        let mut app = draw_app();
        app.dispatch_typed(DrawCommand::AddLayer(add_layer::AddLayer { kind: "shape:rect".into() }), &fw_testkit::meta("local")).expect("add");
        let projection = app.snapshot().expect("projection");
        let doc = ArtifactView { snapshot: &projection, history: &semio_framework_plugin::HistoryView::empty() };
        let app_impl = DrawPlayApp::default();
        let vector = DrawPlayApp::export_media("vector:out", &doc).expect("vector:out");
        let MediaPayload::Structured { schema, json } = vector.payload else { panic!("expected structured svg payload") };
        assert_eq!(schema, "2d.drawing");
        assert!(json.starts_with("<svg"));
        assert!(DrawPlayApp::export_media("document:out", &doc).is_ok());
        assert!(matches!(DrawPlayApp::export_media("unknown:out", &doc), Err(MediaError::NotImplemented)));
    }

    //#region 🔖️GesturePreview
    #[test]
    fn gesture_preview_is_none_while_idle() {
        let session = DrawSession::default();
        assert!(session.gesture_preview().is_none(), "no live gesture, nothing to preview");
    }

    #[test]
    fn gesture_preview_reflects_live_shape_drag_and_clears_on_commit() {
        let mut session = DrawSession::default();
        let document = default_draw_document("empty", None);
        let mut config = DrawConfig { active_utility_id: "shapeRect".into(), ..Default::default() };

        let down = session.step_gesture(canvas::draw_gesture::Event::PointerDown { utility: "shapeRect".into(), world: [10.0, 10.0], shift: false, ctrl: false, meta: false }, &document, &mut config);
        assert!(down.artifact_mutations.is_empty(), "pointer-down starts a scratch drag, not a document operation");
        let (key, seq_after_down, payload) = session.gesture_preview().expect("shape drag is live after pointer-down");
        assert_eq!(key, "gesture");
        let value: Value = serde_json::from_slice(&payload).expect("payload is valid json");
        assert_eq!(value["start"], serde_json::json!([10.0, 10.0]));
        assert_eq!(value["cursor"], serde_json::json!([10.0, 10.0]));

        let moved = session.step_gesture(canvas::draw_gesture::Event::PointerMove { world: [40.0, 30.0], marquee_threshold_world: 4.0 }, &document, &mut config);
        assert!(moved.artifact_mutations.is_empty(), "mid-drag ticks emit zero operations (scratch-commit pattern)");
        let (_, seq_after_move, payload) = session.gesture_preview().expect("shape drag is still live mid-drag");
        let value: Value = serde_json::from_slice(&payload).expect("payload is valid json");
        assert_eq!(value["cursor"], serde_json::json!([40.0, 30.0]), "preview tracks the live cursor, not the drag start");
        assert!(seq_after_move > seq_after_down, "seq is monotone per tick, for staleness detection on the receiving end");

        let up = session.step_gesture(canvas::draw_gesture::Event::PointerUp { utility: "shapeRect".into(), world: [40.0, 30.0], shift: false, ctrl: false, meta: false }, &document, &mut config);
        assert_eq!(up.artifact_mutations.len(), 1, "pointer-up commits the shape as one real DrawMutation");
        assert!(session.gesture_preview().is_none(), "the gesture returned to idle: nothing left to preview, and the commit above already carried the real operation");
    }

    #[test]
    fn gesture_preview_is_a_pure_read_never_mutating_gesture_context() {
        let mut session = DrawSession::default();
        let document = default_draw_document("empty", None);
        let mut config = DrawConfig { active_utility_id: "shapeRect".into(), ..Default::default() };
        session.step_gesture(canvas::draw_gesture::Event::PointerDown { utility: "shapeRect".into(), world: [1.0, 2.0], shift: false, ctrl: false, meta: false }, &document, &mut config);
        let context_before = session.gesture.context.clone();
        let _ = session.gesture_preview();
        let _ = session.gesture_preview();
        assert_eq!(session.gesture.context, context_before, "gesture_preview must never mutate the live gesture scratch it reads");
    }
    //#endregion 🔖️GesturePreview

    //#region 🔖️WireGuards
    /// 🔖️ One `DrawCommand` value per row, in binary-variant-ordinal order — feeds both the
    /// op-text/binary equivalence loop and the "printed line starts with the row's wire keyword"
    /// assertion. Permanent wire guard: appending a variant is safe, reordering breaks the format.
    fn every_command() -> Vec<DrawCommand> {
        vec![
            DrawCommand::SetSnapshot(set_snapshot::SetSnapshot { snapshot: default_draw_document("cmd-doc", None) }),
            DrawCommand::CommitDocument(commit_document::CommitDocument { snapshot: default_draw_document("cmd-doc-2", None) }),
            DrawCommand::SetFixtureJson(set_fixture_json::SetFixtureJson { json: "{}".into() }),
            DrawCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "semio".into() }),
            DrawCommand::SetSelectedOpacity(set_selected_opacity::SetSelectedOpacity { value: 0.5 }),
            DrawCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some("Renamed \"layer\"".into()) }),
            DrawCommand::AddLayer(add_layer::AddLayer { kind: "shape:rect".into() }),
            DrawCommand::DropLayerKind(drop_layer_kind::DropLayerKind { kind: "path".into(), target_row_id: "draw-play-layers".into(), drop_position: "inside".into() }),
            DrawCommand::MoveLayer(move_layer::MoveLayer { layer_id: "layer-1".into(), target_row_id: "draw-play-layers".into(), drop_position: "after".into() }),
            DrawCommand::DeleteLayer(delete_layer::DeleteLayer { layer_id: "layer-1".into() }),
            DrawCommand::DuplicateLayer(duplicate_layer::DuplicateLayer { layer_id: "layer-1".into() }),
            DrawCommand::ToggleLayerVisible(toggle_layer_visible::ToggleLayerVisible { layer_id: "layer-1".into() }),
            DrawCommand::CombineBoolean(combine_boolean::CombineBoolean { operation: "union".into(), ids: vec!["a".into(), "b".into()] }),
            DrawCommand::PatchLayer(patch_layer::PatchLayer { layer_id: "layer-1".into(), field: "opacity".into(), value: "0.4".into() }),
            DrawCommand::PatchLayers(patch_layers::PatchLayers { layer_ids: vec!["a".into(), "b".into()], field: "blendMode".into(), value: "\"multiply\"".into() }),
            DrawCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "pen".into() }),
            DrawCommand::SetCamera(set_camera::SetCamera { camera: crate::artifacts::draw::DrawCamera { x: 1.0, y: 2.0, zoom: 1.5 } }),
            DrawCommand::SetCameraZoom(set_camera_zoom::SetCameraZoom { value: 2.0 }),
            DrawCommand::SetSelection(set_selection::SetSelection { ids: vec!["a".into(), "b".into()] }),
            DrawCommand::SetHover(set_hover::SetHover { id: Some("a".into()) }),
            DrawCommand::SelectAll(select_all::SelectAll {}),
            DrawCommand::ClearSelection(clear_selection::ClearSelection {}),
            DrawCommand::EngagementInput(engagement_input::EngagementInput { value: "typing".into() }),
            DrawCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
            DrawCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { x: 1.0, y: 2.0, width: 800.0, height: 600.0, shift: true, ctrl: false, meta: false }),
            DrawCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 1.0, y: 2.0, width: 800.0, height: 600.0 }),
            DrawCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { x: 1.0, y: 2.0, width: 800.0, height: 600.0, shift: false, ctrl: true, meta: false }),
            DrawCommand::CanvasDoubleClick(canvas_double_click::CanvasDoubleClick {}),
            DrawCommand::CanvasCommitDraft(canvas_commit_draft::CanvasCommitDraft {}),
            DrawCommand::CanvasEscape(canvas_escape::CanvasEscape {}),
        ]
    }

    #[test]
    fn draw_command_op_text_round_trips_every_variant() {
        for command in every_command() {
            store::os_store::test_support::assert_op_line_round_trip(&command);
        }
        // The two `None`-field variants missing from `every_command` (kept distinct from their
        // `Some` counterpart above, matching the pre-migration wire-baseline capture).
        store::os_store::test_support::assert_op_line_round_trip(&DrawCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: None }));
        store::os_store::test_support::assert_op_line_round_trip(&DrawCommand::SetHover(set_hover::SetHover { id: None }));
    }

    #[test]
    fn draw_command_op_binary_round_trips_every_variant() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// 🔖️ Pins the exact pre-migration hex for the two rows whose `Option` fields make `None`/`Some`
    /// distinct wire cases — copied verbatim from the `wire-baseline-before.txt` capture taken from
    /// the OLD `draw_protocol` crate before this migration. A byte-for-byte diff, not just a
    /// round-trip law, since round-trip alone would happily pass on a changed-but-consistent format.
    #[test]
    fn optional_field_rows_keep_their_pre_migration_bytes() {
        use protocol::OpBinary;
        let engagement_submit_some = DrawCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some("Renamed \"layer\"".into()) });
        assert_eq!(engagement_submit_some.encode_op().expect("encode"), hex_bytes("0105010f52656e616d656420226c617965722201000600"));
        let engagement_submit_none = DrawCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: None });
        assert_eq!(engagement_submit_none.encode_op().expect("encode"), hex_bytes("01050000"));
        let set_hover_some = DrawCommand::SetHover(set_hover::SetHover { id: Some("a".into()) });
        assert_eq!(set_hover_some.encode_op().expect("encode"), hex_bytes("011301016101000600"));
        let set_hover_none = DrawCommand::SetHover(set_hover::SetHover { id: None });
        assert_eq!(set_hover_none.encode_op().expect("encode"), hex_bytes("01130000"));
    }

    fn hex_bytes(hex: &str) -> Vec<u8> {
        (0..hex.len()).step_by(2).map(|i| u8::from_str_radix(&hex[i..i + 2], 16).expect("valid hex")).collect()
    }

    #[test]
    fn every_command_row_prints_starting_with_its_wire_keyword() {
        use protocol::OpText;
        let expected_keywords = [
            "set-snapshot",
            "commit-document",
            "fixture-json",
            "active-example",
            "selected-opacity",
            "engagement-submit",
            "add-layer",
            "drop-layer-kind",
            "move-layer",
            "delete-layer",
            "duplicate-layer",
            "toggle-layer-visible",
            "combine-boolean",
            "patch-layer",
            "patch-layers",
            "active-utility",
            "camera",
            "camera-zoom",
            "set-selection",
            "set-hover",
            "select-all",
            "clear-selection",
            "engagement-input",
            "locale",
            "canvas-pointer-down",
            "canvas-pointer-move",
            "canvas-pointer-up",
            "canvas-double-click",
            "canvas-commit-draft",
            "canvas-escape",
        ];
        for (command, keyword) in every_command().into_iter().zip(expected_keywords) {
            let printed = command.print_op();
            assert!(printed.starts_with(keyword), "expected '{printed}' to start with '{keyword}'");
        }
    }
    //#endregion 🔖️WireGuards
}
//#endregion 🧪️Tests
