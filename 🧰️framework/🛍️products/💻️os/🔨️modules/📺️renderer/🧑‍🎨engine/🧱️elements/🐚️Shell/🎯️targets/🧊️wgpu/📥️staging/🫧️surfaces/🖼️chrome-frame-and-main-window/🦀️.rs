//! 🫧️ `shell-surfaces` staging of `chrome-frame-and-main-window`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
impl SurfaceState {
    /// 🧵️ Reports whether the frame has retained chrome I/O or preview work to resume.
    pub fn chrome_maintenance_pending(&self, chrome: &ChromeState) -> bool {
        !self.retired_world3d_states.is_empty() || chrome.chrome_present.maintenance.pending()
    }

    /// 🪦 Advances exactly one retired scene owner under the ordinary frame maintenance budget.
    pub fn world3d_retirement_fault(&self)->Option<infinite_world::world::WorldDynamicFault> {self.world3d_retirement_fault}

    pub fn advance_world3d_retirement_step(&mut self) -> bool {
        let Some((_, state)) = self.retired_world3d_states.front_mut() else { return false };
        if world3d_dynamic_retirement_terminal_is_empty(state) {
            self.retired_world3d_states.pop_front();
            assert!(self.world3d_states.set_external_reservations(self.retired_world3d_states.len()), "retired World3d release restores admission credits");
            return !self.retired_world3d_states.is_empty();
        }
        let terrain_grant=semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:16*1024*1024,maximum_depth:64};
        let now = semio_framework_job::default_now_us();
        let Some(budget) = now.and_then(|now| semio_framework_job::StepBudget::from_duration(1, now, semio_framework_job::MAINTENANCE_LANE_WALL_US, terrain_grant)) else {
            self.world3d_retirement_fault=Some(infinite_world::world::WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::WorkLimit));
            return true;
        };
        let mut retained_progress = semio_framework_job::RetainedCloneProgress::default();
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(self.world3d_retirement_epoch),
            semio_framework_job::Generation(1),
            budget,
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us,
            &mut self.world3d_retirement_sequence,
            &mut retained_progress,
        );
        let terrain=infinite_world::world::step_world3d_terrain_retirement(&mut state.session, &mut state.terrain,terrain_grant,&mut context);
        if !terrain.ownership.fits(terrain_grant) {self.world3d_retirement_fault=Some(infinite_world::world::WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::InvariantViolated));return true;}
        match terrain.step {
            infinite_world::world::WorldTerrainMeshPublicationStep::Idle=>{},
            infinite_world::world::WorldTerrainMeshPublicationStep::Pending|infinite_world::world::WorldTerrainMeshPublicationStep::Complete=>return true,
            infinite_world::world::WorldTerrainMeshPublicationStep::Fault(fault)=>{self.world3d_retirement_fault=Some(fault);return true;}
        }
        step_world3d_dynamic_retirement(state, &mut context);
        if world3d_dynamic_retirement_terminal_is_empty(state) {
            self.retired_world3d_states.pop_front();
            assert!(self.world3d_states.set_external_reservations(self.retired_world3d_states.len()), "retired World3d release restores admission credits");
        }
        !self.retired_world3d_states.is_empty()
    }

    pub fn close_component_world_step(&mut self, target: &semio_framework_os_renderer_wgpu_contracts::ScenePointerTarget, grant: semio_framework_value::retained_clone::RetainedCloneGrant, context: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_value::retained_clone::RetainedCloneStep,infinite_world::world::WorldDynamicFault> {
        use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep,ValueRefusalKind};
        use infinite_world::world::{WorldDynamicFault,WorldTerrainMeshPublicationStep};
        if !matches!(target.kind, semio_framework_ui_declaration::ui::SurfaceKind::World3d | semio_framework_ui_declaration::ui::SurfaceKind::IconRender) {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        if context.is_cancelled() || context.should_yield() || grant.maximum_items==0 {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
        }
        if grant.maximum_depth==0 {return Err(WorldDynamicFault::Ownership(ValueRefusalKind::DepthLimit));}
        if self.component_world3d_retirement.is_none() {
            let Some(token)=self.world3d_states.token(&target.host_id) else {return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));};
            let Some(owner)=self.world3d_states.take_exact_to_retirement(token) else {return Err(WorldDynamicFault::StaleToken);};
            self.component_world3d_retirement=Some(owner);
            context.stage("componentWorldRetirement");
            context.consume_fuel(1);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:0,retained_capacity_bytes:0,released_bytes:0}));
        }
        let owner=self.component_world3d_retirement.as_mut().expect("component World3d retirement owner");
        if owner.id!=target.host_id {return Err(WorldDynamicFault::StaleToken);}
        let terrain=infinite_world::world::step_world3d_terrain_retirement(&mut owner.value.session, &mut owner.value.terrain,grant,context);
        if !terrain.ownership.fits(grant) {return Err(WorldDynamicFault::Ownership(ValueRefusalKind::InvariantViolated));}
        match terrain.step {
            WorldTerrainMeshPublicationStep::Pending|WorldTerrainMeshPublicationStep::Complete=>Ok(RetainedCloneStep::Progress(terrain.ownership)),
            WorldTerrainMeshPublicationStep::Fault(fault)=>Err(fault),
            WorldTerrainMeshPublicationStep::Idle=>Err(WorldDynamicFault::Ownership(ValueRefusalKind::UnsupportedOwner)),
        }
    }
}

impl TutorialState {
    /// 🎬️ Extra vertical space the tutorial control bar reserves below the navbar while a tutorial is
    /// active — `0.0` otherwise, so `body_rect`/the canvas layout are byte-identical to before this
    /// region existed whenever no tutorial is running.
    pub fn tutorial_bar_reserve(&self, theme: &Theme) -> f32 {
        if self.tutorial.is_some() {
            tutorial_bar_height(theme)
        } else {
            0.0
        }
    }
}

//#endregion ShellChrome

