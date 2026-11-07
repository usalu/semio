//! 🖼️ Real image event content shares one immutable decoded root through granted aliases.
use super::SemioImageSnapshot;
use std::{mem::{ManuallyDrop,size_of},sync::Arc};
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind,ToValue,FromValue};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,RetirementStep,arc_custody::{SharedControlledRetirement,admit_shared_allocation,shared_allocation_bytes}};
use semio_framework_value::retained_clone::{RetainedClone,RetainedCloneCursor,RetainedCloneBinding,RetainedCloneClose,RetainedCloneRef,retained_clone_grant_admits};

/// 🪆️ Immutable decoded bytes remain actual event content while preparation shares their root.
pub struct RasterImagePayload {owner:ManuallyDrop<Option<Arc<SemioImageSnapshot>>>}

fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
fn moved(bytes:usize)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}}

impl RasterImagePayload {
    fn retain(owner:Arc<SemioImageSnapshot>)->Self{Self{owner:ManuallyDrop::new(Some(owner))}}
    pub fn owned_birth_demands()->RetirementDemand{RetirementDemand{capacity_bytes:shared_allocation_bytes::<SemioImageSnapshot>(),depth:1,..Default::default()}}
    pub fn alias_demands()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
    pub fn from_owned(image:SemioImageSnapshot,grant:RetainedCloneGrant)->RetainedWorkOutcome<(Self,RetainedCloneProgress),(ValueError,SemioImageSnapshot)>{let turn=admit_shared_allocation(image,grant);RetainedWorkOutcome{result:turn.result.map(|(owner,p)|(Self::retain(owner),p)),work:turn.work}}
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
    fn retirement(mut self)->Box<dyn RetirementCursor>{Box::new(SharedControlledRetirement::lease(self.owner.take().expect("image payload retains its original owner")))}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<SharedControlledRetirement<SemioImageSnapshot>>())}
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
impl<I:Sync+'static+?Sized>semio_framework_value_native_owned::owned::NativeReceive<I> for RasterImagePayload where Arc<SemioImageSnapshot>:semio_framework_value_native_owned::owned::NativeReceive<I>{
    type Receiver=semio_framework_value_native_owned::owned::NativeMappedReceiver<I,Arc<SemioImageSnapshot>,Self>;
    fn receiver()->Self::Receiver{Self::Receiver::new()}
}
impl semio_framework_value_native_owned::owned::NativeRecordFields for RasterImagePayload {
    fn native_schema_count()->usize{<SemioImageSnapshot as semio_framework_value_native_owned::owned::NativeRecordFields>::native_schema_count()}
    fn native_schema_field(index:usize)->Option<semio_framework_value_native_owned::owned::NativeRecordSchema>{<SemioImageSnapshot as semio_framework_value_native_owned::owned::NativeRecordFields>::native_schema_field(index)}
}
impl semio_framework_pack_json::ArtifactCanonicalJsonTree for RasterImagePayload {
    fn canonical_tree_node(&self)->Result<semio_framework_pack_json::ArtifactCanonicalJsonNode<'_>,ValueError>{semio_framework_pack_json::ArtifactCanonicalJsonTree::canonical_tree_node(self.image())}
    fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn semio_framework_pack_json::ArtifactCanonicalJsonTree,ValueError>{semio_framework_pack_json::ArtifactCanonicalJsonTree::canonical_tree_child(self.image(),ordinal)}
    fn canonical_tree_key(&self,ordinal:usize)->Result<semio_framework_pack_json::ArtifactCanonicalJsonText<'_>,ValueError>{semio_framework_pack_json::ArtifactCanonicalJsonTree::canonical_tree_key(self.image(),ordinal)}
}

/// 🧵️ A genuine source binding and one image alias are independently closeable.
pub struct RasterImagePayloadClone {source:Option<RetainedCloneBinding>,output:ManuallyDrop<Option<RasterImagePayload>>,close:RetainedCloneClose,closing:bool,spent:bool}
impl RetainedClone for RasterImagePayload {type Cursor=RasterImagePayloadClone;fn retained_clone_cursor()->Self::Cursor{RasterImagePayloadClone{source:None,output:ManuallyDrop::new(None),close:Default::default(),closing:false,spent:false}}}
impl RetainedCloneCursor<RasterImagePayload> for RasterImagePayloadClone {
    fn advance_demands(&self,source:RetainedCloneRef<'_,RasterImagePayload>,_body:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.spent{return Err(invalid("image clone is closing or spent"));}if let Some(d)=source.binding_demand(&self.source)?{return Ok(d);}Ok(RasterImagePayload::alias_demands())}
    fn advance(&mut self,source:RetainedCloneRef<'_,RasterImagePayload>,body:usize,grant:RetainedCloneGrant)->RetainedWorkOutcome{let mut work=0;let result=(||{let demand=self.advance_demands(source,body)?;if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}let turn=source.bind(&mut self.source,grant);work+=turn.work;if let Some(p)=turn.result?{return Ok(RetainedCloneStep::Progress(p));}if self.output.is_some(){return Ok(RetainedCloneStep::Complete(Default::default()));}let turn=source.get().owner(grant);work+=turn.work;let(owner,p)=turn.result?;*self.output=Some(RasterImagePayload::retain(owner));Ok(RetainedCloneStep::Complete(p))})();RetainedWorkOutcome{result,work}}
    fn output_ready(&self)->bool{!self.closing&&!self.spent&&self.output.is_some()}
    fn take_demand(&self)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:size_of::<RasterImagePayload>(),depth:1,..Default::default()})}
    fn take(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(RasterImagePayload,RetainedCloneProgress)>>{if !self.output_ready(){return RetainedWorkOutcome{result:Ok(None),work:0}}let d=RetirementDemand{copy_bytes:size_of::<RasterImagePayload>(),depth:1,..Default::default()};if !retained_clone_grant_admits(grant,d){return RetainedWorkOutcome{result:Ok(None),work:0}}self.spent=true;RetainedWorkOutcome{result:Ok(self.output.take().map(|output|(output,moved(d.copy_bytes)))),work:1}}
    fn begin_close(&mut self)->bool{if self.closing{return false;}self.closing=true;true}
    fn close_started(&self)->bool{self.closing}
    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close.next_owner_demands::<RasterImagePayload>(self.output.is_some(),&self.source,body)}
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{if !self.closing{return RetainedWorkOutcome{result:Err(invalid("image clone requires closing intent")),work:0}}let d=match self.close_demands(grant.maximum_copy_bytes){Ok(d)=>d,Err(e)=>return RetainedWorkOutcome{result:Err(e),work:0}};if !retained_clone_grant_admits(grant,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}if !self.close.is_empty(){return self.close.step_granted(grant);}if self.output.is_some(){let turn=self.close.begin_granted(&mut self.output,grant);return RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}RetainedCloneBinding::close_one(&mut self.source,grant)}
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(32768)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(32768)?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(32768)?.depth)}
    fn terminal_is_empty(&self)->bool{self.closing&&self.source.is_none()&&self.output.is_none()&&self.close.is_empty()}
}
impl Drop for RasterImagePayloadClone {fn drop(&mut self){assert!(std::thread::panicking()||self.source.is_none()&&self.output.is_none()&&self.close.is_empty(),"image clone retains original ownership");}}
struct CloneRetirement {cursor:ManuallyDrop<RasterImagePayloadClone>}
impl RetirementCursor for CloneRetirement {
    fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.cursor.close_demands(body)}
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{let turn=self.cursor.close_step(grant);match turn.result{Ok(step)=>RetirementStep::Progress(step.progress(),turn.work),Err(error)=>RetirementStep::Failure(error,turn.work)}}
    fn terminal_is_empty(&self)->bool{self.cursor.terminal_is_empty()}
    fn next_work_byte_demand(&self)->Result<usize,ValueError>{self.cursor.next_close_copy_byte_demand()}
    fn next_birth_bytes(&self,body:usize)->Option<usize>{self.cursor.next_close_capacity_byte_demand(body).ok()}
    fn next_close_byte_demand(&self)->Option<usize>{self.cursor.next_close_release_byte_demand().ok()}
    fn next_depth_demand(&self)->Result<usize,ValueError>{self.cursor.next_close_depth_demand()}
    fn allows_admitted_narrow_work(&self)->bool{true}
    fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl Drop for CloneRetirement {fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"image clone retirement retains ownership");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.cursor);}}}}
impl RetireOwned for RasterImagePayloadClone {
    fn retirement(mut self)->Box<dyn RetirementCursor>{self.begin_close();Box::new(CloneRetirement{cursor:ManuallyDrop::new(self)})}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<CloneRetirement>())}
    fn controlled_retirement_supported()->bool{true}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
