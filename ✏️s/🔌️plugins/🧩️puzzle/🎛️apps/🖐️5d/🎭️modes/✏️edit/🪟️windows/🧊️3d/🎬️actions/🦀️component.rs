//! 🎬️ 3D-window action scope — the World3d surface owns the transform-gumball operations
//! (move/rotate/scale/relocate are 3D-only utilities) plus its own camera. Select/brush/fill create
//! operations, deletion, engagement and the global example/json actions apply to both surfaces and
//! stay unscoped orphans, appearing on both windows.

use semio_framework_plugin::ActionRef;

/// 🧱️ Bound onto `WindowKindDefinition::actions` by this window's `definition()` — the taxonomy-node
/// form of the pre-migration `.window_kind_actions(PUZZLE5D_PLAY_WINDOW_3D, …)` builder call.
pub fn ids() -> Vec<ActionRef> {
    vec!["translateSelection".into(), "rotateSelection".into(), "scaleSelection".into(), "worldRelocate".into(), "setCamera3d".into()]
}
