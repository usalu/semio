//! 🫧️ Ephemeral interaction state for one exact Drawing Canvas window.

use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DrawingCanvasWindowTransient {
    pub engagement_input: String,
    pub trace_pointer_generation: u64,
    pub trace_pointer_completed_work: u64,
    pub trace_pointer_pending_work: u64,
}

semio_framework_value::artifact_retire_struct!(DrawingCanvasWindowTransient { engagement_input, trace_pointer_generation, trace_pointer_completed_work, trace_pointer_pending_work });

semio_framework_plugin::transient_root! {
    state: DrawingCanvasWindowTransient,
    mutation: DrawingCanvasWindowTransientMutation,
    diff: DrawingCanvasWindowTransientDiff,
    owner: "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🫧️transient",
    kind: "set-window-transient",
    display_name: "Set Drawing Canvas Window Transient",
    payload_schema: "drawing.canvas-window.transient",
    envelope: "s.draw.drawing.canvas-window.transient",
    extension: "drawingcanvaswindowtransient",
    fields: { engagement_input: String, trace_pointer_generation: u64, trace_pointer_completed_work: u64, trace_pointer_pending_work: u64 },
}

semio_framework_plugin::window_transient_owners! {
    state: DrawingCanvasWindowTransient,
    mutation: DrawingCanvasWindowTransientMutation,
    windows: {
        DrawingCanvasWindowTransientOwner => super::DRAWING_PLAY_WINDOW_CANVAS,
    },
}
