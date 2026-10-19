//! 🕹️ `shell-input` staging of `chrome-paint-and-navbar`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ActionPanelAndUtilities
/// 🖱️ Maps a `UtilityDefinition.cursor` CSS/winit cursor name onto the shell's {@link ui_wgpu::wgpu::SemioCursor}.
pub fn semio_cursor_from_name(name: &str) -> semio_framework_ui_wgpu_hosting::cursor::SemioCursor {
    use semio_framework_ui_wgpu_hosting::cursor::SemioCursor;
    match name.trim().to_ascii_lowercase().as_str() {
        "pointer" => SemioCursor::Pointer,
        "text" => SemioCursor::Text,
        "grab" => SemioCursor::Grab,
        "grabbing" => SemioCursor::Grabbing,
        "move" => SemioCursor::Move,
        "crosshair" | "cross" => SemioCursor::Crosshair,
        "not-allowed" | "notallowed" | "forbidden" => SemioCursor::NotAllowed,
        "ew-resize" | "col-resize" | "e-resize" | "w-resize" => SemioCursor::EwResize,
        "ns-resize" | "row-resize" | "n-resize" | "s-resize" => SemioCursor::NsResize,
        "nwse-resize" | "nw-resize" | "se-resize" => SemioCursor::NwseResize,
        "nesw-resize" | "ne-resize" | "sw-resize" => SemioCursor::NeswResize,
        "cell" | "selectable" => SemioCursor::Selectable,
        _ => SemioCursor::Default,
    }
}

//#endregion ActionPanelAndUtilities

