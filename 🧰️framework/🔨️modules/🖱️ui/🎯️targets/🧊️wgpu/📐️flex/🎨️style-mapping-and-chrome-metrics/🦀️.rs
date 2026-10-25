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

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ChromeBand {
    pub y: f32,
    pub height: f32,
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
    let title = title_height.map(|height| ChromeBand { y: 0.0, height: height.max(0.0) });
    let top = title.map_or(0.0, |band| band.height + SECTION_TITLE_BODY_GAP);
    SectionFieldChromeMetrics { title, top, bottom: SECTION_TRAILING_MARGIN, total: top + content_height.max(0.0) + SECTION_TRAILING_MARGIN, ..SectionFieldChromeMetrics::default() }
}

pub fn field_chrome_metrics(description_height: Option<f32>, control_height: f32, error_height: Option<f32>, gap: f32) -> SectionFieldChromeMetrics {
    let gap = gap.max(0.0);
    let mut cursor = FIELD_LABEL_LINE_HEIGHT;
    let label = Some(ChromeBand { y: 0.0, height: FIELD_LABEL_LINE_HEIGHT });
    cursor += gap;
    let description = description_height.map(|height| {
        let band = ChromeBand { y: cursor, height: height.max(0.0) };
        cursor += band.height + gap;
        band
    });
    let control = ChromeBand { y: cursor, height: control_height.max(0.0) };
    cursor += control.height;
    let error = error_height.map(|height| {
        cursor += gap;
        let band = ChromeBand { y: cursor, height: height.max(0.0) };
        cursor += band.height;
        band
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
