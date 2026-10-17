//! 🎯️ Note dispatch context — the per-dispatch scratch every leaf command handler receives.

use crate::editor::note::window::NoteCompositeWindowTransient;

/// 🕹️ Per-dispatch scratch: the "blocks" domain's current selection, resolved once by
/// `ArtifactEditor::handle` from `InteractionView` and threaded to every leaf command handler —
/// `app_commands!`'s generated `dispatch` has no way to thread `InteractionView` itself (mirrors
/// `📐️cad`'s `CadDispatchCtx`). Row-id-prefixed `InteractionTarget` ids (the SAME canonical ids the
/// document panel tree renders, see its own doc comment) are converted back to raw block ids here,
/// once, so every downstream handler keeps working with the raw ids it always did.
pub struct NoteDispatchCtx {
    pub selected_block_ids: Vec<String>,
    pub id_owner: crate::schema::NoteIdOwner,
    pub view_state: Option<semio_framework_plugin::ViewModel>,
    pub window_transient: NoteCompositeWindowTransient,
    pub window_transient_owner: Option<semio_framework_plugin::WindowTransientSnapshot>,
}
