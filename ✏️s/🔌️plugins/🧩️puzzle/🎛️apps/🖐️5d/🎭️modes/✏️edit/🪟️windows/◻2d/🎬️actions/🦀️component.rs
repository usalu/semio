//! 🎬️ 2D-window action scope — the Board2d surface owns board-event dispatch and its own camera.
//! Select/brush/fill create operations, deletion, engagement and the global example/json actions
//! apply to both surfaces and stay unscoped orphans, appearing on both windows.

use semio_framework_plugin::ActionRef;

/// 🧱️ Bound onto `WindowKindDefinition::actions` by this window's `definition()` — the taxonomy-node
/// form of the pre-migration `.window_kind_actions(PUZZLE5D_PLAY_WINDOW_2D, …)` builder call.
pub fn ids() -> Vec<ActionRef> {
    vec!["applyBoardEvents".into(), "setCamera2d".into()]
}
