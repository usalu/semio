//! 🧩️ Reserved contributions of the app surface: the contribution a framework reserves in every app and its audience rule. See [`super`].

use super::*;

// #region 🛂️Manifest
//#region 🔖️Manifest
//#region 🧩️ReservedContribution
/// 🧩️ What one feature reserves on the apps that host it: actions joined to the app roster, actions joined to every
/// window kind of a built app, dialogs joined to an app that may edit, and the audience it fixes for its own verbs. A
/// host (the app builder, the capability catalog) passes the contributions it hosts to [`reserved_app_actions`],
/// [`reserved_window_actions`], [`reserved_editor_dialogs`] and [`reserved_audience_violation`], so a feature reaches
/// every app without the manifest naming it.
#[derive(Clone, Copy)]
pub struct ReservedContribution {
    pub id: &'static str,
    pub app_actions: fn() -> Vec<ActionDefinition>,
    pub window_actions: fn(&AppDefinition) -> Vec<ActionDefinition>,
    pub editor_dialogs: fn() -> Vec<DialogDefinition>,
    pub fixed_audience: fn(&str) -> Option<CapabilityAudience>,
}

impl ReservedContribution {
    /// 🌱️ A contribution named `id` that reserves nothing yet.
    pub const fn new(id: &'static str) -> Self {
        Self { id, app_actions: no_reserved_actions, window_actions: no_reserved_window_actions, editor_dialogs: no_reserved_dialogs, fixed_audience: no_fixed_audience }
    }

    /// 🕹️ Reserves `app_actions` on the app roster of every hosting app.
    pub const fn with_app_actions(self, app_actions: fn() -> Vec<ActionDefinition>) -> Self {
        Self { app_actions, ..self }
    }

    /// 🪟️ Reserves `window_actions` on every window kind of a built app.
    pub const fn with_window_actions(self, window_actions: fn(&AppDefinition) -> Vec<ActionDefinition>) -> Self {
        Self { window_actions, ..self }
    }

    /// 🗳️ Reserves `editor_dialogs` on every hosting app that may edit.
    pub const fn with_editor_dialogs(self, editor_dialogs: fn() -> Vec<DialogDefinition>) -> Self {
        Self { editor_dialogs, ..self }
    }

    /// 🧭️ Fixes the audience of this contribution's own verbs.
    pub const fn with_fixed_audience(self, fixed_audience: fn(&str) -> Option<CapabilityAudience>) -> Self {
        Self { fixed_audience, ..self }
    }
}

fn no_reserved_actions() -> Vec<ActionDefinition> {
    Vec::new()
}

fn no_reserved_window_actions(_app: &AppDefinition) -> Vec<ActionDefinition> {
    Vec::new()
}

fn no_reserved_dialogs() -> Vec<DialogDefinition> {
    Vec::new()
}

fn no_fixed_audience(_action_id: &str) -> Option<CapabilityAudience> {
    None
}

/// 🕹️ The app-roster actions of every contribution, in contribution order.
pub fn reserved_app_actions(contributions: &[ReservedContribution]) -> Vec<ActionDefinition> {
    contributions.iter().flat_map(|contribution| (contribution.app_actions)()).collect()
}

/// 🪟️ The window-kind actions every contribution reserves on the built `app`, in contribution order.
pub fn reserved_window_actions(contributions: &[ReservedContribution], app: &AppDefinition) -> Vec<ActionDefinition> {
    contributions.iter().flat_map(|contribution| (contribution.window_actions)(app)).collect()
}

/// 🗳️ The dialogs every contribution reserves on an app that may edit, in contribution order.
pub fn reserved_editor_dialogs(contributions: &[ReservedContribution]) -> Vec<DialogDefinition> {
    contributions.iter().flat_map(|contribution| (contribution.editor_dialogs)()).collect()
}

/// 🧭️ The audience the first contribution that owns `action_id` fixes for it.
pub fn reserved_fixed_audience(contributions: &[ReservedContribution], action_id: &str) -> Option<CapabilityAudience> {
    contributions.iter().find_map(|contribution| (contribution.fixed_audience)(action_id))
}

/// ⚖️ The audience a contribution fixes for `action` when `action` resolves to a different one.
pub fn reserved_audience_violation(contributions: &[ReservedContribution], action: &ActionDefinition) -> Option<CapabilityAudience> {
    reserved_fixed_audience(contributions, &action.id).filter(|expected| *expected != resolve_audience(action))
}
//#endregion 🧩️ReservedContribution
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
