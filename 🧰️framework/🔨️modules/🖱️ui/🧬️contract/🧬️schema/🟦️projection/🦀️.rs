//! 🧬️ Versioned owned semantic UI TypeScript projection.
//#region 🔖️TypescriptProjection

use std::collections::HashSet;

/// 🧬️ One versioned semantic UI wire type and its owned TypeScript projection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SchemaMetadata {
    pub name: &'static str,
    pub version: u16,
    pub typescript: &'static str,
}

#[path = "📐️schema-rows-absolute-layout-to-extension-props/🦀️.rs"]
mod schema_rows_absolute_layout_to_extension_props;
use schema_rows_absolute_layout_to_extension_props::{
    ABSOLUTE_LAYOUT, ACCESSIBILITY_SPEC, ACTION_BINDING, ACTION_ID, ACTIVITY, ALIGN, ANCHOR, AXIS,
    BUILT_NODE, BUTTON_PROPS, COMPONENT, CONTAINER_PROPS, CONTAINER_ROLE, DENSITY,
    DROP_OVERLAY_SPEC, EDGE_SPACE, EMPHASIS, EXTENSION_PROPS,
};

#[path = "🧱️schema-rows-grid-layout-to-peer-mark/🦀️.rs"]
mod schema_rows_grid_layout_to_peer_mark;
use schema_rows_grid_layout_to_peer_mark::{
    GRID_LAYOUT, GRID_TRACK, ICON_SELECT_PROPS, IMAGE_PROPS, INPUT_KIND, INPUT_PROPS, JUSTIFY,
    KEY_VALUE_ENTRY, KEY_VALUE_LIST_PROPS, LABEL, LAYOUT_SPEC, LEAF_LAYOUT, LIVENESS, MENU_REF,
    NUMBER_STEPPER_PROPS, OVERLAY_LAYOUT, OWN_PRESENCE, PATCH_REJECTION, PEER_MARK,
};

#[path = "🧩️schema-rows-presence-update-to-slider-appearance/🦀️.rs"]
mod schema_rows_presence_update_to_slider_appearance;
use schema_rows_presence_update_to_slider_appearance::{
    PRESENCE_UPDATE, PROGRESS_PROPS, QUOTA_KIND, RING_PROPS, ROW_ACTION, ROW_ACTION_PLACEMENT,
    ROW_TARGET, SCROLL_AXES, SCROLL_LAYOUT, SELECT_APPEARANCE, SELECT_ITEM, SELECT_PROPS,
    SEPARATOR_PROPS, SIZE_TOKEN, SIZING, SLIDER_APPEARANCE,
};

#[path = "🎨️schema-rows-slider-props-to-trigger/🦀️.rs"]
mod schema_rows_slider_props_to_trigger;
use schema_rows_slider_props_to_trigger::{
    SLIDER_PROPS, SPACE_TOKEN, STACK_LAYOUT, STYLE_SPEC, SURFACE_DOC, SURFACE_ID, SURFACE_KIND,
    SURFACE_PROPS, TABLE_PROPS, TABLE_ROW_PROPS, TEXT_PROPS, TOGGLE_APPEARANCE, TOGGLE_PROPS, TONE,
    TRANSITION_HINT, TREE_ITEM_PROPS, TREE_PRESENTATION, TREE_PROPS, TREE_SECTION_PROPS,
    TREE_WINDOW, TREE_WINDOW_ROW_EXTENT, TRIGGER,
};

#[path = "🧮️schema-rows-ui-contract-violation-to-window-stack-corner/🦀️.rs"]
mod schema_rows_ui_contract_violation_to_window_stack_corner;
use schema_rows_ui_contract_violation_to_window_stack_corner::{
    UI_CONTRACT_VIOLATION, UI_DOCUMENT_LIMITS, UI_INTENT, UI_NODE_ID, UI_NODE_RECORD,
    UI_NUMBER_BOUND, UI_NUMBER_LIMITS, UI_NUMBER_SCALE, UI_PATCH, UI_PATCH_OP, UI_REVISION,
    UI_SNAPSHOT, UI_VALUE, VARIANT, WINDOW_LAYOUT, WINDOW_LAYOUT_NODE, WINDOW_STACK_CORNER,
};

pub const TYPES: &[SchemaMetadata] = &[
    ABSOLUTE_LAYOUT,
    ACCESSIBILITY_SPEC,
    ACTION_BINDING,
    ACTION_ID,
    ACTIVITY,
    ALIGN,
    ANCHOR,
    AXIS,
    BUILT_NODE,
    BUTTON_PROPS,
    COMPONENT,
    CONTAINER_PROPS,
    CONTAINER_ROLE,
    DENSITY,
    DROP_OVERLAY_SPEC,
    EDGE_SPACE,
    EMPHASIS,
    EXTENSION_PROPS,
    GRID_LAYOUT,
    GRID_TRACK,
    ICON_SELECT_PROPS,
    IMAGE_PROPS,
    INPUT_KIND,
    INPUT_PROPS,
    JUSTIFY,
    KEY_VALUE_ENTRY,
    KEY_VALUE_LIST_PROPS,
    LABEL,
    LAYOUT_SPEC,
    LEAF_LAYOUT,
    LIVENESS,
    MENU_REF,
    NUMBER_STEPPER_PROPS,
    OVERLAY_LAYOUT,
    OWN_PRESENCE,
    PATCH_REJECTION,
    PEER_MARK,
    PRESENCE_UPDATE,
    PROGRESS_PROPS,
    QUOTA_KIND,
    RING_PROPS,
    ROW_ACTION,
    ROW_ACTION_PLACEMENT,
    ROW_TARGET,
    SCROLL_AXES,
    SCROLL_LAYOUT,
    SELECT_APPEARANCE,
    SELECT_ITEM,
    SELECT_PROPS,
    SEPARATOR_PROPS,
    SIZE_TOKEN,
    SIZING,
    SLIDER_APPEARANCE,
    SLIDER_PROPS,
    SPACE_TOKEN,
    STACK_LAYOUT,
    STYLE_SPEC,
    SURFACE_DOC,
    SURFACE_ID,
    SURFACE_KIND,
    SURFACE_PROPS,
    TABLE_PROPS,
    TABLE_ROW_PROPS,
    TEXT_PROPS,
    TOGGLE_APPEARANCE,
    TOGGLE_PROPS,
    TONE,
    TRANSITION_HINT,
    TREE_ITEM_PROPS,
    TREE_PRESENTATION,
    TREE_PROPS,
    TREE_SECTION_PROPS,
    TREE_WINDOW,
    TREE_WINDOW_ROW_EXTENT,
    TRIGGER,
    UI_CONTRACT_VIOLATION,
    UI_DOCUMENT_LIMITS,
    UI_INTENT,
    UI_NODE_ID,
    UI_NODE_RECORD,
    UI_NUMBER_BOUND,
    UI_NUMBER_LIMITS,
    UI_NUMBER_SCALE,
    UI_PATCH,
    UI_PATCH_OP,
    UI_REVISION,
    UI_SNAPSHOT,
    UI_VALUE,
    VARIANT,
    WINDOW_LAYOUT,
    WINDOW_LAYOUT_NODE,
    WINDOW_STACK_CORNER,
];

/// 🔍️ Rejects unversioned, duplicate, or name-mismatched schema rows before generation.
pub fn validate() -> Result<(), String> {
    let mut names = HashSet::with_capacity(TYPES.len());
    for metadata in TYPES {
        if metadata.version == 0 {
            return Err(format!("schema '{}' has version zero", metadata.name));
        }
        if !names.insert(metadata.name) {
            return Err(format!("duplicate schema '{}'", metadata.name));
        }
        let type_prefix = format!("export type {}", metadata.name);
        let interface_prefix = format!("export interface {}", metadata.name);
        if !metadata.typescript.contains(&type_prefix) && !metadata.typescript.contains(&interface_prefix) {
            return Err(format!("schema '{}' declaration has a mismatched name", metadata.name));
        }
    }
    Ok(())
}

/// 🟦️ Renders the stable language projection consumed by every semantic UI host.
pub fn render_typescript() -> String {
    let mut output = String::from("/** @generated by `repo run @semio-tech/ui-contract-rs:generate` from versioned owned UI schema metadata. Do not edit. */\n\n");
    for (index, metadata) in TYPES.iter().enumerate() {
        output.push_str(metadata.typescript);
        output.push_str(if index + 1 == TYPES.len() { "\n" } else { "\n\n" });
    }
    output
}

//#endregion 🔖️TypescriptProjection

