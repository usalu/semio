//! 🧺️ Releases mounted containers after the close ladder retires their original values.
use super::*;
use semio_plugin_ownership::bounded_ownership::{EmptyBacking,plugin_effect_close,plugin_grant_funds,plugin_settle_poll};
use semio_plugin_instance_jobs::jobs_components::ArtifactInstanceOperationOwnerAliasRetirement;
use semio_framework_value::{FactoryAuthority,FactoryRetirement};
use std::sync::Arc;

macro_rules! shared {
    ($entry:expr)=>{&$entry as &dyn EmptyBacking}
}
macro_rules! exclusive {
    ($entry:expr)=>{&mut $entry as &mut dyn EmptyBacking}
}
macro_rules! mounted_backings {
    ($app:ident,$access:ident)=>{[
        $access!($app.roster.peer_roster_publications),
        $access!($app.roster.peer_roster_outcomes),
        $access!($app.roster.peer_presence_retirements),
        $access!($app.roster.presence_peer_retirements),
        $access!($app.child_composition.children),
        $access!($app.pending.pending_reserved),
        $access!($app.pending.reserved_commits),
        $access!($app.tools.typed_effect_outbox),
        $access!($app.tools.typed_event_outbox),
        $access!($app.tools.typed_ui_outbox),
        $access!($app.tools.typed_completion_outbox),
        $access!($app.tools.typed_composed_outbox),
        $access!($app.tools.tool_operations),
        $access!($app.tools.latest_wins_commands),
        $access!($app.tools.latest_wins_order),
        $access!($app.media.media_exports),
        $access!($app.media.media_closures),
        $access!($app.media.snapshot_retirements),
        $access!($app.media.segmented_downloads),
        $access!($app.media.segmented_closures),
        $access!($app.ingest.envelope_ingress),
        $access!($app.ingest.envelope_decode_jobs),
        $access!($app.ingest.store_replacement_jobs),
        $access!($app.ingest.envelope_field_decoder_retirements),
        $access!($app.ingest.envelope_completed_record_retirements),
    ]}
}

macro_rules! factory_fields {
    ($stores:ident,$test:ident)=>{[
        $test!($stores.artifact_one_item_factory),
        $test!($stores.config_one_item_factory),
        $test!($stores.draft_one_item_factory),
        $test!($stores.presence_one_item_factory),
        $test!($stores.transient_one_item_factory),
        $test!($stores.presence_local_root_retirement_factory),
        $test!($stores.transient_local_root_retirement_factory),
    ]}
}
macro_rules! occupied {
    ($entry:expr)=>{$entry.is_some()}
}

fn take_store_factory<A:ArtifactApp>(stores:&mut semio_plugin_instance_stores::stores_components::InstanceStores<A>)->Option<Arc<dyn FactoryRetirement>>{
    macro_rules! take {
        ($entry:expr)=>{if let Some(factory)=$entry.take(){return Some(factory)}};
    }
    let _=factory_fields!(stores,take);
    None
}

fn transfer_demand()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
fn transferred()->RetainedCloneStep{RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})}

impl<'v,A:ArtifactApp,M:ReferenceSpaceMember+MemberFactory+'static,X:InstanceExtension> LadderRef<'v,A,M,X>{
    fn mounted_empty_backing_demand(&self)->Option<RetirementDemand>{
        mounted_backings!(self,shared).into_iter().find_map(|owner|owner.release_bytes().filter(|bytes|*bytes!=0))
            .map(|release_bytes|RetirementDemand{release_bytes,depth:1,..Default::default()})
    }

    /// 🔎️ Quotes the first physical backing or retained operation alias after the active owners close.
    pub fn mounted_final_backing_demand(&self,body:usize)->Result<Option<RetirementDemand>,ValueError>{
        if let Some(demand)=self.mounted_empty_backing_demand(){return Ok(Some(demand))}
        if let Some(owner)=self.close.close_store_factory.as_ref(){return owner.demands(body).map(Some)}
        let stores=self.stores;
        if factory_fields!(stores,occupied).into_iter().any(|present|present){return Ok(Some(transfer_demand()))}
        if let Some(scope)=self.close.close_peer_roster_scope.as_ref(){return scope.retirement_demands().map(Some)}
        if self.roster.peer_roster_scope.is_some(){return Ok(Some(transfer_demand()))}
        if let Some(owner)=self.close.close_instance_operation_aliases.as_ref(){return owner.demands(body).map(Some)}
        Ok(self.tools.instance_operation_owner.retains_aliases().then(ArtifactInstanceOperationOwnerAliasRetirement::constructor_demands))
    }

    /// 🏁️ The App shell may be released after its containers and operation aliases have all returned.
    pub fn mounted_final_backings_are_empty(&self)->bool{
        let stores=self.stores;
        mounted_backings!(self,shared).into_iter().all(|owner|owner.release_bytes()==Some(0))
            && !factory_fields!(stores,occupied).into_iter().any(|present|present)
            && self.close.close_store_factory.is_none()
            && self.roster.peer_roster_scope.is_none()
            && self.close.close_peer_roster_scope.is_none()
            && !self.tools.instance_operation_owner.retains_aliases()
            && self.close.close_instance_operation_aliases.is_none()
    }
}

impl<'ctx,A:ArtifactApp,M:ReferenceSpaceMember+MemberFactory+'static,X:InstanceExtension> LadderCtx<'ctx,A,M,X>{
    /// 📮️ Releases at most one empty original allocation while the App still owns its frame.
    pub fn release_mounted_final_backing(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,PluginLifecycleFailure>{
        if let Some(owner)=mounted_backings!(self,exclusive).into_iter().find(|owner|owner.release_bytes().is_some_and(|bytes|bytes!=0)){
            return owner.release(grant).map(Some).map_err(Into::into)
        }
        if let Some(owner)=self.close.close_store_factory.as_mut(){
            let step=plugin_effect_close(cx,grant,|effect|plugin_settle_poll(owner.step(grant.maximum_copy_bytes,effect)))?;
            if owner.terminal_is_empty(){self.close.close_store_factory=None;}
            return Ok(Some(RetainedCloneStep::Progress(step.progress())));
        }
        if plugin_grant_funds(transfer_demand(),grant){
            if let Some(factory)=take_store_factory(self.stores){
                self.close.close_store_factory=Some(FactoryAuthority::new(factory));
                return Ok(Some(transferred()));
            }
        } else {
            let stores=&*self.stores;
            if factory_fields!(stores,occupied).into_iter().any(|present|present){
                return Ok(Some(RetainedCloneStep::Progress(Default::default())));
            }
        }
        if let Some(scope)=self.close.close_peer_roster_scope.as_mut(){
            let step=plugin_effect_close(cx,grant,|effect|match scope.close_step(effect).result{
                Ok(step)=>Ok(step),
                Err(semio_framework_async::CancelTokenRetirementError::Blocked(_))=>Ok(RetainedCloneStep::Progress(Default::default())),
                Err(semio_framework_async::CancelTokenRetirementError::Refused(error))=>Err(PluginLifecycleFailure::from(error)),
            })?;
            if scope.terminal_is_empty(){self.close.close_peer_roster_scope=None;}
            return Ok(Some(RetainedCloneStep::Progress(step.progress())));
        }
        if self.roster.peer_roster_scope.is_some(){
            if !plugin_grant_funds(transfer_demand(),grant){return Ok(Some(RetainedCloneStep::Progress(Default::default())))}
            self.close.close_peer_roster_scope=self.roster.peer_roster_scope.take().map(semio_framework_async::CancelTokenRetirement::from_token);
            return Ok(Some(transferred()));
        }
        if let Some(owner)=self.close.close_instance_operation_aliases.as_mut(){
            let step=owner.step(cx,grant)?;
            if owner.terminal_is_empty(){drop(self.close.close_instance_operation_aliases.take());}
            return Ok(Some(RetainedCloneStep::Progress(step.progress())));
        }
        if !self.tools.instance_operation_owner.retains_aliases(){return Ok(None)}
        let demand=ArtifactInstanceOperationOwnerAliasRetirement::constructor_demands();
        if !plugin_grant_funds(demand,grant){return Ok(Some(RetainedCloneStep::Progress(Default::default())))}
        self.close.close_instance_operation_aliases=self.tools.instance_operation_owner.take_aliases_for_retirement();
        Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})))
    }
}
