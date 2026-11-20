//! 🧳️ Genuine Drawing retained decision and inverse preparation share immutable command and base sources.
use crate::{retained_drag::DrawingOriginalDrag,DrawingMutation,retained_decision::DrawingOriginalFieldDecision,retained_inverse::DrawingOriginalFieldInverse};
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_diff::DrawingDiff;
use protocol::{RetainedMutation,RetainedMutationPreparation,RetainedMutationDecision,RetainedMutationCloseContext,RetainedDiagnosticPreparation,RetainedDiagnosticSpec,MutationMessage,MutationMessageLedgerRetirement};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneClose,retained_clone_grant_admits}};
use semio_framework_value::factory::vector::FactoryVectorRetirement;
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkOutcome,RetirementPollExit};
use protocol::{RetainedDone as Done,RetainedFlight as Flight,RetainedReply as Reply,retained_conclude as conclude};
use std::mem::{ManuallyDrop,size_of};

pub(crate) fn measured<T>(
    effect: &mut RetainedAllocationEffect<'_>,
    work: &mut u128,
    action: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> Result<T, ValueError>,
) -> Result<T, ValueError> {
    let before = effect.performed_work();
    let result = action(effect);
    *work += effect.performed_work().saturating_sub(before);
    result
}

pub(crate) fn field_close<T: semio_framework_value::retained_clone::RetainedClone>(
    field: &mut protocol::RetainedFieldPreparation<T>,
    flight: &mut Flight,
    route: u8,
    mut original: Option<&mut OriginalRetirementCloseLoan<'_>>,
    body: usize,
    effect: &mut RetainedAllocationEffect<'_>,
    recover: bool,
    work: &mut u128,
) -> Result<RetainedCloneStep, RetirementPollExit> {
    if !field.close_started() {
        let turn = field.begin_close_paid(effect);
        *work += turn.work;
        return Ok(RetainedCloneStep::Progress(turn.result?));
    }
    if recover {
        if flight.resume(effect, body, 0, original.as_deref_mut())? != route {
            return Err(invalid().into());
        }
    } else {
        flight.start(effect, route, body, 0, original.as_deref_mut())?;
    }
    let before = effect.performed_work();
    let mut stored = None;
    let slot = semio_framework_value::retained_clone::OriginalEffectReply::borrow(&mut stored)?;
    drop(match (recover, original) {
        (false, None) => field.close_into(body, effect, slot),
        (true, None) => field.recover_close_into(body, effect, slot),
        (false, Some(loan)) => field.close_original_into(loan, body, effect, slot),
        (true, Some(loan)) => field.recover_close_original_into(loan, body, effect, slot),
    }?);
    *work += effect.performed_work().saturating_sub(before);
    flight.settle(&mut stored).ok_or(RetirementPollExit::Pending(0))?.map_err(Into::into)
}

pub(crate) fn poll_close(
    flight: &mut Flight,
    route: u8,
    mut original: Option<&mut OriginalRetirementCloseLoan<'_>>,
    body: usize,
    effect: &mut RetainedAllocationEffect<'_>,
    recover: bool,
    work: &mut u128,
    action: impl FnOnce(Option<&mut OriginalRetirementCloseLoan<'_>>, &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll,
) -> Result<RetainedCloneStep, RetirementPollExit> {
    if recover {
        if flight.resume(effect, body, 0, original.as_deref_mut())? != route {
            return Err(invalid().into());
        }
    } else {
        flight.start(effect, route, body, 0, original.as_deref_mut())?;
    }
    match action(original, effect) {
        semio_framework_value::RetainedWorkPoll::Complete(turn) => {
            *work += turn.work;
            flight.settle(&mut Some(()));
            turn.result.map_err(Into::into)
        }
        semio_framework_value::RetainedWorkPoll::Pending { work: performed } => {
            *work += performed;
            Err(RetirementPollExit::Pending(0))
        }
    }
}

pub(crate) fn owner_close<T: semio_framework_value::RetireOwned>(
    retirement: &mut RetainedCloneClose,
    output: &mut Option<T>,
    flight: &mut Flight,
    route: u8,
    original: Option<&mut OriginalRetirementCloseLoan<'_>>,
    body: usize,
    effect: &mut RetainedAllocationEffect<'_>,
    recover: bool,
    work: &mut u128,
) -> Result<RetainedCloneStep, RetirementPollExit> {
    if !retirement.is_empty() {
        return poll_close(flight, route, original, body, effect, recover, work, |original, effect| {
            if let Some(original) = original {
                retirement.step_original(original, body, effect)
            } else {
                retirement.step_granted(body, effect)
            }
        });
    }
    let turn = retirement.begin_granted(output, effect);
    *work += turn.work;
    Ok(turn.result?.unwrap_or(RetainedCloneStep::Complete(Default::default())))
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"Drawing original semantic output is not retained")}
fn progress(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
pub struct DrawingRetainedPreparation{flight:Flight,drag:DrawingOriginalDrag,diagnostic:RetainedDiagnosticPreparation<0>,messages:ManuallyDrop<Option<Vec<MutationMessage>>>,ledger:ManuallyDrop<Option<MutationMessageLedgerRetirement>>,reason:Option<&'static str>,unchanged:bool,decision_phase:u8,decision:DrawingOriginalFieldDecision,inverse:DrawingOriginalFieldInverse,output:ManuallyDrop<Option<DrawingDiff>>,rows:ManuallyDrop<Option<Vec<DrawingMutation>>>,retirement:RetainedCloneClose,row_close:Option<FactoryVectorRetirement<DrawingMutation>>,decision_done:bool,inverse_done:bool,closing:bool}
impl RetainedMutation<DrawingSnapshot> for DrawingMutation{type Preparation=DrawingRetainedPreparation;}
impl RetainedMutationPreparation<DrawingSnapshot,DrawingMutation> for DrawingRetainedPreparation{
 fn new()->Self{Self{flight:Default::default(),drag:Default::default(),diagnostic:Default::default(),messages:ManuallyDrop::new(None),ledger:ManuallyDrop::new(None),reason:None,unchanged:false,decision_phase:0,decision:Default::default(),inverse:Default::default(),output:ManuallyDrop::new(None),rows:ManuallyDrop::new(None),retirement:Default::default(),row_close:None,decision_done:false,inverse_done:false,closing:false}}
 fn birth_work()->Result<u64,ValueError>{
  let owners=[
   protocol::RetainedFieldPreparation::<DrawingMutation>::embedded_birth_work()?,
   protocol::RetainedFieldPreparation::<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>::embedded_birth_work()?.checked_mul(2).ok_or_else(invalid)?,
   protocol::RetainedFieldPreparation::<Option<semio_s_artifact_draw_drawing_model::FillStyle>>::embedded_birth_work()?.checked_mul(2).ok_or_else(invalid)?,
   protocol::RetainedFieldPreparation::<Option<semio_s_artifact_draw_drawing_model::StrokeStyle>>::embedded_birth_work()?.checked_mul(2).ok_or_else(invalid)?,
   protocol::RetainedFieldPreparation::<semio_framework_value_persistent::list::PagedList<semio_s_artifact_draw_drawing_model::PathSegment,{usize::MAX}>>::embedded_birth_work()?.checked_mul(2).ok_or_else(invalid)?,
  ];
  owners.into_iter().try_fold(1u64,|work,owner|work.checked_add(owner).ok_or_else(invalid))
 }
 fn decision_demand(&self,m:RetainedCloneRef<'_,DrawingMutation>,b:RetainedCloneRef<'_,DrawingSnapshot>,body:usize,max:u64)->Result<RetirementDemand,ValueError>{if self.closing||self.decision_done{return Err(invalid())}Ok(match self.decision_phase{0=>if matches!(m.get(),DrawingMutation::DragLayers(_)){if self.drag.ready(){let mut d=self.drag.take_decision_demand()?;d.copy_bytes+=2*size_of::<Option<DrawingDiff>>()+2*size_of::<Option<Vec<MutationMessage>>>()+64;d}else{self.drag.demand(b,m,body,max)?}}else if matches!(m.get(),DrawingMutation::SetArtboard(_)){fixed(2*size_of::<DrawingDiff>()+2*size_of::<Option<semio_s_artifact_draw_drawing_model::DrawingArtboard>>()+256)}else if self.decision.ready(){let mut d=self.decision.take_demand()?;d.copy_bytes+=2*size_of::<Option<DrawingDiff>>()+size_of::<bool>()*2+size_of::<Option<&'static str>>();d}else{self.decision.demand(b,m,body,max)?},1=>RetainedDiagnosticPreparation::<0>::start_demand(),2=>self.diagnostic.demand([],body,max)?,3=>fixed(3*size_of::<Vec<MutationMessage>>()+size_of::<bool>()),_=>return Err(invalid())})}
 fn decision_work_demand(&self,m:RetainedCloneRef<'_,DrawingMutation>,b:RetainedCloneRef<'_,DrawingSnapshot>,body:usize,max:u64)->Result<u64,ValueError>{
  self.decision_demand(m,b,body,max)?;
  let work=match self.decision_phase{
   0 if matches!(m.get(),DrawingMutation::DragLayers(_))&&!self.drag.ready()=>self.drag.work_demand(b,m,body,max)?,
   0 if !matches!(m.get(),DrawingMutation::DragLayers(_)|DrawingMutation::SetArtboard(_))&&!self.decision.ready()=>self.decision.work_demand(b,m,body,max)?,
   2=>self.diagnostic.advance_work_demand([],body,max)?,
   3=>self.diagnostic.take_work_demand()?,
   _=>1,
  };
  Ok(if work<=max{work}else{0})
 }
 fn decide_reply<'r>(&mut self,m:RetainedCloneRef<'_,DrawingMutation>,b:RetainedCloneRef<'_,DrawingSnapshot>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let start=effect.performed_work();
  let g=effect.grant();
  let mut work=0;
  let result=(||->Result<RetainedCloneStep,RetirementPollExit>{
   let d=if recover{Default::default()}else{let d=self.decision_demand(m,b,body,max)?;if !retained_clone_grant_admits(g,d)||self.decision_work_demand(m,b,body,max)?==0{return Ok(RetainedCloneStep::Progress(Default::default()))}d};
   let mut p=progress(d);
   match self.decision_phase{
  0=>if matches!(m.get(),DrawingMutation::DragLayers(_)){if self.drag.ready(){work+=1;let(reason,unchanged)=self.drag.outcome();let(output,messages,q)=self.drag.take_decision(g)?;*self.output=output;*self.messages=Some(messages);self.reason=reason;self.unchanged=unchanged;self.decision_done=true;p=q;p.copied_bytes=d.copy_bytes;return Ok(RetainedCloneStep::Complete(p))}else{p=self.drag.advance(b,m,body,max,effect,recover,&mut work)?.progress()}}else if let DrawingMutation::SetArtboard(v)=m.get(){work+=1;self.reason=(!crate::artboard::valid(&v.artboard)).then_some("mutation.invariant");self.unchanged=v.artboard==b.get().artboard;if self.reason.is_none(){*self.output=Some(if self.unchanged{DrawingDiff::default()}else{DrawingDiff{artboard:Some(crate::artboard::copy(&v.artboard)),..Default::default()}})}if self.reason.is_some()||self.unchanged{self.decision_phase=1}else{self.decision_done=true;return Ok(RetainedCloneStep::Complete(p))}}else if self.decision.ready(){work+=1;let(reason,unchanged)=self.decision.outcome();let(v,q)=self.decision.take(g)?;*self.output=v;self.reason=reason;self.unchanged=unchanged;p=q;p.copied_bytes=d.copy_bytes;if reason.is_some()||unchanged&&!matches!(m.get(),DrawingMutation::UpdatePathGeometry(_)|DrawingMutation::UpdateText(_)|DrawingMutation::UpdateImage(_)|DrawingMutation::SetShapeCoordinate(_)){self.decision_phase=1}else{self.decision_done=true;return Ok(RetainedCloneStep::Complete(p))}}else{p=self.decision.advance(b,m,body,max,effect,recover,&mut work)?.progress()},
  1=>{work+=1;let(level,code,message)=if let Some(code)=self.reason{(if code=="mutation.invariant"{semio_framework_diagnostic::Severity::Fatal}else{semio_framework_diagnostic::Severity::Error},code,"The Drawing command requires an admissible original target and native field value.")}else{(semio_framework_diagnostic::Severity::Warning,"mutation.no-op","The original Drawing field already has the authored value.")};p=self.diagnostic.start(RetainedDiagnosticSpec{level,code,message},g)?;self.decision_phase=2},
  2=>{let s=protocol::retained_routed!(self.flight,effect,recover,1,body,max,None,work,|reply|
    self.diagnostic.advance_into([],body,max,effect,reply),self.diagnostic.recover_advance_into([],body,max,effect,reply)
   ).map_err(RetirementPollExit::from)?;p=s.progress();if matches!(s,RetainedCloneStep::Complete(_)){self.decision_phase=3}},
  3=>{let turn=self.diagnostic.take(g);work+=turn.work;let(rows,q)=turn.result.map_err(RetirementPollExit::from)?.ok_or_else(invalid)?;*self.messages=Some(rows);self.decision_done=true;p=q;p.copied_bytes=d.copy_bytes;return Ok(RetainedCloneStep::Complete(p))},_=>return Err(invalid().into())}Ok(RetainedCloneStep::Progress(p))})();
  conclude(effect,start,result,work,|step|step.progress(),reply)
 }
 fn take_decision(&mut self)->RetainedMutationDecision<DrawingDiff>{let messages=self.messages.take().unwrap_or_default();if self.reason.is_some(){RetainedMutationDecision::Refused(messages)}else{RetainedMutationDecision::Admitted(self.output.take().expect("funded original Drawing decision"),messages)}}
 fn inverse_demand(&self,m:RetainedCloneRef<'_,DrawingMutation>,b:RetainedCloneRef<'_,DrawingSnapshot>,_:RetainedCloneRef<'_,DrawingDiff>,body:usize,max:u64)->Result<RetirementDemand,ValueError>{if self.closing||self.inverse_done{return Err(invalid())}if matches!(m.get(),DrawingMutation::DragLayers(_)){let mut d=self.drag.take_inverse_demand()?;d.copy_bytes+=2*size_of::<Option<Vec<DrawingMutation>>>()+size_of::<bool>();return Ok(d)}if self.unchanged{return Ok(fixed(2*size_of::<Option<Vec<DrawingMutation>>>()+size_of::<bool>()))}if matches!(m.get(),DrawingMutation::SetArtboard(_)){return Ok(RetirementDemand{capacity_bytes:size_of::<DrawingMutation>(),..fixed(2*size_of::<DrawingMutation>()+2*size_of::<Vec<DrawingMutation>>()+256)})}if self.inverse.ready(){let mut d=self.inverse.take_demand()?;d.copy_bytes+=2*size_of::<Option<Vec<DrawingMutation>>>()+size_of::<bool>();Ok(d)}else{self.inverse.demand(b,m,body,max)}}
 fn inverse_work_demand(&self,m:RetainedCloneRef<'_,DrawingMutation>,b:RetainedCloneRef<'_,DrawingSnapshot>,diff:RetainedCloneRef<'_,DrawingDiff>,body:usize,max:u64)->Result<u64,ValueError>{
  self.inverse_demand(m,b,diff,body,max)?;
  if max==0{return Ok(0)}
  if matches!(m.get(),DrawingMutation::DragLayers(_)|DrawingMutation::SetArtboard(_))||self.unchanged||self.inverse.ready(){return Ok(1)}
  self.inverse.work_demand(b,m,body,max)
 }
 fn inverse_reply<'r>(&mut self,m:RetainedCloneRef<'_,DrawingMutation>,b:RetainedCloneRef<'_,DrawingSnapshot>,diff:RetainedCloneRef<'_,DrawingDiff>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let start=effect.performed_work();let g=effect.grant();let mut work=0;
  let result=(||->Result<RetainedCloneStep,RetirementPollExit>{
   let d=if recover{Default::default()}else{let d=self.inverse_demand(m,b,diff,body,max)?;if !retained_clone_grant_admits(g,d)||self.inverse_work_demand(m,b,diff,body,max)?==0{return Ok(RetainedCloneStep::Progress(Default::default()))}d};
   if matches!(m.get(),DrawingMutation::DragLayers(_)){work+=1;let(rows,mut p)=self.drag.take_inverse(g)?;*self.rows=Some(rows.ok_or_else(invalid)?);self.inverse_done=true;p.copied_bytes=d.copy_bytes;return Ok(RetainedCloneStep::Complete(p))}
   if self.unchanged{work+=1;*self.rows=Some(Vec::new());self.inverse_done=true;return Ok(RetainedCloneStep::Complete(progress(d)))}
   if matches!(m.get(),DrawingMutation::SetArtboard(_)){
    work+=1;*self.rows=Some(Vec::new());let rows=self.rows.as_mut().unwrap();let allocated=rows.try_reserve_exact(1);
    let p=RetainedCloneProgress{retained_capacity_bytes:rows.capacity()*size_of::<DrawingMutation>(),..progress(d)};
    if allocated.is_err()||!p.fits(g){return Err(invalid().with_retained_progress(p).into())}
    rows.push(crate::artboard::set_artboard(crate::artboard::copy(&b.get().artboard)));self.inverse_done=true;return Ok(RetainedCloneStep::Complete(p))
   }
   if self.inverse.ready(){work+=1;let(v,mut p)=self.inverse.take(g)?;*self.rows=Some(v.ok_or_else(invalid)?);self.inverse_done=true;p.copied_bytes=d.copy_bytes;return Ok(RetainedCloneStep::Complete(p))}
   let step=self.inverse.advance(b,m,body,max,effect,recover,&mut work)?;
   Ok(RetainedCloneStep::Progress(step.progress()))
  })();
  conclude(effect,start,result,work,|step|step.progress(),reply)
 }
 fn take_inverse(&mut self)->Vec<DrawingMutation>{self.rows.take().expect("funded original Drawing inverse")}
 fn begin_close(&mut self){self.closing=true;self.drag.begin_close();self.diagnostic.begin_close();self.decision.begin_close();self.inverse.begin_close();}
 fn close_started(&self)->bool{self.closing}
 fn begin_close_demand(&self)->RetirementDemand{if self.closing{Default::default()}else{fixed(13*size_of::<bool>())}}
 fn begin_close_work_demand(&self)->u64{u64::from(!self.closing)}
 fn begin_close_paid(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress,ValueError>{
  let d=self.begin_close_demand();
  if self.closing||!retained_clone_grant_admits(effect.grant(),d){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
  match effect.remaining_work_units().and_then(|remaining|if remaining==0{Ok(false)}else{effect.performed(1)}){
   Ok(true)=>{self.begin_close();RetainedWorkOutcome{result:Ok(progress(d)),work:1}},
   Ok(false)=>RetainedWorkOutcome{result:Ok(Default::default()),work:0},
   Err(error)=>RetainedWorkOutcome{result:Err(error),work:0},
  }
 }
 fn close_demand(&self,c:RetainedMutationCloseContext<'_,DrawingMutation>,body:usize)->Result<RetirementDemand,ValueError>{if !self.diagnostic.terminal_is_empty(){return self.diagnostic.close_demand(body)}if let Some(l)=self.ledger.as_ref(){return semio_framework_value::ErasedSnapshotRetirement::next_demand(l,body)}if self.messages.is_some(){return Ok(fixed(size_of::<MutationMessageLedgerRetirement>()+2*size_of::<Vec<MutationMessage>>()))}if !self.drag.terminal_is_empty(){return self.drag.close_demand(c,body)}if !self.decision.terminal_is_empty(){return self.decision.close_demand(body)}if !self.inverse.terminal_is_empty(){return self.inverse.close_demand(c,body)}if !self.retirement.is_empty()||self.output.is_some(){return self.retirement.next_owner_demands::<DrawingDiff>(self.output.is_some(),&None,body)}if let Some(r)=self.row_close.as_ref(){return if r.terminal_is_empty(){Ok(fixed(2*size_of::<Option<FactoryVectorRetirement<DrawingMutation>>>()))}else{r.demands(c.mutations,body)}}if self.rows.is_some(){return Ok(FactoryVectorRetirement::<DrawingMutation>::constructor_demand())}Ok(Default::default())}
 fn close_work_demand(&self,c:RetainedMutationCloseContext<'_,DrawingMutation>,body:usize)->Result<u64,ValueError>{
  if !self.diagnostic.terminal_is_empty(){return self.diagnostic.close_work_demand(body)}
  if let Some(ledger)=self.ledger.as_ref(){return semio_framework_value::ErasedSnapshotRetirement::next_computational_work(ledger,body)}
  if self.messages.is_some(){return Ok(1)}
  if !self.drag.terminal_is_empty(){return self.drag.close_work_demand(c,body)}
  if !self.decision.terminal_is_empty(){return self.decision.close_work_demand(body)}
  if !self.inverse.terminal_is_empty(){return self.inverse.close_work_demand(c,body)}
  if !self.retirement.is_empty()||self.output.is_some(){return self.retirement.next_owner_work::<DrawingDiff>(self.output.is_some(),&None,body)}
  if let Some(rows)=self.row_close.as_ref(){return if rows.terminal_is_empty(){Ok(1)}else{rows.close_work_demand(c.mutations,body)}}
  Ok(u64::from(self.rows.is_some()))
 }
 fn requires_original_birth(&self)->bool{
  if !self.flight.idle(){return false}
  if !self.diagnostic.terminal_is_empty(){return self.diagnostic.requires_original_birth()}
  if self.ledger.is_some()||self.messages.is_some(){return false}
  if !self.drag.terminal_is_empty(){return self.drag.requires_original_birth()}
  if !self.decision.terminal_is_empty(){return self.decision.requires_original_birth()}
  if !self.inverse.terminal_is_empty(){return self.inverse.requires_original_birth()}
  if !self.retirement.is_empty(){return self.retirement.requires_original_birth()}
  self.output.is_none()&&self.row_close.as_ref().is_some_and(FactoryVectorRetirement::requires_original_birth)
 }
 fn requires_original_control(&self)->bool{
  if let Some(loaned)=self.flight.loaned(){return loaned}
  if !self.diagnostic.terminal_is_empty(){return self.diagnostic.requires_original_control()}
  if self.ledger.is_some()||self.messages.is_some(){return false}
  if !self.drag.terminal_is_empty(){return self.drag.requires_original_control()}
  if !self.decision.terminal_is_empty(){return self.decision.requires_original_control()}
  if !self.inverse.terminal_is_empty(){return self.inverse.requires_original_control()}
  if !self.retirement.is_empty(){return self.retirement.requires_original_control()}
  self.output.is_none()&&self.row_close.as_ref().is_some_and(FactoryVectorRetirement::requires_original_control)
 }
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
  if !self.diagnostic.terminal_is_empty(){return self.diagnostic.close_original_birth_step(original,grant)}
  if !self.drag.terminal_is_empty(){return self.drag.close_original_birth_step(original,grant)}
  if !self.decision.terminal_is_empty(){return self.decision.close_original_birth_step(original,grant)}
  if !self.inverse.terminal_is_empty(){return self.inverse.close_original_birth_step(original,grant)}
  if !self.retirement.is_empty(){return self.retirement.step_original_birth(original,grant)}
  self.row_close.as_mut().expect("selected original Drawing inverse rows").close_original_birth_step(original,grant)
 }
 fn close_reply<'r>(&mut self,c:RetainedMutationCloseContext<'_,DrawingMutation>,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let start=effect.performed_work();let g=effect.grant();let mut work=0;
  let result=(||->Result<RetainedCloneStep,RetirementPollExit>{
   if !self.closing{return Err(invalid().into())}
   let d=if recover{Default::default()}else{let d=self.close_demand(c,body)?;if !retained_clone_grant_admits(g,d)||self.close_work_demand(c,body)?>effect.remaining_work_units()?{return Ok(RetainedCloneStep::Progress(Default::default()))}d};
   let step=if !self.diagnostic.terminal_is_empty(){
    if recover{if self.flight.resume(effect,body,0,original.as_deref_mut())?!=2{return Err(invalid().into())}}else{self.flight.start(effect,2,body,0,original.as_deref_mut())?;}
    let before=effect.performed_work();
    let mut stored=None;
    let reply=semio_framework_value::retained_clone::OriginalEffectReply::borrow(&mut stored)?;
    drop(match (recover,original){
     (false,None)=>self.diagnostic.close_into(body,effect,reply),
     (true,None)=>self.diagnostic.recover_close_into(body,effect,reply),
     (false,Some(loan))=>self.diagnostic.close_original_into(loan,body,effect,reply),
     (true,Some(loan))=>self.diagnostic.recover_close_original_into(loan,body,effect,reply),
    }?);
    work+=effect.performed_work().saturating_sub(before);
    self.flight.settle(&mut stored).ok_or(RetirementPollExit::Pending(0))?.map_err(RetirementPollExit::from)?
   }else if let Some(ledger)=self.ledger.as_mut(){
    let step=ledger.close_step(g)?;work+=u128::from(ledger.last_turn_work());if ledger.terminal_is_empty(){self.ledger.take();}step
   }else if let Some(rows)=self.messages.take(){
    *self.ledger=Some(MutationMessageLedgerRetirement::new(String::new(),rows));work+=1;RetainedCloneStep::Progress(progress(d))
   }else if !self.drag.terminal_is_empty(){
    self.drag.close_step(c,original,body,effect,recover,&mut work)?
   }else if !self.decision.terminal_is_empty(){
    self.decision.close_step(original,body,effect,recover,&mut work)?
   }else if !self.inverse.terminal_is_empty(){
    self.inverse.close_step(c,original,body,effect,recover,&mut work)?
   }else if !self.retirement.is_empty()||self.output.is_some(){
    owner_close(&mut self.retirement,&mut self.output,&mut self.flight,3,original,body,effect,recover,&mut work)?
   }else if let Some(rows)=self.row_close.as_mut(){
    if rows.terminal_is_empty(){self.row_close=None;work+=1;RetainedCloneStep::Progress(progress(d))}
    else{poll_close(&mut self.flight,4,original,body,effect,recover,&mut work,|original,effect|if let Some(original)=original{rows.close_original_step(original,body,effect)}else{rows.close_step(c.mutations,body,effect)})?}
   }else if self.rows.is_some(){
    poll_close(&mut self.flight,5,None,body,effect,recover,&mut work,|_,effect|FactoryVectorRetirement::admit(&mut self.rows,&mut self.row_close,effect).map_err(Into::into))?
   }else{return Ok(RetainedCloneStep::Complete(Default::default()))};
   Ok(RetainedCloneStep::Progress(step.progress()))
  })();
  conclude(effect,start,result,work,|step|step.progress(),reply)
 }
 fn terminal_is_empty(&self)->bool{self.closing&&self.drag.terminal_is_empty()&&self.diagnostic.terminal_is_empty()&&self.messages.is_none()&&self.ledger.is_none()&&self.decision.terminal_is_empty()&&self.inverse.terminal_is_empty()&&self.output.is_none()&&self.rows.is_none()&&self.row_close.is_none()&&self.retirement.is_empty()&&self.flight.idle()}
}
impl Drop for DrawingRetainedPreparation{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"Drawing original semantic preparation requires exact issuer closure");}}
