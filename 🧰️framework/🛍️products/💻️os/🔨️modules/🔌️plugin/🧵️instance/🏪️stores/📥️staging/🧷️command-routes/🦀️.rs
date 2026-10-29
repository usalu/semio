//! 🧼️ Staged slice of the `command-routes` band of the mounted artifact instance: its `instance-stores` members (WP6 step S0 of the instance family).
//!
//! 🧷️ Command routes of the mounted artifact instance: framework-reserved routes, action dispatch and command dispatch.
use store::os_store::ArtifactStoreNativeVcsTurn as _;
use semio_framework_plugin_decl_app::ArtifactApp;
use store::os_store::store_schema::ArtifactCommand;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::{Fault, FaultFrom};
use std::future::Future;
use crate::stores_components::InstanceStores;
use store::{MemberFactory, Mutation, SpaceMember};
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_ui_contract::*;

impl<A: ArtifactApp> InstanceStores<A> {
    /// ⏪️ Advances one retained original Undo until its fixed receipt and physical return complete.
    pub fn framework_revert_unit(&mut self,edit_id:&str,command:&mut Option<store::os_store::ArtifactNativeVcsCommand<A::Snapshot,A::Mutation>>,result:&mut Option<store::os_store::ArtifactNativeVcsResult>,closing:&mut bool,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<bool>,semio_framework_job::OriginalTurnFailure>{
        if command.is_none()&&*closing{*closing=false;return Ok(Some(false))}
        if let Some(owner)=command.as_mut(){
            if *closing||cx.is_cancelled(){*closing=true;let complete=cx.with_original_retirement(|cx|self.store.close_native_vcs_command(owner,cx))?;if complete{let applied=result.take().is_some_and(|result|result.selected.is_some());*command=None;*closing=false;return Ok(Some(applied))}return Ok(None)}
            if result.is_some(){if self.store.ack_native_vcs_command(owner,cx)?{*closing=true;}return Ok(None)}
            if owner.terminal_is_empty(){*command=None;return Ok(Some(false))}
            let advance=self.store.advance_native_vcs_command(owner,cx)?;
            if advance.phase==store::os_store::ArtifactNativeVcsPhase::AwaitingAck{self.store.take_native_vcs_result(owner,cx,result)?;}
            return Ok(None)
        }
        let applied=self.store.applied_edit_ids();
        if cx.is_cancelled()||!applied.iter().any(|id|id.as_str()==edit_id)||applied.last().is_none_or(|tail|tail.as_str()==edit_id){return Ok(Some(false))}
        let mut request=Some(store::os_store::ArtifactNativeVcsRequest::Undo{lane:store::os_store::store_schema::HistoryLane::Document,policy:protocol::UndoPolicy::TransformAgainstConcurrent});
        self.store.admit_native_vcs_command(&mut request,cx,command)?;
        Ok(None)
    }

    /// 🪪️ The identity every id one admitted command mints is scoped by ([`AppOperationContext::authoring_seed`]):
    /// content-addressed with [`store::content_id`] over the admitting actor and this document store's hybrid
    /// logical clock ticked for the admission — per author and moment, never a per-instance counter a fresh
    /// session restarts.
    pub fn authoring_seed(&self, actor: &str) -> String {
        let mut clock = self.store.clock_now();
        clock.tick(semio_framework_job::default_now_ms().unwrap_or(clock.physical_ms));
        store::content_id("authoring-seed", format!("{actor}\u{1f}{}:{}:{}", clock.actor, clock.physical_ms, clock.logical).as_bytes())
    }
}
