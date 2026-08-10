//! 🏙️ Block 3D play app — the `DocumentApp` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the world window
//! (+ its `🎚️options/*`) in `🎭️modes/✏️edit/🪟️windows/🌐️world`, panel trees in `📌️panels/*`, labels in
//! `🦀️terminology.rs`, view state in `🦀️config.rs`, world-scene compute needing both document+config in
//! `🦀️world.rs`, and pure document-side compute in `crate::artifacts::block3d::engine`.

use crate::apps::block3d::commands::brush::{hover_surface, leave_surface, place_vortex, set_brush_flip, set_brush_radius, set_brush_vortex_kind};
use crate::apps::block3d::commands::camera::set_camera;
use crate::apps::block3d::commands::example::{edit, set_active_example};
use crate::apps::block3d::commands::kind::patch_object_kind;
use crate::apps::block3d::commands::representation::{add_representation, patch_representation, remove_representation};
use crate::apps::block3d::commands::selection::{hover_vortex, select_vortex, set_selection};
use crate::apps::block3d::commands::vortex::{add_vortex, remove_vortex};
use crate::apps::block3d::commands::vortex_kind::{add_vortex_kind, remove_vortex_kind};
use crate::apps::block3d::commands::window::{set_active_representation, set_active_utility, set_window_arrangement, set_window_representations, set_window_spacing, toggle_window_representation};
use crate::apps::block3d::config::{Block3dConfig, Block3dConfigMutation};
use crate::apps::block3d::modes::edit as edit_mode;
use crate::apps::block3d::modes::edit::windows::world;
use crate::apps::block3d::panels::{document as document_panel, inspection as inspection_panel};
use crate::apps::block3d::terminology::block3d_labels;
use crate::artifacts::block3d::op::Block3dMutation;
use crate::artifacts::block3d::{artifact_kind, Block3dSnapshot, BLOCK_3D_SCHEMA};
use crate::BlockCamera3d;
use semio_framework_plugin::{NoDraft, NoDraftMutation, DraftView, 
    ActionDescriptor, App, ArtifactKindSpec, ConfigView, DocumentApp, DocumentView, Emit, Fault, FaultCode, FaultOrigin, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType,
    UiNode, UtilityDefinition,
};
use store::EngineHandles;
use serde_json::Value;
use std::collections::HashMap;

//#region 🔖️Constants
pub const BLOCK3D_PLAY_APP_ID: &str = "block3d-play";
pub const BLOCK3D_PLAY_SURFACE_ID: &str = "block3d.play.world";
pub const BLOCK3D_DEFAULT_WINDOW_ID: &str = "block3d-world";
pub const BLOCK3D_WORLD_OBJECT_ID: &str = "block3d-object";
pub const BLOCK3D_UTILITY_SELECT: &str = "select";
pub const BLOCK3D_UTILITY_SURFACE_BRUSH: &str = "surfaceBrush";
/// 🗂️ The `s/plugin/puzzle` 3d catalog artifact kind block3d's `"catalog:out"` port produces — see
/// `crate::artifacts::block3d::engine::block3d_io` and `Block3dPlayApp::export_media`.
const KIT_CATALOG_ARTIFACT_ID: &str = "kit.catalog";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`, `🎚️options/*`, `🎮️commands/*`) builds its `on_change`/item actions with.
pub fn block3d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(BLOCK3D_PLAY_APP_ID).action(action, args)
}

fn block3d_resolve_world_body(body_key: &str) -> (&str, String) {
    if body_key == world::BLOCK3D_BODY_WORLD || body_key.starts_with(&format!("{}:", world::BLOCK3D_BODY_WORLD)) {
        if let Some((_, window_id)) = body_key.split_once(':') {
            return (world::BLOCK3D_BODY_WORLD, window_id.to_string());
        }
        return (world::BLOCK3D_BODY_WORLD, BLOCK3D_DEFAULT_WINDOW_ID.into());
    }
    (body_key, BLOCK3D_DEFAULT_WINDOW_ID.into())
}

fn f64_vec3_field(args: Option<&Value>, key: &str) -> Option<[f64; 3]> {
    let array = args.and_then(|value| value.get(key))?.as_array()?;
    if array.len() < 3 {
        return None;
    }
    Some([array[0].as_f64()?, array[1].as_f64()?, array[2].as_f64()?])
}

fn window_id_from_args(args: Option<&Value>) -> String {
    args.and_then(|value| value.get("windowId").or_else(|| value.get("pane")).or_else(|| value.get("surfaceId")))
        .and_then(Value::as_str)
        .map_or_else(|| BLOCK3D_DEFAULT_WINDOW_ID.into(), str::to_string)
}
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Block3dPlayApp::Command` — the SOLE dispatch surface for block3d's own behavior, covering
    /// every action `create_block3d_app` declares. Row order is the binary variant ordinal: appending
    /// is safe, reordering is a wire-format break. Every id/key pair is IDENTICAL EXCEPT the three
    /// `worldSurface*` rows (`HoverSurface`/`LeaveSurface`/`PlaceVortex`), whose manifest action id and
    /// `#[dsl(key)]` wire keyword genuinely diverge pre-migration — preserved verbatim.
    pub enum Block3dCommand for Block3dSnapshot, Block3dMutation, Block3dConfig, Block3dConfigMutation {
        "patchObjectKind" as "patchObjectKind" => patch_object_kind::PatchObjectKind,
        "addRepresentation" as "addRepresentation" => add_representation::AddRepresentation,
        "removeRepresentation" as "removeRepresentation" => remove_representation::RemoveRepresentation,
        "addVortexKind" as "addVortexKind" => add_vortex_kind::AddVortexKind,
        "removeVortexKind" as "removeVortexKind" => remove_vortex_kind::RemoveVortexKind,
        "addVortex" as "addVortex" => add_vortex::AddVortex,
        "removeVortex" as "removeVortex" => remove_vortex::RemoveVortex,
        "setActiveExample" as "setActiveExample" => set_active_example::SetActiveExample,
        "edit" as "edit" => edit::Edit,
        "setSelection" as "setSelection" => set_selection::SetSelection,
        "setActiveRepresentation" as "setActiveRepresentation" => set_active_representation::SetActiveRepresentation,
        "setWindowRepresentations" as "setWindowRepresentations" => set_window_representations::SetWindowRepresentations,
        "toggleWindowRepresentation" as "toggleWindowRepresentation" => toggle_window_representation::ToggleWindowRepresentation,
        "setWindowArrangement" as "setWindowArrangement" => set_window_arrangement::SetWindowArrangement,
        "setWindowSpacing" as "setWindowSpacing" => set_window_spacing::SetWindowSpacing,
        "setActiveUtility" as "setActiveUtility" => set_active_utility::SetActiveUtility,
        "setBrushVortexKind" as "setBrushVortexKind" => set_brush_vortex_kind::SetBrushVortexKind,
        "setBrushRadius" as "setBrushRadius" => set_brush_radius::SetBrushRadius,
        "setBrushFlip" as "setBrushFlip" => set_brush_flip::SetBrushFlip,
        "worldSurfaceHover" as "hoverSurface" => hover_surface::HoverSurface,
        "worldSurfaceLeave" as "leaveSurface" => leave_surface::LeaveSurface,
        "worldSurfacePlace" as "placeVortex" => place_vortex::PlaceVortex,
        "setCamera" as "setCamera" => set_camera::SetCamera,
        "selectVortex" as "selectVortex" => select_vortex::SelectVortex,
        "hoverVortex" as "hoverVortex" => hover_vortex::HoverVortex,
        "patchRepresentation" as "patchRepresentation" => patch_representation::PatchRepresentation,
    }
}
//#endregion 🔖️Commands

//#region 🔖️Block3dPlayApp
/// 🧪️ B1: unit struct — every former `RefCell` field now lives in `crate::apps::block3d::config::
/// Block3dConfig`, written through `Block3dConfigMutation`s.
#[derive(Default)]
pub struct Block3dPlayApp;

impl DocumentApp for Block3dPlayApp {
    type Snapshot = Block3dSnapshot;
    type Mutation = Block3dMutation;
    type Config = Block3dConfig;
    type ConfigMutation = Block3dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::apps::block3d::presence::Block3dPresence;
    type PresenceMutation = crate::apps::block3d::presence::Block3dPresenceMutation;

    type Command = Block3dCommand;

    const APP_ID: &'static str = BLOCK3D_PLAY_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = BLOCK_3D_SCHEMA;

    fn initial_snapshot() -> Block3dSnapshot {
        crate::artifacts::block3d::engine::empty_block3d_snapshot()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(crate::artifacts::block3d::engine::block3d_io())
    }

    fn command_id(command: &Block3dCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + JSON args onto `Block3dCommand` — React/wgpu still speak the stringly
    /// `{action,args}` wire; this is the typed-command bridge until those call sites send `OpBinary`
    /// bytes directly.
    fn command_from_action(action: &str, args: Option<&Value>) -> Result<Self::Command, Fault> {
        let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_str).map(str::to_string);
        let str_vec_field = |key: &str| -> Vec<String> {
            args.and_then(|value| value.get(key))
                .and_then(|value| value.as_array())
                .map(|rows| rows.iter().filter_map(|row| row.as_str().map(str::to_string)).collect())
                .unwrap_or_default()
        };
        match action {
            "patchObjectKind" => Ok(Block3dCommand::PatchObjectKind(patch_object_kind::PatchObjectKind { field: str_field("field").unwrap_or_default(), value: str_field("value").unwrap_or_default() })),
            "addRepresentation" => Ok(Block3dCommand::AddRepresentation(add_representation::AddRepresentation {})),
            "removeRepresentation" => Ok(Block3dCommand::RemoveRepresentation(remove_representation::RemoveRepresentation { id: str_field("id").unwrap_or_default() })),
            "addVortexKind" => Ok(Block3dCommand::AddVortexKind(add_vortex_kind::AddVortexKind {})),
            "removeVortexKind" => Ok(Block3dCommand::RemoveVortexKind(remove_vortex_kind::RemoveVortexKind { id: str_field("id").unwrap_or_default() })),
            "addVortex" => Ok(Block3dCommand::AddVortex(add_vortex::AddVortex {})),
            "removeVortex" => Ok(Block3dCommand::RemoveVortex(remove_vortex::RemoveVortex { id: str_field("id").unwrap_or_default() })),
            "setActiveExample" => Ok(Block3dCommand::SetActiveExample(set_active_example::SetActiveExample { id: str_field("exampleId").or_else(|| str_field("id")).unwrap_or_default() })),
            "edit" => Ok(Block3dCommand::Edit(edit::Edit { text: str_field("text").unwrap_or_default() })),
            "setSelection" => Ok(Block3dCommand::SetSelection(set_selection::SetSelection { ids: str_vec_field("ids") })),
            "setActiveRepresentation" => Ok(Block3dCommand::SetActiveRepresentation(set_active_representation::SetActiveRepresentation { representation_id: str_field("representationId").or_else(|| str_field("representation_id")) })),
            "setWindowRepresentations" => {
                let rep = str_field("value").or_else(|| str_field("representationId"));
                let representation_ids = rep.filter(|id| !id.is_empty()).map(|id| vec![id]).unwrap_or_default();
                Ok(Block3dCommand::SetWindowRepresentations(set_window_representations::SetWindowRepresentations { window_id: window_id_from_args(args), representation_ids }))
            }
            "toggleWindowRepresentation" => Ok(Block3dCommand::ToggleWindowRepresentation(toggle_window_representation::ToggleWindowRepresentation {
                window_id: window_id_from_args(args),
                representation_id: str_field("representationId").unwrap_or_default(),
                visible: args.and_then(|value| value.get("visible")).and_then(Value::as_bool).unwrap_or(true),
            })),
            "setWindowArrangement" => Ok(Block3dCommand::SetWindowArrangement(set_window_arrangement::SetWindowArrangement { window_id: window_id_from_args(args), arrangement: str_field("value").unwrap_or_else(|| "overlap".into()) })),
            "setWindowSpacing" => Ok(Block3dCommand::SetWindowSpacing(set_window_spacing::SetWindowSpacing {
                window_id: window_id_from_args(args),
                spacing: args.and_then(|value| value.get("value")).and_then(Value::as_f64).unwrap_or(8.0),
            })),
            "setActiveUtility" => Ok(Block3dCommand::SetActiveUtility(set_active_utility::SetActiveUtility {
                window_id: window_id_from_args(args),
                utility_id: str_field("utilityId").unwrap_or_else(|| BLOCK3D_UTILITY_SELECT.into()),
            })),
            "setBrushVortexKind" => Ok(Block3dCommand::SetBrushVortexKind(set_brush_vortex_kind::SetBrushVortexKind { vortex_kind_id: str_field("value").or_else(|| str_field("vortexKindId")) })),
            "setBrushRadius" => Ok(Block3dCommand::SetBrushRadius(set_brush_radius::SetBrushRadius { radius: args.and_then(|value| value.get("value")).and_then(Value::as_f64).unwrap_or(0.3) })),
            "setBrushFlip" => Ok(Block3dCommand::SetBrushFlip(set_brush_flip::SetBrushFlip { flip: args.and_then(|value| value.get("flip")).and_then(Value::as_bool).unwrap_or(false) })),
            "worldSurfaceHover" => Ok(Block3dCommand::HoverSurface(hover_surface::HoverSurface {
                window_id: window_id_from_args(args),
                object_id: str_field("objectId").unwrap_or_default(),
                position: f64_vec3_field(args, "position").unwrap_or([0.0, 0.0, 0.0]),
                normal: f64_vec3_field(args, "normal").unwrap_or([0.0, 0.0, 1.0]),
            })),
            "worldSurfaceLeave" => Ok(Block3dCommand::LeaveSurface(leave_surface::LeaveSurface {})),
            "worldSurfacePlace" => Ok(Block3dCommand::PlaceVortex(place_vortex::PlaceVortex {
                window_id: window_id_from_args(args),
                object_id: str_field("objectId").unwrap_or_default(),
                position: f64_vec3_field(args, "position").unwrap_or([0.0, 0.0, 0.0]),
                normal: f64_vec3_field(args, "normal").unwrap_or([0.0, 0.0, 1.0]),
            })),
            "selectVortex" => Ok(Block3dCommand::SelectVortex(select_vortex::SelectVortex { full_id: str_field("fullId").unwrap_or_default(), merge: args.and_then(|value| value.get("merge")).and_then(Value::as_bool).unwrap_or(false) })),
            "hoverVortex" => Ok(Block3dCommand::HoverVortex(hover_vortex::HoverVortex { full_id: str_field("fullId") })),
            "patchRepresentation" => Ok(Block3dCommand::PatchRepresentation(patch_representation::PatchRepresentation { id: str_field("id").unwrap_or_default(), field: str_field("field").unwrap_or_default(), value: str_field("value").unwrap_or_default() })),
            // 🩹️ Forward-fix, not a preserved behavior: pre-migration `command_from_action` had NO arm
            // for the manifest-declared `setCamera` view action at all (fell through to the reserved-
            // action error) — a real gap, the `Block3dCommand::SetCamera` variant was only reachable via
            // direct `dispatch_typed`/binary `OpBinary`. `assert_declared_actions_bridge_to_commands`
            // (added by this migration) requires every declared action to bridge, so this parses the
            // camera pose from `{position,target,zoom}` args the same shape `BlockCamera3d` serializes to.
            "setCamera" => Ok(Block3dCommand::SetCamera(set_camera::SetCamera {
                camera: BlockCamera3d {
                    position: f64_vec3_field(args, "position").unwrap_or([0.0, 0.0, 0.0]),
                    target: f64_vec3_field(args, "target").unwrap_or([0.0, 0.0, 0.0]),
                    zoom: args.and_then(|value| value.get("zoom")).and_then(Value::as_f64).unwrap_or(1.0),
                },
            })),
            other => Err(Fault::new(
                FaultOrigin::App,
                FaultCode::new("block3d.unhandled-action"),
                format!("action '{other}' is not a framework-reserved action — app actions are dispatched exclusively through the typed command channel"),
            )),
        }
    }

    fn handle(command: &Block3dCommand, doc: &DocumentView<'_, Block3dSnapshot>, cfg: &ConfigView<'_, Block3dConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<Block3dMutation, Block3dConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn window_measures(doc: &DocumentView<'_, Block3dSnapshot>, cfg: &ConfigView<'_, Block3dConfig>) -> HashMap<String, Vec<semio_framework_plugin::WindowMeasure>> {
        let labels = block3d_labels(cfg.snapshot);
        let mut measures = HashMap::new();
        measures.insert(BLOCK3D_DEFAULT_WINDOW_ID.into(), world::window_measures(doc.snapshot, cfg.snapshot, BLOCK3D_DEFAULT_WINDOW_ID, labels));
        measures
    }

    fn render(body_key: &str, doc: &DocumentView<'_, Block3dSnapshot>, cfg: &ConfigView<'_, Block3dConfig>) -> UiNode {
        let labels = block3d_labels(cfg.snapshot);
        let active_representation_id = cfg.snapshot.active_representation_id.as_deref();
        let (base_body, window_id) = block3d_resolve_world_body(body_key);
        match base_body {
            world::BLOCK3D_BODY_WORLD => world::render(doc.snapshot, cfg.snapshot, &window_id),
            document_panel::BLOCK3D_BODY_DOCUMENT => document_panel::render(doc.snapshot, &cfg.snapshot.selected_ids, labels),
            inspection_panel::BLOCK3D_BODY_INSPECTOR => inspection_panel::render(doc.snapshot, active_representation_id, labels),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    /// 🌉️ The flagship seam: `puzzle3d_catalog_fragment`'s first real caller. Wraps the block-3d
    /// document's puzzle3d-shaped catalog fragment as a `kit.catalog`-schema `Media` value for the
    /// `"catalog:out"` port declared in `crate::artifacts::block3d::engine::block3d_io`. `wanted_tags`
    /// should come from `cfg.wanted_tags` but `DocumentApp::export_media`'s landed signature doesn't
    /// thread `ConfigView` through yet — see `Block3dConfig::wanted_tags`'s doc — so this always
    /// resolves the active representation with an empty (all-tags) filter until that lands. Falls
    /// through to the default whole-document pack export for every other port (`"document:out"`).
    fn export_media(port: &str, doc: &DocumentView<'_, Block3dSnapshot>) -> Result<Media, MediaError> {
        if port != "catalog:out" {
            // 🌉️ Reimplements `DocumentApp::export_media`'s default `"document:out"` behavior
            // verbatim — overriding the trait method forfeits the ability to delegate back to its
            // own default body, so the whole-document pack export is duplicated here rather than
            // left unreachable for this app.
            if port != "document:out" {
                return Err(MediaError::NotImplemented);
            }
            let media_type = Self::io().map_or(MediaType { class: MediaClass::Kit, form: MediaForm::Type }, |io| io.document_media_type);
            let bytes = store::DocumentPack::encode_pack(doc.snapshot);
            return Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } });
        }
        let fragment = crate::artifacts::block3d::engine::puzzle3d_catalog_fragment(doc.snapshot, &[]);
        Ok(Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: MediaPayload::Structured { schema: KIT_CATALOG_ARTIFACT_ID.into(), json: fragment.to_string() } })
    }
}
//#endregion 🔖️Block3dPlayApp

//#region 🔖️Manifest
pub fn create_block3d_app() -> App {
    App::from_builder(
        App::builder(BLOCK3D_PLAY_APP_ID, LocalizedLabel::native("Block 3D", "Block 3D"))
            .document(["semio", "block", "3d"])
            .artifact_kind(artifact_kind())
            // 🗂️ The puzzle3d catalog artifact this app's new `"catalog:out"` port produces — see
            // `crate::artifacts::block3d::engine::block3d_io`/`Block3dPlayApp::export_media`.
            .artifact_kind(ArtifactKindSpec {
                id: KIT_CATALOG_ARTIFACT_ID.into(),
                name: "Kit Catalog".into(),
                source_format: KIT_CATALOG_ARTIFACT_ID.into(),
                component_kind: "kit-catalog".into(),
                dimension: "3d".into(),
                media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                schema: KIT_CATALOG_ARTIFACT_ID.into(),
                export_formats: vec![],
                import_formats: vec![],
                    export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    })
            .icon_id("box")
            .mode_def(edit_mode::definition())
            .default_mode_id(edit_mode::BLOCK3D_PLAY_MODE_EDIT)
            .window_kind_def(world::definition())
            .utility(UtilityDefinition::new(BLOCK3D_UTILITY_SELECT, LocalizedLabel::native("Select", "Auswählen"), "mouse-pointer"))
            .utility(UtilityDefinition::new(BLOCK3D_UTILITY_SURFACE_BRUSH, LocalizedLabel::native("Surface brush", "Flächenpinsel"), "paintbrush"))
            .window_kind_utilities(world::BLOCK3D_WINDOW_WORLD, vec![BLOCK3D_UTILITY_SELECT.into(), BLOCK3D_UTILITY_SURFACE_BRUSH.into()])
            .default_layout(edit_mode::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            .mutation("patchObjectKind", LocalizedLabel::native("Patch Object Kind", "Objektart bearbeiten"))
            .mutation("addRepresentation", LocalizedLabel::native("Add Representation", "Darstellung hinzufügen"))
            .mutation("removeRepresentation", LocalizedLabel::native("Remove Representation", "Darstellung entfernen"))
            .mutation("addVortexKind", LocalizedLabel::native("Add Vortex Kind", "Wirbelart hinzufügen"))
            .mutation("removeVortexKind", LocalizedLabel::native("Remove Vortex Kind", "Wirbelart entfernen"))
            .mutation("addVortex", LocalizedLabel::native("Add Vortex", "Wirbel hinzufügen"))
            .mutation("removeVortex", LocalizedLabel::native("Remove Vortex", "Wirbel entfernen"))
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .mutation("edit", LocalizedLabel::native("Edit", "Bearbeiten"))
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
            .view_action("setActiveRepresentation", LocalizedLabel::native("Set Active Representation", "Aktive Darstellung festlegen"))
            .view_action("setWindowRepresentations", LocalizedLabel::native("Set Window Representations", "Fensterdarstellungen festlegen"))
            .view_action("toggleWindowRepresentation", LocalizedLabel::native("Toggle Representation", "Darstellung umschalten"))
            .view_action("setWindowArrangement", LocalizedLabel::native("Set Arrangement", "Anordnung festlegen"))
            .view_action("setWindowSpacing", LocalizedLabel::native("Set Spacing", "Abstand festlegen"))
            .view_action("setBrushVortexKind", LocalizedLabel::native("Set Brush Vortex Kind", "Pinsel-Wirbelart festlegen"))
            .view_action("setBrushRadius", LocalizedLabel::native("Set Brush Radius", "Pinselradius festlegen"))
            .view_action("setBrushFlip", LocalizedLabel::native("Set Brush Flip", "Pinselrichtung umkehren"))
            .view_action("worldSurfaceHover", LocalizedLabel::native("Surface Hover", "Flächenhover"))
            .view_action("worldSurfaceLeave", LocalizedLabel::native("Surface Leave", "Fläche verlassen"))
            .mutation("worldSurfacePlace", LocalizedLabel::native("Place Vortex", "Wirbel platzieren"))
            .view_action("selectVortex", LocalizedLabel::native("Select Vortex", "Wirbel auswählen"))
            .view_action("hoverVortex", LocalizedLabel::native("Hover Vortex", "Wirbel hovern"))
            .mutation("patchRepresentation", LocalizedLabel::native("Patch Representation", "Darstellung bearbeiten"))
            .io(crate::artifacts::block3d::engine::block3d_io()),
    )
    .example(
        crate::apps::block3d::commands::example::BLOCK3D_EXAMPLE_CAPSULE,
        LocalizedLabel::native("Nakagin Capsule", "Nakagin Capsule"),
        serde_json::to_string(&crate::artifacts::block3d::dsl::parse_dsl(crate::artifacts::block3d::dsl::BLOCK3D_NAKAGIN_CAPSULE_EXAMPLE_TEXT).unwrap_or_default()).unwrap_or_default(),
        "building",
    )
    .example(
        crate::apps::block3d::commands::example::BLOCK3D_EXAMPLE_FOREST_LEFT,
        LocalizedLabel::native("Hexagonal Cut Concrete Forest Left", "Sechseckig geschnittener Betonwald links"),
        serde_json::to_string(&crate::artifacts::block3d::dsl::parse_dsl(crate::artifacts::block3d::dsl::BLOCK3D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).unwrap_or_default()).unwrap_or_default(),
        "list-tree",
    )
    .workflow("block3d", "Block 3D", "model")
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app as sdk_new_app, new_app_with_registry};
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsDocumentApp, ViewModel};

    pub type Block3dApp = VcsDocumentApp<Block3dPlayApp>;

    pub fn new_app() -> Block3dApp {
        sdk_new_app::<Block3dPlayApp>()
    }

    pub fn app_with_registry() -> Block3dApp {
        new_app_with_registry::<Block3dPlayApp>(create_block3d_app)
    }

    pub fn dispatch(app: &mut Block3dApp, command: Block3dCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut Block3dApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }

    pub fn main_window_measures(app: &mut Block3dApp) -> Vec<semio_framework_plugin::WindowMeasure> {
        app.window_measures().get(BLOCK3D_DEFAULT_WINDOW_ID).cloned().unwrap_or_default()
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use testkit::{new_app, Block3dApp};
    use semio_framework_plugin::PluginApp;


    //#region 🔖️CommandSurface
    fn every_command() -> Vec<Block3dCommand> {
        vec![
            Block3dCommand::PatchObjectKind(patch_object_kind::PatchObjectKind { field: "name".into(), value: "x".into() }),
            Block3dCommand::AddRepresentation(add_representation::AddRepresentation {}),
            Block3dCommand::RemoveRepresentation(remove_representation::RemoveRepresentation { id: "r0".into() }),
            Block3dCommand::AddVortexKind(add_vortex_kind::AddVortexKind {}),
            Block3dCommand::RemoveVortexKind(remove_vortex_kind::RemoveVortexKind { id: "v0".into() }),
            Block3dCommand::AddVortex(add_vortex::AddVortex {}),
            Block3dCommand::RemoveVortex(remove_vortex::RemoveVortex { id: "v0".into() }),
            Block3dCommand::SetActiveExample(set_active_example::SetActiveExample { id: "capsule".into() }),
            Block3dCommand::Edit(edit::Edit { text: "{}".into() }),
            Block3dCommand::SetSelection(set_selection::SetSelection { ids: vec!["r0".into()] }),
            Block3dCommand::SetActiveRepresentation(set_active_representation::SetActiveRepresentation { representation_id: Some("r0".into()) }),
            Block3dCommand::SetWindowRepresentations(set_window_representations::SetWindowRepresentations { window_id: "w0".into(), representation_ids: vec!["r0".into()] }),
            Block3dCommand::ToggleWindowRepresentation(toggle_window_representation::ToggleWindowRepresentation { window_id: "w0".into(), representation_id: "r0".into(), visible: true }),
            Block3dCommand::SetWindowArrangement(set_window_arrangement::SetWindowArrangement { window_id: "w0".into(), arrangement: "x".into() }),
            Block3dCommand::SetWindowSpacing(set_window_spacing::SetWindowSpacing { window_id: "w0".into(), spacing: 8.0 }),
            Block3dCommand::SetActiveUtility(set_active_utility::SetActiveUtility { window_id: "w0".into(), utility_id: "select".into() }),
            Block3dCommand::SetBrushVortexKind(set_brush_vortex_kind::SetBrushVortexKind { vortex_kind_id: Some("v0".into()) }),
            Block3dCommand::SetBrushRadius(set_brush_radius::SetBrushRadius { radius: 0.3 }),
            Block3dCommand::SetBrushFlip(set_brush_flip::SetBrushFlip { flip: true }),
            Block3dCommand::HoverSurface(hover_surface::HoverSurface { window_id: "w0".into(), object_id: "r0".into(), position: [0.0, 0.0, 0.0], normal: [0.0, 1.0, 0.0] }),
            Block3dCommand::LeaveSurface(leave_surface::LeaveSurface {}),
            Block3dCommand::PlaceVortex(place_vortex::PlaceVortex { window_id: "w0".into(), object_id: "r0".into(), position: [0.0, 0.0, 0.0], normal: [0.0, 1.0, 0.0] }),
            Block3dCommand::SetCamera(set_camera::SetCamera { camera: BlockCamera3d::default() }),
            Block3dCommand::SelectVortex(select_vortex::SelectVortex { full_id: "r0:v0".into(), merge: true }),
            Block3dCommand::HoverVortex(hover_vortex::HoverVortex { full_id: Some("r0:v0".into()) }),
            Block3dCommand::PatchRepresentation(patch_representation::PatchRepresentation { id: "r0".into(), field: "name".into(), value: "x".into() }),
        ]
    }

    #[test]
    fn command_ids_are_unique_and_cover_every_row() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(Block3dCommand::command_id).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 26, "every Block3dCommand row must be covered by every_command()");
    }

    #[test]
    fn every_command_round_trips_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// 🧷️ Pins the exact pre-migration bytes for the three divergent-key rows plus a handful of
    /// `Option`/`Vec` rows — copied verbatim from the ticket's `🧪️wire-baseline-3d-before.txt`.
    #[test]
    fn divergent_key_rows_keep_their_pre_migration_bytes() {
        let hex = |command: &Block3dCommand| protocol::OpBinary::encode_op(command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>();
        assert_eq!(protocol::OpText::print_op(&Block3dCommand::LeaveSurface(leave_surface::LeaveSurface {})), "leaveSurface");
        assert_eq!(hex(&Block3dCommand::LeaveSurface(leave_surface::LeaveSurface {})), "01140000");
    }

    #[test]
    fn command_from_action_covers_every_declared_action_and_rejects_unknown_ones() {
        semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<Block3dPlayApp>(create_block3d_app);
        assert!(Block3dPlayApp::command_from_action("noSuchAction", None).is_err());
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️Manifest
    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let definition = create_block3d_app().definition;
        assert_eq!(definition.modes.len(), 1);
        assert_eq!(definition.window_kinds.len(), 1);
        for body_key in [document_panel::BLOCK3D_BODY_DOCUMENT, inspection_panel::BLOCK3D_BODY_INSPECTOR] {
            assert!(definition.panel_tabs.iter().any(|tab| tab.body_key.as_deref() == Some(body_key)), "panel tab {body_key} is stitched into the manifest");
        }
        assert!(definition.artifact_kinds.iter().any(|kind| kind.id == "kit.catalog"));
    }

    #[test]
    fn renders_document_tree_and_inspector() {
        let mut app: Block3dApp = new_app();
        let json = testkit::render(&mut app, document_panel::BLOCK3D_BODY_DOCUMENT);
        assert!(json.contains("Representations"));
        let inspector = testkit::render(&mut app, inspection_panel::BLOCK3D_BODY_INSPECTOR);
        assert!(inspector.contains("\"type\":\"tree\""));
        assert!(inspector.contains("Name"));
        assert!(inspector.contains("Vortices"));
    }
    //#endregion 🔖️Manifest

    //#region 🔖️Behavior
    #[test]
    fn add_representation_then_set_active_then_render_world_shows_mesh() {
        let mut app: Block3dApp = new_app();
        testkit::dispatch(&mut app, Block3dCommand::AddRepresentation(add_representation::AddRepresentation {}));
        let representation_id = app.snapshot().expect("snapshot").representations[0].id.clone();
        testkit::dispatch(&mut app, Block3dCommand::SetActiveRepresentation(set_active_representation::SetActiveRepresentation { representation_id: Some(representation_id) }));
        let json = testkit::render(&mut app, world::BLOCK3D_BODY_WORLD);
        assert!(json.contains("\"type\":\"componentScene\""), "world body must render a 3d scene");
    }

    #[test]
    fn add_vortex_kind_then_add_vortex_then_remove_round_trips() {
        let mut app: Block3dApp = new_app();
        testkit::dispatch(&mut app, Block3dCommand::AddVortexKind(add_vortex_kind::AddVortexKind {}));
        testkit::dispatch(&mut app, Block3dCommand::AddVortex(add_vortex::AddVortex {}));
        let projection = app.snapshot().expect("snapshot");
        assert_eq!(projection.vortices.len(), 1);
        let vortex_id = projection.vortices[0].id.clone();
        testkit::dispatch(&mut app, Block3dCommand::RemoveVortex(remove_vortex::RemoveVortex { id: vortex_id }));
        assert_eq!(app.snapshot().expect("snapshot").vortices.len(), 0);
    }

    #[test]
    fn set_active_example_loads_capsule_fixture() {
        let mut app: Block3dApp = new_app();
        testkit::dispatch(&mut app, Block3dCommand::SetActiveExample(set_active_example::SetActiveExample { id: crate::apps::block3d::commands::example::BLOCK3D_EXAMPLE_CAPSULE.into() }));
        let projection = app.snapshot().expect("snapshot");
        assert_eq!(projection.object_kind.id, "Capsule J");
        assert_eq!(projection.representations.len(), 2);
    }

    #[test]
    fn undo_redo_round_trips_through_the_wrapper() {
        let mut app: Block3dApp = new_app();
        testkit::dispatch(&mut app, Block3dCommand::AddVortexKind(add_vortex_kind::AddVortexKind {}));
        assert_eq!(app.snapshot().expect("snapshot").vortex_kinds.len(), 1);
        app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).expect("undo");
        assert_eq!(app.snapshot().expect("snapshot").vortex_kinds.len(), 0);
        app.handle_action("redo", None, &semio_framework_plugin::testkit::meta("local")).expect("redo");
        assert_eq!(app.snapshot().expect("snapshot").vortex_kinds.len(), 1);
    }

    #[test]
    fn set_selection_writes_config_not_document() {
        let mut app: Block3dApp = new_app();
        let result = app.dispatch_typed(Block3dCommand::SetSelection(set_selection::SetSelection { ids: vec!["representation:r0".into()] }), &semio_framework_plugin::testkit::meta("local")).expect("select");
        assert!(result.mutations.is_empty(), "setSelection is config-only and must emit no document operations");
    }

    #[test]
    fn export_media_catalog_out_wraps_the_puzzle3d_fragment() {
        let mut app: Block3dApp = new_app();
        testkit::dispatch(&mut app, Block3dCommand::SetActiveExample(set_active_example::SetActiveExample { id: crate::apps::block3d::commands::example::BLOCK3D_EXAMPLE_CAPSULE.into() }));
        let media = app.export_media("catalog:out").expect("export catalog");
        assert_eq!(media.media_type, MediaType { class: MediaClass::Kit, form: MediaForm::Type });
        match media.payload {
            MediaPayload::Structured { schema, json } => {
                assert_eq!(schema, "kit.catalog");
                let value: Value = serde_json::from_str(&json).expect("valid json");
                assert_eq!(value["objectKinds"][0]["id"], "Capsule J");
            }
            other => panic!("expected Structured payload, got {other:?}"),
        }
    }

    #[test]
    fn place_vortex_on_surface_auto_creates_kind_and_vortex() {
        let mut app: Block3dApp = new_app();
        testkit::dispatch(&mut app, Block3dCommand::SetActiveExample(set_active_example::SetActiveExample { id: crate::apps::block3d::commands::example::BLOCK3D_EXAMPLE_CAPSULE.into() }));
        testkit::dispatch(
            &mut app,
            Block3dCommand::PlaceVortex(place_vortex::PlaceVortex { window_id: BLOCK3D_DEFAULT_WINDOW_ID.into(), object_id: "r0".into(), position: [0.5, 0.0, 1.0], normal: [0.0, 1.0, 0.0] }),
        );
        let projection = app.snapshot().expect("snapshot");
        assert!(!projection.vortex_kinds.is_empty());
        assert_eq!(projection.vortices.len(), 2);
    }

    #[test]
    fn command_from_action_bridges_set_active_example() {
        let app = Block3dPlayApp;
        assert!(matches!(Block3dPlayApp::command_from_action("setActiveExample", Some(&serde_json::json!({ "exampleId": "capsule" }))), Ok(Block3dCommand::SetActiveExample(set_active_example::SetActiveExample { id })) if id == "capsule"));
    }
    //#endregion 🔖️Behavior

    //#region 🔖️WindowMeasures
    /// 🧬️ Kind-discipline wrapper: the real registry enforces View actions never emit document
    /// operations. Exercising it here (rather than only the plain `new_app()`) is the reason
    /// `testkit::app_with_registry` exists.
    #[test]
    fn view_actions_never_emit_document_mutations_under_the_real_registry() {
        let mut app = testkit::app_with_registry();
        let result = testkit::dispatch(&mut app, Block3dCommand::SetSelection(set_selection::SetSelection { ids: vec!["r0".into()] }));
        assert!(result.mutations.is_empty(), "setSelection is a view action and must never reach document operations under kind discipline");
    }

    /// 🎚️ The world window collects its five option measures (representations/quick-pick/arrangement/
    /// spacing/brush) fresh per frame — never frozen into the manifest.
    #[test]
    fn world_window_measures_collect_all_five_options() {
        let mut app: Block3dApp = new_app();
        testkit::dispatch(&mut app, Block3dCommand::AddRepresentation(add_representation::AddRepresentation {}));
        let measures = testkit::main_window_measures(&mut app);
        assert_eq!(measures.len(), 5, "world window must expose representations/quick-pick/arrangement/spacing/brush");
    }
    //#endregion 🔖️WindowMeasures
}
//#endregion 🧪️Tests
