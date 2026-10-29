//! 🎹️ Staged slice of the `group` band of the mounted artifact instance: its `instance-children` members (WP6 step S0 of the instance family).
//!
//! 🤝️ The instance's private child group publication and mounted command log turns.
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework::Fault;
use store::MemberFactory;
use semio_framework_plugin_publication_mounted::owner::MountedPrivateChildGroup;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::retained_clone::RetainedCloneStep;
use semio_framework_value::RetirementDemand;
use store::SpaceMember;
use semio_framework_value::ValueError;
use semio_framework_plugin_publication_mounted::mounted_child_group_receipt;
use semio_framework_plugin_ownership::bounded_ownership::plugin_grant_funds;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use crate::children_components::ChildComposition;
use std::future::Future;
use semio_framework_plugin_instance_stores::stores_components::InstanceStores;
use store::Mutation;
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework_ui_contract::*;
use semio_framework_plugin_decl_views::child_view::ChildContentRetainers;
use semio_framework_plugin_publication_mounted::mounted_child_group_receipt::mounted_nested_retirement_demand;
use semio_framework_os_kernel as store;

impl<M: SpaceMember + MemberFactory + 'static> ChildComposition<M> {
    pub fn admit_private_child_group_frame(&mut self,operation:u64,captured_millis:i64,expires_at_us:u64,grant:RetainedCloneGrant)->Result<RetainedCloneStep,Fault>{
        use semio_framework_value::retained_clone::{RetainedCloneProgress,RetainedCloneStep};
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.private_child_groups.get(operation).is_some(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        if !self.private_child_groups.can_insert(operation){return Err(plugin_sdk_fault("mounted child group lacks its exact original fixed registry slot"));}
        let bytes=MountedPrivateChildGroup::<M>::frame_birth_bytes();
        if grant.maximum_capacity_bytes<bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}
        self.private_child_groups.insert_admitted(operation,Box::new(MountedPrivateChildGroup::new(captured_millis,expires_at_us)));
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()}))
    }

    pub fn private_child_group_close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let Some((_,operation))=self.private_child_groups.next_id_from(0) else {
            let release_bytes=self.private_child_groups.empty_backing_byte_demand().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"private group registry still owns its original slot"))?;
            return Ok(RetirementDemand { release_bytes, depth:usize::from(release_bytes!=0), ..Default::default() });
        };
        self.private_child_group_operation_close_demands(operation, body)
    }

    pub fn private_child_group_operation_close_demands(&self, operation:u64, body:usize) -> Result<RetirementDemand, ValueError> {
        let Some(owner)=self.private_child_groups.get(operation)else{return Ok(Default::default());};
        if owner.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:MountedPrivateChildGroup::<M>::frame_birth_bytes(),depth:1,..Default::default()});}
        mounted_child_group_receipt::mounted_nested_retirement_demand(owner.retirement_demands(body)?,1)
    }

    pub fn private_child_groups_terminal_is_empty(&self)->bool{self.private_child_groups.is_empty()&&self.private_child_groups.empty_backing_byte_demand()==Some(0)}

    pub fn close_private_child_group_step<A: ArtifactApp>(&mut self, stores: &mut InstanceStores<A>,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        let grant=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_grant(cx,grant);
        let demand=self.private_child_group_close_demands(grant.maximum_copy_bytes)?;
        if self.private_child_groups_terminal_is_empty(){return Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()});}
        if !plugin_grant_funds(demand,grant){return Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()});}
        let Some((_,operation))=self.private_child_groups.next_id_from(0)else{
            if cx.should_yield(){return Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()})}
            let step=self.private_child_groups.close_empty_backing_step(grant)?;cx.consume_retained(step.progress())?;cx.consume_fuel(1);
            return match step{
                RetainedCloneStep::Progress(progress)=>Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress}),
                RetainedCloneStep::Complete(progress)=>Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress}),
            };
        };
        self.close_private_child_group_operation_step(stores, operation, cx, grant)
    }

    pub fn close_private_child_group_operation_step<A: ArtifactApp>(&mut self, stores: &mut InstanceStores<A>,operation:u64,cx:&mut semio_framework_job::StepContext<'_>,loan:RetainedCloneGrant)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        let demand=self.private_child_group_operation_close_demands(operation, loan.maximum_copy_bytes)?;
        let step=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,loan,demand,|same,grant|{
            let step=self.close_private_child_group_operation_granted(stores, operation, same, grant)?;
            let terminal=self.private_child_groups.get(operation).is_none();
            Ok((match step{semio_framework_job::InteractiveJobCloseStep::Pending{progress}=>PluginLifecycleStep::Progress(progress),semio_framework_job::InteractiveJobCloseStep::Complete{progress}=>if terminal{PluginLifecycleStep::Complete(progress)}else{PluginLifecycleStep::Progress(progress)},semio_framework_job::InteractiveJobCloseStep::Blocked=>PluginLifecycleStep::Blocked{reason:"private group retains its original source"}},terminal))
        })?;
        Ok(match step{PluginLifecycleStep::Progress(progress)=>semio_framework_job::InteractiveJobCloseStep::Pending{progress},PluginLifecycleStep::Complete(progress)=>semio_framework_job::InteractiveJobCloseStep::Complete{progress},PluginLifecycleStep::AwaitingInput{..}|PluginLifecycleStep::Blocked{..}=>semio_framework_job::InteractiveJobCloseStep::Blocked})
    }

    pub(crate) fn close_private_child_group_operation_granted<A: ArtifactApp>(&mut self, stores: &mut InstanceStores<A>,operation:u64,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        let Some(original)=self.private_child_groups.get(operation)else{return Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()});};
        let terminal=original.terminal_is_empty();
        let demand=self.private_child_group_operation_close_demands(operation, grant.maximum_copy_bytes)?;
        if !plugin_grant_funds(demand,grant){return Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()});}
        if terminal{drop(self.private_child_groups.remove(operation));return Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}});}
        let owner=self.private_child_groups.get_mut(operation).ok_or_else(||plugin_sdk_fault("private group changed during its exact bounded close"))?;
        let child_grant=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
        let step=owner.close_step(&mut *stores.store,&mut self.children,self.composition.ownership_graph_mut(),&self.child_content_root,cx,child_grant)?;
        Ok(step.admit(child_grant,owner.terminal_is_empty())?)
    }
}
