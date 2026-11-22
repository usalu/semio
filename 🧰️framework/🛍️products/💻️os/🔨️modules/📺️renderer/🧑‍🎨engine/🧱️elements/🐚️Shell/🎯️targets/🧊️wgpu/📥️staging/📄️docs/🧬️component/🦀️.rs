//! 📄️ `shell-docs` component structs: `DocumentState` (RENDERER-WP4 step T1).
//!
//! Fields are cut from the former `ShellState` in declaration order and keep their names.

use super::*;

/// 📄️ `DocumentState`: the active session, per-window documents, spawned apps and document identities.
///
/// Fields keep the names they had on the former `ShellState`; the composer owns one value of this struct as `docs`.
pub struct DocumentState {
    pub session: Option<ActiveSession>,
    pub workspace_clipboard: Option<semio_framework::kernel::ClipboardFragment>,
    pub window_ui: HashMap<String, UiDocumentLease>,
    pub panel_documents: HashMap<String, UiDocumentLease>,
    pub spawned_ui: Option<UiDocumentLease>,
    pub spawned_session: Option<ActiveSession>,
    pub app_document_identities: semio_renderer_wgpu_media_slots::DocumentIdentityRegistry,
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
    /// 🏷️ The active program's published `labels` section, React's `windowUi.appLabelsOverlay`: the navbar example select reads its example labels from it.
    pub app_labels_overlay: semio_framework::manifest::AppLabelsOverlay,
}

impl DocumentState {
    /// 📋️ Resolves the workspace fragment at paste dispatch and preserves the caller's placement.
    pub fn bind_workspace_paste(&self, action: &mut ActionDescriptor) {
        if action.action != "paste" || action.args.as_ref().and_then(|args| args.get("fragment")).is_some() {
            return;
        }
        let Some(fragment) = self.workspace_clipboard.as_ref() else { return };
        let mut args = match action.args.take() {
            Some(semio_framework_value::DslValue::Object(entries)) => entries,
            _ => Vec::new(),
        };
        args.push(("fragment".into(), semio_framework_value::ToValue::to_value(fragment)));
        action.args = Some(semio_framework_value::DslValue::Object(args));
    }
}
