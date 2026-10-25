//! 📤️ Sgr emit and session modes: the ansi patch, the sgr state and run emission, the session mode table and the setup and teardown sequences. See [`super`].

use super::*;

//#region 📤️Emit
/// 🩹️ A batch of raw ANSI bytes ready to write to a terminal (or feed to xterm.js).
#[derive(Default, Clone)]
pub struct AnsiPatch(pub String);

impl AnsiPatch {
    /// 🕳️ Whether the patch repaints nothing.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Clone, Copy, PartialEq)]
struct SgrState {
    fg: Option<Rgb>,
    bg: Option<Rgb>,
    attrs: u8,
}

fn push_sgr(out: &mut String, state: &mut SgrState, cell: &Cell) {
    if state.fg == Some(cell.fg) && state.bg == Some(cell.bg) && state.attrs == cell.attrs {
        return;
    }
    out.push_str("\x1b[0");
    if cell.attrs & semio_framework_ui_tui_core::cell::attr::BOLD != 0 {
        out.push_str(";1");
    }
    if cell.attrs & semio_framework_ui_tui_core::cell::attr::DIM != 0 {
        out.push_str(";2");
    }
    if cell.attrs & semio_framework_ui_tui_core::cell::attr::ITALIC != 0 {
        out.push_str(";3");
    }
    if cell.attrs & semio_framework_ui_tui_core::cell::attr::UNDERLINE != 0 {
        out.push_str(";4");
    }
    if cell.attrs & semio_framework_ui_tui_core::cell::attr::REVERSE != 0 {
        out.push_str(";7");
    }
    out.push_str(&format!(";38;2;{};{};{}", cell.fg[0], cell.fg[1], cell.fg[2]));
    out.push_str(&format!(";48;2;{};{};{}", cell.bg[0], cell.bg[1], cell.bg[2]));
    out.push('m');
    *state = SgrState { fg: Some(cell.fg), bg: Some(cell.bg), attrs: cell.attrs };
}

/// ✍️ Emits the minimal ANSI needed to repaint `runs` of `next` onto a terminal.
pub fn emit_runs(next: &CellBuffer, runs: &[DiffRun], out: &mut AnsiPatch) {
    let mut state = SgrState { fg: None, bg: None, attrs: u8::MAX };
    for run in runs {
        out.0.push_str(&format!("\x1b[{};{}H", run.y + 1, run.x + 1));
        let mut x = run.x;
        while x < run.x + run.len {
            let Some(c) = next.get(x, run.y) else { break };
            if c.width == 0 {
                x += 1;
                continue;
            }
            push_sgr(&mut out.0, &mut state, c);
            next.push_glyph(x, run.y, &mut out.0);
            x += u16::from(c.width.max(1));
        }
    }
}
//#endregion 📤️Emit

//#region 🎚️Modes
struct SessionMode {
    pub(super) code: u16,
    held: bool,
}

const SESSION_MODES: &[SessionMode] = &[
    SessionMode { code: 1049, held: true },
    SessionMode { code: 25, held: false },
    SessionMode { code: 1000, held: true },
    SessionMode { code: 1003, held: true },
    SessionMode { code: 1006, held: true },
    SessionMode { code: 1004, held: true },
    SessionMode { code: 2004, held: true },
];

/// 🔄️ Begin of a synchronized-output frame (DEC private mode 2026).
pub const SYNC_BEGIN: &str = "\x1b[?2026h";

/// 🔚️ End of a synchronized-output frame (DEC private mode 2026).
pub const SYNC_END: &str = "\x1b[?2026l";

fn mode_sequence(code: u16, set: bool) -> String {
    format!("\x1b[?{code}{}", if set { 'h' } else { 'l' })
}

/// 🚪️ Enters the alternate screen, hides the cursor, resets cursor-key modes, and enables any-motion SGR mouse, focus and paste reporting.
pub fn setup_sequence() -> &'static str {
    static SETUP: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    SETUP.get_or_init(|| {
        let mut out = String::from("\x1b[?1l\x1b>");
        for mode in SESSION_MODES {
            out.push_str(&mode_sequence(mode.code, mode.held));
        }
        out.push_str("\x1b[2J");
        out
    })
}

/// 🏠️ Restores the primary screen and every default mode, in reverse order of setup.
pub fn teardown_sequence() -> &'static str {
    static TEARDOWN: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    TEARDOWN.get_or_init(|| {
        let mut out = String::from(SYNC_END);
        for mode in SESSION_MODES.iter().rev() {
            out.push_str(&mode_sequence(mode.code, !mode.held));
        }
        out.push_str("\x1b[0 q\x1b[0m");
        out
    })
}
//#endregion 🎚️Modes
