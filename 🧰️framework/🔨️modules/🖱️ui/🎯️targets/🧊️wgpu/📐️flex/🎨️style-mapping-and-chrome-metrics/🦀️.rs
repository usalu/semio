//! 🎨️ Style mapping and chrome metrics: the section and field chrome metrics and the grow children rule. See [`super`].

use super::*;

// #region flex
//#region 🎨️StyleMapping
pub const SECTION_TITLE_FONT_SIZE: f32 = ui_styling::metrics::typography::TEXT2XL_PX as f32;

pub const SECTION_TITLE_LINE_HEIGHT: f32 = ui_styling::metrics::typography::TEXT2XL_LINE_HEIGHT_PX as f32;

pub const SECTION_TITLE_BODY_GAP: f32 = ui_styling::metrics::dom::ROOT_REM_PX as f32;

pub const SECTION_TRAILING_MARGIN: f32 = ui_styling::metrics::dom::ROOT_REM_PX as f32 * 2.0;

pub const FIELD_LABEL_FONT_SIZE: f32 = ui_styling::metrics::typography::TEXT_SM_PX as f32;

pub const FIELD_LABEL_LINE_HEIGHT: f32 = ui_styling::metrics::typography::TEXT_SM_LINE_HEIGHT_PX as f32;

pub const FIELD_DETAIL_FONT_SIZE: f32 = ui_styling::metrics::typography::TEXT_XS_PX as f32;

/// 🪢️ Block padding of one shared-border row cell (`py-1`, React `SharedBorderRowCell`).
pub const FIELD_CELL_PADDING_BLOCK: f32 = ui_styling::metrics::dom::ROOT_REM_PX as f32 * 0.25;

/// 🪢️ The one hairline divider between two cells of a vertical shared-border row (`border-block-end`).
pub const FIELD_CELL_DIVIDER: f32 = ui_styling::strokes::CHROME_BORDER_HAIRLINE as f32;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ChromeBand {
    pub y: f32,
    pub height: f32,
    pub inline: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SectionFieldChromeMetrics {
    pub title: Option<ChromeBand>,
    pub label: Option<ChromeBand>,
    pub description: Option<ChromeBand>,
    pub control: Option<ChromeBand>,
    pub error: Option<ChromeBand>,
    pub top: f32,
    pub bottom: f32,
    pub total: f32,
}

pub fn section_chrome_metrics(title_height: Option<f32>, content_height: f32) -> SectionFieldChromeMetrics {
    let title = title_height.map(|height| ChromeBand { y: 0.0, height: height.max(0.0), inline: 0.0 });
    let top = title.map_or(0.0, |band| band.height + SECTION_TITLE_BODY_GAP);
    SectionFieldChromeMetrics { title, top, bottom: SECTION_TRAILING_MARGIN, total: top + content_height.max(0.0) + SECTION_TRAILING_MARGIN, ..SectionFieldChromeMetrics::default() }
}

/// 🪢️ Height of the description text that fits between the label cell and a control starting at `control_top`.
pub fn field_description_height(control_top: f32) -> f32 {
    control_top - field_label_cell_height() - FIELD_CELL_PADDING_BLOCK * 2.0 - FIELD_CELL_DIVIDER
}

fn field_label_cell_height() -> f32 {
    FIELD_CELL_PADDING_BLOCK * 2.0 + FIELD_LABEL_LINE_HEIGHT + FIELD_CELL_DIVIDER
}

/// 🪢️ Vertical shared-border row: label, description and error are cells (block padding, inline padding `cell_inline`, one hairline divider to the next cell, none after the last); the control row follows the description with no gap.
/// @see `🧱️elements/📝️Field/🟦️.tsx`
pub fn field_chrome_metrics(description_height: Option<f32>, control_height: f32, error_height: Option<f32>, cell_inline: f32) -> SectionFieldChromeMetrics {
    let inline = cell_inline.max(0.0);
    let band = |y: f32, height: f32| ChromeBand { y, height: height.max(0.0), inline };
    let label = Some(band(FIELD_CELL_PADDING_BLOCK, FIELD_LABEL_LINE_HEIGHT));
    let mut cursor = field_label_cell_height();
    let description = description_height.map(|height| {
        let text = band(cursor + FIELD_CELL_PADDING_BLOCK, height);
        cursor += FIELD_CELL_PADDING_BLOCK * 2.0 + text.height + FIELD_CELL_DIVIDER;
        text
    });
    let control = ChromeBand { y: cursor, height: control_height.max(0.0), inline: 0.0 };
    cursor += control.height;
    let error = error_height.map(|height| {
        cursor += FIELD_CELL_DIVIDER;
        let text = band(cursor + FIELD_CELL_PADDING_BLOCK, height);
        cursor += FIELD_CELL_PADDING_BLOCK * 2.0 + text.height;
        text
    });
    SectionFieldChromeMetrics { label, description, control: Some(control), error, top: control.y, bottom: cursor - control.y - control.height, total: cursor, ..SectionFieldChromeMetrics::default() }
}

/// 🌱️ Whether `kind`'s children grow into leftover main-axis space by virtue of the parent's kind.
/// This remains true only for legacy declarative stacks; an authored spec carries each child's own
/// `StackLayout::grow`.
pub(super) fn grows_children(kind: LayoutNodeKind, authored: bool) -> bool {
    !authored && matches!(kind, LayoutNodeKind::Stack { .. })
}
//#endregion 🎨️StyleMapping
// #endregion flex
