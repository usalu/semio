//! 🎬️ Request-owned Icon World scene construction and prepared-packet publication.
use semio_framework_job::{InteractiveJob, StepOutcome};
use std::collections::HashMap;
use ui_wgpu::wgpu::{ActionDescriptor, DrawList, FontAtlas, InputState, PreparedRenderInput, PreparedRenderInputRejected, PreparedRenderJob, PreparedRenderJobRejected, PreparedRenderPacket, Rect, Theme};

use infinite_world::world::{
    begin_world3d_dynamic_retirement, close_world3d_draw_rebuild_step, publish_world3d_asset_mesh, step_world3d_draw_rebuild, step_world3d_dynamic_retirement, step_world3d_scene_bridge, step_world3d_snapshot, world3d_cursor_work_pending,
    world3d_dynamic_retirement_terminal_is_empty, World3dBuildContext, World3dBuildRejected, World3dMeshAsset, World3dSceneBridgeStep, World3dSnapshotApplyStep, World3dState, WorldCursorWakeAuthority, WorldDrawRebuildStep,
};

use super::super::{icon_render_shadow_profile, icon_render_world_component_scene, IconRenderFormat, IconRenderRequestFields};

const PREVIEW_GENERATION: u64 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum IconExportFormat {
    Png,
    Svg,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase {
    Paint,
    Bridge,
    Transfer,
    Prepare,
    CloseJob,
    Complete,
    Close,
}

pub(crate) struct IconExportSceneRejected {
    fault: String,
    asset: Option<World3dMeshAsset>,
}

impl std::fmt::Debug for IconExportSceneRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("IconExportSceneRejected").field("fault", &self.fault).field("owns_asset", &self.asset.is_some()).finish()
    }
}

impl IconExportSceneRejected {
    pub(crate) fn fault(&self) -> &str {
        &self.fault
    }

    pub(crate) fn close_step(&mut self) -> bool {
        let Some(asset) = self.asset.as_mut() else { return true };
        if !asset.close_step() {
            return false;
        }
        self.asset = None;
        true
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.asset.is_none()
    }
}

pub(crate) struct IconExportPreparedScene {
    packet: Option<PreparedRenderPacket>,
    state: Option<World3dState>,
    dimensions: (u32, u32),
    format: IconExportFormat,
    closing: bool,
    sequence: u64,
}

impl IconExportPreparedScene {
    pub(crate) fn dimensions(&self) -> (u32, u32) {
        self.dimensions
    }

    pub(crate) fn format(&self) -> IconExportFormat {
        self.format
    }

    /// 📦️ Transfers the accepted packet while retaining its mesh authority in this owner.
    pub(crate) fn take_packet(&mut self) -> Option<PreparedRenderPacket> {
        (!self.closing).then(|| self.packet.take()).flatten()
    }

    pub(crate) fn begin_close(&mut self) {
        self.closing = true;
    }

    pub(crate) fn close_step(&mut self) -> bool {
        self.closing = true;
        if let Some(packet) = self.packet.as_mut() {
            if !packet.retire_step() {
                return false;
            }
            self.packet = None;
            return false;
        }
        let Some(state) = self.state.as_mut() else { return true };
        if state.dynamic_retirement_is_idle() && !world3d_dynamic_retirement_terminal_is_empty(state) {
            begin_world3d_dynamic_retirement(state);
            return false;
        }
        let mut context = scene_step_context(PREVIEW_GENERATION, 1, &mut self.sequence);
        if !step_world3d_dynamic_retirement(state, &mut context) {
            return false;
        }
        self.state = None;
        true
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.packet.is_none() && self.state.is_none()
    }
}

pub(crate) struct IconExportScenePreparation {
    phase: Phase,
    scene: ui_wgpu::wgpu::UiComponentSceneNode,
    state: Option<World3dState>,
    resources: World3dBuildContext,
    input: Option<PreparedRenderInput>,
    input_rejected: Option<PreparedRenderInputRejected>,
    job: Option<PreparedRenderJob>,
    job_rejected: Option<PreparedRenderJobRejected>,
    job_outcome: Option<StepOutcome>,
    packet: Option<PreparedRenderPacket>,
    world_rejected: Option<World3dBuildRejected>,
    dimensions: (u32, u32),
    format: IconExportFormat,
    viewport_mask: ui_wgpu::wgpu::SceneViewportMask3d,
    shadow_profile: infinite_world::world::World3dShadowProfile,
    scene_revision: u64,
    sequence: u64,
    fault: Option<String>,
    cancelled: bool,
}

pub(super) struct ValidatedIconExportRequest {
    pub(super) request: IconRenderRequestFields,
    pub(super) dimensions: (u32, u32),
    pub(super) format: IconExportFormat,
}

impl IconExportScenePreparation {
    pub(crate) fn asset_url(request_json: &str) -> Result<String, String> {
        validate_request(request_json).map(|validated| crate::mesh_assets::mesh_asset_transport_url(&validated.request.asset_url))
    }

    pub(crate) fn request_format(request_json: &str) -> Result<IconExportFormat, String> {
        validate_request(request_json).map(|validated| validated.format)
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact decoded mesh and appearance owners for bounded retirement.")]
    pub(crate) fn new(request_json: String, asset: World3dMeshAsset) -> Result<Self, IconExportSceneRejected> {
        let ValidatedIconExportRequest { request, dimensions: (width, height), format } = match validate_request(&request_json) {
            Ok(validated) => validated,
            Err(fault) => return Err(IconExportSceneRejected { fault, asset: Some(asset) }),
        };
        let shadow_profile = icon_render_shadow_profile(&request);
        let viewport_mask = if request.shape.as_deref() == Some("ellipse") { ui_wgpu::wgpu::SceneViewportMask3d::Ellipse } else { ui_wgpu::wgpu::SceneViewportMask3d::Rectangle };
        let revision = asset.mesh.revision();
        if revision == 0 || revision == u64::MAX {
            return Err(IconExportSceneRejected { fault: "icon export mesh revision was invalid".into(), asset: Some(asset) });
        }
        let subject_bounds = match asset.mesh.aabb() {
            Ok(bounds) => bounds,
            Err(error) => {
                return Err(IconExportSceneRejected { fault: format!("icon export mesh bounds were unavailable: {error:?}"), asset: Some(asset) });
            }
        };
        let asset_url = crate::mesh_assets::mesh_asset_transport_url(&request.asset_url);
        let mesh_id = semio_framework_plugin::world3d_mesh_id_from_url(&asset_url);
        let surface_id = "icon-export-surface".to_string();
        let controller_id = "icon-export-controller".to_string();
        let mut state = World3dState::request_owned(surface_id.clone(), controller_id.clone(), revision);
        if let Err(rejected) = publish_world3d_asset_mesh(&mut state, &asset_url, asset) {
            return Err(IconExportSceneRejected { fault: format!("icon export mesh publication was refused: {:?}", rejected.fault), asset: Some(rejected.value) });
        }
        let scene = icon_render_world_component_scene(&request, &asset_url, &mesh_id, Some(subject_bounds), 1.0, "icon-export-host".into(), surface_id, controller_id);
        Ok(Self {
            phase: Phase::Paint,
            scene,
            state: Some(state),
            resources: World3dBuildContext::new(WorldCursorWakeAuthority::new()),
            input: None,
            input_rejected: None,
            job: None,
            job_rejected: None,
            job_outcome: None,
            packet: None,
            world_rejected: None,
            dimensions: (width, height),
            format,
            viewport_mask,
            shadow_profile,
            scene_revision: revision,
            sequence: 0,
            fault: None,
            cancelled: false,
        })
    }

    pub(crate) fn dimensions(&self) -> (u32, u32) {
        self.dimensions
    }

    pub(crate) fn format(&self) -> IconExportFormat {
        self.format
    }

    pub(crate) fn cancel(&mut self) {
        self.cancelled = true;
        self.phase = Phase::Close;
        if let Some(job) = self.job.as_mut() {
            InteractiveJob::begin_close(job);
        }
    }

    pub(crate) fn advance(&mut self) -> Result<bool, String> {
        if self.phase == Phase::Complete {
            return Ok(true);
        }
        if self.phase == Phase::Close {
            return self.close_step();
        }
        if let Err(fault) = self.advance_one() {
            self.fault = Some(fault.clone());
            self.phase = Phase::Close;
            return Err(fault);
        }
        Ok(self.phase == Phase::Complete)
    }

    fn advance_one(&mut self) -> Result<(), String> {
        match self.phase {
            Phase::Paint => self.paint_step(),
            Phase::Bridge => self.bridge_step(),
            Phase::Transfer => self.transfer_step(),
            Phase::Prepare => self.prepare_step(),
            Phase::CloseJob => self.close_job_step(),
            Phase::Complete | Phase::Close => Ok(()),
        }
    }

    fn paint_step(&mut self) -> Result<(), String> {
        let state = self.state.as_mut().ok_or("icon export World state was missing")?;
        let (width, height) = self.dimensions;
        let mut draw = DrawList::default();
        draw.set_screen_height(height as f32);
        let mut atlas = FontAtlas::builtin();
        let mut input = InputState::<ActionDescriptor>::default();
        let theme = Theme::default();
        let mut scroll = HashMap::new();
        let mut collapsed = HashMap::new();
        let mut selects = HashMap::new();
        {
            let mut context = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, None, &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, height as f32);
            infinite_world::world::render_world_3d(&self.scene, Rect::new(0.0, 0.0, width as f32, height as f32), &mut context, state, &mut self.resources, self.shadow_profile);
        }
        if state.snapshot_fault().is_some() {
            return Err("icon export World scene faulted while building".into());
        }
        if world3d_cursor_work_pending(state) {
            self.phase = Phase::Bridge;
            return Ok(());
        }
        if draw.scene_passes.is_empty() {
            return Err("icon export World scene published no 3D pass".into());
        }
        if draw.scene_passes.iter().any(|pass| pass.viewport_mask != self.viewport_mask) {
            return Err("icon export World scene lost the requested viewport mask".into());
        }
        match PreparedRenderInput::try_new(self.scene_revision, PREVIEW_GENERATION, draw, None, 0.0) {
            Ok(input) => {
                self.input = Some(input);
                self.phase = Phase::Transfer;
            }
            Err(rejected) => {
                let fault = rejected.fault().to_string();
                self.input_rejected = Some(rejected);
                return Err(fault);
            }
        }
        Ok(())
    }

    fn bridge_step(&mut self) -> Result<(), String> {
        let state = self.state.as_mut().ok_or("icon export World state was missing")?;
        let mut context = scene_step_context(PREVIEW_GENERATION, 3, &mut self.sequence);
        match step_world3d_scene_bridge(state, &mut context) {
            World3dSceneBridgeStep::Fault => return Err("icon export World bridge faulted".into()),
            World3dSceneBridgeStep::Idle | World3dSceneBridgeStep::Pending | World3dSceneBridgeStep::Complete => {}
        }
        match step_world3d_draw_rebuild(state, &mut context) {
            WorldDrawRebuildStep::Fault => return Err("icon export World draw publication faulted".into()),
            WorldDrawRebuildStep::Stale => {
                close_world3d_draw_rebuild_step(state, &mut context);
            }
            WorldDrawRebuildStep::Pending | WorldDrawRebuildStep::Complete => {}
        }
        match step_world3d_snapshot(state, &mut context) {
            World3dSnapshotApplyStep::Fault | World3dSnapshotApplyStep::Stale => return Err("icon export World snapshot faulted".into()),
            World3dSnapshotApplyStep::Idle | World3dSnapshotApplyStep::Pending | World3dSnapshotApplyStep::Complete => {}
        }
        self.phase = Phase::Paint;
        Ok(())
    }

    fn transfer_step(&mut self) -> Result<(), String> {
        let input = self.input.as_mut().ok_or("icon export prepared input was missing")?;
        match self.resources.append_step(input) {
            Ok(true) => {
                let input = self.input.take().expect("validated input owner");
                match PreparedRenderJob::try_new(input) {
                    Ok(job) => {
                        self.job = Some(job);
                        self.phase = Phase::Prepare;
                    }
                    Err(rejected) => {
                        let fault = rejected.fault().to_string();
                        self.job_rejected = Some(rejected);
                        return Err(fault);
                    }
                }
            }
            Ok(false) => {}
            Err(rejected) => {
                self.world_rejected = Some(rejected);
                return Err("icon export World resource transfer was refused".into());
            }
        }
        Ok(())
    }

    fn prepare_step(&mut self) -> Result<(), String> {
        let job = self.job.as_mut().ok_or("icon export prepared job was missing")?;
        let mut context = scene_step_context(PREVIEW_GENERATION, 1, &mut self.sequence);
        match job.step(&mut context) {
            StepOutcome::Yield => Ok(()),
            outcome @ StepOutcome::Complete(_) => {
                self.packet = job.take_packet();
                if self.packet.is_none() {
                    return Err("icon export prepared job completed without a packet".into());
                }
                self.job_outcome = Some(outcome);
                InteractiveJob::begin_close(job);
                self.phase = Phase::CloseJob;
                Ok(())
            }
            outcome @ (StepOutcome::Cancelled | StepOutcome::Fault(_)) => {
                self.job_outcome = Some(outcome);
                Err(job.fault().unwrap_or("icon export prepared job was cancelled").into())
            }
            outcome => {
                self.job_outcome = Some(outcome);
                Err("icon export prepared job returned an unsupported host outcome".into())
            }
        }
    }

    fn close_job_step(&mut self) -> Result<(), String> {
        if let Some(outcome) = self.job_outcome.as_mut() {
            if outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) != semio_framework_job::JobPayloadCloseStep::Complete {
                return Ok(());
            }
            self.job_outcome = None;
            return Ok(());
        }
        if let Some(job) = self.job.as_mut() {
            if !PreparedRenderJob::close_step(job) {
                return Ok(());
            }
            self.job = None;
            return Ok(());
        }
        self.phase = Phase::Complete;
        Ok(())
    }

    pub(crate) fn take_prepared(&mut self) -> Option<IconExportPreparedScene> {
        if self.phase != Phase::Complete || self.cancelled || self.fault.is_some() || self.packet.is_none() || self.state.is_none() {
            return None;
        }
        let packet = self.packet.take().expect("validated Icon export packet owner");
        let state = self.state.take().expect("validated Icon export World owner");
        self.phase = Phase::Close;
        Some(IconExportPreparedScene { packet: Some(packet), state: Some(state), dimensions: self.dimensions, format: self.format, closing: false, sequence: 0 })
    }

    pub(crate) fn progress(&self) -> (u8, usize, usize, usize) {
        let detail = self.job.as_ref().map_or(0, |job| job.fault().is_some() as usize);
        (self.phase as u8, usize::from(self.phase == Phase::Complete), 1, detail)
    }

    pub(crate) fn terminal(&self) -> bool {
        self.phase == Phase::Complete || (self.phase == Phase::Close && self.terminal_is_empty())
    }

    fn close_step(&mut self) -> Result<bool, String> {
        if let Some(outcome) = self.job_outcome.as_mut() {
            if outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) != semio_framework_job::JobPayloadCloseStep::Complete {
                return Ok(false);
            }
            self.job_outcome = None;
            return Ok(false);
        }
        if let Some(packet) = self.packet.as_mut() {
            if !packet.retire_step() {
                return Ok(false);
            }
            self.packet = None;
            return Ok(false);
        }
        if let Some(job) = self.job.as_mut() {
            InteractiveJob::begin_close(job);
            if !PreparedRenderJob::close_step(job) {
                return Ok(false);
            }
            self.job = None;
            return Ok(false);
        }
        if let Some(rejected) = self.job_rejected.as_mut() {
            if !rejected.close_step() {
                return Ok(false);
            }
            self.job_rejected = None;
            return Ok(false);
        }
        if let Some(input) = self.input.as_mut() {
            if !input.close_step() {
                return Ok(false);
            }
            self.input = None;
            return Ok(false);
        }
        if let Some(rejected) = self.input_rejected.as_mut() {
            if !rejected.close_step() {
                return Ok(false);
            }
            self.input_rejected = None;
            return Ok(false);
        }
        if let Some(rejected) = self.world_rejected.as_mut() {
            if !rejected.close_step() {
                return Ok(false);
            }
            self.world_rejected = None;
            return Ok(false);
        }
        if !self.resources.close_step() {
            return Ok(false);
        }
        let Some(state) = self.state.as_mut() else { return Ok(true) };
        if state.dynamic_retirement_is_idle() && !world3d_dynamic_retirement_terminal_is_empty(state) {
            begin_world3d_dynamic_retirement(state);
            return Ok(false);
        }
        let mut context = scene_step_context(PREVIEW_GENERATION, 1, &mut self.sequence);
        if !step_world3d_dynamic_retirement(state, &mut context) {
            return Ok(false);
        }
        self.state = None;
        Ok(true)
    }

    fn terminal_is_empty(&self) -> bool {
        self.state.is_none()
            && self.input.is_none()
            && self.input_rejected.is_none()
            && self.job.is_none()
            && self.job_rejected.is_none()
            && self.job_outcome.is_none()
            && self.packet.is_none()
            && self.world_rejected.is_none()
            && self.resources.terminal_is_empty()
    }
}

fn exact_dimension(value: f64) -> Option<u32> {
    (value.is_finite() && value >= 1.0 && value <= f64::from(u32::MAX) && value.fract() == 0.0).then_some(value as u32)
}

pub(super) fn validate_request(request_json: &str) -> Result<ValidatedIconExportRequest, String> {
    let request = serde_json::from_str::<IconRenderRequestFields>(request_json).map_err(|error| format!("icon export request was invalid: {error}"))?;
    if request.asset_url.is_empty() {
        return Err("icon export asset URL was empty".into());
    }
    if request.camera.position.iter().chain(request.camera.target.iter()).chain(request.camera.up.iter().flatten()).any(|value| !value.is_finite()) {
        return Err("icon export camera vectors must be finite".into());
    }
    if !request.camera.zoom.is_finite() || request.camera.zoom <= 0.0 {
        return Err("icon export camera zoom must be positive".into());
    }
    if request.camera.fov.is_some_and(|fov| !fov.is_finite() || fov <= 0.0 || fov >= 180.0) {
        return Err("icon export camera field of view must be between zero and 180 degrees".into());
    }
    if request.shape.as_deref().is_some_and(|shape| !matches!(shape, "rectangle" | "ellipse")) {
        return Err("icon export shape was invalid".into());
    }
    let width = exact_dimension(request.width).ok_or("icon export width exceeded the image budget")?;
    let height = exact_dimension(request.height).ok_or("icon export height exceeded the image budget")?;
    if width > 16_384 || height > 16_384 {
        return Err("icon export dimensions exceeded the image limit".into());
    }
    semio_framework_pixels::editing::validate_extent(width, height).map_err(|error| error.to_string())?;
    let format = match request.format {
        IconRenderFormat::Png => IconExportFormat::Png,
        IconRenderFormat::Svg => IconExportFormat::Svg,
    };
    Ok(ValidatedIconExportRequest { request, dimensions: (width, height), format })
}

fn scene_step_context<'a>(generation: u64, fuel: u64, sequence: &'a mut u64) -> semio_framework_job::StepContext<'a> {
    semio_framework_job::StepContext::new(
        semio_framework_job::OperationId(0x4943_4f4e),
        semio_framework_job::Generation(generation),
        semio_framework_job::StepBudget::new(fuel, u64::MAX),
        semio_framework_job::root_cancel_token(),
        || Some(0),
        sequence,
    )
}
