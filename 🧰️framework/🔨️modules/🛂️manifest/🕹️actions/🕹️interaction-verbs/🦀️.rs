//! 🕹️ Interaction verbs: the framework-owned selection, hover and granularity action ids, the closed interaction verb type and its refresh scope derivation. See [`super`].

use super::*;

// #region 🛂️Manifest
//#region 🔖️Manifest
//#region 🔖️Interaction
/// 🕹️ The framework-owned action id a renderer dispatches to change a domain's selection (pick,
/// marquee gather, keyboard range/toggle) — never in the palette: renderers translate raw
/// pointer/keyboard input into this, the user never picks it from a menu.
pub const INTERACTION_SELECT_ACTION_ID: &str = "interactionSelect";

/// 🐁️ The framework-owned action id a renderer dispatches to change a domain's hover — never in the
/// palette (mirrors `INTERACTION_SELECT_ACTION_ID`).
pub const INTERACTION_HOVER_ACTION_ID: &str = "interactionHover";

/// 🧹️ The framework-owned action id apps dispatch to clear every declared domain's selection.
pub const CLEAR_SELECTION_ACTION_ID: &str = "clearSelection";

/// 🗂️ The framework-owned action id apps dispatch to select every target of the active domain at its
/// active granularity.
pub const SELECT_ALL_ACTION_ID: &str = "selectAll";

/// 🔀️ The framework-owned action id apps dispatch to switch a domain's active `SelectionMode`.
pub const SET_SELECTION_MODE_ACTION_ID: &str = "setSelectionMode";

/// 🪜️ The framework-owned action id apps dispatch to switch a domain's active granularity.
pub const SET_INTERACTION_GRANULARITY_ACTION_ID: &str = "setInteractionGranularity";

/// 🎮️ The six framework-owned interaction verbs as one closed type — the key an app's declared
/// interaction refresh scope is resolved by (`ArtifactApp::interaction_scope`), so "what does a hover
/// repaint" is a match on a verb rather than a string compare re-derived at every call site. The
/// framework owns the verbs; only the SCOPE each one dirties is app knowledge, because only the app
/// knows which of its window/panel bodies render a selection or a hover.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InteractionVerb {
    /// 🖱️ `interactionSelect` — a pick, marquee gather or keyboard range/toggle.
    Select,
    /// 🐁️ `interactionHover` — pointer motion, the highest-frequency verb in the shell.
    Hover,
    /// 🧹️ `clearSelection` — every declared domain at once.
    ClearSelection,
    /// 🗂️ `selectAll` — every target of every declared domain at its active granularity.
    SelectAll,
    /// 🔀️ `setSelectionMode` — one domain's active `SelectionMode`.
    SetSelectionMode,
    /// 🪜️ `setInteractionGranularity` — one domain's active granularity.
    SetGranularity,
}

impl InteractionVerb {
    /// 🎮️ The verb one dispatched action id stands for; `None` for every action that is not one of the
    /// six (the same membership test `INTERACTION_ACTION_IDS` makes, as a typed answer).
    pub fn of_action(action: &str) -> Option<Self> {
        match action {
            INTERACTION_SELECT_ACTION_ID => Some(Self::Select),
            INTERACTION_HOVER_ACTION_ID => Some(Self::Hover),
            CLEAR_SELECTION_ACTION_ID => Some(Self::ClearSelection),
            SELECT_ALL_ACTION_ID => Some(Self::SelectAll),
            SET_SELECTION_MODE_ACTION_ID => Some(Self::SetSelectionMode),
            SET_INTERACTION_GRANULARITY_ACTION_ID => Some(Self::SetGranularity),
            _ => None,
        }
    }

    /// 🎮️ The declared action id this verb is dispatched under — exact inverse of `of_action`.
    pub fn action_id(self) -> &'static str {
        match self {
            Self::Select => INTERACTION_SELECT_ACTION_ID,
            Self::Hover => INTERACTION_HOVER_ACTION_ID,
            Self::ClearSelection => CLEAR_SELECTION_ACTION_ID,
            Self::SelectAll => SELECT_ALL_ACTION_ID,
            Self::SetSelectionMode => SET_SELECTION_MODE_ACTION_ID,
            Self::SetGranularity => SET_INTERACTION_GRANULARITY_ACTION_ID,
        }
    }

    /// 🎮️ Every verb, in dispatch-id declaration order — what a law walks so a seventh verb cannot be
    /// added without an author deciding what it dirties.
    pub const ALL: [Self; 6] = [Self::Select, Self::Hover, Self::ClearSelection, Self::SelectAll, Self::SetSelectionMode, Self::SetGranularity];
}

//#region 🔖️InteractionRefreshScope
/// 🐢️ The refresh scope one reserved interaction verb owes, DERIVED from the app's own surface
/// declarations instead of hand-written once per app.
///
/// 🪟️ `window_bodies` is the body of every window kind that declares one of the domains the verb
/// touched ([`WindowKindDefinition::interactions`] — the same list that lets a renderer dispatch the
/// verb from that pane at all), so a window that never paints the domain is never asked to re-render
/// for it.
///
/// 🧯️ Why the panels are NOT narrowed the same way: a [`PanelTabDefinition`] declares no interaction
/// refs, so nothing in the manifest says which panel paints a selection — every declared panel body
/// therefore stays dirty for the three verbs that MOVE a selection, which is the widest honest
/// answer and can never under-refresh an inspector. The one lane narrowed without a declaration is
/// [`InteractionVerb::Hover`]: hover is the pointer-transient lane, owed to the surfaces the pointer
/// is over, and a panel row that chased the pointer would flicker rather than inform.
///
/// 🎛️ `measures` is the window's own Select chrome (active mode / active granularity, bound by the
/// measures rail): every verb but `Hover` can move persisted interaction state a rail control binds,
/// and `Hover` can move none. Utilities, tools, engagements, labels and the app-static catalogue are
/// moved by no interaction verb.
pub fn interaction_verb_surface_scope(verb: InteractionVerb, window_bodies: &[String], panel_bodies: &[String]) -> UiDirtyScope {
    let selection_moved = matches!(verb, InteractionVerb::Select | InteractionVerb::ClearSelection | InteractionVerb::SelectAll);
    UiDirtyScope::Partial {
        window_bodies: window_bodies.to_vec(),
        panel_bodies: if selection_moved { panel_bodies.to_vec() } else { Vec::new() },
        utilities: false,
        tools: false,
        engagements: false,
        measures: verb != InteractionVerb::Hover,
        labels: false,
    }
}

/// 🐢️ The framework's own answer to "what does this verb repaint", for every app that implements no
/// `ArtifactApp::interaction_scope` of its own. `None` — and with it the framework's widest
/// [`UiDirtyScope::Full`] — is the answer whenever NO declared window paints a touched
/// domain, because then there is no declaration to derive from and narrowing would be a guess.
pub fn interaction_declared_refresh_scope(verb: InteractionVerb, domains: &[&str], windows_by_domain: &BTreeMap<String, Vec<String>>, panel_bodies: &[String]) -> Option<UiDirtyScope> {
    let mut window_bodies: Vec<String> = Vec::new();
    for domain in domains {
        for body in windows_by_domain.get(*domain).into_iter().flatten() {
            if !window_bodies.iter().any(|entry| entry == body) {
                window_bodies.push(body.clone());
            }
        }
    }
    (!window_bodies.is_empty()).then(|| interaction_verb_surface_scope(verb, &window_bodies, panel_bodies))
}
//#endregion 🔖️InteractionRefreshScope
//#endregion 🔖️Interaction
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
