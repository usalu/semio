//! ♻️ Trinity Rewriting editor — the `edit` mode: the two-row layout (LHS/RHS/Jack over Parameters/
//! Before/After) stitching this app's six modeless windows into the taxonomy's mode dir. The app
//! itself has no other mode; this is the only `ModeDefinition` `create_rewriting_app` registers.

use crate::editor::rewriting::{TRINITY_REWRITING_PLAY_WINDOW_AFTER, TRINITY_REWRITING_PLAY_WINDOW_BEFORE, TRINITY_REWRITING_PLAY_WINDOW_JACK, TRINITY_REWRITING_PLAY_WINDOW_LHS, TRINITY_REWRITING_PLAY_WINDOW_PARAMETERS, TRINITY_REWRITING_PLAY_WINDOW_RHS};
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const TRINITY_REWRITING_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::rewriting::create_rewriting_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: TRINITY_REWRITING_MODE_EDIT.into(), label: LocalizedLabel::native("Explore", "Erkunden"), icon_id: "focus".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ One quadrant of the layout: a stack holding a single window kind.
fn window_stack(window_kind_id: &str, title: &str, size: Option<f64>) -> WindowLayoutChild {
    WindowLayoutChild::Stack(WindowLayoutStackNode {
        kind: "stack".into(),
        size,
        active_window_kind_id: None,
        children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: window_kind_id.into(), title: Some(title.into()), instance_id: None, template_id: None, corner: None }],
    })
}

/// @emoji 🪟️ Two rows: LHS/RHS/Jack on top (50%), Parameters/Before/After on the bottom (50%).
pub fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
            kind: "column".into(),
            size: None,
            children: vec![
                WindowLayoutChild::Axis(WindowLayoutAxisNode {
                    kind: "row".into(),
                    size: Some(0.5),
                    children: vec![window_stack(TRINITY_REWRITING_PLAY_WINDOW_LHS, "LHS", Some(0.34)), window_stack(TRINITY_REWRITING_PLAY_WINDOW_RHS, "RHS", Some(0.34)), window_stack(TRINITY_REWRITING_PLAY_WINDOW_JACK, "Jack", Some(0.32))],
                }),
                WindowLayoutChild::Axis(WindowLayoutAxisNode {
                    kind: "row".into(),
                    size: Some(0.5),
                    children: vec![
                        window_stack(TRINITY_REWRITING_PLAY_WINDOW_PARAMETERS, "Parameters", Some(0.34)),
                        window_stack(TRINITY_REWRITING_PLAY_WINDOW_BEFORE, "Before", Some(0.33)),
                        window_stack(TRINITY_REWRITING_PLAY_WINDOW_AFTER, "After", Some(0.33)),
                    ],
                }),
            ],
        }),
    }
}
//#endregion 🔖️Definition
