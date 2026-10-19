//! 🛠️ Surface and frame operations of the Flow domain adapter.

use super::*;

impl FlowDomainAdapter {
    pub fn attach_surface(&mut self, args: &FlowArguments) -> Result<Vec<u8>, FlowFailure> {
        let id = SurfaceId::try_new(u32_integer(args, "surface")?).map_err(abi_failure)?;
        let generation = SurfaceGeneration::try_new(u32_integer(args, "surfaceGeneration")?).map_err(abi_failure)?;
        if let Some(current) = self.surface {
            return Err(FlowFailure::new(if current.id == id && current.generation == generation { AbiErrorCode::Busy } else { AbiErrorCode::StaleGeneration }, "Flow surface requires exact close before replacement"));
        }
        let width = u32_integer(args, "width")?;
        let height = u32_integer(args, "height")?;
        let dpr = number(args, "dpr")?;
        let metrics = CanvasMetrics::try_new(width, height, dpr as f32).map_err(abi_failure)?;
        self.width = width;
        self.height = height;
        self.dpr = dpr;
        self.host.set_viewport(width, height, dpr);
        self.surface = Some(FlowSurface { id, generation, metrics, state: SurfaceState::Pending });
        Ok(self.surface_status_bytes())
    }

    pub fn apply_surface_status(&mut self, args: &FlowArguments) -> Result<Vec<u8>, FlowFailure> {
        let id = SurfaceId::try_new(u32_integer(args, "surface")?).map_err(abi_failure)?;
        let generation = SurfaceGeneration::try_new(u32_integer(args, "surfaceGeneration")?).map_err(abi_failure)?;
        let current = self.surface.as_mut().ok_or_else(|| abi_failure(AbiErrorCode::UnknownHandle))?;
        if current.id != id {
            return Err(abi_failure(AbiErrorCode::UnknownHandle));
        }
        if generation.get() < current.generation.get() {
            return Err(abi_failure(AbiErrorCode::AbaHandle));
        }
        if generation != current.generation {
            return Err(abi_failure(AbiErrorCode::StaleGeneration));
        }
        match text(args, "status")? {
            "created" | "recovered" => current.state = SurfaceState::Ready,
            "lost" | "device-lost" => {
                current.state = SurfaceState::Lost;
                surface_canvas::flow_detach_surface_canvas(id.get());
            }
            "cancelled" | "rejected" => {
                self.surface = None;
                surface_canvas::flow_detach_surface_canvas(id.get());
            }
            _ => return Err(abi_failure(AbiErrorCode::MalformedTag)),
        }
        Ok(self.surface_status_bytes())
    }

    pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) -> Result<(), FlowFailure> {
        let metrics = CanvasMetrics::try_new(width, height, dpr as f32).map_err(abi_failure)?;
        self.width = width;
        self.height = height;
        self.dpr = dpr;
        self.host.set_viewport(width, height, dpr);
        if let Some(surface) = self.surface.as_mut() {
            surface.metrics = metrics;
            surface_canvas::flow_resize_surface_canvas(surface.id.get(), width, height, dpr);
        }
        Ok(())
    }

    /// 🖼️ Paints the surface and PRESENTS it — the real `canvas::Scene` this host's own LOD-aware
    /// vector painter produces, not a stand-in for it.
    ///
    /// Two exits, one picture. When the surface's canvas was bound to a WebGPU device
    /// (`flowAttachSurfaceCanvas`, below) the scene is rasterized in-process through the SAME
    /// `CanvasGpuSession` every other browser board session presents with (`DagSession`,
    /// `BoardSession`, the paint/map/editor sessions) and the reply carries an EMPTY draw list —
    /// the pixels are already on screen. With no device the reply carries the scene itself, encoded
    /// by `canvas::draw_list` for the host's 2D-context replayer. `paint_scene` bakes the camera
    /// into every command's affine, so neither exit needs a second transform pipeline.
    pub fn render_frame(&mut self) -> Result<Vec<u8>, FlowFailure> {
        let surface = self.surface.filter(|surface| surface.state == SurfaceState::Ready).ok_or_else(|| abi_failure(AbiErrorCode::UnknownHandle))?;
        self.host.sync_dag_ghost();
        let mut scene = canvas::Scene::new();
        self.host.paint_scene(&mut scene, self.width, self.height, self.dpr);
        let clear = self.host.dag.canvas_theme.raster_clear;
        let presented = surface_canvas::present_surface_scene(surface.id.get(), &scene, clear, self.dpr);
        let fixture = self.host.host_snapshot_json().map_err(domain_error)?;
        let labels = self.host.label_overlay_paint_state_json().map_err(domain_error)?;
        let rgba = clear.to_rgba8();
        let payload = &mut self.frame_payload;
        payload.clear();
        use std::fmt::Write as _;
        let _ = write!(
            payload,
            "{{\"surface\":{},\"surfaceGeneration\":{},\"width\":{},\"height\":{},\"dpr\":{},\"present\":\"{}\",\"clear\":[{},{},{},{}],\"draw\":",
            surface.id.get(),
            surface.generation.get(),
            self.width,
            self.height,
            self.dpr,
            if presented { "gpu" } else { "2d" },
            rgba.r,
            rgba.g,
            rgba.b,
            rgba.a
        );
        if presented {
            payload.push_str(r#"{"version":1,"truncated":false,"commands":[]}"#);
        } else {
            canvas::draw_list::write_scene_draw_list(payload, &scene, canvas::draw_list::DrawListOptions::default());
        }
        payload.push_str(",\"hostDocument\":");
        payload.push_str(&fixture);
        payload.push_str(",\"labels\":");
        payload.push_str(&labels);
        payload.push('}');
        Ok(payload.as_bytes().to_vec())
    }

    pub(super) fn surface_status_bytes(&self) -> Vec<u8> {
        self.surface
            .map(|surface| {
                let status = match surface.state {
                    SurfaceState::Pending => "pending",
                    SurfaceState::Ready => "created",
                    SurfaceState::Lost => "lost",
                };
                format!("{{\"surface\":{},\"surfaceGeneration\":{},\"status\":\"{status}\"}}", surface.id.get(), surface.generation.get()).into_bytes()
            })
            .unwrap_or_else(|| br#"{"status":"cancelled"}"#.to_vec())
    }
}
