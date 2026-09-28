//! 📤️ Frame-pumped icon export batches with request-owned cleanup before delivery.
use super::{present_media_export_bytes_cancellable, shell_chrome_string, ShellDetached, ShellState};
use crate::scenes::icon_export::{asset::IconExportAssetRequest, IconExportFormat, IconExportPreparedScene, IconExportScenePreparation, IconExportSceneRejected, IconGpuPngExport, IconGpuPngRejected, IconSvgExport, IconSvgRejected};
use semio_framework::kernel::IconRenderExportItem;
use semio_framework_async::CancelToken;
use std::collections::LinkedList;
use ui_wgpu::wgpu::GpuContext;

pub(super) const CONTROL_ID: &str = "shell.icon-export.cancel";

struct IconExportItems {
    items: std::vec::IntoIter<IconRenderExportItem>,
    cancel: CancelToken,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase { Next, Load, Prepare, Device, Render, Vector, Retire, Save, Complete, Cancelled, Failed }

pub(super) struct IconExportBatch {
    items: LinkedList<IconExportItems>,
    total: usize,
    completed: usize,
    failed: usize,
    discarded: usize,
    current: usize,
    filename: String,
    request: String,
    format: IconExportFormat,
    phase: Phase,
    asset: Option<IconExportAssetRequest>,
    preparing: Option<IconExportScenePreparation>,
    scene_rejected: Option<IconExportSceneRejected>,
    scene: Option<IconExportPreparedScene>,
    initializing: Option<ShellDetached<Result<GpuContext, String>>>,
    source: Option<GpuContext>,
    rendering: Option<IconGpuPngExport>,
    gpu_rejected: Option<IconGpuPngRejected>,
    vector: Option<IconSvgExport>,
    vector_rejected: Option<IconSvgRejected>,
    saving: Option<ShellDetached<Result<bool, String>>>,
    bytes: Option<Vec<u8>>,
    item_fault: Option<String>,
    last_fault: Option<String>,
    cancelled: bool,
    cancel: CancelToken,
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

    fn fail(&mut self, fault: String) {
        if self.item_fault.is_none() {
            ShellState::debug_log(&format!("[DEBUG] wgpu icon export refused name={} reason={fault}", self.filename));
            self.item_fault = Some(fault);
        }
        self.bytes = None;
        self.phase = Phase::Retire;
    }

    pub(super) fn advance(&mut self) {
        match self.phase {
            Phase::Next => self.next(),
            Phase::Load => {
                let Some(answer) = self.asset.as_mut().and_then(IconExportAssetRequest::take_ready) else { return };
                self.asset = None;
                match answer {
                    Ok(asset) if self.format == IconExportFormat::Svg => match IconSvgExport::new(&self.request, asset) {
                        Ok(vector) => { self.vector = Some(vector); self.phase = Phase::Vector; }
                        Err(rejected) => { self.fail(rejected.fault().to_string()); self.vector_rejected = Some(rejected); }
                    },
                    Ok(asset) => match IconExportScenePreparation::new(std::mem::take(&mut self.request), asset) {
                        Ok(preparation) => { self.preparing = Some(preparation); self.phase = Phase::Prepare; }
                        Err(rejected) => { self.fail(rejected.fault().to_string()); self.scene_rejected = Some(rejected); }
                    },
                    Err(fault) => self.fail(fault),
                }
            }
            Phase::Prepare => {
                let Some(preparation) = self.preparing.as_mut() else { return self.fail("icon export preparation was missing".into()) };
                match preparation.advance() {
                    Ok(true) => {
                        self.scene = preparation.take_prepared();
                        self.preparing = None;
                        if self.scene.is_some() { self.phase = Phase::Device; } else { self.fail("icon export scene was missing".into()); }
                    }
                    Ok(false) => {}
                    Err(fault) => self.fail(fault),
                }
            }
            Phase::Device => self.device(),
            Phase::Render => {
                let Some(rendering) = self.rendering.as_mut() else { return self.fail("icon export render was missing".into()) };
                match rendering.advance() {
                    Ok(true) => {
                        self.bytes = rendering.take_png();
                        self.phase = Phase::Retire;
                        if self.bytes.is_none() { self.fail("icon export produced no PNG".into()); }
                    }
                    Ok(false) => {}
                    Err(fault) => self.fail(fault),
                }
            }
            Phase::Vector => {
                let Some(vector) = self.vector.as_mut() else { return self.fail("icon export vector renderer was missing".into()) };
                match vector.advance() {
                    Ok(true) => {
                        self.bytes = vector.take_svg();
                        self.phase = Phase::Retire;
                        if self.bytes.is_none() { self.fail("icon export produced no SVG".into()); }
                    }
                    Ok(false) => {}
                    Err(fault) => self.fail(fault),
                }
            }
            Phase::Retire => { self.retire(); }
            Phase::Save => {
                let Some(answer) = self.saving.as_ref().and_then(ShellDetached::take) else { return };
                self.saving = None;
                match answer {
                    Ok(true) => {
                        self.completed += 1;
                        ShellState::debug_log(&format!("[DEBUG] wgpu icon export delivered name={} completed={}/{}", self.filename, self.completed, self.total));
                    }
                    Ok(false) => self.cancel(),
                    Err(fault) => { self.item_fault = Some(fault); }
                }
                self.finish_item();
            }
            Phase::Complete | Phase::Cancelled | Phase::Failed => {}
        }
    }

    fn next(&mut self) {
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
        let Some(scene) = self.scene.as_mut() else { return self.fail("icon export scene was missing".into()) };
        if self.source.is_none() {
            if let Some(initializing) = self.initializing.as_ref() {
                let Some(answer) = initializing.take() else { return };
                self.initializing = None;
                match answer {
                    Ok(source) => self.source = Some(source),
                    Err(fault) => return self.fail(fault),
                }
            } else {
                self.initializing = Some(ShellDetached::spawn(GpuContext::headless(1, 1)));
                return;
            }
        }
        let (width, height) = scene.dimensions();
        let Some(packet) = scene.take_packet() else { return self.fail("icon export packet was already transferred".into()) };
        match IconGpuPngExport::new(self.source.as_ref().expect("accepted device"), width, height, packet) {
            Ok(rendering) => { self.rendering = Some(rendering); self.phase = Phase::Render; }
            Err(rejected) => { self.fail(rejected.fault.clone()); self.gpu_rejected = Some(rejected); }
        }
    }

    fn retire(&mut self) -> bool {
        if let Some(vector) = self.vector.as_mut() {
            if !vector.close_step() { return false; }
            self.vector = None;
            return false;
        }
        if let Some(rejected) = self.vector_rejected.as_mut() {
            if !rejected.close_step() { return false; }
            self.vector_rejected = None;
            return false;
        }
        if let Some(saving) = self.saving.as_ref() {
            let Some(answer) = saving.take() else { return false };
            self.saving = None;
            if matches!(answer, Ok(true)) { self.completed += 1; }
            return false;
        }
        if let Some(initializing) = self.initializing.as_ref() {
            let Some(answer) = initializing.take() else { return false };
            self.initializing = None;
            if let Ok(source) = answer { self.source = Some(source); }
            return false;
        }
        if let Some(rendering) = self.rendering.as_mut() {
            if !rendering.terminal() {
                rendering.cancel();
                if let Err(fault) = rendering.advance() { self.last_fault = Some(fault); }
                return false;
            }
            self.rendering = None;
            return false;
        }
        if let Some(rejected) = self.gpu_rejected.as_mut() {
            if !rejected.close_step() { return false; }
            self.gpu_rejected = None;
            return false;
        }
        if let Some(preparing) = self.preparing.as_mut() {
            preparing.cancel();
            if let Err(fault) = preparing.advance() { self.last_fault = Some(fault); }
            if !preparing.terminal() { return false; }
            self.preparing = None;
            return false;
        }
        if let Some(rejected) = self.scene_rejected.as_mut() {
            if !rejected.close_step() { return false; }
            self.scene_rejected = None;
            return false;
        }
        if let Some(asset) = self.asset.as_mut() {
            if !asset.close_step() { return false; }
            self.asset = None;
            return false;
        }
        if let Some(scene) = self.scene.as_mut() {
            scene.begin_close();
            if !scene.close_step() { return false; }
            self.scene = None;
            return false;
        }
        if !self.cancelled && self.item_fault.is_none() {
            if let Some(bytes) = self.bytes.take() {
                let filename = self.filename.clone();
                let cancel = self.cancel.clone();
                self.saving = Some(ShellDetached::spawn(present_media_export_bytes_cancellable(filename, self.mime().into(), bytes, cancel)));
                self.phase = Phase::Save;
                return true;
            }
        }
        self.bytes = None;
        self.finish_item();
        true
    }

    fn finish_item(&mut self) {
        if let Some(fault) = self.item_fault.take() {
            self.failed += 1;
            self.last_fault = Some(fault);
        }
        self.request.clear();
        self.phase = Phase::Next;
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        !self.running() && self.items.is_empty() && self.asset.is_none() && self.preparing.is_none() && self.scene_rejected.is_none()
            && self.scene.is_none() && self.initializing.is_none() && self.source.is_none() && self.rendering.is_none()
            && self.gpu_rejected.is_none() && self.vector.is_none() && self.vector_rejected.is_none() && self.saving.is_none() && self.bytes.is_none()
    }

    fn mime(&self) -> &'static str {
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
        let detail = self.rendering.as_ref().map(IconGpuPngExport::progress)
            .or_else(|| self.preparing.as_ref().map(IconExportScenePreparation::progress))
            .or_else(|| self.vector.as_ref().map(|vector| { let (phase, done, total) = vector.progress(); (phase, done, total, 0) }));
        let progress = detail.filter(|(_, _, total, _)| *total > 1).map(|(_, done, total, _)| format!(" · {done}/{total}")).unwrap_or_default();
        let status = format!("{phase} {step}/{} · {}{progress}", self.total, self.filename);
        if self.phase == Phase::Failed {
            format!("{status}: {}", self.last_fault.as_deref().unwrap_or_default())
        } else {
            status
        }
    }

    pub(super) fn counts(&self) -> (usize, usize) { (self.completed + self.failed + self.discarded, self.total) }
}


impl ShellState {
    pub(super) fn enqueue_icon_export(&mut self, items: Vec<IconRenderExportItem>) {
        if items.is_empty() { return; }
        if let Some(batch) = self.icon_export.as_mut().filter(|batch| batch.running()) {
            batch.append(items);
        } else {
            self.icon_export = Some(IconExportBatch::new(items));
        }
    }

    pub(super) fn advance_icon_export(&mut self) {
        if let Some(batch) = self.icon_export.as_mut().filter(|batch| batch.running()) {
            let started = super::chrome_now_ms();
            for _ in 0..32 {
                batch.advance();
                if !batch.running() || super::chrome_now_ms() - started >= 2.0 { break; }
            }
        }
    }

    pub(super) fn cancel_icon_export(&mut self) {
        if let Some(batch) = self.icon_export.as_mut() {
            if batch.running() { batch.cancel(); } else { self.icon_export = None; }
        }
    }

    pub(crate) fn close_icon_export_step(&mut self) -> bool {
        let Some(batch) = self.icon_export.as_mut() else { return true };
        batch.cancel();
        batch.advance();
        if !batch.terminal_is_empty() { return false; }
        self.icon_export = None;
        true
    }

    pub(super) fn render_icon_export_step(&mut self, cursor: &mut super::ShellChromeChildCursor, overlay: &mut ui_wgpu::wgpu::DrawList, atlas: &mut ui_wgpu::wgpu::FontAtlas, input: &mut ui_wgpu::wgpu::InputState<ui_wgpu::wgpu::ActionDescriptor>, theme: &ui_wgpu::wgpu::Theme, width: f32) -> bool {
        let Some(batch) = self.icon_export.as_ref() else { return true };
        let is_de = self.locale_id == "de";
        let action_label = shell_chrome_string(if batch.running() { "icon.export.cancel" } else { "common.close" }, is_de);
        let message = batch.message(is_de);
        let severity = if batch.phase == Phase::Failed { semio_framework::Severity::Error } else { semio_framework::Severity::Info };
        let (border, fill, text) = super::transient_notice_tone(severity, theme);
        let mut band = super::document_opening_rect(&message, action_label, width, theme);
        band.y += band.h + super::PLUGIN_INSTALL_TOP_GAP;
        let action = super::plugin_install_action_rect(band, action_label, theme);
        let hair = theme.stroke_hairline;
        match cursor.scalar {
            0 => overlay.push_rounded([band.x, band.y, band.w, band.h], fill, theme.border_radius),
            1..=4 => {
                let edge = match cursor.scalar {
                    1 => [band.x, band.y, band.w, hair],
                    2 => [band.x, band.y + band.h - hair, band.w, hair],
                    3 => [band.x, band.y, hair, band.h],
                    _ => [band.x + band.w - hair, band.y, hair, band.h],
                };
                overlay.push_solid(edge, border);
            }
            5 | 6 => {
                let (value, x, max_width) = if cursor.scalar == 5 {
                    (message.as_str(), band.x + theme.padding_standard, (action.x - band.x - theme.padding_standard * 2.0).max(1.0))
                } else { (action_label, action.x, action.w.max(1.0)) };
                let baseline = band.y + (band.h + theme.font_size_small) * 0.5 - 1.0;
                match super::chrome_text_complete_step(overlay, atlas, value, x, baseline, max_width, theme.font_size_small, text, &mut cursor.glyph) {
                    Ok(false) => return false,
                    Ok(true) => {}
                    Err(()) => cursor.glyph.reset(),
                }
            }
            7 => {
                super::note_chrome_control_name(CONTROL_ID, Some(action_label));
                input.register_hit(ui_wgpu::wgpu::HitTarget { rect: action, event: None, control_id: Some(CONTROL_ID.into()), kind: ui_wgpu::wgpu::HitKind::Button, drag_axis: None, drag_data: None });
            }
            8 => {
                if self.chrome_build.clicked_this_frame && action.contains(input.pointer_x, input.pointer_y) { self.cancel_icon_export(); }
            }
            _ => return true,
        }
        cursor.scalar += 1;
        false
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../../🖼️IconRenderHost/🧪️tests/📤️export-batch/🦀️.rs"]
mod tests;
