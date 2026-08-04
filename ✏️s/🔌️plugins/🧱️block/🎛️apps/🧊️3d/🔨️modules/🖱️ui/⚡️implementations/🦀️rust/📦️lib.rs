//! 🏙️ Block 3D app — DocumentApp impl, render, manifest (constitutional: ui). B1: pure-trait
//! conversion (mirrors `shooting_ui`'s pilot) — `Block3dPlayApp` is a unit struct; every former
//! `RefCell` runtime field (`selected_ids`/`active_representation_id`) now lives in
//! `block_3d_engine::Block3dConfig`, written via `block_3d_op::Block3dConfigOperation`s (real
//! `backwards`, no ad hoc `InverseAction`); every action dispatches through the single typed
//! `block_3d_protocol::Block3dCommand` channel via `DocumentApp::handle`.

use block_3d::{Block3dDefinition, Block3dVortexKind, Block3dVortexTemplate, BLOCK_3D_SCHEMA};
use block_3d_engine::{
    resolve_brush_vortex_kind_id, block3d_window_view, default_vortex_kind, instance_offset_for_representation, next_id, visible_representations, world_camera_json,
    world_instances_json, world_interaction_json, world_meshes_json, world_selection_json, world_vortices_json, Block3dBrushPreview, Block3dConfig, BLOCK3D_DEFAULT_WINDOW_ID,
    BLOCK3D_UTILITY_SELECT, BLOCK3D_UTILITY_SURFACE_BRUSH,
};
use block_3d_op::{Block3dConfigOperation, Block3dOperation};
use block_3d_protocol::Block3dCommand;
use block_shared::BlockRepresentation;
use semio_framework_plugin::{
    build_world_3d_scene, tree_item_with_action, ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_text, world3d_scene_extended, ActionDescriptor, App, AppLabels, ArtifactKindSpec, ConfigView,
    DocumentApp, DocumentView, Emit, Fault, FaultCode, FaultOrigin, Label, Locale, LocalizedLabel, MeasureSelectItem, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, OsMediaCapability,
    PanelGroup, PanelTreeBuilder, SurfaceKind, Terminology, UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiPresence, UiSelectItem, UiSelectNode, UiTreeItemNode, UtilityDefinition, WindowMeasure,
};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const BLOCK3D_PLAY_APP_ID: &str = "block3d-play";
const BLOCK3D_BODY_WORLD: &str = "block3d.play.world";
const BLOCK3D_BODY_DOCUMENT: &str = "block3d.play.document";
const BLOCK3D_BODY_INSPECTOR: &str = "block3d.play.inspector";
const BLOCK3D_WINDOW_WORLD: &str = "block3d-world";
const BLOCK3D_EXAMPLE_CAPSULE: &str = "nakagin-capsule";
const BLOCK3D_EXAMPLE_FOREST_LEFT: &str = "hexagonal-cut-concrete-forest-left";
/// 🗂️ The `s/plugin/puzzle` 3d catalog artifact kind block3d's `"catalog:out"` port produces — see
/// `block_3d_engine::block3d_io` and `Block3dPlayApp::export_media`.
const BLOCK3D_PLAY_SURFACE_ID: &str = "block3d.play.world";

const KIT_CATALOG_ARTIFACT_ID: &str = "kit.catalog";
//#endregion 🔖️Constants

fn block3d_resolve_world_body(body_key: &str) -> (&str, String) {
    if body_key == BLOCK3D_BODY_WORLD || body_key.starts_with(&format!("{BLOCK3D_BODY_WORLD}:")) {
        if let Some((_, window_id)) = body_key.split_once(':') {
            return (BLOCK3D_BODY_WORLD, window_id.to_string());
        }
        return (BLOCK3D_BODY_WORLD, BLOCK3D_DEFAULT_WINDOW_ID.into());
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
        .map(str::to_string)
        .unwrap_or_else(|| BLOCK3D_DEFAULT_WINDOW_ID.into())
}

//#region 🔖️Locale
/// 🗣️ B1: `cfg.locale`-driven counterpart to the deleted `ViewState`-driven
/// `semio_framework_plugin::is_de_locale`/`resolve_labels` — mirrors `cad_ui`'s identical region.
fn block3d_is_de_locale(cfg: &Block3dConfig) -> bool {
    cfg.locale.starts_with("de")
}

/// 🗣️ `Block3dConfig.locale` (a BCP-47 tag, was shell-provided `ViewState.locale` pre-B1) mapped onto
/// the SDK's exhaustive `Locale` enum.
fn block3d_locale(cfg: &Block3dConfig) -> Locale {
    if block3d_is_de_locale(cfg) {
        Locale::De
    } else {
        Locale::En
    }
}

/// 🗣️ Resolves the active `Block3dLabels` cell from the config-carried locale (was shell-provided
/// `ViewState`, deleted by B1) via the SDK's two-axis `AppLabels::labels`. `Block3dConfig` carries no
/// terminology field, so terminology is always `Native`.
fn block3d_labels(cfg: &Block3dConfig) -> &'static Block3dLabels {
    Block3dLabels::labels(block3d_locale(cfg), Terminology::Native)
}
//#endregion 🔖️Locale

//#region 🔖️Terminology
// 🗣️ Complete UI label set for the block3d-play app; one field per label makes every locale combination compile-checked. No separate reuse-terminology concept, so reuse repeats native.
semio_framework_plugin::app_labels! {
    struct Block3dLabels {
        window_world: native_en "Object Kind", native_de "Objektart", reuse_en "Object Kind", reuse_de "Objektart";
        name: native_en "Name", native_de "Name", reuse_en "Name", reuse_de "Name";
        label: native_en "Label", native_de "Bezeichnung", reuse_en "Label", reuse_de "Bezeichnung";
        representation: native_en "Representation", native_de "Darstellung", reuse_en "Representation", reuse_de "Darstellung";
        representations: native_en "Representations", native_de "Darstellungen", reuse_en "Representations", reuse_de "Darstellungen";
        vortex_kinds: native_en "Vortex Kinds", native_de "Wirbelarten", reuse_en "Vortex Kinds", reuse_de "Wirbelarten";
        vortices: native_en "Vortices", native_de "Wirbel", reuse_en "Vortices", reuse_de "Wirbel";
        no_representations: native_en "(no representations)", native_de "(keine Darstellungen)", reuse_en "(no representations)", reuse_de "(keine Darstellungen)";
        no_vortices: native_en "(no vortices)", native_de "(keine Wirbel)", reuse_en "(no vortices)", reuse_de "(keine Wirbel)";
        summary: native_en "Object kind", native_de "Objektart", reuse_en "Object kind", reuse_de "Objektart";
        arrangement: native_en "Arrangement", native_de "Anordnung", reuse_en "Arrangement", reuse_de "Anordnung";
        spacing: native_en "Spacing", native_de "Abstand", reuse_en "Spacing", reuse_de "Abstand";
        brush: native_en "Surface brush", native_de "Flächenpinsel", reuse_en "Surface brush", reuse_de "Flächenpinsel";
        brush_radius: native_en "Radius", native_de "Radius", reuse_en "Radius", reuse_de "Radius";
        flip_normal: native_en "Flip normal", native_de "Normale umkehren", reuse_en "Flip normal", reuse_de "Normale umkehren";
        show_all: native_en "All representations", native_de "Alle Darstellungen", reuse_en "All representations", reuse_de "Alle Darstellungen";
    }
}
//#endregion 🔖️Terminology

fn block3d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(BLOCK3D_PLAY_APP_ID).action(action, args)
}

//#region 🔖️Panels
fn build_document_tree(definition: &Block3dDefinition, selected: &[String], labels: &Block3dLabels) -> UiNode {
    let builder = PanelTreeBuilder::new("block3d-play-document");
    let representation_items: Vec<UiTreeItemNode> = definition
        .representations
        .iter()
        .map(|representation| UiTreeItemNode {
            icon_id: Some("box".into()),
            ..tree_item_with_action(builder.item_id("representation", &representation.id), Label::data(representation.name.clone()), representation.mesh_url.clone(), block3d_action("setSelection", None))
        })
        .collect();
    let vortex_items: Vec<UiTreeItemNode> = definition
        .vortices
        .iter()
        .map(|vortex| UiTreeItemNode { icon_id: Some("circle-dot".into()), ..tree_item_with_action(builder.item_id("vortex", &vortex.id), Label::data(vortex.vortex_kind.clone()), None, block3d_action("setSelection", None)) })
        .collect();
    builder
        .section_or_placeholder("block3d-play-document.representations", Some(labels.representations.into()), true, representation_items, labels.no_representations)
        .section_or_placeholder("block3d-play-document.vortices", Some(labels.vortices.into()), true, vortex_items, labels.no_vortices)
        .selected(selected.to_vec())
        .selection_change(block3d_action("setSelection", None))
        .build()
}

fn text_field(id: &str, label: impl Into<Label>, value: &str, field: &str) -> UiNode {
    UiNode::Field(UiFieldNode {
        presence: UiPresence::default(),
        id: id.into(),
        label: label.into(),
        child: Box::new(UiNode::Input(UiInputNode {
            presence: UiPresence::default(),
            id: format!("{id}.input"),
            input_kind: "text".into(),
            value: value.into(),
            placeholder: None,
            commit: Some("blur".into()),
            on_change: block3d_action("patchObjectKind", Some(json!({ "field": field }))),
            min: None,
            max: None,
            step: None,
            accept: None,
            menu: None,
        })),
        description: None,
        required: None,
        error: None,
        menu: None,
    })
}

fn build_inspection_tree(definition: &Block3dDefinition, active_representation_id: Option<&str>, labels: &Block3dLabels) -> UiNode {
    let representation_select = UiNode::Select(UiSelectNode {
        id: "block3d-play-inspector.representation".into(),
        value: active_representation_id.unwrap_or_default().into(),
        items: definition.representations.iter().map(|representation| UiSelectItem { value: representation.id.clone(), label: Label::data(representation.name.clone()) }).collect(),
        placeholder: None,
        on_change: block3d_action("setActiveRepresentation", None),
        presence: UiPresence::default(),
        menu: None,
    });
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "block3d-play-inspector".into(),
        label: labels.summary.into(),
        default_open: Some(true),
        presence: UiPresence::default(),
        fields: vec![
            text_field("block3d-play-inspector.name", labels.name, &definition.object_kind.name, "name"),
            text_field("block3d-play-inspector.label", labels.label, &definition.object_kind.label, "label"),
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "block3d-play-inspector.representation-field".into(),
                label: labels.representation.into(),
                child: Box::new(representation_select),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
            ui_inspector_readonly_field("block3d-play-inspector.vortex-count", labels.vortices, definition.vortices.len().to_string()),
        ],
    }])
}

fn render_world(definition: &Block3dDefinition, config: &Block3dConfig, window_id: &str) -> UiNode {
    let view = block3d_window_view(config, window_id);
    let visible = visible_representations(definition, &view);
    let scene = world3d_scene_extended(
        world_camera_json(definition, config),
        world_meshes_json(definition, &visible),
        world_instances_json(definition, &visible, &view),
        world_selection_json(config),
        Some(world_vortices_json(definition, config, &visible, &view)),
        None,
        None,
        None,
        None,
        Some(world_interaction_json(config, window_id)),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );
    build_world_3d_scene(BLOCK3D_PLAY_SURFACE_ID, BLOCK3D_PLAY_APP_ID, scene)
}

fn block3d_window_measures(definition: &Block3dDefinition, config: &Block3dConfig, window_id: &str, labels: &Block3dLabels) -> Vec<WindowMeasure> {
    let view = block3d_window_view(config, window_id);
    let visible_set: std::collections::HashSet<&str> = if view.representation_ids.is_empty() {
        definition.representations.iter().map(|r| r.id.as_str()).collect()
    } else {
        view.representation_ids.iter().map(|s| s.as_str()).collect()
    };
    let rep_toggles: Vec<WindowMeasure> = definition
        .representations
        .iter()
        .map(|representation| {
            WindowMeasure::Toggle {
                id: format!("block3d-rep-{}", representation.id),
                icon_id: "box".into(),
                label: Some(representation.name.clone()),
                pressed: visible_set.contains(representation.id.as_str()),
                text: None,
                on_change: block3d_action(
                    "toggleWindowRepresentation",
                    Some(json!({ "windowId": window_id, "representationId": representation.id, "visible": !visible_set.contains(representation.id.as_str()) })),
                ),
            }
        })
        .collect();
    let mut quick_items = vec![MeasureSelectItem { id: "all".into(), value: String::new(), label: labels.show_all.as_str().to_string() }];
    quick_items.extend(definition.representations.iter().map(|representation| MeasureSelectItem {
        id: representation.id.clone(),
        value: representation.id.clone(),
        label: representation.name.clone(),
    }));
    let quick_value = view.representation_ids.first().cloned().unwrap_or_default();
    vec![
        WindowMeasure::measure_group(
            "block3d-representations",
            labels.representations.as_str(),
            rep_toggles,
        ),
        WindowMeasure::Select {
            id: "block3d-rep-quick".into(),
            label: Some(labels.representation.as_str().to_string()),
            value: quick_value,
            items: quick_items,
            on_change: block3d_action("setWindowRepresentations", Some(json!({ "windowId": window_id }))),
        },
        WindowMeasure::Select {
            id: "block3d-arrangement".into(),
            label: Some(labels.arrangement.as_str().to_string()),
            value: view.arrangement.clone(),
            items: vec![
                MeasureSelectItem { id: "overlap".into(), value: "overlap".into(), label: "Overlap".into() },
                MeasureSelectItem { id: "x".into(), value: "x".into(), label: "X".into() },
                MeasureSelectItem { id: "y".into(), value: "y".into(), label: "Y".into() },
                MeasureSelectItem { id: "z".into(), value: "z".into(), label: "Z".into() },
            ],
            on_change: block3d_action("setWindowArrangement", Some(json!({ "windowId": window_id }))),
        },
        WindowMeasure::Slider {
            id: "block3d-spacing".into(),
            label: Some(labels.spacing.as_str().to_string()),
            value: view.spacing,
            min: 0.0,
            max: 40.0,
            step: Some(0.5),
            ready: None,
            loading: None,
            waiting: None,
            disabled: None,
            reveal: None,
            on_change: block3d_action("setWindowSpacing", Some(json!({ "windowId": window_id }))),
        },
        WindowMeasure::Group {
            id: "block3d-brush-options".into(),
            label: labels.brush.as_str().to_string(),
            default_open: Some(true),
            active_utility_id: Some(BLOCK3D_UTILITY_SURFACE_BRUSH.into()),
            value: None,
            min: None,
            max: None,
            step: None,
            ready: None,
            loading: None,
            waiting: None,
            on_change: None,
            children: vec![
                WindowMeasure::Select {
                    id: "block3d-brush-kind".into(),
                    label: Some(labels.vortex_kinds.as_str().to_string()),
                    value: resolve_brush_vortex_kind_id(definition, config),
                    items: definition
                        .vortex_kinds
                        .iter()
                        .map(|kind| MeasureSelectItem { id: kind.id.clone(), value: kind.id.clone(), label: kind.label.clone() })
                        .collect(),
                    on_change: block3d_action("setBrushVortexKind", None),
                },
                WindowMeasure::Slider {
                    id: "block3d-brush-radius".into(),
                    label: Some(labels.brush_radius.as_str().to_string()),
                    value: config.brush_radius,
                    min: 0.05,
                    max: 2.0,
                    step: Some(0.05),
                    ready: None,
                    loading: None,
                    waiting: None,
                    disabled: None,
                    reveal: None,
                    on_change: block3d_action("setBrushRadius", None),
                },
                WindowMeasure::Toggle {
                    id: "block3d-brush-flip".into(),
                    icon_id: "flip-vertical".into(),
                    label: Some(labels.flip_normal.as_str().to_string()),
                    pressed: config.brush_flip,
                    text: None,
                    on_change: block3d_action("setBrushFlip", Some(json!({ "flip": !config.brush_flip }))),
                },
            ],
        },
    ]
}
//#endregion 🔖️Panels

//#region 🔖️Block3dPlayApp
/// 🧪️ B1: unit struct — every former `RefCell` field now lives in `block_3d_engine::Block3dConfig`
/// (see `DocumentApp::Config`), written through `block_3d_op::Block3dConfigOperation`s.
#[derive(Default)]
pub struct Block3dPlayApp;

impl DocumentApp for Block3dPlayApp {
    type Projection = Block3dDefinition;
    type Operation = Block3dOperation;
    type Config = Block3dConfig;
    type ConfigOperation = Block3dConfigOperation;
    type Command = Block3dCommand;

    fn app_id(&self) -> &str {
        BLOCK3D_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        BLOCK_3D_SCHEMA
    }

    fn initial_projection(&self) -> Block3dDefinition {
        block_3d_engine::empty_block3d_definition()
    }

    fn io(&self) -> Option<semio_framework_plugin::AppIo> {
        Some(block_3d_engine::block3d_io())
    }

    /// 🏷️ Maps each `Block3dCommand` variant back to the action id it was declared under in
    /// `create_block3d_app` — used for command-log labeling and the registry's View-kind discipline
    /// check.
    fn command_id(&self, command: &Block3dCommand) -> &str {
        match command {
            Block3dCommand::PatchObjectKind { .. } => "patchObjectKind",
            Block3dCommand::AddRepresentation => "addRepresentation",
            Block3dCommand::RemoveRepresentation { .. } => "removeRepresentation",
            Block3dCommand::AddVortexKind => "addVortexKind",
            Block3dCommand::RemoveVortexKind { .. } => "removeVortexKind",
            Block3dCommand::AddVortex => "addVortex",
            Block3dCommand::RemoveVortex { .. } => "removeVortex",
            Block3dCommand::SetActiveExample { .. } => "setActiveExample",
            Block3dCommand::Edit { .. } => "edit",
            Block3dCommand::SetSelection { .. } => "setSelection",
            Block3dCommand::SetActiveRepresentation { .. } => "setActiveRepresentation",
            Block3dCommand::SetWindowRepresentations { .. } => "setWindowRepresentations",
            Block3dCommand::ToggleWindowRepresentation { .. } => "toggleWindowRepresentation",
            Block3dCommand::SetWindowArrangement { .. } => "setWindowArrangement",
            Block3dCommand::SetWindowSpacing { .. } => "setWindowSpacing",
            Block3dCommand::SetActiveUtility { .. } => "setActiveUtility",
            Block3dCommand::SetBrushVortexKind { .. } => "setBrushVortexKind",
            Block3dCommand::SetBrushRadius { .. } => "setBrushRadius",
            Block3dCommand::SetBrushFlip { .. } => "setBrushFlip",
            Block3dCommand::HoverSurface { .. } => "worldSurfaceHover",
            Block3dCommand::LeaveSurface => "worldSurfaceLeave",
            Block3dCommand::PlaceVortex { .. } => "worldSurfacePlace",
            Block3dCommand::SetCamera { .. } => "setCamera",
            Block3dCommand::SelectVortex { .. } => "selectVortex",
            Block3dCommand::HoverVortex { .. } => "hoverVortex",
            Block3dCommand::PatchRepresentation { .. } => "patchRepresentation",
        }
    }


    fn command_from_action(&self, action: &str, args: Option<&Value>) -> Result<Self::Command, Fault> {
        let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_str).map(str::to_string);
        let str_vec_field = |key: &str| -> Vec<String> {
            args.and_then(|value| value.get(key))
                .and_then(|value| value.as_array())
                .map(|rows| rows.iter().filter_map(|row| row.as_str().map(str::to_string)).collect())
                .unwrap_or_default()
        };
        match action {
            "patchObjectKind" => Ok(Block3dCommand::PatchObjectKind { field: str_field("field").unwrap_or_default(), value: str_field("value").unwrap_or_default() }),
            "addRepresentation" => Ok(Block3dCommand::AddRepresentation),
            "removeRepresentation" => Ok(Block3dCommand::RemoveRepresentation { id: str_field("id").unwrap_or_default() }),
            "addVortexKind" => Ok(Block3dCommand::AddVortexKind),
            "removeVortexKind" => Ok(Block3dCommand::RemoveVortexKind { id: str_field("id").unwrap_or_default() }),
            "addVortex" => Ok(Block3dCommand::AddVortex),
            "removeVortex" => Ok(Block3dCommand::RemoveVortex { id: str_field("id").unwrap_or_default() }),
            "setActiveExample" => Ok(Block3dCommand::SetActiveExample { id: str_field("exampleId").or_else(|| str_field("id")).unwrap_or_default() }),
            "edit" => Ok(Block3dCommand::Edit { text: str_field("text").unwrap_or_default() }),
            "setSelection" => Ok(Block3dCommand::SetSelection { ids: str_vec_field("ids") }),
            "setActiveRepresentation" => Ok(Block3dCommand::SetActiveRepresentation { representation_id: str_field("representationId").or_else(|| str_field("representation_id")) }),
            "setWindowRepresentations" => {
                let rep = str_field("value").or_else(|| str_field("representationId"));
                let representation_ids = rep.filter(|id| !id.is_empty()).map(|id| vec![id]).unwrap_or_default();
                Ok(Block3dCommand::SetWindowRepresentations { window_id: window_id_from_args(args), representation_ids })
            }
            "toggleWindowRepresentation" => Ok(Block3dCommand::ToggleWindowRepresentation {
                window_id: window_id_from_args(args),
                representation_id: str_field("representationId").unwrap_or_default(),
                visible: args.and_then(|value| value.get("visible")).and_then(Value::as_bool).unwrap_or(true),
            }),
            "setWindowArrangement" => Ok(Block3dCommand::SetWindowArrangement { window_id: window_id_from_args(args), arrangement: str_field("value").unwrap_or_else(|| "overlap".into()) }),
            "setWindowSpacing" => Ok(Block3dCommand::SetWindowSpacing {
                window_id: window_id_from_args(args),
                spacing: args.and_then(|value| value.get("value")).and_then(Value::as_f64).unwrap_or(8.0),
            }),
            "setActiveUtility" => Ok(Block3dCommand::SetActiveUtility {
                window_id: window_id_from_args(args),
                utility_id: str_field("utilityId").unwrap_or_else(|| BLOCK3D_UTILITY_SELECT.into()),
            }),
            "setBrushVortexKind" => Ok(Block3dCommand::SetBrushVortexKind { vortex_kind_id: str_field("value").or_else(|| str_field("vortexKindId")) }),
            "setBrushRadius" => Ok(Block3dCommand::SetBrushRadius { radius: args.and_then(|value| value.get("value")).and_then(Value::as_f64).unwrap_or(0.3) }),
            "setBrushFlip" => Ok(Block3dCommand::SetBrushFlip { flip: args.and_then(|value| value.get("flip")).and_then(Value::as_bool).unwrap_or(false) }),
            "worldSurfaceHover" => Ok(Block3dCommand::HoverSurface {
                window_id: window_id_from_args(args),
                object_id: str_field("objectId").unwrap_or_default(),
                position: f64_vec3_field(args, "position").unwrap_or([0.0, 0.0, 0.0]),
                normal: f64_vec3_field(args, "normal").unwrap_or([0.0, 0.0, 1.0]),
            }),
            "worldSurfaceLeave" => Ok(Block3dCommand::LeaveSurface),
            "worldSurfacePlace" => Ok(Block3dCommand::PlaceVortex {
                window_id: window_id_from_args(args),
                object_id: str_field("objectId").unwrap_or_default(),
                position: f64_vec3_field(args, "position").unwrap_or([0.0, 0.0, 0.0]),
                normal: f64_vec3_field(args, "normal").unwrap_or([0.0, 0.0, 1.0]),
            }),
            "selectVortex" => Ok(Block3dCommand::SelectVortex { full_id: str_field("fullId").unwrap_or_default(), merge: args.and_then(|value| value.get("merge")).and_then(Value::as_bool).unwrap_or(false) }),
            "hoverVortex" => Ok(Block3dCommand::HoverVortex { full_id: str_field("fullId") }),
            "patchRepresentation" => Ok(Block3dCommand::PatchRepresentation { id: str_field("id").unwrap_or_default(), field: str_field("field").unwrap_or_default(), value: str_field("value").unwrap_or_default() }),
            other => Err(Fault::new(
                FaultOrigin::App,
                FaultCode::new("block3d.unhandled-action"),
                format!("action '{other}' is not a framework-reserved action — app actions are dispatched exclusively through the typed command channel"),
            )),
        }
    }

    fn handle(&self, command: &Block3dCommand, doc: &DocumentView<'_, Block3dDefinition>, cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dOperation, Block3dConfigOperation>, Fault> {
        match command {
            Block3dCommand::PatchObjectKind { field, value } => {
                let mut object_kind = doc.projection.object_kind.clone();
                match field.as_str() {
                    "name" => object_kind.name = value.clone(),
                    "label" => object_kind.label = value.clone(),
                    "variant" => object_kind.variant = if value.is_empty() { None } else { Some(value.clone()) },
                    "description" => object_kind.description = value.clone(),
                    _ => return Ok(Emit::default()),
                }
                Ok(Emit::operations(vec![Block3dOperation::SetObjectKind { object_kind }]))
            }
            Block3dCommand::AddRepresentation => {
                let id = block_3d_engine::next_id(doc.projection.representations.iter().map(|representation| representation.id.as_str()), "representation-");
                let representation = BlockRepresentation { id: id.clone(), name: id, mesh_url: None, tags: Vec::new(), lod: None, description: String::new(), attributes: Vec::new() };
                Ok(Emit::operations(vec![Block3dOperation::SetRepresentation { index: doc.projection.representations.len(), representation }]))
            }
            Block3dCommand::RemoveRepresentation { id } => Ok(Emit::operations(vec![Block3dOperation::RemoveRepresentation { id: id.clone() }])),
            Block3dCommand::AddVortexKind => {
                let id = block_3d_engine::next_id(doc.projection.vortex_kinds.iter().map(|kind| kind.id.as_str()), "vortex-kind-");
                let vortex_kind = Block3dVortexKind { id: id.clone(), name: id.clone(), label: id, color: "#888888".into(), default_cable_kind: "cable.link".into() };
                Ok(Emit::operations(vec![Block3dOperation::SetVortexKind { index: doc.projection.vortex_kinds.len(), vortex_kind }]))
            }
            Block3dCommand::RemoveVortexKind { id } => Ok(Emit::operations(vec![Block3dOperation::RemoveVortexKind { id: id.clone() }])),
            Block3dCommand::AddVortex => {
                let Some(vortex_kind_id) = doc.projection.vortex_kinds.first().map(|kind| kind.id.clone()) else { return Ok(Emit::default()); };
                let id = block_3d_engine::next_id(doc.projection.vortices.iter().map(|vortex| vortex.id.as_str()), "vortex-");
                let vortex = Block3dVortexTemplate { id, vortex_kind: vortex_kind_id, position: [0.0, 0.0, 0.0], direction: [0.0, 0.0, 1.0], radius: 0.3, label: None };
                Ok(Emit::operations(vec![Block3dOperation::SetVortex { index: doc.projection.vortices.len(), vortex }]))
            }
            Block3dCommand::RemoveVortex { id } => Ok(Emit::operations(vec![Block3dOperation::RemoveVortex { id: id.clone() }])),
            Block3dCommand::SetActiveExample { id } => {
                let example = match id.as_str() {
                    BLOCK3D_EXAMPLE_CAPSULE => block_3d_dsl::parse_dsl(block_3d_dsl::BLOCK3D_NAKAGIN_CAPSULE_EXAMPLE_TEXT).ok(),
                    BLOCK3D_EXAMPLE_FOREST_LEFT => block_3d_dsl::parse_dsl(block_3d_dsl::BLOCK3D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).ok(),
                    _ => None,
                };
                match example {
                    Some(document) => Ok(Emit::operations(vec![Block3dOperation::SetDocument { document }])),
                    None => Ok(Emit::default()),
                }
            }
            Block3dCommand::Edit { text } => match serde_json::from_str::<Block3dDefinition>(text) {
                Ok(document) if &document != doc.projection => Ok(Emit::operations(vec![Block3dOperation::SetDocument { document }])),
                _ => Ok(Emit::default()),
            },
            Block3dCommand::SetSelection { ids } => Ok(Emit::config(vec![Block3dConfigOperation::SetSelection { ids: ids.clone() }])),
            Block3dCommand::SetActiveRepresentation { representation_id } => Ok(Emit::config(vec![Block3dConfigOperation::SetActiveRepresentation { representation_id: representation_id.clone() }])),
            Block3dCommand::SetWindowRepresentations { window_id, representation_ids } => Ok(Emit::config(vec![Block3dConfigOperation::SetWindowRepresentations { window_id: window_id.clone(), representation_ids: representation_ids.clone() }])),
            Block3dCommand::ToggleWindowRepresentation { window_id, representation_id, visible } => Ok(Emit::config(vec![Block3dConfigOperation::ToggleWindowRepresentation { window_id: window_id.clone(), representation_id: representation_id.clone(), visible: *visible }])),
            Block3dCommand::SetWindowArrangement { window_id, arrangement } => Ok(Emit::config(vec![Block3dConfigOperation::SetWindowArrangement { window_id: window_id.clone(), arrangement: arrangement.clone() }])),
            Block3dCommand::SetWindowSpacing { window_id, spacing } => Ok(Emit::config(vec![Block3dConfigOperation::SetWindowSpacing { window_id: window_id.clone(), spacing: *spacing }])),
            Block3dCommand::SetActiveUtility { window_id, utility_id } => Ok(Emit::config(vec![Block3dConfigOperation::SetActiveUtility { window_id: window_id.clone(), utility_id: utility_id.clone() }])),
            Block3dCommand::SetBrushVortexKind { vortex_kind_id } => Ok(Emit::config(vec![Block3dConfigOperation::SetBrushVortexKind { vortex_kind_id: vortex_kind_id.clone() }])),
            Block3dCommand::SetBrushRadius { radius } => Ok(Emit::config(vec![Block3dConfigOperation::SetBrushRadius { radius: *radius }])),
            Block3dCommand::SetBrushFlip { flip } => Ok(Emit::config(vec![Block3dConfigOperation::SetBrushFlip { flip: *flip }])),
            Block3dCommand::HoverSurface { position, normal, .. } => Ok(Emit::config(vec![Block3dConfigOperation::SetBrushPreview { preview: Some(Block3dBrushPreview { position: *position, direction: *normal }) }])),
            Block3dCommand::LeaveSurface => Ok(Emit::config(vec![Block3dConfigOperation::SetBrushPreview { preview: None }])),
            Block3dCommand::PlaceVortex { window_id, object_id, position, normal } => {
                let view = block3d_window_view(cfg.projection, window_id);
                let offset = instance_offset_for_representation(doc.projection, &view, object_id);
                let local_position = [position[0] - offset[0], position[1] - offset[1], position[2] - offset[2]];
                let direction = if cfg.projection.brush_flip { [-normal[0], -normal[1], -normal[2]] } else { *normal };
                let vortex_kind_id = resolve_brush_vortex_kind_id(doc.projection, cfg.projection);
                let mut operations = Vec::new();
                if doc.projection.vortex_kinds.is_empty() {
                    operations.push(Block3dOperation::SetVortexKind { index: 0, vortex_kind: default_vortex_kind() });
                }
                let id = next_id(doc.projection.vortices.iter().map(|vortex| vortex.id.as_str()), "vortex-");
                operations.push(Block3dOperation::SetVortex {
                    index: doc.projection.vortices.len(),
                    vortex: Block3dVortexTemplate { id, vortex_kind: vortex_kind_id, position: local_position, direction, radius: cfg.projection.brush_radius, label: None },
                });
                Ok(Emit { document_operations: operations, config_operations: vec![Block3dConfigOperation::SetBrushPreview { preview: None }], description: None, ..Default::default() })
            }
            Block3dCommand::SetCamera { camera } => Ok(Emit::config(vec![Block3dConfigOperation::SetCamera { camera: camera.clone() }])),
            Block3dCommand::SelectVortex { full_id, merge } => {
                let local = full_id.split_once(':').map(|(_, tail)| tail).unwrap_or(full_id.as_str());
                let id = format!("vortex:{local}");
                let mut ids = if *merge { cfg.projection.selected_ids.clone() } else { Vec::new() };
                if !ids.contains(&id) {
                    ids.push(id);
                }
                Ok(Emit::config(vec![Block3dConfigOperation::SetSelection { ids }]))
            }
            Block3dCommand::HoverVortex { full_id } => Ok(Emit::config(vec![Block3dConfigOperation::SetHoveredVortexFullId { full_id: full_id.clone() }])),
            Block3dCommand::PatchRepresentation { id, field, value } => {
                let Some(index) = doc.projection.representations.iter().position(|representation| representation.id == *id) else {
                    return Ok(Emit::default());
                };
                let mut representation = doc.projection.representations[index].clone();
                match field.as_str() {
                    "name" => representation.name = value.clone(),
                    "meshUrl" | "mesh_url" => representation.mesh_url = if value.is_empty() { None } else { Some(value.clone()) },
                    _ => return Ok(Emit::default()),
                }
                Ok(Emit::operations(vec![Block3dOperation::SetRepresentation { index, representation }]))
            }
        }
    }

    fn window_measures(&self, doc: &DocumentView<'_, Block3dDefinition>, cfg: &ConfigView<'_, Block3dConfig>) -> std::collections::HashMap<String, Vec<WindowMeasure>> {
        let labels = block3d_labels(cfg.projection);
        let mut measures = std::collections::HashMap::new();
        measures.insert(BLOCK3D_DEFAULT_WINDOW_ID.into(), block3d_window_measures(doc.projection, cfg.projection, BLOCK3D_DEFAULT_WINDOW_ID, labels));
        measures
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, Block3dDefinition>, cfg: &ConfigView<'_, Block3dConfig>) -> UiNode {
        let labels = block3d_labels(cfg.projection);
        let active_representation_id = cfg.projection.active_representation_id.as_deref();
        let (base_body, window_id) = block3d_resolve_world_body(body_key);
        match base_body {
            BLOCK3D_BODY_WORLD => render_world(doc.projection, cfg.projection, &window_id),
            BLOCK3D_BODY_DOCUMENT => build_document_tree(doc.projection, &cfg.projection.selected_ids, labels),
            BLOCK3D_BODY_INSPECTOR => build_inspection_tree(doc.projection, active_representation_id, labels),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    /// 🌉️ The flagship seam: `puzzle3d_catalog_fragment`'s first real caller. Wraps the block-3d
    /// document's puzzle3d-shaped catalog fragment (`objectKinds`/`vortexKinds`/`cableKinds`/
    /// `attractionKinds`/`kindCompatibility`) as a `kit.catalog`-schema `Media` value for the
    /// `"catalog:out"` port declared in `block_3d_engine::block3d_io`. `wanted_tags` should come from
    /// `cfg.wanted_tags` (`Block3dConfig`) but `DocumentApp::export_media`'s landed signature doesn't
    /// thread `ConfigView` through yet — see `Block3dConfig::wanted_tags`'s doc — so this always
    /// resolves the active representation with an empty (all-tags) filter until that lands. Falls
    /// through to the default whole-document pack export for every other port (`"document:out"`).
    fn export_media(&self, port: &str, doc: &DocumentView<'_, Block3dDefinition>) -> Result<Media, MediaError> {
        if port != "catalog:out" {
            // 🌉️ Reimplements `DocumentApp::export_media`'s default `"document:out"` behavior
            // verbatim — overriding the trait method forfeits the ability to delegate back to its
            // own default body, so the whole-document pack export is duplicated here rather than
            // left unreachable for this app.
            if port != "document:out" {
                return Err(MediaError::NotImplemented);
            }
            let media_type = self.io().map(|io| io.document_media_type).unwrap_or(MediaType { class: MediaClass::Kit, form: MediaForm::Type });
            let bytes = store::DocumentPack::encode_pack(doc.projection);
            return Ok(Media { media_type, payload: MediaPayload::Structured { schema: self.document_schema().to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } });
        }
        let fragment = block_3d_engine::puzzle3d_catalog_fragment(doc.projection, &[]);
        Ok(Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: MediaPayload::Structured { schema: KIT_CATALOG_ARTIFACT_ID.into(), json: fragment.to_string() } })
    }
}
//#endregion 🔖️Block3dPlayApp

//#region 🔖️Manifest
pub fn create_block3d_app() -> App {
    App::from_builder(
        App::builder(BLOCK3D_PLAY_APP_ID, LocalizedLabel::native("Block 3D", "Block 3D"))
            .document(["semio", "block", "3d"])
            .artifact_kind(ArtifactKindSpec {
                id: "3d.block".into(),
                name: "Object Kind".into(),
                source_format: BLOCK_3D_SCHEMA.into(),
                component_kind: "block3d".into(),
                dimension: "3d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                schema: BLOCK_3D_SCHEMA.into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            // 🗂️ The puzzle3d catalog artifact this app's new `"catalog:out"` port produces — see
            // `block_3d_engine::block3d_io`/`Block3dPlayApp::export_media`. `source_format`/`schema`
            // both pin the `kit.catalog` JSON fragment shape `puzzle3d_catalog_fragment` builds.
            .artifact_kind(ArtifactKindSpec {
                id: KIT_CATALOG_ARTIFACT_ID.into(),
                name: "Kit Catalog".into(),
                source_format: KIT_CATALOG_ARTIFACT_ID.into(),
                component_kind: "kit-catalog".into(),
                dimension: "3d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                schema: KIT_CATALOG_ARTIFACT_ID.into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            .icon_id("box")
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .default_mode_id("edit")
            .window_kind(BLOCK3D_WINDOW_WORLD, LocalizedLabel::native("Object Kind", "Objektart"), BLOCK3D_BODY_WORLD, SurfaceKind::World3d, "box")
            .utility(UtilityDefinition::new(BLOCK3D_UTILITY_SELECT, LocalizedLabel::native("Select", "Auswählen"), "mouse-pointer"))
            .utility(UtilityDefinition::new(BLOCK3D_UTILITY_SURFACE_BRUSH, LocalizedLabel::native("Surface brush", "Flächenpinsel"), "paintbrush"))
            .window_kind_utilities(BLOCK3D_WINDOW_WORLD, vec![BLOCK3D_UTILITY_SELECT.into(), BLOCK3D_UTILITY_SURFACE_BRUSH.into()])
            .panel_tab("framework.panel.document", LocalizedLabel::native("Document", "Dokument"), PanelGroup::Workbench, BLOCK3D_BODY_DOCUMENT)
            .panel_tab("framework.panel.inspection", LocalizedLabel::native("Inspection", "Inspektion"), PanelGroup::Details, BLOCK3D_BODY_INSPECTOR)
            .operation("patchObjectKind", LocalizedLabel::native("Patch Object Kind", "Objektart bearbeiten"))
            .operation("addRepresentation", LocalizedLabel::native("Add Representation", "Darstellung hinzufügen"))
            .operation("removeRepresentation", LocalizedLabel::native("Remove Representation", "Darstellung entfernen"))
            .operation("addVortexKind", LocalizedLabel::native("Add Vortex Kind", "Wirbelart hinzufügen"))
            .operation("removeVortexKind", LocalizedLabel::native("Remove Vortex Kind", "Wirbelart entfernen"))
            .operation("addVortex", LocalizedLabel::native("Add Vortex", "Wirbel hinzufügen"))
            .operation("removeVortex", LocalizedLabel::native("Remove Vortex", "Wirbel entfernen"))
            .operation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .operation("edit", LocalizedLabel::native("Edit", "Bearbeiten"))
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
            .operation("worldSurfacePlace", LocalizedLabel::native("Place Vortex", "Wirbel platzieren"))
            .view_action("selectVortex", LocalizedLabel::native("Select Vortex", "Wirbel auswählen"))
            .view_action("hoverVortex", LocalizedLabel::native("Hover Vortex", "Wirbel hovern"))
            .operation("patchRepresentation", LocalizedLabel::native("Patch Representation", "Darstellung bearbeiten"))
            .io(block_3d_engine::block3d_io()),
    )
    .example(
        BLOCK3D_EXAMPLE_CAPSULE,
        LocalizedLabel::native("Nakagin Capsule", "Nakagin Capsule"),
        serde_json::to_string(&block_3d_dsl::parse_dsl(block_3d_dsl::BLOCK3D_NAKAGIN_CAPSULE_EXAMPLE_TEXT).unwrap_or_default()).unwrap_or_default(),
        "building",
    )
    .example(
        BLOCK3D_EXAMPLE_FOREST_LEFT,
        LocalizedLabel::native("Hexagonal Cut Concrete Forest Left", "Sechseckig geschnittener Betonwald links"),
        serde_json::to_string(&block_3d_dsl::parse_dsl(block_3d_dsl::BLOCK3D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).unwrap_or_default()).unwrap_or_default(),
        "list-tree",
    )
    .workflow("block3d", "Block 3D", "model")
}
//#endregion 🔖️Manifest

pub fn register_block3d_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<Block3dPlayApp>(BLOCK_3D_SCHEMA);
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, PluginApp, ViewState};

    fn new_app() -> semio_framework_plugin::VcsDocumentApp<Block3dPlayApp> {
        testkit::new_app::<Block3dPlayApp>()
    }

    #[test]
    fn renders_document_tree_and_inspector() {
        let mut app = new_app();
        let node = app.render(BLOCK3D_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Representations"));
        let inspector = app.render(BLOCK3D_BODY_INSPECTOR, None, &ViewState::default()).expect("render");
        let inspector_json = serde_json::to_string(&inspector).unwrap();
        assert!(inspector_json.contains("\"type\":\"tree\""), "inspection body must be a tree like document");
        assert!(inspector_json.contains("Name"));
        assert!(inspector_json.contains("Vortices"));
        assert!(!inspector_json.contains("\"type\":\"stack\""), "inspection body must not be a free-form stack");
    }

    #[test]
    fn add_representation_then_set_active_then_render_world_shows_mesh() {
        let mut app = new_app();
        app.dispatch_typed(Block3dCommand::AddRepresentation, &testkit::meta("local")).expect("add representation");
        let representation_id = app.projection().expect("projection").representations[0].id.clone();
        app.dispatch_typed(Block3dCommand::SetActiveRepresentation { representation_id: Some(representation_id) }, &testkit::meta("local")).expect("set active");
        let node = app.render(BLOCK3D_BODY_WORLD, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"type\":\"componentScene\""), "world body must render a 3d scene");
    }

    #[test]
    fn add_vortex_kind_then_add_vortex_then_remove_round_trips() {
        let mut app = new_app();
        app.dispatch_typed(Block3dCommand::AddVortexKind, &testkit::meta("local")).expect("add vortex kind");
        app.dispatch_typed(Block3dCommand::AddVortex, &testkit::meta("local")).expect("add vortex");
        let projection = app.projection().expect("projection");
        assert_eq!(projection.vortices.len(), 1);
        let vortex_id = projection.vortices[0].id.clone();
        app.dispatch_typed(Block3dCommand::RemoveVortex { id: vortex_id }, &testkit::meta("local")).expect("remove vortex");
        assert_eq!(app.projection().expect("projection").vortices.len(), 0);
    }

    #[test]
    fn set_active_example_loads_capsule_fixture() {
        let mut app = new_app();
        app.dispatch_typed(Block3dCommand::SetActiveExample { id: BLOCK3D_EXAMPLE_CAPSULE.into() }, &testkit::meta("local")).expect("load example");
        let projection = app.projection().expect("projection");
        assert_eq!(projection.object_kind.id, "Capsule J");
        assert_eq!(projection.representations.len(), 2);
    }

    #[test]
    fn undo_redo_round_trips_through_the_wrapper() {
        let mut app = new_app();
        app.dispatch_typed(Block3dCommand::AddVortexKind, &testkit::meta("local")).expect("add vortex kind");
        assert_eq!(app.projection().expect("projection").vortex_kinds.len(), 1);
        app.handle_action("undo", None, &testkit::meta("local")).expect("undo");
        assert_eq!(app.projection().expect("projection").vortex_kinds.len(), 0);
        app.handle_action("redo", None, &testkit::meta("local")).expect("redo");
        assert_eq!(app.projection().expect("projection").vortex_kinds.len(), 1);
    }

    #[test]
    fn set_selection_writes_config_not_document() {
        let mut app = new_app();
        let result = app.dispatch_typed(Block3dCommand::SetSelection { ids: vec!["representation:r0".into()] }, &testkit::meta("local")).expect("select");
        assert!(result.operations.is_empty(), "setSelection is config-only and must emit no document operations");
    }

    /// 🌉️ `puzzle3d_catalog_fragment`'s new caller round-trips through the `"catalog:out"` media port.
    #[test]
    fn export_media_catalog_out_wraps_the_puzzle3d_fragment() {
        let mut app = new_app();
        app.dispatch_typed(Block3dCommand::SetActiveExample { id: BLOCK3D_EXAMPLE_CAPSULE.into() }, &testkit::meta("local")).expect("load example");
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
    fn block3d_io_is_wired_into_the_manifest() {
        let definition = create_block3d_app().definition;
        assert!(definition.artifact_kinds.iter().any(|kind| kind.id == "kit.catalog"));
    }

    #[test]
    fn place_vortex_on_surface_auto_creates_kind_and_vortex() {
        let mut app = new_app();
        app.dispatch_typed(Block3dCommand::SetActiveExample { id: BLOCK3D_EXAMPLE_CAPSULE.into() }, &testkit::meta("local")).expect("load example");
        app.dispatch_typed(
            Block3dCommand::PlaceVortex {
                window_id: BLOCK3D_DEFAULT_WINDOW_ID.into(),
                object_id: "r0".into(),
                position: [0.5, 0.0, 1.0],
                normal: [0.0, 1.0, 0.0],
            },
            &testkit::meta("local"),
        )
        .expect("place vortex");
        let projection = app.projection().expect("projection");
        assert!(!projection.vortex_kinds.is_empty());
        assert_eq!(projection.vortices.len(), 2);
    }

    #[test]
    fn command_from_action_bridges_set_active_example() {
        let app = Block3dPlayApp;
        assert!(matches!(app.command_from_action("setActiveExample", Some(&serde_json::json!({ "exampleId": "capsule" }))), Ok(Block3dCommand::SetActiveExample { id }) if id == "capsule"));
    }
}
//#endregion 🧪️Tests
