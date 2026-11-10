use super::*;
use semio_framework_value::{DslValue,RetainedAllocationEffect,RetainedWorkOutcome,ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeReceive,NativeReceivingFailure};
use semio_framework_value_testing_admission::{self as fx,RetainedWorkPoll};
use std::sync::Arc;
#[path="🔐️authorities/🦀️.rs"]
mod authorities;
pub(super) use authorities::{Authorities,WORK};
fn widened(g:RetainedCloneGrant,demands:[semio_framework_value::RetirementDemand;2])->RetainedCloneGrant{
 let sum=|field:fn(&semio_framework_value::RetirementDemand)->usize|demands.iter().map(field).sum::<usize>();
 RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes+sum(|d|d.copy_bytes),maximum_capacity_bytes:g.maximum_capacity_bytes+sum(|d|d.capacity_bytes),maximum_release_bytes:g.maximum_release_bytes+sum(|d|d.release_bytes),maximum_depth:g.maximum_depth+sum(|d|d.depth),maximum_items:g.maximum_items+2,..g}
}
/// ♻️ Runs one owner close turn as an original producer action.
pub(super) fn close_owner<M:NativeReceive<DslValue>+Sync>(owner:&mut OriginalJsonReceiving<M>,control:&mut NativeDecodeControl<'_>,payer:&mut Authorities,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,RetainedCloneProgress>{
let turn=driven(payer,control,g,|control,effect,reply|control.with_retirement(g,|retirement|Ok::<_,ValueError>(match owner.close_step(retirement,effect){RetainedWorkPoll::Complete(turn)=>reply.complete(turn.result),RetainedWorkPoll::Pending{..}=>reply.pending()})));
RetainedWorkOutcome{result:turn.result.map_err(|failure:semio_framework_value::OriginalRetirementFailure|failure.retained_progress()),work:turn.work}
}

/// ♻️ Runs one reply-shaped call as an original producer action so sealed retirement finds its invocation.
pub(super) fn driven<T:Send+'static,F:Send+'static>(payer:&mut Authorities,control:&mut NativeDecodeControl<'_>,g:RetainedCloneGrant,call:impl for<'r,'c,'e,'a> FnOnce(&'c mut NativeDecodeControl<'_>,&'e mut RetainedAllocationEffect<'a>,fx::OriginalEffectReply<'r,T,F>)->Result<fx::OriginalEffectReplyComplete<'r,T,F>,ValueError>)->RetainedWorkOutcome<T,F>{
let turn=payer.turn.identity();
payer.effect(control,widened(g,fx::driven_quotes::<T,F>()),WORK,|control,effect|{let mut call=Some(call);fx::drive_reply(effect,turn,&mut fx::bind_reply::<T,F,_>(|effect,reply|(call.take().expect("driven call spent"))(&mut *control,&mut effect.nested(g),reply)))}).unwrap()
}
