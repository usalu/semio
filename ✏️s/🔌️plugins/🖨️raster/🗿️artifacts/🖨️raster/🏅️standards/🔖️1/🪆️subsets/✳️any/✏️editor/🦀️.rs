//! 🖥️ Raster editor — ArtifactEditor impl, render, manifest (constitutional: ui/general). B1: `RasterPlayApp`
//! is a unit struct — every former `RasterConfig` (`ui`-crate `RefCell`) field (selection, hover, brush
//! size/opacity, navigator composite-viewport size, the session-only free camera) now lives in
//! `crate::editor::raster::config::RasterConfig`, written via `RasterConfigMutation`s. Every action
//! dispatches through the single typed `RasterCommand` channel via `app_commands!` — mirrors
//! `shooting_ui`'s B1 pilot.

use crate::artifacts::raster::op::RasterMutation;
use crate::artifacts::raster::{RasterLayerNode, RasterSnapshot, RASTER_DOCUMENT_SCHEMA};
use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::editor::raster::modes::edit;
use crate::editor::raster::modes::edit::windows::{composite, navigator};
use crate::editor::raster::presence::{RasterPresence, RasterPresenceMutation};
use crate::editor::raster::terminology::raster_play_labels;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDescriptor, ActionFactory, ActionKind, AppDefinition, ArtifactEditor, ArtifactKindSpec, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec,
    InteractionDefinition, InteractionRef, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, MergeMode, NoDraft, NoDraftMutation, OsMediaCapability, SelectionMethod, SelectionMode, SelectionSpec, UiNode,
    UtilityCategory, UtilityDefinition, WindowMeasure,
};
use dsl::os_pack::json::Value;
use std::collections::HashMap;
use store::ArtifactPack;
use store::EngineHandles;

//#region 🔖️Constants
pub const RASTER_PLAY_CONTROLLER_ID: &str = "raster-play";
/// 🌳️ Prefix for every layer-tree row id — shared by the document/masks panels and the `moveLayer`
/// command (which needs to decode a `target_row_id` back into a layer/group id). App-wide tree-encoding
/// concern, not artifact data, so it lives here rather than in any single panel.
pub const RASTER_TREE_PREFIX: &str = "raster-play-layers";
//#endregion 🔖️Constants

//#region 🔖️Document
/// 🌳️ Encodes a layer as its tree-row id — shared by the document/masks panels (which render rows) and
/// `moveLayer` (which decodes a drop target back into an id). More than one consumer, but this is UI row
/// encoding, not artifact data, so it stays app-level rather than in `crate::artifacts::raster::schema`.
pub fn layer_row_id(layer: &RasterLayerNode) -> String {
    let segment = match layer {
        RasterLayerNode::Group { .. } => "group",
        RasterLayerNode::Adjustment { .. } => "adjustment",
        RasterLayerNode::Pixel { .. } => "layer",
    };
    format!("{RASTER_TREE_PREFIX}.{segment}.{}", crate::artifacts::raster::schema::layer_node_id(layer))
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
    let value = dsl::os_pack::json::from_dsl_value(&dsl::ToValue::to_value(document));
    let value = match value {
        Value::Object(object) => {
            let filtered: dsl::os_pack::json::Object =
                object.iter().filter(|(key, _)| *key != "assets" && *key != "brushSize" && *key != "brushOpacity").map(|(key, value)| (key.to_string(), value.clone())).collect();
            Value::Object(filtered)
        }
        other => other,
    };
    value.to_string()
}

/// 🧩️ Resolves every asset handle on `document.assets` back to its real `RasterImageAsset` bytes
/// through the working-scene cache accessor (`crate::artifacts::raster::raster_asset`, ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` — `document.assets` now stores composed
/// `s.stdio.semio.image` CHILD handles, not embedded bytes) — the ONE call site the WASM compositor's
/// real pixel bytes funnel through. A handle whose content is not (or no longer) cached is honestly
/// omitted rather than serialized as an empty/garbage blob (documented staleness gap, matches every
/// other exemplar in this ticket).
fn assets_json_from_document(document: &RasterSnapshot) -> String {
    let resolved: std::collections::BTreeMap<String, crate::artifacts::raster::RasterImageAsset> =
        document.assets.keys().filter_map(|asset_id| crate::artifacts::raster::raster_asset(&document.assets, asset_id).map(|asset| (asset_id.clone(), asset))).collect();
    let object: dsl::os_pack::json::Object = resolved.into_iter().map(|(id, asset)| (id, dsl::os_pack::json::from_dsl_value(&dsl::ToValue::to_value(&asset)))).collect();
    dsl::os_pack::json::to_string(&Value::Object(object))
}

/// 🎞️ Builds the shared `Paint2dScene` payload for both the composite and navigator windows. Takes
/// `&RasterConfig` (an app-only view-state type), so per TEMPLATE.md §4's `DocumentHelpers` placement
/// rule this stays at app level even though it has two window consumers.
///
/// 🕹️ `selection_json`/`hovered_id` used to read `RasterConfig.selected_ids`/`.hovered_id` (deleted,
/// ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM)?; the `"layers"` domain's selection/
/// hover is framework-owned `InteractionState` now, and `ArtifactEditor::render` is not threaded an
/// `InteractionView` this wave (known SDK gap — see the ticket's `w3c-summary.md`) — left at neutral
/// defaults here. The real sync happens below `render`, at the paint surface/host layer
/// (`RasterHost::sync_interaction`, `🧰️framework/🔨️modules/🗺️surface/🎨️paint`), already migrated.
pub fn raster_scene(document: &RasterSnapshot, runtime: &RasterConfig, active_utility: &str, view_mode: &str) -> semio_framework_plugin::Paint2dScene {
    semio_framework_plugin::Paint2dScene {
        document_sync_json: document_sync_json(document),
        assets_json: assets_json_from_document(document),
        camera_json: dsl::os_pack::json::to_json_string(&runtime.camera),
        selection_json: "[]".into(),
        hovered_id: None,
        active_utility: active_utility.into(),
        brush_size: runtime.brush_size,
        brush_opacity: runtime.brush_opacity,
        view_mode: view_mode.into(),
        composite_viewport_json: runtime.composite_viewport.as_ref().map(|viewport| dsl::os_pack::json::to_json_string(viewport)),
    }
}

/// 🎬️ Builds an `ActionDescriptor` dispatched through the raster app's single controller — the one call
/// site every window/panel/option goes through.
pub fn raster_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    ActionFactory::new(RASTER_PLAY_CONTROLLER_ID).action(action, args)
}


/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref())
        .map(semio_framework_plugin::UiValue::Text)
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}

/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}

/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Number(value.into())
}


/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new()
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder
            .push(value)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new()
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder
            .push(key.to_owned(), value)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.
pub fn ui_node_list(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        let node = value?;
        nodes
            .try_push(node)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI node admission failed"))?;
    }
    Ok(nodes)
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
        "addLayer" as "add-layer" => add_layer::AddLayer,
        "dropLayerKind" as "drop-layer-kind" => drop_layer_kind::DropLayerKind,
        "setLayerVisible" as "set-layer-visible" => set_layer_visible::SetLayerVisible,
        "toggleLayerVisible" as "toggle-layer-visible" => toggle_layer_visible::ToggleLayerVisible,
        "deleteLayer" as "delete-layer" => delete_layer::DeleteLayer,
        "duplicateLayer" as "duplicate-layer" => duplicate_layer::DuplicateLayer,
        "patchLayer" as "patch-layer" => patch_layer::PatchLayer,
        "patchLayers" as "patch-layers" => patch_layers::PatchLayers,
        "moveLayer" as "move-layer" => move_layer::MoveLayer,
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
use crate::editor::raster::commands::set_active_utility;
use crate::editor::raster::commands::set_locale;
use crate::editor::raster::commands::{add_layer, delete_layer, drop_layer_kind, duplicate_layer, move_layer, patch_layer, patch_layers, set_layer_visible, toggle_layer_visible};
use crate::editor::raster::commands::{set_brush_opacity, set_brush_size};
use crate::editor::raster::commands::{set_camera, set_camera_zoom, set_composite_viewport};
//#endregion 🔖️Commands

//#region 🔖️RasterPlayApp
/// 🧪️ B1: unit struct — every former `RasterConfig` field now lives in
/// `crate::editor::raster::config::RasterConfig`, written through `RasterConfigMutation`s.
#[derive(Default)]
pub struct RasterPlayApp;

impl ArtifactEditor for RasterPlayApp {
    type Snapshot = RasterSnapshot;
    type Mutation = RasterMutation;
    type Config = RasterConfig;
    type ConfigMutation = RasterConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = RasterPresence;
    type PresenceMutation = RasterPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = RasterCommand;

    const DIALECT: Dialect = crate::artifacts::raster::RASTER_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = RASTER_DOCUMENT_SCHEMA;

    fn build_envelope_decode_owner_bundle() -> Option<store::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(crate::artifacts::raster::spr::raster_envelope_decode_owner_bundle())
    }

    fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::artifacts::raster::spr::raster_document_store_owners())
    }

    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(crate::artifacts::raster::spr::raster_document_store_initialization_job(envelope, operation, generation))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::raster::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> RasterSnapshot {
        crate::artifacts::raster::schema::empty_raster_document()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(raster_io())
    }

    /// 🎞️ `image:in`/`image:out` (see `crate::artifacts::raster::io::raster_image_layer_and_asset`,
    /// `raster_composite_media`) plus the inherited `document:out` default (the pack of
    /// `doc.snapshot`, replicated inline — overriding `export_media` shadows the trait's provided
    /// body for every port on this app, not just the new ones).
    fn export_media(port: &str, doc: &ArtifactView<'_, RasterSnapshot>) -> Result<Media, MediaError> {
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

    /// 🎞️ `image:in` inserts the incoming raster media as a new composited layer + embedded asset —
    /// two real semantic mutations (`add-layer-asset` then `create-layer`, in dependency order)
    /// bundled in one `Emit`, never a whole-document replace (`RasterMutation` has no such variant
    /// anymore). Falls through to the inherited `document:in` default (`MediaError::NotImplemented`,
    /// since `whole_document_operation` is no longer overridden) for any other port.
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, RasterSnapshot>) -> Result<Emit<RasterMutation, RasterConfigMutation, Self::DraftMutation>, MediaError> {
        if port != "image:in" {
            return Err(MediaError::NotImplemented);
        }
        let MediaPayload::Structured { json: png_base64, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "image:in only accepts a Structured (base64 PNG) payload".into()));
        };
        let (asset_id, asset, layer) = crate::artifacts::raster::io::raster_image_layer_and_asset(png_base64);
        Ok(Emit::mutations(vec![
            RasterMutation::AddLayerAsset(crate::artifacts::raster::mutations::add_layer_asset::mutation::AddLayerAsset { asset_id, asset }),
            RasterMutation::CreateLayer(crate::artifacts::raster::mutations::create_layer::mutation::CreateLayer { parent_id: None, index: doc.snapshot.layers.len(), layer: Box::new(layer) }),
        ]))
    }

    fn command_id(command: &RasterCommand) -> &'static str {
        command.command_id()
    }

    fn handle(
        command: &RasterCommand,
        doc: &ArtifactView<'_, RasterSnapshot>,
        cfg: &ConfigView<'_, RasterConfig>,
        _interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<RasterMutation, RasterConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn window_measures(_doc: &ArtifactView<'_, RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        HashMap::from([(composite::RASTER_PLAY_WINDOW_COMPOSITE.into(), composite::window_measures(cfg.snapshot))])
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let document = doc.snapshot;
        let config = cfg.snapshot;
        let labels = raster_play_labels(config);
        match body_key {
            composite::RASTER_PLAY_BODY_COMPOSITE => composite::render(document, config),
            navigator::RASTER_PLAY_BODY_NAVIGATOR => navigator::render(document, config),
            crate::editor::raster::panels::document::RASTER_PLAY_BODY_LAYERS => crate::editor::raster::panels::document::render(document, config, labels),
            crate::editor::raster::panels::masks::RASTER_PLAY_BODY_MASKS => crate::editor::raster::panels::masks::render(document, config, labels),
            crate::editor::raster::panels::catalogue::RASTER_PLAY_BODY_CATALOGUE => crate::editor::raster::panels::catalogue::render(labels),
            crate::editor::raster::panels::inspection::RASTER_PLAY_BODY_PROPERTIES => crate::editor::raster::panels::inspection::render(document, config, labels),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️RasterPlayApp

//#region 🔖️Io
/// 🔌️ Relocated verbatim from `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES,
/// rule 4: anything returning `AppIo` or referencing an app type lives in `🎛️apps/<app>/`). This app's
/// typed media I/O surface (`AppDefinition.io`) — mirrors the `2d.raster` `ArtifactKindSpec` literal
/// `crate::artifacts::raster::artifact_kind` already declares, plus the app-specific `image:in`/
/// `image:out` ports (see below).
pub fn raster_io() -> semio_framework::AppIo {
    semio_framework::AppIo {
        document_schema: RASTER_DOCUMENT_SCHEMA.into(),
        document_media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
        ports: vec![raster_image_in_port(), raster_image_out_port()],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework::ArtifactPresentation { id: "2d.raster".into(), name: "2D Raster".into(), dimension: "2d".into(), component_kind: "raster".into() },
    }
}

/// 🔌️ `image:in` — accepts raster imagery from upstream producers (e.g. draw's `vector:out`,
/// converted Vector→Raster) as a new composited layer. `Many`/optional: several upstream images may
/// feed in, and the port may sit unconnected.
pub fn raster_image_in_port() -> semio_framework::MediaPortSpec {
    semio_framework::MediaPortSpec {
        id: "image:in".into(),
        label: "Image".into(),
        direction: semio_framework::MediaPortDirection::In,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
        kind_id: None,
        required: false,
        multiplicity: semio_framework::PortMultiplicity::Many,
    }
}

/// 🔌️ `image:out` — the raster document's current composited raster, as `2d.image` media (workflow
/// port surface; WORKFLOWS-END-TO-END-TYPED-PORTS Wave 2 port recipe). `kind_id: "2d.image"` — the
/// shared framework-builtin interchange kind (declared on this app's `.artifact_kind(...)` below;
/// `shooting`'s `photos:out` declares the identical shape, harmless duplicate registrations).
pub fn raster_image_out_port() -> semio_framework::MediaPortSpec {
    semio_framework::MediaPortSpec {
        id: "image:out".into(),
        label: "Image".into(),
        direction: semio_framework::MediaPortDirection::Out,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
        kind_id: Some("2d.image".into()),
        required: false,
        multiplicity: semio_framework::PortMultiplicity::Many,
    }
}

/// 🖼️ Composites the current raster document to a PNG `Media` payload for the `image:out` port —
/// `crate::artifacts::raster::io::raster_document_json_to_svg` renders the document's real layer
/// stack (not a placeholder title card) via the `s.stdio.semio/v1/drawing` bridge; the vector→pixels
/// render step still has no stdio bridge (real pixel compositing is wgpu/canvas-host-side, out of
/// this pure headless compute node's reach — see that function's own doc), so its raw renderer
/// output is canonicalized through the real `s.stdio.semio/v1/image` ↔ png round trip inside
/// `🚪️io/🦀️.rs` before leaving this port.
pub fn raster_composite_media(document: &RasterSnapshot) -> Result<Media, MediaError> {
    let (svg, width, height) = crate::artifacts::raster::io::raster_document_json_to_svg(document).map_err(|error| MediaError::Payload("image:out".into(), error))?;
    let rendered = semio_framework_os::rasterize_svg_to_png_base64(&svg, width, height).map_err(|error| MediaError::Payload("image:out".into(), error))?;
    let raw_bytes = base64_codec::base64_standard_decode(rendered.as_bytes()).map_err(|error| MediaError::Payload("image:out".into(), error.to_string()))?;
    let canonical = crate::artifacts::raster::io::canonicalize_png_bytes(&raw_bytes).map_err(|error| MediaError::Payload("image:out".into(), error))?;
    let png_base64 = base64_codec::base64_standard_encode(canonical);
    Ok(Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster }, payload: MediaPayload::Structured { schema: "2d.image".into(), json: png_base64 } })
}
//#endregion 🔖️Io

//#region 🔖️Manifest
/// 🛠️ An internal (non-palette) action declaration — the panel/pointer/gesture-bound vocabulary
/// dispatched by the layer tree, catalogue drops and inspector, never a palette command.
fn raster_internal_action(id: &str, label: impl Into<LocalizedLabel>, kind: ActionKind) -> semio_framework_plugin::ActionDefinition {
    semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::bounded_catalog(id, label, kind) }
}

/// 🧰️ One composite-window utility declaration; ids must stay host-compatible (`paint*` prefix paints,
/// `paintEraser` erases, `selectMarquee` selects) because the scene's active utility feeds `RasterHost`.
fn raster_utility(id: &str, label: impl Into<LocalizedLabel>, icon: &str, group: &str, category: UtilityCategory) -> UtilityDefinition {
    UtilityDefinition { group: Some(group.into()), category: Some(category), ..UtilityDefinition::new(id, label, icon) }
}

/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding/utility declarations (which have no dedicated `_def` passthrough) are
/// written out inline.
///
/// 🚧️ SDK GAP (contract §2.4, `📓️w2-cad-report.md` "SDK gaps found" #4): `EditorBuilder`/
/// `.editor::<E>(def: AppDefinition)` take a bare `AppDefinition`, discarding `App.examples` — there
/// is no place left on this builder for the old `.example(...)`/`.workflow(...)` calls. Both are
/// dropped here, not silently ported; the subset's own `📚️examples/🎬️demo` facet is the likely
/// intended replacement mechanism (unconfirmed, per cad's same finding).
pub fn create_raster_app() -> AppDefinition {
    Editor::builder(crate::artifacts::raster::RASTER_DIALECT).document(["semio", "raster"])
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
                export_formats: vec![],
                import_formats: vec![],
                    export_stdio_kinds: vec!["stdio.png"],
        import_stdio_kinds: vec!["stdio.png"],
    })
            .icon_id("raster")
            .mode_def(edit::definition())
            .default_mode_id(edit::RASTER_PLAY_MODE_EDIT)
            .window_kind_def(composite::definition())
            .window_kind_def(navigator::definition())
            .default_layout(edit::layout())
            .panel_tab_def(crate::editor::raster::panels::document::definition())
            .panel_tab_def(crate::editor::raster::panels::catalogue::definition())
            .panel_tab_def(crate::editor::raster::panels::masks::definition())
            .panel_tab_def(crate::editor::raster::panels::inspection::definition())
            // ✏️ Palette-visible content operations. Whole-document replace (`setSnapshot`,
            // `setActiveExample`) is gone — file-open/load-example go through the `.example(...)`
            // registration at the bottom of this builder, entirely outside `RasterMutation` history.
            .mutation("addLayer", LocalizedLabel::native("Add Layer", "Ebene hinzufügen"))
            // 🔧️ Internal content operations — layer-tree / catalogue-drop / inspector bound.
            .action_with(raster_internal_action("setLayerVisible", LocalizedLabel::native("Set Layer Visible", "Ebenensichtbarkeit festlegen"), ActionKind::Mutation))
            .action_with(raster_internal_action("toggleLayerVisible", LocalizedLabel::native("Toggle Layer Visible", "Ebenensichtbarkeit umschalten"), ActionKind::Mutation))
            .action_with(raster_internal_action("dropLayerKind", LocalizedLabel::native("Drop Layer Kind", "Ebenenart ablegen"), ActionKind::Mutation))
            .action_with(raster_internal_action("deleteLayer", LocalizedLabel::native("Delete Layer", "Ebene löschen"), ActionKind::Mutation))
            .action_with(raster_internal_action("duplicateLayer", LocalizedLabel::native("Duplicate Layer", "Ebene duplizieren"), ActionKind::Mutation))
            .action_with(raster_internal_action("patchLayer", LocalizedLabel::native("Patch Layer", "Ebene aktualisieren"), ActionKind::Mutation))
            .action_with(raster_internal_action("patchLayers", LocalizedLabel::native("Patch Layers", "Ebenen aktualisieren"), ActionKind::Mutation))
            .action_with(raster_internal_action("moveLayer", LocalizedLabel::native("Move Layer", "Ebene verschieben"), ActionKind::Mutation))
            // 🕹️ The framework-owned "layers" interaction domain (ticket
            // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — the layer tree's selection;
            // auto-injects interactionSelect/interactionHover/clearSelection/selectAll/
            // setSelectionMode/setInteractionGranularity, replacing the deleted bespoke
            // setSelection/setHover/selectAll actions below.
            .interaction(InteractionDefinition {
                id: "layers".into(),
                label: LocalizedLabel::native("Layers", "Ebenen"),
                granularities: vec![GranularityDefinition { id: "layer".into(), label: LocalizedLabel::native("Layer", "Ebene"), icon_id: "image".into() }],
                hierarchy: HierarchyProvider::Flat,
                hover: HoverSpec::default(),
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick],
                    merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
                    transitive: false,
                    broadcast: true,
                },
            })
            .window_kind_interactions(composite::RASTER_PLAY_WINDOW_COMPOSITE, vec![InteractionRef::new("layers")])
            // 👁️ Ephemeral view state — live brush controls, navigator viewport, camera.
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
            // 🧰️ Composite-window utilities — one exclusive set, active utility host-owned (never a document operation).
            .utility(raster_utility("selectMarquee", LocalizedLabel::native("Marquee Select", "Rahmenauswahl"), "square-dashed", "Select", UtilityCategory::Selection))
            .utility(raster_utility("paintBrush", LocalizedLabel::native("Brush", "Pinsel"), "paintbrush", "Paint", UtilityCategory::Utilities))
            .utility(raster_utility("paintEraser", LocalizedLabel::native("Eraser", "Radiergummi"), "eraser", "Paint", UtilityCategory::Utilities))
            .window_kind_utilities(composite::RASTER_PLAY_WINDOW_COMPOSITE, vec![
                "selectMarquee".into(), "paintBrush".into(), "paintEraser".into(),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
pub(crate) mod testkit {
    //! 🧪️ Shared harness for every `editor::raster` node's tests — mirrors TEMPLATE.md §7.
    use super::*;
    use semio_framework_plugin::{testkit as framework_testkit, InvocationResult, VcsArtifactApp, ViewModel};

    pub type RasterApp = VcsArtifactApp<EditorApp<RasterPlayApp>>;

    use semio_framework_plugin::PluginApp;

    /// 🚧️ SDK GAP (`📓️w2-cad-report.md` "SDK gaps found" #3): `testkit::new_app_with_registry`'s
    /// signature is still `fn(manifest: fn() -> App)`, unchanged for this ticket; `create_raster_app`
    /// now returns `AppDefinition`. This tiny local wrapper adapts one to the other.
    fn raster_app_manifest_for_testkit() -> App {
        App { definition: create_raster_app(), examples: Vec::new() }
    }

    pub fn app() -> RasterApp {
        framework_testkit::new_app::<EditorApp<RasterPlayApp>>()
    }

    pub fn app_with_registry() -> RasterApp {
        framework_testkit::new_app_with_registry::<EditorApp<RasterPlayApp>>(raster_app_manifest_for_testkit)
    }

    pub fn dispatch(app: &mut RasterApp, command: RasterCommand) -> InvocationResult {
        app.dispatch_typed(command, &framework_testkit::meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut RasterApp, body_key: &str) -> String {
        dsl::os_pack::json::to_json_string(&app.render(body_key, None, &ViewModel::default()).expect("render"))
    }

    pub fn main_window_measures(app: &mut RasterApp) -> Vec<WindowMeasure> {
        app.window_measures().remove(composite::RASTER_PLAY_WINDOW_COMPOSITE).unwrap_or_default()
    }

    pub fn semio_app() -> RasterApp {
        let mut app = framework_testkit::new_app::<EditorApp<RasterPlayApp>>();
        let document = crate::artifacts::raster::schema::semio_example_document();
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
    use crate::artifacts::raster::schema::{empty_raster_document, layer_name, layer_visible};
    use crate::editor::raster::panels::{catalogue, document, inspection, masks};
    use semio_framework_plugin::{testkit, PluginApp, SET_ACTIVE_UTILITY_ACTION_ID};
    use store::MemoryBackbone;

    /// 🌱️ Relocated verbatim from `⚙️engine`'s own test module (rule 4: `raster_io`/`raster_composite_media`
    /// now live in this file's own `🔖️Io` region).
    #[semio_framework_async_macros::async_test]
    async fn raster_io_declares_image_in_and_image_out() {
        let io = raster_io();
        assert_eq!(io.document_schema, RASTER_DOCUMENT_SCHEMA);
        assert_eq!(io.artifact.id, "2d.raster");
        assert!(io.ports.iter().any(|p| p.id == "image:in"));
        let out_port = raster_image_out_port();
        assert_eq!(out_port.kind_id.as_deref(), Some("2d.image"));
    }

    #[semio_framework_async_macros::async_test]
    async fn raster_composite_media_exports_structured_2d_image_payload() {
        let document = crate::artifacts::raster::schema::empty_raster_document();
        let media = raster_composite_media(&document).expect("export image:out");
        let semio_framework::MediaPayload::Structured { schema, json } = media.payload else { panic!("expected structured payload") };
        assert_eq!(schema, "2d.image");
        assert!(!json.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn window_measures_expose_brush_and_eraser_option_groups() {
        let mut app = app();
        let measures = main_window_measures(&mut app);
        assert_eq!(measures.len(), 2);
        assert!(measures.iter().any(|m| matches!(m, WindowMeasure::Group { id, .. } if id == "raster-utility-options-paintBrush")));
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_raster_scene() {
        let mut app = app();
        let json = render(&mut app, composite::RASTER_PLAY_BODY_COMPOSITE);
        assert!(json.contains("raster"));
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_navigator_scene() {
        let mut app = app();
        let json = render(&mut app, navigator::RASTER_PLAY_BODY_NAVIGATOR);
        assert!(json.contains("\"componentKind\":\"paint-2d\""));
        assert!(json.contains("\"viewMode\":\"navigator\""));
    }

    #[semio_framework_async_macros::async_test]
    async fn parses_semio_example_document() {
        let document = crate::artifacts::raster::schema::semio_example_document();
        assert!(!document.layers.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_document_background_layer_has_identity_scale() {
        let document = empty_raster_document();
        let json = document_sync_json(&document);
        assert!(json.contains(r#""scaleX":1.0"#), "expected identity scale in {json}");
        assert!(json.contains(r#""scaleY":1.0"#), "expected identity scale in {json}");
        assert!(!json.contains(r#""scaleX":0.0"#), "layer must not collapse to zero size");
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_layers_tree() {
        let mut app = semio_app();
        let json = render(&mut app, document::RASTER_PLAY_BODY_LAYERS);
        assert!(json.contains("\"type\":\"tree\""));
        assert!(json.contains("Backdrop"));
    }

    #[semio_framework_async_macros::async_test]
    async fn raster_labels_resolve_native_english_by_default() {
        let mut app = app();
        let layers_json = render(&mut app, document::RASTER_PLAY_BODY_LAYERS);
        assert!(layers_json.contains("Add Pixel"));
        assert!(layers_json.contains("Add Group"));
        let masks_json = render(&mut app, masks::RASTER_PLAY_BODY_MASKS);
        assert!(masks_json.contains("Masks"));
        assert!(masks_json.contains("No masks"));
        let catalogue_json = render(&mut app, catalogue::RASTER_PLAY_BODY_CATALOGUE);
        assert!(catalogue_json.contains("Layer kinds"));
        assert!(catalogue_json.contains("raster-catalogue.pixel"));
        assert!(catalogue_json.contains("raster-catalogue.group"));
        assert!(catalogue_json.contains("raster-catalogue.adjustment"));
        let properties_json = render(&mut app, inspection::RASTER_PLAY_BODY_PROPERTIES);
        assert!(properties_json.contains("raster-play-inspector.schema"));
        assert!(properties_json.contains(RASTER_DOCUMENT_SCHEMA));
        assert!(properties_json.contains("raster-play-inspector.brush"));
    }

    #[semio_framework_async_macros::async_test]
    async fn raster_labels_resolve_german_locale() {
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

    #[semio_framework_async_macros::async_test]
    async fn composite_scene_syncs_document_and_assets() {
        let mut app = semio_app();
        let json = render(&mut app, composite::RASTER_PLAY_BODY_COMPOSITE);
        assert!(json.contains("\"componentKind\":\"paint-2d\""));
        assert!(json.contains("\"viewMode\":\"composite\""));
        assert!(!json.contains("\"assetsJson\":\"{}\""), "semio fixture has embedded assets");
        let document = crate::artifacts::raster::schema::semio_example_document();
        let sync_json = document_sync_json(&document);
        assert!(!sync_json.contains("\"assets\""), "sync json must omit assets");
        assert!(sync_json.contains("\"params\""), "adjustment params must survive document→sync roundtrip for the paint host");
        let sync_value: Value = dsl::os_pack::json::parse(&sync_json).expect("sync json");
        let layers = sync_value.get("layers").and_then(Value::as_array).expect("layers");
        assert!(layers.iter().any(|layer| layer.get("kind").and_then(Value::as_str) == Some("adjustment") && layer.get("params").is_some()));
        assert!(document.assets.contains_key("semio-emblem"));
    }

    #[semio_framework_async_macros::async_test]
    async fn semio_example_preserves_adjustment_params() {
        let document = crate::artifacts::raster::schema::semio_fixture_snapshot();
        let RasterLayerNode::Adjustment { params, adjustment_kind, .. } = document.layers.iter().find(|layer| matches!(layer, RasterLayerNode::Adjustment { id, .. } if id == "brighten")).expect("brighten adjustment") else {
            panic!("expected adjustment");
        };
        assert_eq!(adjustment_kind, "brightnessContrast");
        assert!(params.contains_key("brightness"), "fixture brightness must roundtrip");
        assert!(params.contains_key("contrast"), "fixture contrast must roundtrip");
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: layer hover/selection dispatch
    /// through the framework-injected `interactionHover`/`interactionSelect` verbs against the
    /// `"layers"` domain now (`semio-framework-plugin`'s own suite covers that generic machinery);
    /// this app's contribution is declaring the domain and binding the tree to it.
    #[semio_framework_async_macros::async_test]
    async fn document_tree_binds_the_layers_interaction_domain() {
        let mut app = semio_app();
        let json = render(&mut app, document::RASTER_PLAY_BODY_LAYERS);
        assert!(json.contains("\"interactionDomain\":\"layers\""), "layer tree must bind the framework-owned layers domain: {json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn set_composite_viewport_feeds_navigator_scene() {
        let mut app = app();
        dispatch(&mut app, RasterCommand::SetCompositeViewport(set_composite_viewport::SetCompositeViewport { width: 640.0, height: 480.0 }));
        let json = render(&mut app, navigator::RASTER_PLAY_BODY_NAVIGATOR);
        assert!(json.contains("compositeViewportJson"));
        assert!(json.contains(r#"\"width\":640.0"#));
        assert!(json.contains(r#"\"height\":480.0"#));
    }

    #[semio_framework_async_macros::async_test]
    async fn set_camera_mutates_runtime_and_emits_no_operations() {
        let mut app = app();
        let before = app.snapshot().expect("snapshot");
        let result = dispatch(&mut app, RasterCommand::SetCamera(set_camera::SetCamera { camera: crate::artifacts::raster::RasterCamera { x: 4.0, y: 5.0, zoom: 2.0 } }));
        assert!(result.mutations.is_empty(), "camera is a view action and emits no operations");
        assert_eq!(app.snapshot().expect("snapshot"), before, "camera never mutates the document");
        let json = render(&mut app, composite::RASTER_PLAY_BODY_COMPOSITE);
        assert!(json.contains(r#"\"zoom\":2.0"#), "composite scene camera reflects runtime state: {json}");
        assert!(json.contains(r#"\"x\":4.0"#), "composite scene camera reflects runtime state: {json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn set_camera_zoom_updates_zoom_and_keeps_pan_via_runtime() {
        let mut app = app();
        dispatch(&mut app, RasterCommand::SetCamera(set_camera::SetCamera { camera: crate::artifacts::raster::RasterCamera { x: 4.0, y: 5.0, zoom: 1.0 } }));
        let result = dispatch(&mut app, RasterCommand::SetCameraZoom(set_camera_zoom::SetCameraZoom { zoom: 3.0 }));
        assert!(result.mutations.is_empty(), "camera zoom is a view action and emits no operations");
        let json = render(&mut app, composite::RASTER_PLAY_BODY_COMPOSITE);
        assert!(json.contains(r#"\"zoom\":3.0"#), "zoom updated: {json}");
        assert!(json.contains(r#"\"x\":4.0"#), "pan preserved across zoom-only update: {json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn add_layer_action_appends_and_undo_removes() {
        let mut app = app();
        let before = app.snapshot().expect("snapshot").layers.len();
        dispatch(&mut app, RasterCommand::AddLayer(add_layer::AddLayer { kind: "group".into() }));
        let projection = app.snapshot().expect("snapshot");
        assert_eq!(projection.layers.len(), before + 1);
        assert!(matches!(projection.layers.last().unwrap(), RasterLayerNode::Group { .. }));
        app.handle_action("undo", None, &testkit::meta("local")).expect("undo");
        assert_eq!(app.snapshot().expect("snapshot").layers.len(), before);
    }

    #[semio_framework_async_macros::async_test]
    async fn patch_layer_renames_and_toggles_visibility_round_trip() {
        let mut app = app();
        let layer_id = crate::artifacts::raster::schema::layer_node_id(&app.snapshot().expect("snapshot").layers[0]).to_string();
        dispatch(&mut app, RasterCommand::PatchLayer(patch_layer::PatchLayer { layer_id: layer_id.clone(), field: "name".into(), value: "Renamed".into() }));
        assert_eq!(layer_name(&app.snapshot().expect("snapshot").layers[0]), "Renamed");
        dispatch(&mut app, RasterCommand::ToggleLayerVisible(toggle_layer_visible::ToggleLayerVisible { layer_id }));
        assert!(!layer_visible(&app.snapshot().expect("snapshot").layers[0]));
        app.handle_action("undo", None, &testkit::meta("local")).expect("undo toggle");
        assert!(layer_visible(&app.snapshot().expect("snapshot").layers[0]));
    }

    #[semio_framework_async_macros::async_test]
    async fn move_layer_into_group() {
        let mut app = app();
        dispatch(&mut app, RasterCommand::AddLayer(add_layer::AddLayer { kind: "group".into() }));
        let (group_id, pixel_id) = {
            let projection = app.snapshot().expect("snapshot");
            let group = projection.layers.iter().find(|layer| matches!(layer, RasterLayerNode::Group { .. })).unwrap();
            let pixel = projection.layers.iter().find(|layer| matches!(layer, RasterLayerNode::Pixel { .. })).unwrap();
            (crate::artifacts::raster::schema::layer_node_id(group).to_string(), crate::artifacts::raster::schema::layer_node_id(pixel).to_string())
        };
        let target_row = format!("{RASTER_TREE_PREFIX}.group.{group_id}");
        dispatch(&mut app, RasterCommand::MoveLayer(move_layer::MoveLayer { layer_id: pixel_id.clone(), target_row_id: target_row, drop_position: "after".into() }));
        let projection = app.snapshot().expect("snapshot");
        let RasterLayerNode::Group { children, .. } = projection.layers.iter().find(|layer| crate::artifacts::raster::schema::layer_node_id(layer) == group_id).unwrap() else {
            panic!("expected group");
        };
        assert_eq!(children.len(), 1);
        assert_eq!(crate::artifacts::raster::schema::layer_node_id(&children[0]), pixel_id);
    }

    /// 🧪️ The definitional merge proof: A adds a layer while B renames the background layer — disjoint
    /// tree edits on one backbone that must both survive on both instances.
    #[semio_framework_async_macros::async_test]
    async fn two_instances_converge_disjoint_layer_edits_via_backbone() {
        let mut instance_a = app();
        let mut instance_b = app();
        // Seed both from an identical base projection (a background layer with a fixed id) so B's
        // rename targets the same layer A holds — per-instance `initial_snapshot` mints fresh ids.
        let mut base = crate::artifacts::raster::schema::empty_raster_snapshot();
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

    #[semio_framework_async_macros::async_test]
    async fn ingest_operations_is_idempotent() {
        testkit::assert_ingest_idempotent::<RasterPlayApp, usize>(RasterCommand::AddLayer(add_layer::AddLayer { kind: "pixel".into() }), |app| app.snapshot().unwrap().layers.len());
    }

    #[semio_framework_async_macros::async_test]
    async fn set_active_utility_switch_emits_no_ops_and_persists_in_config() {
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

    #[semio_framework_async_macros::async_test]
    async fn utility_registry_declares_utilities_scoped_to_the_composite_window() {
        let definition = create_raster_app();
        let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
        assert_eq!(utility_ids, ["selectMarquee", "paintBrush", "paintEraser"]);
        // The marquee carries the Selection category; the paint utilities are Tools.
        let selects: Vec<&str> = definition.utilities.iter().filter(|utility| utility.category == Some(UtilityCategory::Selection)).map(|utility| utility.id.as_str()).collect();
        assert_eq!(selects, ["selectMarquee"]);
        let composite = definition.window_kinds.iter().find(|window| window.id == composite::RASTER_PLAY_WINDOW_COMPOSITE).expect("composite window");
        assert_eq!(composite.utilities.len(), definition.utilities.len(), "every utility is scoped to the composite window kind");
        // The framework auto-injects the setActiveUtility View action once utilities are declared; no doc operation survives.
        assert!(composite.actions.iter().any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID && matches!(action.kind, ActionKind::View)));
        assert!(!definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == "setActiveUtility" && !matches!(action.kind, ActionKind::View)));
    }

    #[semio_framework_async_macros::async_test]
    async fn raster_io_declares_image_in_out_and_export_media_covers_all_ports() {
        let projection = empty_raster_document();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&projection, &history);
        let app = RasterPlayApp;
        let image_out = semio_framework_plugin::resolve_ready(RasterPlayApp::export_media("image:out", &doc)).expect("image:out");
        let MediaPayload::Structured { schema, json } = image_out.payload else { panic!("expected structured payload") };
        assert_eq!(schema, "2d.image");
        assert!(!json.is_empty());
        assert!(semio_framework_plugin::resolve_ready(RasterPlayApp::export_media("document:out", &doc)).is_ok());
        assert!(matches!(semio_framework_plugin::resolve_ready(RasterPlayApp::export_media("unknown:out", &doc)), Err(MediaError::NotImplemented)));
    }

    #[semio_framework_async_macros::async_test]
    async fn raster_import_media_appends_layer_from_incoming_image() {
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
            RasterCommand::AddLayer(add_layer::AddLayer { kind: "pixel".into() }),
            RasterCommand::DropLayerKind(drop_layer_kind::DropLayerKind { kind: "group".into() }),
            RasterCommand::SetLayerVisible(set_layer_visible::SetLayerVisible { layer_id: "l1".into(), visible: Some(true) }),
            RasterCommand::ToggleLayerVisible(toggle_layer_visible::ToggleLayerVisible { layer_id: "l1".into() }),
            RasterCommand::DeleteLayer(delete_layer::DeleteLayer { layer_id: "l1".into() }),
            RasterCommand::DuplicateLayer(duplicate_layer::DuplicateLayer { layer_id: "l1".into() }),
            RasterCommand::PatchLayer(patch_layer::PatchLayer { layer_id: "l1".into(), field: "opacity".into(), value: "0.4".into() }),
            RasterCommand::PatchLayers(patch_layers::PatchLayers { layer_ids: vec!["a".into(), "b".into()], field: "name".into(), value: "Renamed".into() }),
            RasterCommand::MoveLayer(move_layer::MoveLayer { layer_id: "l1".into(), target_row_id: "raster-play-layers".into(), drop_position: "after".into() }),
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
    #[semio_framework_async_macros::async_test]
    async fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// 🎫️ Every `app_commands!` row's wire keyword must be distinct — the cross-cutting invariant the
    /// macro exists to hold.
    #[semio_framework_async_macros::async_test]
    async fn command_wire_keywords_are_unique_across_every_row() {
        let commands = every_command();
        assert_eq!(commands.len(), 16, "every RasterCommand row must be covered by every_command()");
        let mut keywords: Vec<String> = commands.iter().map(|command| protocol::OpText::print_op(command).split(' ').next().unwrap_or_default().to_string()).collect();
        keywords.sort();
        keywords.dedup();
        assert_eq!(keywords.len(), commands.len(), "every row's wire keyword must be distinct");
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — what a
    /// missing `#[dsl(keyword = ..)]` on a payload struct silently breaks (the record prints with no
    /// keyword at all and no longer parses).
    #[semio_framework_async_macros::async_test]
    async fn every_printed_op_line_starts_with_the_rows_declared_wire_keyword() {
        let expectations: Vec<(&str, RasterCommand)> = every_command()
            .into_iter()
            .map(|command| {
                let keyword: &'static str = match &command {
                    RasterCommand::AddLayer(_) => "add-layer",
                    RasterCommand::DropLayerKind(_) => "drop-layer-kind",
                    RasterCommand::SetLayerVisible(_) => "set-layer-visible",
                    RasterCommand::ToggleLayerVisible(_) => "toggle-layer-visible",
                    RasterCommand::DeleteLayer(_) => "delete-layer",
                    RasterCommand::DuplicateLayer(_) => "duplicate-layer",
                    RasterCommand::PatchLayer(_) => "patch-layer",
                    RasterCommand::PatchLayers(_) => "patch-layers",
                    RasterCommand::MoveLayer(_) => "move-layer",
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

    /// ⚖️ The rows whose `Option` fields make `None`/`Some` distinct wire cases, pinned to exact
    /// bytes so an ACCIDENTAL row reorder is caught. Baseline rebased once, deliberately, by the
    /// `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL` ticket: dropping the two leading `setSnapshot`/
    /// `setActiveExample` rows (whole-document replace is no longer expressible as a mutation)
    /// shifted every later row's binary ordinal down by two — `set-layer-visible` 4→2 (`0104`→`0102`).
    /// Rebased again by `26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM`: the `set-selection`/
    /// `set-hover`/`select-all` rows (the only other `Option`-carrying case, `set-hover`) are deleted
    /// outright — layer selection/hover is the framework-owned `"layers"` interaction domain now.
    /// `set-layer-visible`'s ordinal is unaffected (it sits before the deleted rows). Greenfield repo,
    /// no persisted wire data to migrate. Any FURTHER drift here is a real format break, not a
    /// fixture mismatch.
    #[semio_framework_async_macros::async_test]
    async fn optional_field_rows_keep_their_declared_wire_bytes() {
        let cases: [(RasterCommand, &str, &str); 1] = [(RasterCommand::SetLayerVisible(set_layer_visible::SetLayerVisible { layer_id: "l1".into(), visible: None }), "set-layer-visible set-layer-visible layer-id=l1", "010201026c3101000600")];
        for (command, text, hex) in cases {
            assert_eq!(protocol::OpText::print_op(&command), text, "printed text drifted for {command:?}");
            let bytes = protocol::OpBinary::encode_op(&command).expect("encode");
            assert_eq!(bytes.iter().map(|b| format!("{b:02x}")).collect::<String>(), hex, "binary bytes drifted for {command:?}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn command_ids_are_unique_across_every_row() {
        let mut seen = std::collections::HashSet::new();
        for command in every_command() {
            assert!(seen.insert(command.command_id().to_string()), "duplicate command_id {}", command.command_id());
        }
    }
}
//#endregion 🧪️Tests
