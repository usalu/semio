//! 🎨️ Style mapping and chrome metrics: the taffy style mapping, the section and field chrome metrics and the grow children rule. See [`super`].

use super::*;

// #region flex
//#region 🎨️StyleMapping
pub(super) fn align_items(align: Align) -> taffy::style::AlignItems {
    match align {
        Align::Start => taffy::style::AlignItems::Start,
        Align::Center => taffy::style::AlignItems::Center,
        Align::End => taffy::style::AlignItems::End,
        Align::Stretch => taffy::style::AlignItems::Stretch,
        Align::Baseline => taffy::style::AlignItems::Baseline,
    }
}

fn justify_content(justify: Justify) -> taffy::style::JustifyContent {
    match justify {
        Justify::Start => taffy::style::JustifyContent::Start,
        Justify::Center => taffy::style::JustifyContent::Center,
        Justify::End => taffy::style::JustifyContent::End,
        Justify::SpaceBetween => taffy::style::JustifyContent::SpaceBetween,
        Justify::SpaceAround => taffy::style::JustifyContent::SpaceAround,
        Justify::SpaceEvenly => taffy::style::JustifyContent::SpaceEvenly,
    }
}

/// 🔲️ One grid track, as taffy 0.9's `GridTemplateComponent` (a `Single | Repeat` union, not a bare
/// `TrackSizingFunction`) — the generic `style_helpers` constructors resolve to whichever of the two
/// the field asks for, so no manual conversion step is needed.
fn grid_track(track: GridTrack) -> taffy::style::GridTemplateComponent<String> {
    use taffy::style_helpers::{auto, fr, length, max_content, min_content};
    match track {
        GridTrack::Auto => auto(),
        GridTrack::Fraction(count) => fr(f32::from(count)),
        GridTrack::Fixed(token) => length(token.px()),
        GridTrack::MinContent => min_content(),
        GridTrack::MaxContent => max_content(),
    }
}

fn gap_size(column: f32, row: f32) -> taffy::geometry::Size<taffy::style::LengthPercentage> {
    taffy::geometry::Size { width: taffy::style::LengthPercentage::length(column), height: taffy::style::LengthPercentage::length(row) }
}

/// 🧬️ The taffy reading of a [`FlowStyle`], used only where the arithmetic path hands over: a
/// `Wrapped` or `Grid` container. Derived from the flow style rather than built beside it, so the two
/// readings of one node cannot drift; `tracks` is the authored grid track list, the one piece of the
/// vocabulary a [`FlowStyle`] has no field for.
pub(super) fn taffy_style(flow: &FlowStyle, tracks: Option<&ui_contract::GridLayout>) -> taffy::Style {
    let grid = matches!(flow.kind, FlowKind::Grid);
    let inset = |value: Option<f32>| value.map_or_else(taffy::style::LengthPercentageAuto::auto, taffy::style::LengthPercentageAuto::length);
    let mut style = taffy::Style {
        display: if grid { taffy::style::Display::Grid } else { taffy::style::Display::Flex },
        flex_direction: if flow.row { taffy::style::FlexDirection::Row } else { taffy::style::FlexDirection::Column },
        flex_wrap: if matches!(flow.kind, FlowKind::Wrapped) { taffy::style::FlexWrap::Wrap } else { taffy::style::FlexWrap::NoWrap },
        flex_grow: flow.grow,
        flex_shrink: flow.shrink,
        gap: if grid { gap_size(flow.gap_cross, flow.gap_main) } else { gap_size(flow.gap_main, flow.gap_main) },
        padding: taffy::geometry::Rect {
            left: taffy::style::LengthPercentage::length(flow.padding.left),
            right: taffy::style::LengthPercentage::length(flow.padding.right),
            top: taffy::style::LengthPercentage::length(flow.padding.top),
            bottom: taffy::style::LengthPercentage::length(flow.padding.bottom),
        },
        align_items: Some(align_items(flow.align)),
        justify_content: Some(justify_content(flow.justify)),
        size: taffy::geometry::Size { width: taffy_dim(flow.width), height: taffy_dim(flow.height) },
        min_size: taffy::geometry::Size { width: taffy::style::Dimension::length(flow.min_width), height: taffy::style::Dimension::length(flow.min_height) },
        position: if flow.absolute { taffy::style::Position::Absolute } else { taffy::style::Position::Relative },
        inset: taffy::geometry::Rect { top: inset(flow.inset[0]), right: inset(flow.inset[1]), bottom: inset(flow.inset[2]), left: inset(flow.inset[3]) },
        ..Default::default()
    };
    if flow.clips {
        style.overflow = taffy::geometry::Point { x: taffy::style::Overflow::Hidden, y: taffy::style::Overflow::Hidden };
    }
    if let Some(tracks) = tracks {
        style.grid_template_columns = tracks.columns.iter().copied().map(grid_track).collect();
        style.grid_template_rows = tracks.rows.iter().copied().map(grid_track).collect();
    }
    style
}

fn taffy_dim(dim: Dim) -> taffy::style::Dimension {
    match dim {
        Dim::Auto => taffy::style::Dimension::auto(),
        Dim::Fill => taffy::style::Dimension::percent(1.0),
        Dim::Length(value) => taffy::style::Dimension::length(value),
    }
}

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
