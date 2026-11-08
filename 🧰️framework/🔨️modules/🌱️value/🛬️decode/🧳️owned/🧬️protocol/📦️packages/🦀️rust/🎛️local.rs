//! 🎛️ Resident local producer runs one authored local effect of a native cursor under its original reply.
use crate::{NativeDecodeControl,OriginalReceivingPrimary,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,ValueError};
use semio_framework_value_retirement::retirement::RetireOwned;
use semio_framework_value_native_decoding::native_decoding::{OriginalEffectAuthority,OriginalEffectProducer,OriginalEffectReply,OriginalEffectReplyComplete};
/// 🎛️ Retains no source coordinates: the selected local operation carries the effect.
pub struct NativeLocalProducer<F:Send+'static>{marker:std::marker::PhantomData<fn()->F>}
impl<F:Send+'static> NativeLocalProducer<F>{pub fn new()->Self{Self{marker:std::marker::PhantomData}}}
/// 🧵️ Lends the selected local effect to the resident producer for one turn.
pub enum NativeLocalOperation<'a,F:'static>{Poll(&'a mut dyn FnMut(&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkPoll<RetainedCloneStep,F>),Reply(&'a mut dyn for<'r> FnMut(&mut crate::RetainedAllocationEffect<'_>,OriginalEffectReply<'r,RetainedCloneStep,F>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,F>,ValueError>)}
/// 🪪️ The resident authority one native cursor keeps for its local effects.
pub type NativeLocalAuthority<F>=OriginalEffectAuthority<NativeLocalProducer<F>>;
impl<F:Send+'static> RetireOwned for NativeLocalProducer<F>{type RetirementCursor=<() as RetireOwned>::RetirementCursor;fn controlled_retirement_supported()->bool{<() as RetireOwned>::controlled_retirement_supported()}fn retirement_seed(self)->Self::RetirementCursor{().retirement_seed()}}
impl<F:Send+'static> OriginalEffectProducer for NativeLocalProducer<F>{
 type Output=RetainedCloneStep;type Failure=F;type Context<'turn,'source>=NativeLocalOperation<'turn,F> where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&Self::Context<'turn,'source>)->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&Self::Context<'turn,'source>)->Option<crate::OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn receive<'reply,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,F>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,F>,ValueError>where 'source:'turn{match context{NativeLocalOperation::Poll(operation)=>Ok(match operation(effect){RetainedWorkPoll::Pending{..}=>reply.pending(),RetainedWorkPoll::Complete(turn)=>reply.complete(turn.result)}),NativeLocalOperation::Reply(operation)=>operation(effect,reply)}}
 fn recover<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,_:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,F>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,F>,ValueError>where 'source:'turn{Ok(reply.pending())}
}
/// 🧾️ Runs one pure local transition only after its original work quote is prepared, then commits the actual work.
pub fn native_receive_local(control:&mut NativeDecodeControl<'_>,work:u64,turn:impl FnOnce()->RetainedWorkOutcome<RetainedCloneStep,ValueError>)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{match control.prepare_work(work){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0},Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}}let turn=turn();if let Err(error)=control.commit_work(turn.work){return RetainedWorkOutcome{result:Err(error.into()),work:turn.work}}RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work}}
