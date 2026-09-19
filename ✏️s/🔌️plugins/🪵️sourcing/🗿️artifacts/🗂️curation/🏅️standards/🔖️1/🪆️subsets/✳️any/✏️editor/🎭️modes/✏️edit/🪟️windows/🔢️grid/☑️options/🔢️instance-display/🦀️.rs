//! 🔢️ Grid window option — how curated counts appear in the 3D grid.

use crate::editor::sourcing::modes::edit::windows::grid::config::{GridWindowConfig, GRID_INSTANCE_DISPLAY_LINE_BEHIND, GRID_INSTANCE_DISPLAY_REPRESENTATIVE, GRID_INSTANCE_DISPLAY_REPRESENTATIVE_WITH_COUNT};
use crate::editor::sourcing::sourcing_window_action;
use crate::editor::sourcing::terminology::SourcingLabels;
use semio_framework_plugin::{MeasureSelectItem, WindowMeasure};

pub const SOURCING_GRID_INSTANCE_DISPLAY_MEASURE_ID: &str = "sourcing-grid-window.instance-display";

pub fn measure(cfg: &GridWindowConfig, labels: &SourcingLabels) -> WindowMeasure {
    WindowMeasure::Select {
        id: SOURCING_GRID_INSTANCE_DISPLAY_MEASURE_ID.into(),
        label: Some(labels.grid_instance_display.into()),
        value: cfg.instance_display.clone(),
        items: vec![
            MeasureSelectItem { id: GRID_INSTANCE_DISPLAY_LINE_BEHIND.into(), value: GRID_INSTANCE_DISPLAY_LINE_BEHIND.into(), label: labels.grid_instance_display_line_behind.into() },
            MeasureSelectItem { id: GRID_INSTANCE_DISPLAY_REPRESENTATIVE.into(), value: GRID_INSTANCE_DISPLAY_REPRESENTATIVE.into(), label: labels.grid_instance_display_representative.into() },
            MeasureSelectItem {
                id: GRID_INSTANCE_DISPLAY_REPRESENTATIVE_WITH_COUNT.into(),
                value: GRID_INSTANCE_DISPLAY_REPRESENTATIVE_WITH_COUNT.into(),
                label: labels.grid_instance_display_representative_with_count.into(),
            },
        ],
        on_change: sourcing_window_action("setGridInstanceDisplay", None),
    }
}
