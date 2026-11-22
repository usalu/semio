//! 🚦️ Frame demand: why a host tick may admit a frame build, and the trace line that names each admitted build.

/// 🧭️ Every reason one host tick may owe a frame build. A tick with none of them set rebuilds nothing:
/// frames are built on demand — input, animation or deadline, resource arrival, or work still in flight —
/// never because a tick happened to arrive. Language-agnostic oracle: `🧪️tests/🧫️fixtures/🚦️frame-demand-and-traces/🔣️.json`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FrameDemand {
    pub invalidated: bool,
    pub first_frame: bool,
    pub input_queued: bool,
    pub frame_ready: bool,
    pub live_build: bool,
    pub presentation_pending: bool,
    pub surface_resize: bool,
    pub snapshot_retained: bool,
    pub component_close: bool,
    pub cursor_wake: bool,
    pub runtime_pending: bool,
}

impl FrameDemand {
    fn flags(self) -> [(&'static str, bool); 11] {
        [
            ("invalidated", self.invalidated),
            ("first-frame", self.first_frame),
            ("input-queued", self.input_queued),
            ("frame-ready", self.frame_ready),
            ("live-build", self.live_build),
            ("presentation-pending", self.presentation_pending),
            ("surface-resize", self.surface_resize),
            ("snapshot-retained", self.snapshot_retained),
            ("component-close", self.component_close),
            ("cursor-wake", self.cursor_wake),
            ("runtime-pending", self.runtime_pending),
        ]
    }

    pub fn is_demanded(self) -> bool {
        self.flags().iter().any(|(_, set)| *set)
    }

    pub fn reasons(self) -> String {
        let reasons: Vec<&str> = self.flags().iter().filter(|(_, set)| *set).map(|(name, _)| *name).collect();
        if reasons.is_empty() { "none".to_string() } else { reasons.join("+") }
    }
}

/// 🩺️ The one `[TRACE] frame build admitted` line: emitted exactly once per admitted build, never per tick.
pub fn frame_build_admitted_line(generation: u64, demand: FrameDemand) -> String {
    format!("[TRACE] frame build admitted generation={generation} reason={}", demand.reasons())
}
