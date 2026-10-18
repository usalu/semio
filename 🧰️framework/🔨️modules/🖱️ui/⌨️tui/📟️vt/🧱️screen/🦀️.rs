use super::encode::{InputModes, MouseEncoding, MouseReporting};
use super::history::{reflow, History, Row, Spot};
use super::palette::{flag, Palette, XTERM_BG, XTERM_FG};
use super::parser::VtParser;
use semio_framework_ui_tui_core::cell::{attr, Cell, CellBuffer};
use semio_framework_ui_tui_core::geometry::{Pos, Rect, Size};
use semio_framework_ui_tui_core::text::char_cells;
use semio_framework_ui_tui_core::theme::Rgb;
use semio_framework_ui_tui_core::signal::CursorShape;

#[path = "🧊️screen-grid-and-state/🦀️.rs"]
mod screen_grid_and_state;
use screen_grid_and_state::{
    default_blank, default_tab_stops, Grid, Pen, SavedCursor, DEFAULT_SCROLLBACK, MAX_MATCHES,
};
pub use screen_grid_and_state::{CellPoint, Match, Modes, VtScreen};
pub(super) use screen_grid_and_state::Charset;

#[path = "🏗️construction-view-and-resize/🦀️.rs"]
mod construction_view_and_resize;

#[path = "🖍️cursor-motion-and-scrolling/🦀️.rs"]
mod cursor_motion_and_scrolling;

#[path = "🎚️erase-edit-and-modes/🦀️.rs"]
mod erase_edit_and_modes;

#[path = "🎨️alternate-screen-sgr-and-reports/🦀️.rs"]
mod alternate_screen_sgr_and_reports;

#[path = "🔎️text-extraction-and-search/🦀️.rs"]
mod text_extraction_and_search;
