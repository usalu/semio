//! 🧪️ Neutral cooperative operation laws with UTF-8 work and exact cancellation ownership.

use super::*;
use semio_framework_value::retirement::{RetireOwned, controlled::ControlledRetirement, arc_custody::SharedControlledRetirement};
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(semio_framework_value::RetireOwned)]
struct TextInput {
    pub text: String,
    inverse_refused: bool,
    apply_refused: bool,
}

struct TextPreparation {
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
        Self { original:Some(original),input_close:None,base: Some(base), inverse: None, next: None, prepared: None, active: None, shared: None, inverse_list: None, phase: 0, context, calls, cancelled: false, closing: false }
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
    fn close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{let mut demand=if let Some(owner)=self.input_close.as_ref(){owner.next_demand(body)?}else if let Some(owner)=self.active.as_ref(){owner.next_demand(body)?}else if let Some(owner)=self.shared.as_ref(){owner.next_demand(body)?}else{self.inverse_list.as_ref().map_or(Ok(Default::default()),|owner|owner.next_demand(body))?};demand.depth=demand.depth.max(1);Ok(demand)}
    fn advance_demands(&self,_body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{Ok(semio_framework_value::RetirementDemand{copy_bytes:match self.phase{1=>self.base.as_ref().unwrap().get(self.inverse.as_ref().unwrap().len()..).unwrap().chars().next().map_or(0,char::len_utf8),3=>self.original.as_ref().unwrap().text.get(self.next.as_ref().unwrap().len()..).unwrap().chars().next().map_or(0,char::len_utf8),_=>0},capacity_bytes:match self.phase{0=>self.base.as_ref().unwrap().len(),2=>self.original.as_ref().unwrap().text.len(),_=>0},release_bytes:0,depth:1})}
    fn advance(&mut self, body:usize,grant: ArtifactStoreOneItemGrant,_control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<ArtifactReplayPreparationStep, ValueError> {
        if self.cancelled || self.closing || !grant.permits_one() { return Ok(ArtifactReplayPreparationStep::Blocked); }
        let original=self.original.as_ref().unwrap();
        self.calls.fetch_add(1, Ordering::SeqCst);
        let base = self.base.as_ref().expect("immutable fixture base");
        let mut progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        match self.phase {
            0 => {
                if base.len() > grant.maximum_capacity_bytes { return Ok(ArtifactReplayPreparationStep::Blocked); }
                let mut inverse = String::new();
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
                let mut next = String::new();
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
    fn close_step(&mut self,grant:RetainedCloneGrant,receiving:&mut dyn semio_framework_value::OriginalRetirementControl)->Result<RetainedCloneStep,ValueError>{
        let empty=RetainedCloneProgress::default();
        if !self.closing||grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(empty));}
        let demand=self.close_demands(grant.maximum_copy_bytes)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(empty));}if demand.capacity_bytes!=0{receiving.charge(demand.capacity_bytes)?;}
        if let Some(owner)=self.input_close.as_mut(){let step=owner.step(grant)?;if owner.terminal_is_empty(){self.input_close=None;}return Ok(step);}
        if let Some(owner)=self.active.as_mut(){let step=owner.step(grant)?;if owner.terminal_is_empty(){self.active=None;}return Ok(step);}
        if let Some(owner)=self.shared.as_mut(){let step=owner.step(grant)?;if owner.terminal_is_empty(){self.shared=None;}return Ok(step);}
        if let Some(owner)=self.inverse_list.as_mut(){let step=owner.step(grant)?;if owner.terminal_is_empty(){self.inverse_list=None;}return Ok(step);}
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
    fn terminal_is_empty(&self)->bool{self.closing&&self.original.is_none()&&self.input_close.is_none()&&self.base.is_none()&&self.inverse.is_none()&&self.next.is_none()&&self.prepared.is_none()&&self.active.is_none()&&self.shared.is_none()&&self.inverse_list.is_none()}

}

impl Drop for TextPreparation {
    fn drop(&mut self) { assert!(self.terminal_is_empty() || std::thread::panicking(), "fixture replay producer dropped live owners"); }
}

fn close(mut retirement:ArtifactReplayPreparationRetirement<String,TextInput>,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>){
    let mut last=None;
    for _ in 0..4096 {
        let body=retirement.next_copy_byte_demand().unwrap();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:body,maximum_capacity_bytes:retirement.next_capacity_byte_demand(body).unwrap(),maximum_release_bytes:retirement.next_release_byte_demand().unwrap(),maximum_depth:retirement.next_depth_demand().unwrap()};
        let step=control.with_retirement(grant,|receiving|retirement.close_step(grant,receiving)).expect("exact admitted fixture close");assert!(step.progress().fits(grant));
        last=Some((grant,step));
        if retirement.terminal_is_empty(){return;}
    }
    panic!("fixture replay preparation never closed; last quoted grant and step {last:?}");
}
fn close_owned<T:RetireOwned>(value:T){let mut owner=ControlledRetirement::new(value).unwrap_or_else(|(error,_)|panic!("fixture native owner refused: {error}"));for _ in 0..4096{if owner.terminal_is_empty(){return;}let body=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:body,maximum_capacity_bytes:owner.next_capacity_byte_demand(body).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};assert!(owner.step(grant).unwrap().progress().fits(grant));}panic!("fixture owner did not close");}

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
                while !owner.terminal_is_empty(){let body=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:body,maximum_capacity_bytes:owner.next_capacity_byte_demand(body).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};assert!(owner.step(grant).unwrap().progress().fits(grant));}
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
    fn close_demands(&self,_:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{Ok(semio_framework_value::RetirementDemand{depth:1,..Default::default()})}
    fn advance_demands(&self,_:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{Ok(semio_framework_value::RetirementDemand{depth:1,..Default::default()})}
    fn advance(&mut self,body:usize,_:ArtifactStoreOneItemGrant,_control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<ArtifactReplayPreparationStep, ValueError> { Ok(ArtifactReplayPreparationStep::Blocked) }
    fn take_prepared(&mut self) -> Option<ArtifactReplayPrepared<(), ()>> { None }
    fn cancel(&mut self) { self.payload[0] = 1; }
    fn begin_close(&mut self) { self.closing = true; }
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    fn next_close_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    fn close_step(&mut self, _: RetainedCloneGrant, _: &mut dyn semio_framework_value::OriginalRetirementControl) -> Result<RetainedCloneStep, ValueError> { Ok(RetainedCloneStep::Complete(Default::default())) }
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

    let (demand, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| task.demands(7,[9;32],4096).unwrap());
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    assert_eq!((demand.copy_bytes,demand.capacity_bytes,demand.release_bytes),(0,"original 😀".len(),0));
    let grant=ArtifactStoreOneItemGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
    assert!(task.demands(8,[9;32],4096).is_err());
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let step = task.step(7, [9; 32], grant.maximum_copy_bytes, grant,&mut control, &mut || false).unwrap();
    assert_eq!(step.progress().retained_capacity_bytes, grant.maximum_capacity_bytes);
    assert_eq!(step.progress().copied_bytes, 0);
    task.cancel();
    assert_eq!(task.demands(7,[9;32],4096).unwrap(),Default::default());
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
        let (step, heap) = observe_heap_allocations_on_this_thread(|| control.with_retirement(full,|receiving|owner.close_step(short,receiving)).unwrap());
        assert_eq!(step.progress(), Default::default());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert!(!owner.terminal_is_empty());
    }
    let (step, heap) = observe_heap_allocations_on_this_thread(|| control.with_retirement(full,|receiving|owner.close_step(full,receiving)).unwrap());
    assert_eq!(heap.requested_bytes, 0);
    assert_eq!(heap.released_bytes, bytes);
    assert_eq!(step.progress().released_bytes, bytes);
    assert!(step.progress().fits(full) && owner.terminal_is_empty());
    let (_, heap) = observe_heap_allocations_on_this_thread(|| drop(owner));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
}
