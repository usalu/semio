//! 🧵️ Original member candidates retain their actual issuers through planning, transfer and cancellation.
use super::*;
use semio_framework_value::{NativeEncodeControl,RetainedCloneGrant,RetainedCloneProgress,FactoryAuthority};

fn permits(grant:RetainedCloneGrant,copy:usize,capacity:usize,release:usize)->bool{grant.maximum_items!=0&&grant.maximum_depth!=0&&grant.maximum_copy_bytes>=copy&&grant.maximum_capacity_bytes>=capacity&&grant.maximum_release_bytes>=release}

impl<P,M> DurableStorePreparedOwnerV1<P,M>{
    pub fn prepared(&self)->Option<&ArtifactStoreOneItemPrepared<P,M>>{self.prepared.as_ref()}
    pub fn unbound_ready(&self)->bool{self.unbound.is_some()}
    pub fn plan_ready(&self)->bool{self.plan.is_some()}
    pub fn bound(&self)->Option<&DurableStoreBoundOutcomeV1<P,M>>{self.bound.as_ref()}
    pub fn take_bound(&mut self)->Option<DurableStoreBoundOutcomeV1<P,M>>{self.bound.take()}
    pub fn restore_bound(&mut self,original:DurableStoreBoundOutcomeV1<P,M>)->Result<(),(DurableOwnedGroupDecisionError,DurableStoreBoundOutcomeV1<P,M>)>{if self.bound.is_some(){return Err((DurableOwnedGroupDecisionError::InvalidOutcome,original));}self.bound=Some(original);Ok(())}

    pub fn capture_issuers(&mut self,store:&StorePartsRef<'_,P,M>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,DurableOwnedGroupDecisionError> where P:ValueToValue,M:StoreMutation<P>+ValueToValue{
        let mutation=store.owners.mutation_retirement_factory.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        let snapshot=store.owners.snapshot_retirement_factory.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        if self.mutation_retirement.as_ref().is_some_and(|owner|!Arc::ptr_eq(owner,mutation))||self.snapshot_retirement.as_ref().is_some_and(|owner|!Arc::ptr_eq(owner,snapshot)){return Err(DurableOwnedGroupDecisionError::InvalidOutcome);}
        let copy=usize::from(self.mutation_retirement.is_none())*size_of::<Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<M>>>()+usize::from(self.snapshot_retirement.is_none())*size_of::<Arc<dyn semio_framework_value::SnapshotRetirementFactory<P>>>();
        if copy==0{return Ok(Some(Default::default()));}
        if !permits(grant,copy,0,0){return Ok(None);}
        if self.mutation_retirement.is_none(){self.mutation_retirement=Some(Arc::clone(mutation));}
        if self.snapshot_retirement.is_none(){self.snapshot_retirement=Some(Arc::clone(snapshot));}
        Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}))
    }

    pub fn prepared_member_from_store(&self,role:&str,store:&StorePartsRef<'_,P,M>,grant:RetainedCloneGrant,native:&mut NativeEncodeControl<'_>)->Result<Option<(DurableOwnedGroupMemberV1,RetainedCloneProgress)>,DurableOwnedGroupDecisionError> where P:ValueToValue,M:StoreMutation<P>+ValueToValue{
        let envelope=&store.log.envelope;
        let dialect=envelope.dialect.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidIdentity)?;
        let unbound=self.unbound.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        let mut capacity=identity_text_bytes(&envelope.id,dialect)+role.len()+unbound.recovery_schema.len()+unbound.sha256.len();
        if let Some(owner)=envelope.owner.as_ref(){capacity=capacity.checked_add(identity_text_bytes(&owner.parent.artifact_id,&owner.parent.dialect)+owner.slot.len()+owner.child_id.len()).ok_or(DurableOwnedGroupDecisionError::InvalidIdentity)?;}
        if capacity>DURABLE_OWNED_GROUP_STRUCTURAL_MAX_BYTES{return Err(DurableOwnedGroupDecisionError::InvalidIdentity);}
        let copy=capacity+size_of::<DurableOwnedGroupMemberV1>();
        if !permits(grant,copy,capacity,0){return Ok(None);}
        native.admit_turn_capacity(grant.maximum_capacity_bytes).map_err(DurableOwnedGroupDecisionError::Retirement)?;
        let reference=copy_identity(&envelope.id,dialect,native)?;
        let owner=copy_owner(envelope.owner.as_ref(),native)?;
        let member=prepared_member(role,reference,owner,self,native)?;
        Ok(Some((member,RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:capacity,..Default::default()})))
    }
}

#[cfg(feature="deflate")]
impl<P,M> DurableStorePreparedOwnerV1<P,M>
where P:ArtifactPack+pack::snapshot::ArtifactDsl+semio_framework_dsl_record::BorrowedDslRecord+semio_framework_dsl_record::native_encoding::FieldProjectionSource,
      M:ArtifactCanonicalJsonTree,
{
    pub fn advance_unbound(&mut self,schema:&str,grant:RetainedCloneGrant,native:&mut NativeEncodeControl<'_>)->Result<Option<RetainedCloneProgress>,DurableOwnedGroupDecisionError>{
        if self.unbound.is_some(){return Ok(Some(Default::default()));}
        let original=self.prepared.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        let (output,receipt)=self.member_encoding.advance(original,schema,grant,native)?;
        if let Some(output)=output{self.unbound=Some(output);}
        Ok((receipt!=Default::default()).then_some(receipt))
    }
}

#[cfg(feature="deflate")]
impl<P,M> DurableStorePreparedOwnerV1<P,M>
where P:ValueToValue,M:StoreMutation<P>+ArtifactCanonicalJsonTree+ValueToValue,
{
    pub fn advance_bound(&mut self,store:&StorePartsRef<'_,P,M>,group:&str,grant:RetainedCloneGrant,native:&mut NativeEncodeControl<'_>)->Result<Option<RetainedCloneProgress>,DurableOwnedGroupDecisionError>{
        if self.plan.is_some(){return Ok(Some(Default::default()));}
        let original=self.prepared.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        let wire=self.member_encoding.wire.as_deref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        let (plan,receipt)=self.bound_encoding.advance(original,wire,store,group,grant,native)?;
        if let Some(plan)=plan{self.plan=Some(plan);}
        Ok((receipt!=Default::default()).then_some(receipt))
    }
}

impl<P,M> DurableStorePreparedOwnerV1<P,M>{
    pub fn transfer_bound(&mut self,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,DurableOwnedGroupDecisionError>{
        let mut group=[0u8;64];
        let identity=self.plan.as_ref().and_then(|plan|plan.group_id.as_deref()).ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        if identity.len()!=group.len(){return Err(DurableOwnedGroupDecisionError::InvalidHash);}
        group.copy_from_slice(identity.as_bytes());
        self.transfer(std::str::from_utf8(&group).map_err(|_|DurableOwnedGroupDecisionError::InvalidHash)?,grant)
    }
}

pub fn prepare_staged_diagnostics<P,M>(store:&mut StoreParts<'_,P,M>,grant:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),DurableOwnedGroupDecisionError>
where P:ValueToValue,M:StoreMutation<P>+ValueToValue,
{
    let root=store.wire.durable_group_root.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)?;
    if root.diagnostics.row_count()==0||root.diagnostics.prepared(){return Ok((true,Default::default()));}
    let identity=root.tail_undo_cache.as_ref().map(|(identity,_)|identity.as_str()).ok_or(DurableOwnedGroupDecisionError::InvalidIdentity)?;
    let receipt=root.diagnostics.advance(identity,grant).map_err(DurableOwnedGroupDecisionError::Retirement)?.progress();
    if root.diagnostics.prepared(){
        let bytes=root.diagnostics.measured_bytes(identity).map_err(DurableOwnedGroupDecisionError::Retirement)?;
        if !store.log.envelope.edit_messages.admits_measured(identity,bytes){return Err(DurableOwnedGroupDecisionError::InvalidOutcome);}
    }
    Ok((root.diagnostics.prepared(),receipt))
}

impl<P:Send+Sync+'static,M:Send+'static> DurableStorePreparedOwnerV1<P,M>{
    pub fn close_binding_residue(&mut self,grant:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),DurableOwnedGroupDecisionError>{
        self.retirement_progress=Default::default();
        let error=DurableOwnedGroupDecisionError::Retirement;
        #[cfg(feature="deflate")]
        {
            if !self.bound_encoding.terminal_is_empty(){self.retirement_progress=self.bound_encoding.close(grant).map_err(error)?;return Ok((false,self.retirement_progress));}
            if !self.member_encoding.terminal_is_empty(){self.retirement_progress=self.member_encoding.close(grant).map_err(error)?;return Ok((false,self.retirement_progress));}
        }
        if let Some(owner)=self.previous_authority.as_mut(){
            if owner.terminal_is_empty(){let copy=size_of::<canonical_edit::ArtifactStoreOneItemAuthorityRetirement>();if permits(grant,copy,0,0){self.previous_authority=None;self.retirement_progress=RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()};}}
            else{let demand=owner.next_demand(grant.maximum_copy_bytes).map_err(error)?;if demand.depth<=grant.maximum_depth&&permits(grant,demand.copy_bytes,demand.capacity_bytes,demand.release_bytes){self.retirement_progress=owner.close_step(grant).map_err(error)?.progress();}}
            return Ok((false,self.retirement_progress));
        }
        if let Some(owner)=self.discarded_pack.as_mut(){
            if owner.terminal_is_empty(){let copy=size_of::<semio_framework_value::retirement::controlled::ControlledRetirement<DurablePlanningOutputV1>>();if permits(grant,copy,0,0){self.discarded_pack=None;self.retirement_progress=RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()};}}
            else{let demand=owner.next_demand(grant.maximum_copy_bytes).map_err(error)?;if demand.depth<=grant.maximum_depth&&permits(grant,demand.copy_bytes,demand.capacity_bytes,demand.release_bytes){self.retirement_progress=owner.step(grant).map_err(error)?.progress();}}
            return Ok((false,self.retirement_progress));
        }
        Ok((true,Default::default()))
    }
    pub fn retire_originals_step(&mut self,grant:RetainedCloneGrant)->Result<bool,DurableOwnedGroupDecisionError>{
        self.retirement_progress=Default::default();
        let error=DurableOwnedGroupDecisionError::Retirement;
        let (empty,receipt)=self.close_binding_residue(grant)?;
        self.retirement_progress=receipt;
        if !empty||receipt!=Default::default(){return Ok(false);}
        if let Some(owner)=self.original_retirement.as_mut(){
            if owner.terminal_is_empty(){let release=size_of::<ArtifactStorePreparedRetirement<P,M>>();if permits(grant,size_of::<Option<Box<ArtifactStorePreparedRetirement<P,M>>>>(),0,release){self.original_retirement=None;self.retirement_progress=RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<Box<ArtifactStorePreparedRetirement<P,M>>>>(),released_bytes:release,..Default::default()};}}
            else{let demand=owner.next_demand(grant.maximum_copy_bytes).map_err(error)?;if demand.depth<=grant.maximum_depth&&permits(grant,demand.copy_bytes,demand.capacity_bytes,demand.release_bytes){self.retirement_progress=owner.close_step(grant).map_err(error)?.progress();}}
            return Ok(false);
        }
        if let Some(owner)=self.issuer_retirement.as_mut(){
            let demand=owner.demands(grant.maximum_copy_bytes).map_err(error)?;
            if demand.depth<=grant.maximum_depth&&permits(grant,demand.copy_bytes,demand.capacity_bytes,demand.release_bytes){self.retirement_progress=owner.step(grant).map_err(error)?.progress();if owner.terminal_is_empty(){self.issuer_retirement=None;}}
            return Ok(false);
        }
        let copy=size_of::<Self>()+size_of::<DurablePlanningOutputV1>()+size_of::<canonical_edit::ArtifactStoreOneItemAuthorityRetirement>();
        if self.plan.is_some(){
            if !permits(grant,copy,0,0){return Ok(false);}
            let plan=self.plan.take().unwrap();
            self.previous_authority=Some(canonical_edit::ArtifactStoreOneItemAuthorityRetirement::new(plan.authority));
            self.discarded_pack=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(DurablePlanningOutputV1::Binding{group:plan.group_id,pack:plan.recovery_pack,sha256:plan.recovery_pack_sha256}).unwrap_or_else(|_|unreachable!("native planning fields retain controlled retirement")));
        }else if self.unbound.is_some(){
            if !permits(grant,copy,0,0){return Ok(false);}
            self.discarded_pack=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(DurablePlanningOutputV1::Unbound(self.unbound.take().unwrap())).unwrap_or_else(|_|unreachable!("native unbound fields retain controlled retirement")));
        }else if self.bound.is_some(){
            if !permits(grant,copy,0,0){return Ok(false);}
            if self.prepared.is_some()||self.mutation_retirement.is_some()||self.snapshot_retirement.is_some(){return Err(DurableOwnedGroupDecisionError::InvalidOutcome);}
            let bound=self.bound.take().unwrap();
            self.prepared=Some(bound.prepared);self.mutation_retirement=Some(bound.mutation_retirement);self.snapshot_retirement=Some(bound.snapshot_retirement);
            self.discarded_pack=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(DurablePlanningOutputV1::Bound{unbound_sha256:bound.unbound_sha256,recovery_schema:bound.recovery_schema,recovery_pack:bound.recovery_pack,recovery_pack_sha256:bound.recovery_pack_sha256}).unwrap_or_else(|_|unreachable!("native bound fields retain controlled retirement")));
        }else if self.prepared.is_some(){
            let demand=ArtifactStoreOneItemPrepared::<P,M>::retirement_birth_demand();
            if !permits(grant,copy,demand.capacity_bytes,0)||grant.maximum_depth<demand.depth{return Ok(false);}
            if self.snapshot_retirement.is_none()||self.mutation_retirement.is_none(){return Err(DurableOwnedGroupDecisionError::InvalidOutcome);}
            let original=self.prepared.take().unwrap();
            let mutation=self.mutation_retirement.take().unwrap();
            let snapshot=self.snapshot_retirement.take().unwrap();
            match original.admit_fold_retirement(mutation,snapshot,grant){
                Ok((owner,mut receipt))=>{self.original_retirement=Some(owner);receipt.copied_bytes+=copy;self.retirement_progress=receipt;}
                Err((reason,original,mutation,snapshot))=>{self.prepared=Some(original);self.mutation_retirement=Some(mutation);self.snapshot_retirement=Some(snapshot);return Err(error(reason));}
            }
            return Ok(false);
        }else if self.mutation_retirement.is_some()||self.snapshot_retirement.is_some(){
            let copy=size_of::<FactoryAuthority>()+size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>();
            if !permits(grant,copy,0,0){return Ok(false);}
            let factory:Arc<dyn semio_framework_value::FactoryRetirement>=if let Some(factory)=self.mutation_retirement.take(){factory}else{self.snapshot_retirement.take().unwrap()};
            self.issuer_retirement=Some(FactoryAuthority::new(factory));
            self.retirement_progress=RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()};
            return Ok(false);
        }else{return Ok(self.originals_empty());}
        self.retirement_progress=RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()};
        Ok(false)
    }
}

pub fn retire_binding_member<P:ValueToValue+Send+Sync+'static,M:StoreMutation<P>+ValueToValue+Send+'static>(owner:&mut DurableStorePreparedOwnerV1<P,M>,store:&mut StoreParts<'_,P,M>,grant:semio_framework_os_store_one_item::ArtifactStoreOneItemGrant)->Result<bool,DurableOwnedGroupDecisionError>{
    if owner.prepared.is_some()&&(owner.mutation_retirement.is_none()||owner.snapshot_retirement.is_none()){
        owner.retirement_progress=owner.capture_issuers(&store.shared(),grant.retained_grant())?.unwrap_or_default();
        return Ok(false);
    }
    owner.retire_originals_step(grant.retained_grant())
}

#[cfg(all(test,feature="deflate"))]
#[path="🧪️tests/🦀️.rs"]
mod tests;
