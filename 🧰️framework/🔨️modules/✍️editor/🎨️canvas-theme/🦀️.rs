//! 🎨️ Canvas theme of the editor: the colour bridge to the value codec and the editor canvas theme with its defaults. See [`super`].

use super::*;

// #region 🔖️Theme
/// 🌉️ `Color` bridge — `Color` (`♾️infinite`, a different owner's module) has no `ToValue`/
/// `FromValue`; hand-written via its own public `components()`/`new()` rather than editing that
/// module. `EditorCanvasTheme` below names this via `#[value(with = "color_bridge")]`.
mod color_bridge {
    pub fn to_value(c: &super::Color) -> semio_framework_value::DslValue {
        semio_framework_value::ToValue::to_value(&c.components())
    }
    pub fn from_value(value: semio_framework_value::DslValue) -> Result<super::Color, semio_framework_value::ValueError> {
        <[f32; 4] as semio_framework_value::FromValue>::from_value(value).map(super::Color::new)
    }
}

// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`, so no `#[value(...)]` rename is needed beyond the `Color` bridge.
#[derive(Clone, Copy, Debug, semio_framework_value_derive::ToValue, FromValue)]
pub(super) struct EditorCanvasTheme {
    #[value(with = "color_bridge")]
    pub(super) raster_clear: Color,
    #[value(with = "color_bridge")]
    pub(super) grid_minor_stroke: Color,
    #[value(with = "color_bridge")]
    pub(super) label_fill: Color,
    #[value(with = "color_bridge")]
    pub(super) label_fill_hovered: Color,
    #[value(with = "color_bridge")]
    pub(super) label_halo: Color,
    #[value(with = "color_bridge")]
    pub(super) hover_fill: Color,
    #[value(with = "color_bridge")]
    pub(super) selection_fill: Color,
}

impl Default for EditorCanvasTheme {
    fn default() -> Self {
        Self::from_board(&ui_styling::BOARD_LIGHT)
    }
}

impl EditorCanvasTheme {
    fn from_board(t: &ui_styling::BoardPalette) -> Self {
        Self {
            raster_clear: Color::new(t.raster_clear),
            grid_minor_stroke: Color::new(t.grid_minor_stroke),
            label_fill: Color::new(t.label_fill),
            label_fill_hovered: Color::new(t.label_fill_hovered),
            label_halo: Color::new(t.label_halo),
            hover_fill: Color::new(t.node_fill_hovered),
            selection_fill: Color::new(t.node_fill_selected),
        }
    }

    fn merge_color_field(next: &mut Color, v: &serde_json::Value, key: &str) {
        theme::merge_color_field(next, v, key);
    }

    pub(super) fn merge_from_json(&mut self, json: &str) -> Result<(), EditorError> {
        let v: serde_json::Value = parse_editor_json(json)?;
        let mut next = *self;
        Self::merge_color_field(&mut next.raster_clear, &v, "rasterClear");
        Self::merge_color_field(&mut next.grid_minor_stroke, &v, "gridMinorStroke");
        Self::merge_color_field(&mut next.label_fill, &v, "labelFill");
        Self::merge_color_field(&mut next.label_fill_hovered, &v, "labelFillHovered");
        Self::merge_color_field(&mut next.label_halo, &v, "labelHalo");
        Self::merge_color_field(&mut next.hover_fill, &v, "nodeFillHovered");
        Self::merge_color_field(&mut next.selection_fill, &v, "nodeFillSelected");
        *self = next;
        Ok(())
    }
}
// #endregion 🔖️Theme
