//! 🔎️ Capabilities and frame presenter: the colour depth and unicode level, the capabilities, the waker, the terminal backend trait, the capability detection and the frame presenter. See [`super`].

use super::*;

/// 🌈️ How many colours the attached terminal renders faithfully.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColorDepth {
    Monochrome,
    Ansi16,
    Ansi256,
    TrueColor,
}

/// 🔠️ Which glyph repertoire the attached terminal renders at the widths the text model assumes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnicodeLevel {
    Ascii,
    Full,
}

/// 🧰️ What the attached terminal can do; emitters and painters degrade to it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Capabilities {
    pub color: ColorDepth,
    pub synchronized_output: bool,
    pub unicode: UnicodeLevel,
}

/// ⏰️ A cloneable, thread-safe handle that makes a pending or the next `TerminalBackend::wait` return `Event::Wake`.
#[derive(Clone)]
pub struct Waker(std::sync::Arc<dyn Fn() + Send + Sync>);

impl Waker {
    pub fn new(wake: impl Fn() + Send + Sync + 'static) -> Self {
        Self(std::sync::Arc::new(wake))
    }

    pub fn wake(&self) {
        (self.0)();
    }
}

/// 🔌️ A platform terminal I/O implementation, kept out of the retained-mode core.
pub trait TerminalBackend {
    fn width_mode(&self) -> semio_framework_ui_tui_core::text::WidthMode { semio_framework_ui_tui_core::text::WidthMode::Scalar }
    fn enter(&mut self) -> io::Result<()>;
    fn leave(&mut self) -> io::Result<()>;
    fn size(&self) -> io::Result<Size>;
    fn capabilities(&self) -> Capabilities;
    fn present(&mut self, patch: &AnsiPatch, cursor: Option<CursorSpec>) -> io::Result<()>;
    fn wait(&mut self, deadline: Option<std::time::Instant>) -> io::Result<Vec<Event>>;
    fn waker(&self) -> Waker;
    fn copy(&mut self, text: &str) -> io::Result<()>;
}

//#region 🔎️Capabilities
fn lowered(env: &dyn Fn(&str) -> Option<String>, name: &str) -> String {
    env(name).unwrap_or_default().to_ascii_lowercase()
}

/// 🩺️ Reads colour depth, glyph repertoire and synchronized-output support from the terminal environment (`COLORTERM`, `TERM`, `TERM_PROGRAM`, `WT_SESSION`, `NO_COLOR`, locale).
pub fn detect_capabilities(env: &dyn Fn(&str) -> Option<String>, windows: bool) -> Capabilities {
    let term = lowered(env, "TERM");
    let colorterm = lowered(env, "COLORTERM");
    let program = lowered(env, "TERM_PROGRAM");
    let present = |name: &str| env(name).is_some_and(|value| !value.is_empty());
    let dumb = term == "dumb";
    let modern = ["kitty", "alacritty", "foot", "ghostty", "wezterm", "contour"].iter().any(|name| term.contains(name));
    let color = if present("NO_COLOR") || dumb {
        ColorDepth::Monochrome
    } else if windows || modern || colorterm.contains("truecolor") || colorterm.contains("24bit") || term.contains("truecolor") || term.contains("direct") || present("WT_SESSION") || present("KITTY_WINDOW_ID") || matches!(program.as_str(), "iterm.app" | "wezterm" | "ghostty" | "vscode" | "hyper") {
        ColorDepth::TrueColor
    } else if term.contains("256color") || program == "apple_terminal" {
        ColorDepth::Ansi256
    } else {
        ColorDepth::Ansi16
    };
    let locale = ["LC_ALL", "LC_CTYPE", "LANG"].iter().map(|name| lowered(env, name)).find(|value| !value.is_empty());
    let unicode = if dumb || term == "linux" || (!windows && locale.is_some_and(|value| !value.contains("utf"))) { UnicodeLevel::Ascii } else { UnicodeLevel::Full };
    let vte_current = env("VTE_VERSION").and_then(|value| value.parse::<u32>().ok()).is_some_and(|version| version >= 6800);
    let synchronized_output = !dumb
        && (present("WT_SESSION")
            || present("KITTY_WINDOW_ID")
            || vte_current
            || matches!(program.as_str(), "iterm.app" | "wezterm" | "ghostty" | "vscode")
            || modern);
    Capabilities { color, synchronized_output, unicode }
}

#[cfg(all(feature = "native", not(target_arch = "wasm32"), any(unix, windows)))]
pub(super) fn process_environment(name: &str) -> Option<String> {
    std::env::var(name).ok()
}
//#endregion 🔎️Capabilities

//#region 🎞️Presenter
/// 🎞️ Turns an engine patch and a cursor request into the bytes one terminal frame writes, degraded to the capabilities.
pub struct FramePresenter {
    pub(super) capabilities: Capabilities,
    cursor: crate::ansi::CursorEmitter,
}

impl FramePresenter {
    pub fn new(capabilities: Capabilities) -> Self {
        Self { capabilities, cursor: crate::ansi::CursorEmitter::default() }
    }

    /// 🧾️ What this presenter degrades frames to.
    pub fn capabilities(&self) -> Capabilities {
        self.capabilities
    }

    /// 🧼️ Forgets the cursor state after the terminal was re-entered.
    pub fn reset(&mut self) {
        self.cursor.reset();
    }

    /// ✂️ The bytes of one frame, empty when nothing changes.
    pub fn compose(&mut self, patch: &AnsiPatch, cursor: Option<CursorSpec>) -> String {
        let colored = crate::ansi::quantize_patch(&patch.0, self.capabilities.color);
        self.cursor.frame(&colored, cursor, self.capabilities.synchronized_output)
    }
}
//#endregion 🎞️Presenter
