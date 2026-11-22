//! 🎞️ The shell-owned tutorial video slot: the one presented media slot the shell itself (not a plugin window) owns, carried by the same typed, bounded, retired descriptor path. See [`super`].

use super::{valid_rect, PresentedMediaSlot, PRESENTED_MEDIA_SLOT_CAPACITY};
use semio_framework_value::{DslValue, Number};
use semio_ui_theme::geometry::Rect;

pub const SHELL_TUTORIAL_VIDEO_TOKEN: &str = "shell.tutorial.video";
pub const SHELL_TUTORIAL_VIDEO_WINDOW: &str = "shell";
pub const SHELL_TUTORIAL_VIDEO_NODE_KEY: &str = "ui.tutorial.video";
pub const SHELL_TUTORIAL_VIDEO_OWNER: &str = "shell";
pub const SHELL_TUTORIAL_VIDEO_CONTROLLER: &str = "shell.tutorial";
pub const SHELL_TUTORIAL_VIDEO_KIND: &str = "tutorialVideo";
pub const SHELL_TUTORIAL_VIDEO_SRC_BYTES: usize = 32_768;

/// 📹️ The slot for one resolved tutorial video cue, `None` when its rect is empty or its source exceeds [`SHELL_TUTORIAL_VIDEO_SRC_BYTES`].
pub fn shell_tutorial_video_slot(rect: Rect, src: &str, muted: bool, playing: bool, rate: f32, local_time_ms: f64) -> Option<PresentedMediaSlot> {
    if !valid_rect(rect) || src.is_empty() || src.len() > SHELL_TUTORIAL_VIDEO_SRC_BYTES || !rate.is_finite() || !local_time_ms.is_finite() {
        return None;
    }
    let props = DslValue::Object(vec![
        ("kind".into(), DslValue::String(SHELL_TUTORIAL_VIDEO_KIND.into())),
        ("src".into(), DslValue::String(src.into())),
        ("muted".into(), DslValue::Bool(muted)),
        ("playing".into(), DslValue::Bool(playing)),
        ("rate".into(), DslValue::Number(Number::Float(f64::from(rate)))),
        ("localTimeMs".into(), DslValue::Number(Number::Float(local_time_ms))),
    ]);
    Some(PresentedMediaSlot {
        token: SHELL_TUTORIAL_VIDEO_TOKEN.into(),
        window_id: SHELL_TUTORIAL_VIDEO_WINDOW.into(),
        node_id: "0".into(),
        node_key: SHELL_TUTORIAL_VIDEO_NODE_KEY.into(),
        rect: rect.into(),
        clip: rect.into(),
        paint_order: 0,
        occluded: false,
        plugin_id: SHELL_TUTORIAL_VIDEO_OWNER.into(),
        controller_id: SHELL_TUTORIAL_VIDEO_CONTROLLER.into(),
        app_instance_id: 0,
        parent_document_id: SHELL_TUTORIAL_VIDEO_OWNER.into(),
        props,
    })
}

/// ➕️ Appends the shell slot after the plugin slots, last in paint order, only while the descriptor ceiling has room.
pub fn push_shell_tutorial_video_slot(slots: &mut Vec<PresentedMediaSlot>, slot: Option<PresentedMediaSlot>) {
    let Some(mut slot) = slot else { return };
    if slots.len() >= PRESENTED_MEDIA_SLOT_CAPACITY {
        return;
    }
    slot.paint_order = slots.len() as u32;
    slots.push(slot);
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
