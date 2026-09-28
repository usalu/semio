//! 🧊️ Accepted-packet PNG export with bounded presentation, readback and retirement.
use super::png::IconPngExport;
use ui_wgpu::wgpu::{GpuContext, PreparedGpuPresentCursor, PreparedGpuReadback, PreparedRasterKeepCursorV1, PreparedRasterKeepStepV1, PreparedRenderGate, PreparedRenderPacket};
use ui_wgpu::wgpu::draw::{RasterTextureCleanupStep, RasterTextureWitness};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase {
    Ownership,
    Upload,
    Render,
    Readback,
    Encode,
    CloseCursor,
    AbortRasters,
    CloseRasterUpload,
    CloseMeshUpload,
    CloseRasters,
    CloseMeshes,
    RetirePacket,
    Complete,
}

pub(crate) struct IconGpuPngExport {
    gpu: GpuContext,
    packet: Option<PreparedRenderPacket>,
    witness: RasterTextureWitness,
    keep: PreparedRasterKeepCursorV1,
    upload: usize,
    cursor: Option<PreparedGpuPresentCursor>,
    readback: Option<PreparedGpuReadback>,
    encoder: Option<IconPngExport>,
    phase: Phase,
    bytes: Option<Vec<u8>>,
    fault: Option<String>,
    cancelled: bool,
}

pub(crate) struct IconGpuPngRejected {
    pub(crate) fault: String,
    packet: PreparedRenderPacket,
}

impl IconGpuPngRejected {
    pub(crate) fn close_step(&mut self) -> bool {
        self.packet.retire_step()
    }
}

impl IconGpuPngExport {
    #[expect(clippy::result_large_err, reason = "Refusal returns the exact packet for bounded retirement.")]
    pub(crate) fn new(source: &GpuContext, width: u32, height: u32, packet: PreparedRenderPacket) -> Result<Self, IconGpuPngRejected> {
        let mut gpu = match source.offscreen(width, height) {
            Ok(gpu) => gpu,
            Err(fault) => return Err(IconGpuPngRejected { fault, packet }),
        };
        let gate = PreparedRenderGate::default();
        #[cfg(not(target_arch = "wasm32"))]
        let admitted = gpu.begin_prepared(&ui_wgpu::wgpu::UiPresentToken::mint_for_current_thread(), &gate, &packet, packet.scene_revision(), packet.preview_generation());
        #[cfg(target_arch = "wasm32")]
        let admitted = ui_wgpu::wgpu::OffscreenPresentToken::mint_for_dedicated_worker().map_err(str::to_owned).and_then(|token| gpu.begin_prepared_offscreen(&token, &gate, &packet, packet.scene_revision(), packet.preview_generation()));
        if let Err(fault) = admitted {
            return Err(IconGpuPngRejected { fault, packet });
        }
        let witness = RasterTextureWitness { scene_revision: packet.scene_revision(), preview_generation: packet.preview_generation(), operation: 1 };
        if let Err(fault) = gpu.begin_raster_ownership(witness) {
            return Err(IconGpuPngRejected { fault, packet });
        }
        Ok(Self {
            gpu, packet: Some(packet), witness, keep: PreparedRasterKeepCursorV1::default(), upload: 0,
            cursor: None, readback: None, encoder: None, phase: Phase::Ownership, bytes: None, fault: None, cancelled: false,
        })
    }

    pub(crate) fn advance(&mut self) -> Result<bool, String> {
        if self.phase == Phase::Complete {
            return self.fault.as_ref().map_or(Ok(true), |fault| Err(fault.clone()));
        }
        if let Err(fault) = self.advance_one() {
            if self.fault.is_none() {
                self.fault = Some(fault);
            }
            if matches!(self.phase, Phase::Ownership | Phase::Upload | Phase::Render | Phase::Readback | Phase::Encode) {
                self.discard_candidate();
                self.phase = Phase::CloseCursor;
            } else {
                return Err(self.fault.clone().unwrap_or_default());
            }
        }
        if self.phase == Phase::Complete { self.fault.as_ref().map_or(Ok(true), |fault| Err(fault.clone())) } else { Ok(false) }
    }

    fn advance_one(&mut self) -> Result<(), String> {
        match self.phase {
            Phase::Ownership => {
                let packet = self.packet.as_ref().ok_or("icon export packet was missing")?;
                match packet.raster_keep_step(&mut self.keep) {
                    PreparedRasterKeepStepV1::Pending => {}
                    PreparedRasterKeepStepV1::Key(key) => self.gpu.publish_raster_ownership(self.witness, key)?,
                    PreparedRasterKeepStepV1::Complete => {
                        self.gpu.seal_raster_ownership(self.witness)?;
                        self.phase = Phase::Upload;
                    }
                }
            }
            Phase::Upload => {
                let packet = self.packet.as_ref().ok_or("icon export packet was missing")?;
                if self.upload == packet.uploads().len() {
                    self.phase = Phase::Render;
                } else if self.gpu.apply_prepared_upload_step(packet, self.upload, self.witness, self.witness)? {
                    self.upload += 1;
                }
            }
            Phase::Render => {
                let packet = self.packet.as_ref().ok_or("icon export packet was missing")?;
                if self.cursor.is_none() {
                    self.cursor = Some(self.gpu.begin_prepared_present(packet, self.witness)?);
                    return Ok(());
                }
                let cursor = self.cursor.as_mut().ok_or("icon export presentation was missing")?;
                let done = self.gpu.prepared_present_step(packet, cursor)?;
                if let Some((key, version)) = self.gpu.take_missing_world_mesh() {
                    return Err(format!("icon export mesh was not resident: {key}@{version}"));
                }
                if done {
                    self.readback = Some(self.gpu.begin_prepared_readback(cursor)?);
                    self.phase = Phase::Readback;
                }
            }
            Phase::Readback => {
                let readback = self.readback.as_mut().ok_or("icon export readback was missing")?;
                if self.gpu.prepared_readback_step(readback)? {
                    let (width, height) = readback.dimensions();
                    let rgba = readback.take_rgba().ok_or("icon export pixels were missing")?;
                    self.encoder = Some(IconPngExport::new(width, height, rgba)?);
                    self.readback = None;
                    self.phase = Phase::Encode;
                }
            }
            Phase::Encode => {
                let encoder = self.encoder.as_mut().ok_or("icon export encoder was missing")?;
                if encoder.advance()? {
                    self.bytes = Some(encoder.take_png().ok_or("icon export PNG was missing")?);
                    self.encoder = None;
                    self.phase = Phase::CloseCursor;
                }
            }
            Phase::CloseCursor => {
                if let Some(cursor) = self.cursor.as_mut() {
                    cursor.begin_close();
                    if !cursor.close_step() {
                        return Ok(());
                    }
                    self.cursor = None;
                }
                self.phase = Phase::AbortRasters;
            }
            Phase::AbortRasters => {
                if self.gpu.abort_presented_rasters_step(self.witness)? {
                    self.phase = Phase::CloseRasterUpload;
                }
            }
            Phase::CloseRasterUpload => match self.gpu.close_raster_upload_step() {
                RasterTextureCleanupStep::Complete => self.phase = Phase::CloseMeshUpload,
                RasterTextureCleanupStep::Pending { .. } => {}
                RasterTextureCleanupStep::Blocked(fault) => return Err(fault.into()),
            },
            Phase::CloseMeshUpload => {
                if self.gpu.close_mesh_upload_step() {
                    self.phase = Phase::CloseRasters;
                }
            }
            Phase::CloseRasters => {
                if self.gpu.close_raster_table_step()? {
                    self.phase = Phase::CloseMeshes;
                }
            }
            Phase::CloseMeshes => {
                if self.gpu.close_mesh_table_step() {
                    self.phase = Phase::RetirePacket;
                }
            }
            Phase::RetirePacket => {
                if self.packet.as_mut().is_none_or(PreparedRenderPacket::retire_step) {
                    self.packet = None;
                    self.phase = Phase::Complete;
                }
            }
            Phase::Complete => {}
        }
        Ok(())
    }

    fn discard_candidate(&mut self) {
        if let Some(readback) = self.readback.as_mut() {
            readback.cancel();
        }
        if let Some(encoder) = self.encoder.as_mut() {
            encoder.cancel();
        }
        self.readback = None;
        self.encoder = None;
        self.bytes = None;
    }

    pub(crate) fn cancel(&mut self) {
        self.cancelled = true;
        self.discard_candidate();
        if matches!(self.phase, Phase::Ownership | Phase::Upload | Phase::Render | Phase::Readback | Phase::Encode) {
            self.phase = Phase::CloseCursor;
        }
    }

    pub(crate) fn phase(&self) -> &'static str {
        match self.phase {
            Phase::Ownership => "ownership",
            Phase::Upload => "upload",
            Phase::Render => "render",
            Phase::Readback => "readback",
            Phase::Encode => "encode",
            Phase::Complete => "complete",
            _ => "close",
        }
    }

    pub(crate) fn progress(&self) -> (u8, usize, usize, usize) {
        let (completed, total, detail) = match self.phase {
            Phase::Upload => {
                let inner = self.gpu.prepared_upload_progress();
                (self.upload, self.packet.as_ref().map_or(0, |packet| packet.uploads().len()), inner.0 as usize + inner.1 as usize + inner.2)
            }
            Phase::Render => self.cursor.as_ref().map_or((0, 0, 0), |cursor| {
                let inner = cursor.progress();
                (inner.1, inner.2, inner.0 as usize + inner.3 as usize)
            }),
            Phase::Readback => self.readback.as_ref().map_or((0, 0, 0), |readback| {
                let inner = readback.progress();
                (inner.1, inner.2, inner.0 as usize)
            }),
            Phase::Encode => self.encoder.as_ref().map_or((0, 0, 0), |encoder| {
                let inner = encoder.progress();
                (inner.1, inner.2, inner.0 as usize)
            }),
            _ => (0, 0, 0),
        };
        (self.phase as u8, completed, total, detail)
    }

    pub(crate) fn take_png(&mut self) -> Option<Vec<u8>> {
        if self.phase == Phase::Complete && !self.cancelled && self.fault.is_none() { self.bytes.take() } else { None }
    }

    pub(crate) fn terminal(&self) -> bool {
        self.phase == Phase::Complete
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../../🧪️tests/📤️gpu-export/🦀️.rs"]
mod tests;
