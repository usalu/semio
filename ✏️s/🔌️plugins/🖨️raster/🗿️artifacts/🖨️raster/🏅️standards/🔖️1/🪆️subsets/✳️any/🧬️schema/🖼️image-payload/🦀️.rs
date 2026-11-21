//! 🖼️ Real image event content shares one immutable decoded root through granted aliases.
use super::SemioImageSnapshot;
use std::{mem::{ManuallyDrop,size_of},sync::Arc};
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedAllocationEffect,OriginalRetirementFailure,ValueError,ValueRefusalKind,ToValue,FromValue};
use semio_framework_value::retirement::{RetireOwned,arc_custody::{ArcLease,ArcLeaseRetirement,admit_shared_allocation,shared_allocation_value_demand}};
use semio_framework_value::retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete,RetainedAlias,RetainedAliasCursor,RetainedClone,retained_clone_grant_admits};

/// 🪆️ Immutable decoded bytes remain actual event content while preparation shares their root.
pub struct RasterImagePayload {owner:ManuallyDrop<Option<Arc<SemioImageSnapshot>>>}

fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
fn moved(bytes:usize)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}}

impl RasterImagePayload {
    fn retain(owner:Arc<SemioImageSnapshot>)->Self{Self{owner:ManuallyDrop::new(Some(owner))}}
    pub fn owned_birth_demands() -> RetirementDemand {
        shared_allocation_value_demand::<SemioImageSnapshot>().expect("fixed Raster image header extent")
    }
    /// ⛽️ Quotes both the shared allocation and the original value transfer.
    pub const fn owned_birth_work() -> u64 { 2 }
    pub fn alias_demands()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
    pub fn from_owned(image:SemioImageSnapshot,grant:RetainedCloneGrant)->RetainedWorkOutcome<(Self,RetainedCloneProgress),(ValueError,SemioImageSnapshot)> {
        let demand = Self::owned_birth_demands();
        let mut actual = None;
        let turn = admit_shared_allocation(image, grant, &mut actual);
        RetainedWorkOutcome {
            result: turn.result.map(|(owner, progress)| {
                let receipt = RetainedCloneProgress { copied_bytes: demand.copy_bytes, ..progress };
                (Self::retain(owner), receipt)
            }),
            work: turn.work,
        }
    }

    /// 🎟️ Moves a decoded image through the caller’s original allocation marker and Work.
    pub fn from_owned_in(
        image: &mut Option<SemioImageSnapshot>,
        effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<Option<(Self,RetainedCloneProgress)>,ValueError> {
        let demand = Self::owned_birth_demands();
        let grant = effect.grant();
        if !retained_clone_grant_admits(grant,demand) || effect.remaining_work_units()? < Self::owned_birth_work() {
            return Ok(None);
        }
        if effect.actual().is_some() || image.is_none() {
            return Err(ValueError::invariant("image birth requires its original image and vacant allocation marker"));
        }
        if !effect.performed(1)? { return Ok(None); }
        let turn = admit_shared_allocation(image.take().expect("validated original image"),grant,effect.actual());
        match turn.result {
            Ok((owner,progress)) => {
                assert!(effect.performed(1).expect("preflight admitted original image transfer"));
                Ok(Some((Self::retain(owner),RetainedCloneProgress { copied_bytes:demand.copy_bytes,..progress })))
            }
            Err((error,original)) => {
                *image=Some(original);
                Err(error)
            }
        }
    }
    pub fn from_shared(owner:Arc<SemioImageSnapshot>,grant:RetainedCloneGrant)->RetainedWorkOutcome<(Self,RetainedCloneProgress),(ValueError,Arc<SemioImageSnapshot>)>{if !retained_clone_grant_admits(grant,Self::alias_demands()){return RetainedWorkOutcome{result:Err((invalid("image payload alias is not funded"),owner)),work:0}}RetainedWorkOutcome{result:Ok((Self::retain(owner),moved(0))),work:1}}
    pub fn image(&self)->&SemioImageSnapshot{self.owner.as_deref().expect("image payload has transferred")}
    pub fn owner(&self,grant:RetainedCloneGrant)->RetainedWorkOutcome<(Arc<SemioImageSnapshot>,RetainedCloneProgress)>{if !retained_clone_grant_admits(grant,Self::alias_demands()){return RetainedWorkOutcome{result:Err(invalid("image owner alias is not funded")),work:0}}let Some(owner)=self.owner.as_ref()else{return RetainedWorkOutcome{result:Err(invalid("image payload has transferred")),work:0}};RetainedWorkOutcome{result:Ok((Arc::clone(owner),moved(0))),work:1}}
}

impl Clone for RasterImagePayload {fn clone(&self)->Self{Self{owner:ManuallyDrop::new(self.owner.as_ref().map(Arc::clone))}}}
impl std::ops::Deref for RasterImagePayload {type Target=SemioImageSnapshot;fn deref(&self)->&Self::Target{self.image()}}
impl std::fmt::Debug for RasterImagePayload {fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{let image=self.image();f.debug_struct("RasterImagePayload").field("width",&image.width).field("height",&image.height).finish_non_exhaustive()}}
impl PartialEq for RasterImagePayload {fn eq(&self,other:&Self)->bool{self.image()==other.image()}}
impl Drop for RasterImagePayload {fn drop(&mut self){assert!(std::thread::panicking()||self.owner.is_none(),"image payload requires granted lease retirement");}}

impl RetireOwned for RasterImagePayload {
    type RetirementCursor = ArcLeaseRetirement<SemioImageSnapshot>;
    fn retirement_seed(mut self) -> Self::RetirementCursor {
        ArcLeaseRetirement::lease(self.owner.take().expect("image payload retains its original owner"))
    }
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self::RetirementCursor>())}
    fn controlled_retirement_supported()->bool{true}
}
impl ToValue for RasterImagePayload {
    fn to_value(&self)->semio_framework_value::DslValue{self.image().to_value()}
    fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_value::DslValue,ValueError>{self.image().to_value_controlled(control)}
}
impl FromValue for RasterImagePayload {
    fn from_value(_value:semio_framework_value::DslValue)->Result<Self,ValueError>{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"image payload requires original native receiving"))}
}
impl semio_framework_value_native_owned::owned::NativeReceiveMap<Arc<SemioImageSnapshot>> for RasterImagePayload {fn from_native(owner:Arc<SemioImageSnapshot>)->Self{Self::retain(owner)}}
impl semio_framework_value_native_owned::owned::NativeReceiveMap<ArcLease<SemioImageSnapshot>> for RasterImagePayload {
    fn from_native(mut source: ArcLease<SemioImageSnapshot>) -> Self {
        Self::retain(source.0.take().expect("native image receiving produced a live Arc"))
    }
}
impl<I:semio_framework_value_native_owned::owned::tree::NativeValueSource+?Sized> semio_framework_value_native_owned::owned::NativeReceive<I> for RasterImagePayload where ArcLease<SemioImageSnapshot>:semio_framework_value_native_owned::owned::NativeReceive<I>{
    type Receiver=semio_framework_value_native_owned::owned::NativeMappedReceiver<I,ArcLease<SemioImageSnapshot>,Self>;
    fn receiver()->Self::Receiver{Self::Receiver::new()}
    fn record_subset_receiver()->Option<Self::Receiver>{Self::Receiver::original_record_subset()}
}
unsafe impl semio_framework_value_native_owned::owned::NativeRecordFields for RasterImagePayload {
    fn native_schema_count()->usize{<SemioImageSnapshot as semio_framework_value_native_owned::owned::NativeRecordFields>::native_schema_count()}
    fn native_schema_field(index:usize)->Option<semio_framework_value_native_owned::owned::NativeRecordSchema>{<SemioImageSnapshot as semio_framework_value_native_owned::owned::NativeRecordFields>::native_schema_field(index)}
}
impl semio_framework_pack_json::ArtifactCanonicalJsonTree for RasterImagePayload {
    fn canonical_tree_node(&self)->Result<semio_framework_pack_json::ArtifactCanonicalJsonNode<'_>,ValueError>{semio_framework_pack_json::ArtifactCanonicalJsonTree::canonical_tree_node(self.image())}
    fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn semio_framework_pack_json::ArtifactCanonicalJsonTree,ValueError>{semio_framework_pack_json::ArtifactCanonicalJsonTree::canonical_tree_child(self.image(),ordinal)}
    fn canonical_tree_key(&self,ordinal:usize)->Result<semio_framework_pack_json::ArtifactCanonicalJsonText<'_>,ValueError>{semio_framework_pack_json::ArtifactCanonicalJsonTree::canonical_tree_key(self.image(),ordinal)}
}

impl RetainedAlias for RasterImagePayload {
    fn alias_demand(&self) -> RetirementDemand {
        RetirementDemand { copy_bytes: size_of::<Option<Self>>(), depth: 1, ..Default::default() }
    }

    fn alias_work_demand(&self) -> u64 { 1 }

    fn admit_alias_into<'r>(
        &self,
        output: &mut Option<Self>,
        effect: &mut RetainedAllocationEffect<'_>,
        reply: OriginalEffectReply<'r, RetainedCloneStep, OriginalRetirementFailure>,
    ) -> Result<OriginalEffectReplyComplete<'r, RetainedCloneStep, OriginalRetirementFailure>, ValueError> {
        if output.is_some() || self.owner.is_none() {
            return Err(invalid("image alias requires a live source and vacant output"));
        }
        let demand = self.alias_demand();
        if !retained_clone_grant_admits(effect.grant(), demand) || effect.remaining_work_units()? < self.alias_work_demand() {
            return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))));
        }
        if !effect.performed(self.alias_work_demand())? {
            return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit, "image alias exceeded its original work").into())));
        }
        *output = Some(self.clone());
        Ok(reply.complete(Ok(RetainedCloneStep::Complete(moved(demand.copy_bytes)))))
    }

    fn recover_alias_into<'r>(
        &self,
        _output: &mut Option<Self>,
        _effect: &mut RetainedAllocationEffect<'_>,
        reply: OriginalEffectReply<'r, RetainedCloneStep, OriginalRetirementFailure>,
    ) -> Result<OriginalEffectReplyComplete<'r, RetainedCloneStep, OriginalRetirementFailure>, ValueError> {
        Ok(reply.pending())
    }
}

impl RetainedClone for RasterImagePayload {
    type Cursor = RetainedAliasCursor<Self>;
    fn retained_clone_cursor() -> Self::Cursor { Self::Cursor::default() }
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
