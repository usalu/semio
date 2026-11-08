//! 🧪️ Actual source and authority bank shared by the paged owner laws.
use semio_framework_value_paged_text::{self as value,ErasedSnapshotRetirement};
use value::{OriginalConstructionRecipient,OriginalConstructionGrant,OriginalConstructionGrantComplete,OriginalConstructionAccept,OriginalConstructionAcceptComplete,OriginalWorkRecipient,OriginalWorkQuote,OriginalWorkQuoteComplete,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,ValueError};
use value::paged_text::PagedText;
use semio_framework_value_clone_source::RetainedCloneSource;
use semio_framework_value_retirement::retirement::RetireOwned;
pub(crate) struct Work;
impl OriginalWorkRecipient for Work{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<value::OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn settle_partial<'reply,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,check:value::OriginalWorkPartialCheck<'reply>)->value::OriginalWorkPartialCheckComplete<'reply> where 'source:'turn{check.complete(self.validate_context(context))}
 fn recover_partial<'reply,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,check:value::OriginalWorkPartialCheck<'reply>)->value::OriginalWorkPartialCheckComplete<'reply> where 'source:'turn{check.complete(self.validate_context(context))}
 fn admits<'a,'turn,'source>(&mut self,_:&mut (),quote:OriginalWorkQuote<'a>,_:u64,_:u64)->OriginalWorkQuoteComplete<'a>where 'source:'turn{quote.complete(Ok(true))}
}
pub(crate) struct Physical{pub(crate) interrupt:bool,pub(crate) submitted:RetainedCloneProgress}
impl OriginalConstructionRecipient for Physical{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<value::OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn grant<'a,'turn,'source>(&mut self,_:&mut (),quote:OriginalConstructionGrant<'a>)->OriginalConstructionGrantComplete<'a>where 'source:'turn{let d=quote.demand();let reply=quote.complete(Ok(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}));if self.interrupt{self.interrupt=false;std::hint::black_box(&reply);panic!("original construction grant interrupted after recording its reply")}reply}
 fn accept<'a,'turn,'source>(&mut self,_:&mut (),receipt:OriginalConstructionAccept<'a>)->OriginalConstructionAcceptComplete<'a>where 'source:'turn{let result=self.submitted.checked_add(receipt.progress()).map(|next|self.submitted=next);receipt.complete(result)}
 fn submitted<'turn,'source>(&self,_:&())->RetainedCloneProgress where 'source:'turn{self.submitted}
}
pub(crate) fn grant(d:value::RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
pub(crate) fn source<T:RetireOwned+Sync>(owner:T)->RetainedCloneSource<T>{
 let mut admission=RetainedCloneSource::admission_owned(owner,());let mut output=None;let mut cpu=100_000u64;for _ in 0..10000{if admission.terminal_is_empty(){break}let g=grant(admission.next_demand().unwrap());let mut actual=None;let turn=if admission.phase()=="ready"{admission.take_output(&mut output,g)}else{super::eff::with_effect(g,cpu,&mut actual,|effect|admission.advance(effect))};cpu=cpu.checked_sub(u64::try_from(turn.work).unwrap()).unwrap();let progress=turn.result.unwrap().progress();if let Some(actual)=actual{assert_eq!(actual,progress.retained_capacity_bytes)}}
 assert!(admission.terminal_is_empty());output.expect("original source was published")
}
pub(crate) fn close_source<T:RetireOwned+Sync>(mut source:RetainedCloneSource<T>){
 let mut cpu=100_000u64;for _ in 0..10000{if source.terminal_is_empty(){break}let d=source.next_close_demand(4096).unwrap();let g=grant(d);let work=source.next_close_work_demand(4096).unwrap();assert!(work<=cpu);let mut actual=None;let turn=super::eff::settled(super::eff::with_effect(g,work,&mut actual,|effect|source.close_step(effect)));cpu-=u64::try_from(turn.work).unwrap();let progress=turn.result.unwrap().progress();if let Some(actual)=actual{assert_eq!(actual,progress.retained_capacity_bytes)}}assert!(source.terminal_is_empty());
}
pub(crate) fn close_text<const N:usize>(mut text:PagedText<N>)->usize{
 let mut cpu=100_000u64;let mut released=0;for _ in 0..100000{if text.terminal_is_empty(){break}let d=text.next_demand(4096).unwrap();let g=grant(d);let work=text.next_computational_work(4096).unwrap();assert!(work<=cpu);let mut actual=None;let turn=super::eff::settled(super::eff::with_effect(g,work,&mut actual,|effect|text.close_step(effect)));cpu-=u64::try_from(turn.work).unwrap();released+=turn.result.unwrap().progress().released_bytes;assert!(actual.is_none());}assert!(text.terminal_is_empty());released
}

/// 🧵️ Drives the actual decoder frame and original authorities to paid publication.
pub(crate) fn decode_success<T:RetireOwned+Sync+value::paged_text::TextReadSource,const N:usize>(original:&RetainedCloneSource<T>,control:&mut value::NativeDecodeControl<'_>,physical:&mut value::OriginalConstructionAuthority<Physical>,work:&mut value::OriginalWorkAuthority<Work>)->PagedText<N>{
 let mut output=None;let mut frame=value::paged_text::PagedTextDecodeFrame::<N>::new(&mut output).unwrap();let mut completed=false;
 for _ in 0..100000{let g=grant(frame.demand(original.borrow()).unwrap());let turn=physical.with_loan(&mut (),|loan|work.with_loan(&mut (),|work_loan|control.with_work_recipient(work_loan,|control|control.with_construction_recipient(loan,|control|Ok::<_,ValueError>(frame.advance(original.borrow(),control,g)))))).unwrap().unwrap().unwrap();match turn.result{Ok(value::paged_text::PagedTextReadStep::Complete(_))=>{completed=true;break},Ok(value::paged_text::PagedTextReadStep::Pending|value::paged_text::PagedTextReadStep::Progress(_))=>{},refused=>panic!("fully funded original decoder refused: {:?}",refused.map(|_|()))}}assert!(completed);
 frame.begin_close();for _ in 0..10000{if frame.terminal_is_empty(){break}let g=grant(frame.close_demand(4096).unwrap());let units=frame.close_work_demand(4096).unwrap();let mut actual=None;let turn=super::eff::settled(super::eff::with_effect(g,units,&mut actual,|effect|frame.close_step(effect)));let progress=turn.result.unwrap().progress();if let Some(actual)=actual{assert_eq!(actual,progress.retained_capacity_bytes)}}assert!(frame.terminal_is_empty());drop(frame);output.expect("original decoder paid publication")
}

/// 🧵️ Drives the actual encoder frame and original authorities to paid publication.
pub(crate) fn encode_success<T:RetireOwned+Sync+value::paged_text::TextReadSource,const N:usize>(original:&RetainedCloneSource<T>,control:&mut value::NativeEncodeControl<'_>,physical:&mut value::OriginalConstructionAuthority<Physical>,work:&mut value::OriginalWorkAuthority<Work>,maximum:usize,prefix:&'static str,suffix:&'static str)->PagedText<N>{
 let mut output=None;let mut frame=value::paged_text::PagedTextEncodeFrame::<N>::formatter(&mut output,maximum,prefix,suffix).unwrap();let mut completed=false;
 for _ in 0..100000{let g=grant(frame.demand(original.borrow()).unwrap());let turn=physical.with_loan(&mut (),|loan|work.with_loan(&mut (),|work_loan|control.with_work_recipient(work_loan,|control|control.with_construction_recipient(loan,|control|Ok::<_,ValueError>(frame.advance(original.borrow(),control,g)))))).unwrap().unwrap().unwrap();match turn.result{Ok(value::paged_text::PagedTextReadStep::Complete(_))=>{completed=true;break},Ok(value::paged_text::PagedTextReadStep::Pending|value::paged_text::PagedTextReadStep::Progress(_))=>{},refused=>panic!("fully funded original encoder refused: {:?}",refused.map(|_|()).map_err(|failure|(failure.cause.primary,failure.cause.context,failure.cause.invariant,failure.cause.physical_invariant,failure.cause.capacity_invariant)))}}assert!(completed);
 frame.begin_close();for _ in 0..10000{if frame.terminal_is_empty(){break}let g=grant(frame.close_demand(4096).unwrap());let units=frame.close_work_demand(4096).unwrap();let mut actual=None;let turn=super::eff::settled(super::eff::with_effect(g,units,&mut actual,|effect|frame.close_step(effect)));let progress=turn.result.unwrap().progress();if let Some(actual)=actual{assert_eq!(actual,progress.retained_capacity_bytes)}}assert!(frame.terminal_is_empty());drop(frame);output.expect("original encoder paid publication")
}

/// 🧹️ Lends the test's resident marker and finite CPU to the actual selected owner.
pub(crate) fn close_step<T:ErasedSnapshotRetirement>(owner:&mut T,grant:RetainedCloneGrant,cpu:u64,actual:&mut Option<usize>)->semio_framework_value_retirement::OriginalRetirementOutcome{
 assert!(actual.is_none());let turn=super::eff::settled(super::eff::with_effect(grant,cpu,actual,|effect|owner.close_step(effect)));let progress=match &turn.result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};if let Some(observed)=actual.take(){assert_eq!(observed,progress.retained_capacity_bytes)}turn
}

/// 🔎️ Reconciles one actual test-bank marker with the retained close receipt.
pub(crate) fn check_close(actual:&mut Option<usize>,turn:&semio_framework_value_retirement::OriginalRetirementOutcome){let progress=match &turn.result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};if let Some(observed)=actual.take(){assert_eq!(observed,progress.retained_capacity_bytes)}}

pub(crate) fn close_owned_failure<T:RetireOwned>(failure:T){use semio_framework_value_clone_source::RetainedCloneClose;let mut owner=Some(failure);let mut close=RetainedCloneClose::default();let binding=None;for _ in 0..10000{if owner.is_none()&&close.is_empty(){break}let d=close.next_owner_demands::<T>(owner.is_some(),&binding,4096).unwrap();let units=close.next_owner_work::<T>(owner.is_some(),&binding,4096).unwrap();let mut actual=None;let progress=super::eff::with_effect(grant(d),units,&mut actual,|effect|if close.is_empty(){let turn=close.begin_granted(&mut owner,effect);turn.result.unwrap().map(|step|step.progress()).unwrap_or_default()}else{let turn=super::eff::settled(close.step_granted(effect));turn.result.unwrap().progress()});if let Some(actual)=actual{assert_eq!(actual,progress.retained_capacity_bytes)}}assert!(owner.is_none()&&close.is_empty());}
pub(crate) fn close_encode_failure(failure:value::paged_text::PagedTextReservationFailure){close_owned_failure(failure)}

/// 🎟️ Advances the actual bound encoder once with the same original authority loans.
pub(crate) fn encode_turn<T:RetireOwned+Sync+value::paged_text::TextReadSource,const N:usize>(frame:&mut value::paged_text::PagedTextEncodeFrame<'_,N>,original:&RetainedCloneSource<T>,control:&mut value::NativeEncodeControl<'_>,physical:&mut value::OriginalConstructionAuthority<Physical>,work:&mut value::OriginalWorkAuthority<Work>)->value::RetainedWorkOutcome<value::paged_text::PagedTextReadStep,value::paged_text::PagedTextReservationFailure>{
 let g=grant(frame.demand(original.borrow()).unwrap());physical.with_loan(&mut (),|loan|work.with_loan(&mut (),|work_loan|control.with_work_recipient(work_loan,|control|control.with_construction_recipient(loan,|control|Ok::<_,ValueError>(frame.advance(original.borrow(),control,g)))))).unwrap().unwrap().unwrap()
}
/// 🧹️ Settles the same frame's actual partial owners and returns the exact released bytes.
pub(crate) fn close_encode_frame<const N:usize>(frame:&mut value::paged_text::PagedTextEncodeFrame<'_,N>)->usize{
 frame.begin_close();let mut released=0;let mut births=0;for _ in 0..100000{if frame.terminal_is_empty(){break}let g=grant(frame.close_demand(4096).unwrap());let units=frame.close_work_demand(4096).unwrap();let mut actual=None;let turn=super::eff::settled(super::eff::with_effect(g,units,&mut actual,|effect|frame.close_step(effect)));let progress=turn.result.unwrap().progress();released+=progress.released_bytes;if let Some(actual)=actual{assert_eq!(actual,progress.retained_capacity_bytes);births+=actual}}assert!(frame.terminal_is_empty());released.checked_sub(births).expect("same original close births retire before their frame")
}

/// 🎟️ Advances the actual bound decoder once with the same original authority loans.
pub(crate) fn decode_turn<T:RetireOwned+Sync+value::paged_text::TextReadSource,const N:usize>(frame:&mut value::paged_text::PagedTextDecodeFrame<'_,N>,original:&RetainedCloneSource<T>,control:&mut value::NativeDecodeControl<'_>,physical:&mut value::OriginalConstructionAuthority<Physical>,work:&mut value::OriginalWorkAuthority<Work>)->value::RetainedWorkOutcome<value::paged_text::PagedTextReadStep,value::native_decoding::NativeReceivingEffectFailure<ValueError>>{
 let g=grant(frame.demand(original.borrow()).unwrap());physical.with_loan(&mut (),|loan|work.with_loan(&mut (),|work_loan|control.with_work_recipient(work_loan,|control|control.with_construction_recipient(loan,|control|Ok::<_,ValueError>(frame.advance(original.borrow(),control,g)))))).unwrap().unwrap().unwrap()
}
/// 🧹️ Retires the same bound decoder frame and reconciles its actual cleanup births.
pub(crate) fn close_decode_frame<const N:usize>(frame:&mut value::paged_text::PagedTextDecodeFrame<'_,N>)->usize{
 frame.begin_close();let mut released=0;let mut births=0;for _ in 0..100000{if frame.terminal_is_empty(){break}let g=grant(frame.close_demand(4096).unwrap());let units=frame.close_work_demand(4096).unwrap();let mut actual=None;let turn=super::eff::settled(super::eff::with_effect(g,units,&mut actual,|effect|frame.close_step(effect)));let progress=turn.result.unwrap().progress();released+=progress.released_bytes;if let Some(actual)=actual{assert_eq!(actual,progress.retained_capacity_bytes);births+=actual}}assert!(frame.terminal_is_empty());released.checked_sub(births).expect("same original close births retire before their frame")
}

/// 🧩️ Publishes the actual selected inline or paged formatter owner.
pub(crate) fn encode_inline_success<T:RetireOwned+Sync+value::paged_text::TextReadSource,const N:usize>(original:&RetainedCloneSource<T>,control:&mut value::NativeEncodeControl<'_>,physical:&mut value::OriginalConstructionAuthority<Physical>,work:&mut value::OriginalWorkAuthority<Work>,maximum:usize,prefix:&'static str,suffix:&'static str)->(Option<PagedText<N>>,Option<value::paged_text::InlineTextBuffer>){
 let mut output=None;let mut inline=None;let mut frame=value::paged_text::PagedTextEncodeFrame::<N>::formatter_inline(&mut output,&mut inline,maximum,prefix,suffix).unwrap();let mut completed=false;
 for _ in 0..100000{let turn=encode_turn(&mut frame,original,control,physical,work);match turn.result{Ok(value::paged_text::PagedTextReadStep::Complete(_))=>{completed=true;break},Ok(value::paged_text::PagedTextReadStep::Pending|value::paged_text::PagedTextReadStep::Progress(_))=>{},refused=>panic!("fully funded original inline formatter refused: {:?}",refused.map(|_|()).map_err(|failure|(failure.cause.primary,failure.cause.context,failure.cause.invariant,failure.cause.physical_invariant,failure.cause.capacity_invariant)))}}assert!(completed);close_encode_frame(&mut frame);drop(frame);(output,inline)
}
