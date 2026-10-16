//! 📂️ `shell-doc-open` staging of `icon-export`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;
use super::icon_export::*;
use super::{present_media_export_bytes_cancellable, shell_chrome_string, ShellDetached, ShellState};
use crate::scenes::icon_export::{asset::IconExportAssetRequest, IconExportFault, IconExportFormat, IconExportPreparedScene, IconExportScenePreparation, IconExportSceneRejected, IconGpuPngExport, IconGpuPngRejected, IconSvgExport, IconSvgRejected};
use semio_framework::kernel::IconRenderExportItem;
use semio_framework_async::CancelToken;

pub(super) struct IconExportItems {
    pub(super) items: std::vec::IntoIter<IconRenderExportItem>,
    pub(super) cancel: CancelToken,
}

impl OpeningState {
    pub(super) fn enqueue_icon_export(&mut self, items: Vec<IconRenderExportItem>) {
        if items.is_empty() { return; }
        if let Some(batch) = self.icon_export.as_mut().filter(|batch| batch.running()) {
            batch.append(items);
        } else {
            self.icon_export = Some(IconExportBatch::new(items));
        }
    }

    pub(super) fn advance_icon_export(&mut self,cx:&mut semio_framework_job::StepContext<'_>) -> Result<(),semio_framework_job::OriginalTurnFailure> {
        if let Some(batch) = self.icon_export.as_mut().filter(|batch| batch.running()) {
            let started = super::chrome_now_ms();
            for _ in 0..32 {
                batch.advance(cx)?;
                if cx.should_yield() || !batch.running() || super::chrome_now_ms() - started >= 2.0 { break; }
            }
        }
        Ok(())
    }

    pub(super) fn cancel_icon_export(&mut self) {
        if let Some(batch) = self.icon_export.as_mut() {
            if batch.running() { batch.cancel(); } else { self.icon_export = None; }
        }
    }

    pub(crate) fn close_icon_export_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool,semio_framework_job::OriginalTurnFailure> {
        let Some(batch) = self.icon_export.as_mut() else { return Ok(true) };
        batch.cancel();
        batch.advance(cx)?;
        if !batch.terminal_is_empty() { return Ok(false); }
        self.icon_export = None;
        Ok(true)
    }
}

