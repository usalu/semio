//! 📤️ Frame-pumped icon export batches with request-owned cleanup before delivery.
use super::*;
use crate::scenes::icon_export::{asset::IconExportAssetRequest, IconExportFault, IconExportFormat, IconExportPreparedScene, IconExportScenePreparation, IconExportSceneRejected, IconGpuPngExport, IconGpuPngRejected, IconSvgExport, IconSvgRejected};
use semio_framework::kernel::IconRenderExportItem;
use semio_framework_async::CancelToken;
use std::collections::LinkedList;
use ui_gpu::GpuContext;

pub(super) const CONTROL_ID: &str = "shell.icon-export.cancel";

pub(super) struct IconExportBatch {
    pub(super) items: LinkedList<IconExportItems>,
    pub(super) total: usize,
    pub(super) completed: usize,
    pub(super) failed: usize,
    pub(super) discarded: usize,
    pub(super) current: usize,
    pub(super) filename: String,
    pub(super) request: String,
    pub(super) format: IconExportFormat,
    pub(super) phase: Phase,
    pub(super) asset: Option<IconExportAssetRequest>,
    preparing: Option<Box<IconExportScenePreparation>>,
    scene_rejected: Option<IconExportSceneRejected>,
    pub(super) scene: Option<Box<IconExportPreparedScene>>,
    initializing: Option<ShellDetached<Result<GpuContext, String>>>,
    pub(super) source: Option<Box<GpuContext>>,
    pub(super) rendering: Option<Box<IconGpuPngExport>>,
    gpu_rejected: Option<IconGpuPngRejected>,
    pub(super) vector: Option<IconSvgExport>,
    vector_rejected: Option<IconSvgRejected>,
    saving: Option<ShellDetached<Result<bool, String>>>,
    pub(super) bytes: Option<Vec<u8>>,
    item_fault: Option<IconExportFault>,
    last_fault: Option<IconExportFault>,
    pub(super) cancelled: bool,
    pub(super) cancel: CancelToken,
    admission_cancel: CancelToken,
}

impl IconExportBatch {
    pub(super) fn new(items: Vec<IconRenderExportItem>) -> Self {
        let total = items.len();
        let cancel = CancelToken::root_now();
        let items = LinkedList::from([IconExportItems { items: items.into_iter(), cancel: cancel.clone() }]);
        Self {
            total, items, completed: 0, failed: 0, discarded: 0, current: 0, filename: String::new(), request: String::new(),
            format: IconExportFormat::Png, phase: Phase::Next, asset: None, preparing: None, scene_rejected: None, scene: None, initializing: None, source: None,
            rendering: None, gpu_rejected: None, vector: None, vector_rejected: None, saving: None, bytes: None, item_fault: None, last_fault: None, cancelled: false, admission_cancel: cancel.clone(), cancel,
        }
    }

    pub(super) fn append(&mut self, items: Vec<IconRenderExportItem>) {
        if self.admission_cancel.is_cancelled_now() { self.admission_cancel = CancelToken::root_now(); }
        self.total += items.len();
        self.items.push_back(IconExportItems { items: items.into_iter(), cancel: self.admission_cancel.clone() });
    }

    pub(super) fn running(&self) -> bool {
        !matches!(self.phase, Phase::Complete | Phase::Cancelled | Phase::Failed)
    }

    pub(super) fn cancel(&mut self) {
        self.cancelled = true;
        self.cancel.cancel_now();
        self.admission_cancel.cancel_now();
        self.bytes = None;
        if self.running() && self.phase != Phase::Next {
            self.phase = Phase::Retire;
        }
    }

    fn fail(&mut self, fault: impl Into<IconExportFault>) {
        let fault=fault.into();
        if self.item_fault.is_none() {
            debug_log(&format!("[TRACE] wgpu icon export refused name={} reason={fault}", self.filename));
            self.item_fault = Some(fault.into());
        }
        self.bytes = None;
        self.phase = Phase::Retire;
    }

    pub(super) fn advance(&mut self,cx:&mut semio_framework_job::StepContext<'_>) -> Result<(),semio_framework_job::OriginalTurnFailure> {
        if cx.should_yield() { return Ok(()); }
        let result=self.advance_one(cx);
        if !cx.should_yield(){cx.consume_fuel(1)}
        result
    }

    fn advance_one(&mut self,cx:&mut semio_framework_job::StepContext<'_>) -> Result<(),semio_framework_job::OriginalTurnFailure> {
        match self.phase {
            Phase::Next => self.next(),
            Phase::Load => {
                let Some(answer) = self.asset.as_mut().and_then(IconExportAssetRequest::take_ready) else { return Ok(()) };
                self.asset = None;
                match answer {
                    Ok(asset) if self.format == IconExportFormat::Svg => match IconSvgExport::new(&self.request, asset) {
                        Ok(vector) => { self.vector = Some(vector); self.phase = Phase::Vector; }
                        Err(rejected) => { self.fail(rejected.fault().to_string()); self.vector_rejected = Some(rejected); }
                    },
                    Ok(asset) => match IconExportScenePreparation::new(std::mem::take(&mut self.request), asset) {
                        Ok(preparation) => { self.preparing = Some(Box::new(preparation)); self.phase = Phase::Prepare; }
                        Err(rejected) => { self.fail(rejected.fault().to_string()); self.scene_rejected = Some(rejected); }
                    },
                    Err(fault) => self.fail(fault),
                }
            }
            Phase::Prepare => {
                let Some(preparation) = self.preparing.as_mut() else { self.fail("icon export preparation was missing"); return Ok(()) };
                match preparation.advance(cx) {
                    Ok(true) => {
                        self.scene = preparation.take_prepared().map(Box::new);
                        if self.scene.is_some() { self.phase = Phase::Device; } else { self.fail("icon export scene was missing"); }
                    }
                    Ok(false) => {}
                    Err(IconExportFault::Original(failure)) => return Err(failure),
                    Err(fault) => self.fail(fault),
                }
            }
            Phase::Device => self.device(),
            Phase::Render => {
                let Some(rendering) = self.rendering.as_mut() else { self.fail("icon export render was missing"); return Ok(()) };
                match rendering.advance(cx) {
                    Ok(true) => {
                        self.bytes = rendering.take_png();
                        self.phase = Phase::Retire;
                        if self.bytes.is_none() { self.fail("icon export produced no PNG"); }
                    }
                    Ok(false) => {}
                    Err(IconExportFault::Original(failure)) => return Err(failure),
                    Err(fault) => self.fail(fault),
                }
            }
            Phase::Vector => {
                let Some(vector) = self.vector.as_mut() else { self.fail("icon export vector renderer was missing"); return Ok(()) };
                match vector.advance() {
                    Ok(true) => {
                        self.bytes = vector.take_svg();
                        self.phase = Phase::Retire;
                        if self.bytes.is_none() { self.fail("icon export produced no SVG"); }
                    }
                    Ok(false) => {}
                    Err(fault) => self.fail(fault),
                }
            }
            Phase::Retire => { self.retire(cx)?; }
            Phase::Save => {
                let Some(answer) = self.saving.as_ref().and_then(ShellDetached::take) else { return Ok(()) };
                self.saving = None;
                match answer {
                    Ok(true) => {
                        self.completed += 1;
                        debug_log(&format!("[TRACE] wgpu icon export delivered name={} completed={}/{}", self.filename, self.completed, self.total));
                    }
                    Ok(false) => self.cancel(),
                    Err(fault) => { self.item_fault = Some(fault.into()); }
                }
                self.finish_item();
            }
            Phase::Complete | Phase::Cancelled | Phase::Failed => {}
        }
        Ok(())
    }

    pub(super) fn next(&mut self) {
        let Some(group) = self.items.front_mut() else {
            self.source = None;
            self.phase = if self.cancelled { Phase::Cancelled } else if self.failed > 0 { Phase::Failed } else { Phase::Complete };
            return;
        };
        let Some(item) = group.items.next() else { self.items.pop_front(); return; };
        self.current += 1;
        if group.cancel.is_cancelled_now() { self.discarded += 1; return; }
        self.cancel = group.cancel.clone();
        self.cancelled = false;
        self.filename = item.filename;
        self.request = super::dsl_value_as_json(&item.request).to_string();
        self.format = match IconExportScenePreparation::request_format(&self.request) {
            Ok(format) => format,
            Err(fault) => return self.fail(fault),
        };
        match IconExportScenePreparation::asset_url(&self.request).and_then(|url| IconExportAssetRequest::new(&url)) {
            Ok(asset) => { self.asset = Some(asset); self.phase = Phase::Load; }
            Err(fault) => self.fail(fault),
        }
    }

    fn device(&mut self) {
        let Some(scene) = self.scene.as_mut() else { return self.fail("icon export scene was missing") };
        if self.source.is_none() {
            if let Some(initializing) = self.initializing.as_ref() {
                let Some(answer) = initializing.take() else { return };
                self.initializing = None;
                match answer {
                    Ok(source) => self.source = Some(Box::new(source)),
                    Err(fault) => return self.fail(fault),
                }
            } else {
                self.initializing = Some(ShellDetached::spawn(GpuContext::headless(1, 1)));
                return;
            }
        }
        let (width, height) = scene.dimensions();
        let Some(packet) = scene.take_packet() else { return self.fail("icon export packet was already transferred") };
        match IconGpuPngExport::new(self.source.as_deref().expect("accepted device"), width, height, packet) {
            Ok(rendering) => { self.rendering = Some(Box::new(rendering)); self.phase = Phase::Render; }
            Err(rejected) => { self.fail(rejected.fault.clone()); self.gpu_rejected = Some(rejected); }
        }
    }

    pub(super) fn retire(&mut self,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool,semio_framework_job::OriginalTurnFailure> {
        if let Some(vector) = self.vector.as_mut() {
            if !vector.close_step() { return Ok(false); }
            self.vector = None;
            return Ok(false);
        }
        if let Some(rejected) = self.vector_rejected.as_mut() {
            if !rejected.close_step() { return Ok(false); }
            self.vector_rejected = None;
            return Ok(false);
        }
        if let Some(saving) = self.saving.as_ref() {
            let Some(answer) = saving.take() else { return Ok(false) };
            self.saving = None;
            if matches!(answer, Ok(true)) { self.completed += 1; }
            return Ok(false);
        }
        if let Some(initializing) = self.initializing.as_ref() {
            let Some(answer) = initializing.take() else { return Ok(false) };
            self.initializing = None;
            if let Ok(source) = answer {
                self.source = Some(Box::new(source));
            }
            return Ok(false);
        }
        if let Some(rendering) = self.rendering.as_mut() {
            if !rendering.terminal() {
                rendering.cancel();
                match rendering.advance(cx){Err(IconExportFault::Original(failure))=>return Err(failure),Err(fault)=>self.last_fault=Some(fault),Ok(_)=>{}}
                return Ok(false);
            }
            self.rendering = None;
            return Ok(false);
        }
        if let Some(rejected) = self.gpu_rejected.as_mut() {
            match rejected.close_step(cx){Ok(true)=>{},Ok(false)=>return Ok(false),Err(IconExportFault::Original(failure))=>return Err(failure),Err(fault)=>{self.last_fault=Some(fault);return Ok(false)}}
            self.gpu_rejected = None;
            return Ok(false);
        }
        if let Some(preparing) = self.preparing.as_mut() {
            preparing.cancel();
            match preparing.advance(cx) { Err(IconExportFault::Original(failure))=>return Err(failure),Err(fault)=>self.last_fault=Some(fault),Ok(_)=>{} }
            if !preparing.terminal() { return Ok(false); }
            self.preparing = None;
            return Ok(false);
        }
        if let Some(rejected) = self.scene_rejected.as_mut() {
            if !rejected.close_step() { return Ok(false); }
            self.scene_rejected = None;
            return Ok(false);
        }
        if let Some(asset) = self.asset.as_mut() {
            if !asset.close_step() { return Ok(false); }
            self.asset = None;
            return Ok(false);
        }
        if let Some(scene) = self.scene.as_mut() {
            scene.begin_close();
            match scene.close_step(cx) {
                Ok(true)=>{},
                Ok(false)=>return false,
                Err(IconExportFault::Original(failure))=>return Err(failure),
                Err(fault)=>{self.last_fault=Some(fault);return Ok(false);}
            }
            self.scene = None;
            return Ok(false);
        }
        if !self.cancelled && self.item_fault.is_none() {
            if let Some(bytes) = self.bytes.take() {
                let filename = self.filename.clone();
                let cancel = self.cancel.clone();
                self.saving = Some(ShellDetached::spawn(present_media_export_bytes_cancellable(filename, self.mime().into(), bytes, cancel)));
                self.phase = Phase::Save;
                return Ok(true);
            }
        }
        self.bytes = None;
        self.finish_item();
        Ok(true)
    }

    fn finish_item(&mut self) {
        if let Some(fault) = self.item_fault.take() {
            self.failed += 1;
            self.last_fault = Some(fault.into());
        }
        self.request.clear();
        self.phase = Phase::Next;
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        !self.running() && self.items.is_empty() && self.asset.is_none() && self.preparing.is_none() && self.scene_rejected.is_none()
            && self.scene.is_none() && self.initializing.is_none() && self.source.is_none() && self.rendering.is_none()
            && self.gpu_rejected.is_none() && self.vector.is_none() && self.vector_rejected.is_none() && self.saving.is_none() && self.bytes.is_none()
    }

    pub(super) fn mime(&self) -> &'static str {
        match self.format { IconExportFormat::Png => "image/png", IconExportFormat::Svg => "image/svg+xml" }
    }

    pub(super) fn phase_key(&self) -> &'static str {
        match self.phase {
            Phase::Next | Phase::Load => "loading",
            Phase::Prepare => "preparing",
            Phase::Device | Phase::Render | Phase::Vector => "rendering",
            Phase::Retire => "closing",
            Phase::Save => "saving",
            Phase::Complete => "complete",
            Phase::Cancelled => "cancelled",
            Phase::Failed => "failed",
        }
    }

    pub(super) fn message(&self, is_de: bool) -> String {
        let phase_key = match self.phase_key() {
            "loading" => "icon.export.loading", "preparing" => "icon.export.preparing", "rendering" => "icon.export.rendering",
            "closing" => "icon.export.closing", "saving" => "icon.export.saving", "complete" => "icon.export.complete",
            "cancelled" => "icon.export.cancelled", _ => "icon.export.failed",
        };
        let phase = shell_chrome_string(phase_key, is_de);
        let step = self.current.min(self.total);
        let detail = self.rendering.as_deref().map(IconGpuPngExport::progress)
            .or_else(|| self.preparing.as_deref().map(IconExportScenePreparation::progress))
            .or_else(|| self.vector.as_ref().map(|vector| { let (phase, done, total) = vector.progress(); (phase, done, total, 0) }));
        let progress = detail.filter(|(_, _, total, _)| *total > 1).map(|(_, done, total, _)| format!(" · {done}/{total}")).unwrap_or_default();
        let status = format!("{phase} {step}/{} · {}{progress}", self.total, self.filename);
        if self.phase == Phase::Failed {
            format!("{status}: {}", self.last_fault.as_ref().map(ToString::to_string).unwrap_or_default())
        } else {
            status
        }
    }

    pub(super) fn counts(&self) -> (usize, usize) { (self.completed + self.failed + self.discarded, self.total) }
}


#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../../🖼️IconRenderHost/🧪️tests/📤️export-batch/🦀️.rs"]
mod tests;
