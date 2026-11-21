//! 🧲 Resolves the chooser effect to its exact mounted application before any native I/O.
use super::*;
use semio_renderer_wgpu_program_bridge::NativeOriginalMediaRequest;

impl WindowActionsCtx<'_> {
    /// 🧾 Folds native transport progress, persistence effects and completed image history into the frame pump.
    pub async fn poll_original_media(&mut self) -> bool {
        let receipts = self.opening.poll_original_media(&self.docs, &self.plugins);
        let changed = !receipts.is_empty();
        for receipt in receipts {
            let scope = self.docs.host_effect_earned_scope(&receipt.effects);
            self.queue_host_effects(&receipt.controller, receipt.effects);
            self.settle.owe_refresh(scope);
            match receipt.result {
                Some(Ok(true)) => {
                    self.as_session().reread_history_projection().await;
                    self.settle.owe_refresh(UiDirtyScope::Full);
                }
                Some(Err(error)) => { self.chrome.show_transient_notice(error, semio_framework::Severity::Error, Some("import-failed")); }
                _ => {}
            }
        }
        changed
    }
    pub(super) fn queue_original_media(&mut self, controller: &str, args: Option<&DslValue>) -> bool {
        let Some(args) = args.filter(|args| args.get("originalMedia").is_some()) else { return false; };
        let found = self.docs.session.iter().chain(self.docs.spawned_session.iter())
            .filter(|session| session.app.controller_id == controller)
            .find_map(|session| NativeOriginalMediaRequest::parse(args, session.instance_id).ok().flatten().and_then(|request| {
                let program = self.plugins.plugins.iter().find(|program| program.plugin_id == session.plugin_id)?.clone();
                Some((program, request))
            }));
        let Some((program, request)) = found else {
            self.chrome.show_transient_notice("original-media.invalid-issued-request",
                semio_framework::Severity::Error, Some("import-failed"));
            return true;
        };
        self.opening.queue_original_media(program, controller.to_owned(), request);
        true
    }
}
