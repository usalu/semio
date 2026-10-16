//! 🛍️ `shell-pointer` bundle: `PointerCtx`, the borrowed split of the `ShellState` composer that every use-case method of this crate runs on (PLAN-RENDERER-WGPU-CRATES.md 7.2.3).
//!
//! Each field borrows one component of the composer, so a method touches only the components it names and borrow splitting stays field-precise. Methods of lower use-case crates are reached through the explicit `as_<crate>` bridges, which reborrow the same components.

use super::*;

//#region 🛍️PointerCtx
/// 🛍️ The borrowed component split of one `ShellState` for the `shell-pointer` use cases; `PointerCtx::of` is the only constructor from the composer.
pub struct PointerCtx<'a> {
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

impl<'a> PointerCtx<'a> {
    /// 🛍️ Splits the composer into its components.
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

    /// 🛍️ The same components as a `SessionCtx`, for calling a use case of the lower crate `shell-session`.
    pub fn as_session(&mut self) -> SessionCtx<'_> {
        SessionCtx {
            chrome: &mut *self.chrome,
            surfaces: &mut *self.surfaces,
            tutorial: &mut *self.tutorial,
            settle: &mut *self.settle,
            plugins: &mut *self.plugins,
            history: &mut *self.history,
            prefs: &mut *self.prefs,
            directory: &mut *self.directory,
            sync: &mut *self.sync,
            docs: &mut *self.docs,
            opening: &mut *self.opening,
            windows: &mut *self.windows,
            dock: &mut *self.dock,
            input: &mut *self.input,
            commands: &mut *self.commands,
        }
    }

    /// 🛍️ The same components as a `WindowActionsCtx`, for calling a use case of the lower crate `shell-window-actions`.
    pub fn as_window_actions(&mut self) -> WindowActionsCtx<'_> {
        WindowActionsCtx {
            chrome: &mut *self.chrome,
            surfaces: &mut *self.surfaces,
            tutorial: &mut *self.tutorial,
            settle: &mut *self.settle,
            plugins: &mut *self.plugins,
            history: &mut *self.history,
            prefs: &mut *self.prefs,
            directory: &mut *self.directory,
            sync: &mut *self.sync,
            docs: &mut *self.docs,
            opening: &mut *self.opening,
            windows: &mut *self.windows,
            dock: &mut *self.dock,
            input: &mut *self.input,
            commands: &mut *self.commands,
        }
    }

    /// 🛍️ The same components as a `ActionsCtx`, for calling a use case of the lower crate `shell-actions`.
    pub fn as_actions(&mut self) -> ActionsCtx<'_> {
        ActionsCtx {
            chrome: &mut *self.chrome,
            surfaces: &mut *self.surfaces,
            tutorial: &mut *self.tutorial,
            settle: &mut *self.settle,
            plugins: &mut *self.plugins,
            history: &mut *self.history,
            prefs: &mut *self.prefs,
            directory: &mut *self.directory,
            sync: &mut *self.sync,
            docs: &mut *self.docs,
            opening: &mut *self.opening,
            windows: &mut *self.windows,
            dock: &mut *self.dock,
            input: &mut *self.input,
            commands: &mut *self.commands,
        }
    }
}
//#endregion 🛍️PointerCtx
