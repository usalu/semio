//! 🪜️ Runs pressure units first, then visits live owners in round-robin order. The close ladder shares the ingress, store-replacement, and archive-load units.
use semio_plugin_decl_app::ArtifactApp;
use semio_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use std::future::Future;
use semio_plugin_instance_extension::InstanceExtension;
use semio_plugin_reactor_app::{LAST_MAINTENANCE_STAGE, PluginApp};
use store::{MemberFactory, Mutation, ReferenceSpaceMember};
use protocol::OpText;
use semio_plugin_ownership::bounded_ownership::{PluginLifecycleStep, plugin_grant_funds};
use crate::VcsArtifactApp;
use semio_ui_contract::*;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_plugin_instance_ladders::ladders_plugin_app_impl::MAINTENANCE_STAGES;
use semio_plugin_instance_ladders::{DOCUMENT_ARCHIVE_STEP_MINIMUM_ITEMS, MAINTENANCE_POSITIONS, MaintenanceUnit};

fn empty_demand()->semio_framework_value::RetirementDemand{Default::default()}

/// 📐️ Derives the unit grant from the unit's declared indivisible demand: copy and capacity ceilings are raised to the covering power of two, never past the caller's actual remaining turn authority, so no retained owner can deadlock on a step larger than the fixed host loan.
fn covering_loan(loan:RetainedCloneGrant,demand:semio_framework_value::RetirementDemand,ceiling:RetainedCloneGrant)->RetainedCloneGrant{
    let cover=|current:usize,needed:usize,limit:usize|if needed<=current{current}else{needed.checked_next_power_of_two().unwrap_or(needed).min(limit).max(current)};
    RetainedCloneGrant{maximum_copy_bytes:cover(loan.maximum_copy_bytes,demand.copy_bytes,ceiling.maximum_copy_bytes),maximum_capacity_bytes:cover(loan.maximum_capacity_bytes,demand.capacity_bytes,ceiling.maximum_capacity_bytes),maximum_release_bytes:cover(loan.maximum_release_bytes,demand.release_bytes,ceiling.maximum_release_bytes),maximum_depth:cover(loan.maximum_depth,demand.depth,ceiling.maximum_depth),..loan}
}

impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + Send + 'static> VcsArtifactApp<A, M> {
    /// 🎡️ Visits units in fair order and spends the grant on the first one that releases something. An empty or unfunded
    /// unit never consumes the call; a blocked or input-starved unit is remembered and reported only when nothing else moved.
    pub(crate) fn maintenance_ladder_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        use semio_plugin_ownership::bounded_ownership::original_lifecycle_delta;
        let empty = RetainedCloneProgress::default();
        let before = cx.retained_progress();
        let reserved_before = cx.reserved_return_progress();
        let mut spent = empty;
        loop {
            let reserved = original_lifecycle_delta(reserved_before, cx.reserved_return_progress())?;
            let fresh = original_lifecycle_delta(reserved, spent)?;
            let remaining = semio_plugin_reactor_app::remaining_plugin_turn_grant(loan, fresh);
            if cx.should_yield() || remaining.maximum_items == 0 { return Ok(PluginLifecycleStep::Progress(spent)); }
            let step = self.maintenance_ladder_unit_step(cx, remaining)?;
            spent = original_lifecycle_delta(before, cx.retained_progress())?;
            if !matches!(step, PluginLifecycleStep::Progress(progress) if progress.copied_items != 0) {
                return Ok(if spent == empty { step } else { PluginLifecycleStep::Progress(spent) });
            }
        }
    }

    fn maintenance_ladder_unit_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        LAST_MAINTENANCE_STAGE.store(u64::MAX,std::sync::atomic::Ordering::Relaxed);let empty=RetainedCloneProgress::default();let cursor=self.maintenance.maintenance_stage;let mut awaiting=None;let mut blocked=None;let archive_pending=self.ladder_ref().maintenance_unit_demand(MaintenanceUnit::Stage(24),loan.maximum_copy_bytes,cx.fuel_remaining())?.is_some_and(|demand|demand!=empty_demand());
        for position in 0..MAINTENANCE_POSITIONS{
            if cx.should_yield(){return Ok(PluginLifecycleStep::Progress(empty))}
            let grant=semio_plugin_ownership::bounded_ownership::original_lifecycle_grant(cx,loan);if grant.maximum_items==0{return Ok(PluginLifecycleStep::Progress(empty))}
            let Some(unit)=self.ladder_ref().maintenance_unit_at(position,cursor)else{cx.consume_fuel(1);continue};
            let archive=matches!(unit,MaintenanceUnit::Stage(24));
            let available_items=loan.maximum_items.min(cx.retained_grant().maximum_items);
            if archive_pending{
                let archive_dependency = matches!(unit, MaintenanceUnit::Stage(8 | 14 | 15));
                if archive && available_items < DOCUMENT_ARCHIVE_STEP_MINIMUM_ITEMS { cx.consume_fuel(1); continue; }
                if !archive && !archive_dependency && available_items <= DOCUMENT_ARCHIVE_STEP_MINIMUM_ITEMS { cx.consume_fuel(1); continue; }
            }
            let Some(demand)=self.ladder_ref().maintenance_unit_demand(unit,grant.maximum_copy_bytes,cx.fuel_remaining())?else{cx.consume_fuel(1);continue};
            let (loan,grant)=if semio_plugin_ownership::bounded_ownership::plugin_grant_funds(demand,grant){(loan,grant)}else{let widened=covering_loan(loan,demand,cx.retained_grant());(widened,semio_plugin_ownership::bounded_ownership::original_lifecycle_grant(cx,widened))};
            if !semio_plugin_ownership::bounded_ownership::plugin_grant_funds(demand,grant){cx.consume_fuel(1);continue}
            if matches!(unit,MaintenanceUnit::MediaRefusal)&&self.media.quoted_media_refusal_selected(){return self.ladder_ctx().maintenance_unit_step(unit,cx,grant)}
            // Preparing an original window partition can open a receiving scope before it copies
            // its leaf. The follow owner posts both parts onto this same mounted turn.
            let step=if matches!(unit,MaintenanceUnit::ToolOverlay)&&self.tools.original_tool_follow.is_some(){
                LAST_MAINTENANCE_STAGE.store(unit.trace(),std::sync::atomic::Ordering::Relaxed);
                self.ladder_ctx().maintenance_unit_step(unit,cx,loan)?
            }else if let MaintenanceUnit::Stage(stage @ (0 | 14 | 24))=unit{
                self.maintenance.maintenance_stage=(stage+1)%MAINTENANCE_STAGES;
                LAST_MAINTENANCE_STAGE.store(unit.trace(),std::sync::atomic::Ordering::Relaxed);
                self.ladder_ctx().maintenance_unit_step(unit,cx,loan)?
            }else{
                semio_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|same,actual|{
                    if let MaintenanceUnit::Stage(stage)=unit{self.maintenance.maintenance_stage=(stage+1)%MAINTENANCE_STAGES}LAST_MAINTENANCE_STAGE.store(unit.trace(),std::sync::atomic::Ordering::Relaxed);
                    let step=self.ladder_ctx().maintenance_unit_step(unit,same,actual)?;
                    Ok((step,true))
                })?
            };
            match step{PluginLifecycleStep::Progress(progress)|PluginLifecycleStep::Complete(progress)if progress!=empty=>return Ok(PluginLifecycleStep::Progress(progress)),step@PluginLifecycleStep::AwaitingInput{..}=>awaiting=awaiting.or(Some(step)),step@PluginLifecycleStep::Blocked{..}=>blocked=blocked.or(Some(step)),PluginLifecycleStep::Progress(_)|PluginLifecycleStep::Complete(_)=>{}}
        }
        if awaiting.is_none() && self.ladder_ref().document_archive_waits_for_host() {
            awaiting=Some(PluginLifecycleStep::AwaitingInput { reason: "retained archive load awaits original host polling or acknowledgement" });
        }
        Ok(awaiting.or(blocked).unwrap_or_else(||if self.maintenance_terminal_is_empty(){PluginLifecycleStep::Complete(empty)}else{PluginLifecycleStep::Progress(empty)}))
    }
}
