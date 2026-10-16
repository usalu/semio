//! ⚓️ `shell-dock` staging of `pointer-input`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellInput

//#endregion ShellInput

/// 🖱️ DOM-standard `MouseEvent.button` code onto the retained event's own button identity.
pub(super) fn retained_pointer_button(button: i16) -> ui_wgpu::wgpu::PointerButton {
    match button {
        1 => ui_wgpu::wgpu::PointerButton::Middle,
        2 => ui_wgpu::wgpu::PointerButton::Secondary,
        _ => ui_wgpu::wgpu::PointerButton::Primary,
    }
}

/// ⌨️🖱️ The live modifier set the retained router's POINTER events carry — the aggregate
/// `InputState::modifiers` the ingress refreshes on every pointer and key event, projected onto
/// `events`' own type. React reads the very same four flags straight off the DOM `PointerEvent`
/// (`⚙️VirtualFileSystem/🟦️.tsx:520-521`), so a retained press that dropped them could never
/// express shift-extend or ctrl-toggle on any list surface
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️audit-w14-scenes-residual.md` C1).
pub(super) fn retained_event_modifiers(input: &InputState<ActionDescriptor>) -> ui_wgpu::wgpu::EventModifiers {
    ui_wgpu::wgpu::EventModifiers { shift: input.modifiers.shift, ctrl: input.modifiers.ctrl, alt: input.modifiers.alt, meta: input.modifiers.meta }
}
