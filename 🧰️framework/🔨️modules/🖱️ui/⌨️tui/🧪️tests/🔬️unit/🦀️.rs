//! 🔬️ tui unit corpus: layout, window, table, parser, theme, geometry, text, cell, ANSI, vt and widget laws.

#![cfg(test)]


use semio_framework_ui_tui_terminal::ansi::{AnsiParser, AnsiPatch, emit_runs, setup_sequence, teardown_sequence};
use semio_framework_ui_tui_core::cell::{Cell, CellBuffer, DiffRun, attr, diff};
use semio_framework_ui_tui_frame::chrome::{ChromeState, FooterState, KeyHint, NavItem, NavbarState, WindowState, mount_window_layout, shell, window_chip_layout, window_content_padding};
use semio_framework_ui_tui_core::event::{Event, Key, KeyEvent, MouseButton, MouseEvent, MouseKind};
use semio_framework_ui_tui_core::geometry::{Pos, Rect, Size};
use semio_framework_ui_tui_frame::layout::{Constraint, Dimension, Direction, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode, create_default_layout, even_window_layout, solve, solve_window_layout};
use semio_framework_ui_tui_frame::scene::{Node, NodeContent, Scene};
use semio_framework_ui_tui_core::text::{display_width, truncate_to};
use semio_framework_ui_tui_core::theme::{Role, Surface, Theme};
use semio_framework_ui_tui_widgets::widget::{Align, ChipState, DividerState, InputState, LabelState, ListState, LogScroll, LogState, SelectState, TableAlign, TableColumn, TableRow, TableState, TabsState, WidgetState, WizardState};
use semio_framework_ui_tui_vt::pane::TerminalState;
use semio_framework_ui_tui_core::signal::WidgetSignal;
use ui_styling::appearance::AppearanceName;

include!("🪟️tui-layout-and-window-laws/🦀️.rs");
include!("📊️tui-table-diff-parser-and-theme-laws/🦀️.rs");
include!("📐️tui-geometry-text-cell-and-ansi-laws/🦀️.rs");
include!("📟️tui-vt-scene-and-layout-laws/🦀️.rs");
include!("⌨️tui-widget-key-handling-laws/🦀️.rs");
include!("🎨️tui-widget-paint-and-window-silhouette-laws/🦀️.rs");
include!("🚦️tui-chrome-engine-wasm-clipboard-and-pty-laws/🦀️.rs");
