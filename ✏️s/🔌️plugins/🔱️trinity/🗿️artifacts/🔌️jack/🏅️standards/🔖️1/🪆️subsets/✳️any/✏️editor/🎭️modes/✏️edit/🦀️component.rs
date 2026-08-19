//! 🔱️ Trinity Jack editor — the `edit` mode: the single quad-esque layout (graph left, jack query +
//! results stacked right) stitching this app's three modeless windows into the taxonomy's mode dir.
//! The app itself has no other mode; this is the only `ModeDefinition` `create_trinity_jack_app`
//! registers.

use crate::editor::jack::{TRINITY_JACK_PLAY_WINDOW_EDITOR, TRINITY_JACK_PLAY_WINDOW_GRAPH, TRINITY_JACK_PLAY_WINDOW_RESULTS};
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const TRINITY_JACK_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::jack::create_trinity_jack_app`.
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: TRINITY_JACK_MODE_EDIT.into(), label: LocalizedLabel::native("Explore", "Erkunden"), icon_id: "focus".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ One quadrant of the layout: a stack holding a single window kind.
async fn window_stack(window_kind_id: &str, title: &str, size: Option<f64>) -> WindowLayoutChild {
    WindowLayoutChild::Stack(WindowLayoutStackNode {
        kind: "stack".into(),
        size,
        active_window_kind_id: None,
        children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: window_kind_id.into(), title: Some(title.into()), instance_id: None, template_id: None, corner: None }],
    })
}

/// @emoji 🪟️ Nakagin graph left (60%), Jack query over Results stacked right (40%).
pub async fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
            kind: "row".into(),
            size: None,
            children: vec![
                window_stack(TRINITY_JACK_PLAY_WINDOW_GRAPH, "Nakagin Graph", Some(0.6)),
                WindowLayoutChild::Axis(WindowLayoutAxisNode {
                    kind: "column".into(),
                    size: Some(0.4),
                    children: vec![window_stack(TRINITY_JACK_PLAY_WINDOW_EDITOR, "Jack Query", Some(0.55)), window_stack(TRINITY_JACK_PLAY_WINDOW_RESULTS, "Results", Some(0.45))],
                }),
            ],
        }),
    }
}
//#endregion 🔖️Definition
