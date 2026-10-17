//! 🩺️ Staged slice of the `maintenance-ladder` band of the mounted artifact instance: its `instance-app` members (WP6 step S0 of the instance family).
//!
//! 🪜️ The live-maintenance rotation of one hosted app: priority and pressure units first, then a fair fixed round robin
//! in which an empty stage never consumes the call. Every unit quotes its next demand on all axes and spends the caller's
//! unchanged grant on exactly one owner; the same module drives the envelope-ingress, store-replacement and archive-load
//! frontiers for both the live rotation and the close ladder.
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use std::future::Future;
use semio_framework_plugin_instance_extension::InstanceExtension;
use semio_framework_plugin_reactor_app::{LAST_MAINTENANCE_STAGE, PluginApp};
use store::{MemberFactory, Mutation, SpaceMember};
use protocol::OpText;
use semio_framework_plugin_ownership::bounded_ownership::{PluginLifecycleStep, plugin_grant_funds};
use crate::VcsArtifactApp;
use semio_framework_ui_contract::*;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_plugin_instance_ladders::ladders_plugin_app_impl::MAINTENANCE_STAGES;
use semio_framework_plugin_instance_ladders::{MAINTENANCE_POSITIONS, MaintenanceUnit};

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static> VcsArtifactApp<A, M> {
    /// 🎡️ Visits units in fair order and spends the grant on the first one that releases something. An empty or unfunded
    /// unit never consumes the call; a blocked or input-starved unit is remembered and reported only when nothing else moved.
    pub(crate) fn maintenance_ladder_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        let empty=RetainedCloneProgress::default();let mut spent=empty;
        loop{
            if cx.should_yield(){return Ok(PluginLifecycleStep::Progress(spent))}
            let remaining=semio_framework_plugin_reactor_app::remaining_plugin_turn_grant(loan,spent);
            let unit=match self.maintenance_ladder_unit_step(cx,remaining)?{PluginLifecycleStep::Progress(progress)if progress!=empty=>progress,step=>return Ok(if spent==empty{step}else{PluginLifecycleStep::Progress(spent)})};
            spent=spent.checked_add(unit)?;if unit.copied_items==0||semio_framework_plugin_reactor_app::remaining_plugin_turn_grant(loan,spent).maximum_items==0{return Ok(PluginLifecycleStep::Progress(spent))}
        }
    }

    fn maintenance_ladder_unit_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        LAST_MAINTENANCE_STAGE.store(u64::MAX,std::sync::atomic::Ordering::Relaxed);let empty=RetainedCloneProgress::default();let cursor=self.maintenance.maintenance_stage;let mut awaiting=None;let mut blocked=None;
        for position in 0..MAINTENANCE_POSITIONS{
            if cx.should_yield(){return Ok(PluginLifecycleStep::Progress(empty))}
            let grant=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_grant(cx,loan);if grant.maximum_items==0{return Ok(PluginLifecycleStep::Progress(empty))}
            let Some(unit)=self.ladder_ref().maintenance_unit_at(position,cursor)else{cx.consume_fuel(1);continue};
            let Some(demand)=self.ladder_ref().maintenance_unit_demand(unit,grant.maximum_copy_bytes)?else{cx.consume_fuel(1);continue};
            if !semio_framework_plugin_ownership::bounded_ownership::plugin_grant_funds(demand,grant){cx.consume_fuel(1);continue}
            let step=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|same,actual|{
                if let MaintenanceUnit::Stage(stage)=unit{self.maintenance.maintenance_stage=(stage+1)%MAINTENANCE_STAGES}LAST_MAINTENANCE_STAGE.store(unit.trace(),std::sync::atomic::Ordering::Relaxed);
                let step=self.ladder_ctx().maintenance_unit_step(unit,same,actual)?;Ok((step,true))
            })?;
            match step{PluginLifecycleStep::Progress(progress)|PluginLifecycleStep::Complete(progress)if progress!=empty=>return Ok(PluginLifecycleStep::Progress(progress)),step@PluginLifecycleStep::AwaitingInput{..}=>awaiting=awaiting.or(Some(step)),step@PluginLifecycleStep::Blocked{..}=>blocked=blocked.or(Some(step)),PluginLifecycleStep::Progress(_)|PluginLifecycleStep::Complete(_)=>{}}
        }
        Ok(awaiting.or(blocked).unwrap_or_else(||if self.maintenance_terminal_is_empty(){PluginLifecycleStep::Complete(empty)}else{PluginLifecycleStep::Progress(empty)}))
    }
}
