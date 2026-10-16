//! 🧰️ Test fixtures of the frame actions: the drain every renderer law uses to read the actions an input authority admitted.
//!
//! 🧭️ Behind `testkit`: the monolith test unit held this function in the standalone renderer law file; laws in the test bins name it through this crate.

use super::settle_renderer_action_receipt;
use semio_framework_os_renderer_wgpu_engine_text::text_editor::TextEditorActionOutcome;
use ui_wgpu::wgpu::{ActionDescriptor, InputState};

/// 🧾️ Takes every action out of the bounded input authority, settles its receipt as accepted and returns the descriptors in order.
pub fn collect_fixture_actions(input: &mut InputState<ActionDescriptor>) -> Vec<ActionDescriptor> {
    let mut actions = Vec::new();
    while let Some(action) = input.take_action_step().expect("fixture action authority remains live") {
        let queued = action.into_envelope().expect("bounded fixture action materializes");
        if let Some(receipt) = queued.receipt {
            settle_renderer_action_receipt(receipt, TextEditorActionOutcome::Accepted);
        }
        actions.push(queued.descriptor);
    }
    actions
}
