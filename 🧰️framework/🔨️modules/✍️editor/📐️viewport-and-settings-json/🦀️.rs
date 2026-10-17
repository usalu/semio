//! 📐️ Viewport and wire records of the editor: the gutter and padding constants, the screen and world conversions, the settings record and the semantic token, span, range, placeholder, diagnostic and edit records. See [`super`].

use super::*;

// #region 🔖️EditorViewport
pub(super) const PAD_X: f64 = 12.0;

pub(super) const PAD_Y: f64 = 8.0;

pub(super) const DEFAULT_GUTTER_WIDTH: f64 = 56.0;

pub(super) const DEFAULT_FONT_PX: f64 = 14.0;

pub(super) const DEFAULT_LINE_HEIGHT: f64 = 22.0;

pub(super) const DEFAULT_TAB_SIZE: usize = 2;

pub(super) fn editor_screen_to_world(camera: &Camera, p: Point) -> Point {
    Point::new(p.x + camera.x, p.y + camera.y)
}

pub(super) fn editor_world_to_screen(camera: &Camera, p: Point) -> Point {
    Point::new(p.x - camera.x, p.y - camera.y)
}

pub(super) fn editor_content_affine(camera: &Camera) -> Affine {
    Affine::new([1.0, 0.0, 0.0, 1.0, -camera.x, -camera.y])
}
// #endregion 🔖️EditorViewport

// #region 🔖️EditorState
#[derive(Clone, Debug, Deserialize, FromValue, ToValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct EditorSettingsJson {
    #[serde(default = "default_font_px")]
    #[value(default = "default_font_px", default_controlled = "default_font_px_controlled")]
    pub(super) font_px: f64,
    #[serde(default = "default_line_height")]
    #[value(default = "default_line_height", default_controlled = "default_line_height_controlled")]
    pub(super) line_height: f64,
    #[serde(default = "default_show_line_numbers")]
    #[value(default = "default_show_line_numbers", default_controlled = "default_show_line_numbers_controlled")]
    pub(super) show_line_numbers: bool,
    #[serde(default = "default_tab_size")]
    #[value(default = "default_tab_size", default_controlled = "default_tab_size_controlled")]
    pub(super) tab_size: usize,
}

fn default_font_px() -> f64 {
    DEFAULT_FONT_PX
}

fn default_line_height() -> f64 {
    DEFAULT_LINE_HEIGHT
}

fn default_show_line_numbers() -> bool {
    true
}

fn default_tab_size() -> usize {
    DEFAULT_TAB_SIZE
}

fn default_font_px_controlled(control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<f64, semio_framework_value::ValueError> {
 control.scoped_stage(|control| {control.begin_stage(1)?;control.step()?;Ok(default_font_px())})
}

fn default_line_height_controlled(control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<f64, semio_framework_value::ValueError> {
 control.scoped_stage(|control| {control.begin_stage(1)?;control.step()?;Ok(default_line_height())})
}

fn default_show_line_numbers_controlled(control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<bool, semio_framework_value::ValueError> {
 control.scoped_stage(|control| {control.begin_stage(1)?;control.step()?;Ok(default_show_line_numbers())})
}

fn default_tab_size_controlled(control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<usize, semio_framework_value::ValueError> {
 control.scoped_stage(|control| {control.begin_stage(1)?;control.step()?;Ok(default_tab_size())})
}

#[derive(Clone, Debug, Deserialize, FromValue, ToValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct SemanticTokenJson {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) class: String,
}

#[derive(Clone, Debug, Deserialize, FromValue, ToValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct SelectableSpanJson {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) kind: String,
}

#[derive(Clone, Debug, Deserialize, FromValue, ToValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct ByteRangeJson {
    pub(super) start: usize,
    pub(super) end: usize,
}

#[derive(Clone, Debug, Deserialize, FromValue, ToValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct PlaceholderJson {
    pub(super) offset: usize,
    pub(super) label: String,
}

#[derive(Clone, Debug, Deserialize, FromValue, ToValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct DiagnosticJson {
    pub(super) start: usize,
    pub(super) end: usize,
    #[value(skip_serializing_if = "Option::is_none")]
    pub(super) severity: Option<String>,
    #[allow(dead_code)]
    pub(super) message: String,
}

#[derive(Clone, Debug, Deserialize, FromValue, ToValue)]
pub(super) struct TextEditJson {
    pub(super) range: TextRangeJson,
    #[serde(rename = "newText")]
    #[value(rename = "newText")]
    pub(super) new_text: String,
}

#[derive(Clone, Debug, Deserialize, FromValue, ToValue)]
struct TextRangeJson {
    pub(super) start: TextPosJson,
    pub(super) end: TextPosJson,
}

#[derive(Clone, Debug, Deserialize, FromValue, ToValue)]
pub(super) struct TextPosJson {
    pub(super) line: u32,
    pub(super) character: u32,
}
// #endregion 🔖️EditorState
