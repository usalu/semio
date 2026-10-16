//! 🔀️ `shell-actions` staging of `space-admin-and-settle`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellTypes
//#region 🧯️BootProgramSelection
/// 🎯️ Which program + app a standalone (non-host) boot opens, resolved from the requested variant
/// alone. `resolvePlaygroundBoot` hands the renderer its plugins in DEPENDENCY order, so the first
/// entry is a dependency of the requested plugin (`flow` ahead of `procedural` for `generation3d`),
/// never the requested plugin itself: taking `plugins.first()` booted a foreign plugin's first app
/// and made that plugin's first-step trap the whole shell's boot failure. Falls back to the program
/// that actually declares the requested app id, and only then to the requested program's first app;
/// a variant whose plugin is absent resolves to `None` rather than to somebody else's app.
///
/// 📌️ `?app=`/`--app`/`appId` PINS the anchor, React's `appId` — a pin the manifest does not
/// declare is dropped back to the variant's own app, the same "a url is not a place to hard-fail a
/// shell" rule `?role=`/`?mode=`/`?example=` follow.
pub fn select_boot_program(programs: &[(&str, &semio_framework::PluginManifest)], request: BootSelectionRequest<'_>, app_role: semio_framework::manifest::AppRole, pinned_app_id: Option<&str>) -> Option<(usize, AppDefinition)> {
    let requested_app_id = pinned_app_id.filter(|app_id| !app_id.is_empty() && programs.iter().any(|(_, manifest)| manifest.apps.iter().any(|app| app.id == *app_id))).or(request.app_id);
    let index = programs.iter().position(|(plugin_id, _)| *plugin_id == request.plugin_id).or_else(|| requested_app_id.and_then(|app_id| programs.iter().position(|(_, manifest)| manifest.apps.iter().any(|app| app.id == app_id))))?;
    let manifest = programs[index].1;
    let anchor = requested_app_id.and_then(|app_id| manifest.apps.iter().find(|app| app.id == app_id)).or_else(|| manifest.apps.first())?;
    Some((index, project_boot_app_role(manifest, anchor, app_role).clone()))
}

//#endregion 🧯️BootProgramSelection

/// 🧯 The watchdog's verdict for ONE live producer, and the only place the stall is counted.
///
/// A witness that MOVED resets everything — including the terminal-drive count, so a producer that
/// recovered after an abort is watched from scratch. A witness that is frozen counts up, and the step
/// that crosses [`SHELL_SETTLE_STALL_STEPS`] asks for a terminal drive at most
/// [`SHELL_SETTLE_TERMINAL_DRIVES`] times — a producer that answers an abort with a fresh wedge is a
/// producer defect, not a reason for the host to abort forever.
///
/// ⚠️ A producer that offers no `cancelAction` is never driven: this shell learns no domain verb from
/// code, so a run it has no published way to end is left to the user.
pub(crate) fn settle_watch_verdict(watch: &mut ShellSettleWatch, status: &World3dComputeStatus) -> ShellSettleVerdict {
    let signature = settle_progress_signature(status);
    if watch.signature != signature {
        watch.signature = signature;
        watch.stalled_steps = 0;
        watch.terminal_drives = 0;
        watch.standing = false;
        return ShellSettleVerdict::Fund;
    }
    watch.stalled_steps = watch.stalled_steps.saturating_add(1);
    if watch.stalled_steps < SHELL_SETTLE_STALL_STEPS {
        return ShellSettleVerdict::Fund;
    }
    if watch.terminal_drives >= SHELL_SETTLE_TERMINAL_DRIVES || !status.cancellable {
        watch.stalled_steps = SHELL_SETTLE_STALL_STEPS;
        watch.standing = true;
        return ShellSettleVerdict::Stand;
    }
    watch.stalled_steps = 0;
    watch.terminal_drives = watch.terminal_drives.saturating_add(1);
    ShellSettleVerdict::Terminal
}

//#endregion ShellTypes

