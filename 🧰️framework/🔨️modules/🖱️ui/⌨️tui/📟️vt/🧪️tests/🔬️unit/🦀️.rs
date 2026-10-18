use super::encode::{encode_focus, encode_key, encode_keypad, encode_mouse, encode_paste, encode_wheel_as_arrows, InputModes, KeypadKey, MouseEncoding, MouseReporting};
use super::pane::{SelectionMode, TerminalState};
use super::{CellPoint, Palette, VtScreen};
use semio_framework_ui_tui_core::cell::{attr, Cell, CellBuffer};
use semio_framework_ui_tui_core::event::{mods, Key, KeyEvent, MouseButton, MouseEvent, MouseKind};
use semio_framework_ui_tui_core::geometry::{Pos, Rect, Size};
use semio_framework_ui_tui_core::theme::Theme;
use semio_framework_ui_tui_core::signal::{CursorShape, WidgetSignal};
use ui_styling::appearance::AppearanceName;

include!("🧫️vt-shared-fixtures-and-corpus-laws/🦀️.rs");
include!("⌨️vt-key-mouse-paste-and-focus-encoding-laws/🦀️.rs");
include!("📟️vt-screen-sequence-laws/🦀️.rs");
include!("📜️vt-history-reflow-and-selection-text-laws/🦀️.rs");
include!("🖥️vt-pane-input-search-and-browse-laws/🦀️.rs");
include!("🖱️vt-pane-pointer-selection-and-paint-laws/🦀️.rs");
include!("🎲️vt-robustness-law/🦀️.rs");
