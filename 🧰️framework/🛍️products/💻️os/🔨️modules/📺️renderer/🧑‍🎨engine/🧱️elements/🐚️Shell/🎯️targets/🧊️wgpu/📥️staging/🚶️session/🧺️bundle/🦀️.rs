//! 🎒️ `shell-session` bundle: `SessionCtx`, the borrowed split of the `ShellState` composer that every use-case method of this crate runs on (PLAN-RENDERER-WGPU-CRATES.md 7.2.3).
//!
//! Each field borrows one component of the composer, so a method touches only the components it names and borrow splitting stays field-precise. Methods of lower use-case crates are reached through the explicit `as_<crate>` bridges, which reborrow the same components.

use super::*;

//#region 🎒️SessionCtx
/// 🎒️ The borrowed component split of one `ShellState` for the `shell-session` use cases; `SessionCtx::of` is the only constructor from the composer.
pub struct SessionCtx<'a> {
    pub chrome: &'a mut ChromeState,
    pub surfaces: &'a mut SurfaceState,
    pub tutorial: &'a mut TutorialState,
    pub settle: &'a mut SettleState,
    pub plugins: &'a mut PluginState,
    pub history: &'a mut HistoryState,
    pub prefs: &'a mut PrefsState,
    pub directory: &'a mut DirectoryState,
    pub sync: &'a mut SyncState,
    pub docs: &'a mut DocumentState,
    pub opening: &'a mut OpeningState,
    pub windows: &'a mut WindowState,
    pub dock: &'a mut ShellDockState,
    pub input: &'a mut ShellInputState,
    pub commands: &'a mut CommandState,
}

impl<'a> SessionCtx<'a> {
    /// 🎒️ Splits the composer into its components.
    pub fn of(shell: &'a mut ShellState) -> Self {
        Self {
            chrome: &mut shell.chrome,
            surfaces: &mut shell.surfaces,
            tutorial: &mut shell.tutorial,
            settle: &mut shell.settle,
            plugins: &mut shell.plugins,
            history: &mut shell.history,
            prefs: &mut shell.prefs,
            directory: &mut shell.directory,
            sync: &mut shell.sync,
            docs: &mut shell.docs,
            opening: &mut shell.opening,
            windows: &mut shell.windows,
            dock: &mut shell.dock,
            input: &mut shell.input,
            commands: &mut shell.commands,
        }
    }
}
//#endregion 🎒️SessionCtx
