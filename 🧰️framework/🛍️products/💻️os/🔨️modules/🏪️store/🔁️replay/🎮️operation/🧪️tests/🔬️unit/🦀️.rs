//! 🧪️ Neutral cooperative operation laws with UTF-8 work and exact cancellation ownership.

use super::*;
use semio_framework_value::retirement::{RetireOwned, controlled::ControlledRetirement, arc_custody::SharedControlledRetirement};
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use crate::effect::{drive,settled,with_effect};
type Exit=semio_framework_value::RetirementPollExit;
type Effect<'a,'b>=&'a mut semio_framework_value::RetainedAllocationEffect<'b>;
type Poll=semio_framework_value::OriginalRetirementPoll;
type Control<'a>=&'a mut dyn semio_framework_value::OriginalRetirementControl;

#[derive(semio_framework_value::RetireOwned)]
struct TextInput {
    pub text: String,
    inverse_refused: bool,
    apply_refused: bool,
}

struct TextPreparation {
    last_work:u64,
    original:Option<Arc<TextInput>>,
    input_close:Option<SharedControlledRetirement<TextInput>>,
    pub base: Option<Arc<String>>,
    pub inverse: Option<String>,
    pub next: Option<String>,
    pub prepared: Option<ArtifactReplayPrepared<String, String>>,
    pub active: Option<ControlledRetirement<String>>,
    shared: Option<SharedControlledRetirement<String>>,
    inverse_list: Option<ControlledRetirement<semio_framework_value_persistent::list::PagedList<String, {usize::MAX}>>>,
    pub phase: u8,
    pub context: ArtifactReplayPreparationContext,
    calls: Arc<AtomicUsize>,
    pub cancelled: bool,
    pub closing: bool,
}

impl TextPreparation {
    fn new(base: Arc<String>, context: ArtifactReplayPreparationContext, calls: Arc<AtomicUsize>,original:Arc<TextInput>) -> Self {
        Self { last_work:0,original:Some(original),input_close:None,base: Some(base), inverse: None, next: None, prepared: None, active: None, shared: None, inverse_list: None, phase: 0, context, calls, cancelled: false, closing: false }
    }

    fn copy_scalar(source: &str, destination: &mut String, maximum_bytes: usize) -> RetainedCloneProgress {
        let Some(scalar) = source[destination.len()..].chars().next() else { return RetainedCloneProgress { copied_items: 1, ..Default::default() } };
        let bytes = scalar.len_utf8();
        if bytes > maximum_bytes { return RetainedCloneProgress::default(); }
        destination.push(scalar);
        RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }
    }
}

impl ArtifactReplayPreparation<String, TextInput> for TextPreparation {
    fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if let Some(owner)=self.input_close.as_ref(){return semio_framework_value::ErasedSnapshotRetirement::next_computational_work(owner,body)?.checked_add(1).ok_or_else(||ValueError::invariant("fixture terminal handoff work overflow"))}if let Some(owner)=self.active.as_ref(){return semio_framework_value::ErasedSnapshotRetirement::next_computational_work(owner,body)?.checked_add(1).ok_or_else(||ValueError::invariant("fixture terminal handoff work overflow"))}if let Some(owner)=self.shared.as_ref(){return semio_framework_value::ErasedSnapshotRetirement::next_computational_work(owner,body)?.checked_add(1).ok_or_else(||ValueError::invariant("fixture terminal handoff work overflow"))}if let Some(owner)=self.inverse_list.as_ref(){return semio_framework_value::ErasedSnapshotRetirement::next_computational_work(owner,body)?.checked_add(1).ok_or_else(||ValueError::invariant("fixture terminal handoff work overflow"))}Ok(u64::from(self.prepared.is_some()||self.inverse.is_some()||self.next.is_some()||self.base.is_some()||self.original.is_some()))}

    fn last_turn_work(&self)->u64{self.last_work}
    fn close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{let mut demand=if let Some(owner)=self.input_close.as_ref(){owner.next_demand(body)?}else if let Some(owner)=self.active.as_ref(){owner.next_demand(body)?}else if let Some(owner)=self.shared.as_ref(){owner.next_demand(body)?}else{self.inverse_list.as_ref().map_or(Ok(Default::default()),|owner|owner.next_demand(body))?};demand.depth=demand.depth.max(1);Ok(demand)}
    fn advance_demands(&self,_body:usize,_maximum_work_units:u64)->Result<semio_framework_value::RetirementDemand,ValueError>{Ok(semio_framework_value::RetirementDemand{copy_bytes:match self.phase{1=>self.base.as_ref().unwrap().get(self.inverse.as_ref().unwrap().len()..).unwrap().chars().next().map_or(0,char::len_utf8),3=>self.original.as_ref().unwrap().text.get(self.next.as_ref().unwrap().len()..).unwrap().chars().next().map_or(0,char::len_utf8),_=>0},capacity_bytes:match self.phase{0=>self.base.as_ref().unwrap().len(),2=>self.original.as_ref().unwrap().text.len(),_=>0},release_bytes:0,depth:1})}
    fn advance(&mut self, body:usize,grant: ArtifactStoreOneItemGrant,_control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<ArtifactReplayPreparationStep, semio_framework_value::OriginalRetirementFailure> {
        self.last_work=0;
        if self.cancelled || self.closing || !grant.permits_one() { return Ok(ArtifactReplayPreparationStep::Blocked); }
        let original=self.original.as_ref().unwrap();
        self.calls.fetch_add(1, Ordering::SeqCst);
        let base = self.base.as_ref().expect("immutable fixture base");
        let mut progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        match self.phase {
            0 => {
                if base.len() > grant.maximum_capacity_bytes { return Ok(ArtifactReplayPreparationStep::Blocked); }
                self.last_work=1;let mut inverse = String::new();
                inverse.try_reserve_exact(base.len()).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"fixture inverse allocation refused"))?;
                progress.retained_capacity_bytes = inverse.capacity();
                self.inverse = Some(inverse);
                self.phase = 1;
            }
            1 => {
                let inverse = self.inverse.as_mut().expect("inverse owner");
                progress = Self::copy_scalar(base, inverse, grant.maximum_copy_bytes);
                if inverse.len() == base.len() { self.phase = 2; }
            }
            2 => {
                if original.text.len() > grant.maximum_capacity_bytes { return Ok(ArtifactReplayPreparationStep::Blocked); }
                self.last_work=1;let mut next = String::new();
                next.try_reserve_exact(original.text.len()).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"fixture next allocation refused"))?;
                progress.retained_capacity_bytes = next.capacity();
                self.next = Some(next);
                self.phase = 3;
            }
            3 => {
                let next = self.next.as_mut().expect("next owner");
                progress = Self::copy_scalar(&original.text, next, grant.maximum_copy_bytes);
                if next.len() == original.text.len() { self.phase = 4; }
            }
            4 => {
                self.last_work=1;
                let inverse = if original.inverse_refused && self.context.mode != ArtifactReplayPreparationMode::Prefix {
                    Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "fixture inverse refused"))
                } else if self.context.mode == ArtifactReplayPreparationMode::Prefix || original.apply_refused {
                    Ok(Default::default())
                } else {
                    Ok(vec![self.inverse.take().expect("prepared inverse owner")].into())
                };
                self.prepared = Some(ArtifactReplayPrepared {
                    next: (!original.apply_refused).then(|| Arc::new(self.next.take().expect("prepared next owner"))),
                    inverse,
                    messages: Vec::new(),
                    apply_refusal: original.apply_refused.then(|| MutationApplyError::new("mutation.apply.refused", "fixture apply refused")),
                    input_refusal: None,
                    foreign_steps: false,
                });
                self.phase = 5;
            }
            5 => return Ok(ArtifactReplayPreparationStep::Prepared(RetainedCloneProgress::default())),
            _ => unreachable!(),
        }
        if progress!=Default::default(){self.last_work=1;}
        Ok(if self.phase == 5 { ArtifactReplayPreparationStep::Prepared(progress) } else { ArtifactReplayPreparationStep::Pending(progress) })
    }

    fn take_prepared(&mut self) -> Option<ArtifactReplayPrepared<String, TextInput>> {
        self.prepared.take().map(|prepared| ArtifactReplayPrepared {
            next: prepared.next,
            inverse: prepared.inverse.map(|inverse| inverse.into_iter().map(|text| TextInput { text, inverse_refused: false, apply_refused: false }).collect()),
            messages: prepared.messages,
            apply_refusal: prepared.apply_refusal,
            input_refusal: prepared.input_refusal,
            foreign_steps: prepared.foreign_steps,
        })
    }

    fn cancel(&mut self) { self.cancelled = true; }

    fn begin_close(&mut self) { self.closing = true; }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { if let Some(owner)=self.input_close.as_ref(){return owner.next_copy_byte_demand();} if let Some(owner)=self.active.as_ref(){return owner.next_copy_byte_demand();} if let Some(owner)=self.shared.as_ref(){return owner.next_copy_byte_demand();} self.inverse_list.as_ref().map_or(Ok(0),ControlledRetirement::next_copy_byte_demand) }
    fn next_close_capacity_byte_demand(&self, bytes:usize) -> Result<usize, ValueError> { if let Some(owner)=self.input_close.as_ref(){return owner.next_capacity_byte_demand(bytes);} if let Some(owner)=self.active.as_ref(){return owner.next_capacity_byte_demand(bytes);} if let Some(owner)=self.shared.as_ref(){return owner.next_capacity_byte_demand(bytes);} self.inverse_list.as_ref().map_or(Ok(0),|owner|owner.next_capacity_byte_demand(bytes)) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { if let Some(owner)=self.input_close.as_ref(){return owner.next_release_byte_demand();} if let Some(owner)=self.active.as_ref(){return owner.next_release_byte_demand();} if let Some(owner)=self.shared.as_ref(){return owner.next_release_byte_demand();} self.inverse_list.as_ref().map_or(Ok(0),ControlledRetirement::next_release_byte_demand) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { if let Some(owner)=self.input_close.as_ref(){return owner.next_depth_demand();} if let Some(owner)=self.active.as_ref(){return owner.next_depth_demand().map(|depth|depth.max(1));} if let Some(owner)=self.shared.as_ref(){return owner.next_depth_demand().map(|depth|depth.max(1));} self.inverse_list.as_ref().map_or(Ok(1),|owner|owner.next_depth_demand().map(|depth|depth.max(1))) }
    fn close_step(&mut self,effect:Effect<'_,'_>,receiving:Control<'_>)->Poll{let result=self.close_fixture(effect,receiving);Exit::settle(effect,result)}
    fn terminal_is_empty(&self)->bool{self.closing&&self.original.is_none()&&self.input_close.is_none()&&self.base.is_none()&&self.inverse.is_none()&&self.next.is_none()&&self.prepared.is_none()&&self.active.is_none()&&self.shared.is_none()&&self.inverse_list.is_none()}
}
impl TextPreparation{
    fn close_fixture(&mut self,effect:Effect<'_,'_>,receiving:Control<'_>)->Result<RetainedCloneStep,Exit>{
        let grant=effect.grant();let empty=RetainedCloneProgress::default();
        if !self.closing||grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(empty));}
        let demand=self.close_demands(grant.maximum_copy_bytes)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(empty));}if demand.capacity_bytes!=0{receiving.charge(demand.capacity_bytes)?;}
        if let Some(owner)=self.input_close.as_mut(){let step=Exit::resume(owner.step(effect))?;if owner.terminal_is_empty(){self.input_close=None;Exit::pay(effect,1)?;}return Ok(step);}
        if let Some(owner)=self.active.as_mut(){let step=Exit::resume(owner.step(effect))?;if owner.terminal_is_empty(){self.active=None;Exit::pay(effect,1)?;}return Ok(step);}
        if let Some(owner)=self.shared.as_mut(){let step=Exit::resume(owner.step(effect))?;if owner.terminal_is_empty(){self.shared=None;Exit::pay(effect,1)?;}return Ok(step);}
        if let Some(owner)=self.inverse_list.as_mut(){let step=Exit::resume(owner.step(effect))?;if owner.terminal_is_empty(){self.inverse_list=None;Exit::pay(effect,1)?;}return Ok(step);}
        Exit::pay(effect,u128::from(self.prepared.is_some()||self.inverse.is_some()||self.next.is_some()||self.base.is_some()||self.original.is_some()))?;
        if let Some(prepared)=self.prepared.as_mut(){
            if let Some(next)=prepared.next.take(){self.shared=Some(SharedControlledRetirement::lease(next));}
            else if let Ok(inverse)=&mut prepared.inverse {if !inverse.terminal_is_empty(){self.inverse_list=Some(ControlledRetirement::new(std::mem::take(inverse)).unwrap_or_else(|(error,_)|panic!("native fixture inverse refused: {error}")));}else{self.prepared=None;}}
            else{self.prepared=None;}
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}));
        }
        if let Some(value)=self.inverse.take().or_else(||self.next.take()){self.active=Some(ControlledRetirement::new(value).unwrap_or_else(|(error,_)|panic!("fixture text retirement refused: {error}")));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}));}
        if let Some(base)=self.base.take(){self.shared=Some(SharedControlledRetirement::lease(base));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}));}
        if let Some(input)=self.original.take(){self.input_close=Some(SharedControlledRetirement::lease(input));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}));}
        Ok(RetainedCloneStep::Complete(empty))
    }


}

impl Drop for TextPreparation {
    fn drop(&mut self) { assert!(self.terminal_is_empty() || std::thread::panicking(), "fixture replay producer dropped live owners"); }
}

fn close_turn<P,M>(owner:&mut ArtifactReplayPreparationRetirement<P,M>,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,loan:RetainedCloneGrant,grant:RetainedCloneGrant)->(Result<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>,u128,semio_framework_trace::HeapAllocationObservation){
    let((poll,heap),_)=with_effect(grant,1<<20,&mut None,|effect|semio_framework_trace::observe_heap_allocations_on_this_thread(||control.with_retirement(loan,|receiving|Ok::<_,ValueError>(owner.close_step(effect,receiving)))));
    match poll{Ok(poll)=>{let turn=settled(poll);(turn.result,turn.work,heap)},Err(error)=>(Err(error.into()),0,heap)}
}
fn close(mut retirement:ArtifactReplayPreparationRetirement<String,TextInput>,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>){
    let mut last=None;
    for _ in 0..4096 {
        let d=retirement.next_demand(4096).unwrap();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
        let step=close_turn(&mut retirement,control,grant,grant).0.expect("exact admitted fixture close");assert!(step.progress().fits(grant));
        last=Some((grant,step));
        if retirement.terminal_is_empty(){return;}
    }
    panic!("fixture replay preparation never closed; last quoted grant and step {last:?}");
}
fn close_owned<T:RetireOwned>(value:T){let mut owner=ControlledRetirement::new(value).unwrap_or_else(|(error,_)|panic!("fixture native owner refused: {error}"));for _ in 0..4096{if owner.terminal_is_empty(){return;}let d=owner.next_demand(4096).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let step=drive(grant,|effect|owner.step(effect)).unwrap();assert!(step.progress().fits(grant));}panic!("fixture owner did not close");}

#[test]
fn cooperative_history_operation_obeys_the_neutral_law() {
    let mut callback=|_:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|true;let mut recipient=semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient::new();let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new_retained(&mut callback);control.install_retirement_recipient(&mut recipient).unwrap();
    let law: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).expect("neutral operation law");
    let context = ArtifactReplayPreparationContext { mode: ArtifactReplayPreparationMode::Report, operation_index: 0, generation: 7, base_revision: [9; 32] };
    for case in law["cases"].as_array().unwrap() {
        for bytes in law["grants"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize) {
            let mut callback=|_:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|true;let mut recipient=semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient::new();let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new_retained(&mut callback);control.install_retirement_recipient(&mut recipient).unwrap();
            let base = Arc::new(case["base"].as_str().unwrap().to_string());
            let input = Arc::new(TextInput { text: case["input"].as_str().unwrap().to_string(), inverse_refused: case["inverseRefused"].as_bool().unwrap(), apply_refused: case["applyRefused"].as_bool().unwrap() });
            let calls = Arc::new(AtomicUsize::new(0));
            let cursor = Box::new(TextPreparation::new(base.clone(), context, calls.clone(),Arc::clone(&input)));
            let mut task = ArtifactReplayPreparationTask::new(context, cursor);
            let grant = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: bytes, maximum_capacity_bytes: bytes, maximum_release_bytes: 0, maximum_depth: 64 };
            assert_eq!(task.step(7, [9; 32], grant.maximum_copy_bytes, grant,&mut control, &mut || true).unwrap(), ArtifactReplayPreparationStep::Blocked);
            assert_eq!(calls.load(Ordering::SeqCst), 0);
            assert!(task.step(8, [9; 32], grant.maximum_copy_bytes, grant,&mut control, &mut || false).is_err());
            assert_eq!(calls.load(Ordering::SeqCst), 0);
            for _ in 0..1024 {
                if matches!(task.step(7, [9; 32], grant.maximum_copy_bytes, grant,&mut control, &mut || false).unwrap(), ArtifactReplayPreparationStep::Prepared(_)) { break; }
            }
            assert!(calls.load(Ordering::SeqCst) > 20);
            let mut prepared = task.take_prepared().expect("granted preparation exposed owners");
            assert_eq!(prepared.next.as_deref().unwrap_or(base.as_ref()), case["expected"]["state"].as_str().unwrap());
            assert_eq!(prepared.inverse.is_err(), input.inverse_refused);
            if let Ok(inverse) = &prepared.inverse {
                assert_eq!(inverse.first().map(|value| value.text.as_str()), case["expected"]["inverse"].as_str());
            }
            if let Some(next) = prepared.next.take() {
                let mut owner=SharedControlledRetirement::lease(next);
                while !owner.terminal_is_empty(){let d=owner.next_demand(4096).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert!(drive(grant,|effect|owner.step(effect)).unwrap().progress().fits(grant));}
            }
            if let Ok(inverse) = prepared.inverse {
                for value in inverse {
                    close_owned(value.text);
                }
            }
            close(task.into_retirement(),&mut control);
            for cancel_at in law["cancelAt"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize) {
                let calls = Arc::new(AtomicUsize::new(0));
                let cursor = Box::new(TextPreparation::new(base.clone(), context, calls.clone(),Arc::clone(&input)));
                let mut task = ArtifactReplayPreparationTask::new(context, cursor);
                for _ in 0..cancel_at { task.step(7, [9; 32], grant.maximum_copy_bytes, grant,&mut control, &mut || false).unwrap(); }
                task.cancel();
                let count = calls.load(Ordering::SeqCst);
                assert_eq!(task.step(7, [9; 32], grant.maximum_copy_bytes, grant,&mut control, &mut || false).unwrap(), ArtifactReplayPreparationStep::Blocked);
                assert_eq!(calls.load(Ordering::SeqCst), count);
                close(task.into_retirement(),&mut control);
            }
        }
    }
    assert!(admit_replay_preparation_step(ArtifactReplayPreparationStep::Pending(RetainedCloneProgress { copied_items: 2, ..Default::default() }), ArtifactStoreOneItemGrant { maximum_items: 7, maximum_copy_bytes: 4096, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 64 }).is_err());
}

struct EmptyPreparation { pub payload: [u8; 513], pub closing: bool }
impl ArtifactReplayPreparation<(), ()> for EmptyPreparation {
    fn close_work_demand(&self,_:usize)->Result<u64,ValueError>{Ok(0)}
    fn last_turn_work(&self)->u64{0}
    fn close_demands(&self,_:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{Ok(semio_framework_value::RetirementDemand{depth:1,..Default::default()})}
    fn advance_demands(&self,_:usize,_:u64)->Result<semio_framework_value::RetirementDemand,ValueError>{Ok(semio_framework_value::RetirementDemand{depth:1,..Default::default()})}
    fn advance(&mut self,body:usize,_:ArtifactStoreOneItemGrant,_control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<ArtifactReplayPreparationStep, semio_framework_value::OriginalRetirementFailure> { Ok(ArtifactReplayPreparationStep::Blocked) }
    fn take_prepared(&mut self) -> Option<ArtifactReplayPrepared<(), ()>> { None }
    fn cancel(&mut self) { self.payload[0] = 1; }
    fn begin_close(&mut self) { self.closing = true; }
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    fn next_close_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    fn close_step(&mut self, effect: Effect<'_,'_>, _: Control<'_>) -> Poll { Exit::settle(effect,Ok(RetainedCloneStep::Complete(Default::default()))) }
    fn terminal_is_empty(&self) -> bool { self.closing }
}

#[test]
fn cooperative_history_preparation_receipts_keep_each_physical_currency() {
    let mut callback=|_:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|true;let mut recipient=semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient::new();let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new_retained(&mut callback);control.install_retirement_recipient(&mut recipient).unwrap();
    let law: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for row in law["turnReceipts"]["cases"].as_array().unwrap() {
        let amount = |key: &str| row[key].as_u64().unwrap() as usize;
        let progress = RetainedCloneProgress { copied_items: amount("items"), copied_bytes: amount("copy"), retained_capacity_bytes: amount("capacity"), released_bytes: amount("release") };
        let grant = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: amount("copyGrant"), maximum_capacity_bytes: amount("capacityGrant"), maximum_release_bytes: amount("releaseGrant"), maximum_depth: 1 };
        assert_eq!(admit_replay_preparation_step(ArtifactReplayPreparationStep::Pending(progress), grant).is_ok(), row["accepted"].as_bool().unwrap());
    }
    let context = ArtifactReplayPreparationContext { mode: ArtifactReplayPreparationMode::Report, operation_index: 0, generation: 7, base_revision: [9; 32] };
    let calls = Arc::new(AtomicUsize::new(0));
    let input=Arc::new(TextInput{text:"replacement é".into(),inverse_refused:false,apply_refused:false});
    let cursor = Box::new(TextPreparation::new(Arc::new("original 😀".into()), context, calls.clone(),Arc::clone(&input)));
    let mut task = ArtifactReplayPreparationTask::new(context, cursor);

    let (demand, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| task.demands(7,[9;32],4096,1).unwrap());
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    assert_eq!((demand.copy_bytes,demand.capacity_bytes,demand.release_bytes),(0,"original 😀".len(),0));
    let grant=ArtifactStoreOneItemGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
    assert!(task.demands(8,[9;32],4096,1).is_err());
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let step = task.step(7, [9; 32], grant.maximum_copy_bytes, grant,&mut control, &mut || false).unwrap();
    assert_eq!(step.progress().retained_capacity_bytes, grant.maximum_capacity_bytes);
    assert_eq!(step.progress().copied_bytes, 0);
    task.cancel();
    assert_eq!(task.demands(7,[9;32],4096,1).unwrap(),Default::default());
    close(task.into_retirement(),&mut control);
}

#[test]
fn cooperative_history_preparation_retirement_releases_the_actual_cursor_frame() {
    use semio_framework_trace::observe_heap_allocations_on_this_thread;
    let mut callback=|_:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|true;let mut recipient=semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient::new();let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new_retained(&mut callback);control.install_retirement_recipient(&mut recipient).unwrap();
    let cursor = Box::new(EmptyPreparation { payload: [0; 513], closing: false });
    let (mut owner, heap) = observe_heap_allocations_on_this_thread(|| ArtifactReplayPreparationRetirement::new(cursor));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    let bytes = std::mem::size_of::<EmptyPreparation>();
    assert_eq!(owner.next_copy_byte_demand().unwrap(), 0);
    assert_eq!(owner.next_capacity_byte_demand(4096).unwrap(), 0);
    assert_eq!(owner.next_release_byte_demand().unwrap(), bytes);
    assert_eq!(owner.next_depth_demand().unwrap(), 1);
    let full = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: bytes, maximum_depth: 1 };
    for short in [RetainedCloneGrant::default(), RetainedCloneGrant { maximum_release_bytes: bytes - 1, ..full }, RetainedCloneGrant { maximum_depth: 0, ..full }] {
        let (step, _, heap) = close_turn(&mut owner, &mut control, full, short); let step = step.unwrap();
        assert_eq!(step.progress(), Default::default());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert!(!owner.terminal_is_empty());
    }
    let (step, _, heap) = close_turn(&mut owner, &mut control, full, full); let step = step.unwrap();
    assert_eq!(heap.requested_bytes, 0);
    assert_eq!(heap.released_bytes, bytes);
    assert_eq!(step.progress().released_bytes, bytes);
    assert!(step.progress().fits(full) && owner.terminal_is_empty());
    let (_, heap) = observe_heap_allocations_on_this_thread(|| drop(owner));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
}
struct WorkPreparation{units:u64,close_units:u64,reject:bool,last_work:u64,closing:bool,closed:bool}
impl WorkPreparation{fn perform(&mut self,units:u64){self.last_work=0;for unit in 0..units{std::hint::black_box(unit);self.last_work+=1;}}}
impl ArtifactReplayPreparation<(),()> for WorkPreparation{
 fn close_work_demand(&self,_:usize)->Result<u64,ValueError>{Ok(if self.closing&&!self.closed{self.close_units}else{0})}
 fn advance_demands(&self,_:usize,_:u64)->Result<semio_framework_value::RetirementDemand,ValueError>{Ok(semio_framework_value::RetirementDemand{depth:1,..Default::default()})}
 fn advance(&mut self,_:usize,_:ArtifactStoreOneItemGrant,_:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<ArtifactReplayPreparationStep,semio_framework_value::OriginalRetirementFailure>{self.perform(self.units);if self.reject{return Err(ValueError::invariant("original work fixture refusal").into())}Ok(ArtifactReplayPreparationStep::Pending(Default::default()))}
 fn last_turn_work(&self)->u64{self.last_work}
 fn take_prepared(&mut self)->Option<ArtifactReplayPrepared<(),()>>{None}
 fn cancel(&mut self){self.closing=true;}
 fn begin_close(&mut self){self.closing=true;}
 fn close_demands(&self,_:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{Ok(semio_framework_value::RetirementDemand{depth:1,..Default::default()})}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
 fn next_close_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(1)}
 fn close_step(&mut self,effect:Effect<'_,'_>,_:Control<'_>)->Poll{self.perform(self.close_units);self.closed=true;let result=Exit::pay(effect,u128::from(self.close_units)).map(|()|RetainedCloneStep::Complete(Default::default()));Exit::settle(effect,result)}
 fn terminal_is_empty(&self)->bool{self.closing&&self.closed}
}
#[test]
fn replay_task_preserves_cpu_only_and_failed_child_work_before_result_unwrap(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../🧬️native/🧪️tests/🧫️fixtures/⛽️work.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){
  let mut callback=|_:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|true;let mut recipient=semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient::new();let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new_retained(&mut callback);control.install_retirement_recipient(&mut recipient).unwrap();
  let context=ArtifactReplayPreparationContext{mode:ArtifactReplayPreparationMode::Report,operation_index:0,generation:7,base_revision:[9;32]};
  let cursor=WorkPreparation{units:row["units"].as_u64().unwrap(),close_units:law["closeUnits"].as_u64().unwrap(),reject:row["reject"].as_bool().unwrap(),last_work:0,closing:false,closed:false};let mut task=ArtifactReplayPreparationTask::new(context,Box::new(cursor));
  let g=ArtifactStoreOneItemGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:size_of::<WorkPreparation>(),maximum_depth:64};
  let result=task.step(7,[9;32],0,g,&mut control,&mut||row["deadline"].as_bool().unwrap());
  assert_eq!(task.last_turn_work(),row["expectedWork"].as_u64().unwrap());
  let actual=match result{Err(_)=>"Error",Ok(ArtifactReplayPreparationStep::Blocked)=>"Blocked",Ok(ArtifactReplayPreparationStep::Pending(p))=>{assert_eq!(p,Default::default());"Pending"},Ok(_)=>panic!("CPU-only fixture cannot prepare")};assert_eq!(actual,row["expectedResult"].as_str().unwrap());
  assert_eq!(task.step(7,[9;32],0,g,&mut control,&mut||true).unwrap(),ArtifactReplayPreparationStep::Blocked);assert_eq!(task.last_turn_work(),0);
  let mut close=task.into_retirement();let grant=g.retained_grant();
  let(denied,work,_)=close_turn(&mut close,&mut control,grant,RetainedCloneGrant{maximum_items:0,..grant});assert_eq!(denied.unwrap().progress(),Default::default());assert_eq!(work,0);
  let(first,work,_)=close_turn(&mut close,&mut control,grant,grant);first.unwrap();assert_eq!(work,u128::from(law["closeUnits"].as_u64().unwrap()));assert!(!close.terminal_is_empty());
  let(last,work,_)=close_turn(&mut close,&mut control,grant,grant);last.unwrap();assert_eq!(work,1);assert!(close.terminal_is_empty());
 }
}

#[test]
fn operation_message_close_quotes_one_action_and_preserves_terminal_shell(){let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let law=&law["settlementCloseWork"];let witness=law["witnessBytes"].as_u64().unwrap()as usize;let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let bytes=std::hint::black_box(vec![0u8;witness]);std::hint::black_box(&bytes);drop(bytes)});assert_eq!((heap.requested_bytes,heap.released_bytes),(witness,witness));let outcome=semio_os_spr::MutationReplayOutcome{mutation_id:semio_os_spr::MutationId("original".into()),edit_id:"source".into(),op_index:0,worst:None,messages:Vec::new(),superseded:false,withdrawn:false};let mut owner=OperationMessageSettlement::new(outcome);let mut shell=false;for _ in 0..1000{if owner.terminal_is_empty(){break}let copy=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:0,maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};assert_eq!(owner.next_computational_work(),law["selected"].as_u64().unwrap());assert_eq!(owner.close_step(RetainedCloneGrant{maximum_items:0,..grant}).unwrap().progress(),RetainedCloneProgress::default());assert_eq!(owner.last_turn_work(),law["denied"].as_u64().unwrap());let terminal_shell=owner.retiring.as_ref().is_some_and(semio_os_spr::command::MutationMessageLedgerRetirement::terminal_is_empty);if terminal_shell{assert_eq!(copy,size_of::<Option<semio_os_spr::command::MutationMessageLedgerRetirement>>());assert_eq!(owner.close_step(RetainedCloneGrant{maximum_copy_bytes:copy-1,..grant}).unwrap().progress(),RetainedCloneProgress::default());assert_eq!(owner.last_turn_work(),0)}let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(grant).unwrap());assert_eq!(owner.last_turn_work(),1);assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));shell|=terminal_shell;}assert!(owner.terminal_is_empty()&&shell);}
