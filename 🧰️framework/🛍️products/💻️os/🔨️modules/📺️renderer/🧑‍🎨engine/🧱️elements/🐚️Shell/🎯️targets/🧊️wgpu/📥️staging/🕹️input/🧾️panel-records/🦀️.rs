//! 🕹️ `shell-input` staging of `panel-records`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
//#region 🧾️PanelRecordsAssembler
/// ⌨️ A control id as a readable row label — React's `humanizeControlId`: the last dot segment,
/// camelCase split on word boundaries, first letter upper-cased.
pub fn humanize_control_id(control_id: &str) -> String {
    let leaf = control_id.rsplit('.').next().unwrap_or(control_id);
    let mut out = String::new();
    for (index, ch) in leaf.chars().enumerate() {
        if index > 0 && ch.is_ascii_uppercase() {
            out.push(' ');
            out.push(ch.to_ascii_lowercase());
            continue;
        }
        out.push(if index == 0 { ch.to_ascii_uppercase() } else { ch });
    }
    out
}

//#endregion 🧾️PanelRecordsAssembler
//#endregion ShellLifecycle

