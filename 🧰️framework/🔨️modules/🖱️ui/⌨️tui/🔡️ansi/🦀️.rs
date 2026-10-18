use crate::backend::ColorDepth;
use semio_framework_ui_tui_core::cell::{Cell, CellBuffer, DiffRun};
use semio_framework_ui_tui_core::theme::Rgb;
use semio_framework_ui_tui_core::signal::{CursorShape, CursorSpec};
use std::borrow::Cow;
use std::time::Duration;

#[path = "📤️sgr-emit-and-session-modes/🦀️.rs"]
mod sgr_emit_and_session_modes;
pub use sgr_emit_and_session_modes::{
    emit_runs, setup_sequence, teardown_sequence, AnsiPatch, SYNC_BEGIN, SYNC_END,
};

#[path = "🌈️cursor-emitter-and-colour-quantizing/🦀️.rs"]
mod cursor_emitter_and_colour_quantizing;
pub use cursor_emitter_and_colour_quantizing::{
    ansi256_to_rgb, quantize_patch, rgb_to_ansi16, rgb_to_ansi256, CursorEmitter,
};

#[path = "🔤️parser-state-and-feeding/🦀️.rs"]
mod parser_state_and_feeding;
pub use parser_state_and_feeding::{
    AnsiParser, ESCAPE_TIMEOUT, PASTE_LIMIT, PASTE_TIMEOUT, SEQUENCE_TIMEOUT,
};
use parser_state_and_feeding::{
    function_key_from_tilde, key_from_codepoint, modifier_bits, ParserState, PASTE_END,
};

#[path = "🔣️csi-mouse-and-paste-decoding/🦀️.rs"]
mod csi_mouse_and_paste_decoding;

#[path = "🖱️click-counter/🦀️.rs"]
mod click_counter;
pub use click_counter::{ClickCounter, CLICK_WINDOW_MS};

//#region 🔤️Parse
use semio_framework_ui_tui_core::event::{mods, Event, Key, KeyEvent, KeypadKey, MouseButton, MouseEvent, MouseKind};
use semio_framework_ui_tui_core::geometry::Pos;

//#endregion 🔤️Parse

#[cfg(test)]
include!("../🧪️tests/🔬️ansi-unit/🦀️.rs");

#[cfg(test)]
include!("../🧪️tests/⌨️input-decoding/🦀️.rs");
