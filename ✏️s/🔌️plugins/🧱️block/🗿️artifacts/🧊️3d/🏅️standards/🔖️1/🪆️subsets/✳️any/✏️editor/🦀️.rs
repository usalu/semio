//! 🏙️ Block 3D editor — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1).
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the world window
//! (+ its `☑️options/*`) in `🎭️modes/✏️edit/🪟️windows/🌐️world`, panel trees in `📌️panels/*`, labels in
//! `🗣️terminology`, view state in `🎚️config`, world-scene compute needing both document+config in
//! `🌍️world` (editor-only compute facet, no taxonomy slot — see that file's own doc), pure document-side
//! compute in `crate::standards::v1::subsets::any::schema`/`crate::standards::v1::subsets::any::schema::inferences`, and
//! this surface's own typed media I/O surface (below — constitutional: general, an artifact must never
//! depend on a surface, so it lives here rather than under `🗿️artifacts`).

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_block_3d_mutations::Block3dMutation;
use semio_s_artifact_block_3d_model::{artifact_kind, Block3dSnapshot, BLOCK3D_DIALECT, BLOCK_3D_SCHEMA};
use semio_s_artifact_block_3d_editor_commands::patch_object_kind;
use semio_s_artifact_block_3d_editor_commands::set_camera;
use semio_s_artifact_block_3d_editor_commands::{add_representation, patch_representation, remove_representation};
use semio_s_artifact_block_3d_editor_commands::{add_vortex, remove_vortex};
use semio_s_artifact_block_3d_editor_commands::{add_vortex_kind, remove_vortex_kind};
use semio_s_artifact_block_3d_editor_commands::{edit, set_active_example};
use semio_s_artifact_block_3d_editor_commands::{hover_surface, leave_surface, place_vortex, set_brush_flip, set_brush_radius, set_brush_vortex_kind};
use semio_s_artifact_block_3d_editor_commands::{set_active_representation, set_window_arrangement, set_window_representations, set_window_spacing, toggle_window_representation};
use semio_s_artifact_block_3d_editor_config::{Block3dConfig, Block3dConfigMutation};
use semio_s_artifact_block_3d_editor_modes::{edit as edit_mode, edit::windows::world, edit::windows::world::transient::{Block3dBrushPreview, Block3dWorldWindowTransientOwner, SetBrushPreview}, BLOCK3D_PLAY_APP_ID};
use crate::panels::{document as document_panel, inspection as inspection_panel};
use semio_s_artifact_block_3d_editor_terminology::block3d_labels;
use semio_s_plugin_block::BlockCamera3d;
use semio_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_plugin::ActionDescriptor;
use semio_plugin::AppOperationContext;
use semio_plugin::ArtifactEditor;
use semio_plugin::ArtifactOwnedToolJobFactory;
use semio_plugin::ArtifactOwnedToolJobRequest;
use semio_plugin::ArtifactToolFactoryRegistry;
use semio_plugin::ArtifactToolPublicationContract;
use semio_plugin::ArtifactToolPublicationLane;
use semio_plugin::ArtifactView;
use semio_plugin::ConfigView;
use semio_plugin::DraftView;
use semio_plugin::Editor;
use semio_plugin::EditorApp;
use semio_plugin::Emit;
use semio_plugin::EphemeralEmit;
use semio_plugin::Fault;
use semio_plugin::FaultCode;
use semio_plugin::FaultOrigin;
use semio_ui_locale::Label;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::Media;
use semio_plugin::MediaClass;
use semio_plugin::MediaError;
use semio_plugin::MediaForm;
use semio_plugin::MediaPayload;
use semio_plugin::MediaType;
use semio_plugin::NoDraft;
use semio_plugin::NoDraftMutation;
use semio_plugin::UtilityDefinition;
// 🚧️ SDK GAP: `Dialect`/`InteractionView` are still only reachable through the `app` submodule they're
// declared in — not (yet) in `semio_plugin`'s curated crate-root re-export list, unlike
// `ArtifactEditor`/`Editor` above (closed by W0-F). Mirrors the sibling `👁️viewer`'s own gap note.
use semio_framework::{DomainTopology, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractionTopology, InteractiveJobClassification, MergeMode, SelectionMethod, SelectionMode, SelectionSpec, ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError, TopologyNode};
use semio_plugin::app::{Dialect, InteractionView};
use semio_framework_pack_json::Value;
use std::collections::{BTreeMap, HashMap};
use semio_framework_2d_compute::EngineHandles;

//#region 🔖️Constants
pub const BLOCK3D_DEFAULT_WINDOW_ID: &str = "block3d-world";
pub const BLOCK3D_UTILITY_SELECT: &str = "select";
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the framework-owned hover/selection
/// domain over this surface's representations ("surface" granularity) and rim-vortex templates ("vortex"
/// granularity, the default) — replaces the deleted `Block3dConfig.selected_ids`/`hovered_vortex_full_id`.
pub const BLOCK3D_INTERACTION_VORTEX: &str = "vortex";
pub const BLOCK3D_GRANULARITY_VORTEX: &str = "vortex";
pub const BLOCK3D_GRANULARITY_SURFACE: &str = "surface";
/// 🗂️ The `s/plugin/puzzle` 3d catalog artifact kind block3d's `"catalog:out"` port produces — see
/// `block3d_io` and `Block3dPlayApp::export_media`.
use semio_s_plugin_block_catalog::ARTIFACT_ID;

/// 🎯️ The semantic-contract action binding addressed at this surface — the single factory every
/// contract-built node (`📌️panels/*`) binds its `on_change`/item actions with.
pub fn block3d_action(action: &str, args: Option<semio_plugin::UiValue>) -> semio_plugin::UiAssemblyResult<(semio_plugin::ActionId, Option<semio_plugin::UiValue>)> {
    semio_plugin::ActionFactory::new(BLOCK3D_PLAY_APP_ID).action(action, args)
}

/// 🏷️ Admits resolved block3d text into the semantic UI contract's fixed-capacity label.
pub fn ui_label(value: impl AsRef<str>) -> semio_plugin::UiAssemblyResult<semio_ui_contract::Label> {
    semio_ui_contract::Label::try_from(value.as_ref()).map_err(|_| semio_plugin::PluginAssemblyError::new("ui.fixed-capacity", "block3d UI label admission failed"))
}

/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_plugin::UiAssemblyResult<semio_plugin::UiValue> {
    semio_plugin::UiText::try_from_str(value.as_ref())
        .map(semio_plugin::UiValue::Text)
        .ok_or_else(|| semio_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}

/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_plugin::UiValue {
    semio_plugin::UiValue::Bool(value)
}

/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> semio_plugin::UiValue {
    semio_plugin::UiValue::Number(value.into())
}

/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = semio_plugin::UiValue>) -> semio_plugin::UiAssemblyResult<semio_plugin::UiValue> {
    let mut builder = semio_plugin::UiListBuilder::try_new()
        .ok_or_else(|| semio_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder
            .push(value)
            .map_err(|_| semio_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_plugin::UiValue)>) -> semio_plugin::UiAssemblyResult<semio_plugin::UiValue> {
    let mut builder = semio_plugin::UiMapBuilder::try_new()
        .ok_or_else(|| semio_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder
            .push(key.to_owned(), value)
            .map_err(|_| semio_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_plugin::UiValue::Map(builder.finish()))
}

fn block3d_render_window_id(view_state: &semio_plugin::ViewModel) -> &str {
    view_state.window_id.as_deref().filter(|window_id| !window_id.is_empty()).unwrap_or(BLOCK3D_DEFAULT_WINDOW_ID)
}

fn f64_vec3_field(args: Option<&Value>, key: &str) -> Option<[f64; 3]> {
    let array = args.and_then(|value| value.get(key))?.as_array()?;
    if array.len() < 3 {
        return None;
    }
    Some([array[0].as_f64()?, array[1].as_f64()?, array[2].as_f64()?])
}

fn window_id_from_args(args: Option<&Value>) -> String {
    args.and_then(|value| value.get("windowId").or_else(|| value.get("pane")).or_else(|| value.get("surfaceId"))).and_then(Value::as_str).map_or_else(|| BLOCK3D_DEFAULT_WINDOW_ID.into(), str::to_string)
}
//#endregion 🔖️Constants

//#region 🔖️Io
/// 🔌️ `Block3dPlayApp`'s typed media I/O surface (`AppDefinition.io`) — the implicit document ports
/// (`Kit×Type`, matching the `"3d.block"` artifact kind) plus the `"catalog:out"` port: the puzzle3d
/// seam that gives `puzzle3d_catalog_fragment` a real caller (see `export_media` below).
pub fn block3d_io() -> semio_plugin::AppIo {
    ::semio_framework_async::poll::resolve_ready(
        ::semio_framework_async::poll::resolve_ready(semio_plugin::AppIo::from_artifact(
            BLOCK_3D_SCHEMA,
            MediaType { class: MediaClass::Kit, form: MediaForm::Type },
            semio_plugin::ArtifactPresentation { id: "3d.block".into(), name: "Object Kind".into(), dimension: "3d".into(), component_kind: "block3d".into() },
        ))
        .with_ports(vec![semio_plugin::MediaPortSpec {
            id: "catalog:out".into(),
            label: LocalizedLabel::native("Kit Catalog", "Kit-Katalog"),
            direction: semio_plugin::MediaPortDirection::Out,
            media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
            kind_id: Some("kit.catalog".into()),
            required: false,
            multiplicity: semio_plugin::PortMultiplicity::Many,
        }]),
    )
}
//#endregion 🔖️Io

//#region 🔖️Commands
semio_plugin::app_commands! {
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
        "setActiveRepresentation" as "setActiveRepresentation" => set_active_representation::SetActiveRepresentation,
        "setWindowRepresentations" as "setWindowRepresentations" => set_window_representations::SetWindowRepresentations,
        "toggleWindowRepresentation" as "toggleWindowRepresentation" => toggle_window_representation::ToggleWindowRepresentation,
        "setWindowArrangement" as "setWindowArrangement" => set_window_arrangement::SetWindowArrangement,
        "setWindowSpacing" as "setWindowSpacing" => set_window_spacing::SetWindowSpacing,
        "setBrushVortexKind" as "setBrushVortexKind" => set_brush_vortex_kind::SetBrushVortexKind,
        "setBrushRadius" as "setBrushRadius" => set_brush_radius::SetBrushRadius,
        "setBrushFlip" as "setBrushFlip" => set_brush_flip::SetBrushFlip,
        "worldSurfaceHover" as "hoverSurface" => hover_surface::HoverSurface,
        "worldSurfaceLeave" as "leaveSurface" => leave_surface::LeaveSurface,
        "worldSurfacePlace" as "placeVortex" => place_vortex::PlaceVortex,
        "setCamera" as "setCamera" => set_camera::SetCamera,
        "patchRepresentation" as "patchRepresentation" => patch_representation::PatchRepresentation,
    }
}
//#endregion 🔖️Commands

#[path = "🧰️retained-commands/🦀️.rs"]
mod retained_commands;
use self::retained_commands::*;

#[path = "📬️store-preparation/🦀️.rs"]
mod store_preparation;
use self::store_preparation::*;

#[path = "🧩️play-app/🦀️.rs"]
mod play_app;
pub use self::play_app::*;

#[path = "📋️manifest/🦀️.rs"]
mod manifest;
pub use self::manifest::*;

//#region 🪢️Mounts
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;
#[path = "."]
pub mod panels {
    #[path = "📌️panels/🗿️artifact/🦀️.rs"]
    pub mod document;
    #[path = "📌️panels/🔍️inspection/🦀️.rs"]
    pub mod inspection;
}
//#endregion 🪢️Mounts
