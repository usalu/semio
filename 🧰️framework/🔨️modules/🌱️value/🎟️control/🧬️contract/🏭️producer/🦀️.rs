//! 🏭️ Owns original borrowed producer authority and its resident action.
semio_framework_value_control_reply::original_effect_producer_engine!();


struct InvocationCall<'call>(Option<&'call mut dyn FnMut(&mut RetainedAllocationEffect<'_>)>);
struct InvocationProducer{source:Option<OriginalSourceTurnIdentity>}
impl OriginalEffectProducer for InvocationProducer{
    type Output=();type Failure=ValueError;
    type Context<'turn,'source>=InvocationCall<'turn> where 'source:'turn;
    fn validate_context<'turn,'source>(&self,_:&InvocationCall<'turn>)->Result<(),ValueError> where 'source:'turn{Ok(())}
    fn source_turn<'turn,'source>(&self,_:&InvocationCall<'turn>)->Option<OriginalSourceTurnIdentity> where 'source:'turn{self.source}
    fn receive<'r,'turn,'source>(&mut self,context:&mut InvocationCall<'turn>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,(),ValueError>)->Result<OriginalEffectReplyComplete<'r,(),ValueError>,ValueError> where 'source:'turn{(context.0.take().ok_or_else(||ValueError::invariant("original effect call already spent"))?)(effect);Ok(reply.complete(Ok(())))}
    fn recover<'r,'turn,'source>(&mut self,_:&mut InvocationCall<'turn>,_:&mut RetainedAllocationEffect<'_>,_:OriginalEffectReply<'r,(),ValueError>)->Result<OriginalEffectReplyComplete<'r,(),ValueError>,ValueError> where 'source:'turn{Err(ValueError::invariant("synchronous effect call has no pending action"))}
}
/// 🪝️ Runs one synchronous call inside the caller's effect as an original producer action, so retirement owners find their invocation; it charges nothing beyond the call's own work.
pub fn with_original_invocation(source:Option<OriginalSourceTurnIdentity>,effect:&mut RetainedAllocationEffect<'_>,call:&mut dyn FnMut(&mut RetainedAllocationEffect<'_>))->Result<(),ValueError>{
    let mut context=InvocationCall(Some(call));let mut authority=OriginalEffectAuthority::new(InvocationProducer{source});let mut action=None;
    if !authority.drive(&mut context,effect,&mut action)?{return Err(ValueError::invariant("synchronous effect call stayed pending"))}
    let mut answer=None;if !action.as_mut().ok_or_else(||ValueError::invariant("effect action absent"))?.transfer_reply(&mut answer)?{return Err(ValueError::invariant("effect reply is unsettled"))}
    authority.finish(&mut action)?;answer.ok_or_else(||ValueError::invariant("effect reply absent"))??;authority.into_producer().map(|_|()).map_err(|_|ValueError::invariant("effect producer is still pending"))
}
