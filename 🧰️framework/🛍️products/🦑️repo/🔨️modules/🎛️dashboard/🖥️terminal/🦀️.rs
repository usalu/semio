//! 🖥️ The native developer control plane: a searchable launcher over the command registry, managed
//! terminal windows named after their tasks, and a declarative keymap. The view starts nothing by
//! itself: every start is a resolved registry launch handed to the workspace daemon.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🎮️registry/🦀️.rs
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/⚙️preferences/⌨️keymap/🔣️.json

use std::path::{Path, PathBuf};
use ui_styling::appearance::AppearanceName;
use semio_framework_ui_tui_terminal::backend::{NativeTerminal, TerminalBackend, UnicodeLevel};
use semio_framework_ui_tui_engine::Tui;
use semio_framework_ui_tui_core::geometry::Size;
use semio_framework_ui_tui_core::theme::Theme;

use dashboard_launcher as launcher;

#[path = "🪟️windows/🦀️.rs"]
mod windows;

#[path = "📋️panes/🦀️.rs"]
mod panes;

#[path = "📡️sessions/🦀️.rs"]
mod sessions;

#[path = "⌨️controls/🦀️.rs"]
mod controls;

#[path = "📂️sources/🦀️.rs"]
mod sources;


use controls::Flow;
use windows::{Dashboard, Effect};

const IDLE_SLICE: std::time::Duration = std::time::Duration::from_millis(80);

// #region 🔖️Run
/// 🎛️ Starts native task controls immediately, then connects and discovers asynchronously.
pub fn run(root: &Path) -> i32 { run_with(root, &dashboard_core::args::ParsedArgs::default()) }

/// 📖️ The usage text in the chosen language, with the keyboard section generated from the keymap.
pub fn help_text(preferences: &dashboard_preferences::Preferences) -> String {
    let Some(locale)=preferences.locale() else { return dashboard_preferences::language::declared().choices().iter().map(|choice| { let mut resolved=preferences.clone();resolved.language=Some(choice.language.clone());help_text(&resolved) }).collect::<Vec<_>>().join("\n\n"); };
    let text = dashboard_labels::labels(locale);
    let (keymap, _) = preferences.keymap();
    let mut lines = vec!["repo dashboard [--root PATH] [--config JOURNAL] [--workspace] [--language en|de] [--appearance dark|light] [--terminology native|reuse] [--renderer react|wgpu-wasm|wgpu-native] [--layout tabs|columns|rows] [--prefix KEY] [--bindings JSON]".to_string()];
    for (scope, heading) in [(dashboard_preferences::keymap::Scope::Prefix, text.help_prefix.fill(&[("prefix", &keymap.prefix().label())]).into_string()), (dashboard_preferences::keymap::Scope::Window, text.help_window.as_str().to_string()), (dashboard_preferences::keymap::Scope::View, text.help_view.as_str().to_string())] {
        lines.push(heading);
        for action in keymap.actions(scope) {
            let keys = keymap.keys_label(scope, &action).unwrap_or_default();
            let keys = keys.strip_prefix(&format!("{} ", keymap.prefix().label())).filter(|_| action != dashboard_preferences::keymap::SEND_PREFIX).unwrap_or(&keys).to_string();
            lines.push(format!("  {keys:<16} {}", text.action(&action)));
        }
    }
    lines.push(text.help_terminal.as_str().into());
    lines.push(text.help_customize.as_str().into());
    lines.join("\n")
}

fn present<B:TerminalBackend>(term:&mut B,tui:&mut Tui,patch:&semio_framework_ui_tui_terminal::ansi::AnsiPatch)->Result<(),dashboard_feedback::Failure>{term.present(patch,tui.cursor()).map_err(|error|dashboard_feedback::Failure::io(dashboard_feedback::Operation::Present,error))}

fn run_view<B:TerminalBackend>(term:&mut B,dash:&mut Dashboard,tui:&mut Tui)->Result<(),dashboard_feedback::TerminalFailure>{
    let result:Result<(),dashboard_feedback::Failure>=(||{
    dash.frame(tui);
    let first = tui.render();
    present(term, tui, &first)?;
    let clock = std::time::Instant::now();
    let now = || clock.elapsed().as_millis() as u64;
    loop {
        let polled = dash.poll_daemon(tui);
        let background = dash.poll_background(tui);
        let sources = dash.poll_source_suggestions();
        let documents = dash.poll_documents(tui);
        let slice = IDLE_SLICE;
        let wait = tui.deadline_ms().map_or(slice, |ms| slice.min(std::time::Duration::from_millis(ms)));
        let wait = dash.prefix_wait(std::time::Instant::now()).map_or(wait, |left| wait.min(left));
        let events = term.wait(Some(std::time::Instant::now() + wait)).map_err(|error|dashboard_feedback::Failure::io(dashboard_feedback::Operation::Wait,error))?;
        let mut quit = polled.quit;
        for event in &events { if dash.handle(tui, event) == Flow::Quit { quit = true; break; } }
        if quit || dash.quit_requested { break; }
        let mut effect_changed=false;
        for effect in std::mem::take(&mut dash.effects) { match effect { Effect::Copy(text) => { if let Err(error)=term.copy(&text){dash.record_io(dashboard_feedback::Operation::Copy,error);effect_changed=true;} }, Effect::OpenReady(url) => dash.begin_open_ready(&url) } }
        let expired = dash.expire_prefix(std::time::Instant::now());
        let ticked = tui.tick(now());
        if effect_changed || polled.changed || background || sources || documents || !events.is_empty() || ticked || expired { dash.frame(tui); }
        if let Some(patch) = tui.render_due(now()) { present(term, tui, &patch)?; }
    }
    Ok(())
    })();
    if let Some(job)=&dash.saving_preferences{job.cancel.store(true,std::sync::atomic::Ordering::Release);}
    let mut result=result.map_err(dashboard_feedback::TerminalFailure::from);
    if dash.online(){if let Err(error)=dash.send(&dashboard_wire::ipc::ClientMsg::Detach{}){let cleanup=dashboard_feedback::Failure::io(dashboard_feedback::Operation::Detach,error);match &mut result{Err(failure)=>failure.cleanup.push(cleanup),Ok(())=>result=Err(cleanup.into())}}}
    result
}


/// ⚙️ Starts a native dashboard with strict optional configuration overrides.
pub fn run_with(root: &Path, parsed: &dashboard_core::args::ParsedArgs) -> i32 {
    let root = dashboard_wire::ipc::canonical_path(&parsed.flag("root").map_or_else(|| root.to_path_buf(), PathBuf::from));
    if !parsed.segments.is_empty() { for choice in dashboard_preferences::language::declared().choices(){let locale=ui_locale::Locale::parse(&choice.language).expect("validated choice");if dashboard_preferences::cli_locale(&root,parsed).is_none_or(|selected|selected==locale){eprintln!("{}",dashboard_labels::labels(locale).cli_dashboard_unexpected_arguments.fill(&[("arguments",&parsed.segments.join(" "))]).into_string());}} return 2; }
    let loaded = dashboard_preferences::load_lenient(&root, parsed);
    if parsed.has_flag("help") { println!("{}", help_text(&loaded.map(|(preferences, _)| preferences).unwrap_or_default())); return 0; }
    let (mut preferences, problems) = match loaded { Ok(loaded) => loaded, Err(error) => { let failure=dashboard_feedback::TerminalFailure{primary:dashboard_feedback::Failure::io(dashboard_feedback::Operation::Preferences,error),cleanup:Vec::new()};eprintln!("{}",failure.display(None));return 2; } };
    let mut preferences_locale=preferences.locale();
    let mut term=match NativeTerminal::new(){Ok(term)=>term,Err(error)=>{let failure=dashboard_feedback::TerminalFailure{primary:dashboard_feedback::Failure::text(dashboard_feedback::Operation::Enter,error.to_string()),cleanup:Vec::new()};eprintln!("{}",failure.display(preferences_locale));return 1;}};
    let result=dashboard_feedback::terminal_session(&mut term,|term|{
    if preferences.locale().is_none() {
        let path=if parsed.has_flag("workspace"){dashboard_preferences::shared_path(&root)}else{dashboard_preferences::local_path(&root,parsed)};
        match dashboard_language_choice::run(term,&path,Theme::new(if preferences.appearance=="light"{AppearanceName::Light}else{AppearanceName::Dark}))? {Some(locale)=>{preferences.language=Some(locale.as_str().into());preferences_locale=Some(locale);},None=>return Ok(())}
    }
    let size = term.size().unwrap_or(Size { width: 100, height: 32 });
    let mut tui = Tui::new(size, Theme::new(if preferences.appearance == "light" { AppearanceName::Light } else { AppearanceName::Dark }));
    tui.set_width_mode(term.width_mode());
    let mut dash = Dashboard::new(&mut tui, root.clone(), preferences, dashboard_preferences::local_path(&root, parsed), parsed.has_flag("workspace"));
    dash.waker = Some(term.waker());
    dash.report_keymap_problems();
    if let Some(problem) = problems.first() { dash.notice = Some(dash.text().prefs_journal_problem.fill(&[("problem", problem)]).into_string()); }
    dash.apply_capabilities(&mut tui, term.capabilities().unicode == UnicodeLevel::Full);
    dash.inventory = Some(dashboard_inventory::start(&root, false));
    run_view(term,&mut dash,&mut tui)
    });
    match result{Ok(())=>0,Err(error)=>{eprintln!("{}",error.display(preferences_locale));1}}
}
// #endregion 🔖️Run

/// 🧪️ The internals the terminal laws drive, present only with the `testkit` feature so the shipped API stays `run`, `run_with` and `help_text`.
#[cfg(feature = "testkit")]
pub mod testing {
    pub use crate::controls::{fit_hints, spec_of, Action, Flow};
    pub use crate::panes::{PreferenceJob, RowAction};
    pub use crate::sessions::Link;
    pub use crate::windows::{stack_of, Body, Dashboard, Effect, Kind, LinkState, Place};

    /// 🏃️ Runs the view loop on a backend the law owns, so no real terminal is needed.
    pub fn run_view<B: super::TerminalBackend>(term: &mut B, dash: &mut Dashboard, tui: &mut super::Tui) -> Result<(), dashboard_feedback::TerminalFailure> { super::run_view(term, dash, tui) }
}
