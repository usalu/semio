//! 🔔️ Widget signals and the terminal cursor description shared by every widget, the VT pane and the terminal backend.

/// 📣️ A widget- or window-chrome-level result of handling input, surfaced to the app.
#[derive(Clone, Debug, PartialEq)]
pub enum WidgetSignal {
    Activated(usize),
    SelectionChanged(usize),
    ValueChanged(String),
    Toggled(bool),
    TabChanged(usize),
    NavigateBack,
    Hovered(Option<usize>),
    ContextMenu { pos: crate::geometry::Pos, item: Option<usize> },
    Copy(String),
    OpenUrl(String),
    /// 📤 Bytes the embedded terminal encoded for its child; the host forwards them to the PTY.
    TerminalInput(Vec<u8>),
    /// ❌ The stack tab index whose close control was pressed.
    WindowClose(usize),
    /// 🔲 The stack tab index the maximize control belongs to.
    WindowMaximize(usize),
    /// ➕ The stack tab index the new-tab control belongs to.
    WindowNewTab(usize),
    WindowTabActivated(usize),
    /// 🔆 Keyboard focus moved into the emitting window.
    WindowFocus,
    TabMoved { from: usize, to: usize },
        /// 🧲️ A held tab was released over a group or one of its split edges.
    WindowDocked { source: String, target: String, placement: crate::tui::layout::DockPlacement, tab_index: Option<usize> },
    /// 🔲️ A proposal lets the owning layout resolve the preview after reclaiming the source group.
    WindowDockPreview { source: String, target: String, placement: crate::tui::layout::DockPlacement, tab_index: Option<usize> },
/// ↕️ `path` names the splitter inside its window layout, `delta` the cells the pointer moved along the axis since the last report.
    SplitterDragged { path: Vec<usize>, delta: i16 },
    /// 🙈 The emitting overlay was closed without a choice (Escape or a click outside).
    Dismissed,
}

/// 🖋️ How the terminal draws its text cursor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CursorShape {
    Block,
    Underline,
    Bar,
}

/// 📍️ Where and how the terminal cursor shows while a widget holds focus.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CursorSpec {
    pub pos: crate::geometry::Pos,
    pub shape: CursorShape,
    pub blink: bool,
}
