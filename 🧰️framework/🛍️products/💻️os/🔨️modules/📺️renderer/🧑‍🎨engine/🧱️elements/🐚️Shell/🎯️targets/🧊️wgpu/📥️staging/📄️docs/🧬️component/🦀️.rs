//! 📄️ `shell-docs` component structs: `DocumentState` (RENDERER-WP4 step T1).
//!
//! Fields are cut from the former `ShellState` in declaration order and keep their names.

use super::*;

/// 📄️ `DocumentState`: the active session, per-window documents, spawned apps and document identities.
///
/// Fields keep the names they had on the former `ShellState`; the composer owns one value of this struct as `docs`.
pub struct DocumentState {
    pub session: Option<ActiveSession>,
    pub window_ui: HashMap<String, UiDocumentLease>,
    pub panel_documents: HashMap<String, UiDocumentLease>,
    pub spawned_ui: Option<UiDocumentLease>,
    pub spawned_session: Option<ActiveSession>,
    pub app_document_identities: crate::media_slots::DocumentIdentityRegistry,
    pub closing_documents: ShellDocumentRetirementRegistry,
    /// 🎬️ Each live window instance's projected Actions pane document — see
    /// `//#region 🎬️WindowActionsAndSearchPanes`.
    pub window_actions_documents: HashMap<String, UiDocumentLease>,
    /// 🔎️ Each live window instance's projected Search pane document — the same region's twin.
    pub window_search_documents: HashMap<String, UiDocumentLease>,
    /// 🔀️ Each live World window instance's retained Projection tree — the bottom-right Pane body.
    pub window_projection_documents: HashMap<String, UiDocumentLease>,
    /// 📏️ Each live window instance's projected Measures overlay document — see `//#region 📏️WindowMeasures`.
    pub window_measures_documents: HashMap<String, UiDocumentLease>,
}
