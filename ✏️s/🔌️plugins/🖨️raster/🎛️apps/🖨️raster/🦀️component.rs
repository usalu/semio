//! 🖥️ Raster app — DocumentApp impl, render, manifest (constitutional: ui/general). B1: `RasterPlayApp`
//! is a unit struct — every former `RasterConfig` (`ui`-crate `RefCell`) field (selection, hover, brush
//! size/opacity, navigator composite-viewport size, the session-only free camera) now lives in
//! `crate::apps::raster::config::RasterConfig`, written via `RasterConfigMutation`s. Every action
//! dispatches through the single typed `RasterCommand` channel via `app_commands!` — mirrors
//! `shooting_ui`'s B1 pilot.

use crate::apps::raster::config::{RasterConfig, RasterConfigMutation};
use crate::apps::raster::presence::{RasterPresence, RasterPresenceMutation};
use crate::apps::raster::modes::edit;
use crate::apps::raster::modes::edit::windows::{composite, navigator};
use crate::apps::raster::terminology::raster_play_labels;
use crate::artifacts::raster::engine::{raster_composite_media, raster_io, semio_example_json};
use crate::artifacts::raster::op::RasterMutation;
use crate::artifacts::raster::{RasterLayerNode, RasterSnapshot as RasterSnapshot, RASTER_DOCUMENT_SCHEMA};
use semio_framework_plugin::{NoDraft, NoDraftMutation, DraftView, 
    App, ActionArgDef, ActionArgOption, ActionDescriptor, ActionFactory, ActionKind, ArtifactKindSpec, ConfigView, DocumentApp, DocumentView, Emit, Fault, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType,
    OsMediaCapability, OsMediaFormat, UiNode, UtilityCategory, UtilityDefinition, WindowMeasure,
};
use store::EngineHandles;
use serde_json::Value;
use std::collections::HashMap;
use store::DocumentPack;

//#region 🔖️Constants
pub const RASTER_PLAY_APP_ID: &str = "raster-play";
pub const RASTER_PLAY_CONTROLLER_ID: &str = "raster-play";
/// 🌳️ Prefix for every layer-tree row id — shared by the document/masks panels and the `moveLayer`
/// command (which needs to decode a `target_row_id` back into a layer/group id). App-wide tree-encoding
/// concern, not artifact data, so it lives here rather than in any single panel.
pub const RASTER_TREE_PREFIX: &str = "raster-play-layers";
//#endregion 🔖️Constants

//#region 🔖️Document
/// 🌳️ Encodes a layer as its tree-row id — shared by the document/masks panels (which render rows) and
/// `moveLayer` (which decodes a drop target back into an id). More than one consumer, but this is UI row
/// encoding, not artifact data, so it stays app-level rather than in `crate::artifacts::raster::engine`.
pub fn layer_row_id(layer: &RasterLayerNode) -> String {
    let segment = match layer {
        RasterLayerNode::Group { .. } => "group",
        RasterLayerNode::Adjustment { .. } => "adjustment",
        RasterLayerNode::Pixel { .. } => "layer",
    };
    format!("{RASTER_TREE_PREFIX}.{segment}.{}", crate::artifacts::raster::engine::layer_node_id(layer))
}

pub fn layer_id_from_tree_row_id(row_id: &str) -> Option<String> {
    row_id.strip_prefix(&format!("{RASTER_TREE_PREFIX}.")).and_then(|rest| rest.split('.').nth(1)).map(str::to_string)
}

pub fn mask_row_id(target_id: &str) -> String {
    format!("{RASTER_TREE_PREFIX}.mask.{target_id}")
}

/// 📡️ Document JSON for the WASM compositor, omitting embedded assets/utility/brush — mirrors
/// premigration `rasterDocumentToSyncJson`. Takes `&RasterConfig` nowhere directly (assets live on the
/// document), but stays app-level next to {@link raster_scene}, its only caller.
fn document_sync_json(document: &RasterSnapshot) -> String {
    let mut value = serde_json::to_value(document).unwrap_or(Value::Null);
    if let Value::Object(ref mut map) = value {
        map.remove("assets");
        map.remove("brushSize");
        map.remove("brushOpacity");
    }
    value.to_string()
}

/// 🎞️ Builds the shared `Paint2dScene` payload for both the composite and navigator windows. Takes
/// `&RasterConfig` (an app-only view-state type), so per TEMPLATE.md §4's `DocumentHelpers` placement
/// rule this stays at app level even though it has two window consumers.
pub fn raster_scene(document: &RasterSnapshot, runtime: &RasterConfig, active_utility: &str, view_mode: &str) -> semio_framework_plugin::Paint2dScene {
    semio_framework_plugin::Paint2dScene {
        document_sync_json: document_sync_json(document),
        assets_json: serde_json::to_string(&document.assets).unwrap_or_else(|_| "{}".into()),
        camera_json: serde_json::to_string(&runtime.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into()),
        selection_json: serde_json::to_string(&runtime.selected_ids).unwrap_or_else(|_| "[]".into()),
        hovered_id: runtime.hovered_id.clone(),
        active_utility: active_utility.into(),
        brush_size: runtime.brush_size,
        brush_opacity: runtime.brush_opacity,
        view_mode: view_mode.into(),
        composite_viewport_json: runtime.composite_viewport.as_ref().map(|viewport| serde_json::to_string(viewport).unwrap_or_else(|_| "{}".into())),
    }
}

/// 🎬️ Builds an `ActionDescriptor` dispatched through the raster app's single controller — the one call
/// site every window/panel/option goes through.
pub fn raster_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionFactory::new(RASTER_PLAY_CONTROLLER_ID).action(action, args)
}
//#endregion 🔖️Document

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `RasterPlayApp::Command` — the SOLE dispatch surface for raster's own behavior, assembled from
    /// the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id (`command_id()`,
    /// the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the kebab-case
    /// `#[dsl(key = ..)]` the binary/text codec uses) — different vocabularies, copied verbatim off the
    /// old `raster_protocol::RasterCommand` enum's `#[dsl(key)]` attributes and
    /// `RasterPlayApp::command_id` match arms respectively. **Row order is the binary variant ordinal:
    /// appending is safe, reordering is a wire-format break.**
    pub enum RasterCommand for RasterSnapshot, RasterMutation, RasterConfig, RasterConfigMutation {
        "setSnapshot" as "set-snapshot" => set_snapshot::SetSnapshot,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "addLayer" as "add-layer" => add_layer::AddLayer,
        "dropLayerKind" as "drop-layer-kind" => drop_layer_kind::DropLayerKind,
        "setLayerVisible" as "set-layer-visible" => set_layer_visible::SetLayerVisible,
        "toggleLayerVisible" as "toggle-layer-visible" => toggle_layer_visible::ToggleLayerVisible,
        "deleteLayer" as "delete-layer" => delete_layer::DeleteLayer,
        "duplicateLayer" as "duplicate-layer" => duplicate_layer::DuplicateLayer,
        "patchLayer" as "patch-layer" => patch_layer::PatchLayer,
        "patchLayers" as "patch-layers" => patch_layers::PatchLayers,
        "moveLayer" as "move-layer" => move_layer::MoveLayer,
        "setSelection" as "set-selection" => set_selection::SetSelection,
        "setHover" as "set-hover" => set_hover::SetHover,
        "selectAll" as "select-all" => select_all::SelectAll,
        "setBrushSize" as "brush-size" => set_brush_size::SetBrushSize,
        "setBrushOpacity" as "brush-opacity" => set_brush_opacity::SetBrushOpacity,
        "setCompositeViewport" as "composite-viewport" => set_composite_viewport::SetCompositeViewport,
        "setCamera" as "camera" => set_camera::SetCamera,
        "setCameraZoom" as "camera-zoom" => set_camera_zoom::SetCameraZoom,
        "setActiveUtility" as "active-utility" => set_active_utility::SetActiveUtility,
        "setLocale" as "locale" => set_locale::SetLocale,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use crate::apps::raster::commands::brush::{set_brush_opacity, set_brush_size};
use crate::apps::raster::commands::camera::{set_camera, set_camera_zoom, set_composite_viewport};
use crate::apps::raster::commands::document::{set_active_example, set_snapshot};
use crate::apps::raster::commands::layer::{add_layer, delete_layer, drop_layer_kind, duplicate_layer, move_layer, patch_layer, patch_layers, set_layer_visible, toggle_layer_visible};
use crate::apps::raster::commands::locale::set_locale;
use crate::apps::raster::commands::selection::{select_all, set_hover, set_selection};
use crate::apps::raster::commands::utility::set_active_utility;
//#endregion 🔖️Commands

//#region 🔖️RasterPlayApp
/// 🧪️ B1: unit struct — every former `RasterConfig` field now lives in
/// `crate::apps::raster::config::RasterConfig`, written through `RasterConfigMutation`s.
#[derive(Default)]
pub struct RasterPlayApp;

impl DocumentApp for RasterPlayApp {
    type Snapshot = RasterSnapshot;
    type Mutation = RasterMutation;
    type Config = RasterConfig;
    type ConfigMutation = RasterConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = RasterPresence;
    type PresenceMutation = RasterPresenceMutation;

    type Command = RasterCommand;

    const APP_ID: &'static str = RASTER_PLAY_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = RASTER_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> RasterSnapshot {
        crate::artifacts::raster::engine::empty_raster_document()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(raster_io())
    }

    /// 🎞️ `image:in`/`image:out` (see `crate::artifacts::raster::engine::{raster_append_image_layer,
    /// raster_composite_media}`) plus the inherited `document:out` default (the pack of
    /// `doc.snapshot`, replicated inline — overriding `export_media` shadows the trait's provided
    /// body for every port on this app, not just the new ones).
    fn export_media(port: &str, doc: &DocumentView<'_, RasterSnapshot>) -> Result<Media, MediaError> {
        match port {
            "image:out" => raster_composite_media(doc.snapshot),
            "document:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.document_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🎞️ `image:in` inserts the incoming raster media as a new composited layer + embedded asset
    /// (`raster_append_image_layer`) via a whole-document `ReplaceDocument` — `RasterMutation` has
    /// no granular "add asset" step (see that function's doc). Falls through to the inherited
    /// `document:in` default (base64 pack replace) for any other port.
    fn import_media(port: &str, media: &Media, doc: &DocumentView<'_, RasterSnapshot>) -> Result<Emit<RasterMutation, RasterConfigMutation, Self::DraftMutation>, MediaError> {
        if port != "image:in" {
            return Err(MediaError::NotImplemented);
        }
        let MediaPayload::Structured { json: png_base64, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "image:in only accepts a Structured (base64 PNG) payload".into()));
        };
        let next = crate::artifacts::raster::engine::raster_append_image_layer(doc.snapshot, png_base64);
        Ok(Emit::mutations(vec![RasterMutation::SetSnapshot { snapshot: next }]))
    }

    fn whole_document_operation(snapshot: RasterSnapshot) -> Option<RasterMutation> {
        Some(RasterMutation::SetSnapshot { snapshot })
    }

    fn command_id(command: &RasterCommand) -> &'static str {
        command.command_id()
    }

    fn handle(command: &RasterCommand, doc: &DocumentView<'_, RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<RasterMutation, RasterConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn window_measures(_doc: &DocumentView<'_, RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        HashMap::from([(composite::RASTER_PLAY_WINDOW_COMPOSITE.into(), composite::window_measures(cfg.snapshot))])
    }

    fn render(body_key: &str, doc: &DocumentView<'_, RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>) -> UiNode {
        let document = doc.snapshot;
        let config = cfg.snapshot;
        let labels = raster_play_labels(config);
        match body_key {
            composite::RASTER_PLAY_BODY_COMPOSITE => composite::render(document, config),
            navigator::RASTER_PLAY_BODY_NAVIGATOR => navigator::render(document, config),
            crate::apps::raster::panels::document::RASTER_PLAY_BODY_LAYERS => crate::apps::raster::panels::document::render(document, config, labels),
            crate::apps::raster::panels::masks::RASTER_PLAY_BODY_MASKS => crate::apps::raster::panels::masks::render(document, config, labels),
            crate::apps::raster::panels::catalogue::RASTER_PLAY_BODY_CATALOGUE => crate::apps::raster::panels::catalogue::render(labels),
            crate::apps::raster::panels::inspection::RASTER_PLAY_BODY_PROPERTIES => crate::apps::raster::panels::inspection::render(document, config, labels),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️RasterPlayApp

//#region 🔖️Manifest
/// 🛠️ An internal (non-palette) action declaration — the panel/pointer/gesture-bound vocabulary
/// dispatched by the layer tree, catalogue drops and inspector, never a palette command.
fn raster_internal_action(id: &str, label: impl Into<LocalizedLabel>, kind: ActionKind) -> semio_framework_plugin::ActionDefinition {
    semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new_catalog(id, label, kind) }
}

/// 🧰️ One composite-window utility declaration; ids must stay host-compatible (`paint*` prefix paints,
/// `paintEraser` erases, `selectMarquee` selects) because the scene's active utility feeds `RasterHost`.
fn raster_utility(id: &str, label: impl Into<LocalizedLabel>, icon: &str, group: &str, category: UtilityCategory) -> UtilityDefinition {
    UtilityDefinition { group: Some(group.into()), category: Some(category), ..UtilityDefinition::new(id, label, icon) }
}

/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding/utility declarations (which have no dedicated `_def` passthrough) are
/// written out inline.
pub fn create_raster_app() -> App {
    App::from_builder(
        App::builder(RASTER_PLAY_APP_ID, LocalizedLabel::native("Raster", "Raster")).document(["semio", "raster"])
            .artifact_kind(crate::artifacts::raster::artifact_kind())
            // 🖼️ `2d.image` — the interchange kind `image:out` produces (WORKFLOWS-END-TO-END-TYPED-PORTS
            // Wave 2 port recipe); `shooting`'s `photos:out` already declares the identical shape — a
            // harmless duplicate registration (registry dedupes by id).
            .artifact_kind(ArtifactKindSpec {
                id: "2d.image".into(),
                name: "2D Image".into(),
                source_format: "2d.image".into(),
                component_kind: "image".into(),
                dimension: "2d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
                schema: "2d.image".into(),
                export_formats: vec![OsMediaFormat::Png],
                import_formats: vec![OsMediaFormat::Png],
            })
            .icon_id("raster")
            .mode_def(edit::definition())
            .default_mode_id(edit::RASTER_PLAY_MODE_EDIT)
            .window_kind_def(composite::definition())
            .window_kind_def(navigator::definition())
            .default_layout(edit::layout())
            .panel_tab_def(crate::apps::raster::panels::document::definition())
            .panel_tab_def(crate::apps::raster::panels::catalogue::definition())
            .panel_tab_def(crate::apps::raster::panels::masks::definition())
            .panel_tab_def(crate::apps::raster::panels::inspection::definition())
            // ✏️ Palette-visible content operations.
            .mutation("addLayer", LocalizedLabel::native("Add Layer", "Ebene hinzufügen"))
            .mutation("setSnapshot", LocalizedLabel::native("Set Document", "Dokument festlegen"))
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            // 🔧️ Internal content operations — layer-tree / catalogue-drop / inspector bound.
            .action_with(raster_internal_action("setLayerVisible", LocalizedLabel::native("Set Layer Visible", "Ebenensichtbarkeit festlegen"), ActionKind::Mutation))
            .action_with(raster_internal_action("toggleLayerVisible", LocalizedLabel::native("Toggle Layer Visible", "Ebenensichtbarkeit umschalten"), ActionKind::Mutation))
            .action_with(raster_internal_action("dropLayerKind", LocalizedLabel::native("Drop Layer Kind", "Ebenenart ablegen"), ActionKind::Mutation))
            .action_with(raster_internal_action("deleteLayer", LocalizedLabel::native("Delete Layer", "Ebene löschen"), ActionKind::Mutation))
            .action_with(raster_internal_action("duplicateLayer", LocalizedLabel::native("Duplicate Layer", "Ebene duplizieren"), ActionKind::Mutation))
            .action_with(raster_internal_action("patchLayer", LocalizedLabel::native("Patch Layer", "Ebene aktualisieren"), ActionKind::Mutation))
            .action_with(raster_internal_action("patchLayers", LocalizedLabel::native("Patch Layers", "Ebenen aktualisieren"), ActionKind::Mutation))
            .action_with(raster_internal_action("moveLayer", LocalizedLabel::native("Move Layer", "Ebene verschieben"), ActionKind::Mutation))
            // 👁️ Ephemeral view state — selection, hover, live brush controls, navigator viewport, camera.
            .view_action("selectAll", LocalizedLabel::native("Select All", "Alles auswählen"))
            .action_with(raster_internal_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"), ActionKind::View))
            .action_with(raster_internal_action("setHover", LocalizedLabel::native("Set Hover", "Überfahren festlegen"), ActionKind::View))
            .action_with(raster_internal_action("setBrushSize", LocalizedLabel::native("Set Brush Size", "Pinselgröße festlegen"), ActionKind::View))
            .action_with(raster_internal_action("setBrushOpacity", LocalizedLabel::native("Set Brush Opacity", "Pinseldeckkraft festlegen"), ActionKind::View))
            .action_with(raster_internal_action("setCompositeViewport", LocalizedLabel::native("Set Composite Viewport", "Komposit-Ansichtsfenster festlegen"), ActionKind::View))
            .action_with(raster_internal_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View))
            .action_with(raster_internal_action("setCameraZoom", LocalizedLabel::native("Set Camera Zoom", "Kamerazoom festlegen"), ActionKind::View))
            .action_with(raster_internal_action("setLocale", LocalizedLabel::native("Set Locale", "Sprache festlegen"), ActionKind::View))
            // 📝️ Staged palette-form arguments for the two palette operations.
            .action_args("addLayer", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Layer Kind", "Ebenenart"), vec![
                    ActionArgOption::new("pixel", LocalizedLabel::native("Pixel", "Pixel")),
                    ActionArgOption::new("group", LocalizedLabel::native("Group", "Gruppe")),
                    ActionArgOption::new("adjustment", LocalizedLabel::native("Adjustment", "Anpassung")),
                ]).required().default_value("pixel"),
            ])
            .action_args("setSnapshot", vec![
                ActionArgDef::text("document", LocalizedLabel::native(semio_framework_plugin::FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument")),
            ])
            // 🧰️ Composite-window utilities — one exclusive set, active utility host-owned (never a document operation).
            .utility(raster_utility("selectMarquee", LocalizedLabel::native("Marquee Select", "Rahmenauswahl"), "square-dashed", "Select", UtilityCategory::Selection))
            .utility(raster_utility("paintBrush", LocalizedLabel::native("Brush", "Pinsel"), "paintbrush", "Paint", UtilityCategory::Utilities))
            .utility(raster_utility("paintEraser", LocalizedLabel::native("Eraser", "Radiergummi"), "eraser", "Paint", UtilityCategory::Utilities))
            .window_kind_utilities(composite::RASTER_PLAY_WINDOW_COMPOSITE, vec![
                "selectMarquee".into(), "paintBrush".into(), "paintEraser".into(),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .example("semio", LocalizedLabel::data("Semio"), semio_example_json(), "sparkles")
    .workflow("raster", "Raster", "2d.raster")
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
pub(crate) mod testkit {
    //! 🧪️ Shared harness for every `apps::raster` node's tests — mirrors TEMPLATE.md §7.
    use super::*;
    use semio_framework_plugin::{testkit as framework_testkit, InvocationResult, VcsDocumentApp, ViewModel};

    pub type RasterApp = VcsDocumentApp<RasterPlayApp>;

    use semio_framework_plugin::PluginApp;

    pub fn app() -> RasterApp {
        framework_testkit::new_app::<RasterPlayApp>()
    }

    pub fn app_with_registry() -> RasterApp {
        framework_testkit::new_app_with_registry::<RasterPlayApp>(create_raster_app)
    }

    pub fn dispatch(app: &mut RasterApp, command: RasterCommand) -> InvocationResult {
        app.dispatch_typed(command, &framework_testkit::meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut RasterApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).unwrap()
    }

    pub fn main_window_measures(app: &mut RasterApp) -> Vec<WindowMeasure> {
        app.window_measures().remove(composite::RASTER_PLAY_WINDOW_COMPOSITE).unwrap_or_default()
    }

    pub fn semio_app() -> RasterApp {
        let mut app = framework_testkit::new_app::<RasterPlayApp>();
        let document = crate::artifacts::raster::engine::semio_example_document();
        let envelope = store::create_document_envelope::<RasterSnapshot, RasterMutation>(RASTER_DOCUMENT_SCHEMA, "raster", document, None);
        let files = store::print_document_pack(&envelope).expect("print document pack");
        app.load_document_pack(&files).expect("load semio");
        app
    }
}

#[cfg(test)]
mod tests {
    use super::testkit::*;
    use super::*;
    use crate::apps::raster::panels::{catalogue, document, inspection, masks};
    use crate::artifacts::raster::engine::{empty_raster_document, layer_name, layer_visible};
    use semio_framework_plugin::{testkit, PluginApp, SET_ACTIVE_UTILITY_ACTION_ID};
    use store::MemoryBackbone;

    #[test]
    fn window_measures_expose_brush_and_eraser_option_groups() {
        let mut app = app();
        let measures = main_window_measures(&mut app);
        assert_eq!(measures.len(), 2);
        assert!(measures.iter().any(|m| matches!(m, WindowMeasure::Group { id, .. } if id == "raster-utility-options-paintBrush")));
    }

    #[test]
    fn renders_raster_scene() {
        let mut app = app();
        let json = render(&mut app, composite::RASTER_PLAY_BODY_COMPOSITE);
        assert!(json.contains("raster"));
    }

    #[test]
    fn renders_navigator_scene() {
        let mut app = app();
        let json = render(&mut app, navigator::RASTER_PLAY_BODY_NAVIGATOR);
        assert!(json.contains("\"componentKind\":\"paint-2d\""));
        assert!(json.contains("\"viewMode\":\"navigator\""));
    }

    #[test]
    fn parses_semio_example_document() {
        let document = crate::artifacts::raster::engine::semio_example_document();
        assert!(!document.layers.is_empty());
    }

    #[test]
    fn empty_document_background_layer_has_identity_scale() {
        let document = empty_raster_document();
        let json = document_sync_json(&document);
        assert!(json.contains(r#""scaleX":1.0"#), "expected identity scale in {json}");
        assert!(json.contains(r#""scaleY":1.0"#), "expected identity scale in {json}");
        assert!(!json.contains(r#""scaleX":0.0"#), "layer must not collapse to zero size");
    }

    #[test]
    fn renders_layers_tree() {
        let mut app = semio_app();
        let json = render(&mut app, document::RASTER_PLAY_BODY_LAYERS);
        assert!(json.contains("\"type\":\"tree\""));
        assert!(json.contains("Backdrop"));
    }

    #[test]
    fn raster_labels_resolve_native_english_by_default() {
        let mut app = app();
        let layers_json = render(&mut app, document::RASTER_PLAY_BODY_LAYERS);
        assert!(layers_json.contains("Add Pixel"));
        assert!(layers_json.contains("Add Group"));
        let masks_json = render(&mut app, masks::RASTER_PLAY_BODY_MASKS);
        assert!(masks_json.contains("Masks"));
        assert!(masks_json.contains("No masks"));
        let catalogue_json = render(&mut app, catalogue::RASTER_PLAY_BODY_CATALOGUE);
        assert!(catalogue_json.contains("Layer kinds"));
        let properties_json = render(&mut app, inspection::RASTER_PLAY_BODY_PROPERTIES);
        assert!(properties_json.contains("Schema:"));
    }

    #[test]
    fn raster_labels_resolve_german_locale() {
        let mut app = app();
        dispatch(&mut app, RasterCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }));
        let layers_json = render(&mut app, document::RASTER_PLAY_BODY_LAYERS);
        assert!(layers_json.contains("Pixel hinzufügen"));
        assert!(layers_json.contains("Gruppe hinzufügen"));
        let masks_json = render(&mut app, masks::RASTER_PLAY_BODY_MASKS);
        assert!(masks_json.contains("Masken"));
        assert!(masks_json.contains("Keine Masken"));
        let catalogue_json = render(&mut app, catalogue::RASTER_PLAY_BODY_CATALOGUE);
        assert!(catalogue_json.contains("Ebenenarten"));
    }

    #[test]
    fn composite_scene_syncs_document_and_assets() {
        let mut app = semio_app();
        let json = render(&mut app, composite::RASTER_PLAY_BODY_COMPOSITE);
        assert!(json.contains("\"componentKind\":\"paint-2d\""));
        assert!(json.contains("\"viewMode\":\"composite\""));
        assert!(!json.contains("\"assetsJson\":\"{}\""), "semio fixture has embedded assets");
        let document = crate::artifacts::raster::engine::semio_example_document();
        let sync_json = document_sync_json(&document);
        assert!(!sync_json.contains("\"assets\""), "sync json must omit assets");
        assert!(sync_json.contains("\"params\""), "adjustment params must survive document→sync roundtrip for the paint host");
        let sync_value: Value = serde_json::from_str(&sync_json).expect("sync json");
        let layers = sync_value.get("layers").and_then(Value::as_array).expect("layers");
        assert!(layers.iter().any(|layer| layer.get("kind").and_then(Value::as_str) == Some("adjustment") && layer.get("params").is_some()));
        assert!(document.assets.contains_key("semio-emblem"));
    }

    #[test]
    fn semio_example_preserves_adjustment_params() {
        let document = crate::artifacts::raster::engine::semio_fixture_snapshot();
        let RasterLayerNode::Adjustment { params, adjustment_kind, .. } = document.layers.iter().find(|layer| matches!(layer, RasterLayerNode::Adjustment { id, .. } if id == "brighten")).expect("brighten adjustment") else {
            panic!("expected adjustment");
        };
        assert_eq!(adjustment_kind, "brightnessContrast");
        assert!(params.contains_key("brightness"), "fixture brightness must roundtrip");
        assert!(params.contains_key("contrast"), "fixture contrast must roundtrip");
    }

    #[test]
    fn set_hover_highlights_layer_row_via_runtime() {
        let mut app = semio_app();
        let layer_id = crate::artifacts::raster::engine::layer_node_id(&app.snapshot().expect("snapshot").layers[0]).to_string();
        let row_id = layer_row_id(crate::artifacts::raster::engine::find_layer(&app.snapshot().expect("snapshot").layers, &layer_id).expect("layer"));
        let result = app.dispatch_typed(RasterCommand::SetHover(set_hover::SetHover { id: Some(layer_id) }), &testkit::meta("local")).expect("hover");
        assert!(result.mutations.is_empty(), "hover is a view action and emits no operations");
        let json = render(&mut app, document::RASTER_PLAY_BODY_LAYERS);
        assert!(json.contains(&format!("\"id\":\"{row_id}\"")), "hovered layer row must be present");
        assert!(json.contains("\"state\":\"previewed\""), "hover stamps UiState::Previewed onto the layer row");
    }

    #[test]
    fn set_composite_viewport_feeds_navigator_scene() {
        let mut app = app();
        dispatch(&mut app, RasterCommand::SetCompositeViewport(set_composite_viewport::SetCompositeViewport { width: 640.0, height: 480.0 }));
        let json = render(&mut app, navigator::RASTER_PLAY_BODY_NAVIGATOR);
        assert!(json.contains("compositeViewportJson"));
        assert!(json.contains(r#"\"width\":640.0"#));
        assert!(json.contains(r#"\"height\":480.0"#));
    }

    #[test]
    fn set_camera_mutates_runtime_and_emits_no_operations() {
        let mut app = app();
        let before = app.snapshot().expect("snapshot");
        let result = dispatch(&mut app, RasterCommand::SetCamera(set_camera::SetCamera { camera: crate::artifacts::raster::RasterCamera { x: 4.0, y: 5.0, zoom: 2.0 } }));
        assert!(result.mutations.is_empty(), "camera is a view action and emits no operations");
        assert_eq!(app.snapshot().expect("snapshot"), before, "camera never mutates the document");
        let json = render(&mut app, composite::RASTER_PLAY_BODY_COMPOSITE);
        assert!(json.contains(r#"\"zoom\":2.0"#), "composite scene camera reflects runtime state: {json}");
        assert!(json.contains(r#"\"x\":4.0"#), "composite scene camera reflects runtime state: {json}");
    }

    #[test]
    fn set_camera_zoom_updates_zoom_and_keeps_pan_via_runtime() {
        let mut app = app();
        dispatch(&mut app, RasterCommand::SetCamera(set_camera::SetCamera { camera: crate::artifacts::raster::RasterCamera { x: 4.0, y: 5.0, zoom: 1.0 } }));
        let result = dispatch(&mut app, RasterCommand::SetCameraZoom(set_camera_zoom::SetCameraZoom { zoom: 3.0 }));
        assert!(result.mutations.is_empty(), "camera zoom is a view action and emits no operations");
        let json = render(&mut app, composite::RASTER_PLAY_BODY_COMPOSITE);
        assert!(json.contains(r#"\"zoom\":3.0"#), "zoom updated: {json}");
        assert!(json.contains(r#"\"x\":4.0"#), "pan preserved across zoom-only update: {json}");
    }

    #[test]
    fn add_layer_action_appends_and_undo_removes() {
        let mut app = app();
        let before = app.snapshot().expect("snapshot").layers.len();
        dispatch(&mut app, RasterCommand::AddLayer(add_layer::AddLayer { kind: "group".into() }));
        let projection = app.snapshot().expect("snapshot");
        assert_eq!(projection.layers.len(), before + 1);
        assert!(matches!(projection.layers.last().unwrap(), RasterLayerNode::Group { .. }));
        app.handle_action("undo", None, &testkit::meta("local")).expect("undo");
        assert_eq!(app.snapshot().expect("snapshot").layers.len(), before);
    }

    #[test]
    fn patch_layer_renames_and_toggles_visibility_round_trip() {
        let mut app = app();
        let layer_id = crate::artifacts::raster::engine::layer_node_id(&app.snapshot().expect("snapshot").layers[0]).to_string();
        dispatch(&mut app, RasterCommand::PatchLayer(patch_layer::PatchLayer { layer_id: layer_id.clone(), field: "name".into(), value: "Renamed".into() }));
        assert_eq!(layer_name(&app.snapshot().expect("snapshot").layers[0]), "Renamed");
        dispatch(&mut app, RasterCommand::ToggleLayerVisible(toggle_layer_visible::ToggleLayerVisible { layer_id }));
        assert!(!layer_visible(&app.snapshot().expect("snapshot").layers[0]));
        app.handle_action("undo", None, &testkit::meta("local")).expect("undo toggle");
        assert!(layer_visible(&app.snapshot().expect("snapshot").layers[0]));
    }

    #[test]
    fn move_layer_into_group() {
        let mut app = app();
        dispatch(&mut app, RasterCommand::AddLayer(add_layer::AddLayer { kind: "group".into() }));
        let (group_id, pixel_id) = {
            let projection = app.snapshot().expect("snapshot");
            let group = projection.layers.iter().find(|layer| matches!(layer, RasterLayerNode::Group { .. })).unwrap();
            let pixel = projection.layers.iter().find(|layer| matches!(layer, RasterLayerNode::Pixel { .. })).unwrap();
            (crate::artifacts::raster::engine::layer_node_id(group).to_string(), crate::artifacts::raster::engine::layer_node_id(pixel).to_string())
        };
        let target_row = format!("{RASTER_TREE_PREFIX}.group.{group_id}");
        dispatch(&mut app, RasterCommand::MoveLayer(move_layer::MoveLayer { layer_id: pixel_id.clone(), target_row_id: target_row, drop_position: "after".into() }));
        let projection = app.snapshot().expect("snapshot");
        let RasterLayerNode::Group { children, .. } = projection.layers.iter().find(|layer| crate::artifacts::raster::engine::layer_node_id(layer) == group_id).unwrap() else {
            panic!("expected group");
        };
        assert_eq!(children.len(), 1);
        assert_eq!(crate::artifacts::raster::engine::layer_node_id(&children[0]), pixel_id);
    }

    /// 🧪️ The definitional merge proof: A adds a layer while B renames the background layer — disjoint
    /// tree edits on one backbone that must both survive on both instances.
    #[test]
    fn two_instances_converge_disjoint_layer_edits_via_backbone() {
        let mut instance_a = app();
        let mut instance_b = app();
        // Seed both from an identical base projection (a background layer with a fixed id) so B's
        // rename targets the same layer A holds — per-instance `initial_snapshot` mints fresh ids.
        let mut base = crate::artifacts::raster::engine::empty_raster_snapshot();
        base.layers = vec![RasterLayerNode::Pixel {
            id: "bg".into(),
            name: "Background".into(),
            visible: true,
            opacity: 1.0,
            blend_mode: "normal".into(),
            transform: crate::artifacts::raster::RasterTransform::default(),
            mask: None,
            width: Some(512),
            height: Some(512),
            image_key: None,
        }];
        let base_envelope = store::create_document_envelope::<RasterSnapshot, RasterMutation>(RASTER_DOCUMENT_SCHEMA, "raster", base, None);
        let base_files = store::print_document_pack(&base_envelope).expect("print document pack");
        instance_a.load_document_pack(&base_files).expect("load a");
        instance_b.load_document_pack(&base_files).expect("load b");
        let background_id = "bg".to_string();
        let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://raster-convergence", "mem://raster-convergence");
        instance_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        instance_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

        dispatch(&mut instance_a, RasterCommand::AddLayer(add_layer::AddLayer { kind: "pixel".into() }));
        dispatch(&mut instance_b, RasterCommand::PatchLayer(patch_layer::PatchLayer { layer_id: background_id, field: "name".into(), value: "Renamed By B".into() }));

        instance_a.handle_action("commitCheckpoint", None, &testkit::meta("actor-a")).expect("pump a");
        instance_b.handle_action("commitCheckpoint", None, &testkit::meta("actor-b")).expect("pump b");

        let projection_a = instance_a.snapshot().expect("projection a");
        let projection_b = instance_b.snapshot().expect("projection b");
        assert_eq!(projection_a.layers.len(), 2, "A keeps its added layer");
        assert_eq!(projection_b.layers.len(), 2, "B converges on A's added layer");
        assert_eq!(layer_name(&projection_a.layers[0]), "Renamed By B", "A converges on B's rename");
        assert_eq!(layer_name(&projection_b.layers[0]), "Renamed By B", "B keeps its rename");
    }

    #[test]
    fn ingest_operations_is_idempotent() {
        testkit::assert_ingest_idempotent::<RasterPlayApp, usize>(RasterCommand::AddLayer(add_layer::AddLayer { kind: "pixel".into() }), |app| app.snapshot().unwrap().layers.len());
    }

    #[test]
    fn set_active_utility_switch_emits_no_ops_and_persists_in_config() {
        let mut app = app_with_registry();
        let before = app.snapshot().expect("snapshot");
        // Switching utilities is the framework View action: no document operations, nothing to sync/undo.
        let result = dispatch(&mut app, RasterCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "paintBrush".into() }));
        assert!(result.mutations.is_empty(), "utility switching never emits document operations");
        assert_eq!(app.snapshot().expect("snapshot"), before, "utility switching does not mutate the document");
        // The composite scene reads the host-owned active utility from config, not view state.
        let json = render(&mut app, composite::RASTER_PLAY_BODY_COMPOSITE);
        assert!(json.contains("\"activeUtility\":\"paintBrush\""), "scene reflects host-owned active utility: {json}");
    }

    #[test]
    fn utility_registry_declares_utilities_scoped_to_the_composite_window() {
        let definition = create_raster_app().definition;
        let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
        assert_eq!(utility_ids, ["selectMarquee", "paintBrush", "paintEraser"]);
        // The marquee carries the Selection category; the paint utilities are Tools.
        let selects: Vec<&str> = definition.utilities.iter().filter(|utility| utility.category == Some(UtilityCategory::Selection)).map(|utility| utility.id.as_str()).collect();
        assert_eq!(selects, ["selectMarquee"]);
        let composite = definition.window_kinds.iter().find(|window| window.id == composite::RASTER_PLAY_WINDOW_COMPOSITE).expect("composite window");
        assert_eq!(composite.utilities.len(), definition.utilities.len(), "every utility is scoped to the composite window kind");
        // The framework auto-injects the setActiveUtility View action once utilities are declared; no doc operation survives.
        assert!(definition.actions.iter().any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID && matches!(action.kind, ActionKind::View)));
        assert!(!definition.actions.iter().any(|action| action.id == "setActiveUtility" && !matches!(action.kind, ActionKind::View)));
    }

    #[test]
    fn raster_io_declares_image_in_out_and_export_media_covers_all_ports() {
        let projection = empty_raster_document();
        let doc = DocumentView { snapshot: &projection, history: &semio_framework_plugin::HistoryView::empty() };
        let app = RasterPlayApp;
        let image_out = RasterPlayApp::export_media("image:out", &doc).expect("image:out");
        let MediaPayload::Structured { schema, json } = image_out.payload else { panic!("expected structured payload") };
        assert_eq!(schema, "2d.image");
        assert!(!json.is_empty());
        assert!(RasterPlayApp::export_media("document:out", &doc).is_ok());
        assert!(matches!(RasterPlayApp::export_media("unknown:out", &doc), Err(MediaError::NotImplemented)));
    }

    #[test]
    fn raster_import_media_appends_layer_from_incoming_image() {
        let mut app = app();
        let before = app.snapshot().expect("snapshot").layers.len();
        let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster }, payload: MediaPayload::Structured { schema: "2d.image".into(), json: "aGVsbG8=".into() } };
        let result = app.import_media("image:in", &media, &testkit::meta("local")).expect("import image:in");
        assert!(!result.mutations.is_empty(), "image:in import must emit a real document operation");
        assert_eq!(app.snapshot().expect("snapshot").layers.len(), before + 1);
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order — TEMPLATE.md §7's
    /// permanent wire guard, feeding the round-trip/keyword-uniqueness/leading-token laws below.
    fn every_command() -> Vec<RasterCommand> {
        vec![
            RasterCommand::SetSnapshot(set_snapshot::SetSnapshot { snapshot: empty_raster_document() }),
            RasterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "semio".into() }),
            RasterCommand::AddLayer(add_layer::AddLayer { kind: "pixel".into() }),
            RasterCommand::DropLayerKind(drop_layer_kind::DropLayerKind { kind: "group".into() }),
            RasterCommand::SetLayerVisible(set_layer_visible::SetLayerVisible { layer_id: "l1".into(), visible: Some(true) }),
            RasterCommand::ToggleLayerVisible(toggle_layer_visible::ToggleLayerVisible { layer_id: "l1".into() }),
            RasterCommand::DeleteLayer(delete_layer::DeleteLayer { layer_id: "l1".into() }),
            RasterCommand::DuplicateLayer(duplicate_layer::DuplicateLayer { layer_id: "l1".into() }),
            RasterCommand::PatchLayer(patch_layer::PatchLayer { layer_id: "l1".into(), field: "opacity".into(), value: "0.4".into() }),
            RasterCommand::PatchLayers(patch_layers::PatchLayers { layer_ids: vec!["a".into(), "b".into()], field: "name".into(), value: "Renamed".into() }),
            RasterCommand::MoveLayer(move_layer::MoveLayer { layer_id: "l1".into(), target_row_id: "raster-play-layers".into(), drop_position: "after".into() }),
            RasterCommand::SetSelection(set_selection::SetSelection { ids: vec!["a".into()] }),
            RasterCommand::SetHover(set_hover::SetHover { id: Some("a".into()) }),
            RasterCommand::SelectAll(select_all::SelectAll {}),
            RasterCommand::SetBrushSize(set_brush_size::SetBrushSize { value: 40.0 }),
            RasterCommand::SetBrushOpacity(set_brush_opacity::SetBrushOpacity { value: 0.5 }),
            RasterCommand::SetCompositeViewport(set_composite_viewport::SetCompositeViewport { width: 640.0, height: 480.0 }),
            RasterCommand::SetCamera(set_camera::SetCamera { camera: crate::artifacts::raster::RasterCamera { x: 1.0, y: 2.0, zoom: 1.5 } }),
            RasterCommand::SetCameraZoom(set_camera_zoom::SetCameraZoom { zoom: 2.0 }),
            RasterCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "paintBrush".into() }),
            RasterCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
        ]
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[test]
    fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// 🎫️ Every `app_commands!` row's wire keyword must be distinct — the cross-cutting invariant the
    /// macro exists to hold.
    #[test]
    fn command_wire_keywords_are_unique_across_every_row() {
        let commands = every_command();
        assert_eq!(commands.len(), 21, "every RasterCommand row must be covered by every_command()");
        let mut keywords: Vec<String> = commands.iter().map(|command| protocol::OpText::print_op(command).split(' ').next().unwrap_or_default().to_string()).collect();
        keywords.sort();
        keywords.dedup();
        assert_eq!(keywords.len(), commands.len(), "every row's wire keyword must be distinct");
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — what a
    /// missing `#[dsl(keyword = ..)]` on a payload struct silently breaks (the record prints with no
    /// keyword at all and no longer parses).
    #[test]
    fn every_printed_op_line_starts_with_the_rows_declared_wire_keyword() {
        let expectations: Vec<(&str, RasterCommand)> = every_command()
            .into_iter()
            .map(|command| {
                let keyword: &'static str = match &command {
                    RasterCommand::SetSnapshot(_) => "set-snapshot",
                    RasterCommand::SetActiveExample(_) => "active-example",
                    RasterCommand::AddLayer(_) => "add-layer",
                    RasterCommand::DropLayerKind(_) => "drop-layer-kind",
                    RasterCommand::SetLayerVisible(_) => "set-layer-visible",
                    RasterCommand::ToggleLayerVisible(_) => "toggle-layer-visible",
                    RasterCommand::DeleteLayer(_) => "delete-layer",
                    RasterCommand::DuplicateLayer(_) => "duplicate-layer",
                    RasterCommand::PatchLayer(_) => "patch-layer",
                    RasterCommand::PatchLayers(_) => "patch-layers",
                    RasterCommand::MoveLayer(_) => "move-layer",
                    RasterCommand::SetSelection(_) => "set-selection",
                    RasterCommand::SetHover(_) => "set-hover",
                    RasterCommand::SelectAll(_) => "select-all",
                    RasterCommand::SetBrushSize(_) => "brush-size",
                    RasterCommand::SetBrushOpacity(_) => "brush-opacity",
                    RasterCommand::SetCompositeViewport(_) => "composite-viewport",
                    RasterCommand::SetCamera(_) => "camera",
                    RasterCommand::SetCameraZoom(_) => "camera-zoom",
                    RasterCommand::SetActiveUtility(_) => "active-utility",
                    RasterCommand::SetLocale(_) => "locale",
                };
                (keyword, command)
            })
            .collect();
        for (expected_keyword, command) in expectations {
            let printed = protocol::OpText::print_op(&command);
            assert_eq!(printed.split(' ').next().unwrap_or_default(), expected_keyword, "wire keyword drifted for {command:?}: {printed:?}");
        }
    }

    /// ⚖️ The rows whose `Option` fields make `None`/`Some` distinct wire cases, pinned to the exact
    /// bytes captured from the pre-merge `raster_protocol` crate (this ticket's
    /// `🧪️wire-baseline-before.txt`, rows for `set-layer-visible`/`set-hover` with `None`). A regression
    /// here is a real format break, not a test-fixture mismatch.
    #[test]
    fn optional_field_rows_keep_their_pre_migration_bytes() {
        let cases: [(RasterCommand, &str, &str); 2] = [
            (RasterCommand::SetLayerVisible(set_layer_visible::SetLayerVisible { layer_id: "l1".into(), visible: None }), "set-layer-visible set-layer-visible layer-id=l1", "010401026c3101000600"),
            (RasterCommand::SetHover(set_hover::SetHover { id: None }), "set-hover set-hover", "010c0000"),
        ];
        for (command, text, hex) in cases {
            assert_eq!(protocol::OpText::print_op(&command), text, "printed text drifted for {command:?}");
            let bytes = protocol::OpBinary::encode_op(&command).expect("encode");
            assert_eq!(bytes.iter().map(|b| format!("{b:02x}")).collect::<String>(), hex, "binary bytes drifted for {command:?}");
        }
    }

    #[test]
    fn command_ids_are_unique_across_every_row() {
        let mut seen = std::collections::HashSet::new();
        for command in every_command() {
            assert!(seen.insert(command.command_id().to_string()), "duplicate command_id {}", command.command_id());
        }
    }
}
//#endregion 🧪️Tests
