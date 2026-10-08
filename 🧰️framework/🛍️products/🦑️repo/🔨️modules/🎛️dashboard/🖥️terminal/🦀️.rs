//! 🖥️ The native developer control plane: a searchable launcher over the command registry, managed
//! terminal windows named after their tasks, and a declarative keymap. The view starts nothing by
//! itself: every start is a resolved registry launch handed to the workspace daemon.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🎮️registry/🦀️.rs
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/⚙️preferences/⌨️keymap/🔣️.json

use std::path::{Path, PathBuf};
use ui_styling::appearance::AppearanceName;
use ui_tui::tui::backend::{NativeTerminal, TerminalBackend, UnicodeLevel};
use ui_tui::tui::engine::Tui;
use ui_tui::tui::geometry::Size;
use ui_tui::tui::theme::Theme;

#[path = "🌐️labels/🦀️.rs"]
pub mod labels;

#[path = "🚀️launcher/🦀️.rs"]
pub mod launcher;

#[path = "🪟️windows/🦀️.rs"]
mod windows;

#[path = "📋️panes/🦀️.rs"]
mod panes;

#[path = "📡️sessions/🦀️.rs"]
mod sessions;

#[path = "⌨️controls/🦀️.rs"]
mod controls;

use controls::Flow;
use windows::{Dashboard, Effect};

const IDLE_SLICE: std::time::Duration = std::time::Duration::from_millis(80);

// #region 🔖️Run
/// 🎛️ Starts native task controls immediately, then connects and discovers asynchronously.
pub fn run(root: &Path) -> i32 { run_with(root, &crate::args::ParsedArgs::default()) }

/// 📖️ The usage text in the chosen language, with the keyboard section generated from the keymap.
pub fn help_text(preferences: &crate::preferences::Preferences) -> String {
    let text = labels::labels(preferences.locale());
    let (keymap, _) = preferences.keymap();
    let mut lines = vec!["semio [dashboard] [--root PATH] [--config JOURNAL] [--workspace] [--language en|de] [--appearance dark|light] [--terminology native|reuse] [--renderer react|wgpu-wasm|wgpu-native] [--layout tabs|columns|rows] [--prefix KEY] [--bindings JSON]".to_string()];
    for (scope, heading) in [(crate::preferences::keymap::Scope::Prefix, text.help_prefix.fill(&[("prefix", &keymap.prefix().label())]).into_string()), (crate::preferences::keymap::Scope::Window, text.help_window.as_str().to_string()), (crate::preferences::keymap::Scope::View, text.help_view.as_str().to_string())] {
        lines.push(heading);
        for action in keymap.actions(scope) {
            let keys = keymap.keys_label(scope, &action).unwrap_or_default();
            let keys = keys.strip_prefix(&format!("{} ", keymap.prefix().label())).filter(|_| action != crate::preferences::keymap::SEND_PREFIX).unwrap_or(&keys).to_string();
            lines.push(format!("  {keys:<16} {}", text.action(&action)));
        }
    }
    lines.push(text.help_terminal.as_str().into());
    lines.push(text.help_customize.as_str().into());
    lines.join("\n")
}

fn present(term: &mut NativeTerminal, tui: &mut Tui, patch: &ui_tui::tui::ansi::AnsiPatch) { term.present(patch, tui.cursor()).ok(); }

/// ⚙️ Starts a native dashboard with strict optional configuration overrides.
pub fn run_with(root: &Path, parsed: &crate::args::ParsedArgs) -> i32 {
    let root = crate::ipc::canonical_path(&parsed.flag("root").map_or_else(|| root.to_path_buf(), PathBuf::from));
    let text = labels::labels(crate::preferences::cli_locale(&root, parsed));
    if !parsed.segments.is_empty() { eprintln!("{}", text.cli_dashboard_unexpected_arguments.fill(&[("arguments", &parsed.segments.join(" "))]).into_string()); return 2; }
    let loaded = crate::preferences::load_lenient(&root, parsed);
    if parsed.has_flag("help") { println!("{}", help_text(&loaded.map(|(preferences, _)| preferences).unwrap_or_default())); return 0; }
    let (preferences, problems) = match loaded { Ok(loaded) => loaded, Err(error) => { eprintln!("[dashboard] {error}"); return 2; } };
    let Ok(mut term) = NativeTerminal::new() else { eprintln!("{}", text.cli_dashboard_attach_failed.as_str()); return 1; };
    if term.enter().is_err() { return 1; }
    let size = term.size().unwrap_or(Size { width: 100, height: 32 });
    let mut tui = Tui::new(size, Theme::new(if preferences.appearance == "light" { AppearanceName::Light } else { AppearanceName::Dark }));
    let mut dash = Dashboard::new(&mut tui, root.clone(), preferences, crate::preferences::local_path(&root, parsed), parsed.has_flag("workspace"));
    dash.waker = Some(term.waker());
    dash.report_keymap_problems();
    if let Some(problem) = problems.first() { dash.notice = Some(dash.text().prefs_journal_problem.fill(&[("problem", problem)]).into_string()); }
    dash.apply_capabilities(&mut tui, term.capabilities().unicode == UnicodeLevel::Full);
    dash.inventory = Some(crate::inventory::start(&root, false));
    dash.frame(&mut tui);
    let first = tui.render();
    present(&mut term, &mut tui, &first);
    let clock = std::time::Instant::now();
    let now = || clock.elapsed().as_millis() as u64;
    loop {
        let polled = dash.poll_daemon(&mut tui);
        let background = dash.poll_background(&mut tui);
        let slice = IDLE_SLICE;
        let wait = tui.deadline_ms().map_or(slice, |ms| slice.min(std::time::Duration::from_millis(ms)));
        let wait = dash.prefix_wait(std::time::Instant::now()).map_or(wait, |left| wait.min(left));
        let events = term.wait(Some(std::time::Instant::now() + wait)).unwrap_or_default();
        let mut quit = polled.quit;
        for event in &events { if dash.handle(&mut tui, event) == Flow::Quit { quit = true; break; } }
        for effect in std::mem::take(&mut dash.effects) { match effect { Effect::Copy(text) => { let _ = term.copy(&text); } } }
        if quit || dash.quit_requested { break; }
        let expired = dash.expire_prefix(std::time::Instant::now());
        let ticked = tui.tick(now());
        if polled.changed || background || !events.is_empty() || ticked || expired { dash.frame(&mut tui); }
        if let Some(patch) = tui.render_due(now()) { present(&mut term, &mut tui, &patch); }
    }
    let _ = dash.send(&crate::ipc::ClientMsg::Detach {});
    let _ = term.leave();
    0
}
// #endregion 🔖️Run

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
