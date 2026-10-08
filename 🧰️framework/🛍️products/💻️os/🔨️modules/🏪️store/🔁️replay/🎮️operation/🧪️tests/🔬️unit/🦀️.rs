//! 🧪️ Neutral cooperative operation laws with UTF-8 work and exact cancellation ownership.

use super::*;
use semio_framework_value::retirement::{RetireOwned, controlled::ControlledRetirement, shared::SharedControlledRetirement};
use std::sync::atomic::{AtomicUsize, Ordering};

struct TextInput {
    text: String,
    inverse_refused: bool,
    apply_refused: bool,
}

struct TextPreparation {
    base: Option<Arc<String>>,
    inverse: Option<String>,
    next: Option<String>,
    prepared: Option<ArtifactReplayPrepared<String, String>>,
    active: Option<ControlledRetirement<String>>,
    shared: Option<SharedControlledRetirement<String>>,
    inverse_list: Option<ControlledRetirement<semio_framework_value::list::PagedList<String, {usize::MAX}>>>,
    phase: u8,
    context: ArtifactReplayPreparationContext,
    calls: Arc<AtomicUsize>,
    cancelled: bool,
    closing: bool,
}

impl TextPreparation {
    fn new(base: Arc<String>, context: ArtifactReplayPreparationContext, calls: Arc<AtomicUsize>) -> Self {
        Self { base: Some(base), inverse: None, next: None, prepared: None, active: None, shared: None, inverse_list: None, phase: 0, context, calls, cancelled: false, closing: false }
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
    fn next_advance_copy_demand(&self, original: &TextInput, _: Option<&InputReplacement>) -> Result<usize, ValueError> { Ok(match self.phase { 1 => self.base.as_ref().unwrap().get(self.inverse.as_ref().unwrap().len()..).unwrap().chars().next().map_or(0,char::len_utf8), 3 => original.text.get(self.next.as_ref().unwrap().len()..).unwrap().chars().next().map_or(0,char::len_utf8), _ => 0 }) }
    fn next_advance_capacity_demand(&self, original: &TextInput, _: Option<&InputReplacement>, _: usize) -> Result<usize, ValueError> { Ok(match self.phase { 0 => self.base.as_ref().unwrap().len(), 2 => original.text.len(), _ => 0 }) }
    fn next_advance_release_demand(&self, _: &TextInput, _: Option<&InputReplacement>) -> Result<usize, ValueError> { Ok(0) }
    fn next_advance_depth_demand(&self, _: &TextInput, _: Option<&InputReplacement>) -> Result<usize, ValueError> { Ok(1) }
    fn advance(&mut self, original: &TextInput, replacement: Option<&InputReplacement>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactReplayPreparationStep, String> {
        if self.cancelled || self.closing || !grant.permits_one() { return Ok(ArtifactReplayPreparationStep::Blocked); }
        assert!(replacement.is_none());
        self.calls.fetch_add(1, Ordering::SeqCst);
        let base = self.base.as_ref().expect("immutable fixture base");
        let mut progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        match self.phase {
            0 => {
                if base.len() > grant.maximum_capacity_bytes { return Ok(ArtifactReplayPreparationStep::Blocked); }
                let mut inverse = String::new();
                inverse.try_reserve_exact(base.len()).map_err(|error| error.to_string())?;
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
                next.try_reserve_exact(original.text.len()).map_err(|error| error.to_string())?;
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

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { if let Some(owner)=self.active.as_ref(){return owner.next_copy_byte_demand();} if let Some(owner)=self.shared.as_ref(){return owner.next_copy_byte_demand();} self.inverse_list.as_ref().map_or(Ok(0),ControlledRetirement::next_copy_byte_demand) }
    fn next_close_capacity_byte_demand(&self, bytes:usize) -> Result<usize, ValueError> { if let Some(owner)=self.active.as_ref(){return owner.next_capacity_byte_demand(bytes);} if let Some(owner)=self.shared.as_ref(){return owner.next_capacity_byte_demand(bytes);} self.inverse_list.as_ref().map_or(Ok(0),|owner|owner.next_capacity_byte_demand(bytes)) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { if let Some(owner)=self.active.as_ref(){return owner.next_release_byte_demand();} if let Some(owner)=self.shared.as_ref(){return owner.next_release_byte_demand();} self.inverse_list.as_ref().map_or(Ok(0),ControlledRetirement::next_release_byte_demand) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { if let Some(owner)=self.active.as_ref(){return owner.next_depth_demand();} if let Some(owner)=self.shared.as_ref(){return owner.next_depth_demand();} self.inverse_list.as_ref().map_or(Ok(1),ControlledRetirement::next_depth_demand) }
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let empty=RetainedCloneProgress::default();
        if !self.closing||grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(empty));}
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
        Ok(RetainedCloneStep::Complete(empty))
    }
    fn terminal_is_empty(&self)->bool{self.closing&&self.base.is_none()&&self.inverse.is_none()&&self.next.is_none()&&self.prepared.is_none()&&self.active.is_none()&&self.shared.is_none()&&self.inverse_list.is_none()}

}

impl Drop for TextPreparation {
    fn drop(&mut self) { assert!(self.terminal_is_empty() || std::thread::panicking(), "fixture replay producer dropped live owners"); }
}

fn close(mut retirement:ArtifactReplayPreparationRetirement<String,TextInput>){
    for _ in 0..4096 {
        let body=retirement.next_copy_byte_demand().unwrap();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:body,maximum_capacity_bytes:retirement.next_capacity_byte_demand(body).unwrap(),maximum_release_bytes:retirement.next_release_byte_demand().unwrap(),maximum_depth:retirement.next_depth_demand().unwrap()};
        let step=retirement.close_step(grant).expect("exact admitted fixture close");assert!(step.progress().fits(grant));
        if retirement.terminal_is_empty(){return;}
    }
    panic!("fixture replay preparation never closed");
}
fn close_owned<T:RetireOwned>(value:T){let mut owner=ControlledRetirement::new(value).unwrap_or_else(|(error,_)|panic!("fixture native owner refused: {error}"));for _ in 0..4096{if owner.terminal_is_empty(){return;}let body=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:body,maximum_capacity_bytes:owner.next_capacity_byte_demand(body).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};assert!(owner.step(grant).unwrap().progress().fits(grant));}panic!("fixture owner did not close");}

#[test]
fn cooperative_history_operation_obeys_the_neutral_law() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("neutral operation law");
    let context = ArtifactReplayPreparationContext { mode: ArtifactReplayPreparationMode::Report, operation_index: 0, generation: 7, base_revision: [9; 32] };
    for case in law["cases"].as_array().unwrap() {
        for bytes in law["grants"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize) {
            let base = Arc::new(case["base"].as_str().unwrap().to_string());
            let input = TextInput { text: case["input"].as_str().unwrap().to_string(), inverse_refused: case["inverseRefused"].as_bool().unwrap(), apply_refused: case["applyRefused"].as_bool().unwrap() };
            let calls = Arc::new(AtomicUsize::new(0));
            let cursor = Box::new(TextPreparation::new(base.clone(), context, calls.clone()));
            let mut task = ArtifactReplayPreparationTask::new(context, cursor);
            let grant = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: bytes, maximum_capacity_bytes: bytes, maximum_release_bytes: 0, maximum_depth: 64 };
            assert_eq!(task.step(&input, None, 7, [9; 32], grant, &mut || true).unwrap(), ArtifactReplayPreparationStep::Blocked);
            assert_eq!(calls.load(Ordering::SeqCst), 0);
            assert!(task.step(&input, None, 8, [9; 32], grant, &mut || false).is_err());
            assert_eq!(calls.load(Ordering::SeqCst), 0);
            for _ in 0..1024 {
                if matches!(task.step(&input, None, 7, [9; 32], grant, &mut || false).unwrap(), ArtifactReplayPreparationStep::Prepared(_)) { break; }
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
            close(task.into_retirement());
            for cancel_at in law["cancelAt"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize) {
                let calls = Arc::new(AtomicUsize::new(0));
                let cursor = Box::new(TextPreparation::new(base.clone(), context, calls.clone()));
                let mut task = ArtifactReplayPreparationTask::new(context, cursor);
                for _ in 0..cancel_at { task.step(&input, None, 7, [9; 32], grant, &mut || false).unwrap(); }
                task.cancel();
                let count = calls.load(Ordering::SeqCst);
                assert_eq!(task.step(&input, None, 7, [9; 32], grant, &mut || false).unwrap(), ArtifactReplayPreparationStep::Blocked);
                assert_eq!(calls.load(Ordering::SeqCst), count);
                close(task.into_retirement());
            }
        }
    }
    assert!(admit_replay_preparation_step(ArtifactReplayPreparationStep::Pending(RetainedCloneProgress { copied_items: 2, ..Default::default() }), ArtifactStoreOneItemGrant { maximum_items: 7, maximum_copy_bytes: 4096, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 64 }).is_err());
    println!("[DEBUG] cooperative operation neutral UTF-8, freshness, deadline, inverse/apply refusal and terminal cancellation laws complete");
}

struct EmptyPreparation { payload: [u8; 513], closing: bool }
impl ArtifactReplayPreparation<(), ()> for EmptyPreparation {
    fn next_advance_copy_demand(&self, _: &(), _: Option<&InputReplacement>) -> Result<usize, ValueError> { Ok(0) }
    fn next_advance_capacity_demand(&self, _: &(), _: Option<&InputReplacement>, _: usize) -> Result<usize, ValueError> { Ok(0) }
    fn next_advance_release_demand(&self, _: &(), _: Option<&InputReplacement>) -> Result<usize, ValueError> { Ok(0) }
    fn next_advance_depth_demand(&self, _: &(), _: Option<&InputReplacement>) -> Result<usize, ValueError> { Ok(1) }
    fn advance(&mut self, _: &(), _: Option<&InputReplacement>, _: ArtifactStoreOneItemGrant) -> Result<ArtifactReplayPreparationStep, String> { Ok(ArtifactReplayPreparationStep::Blocked) }
    fn take_prepared(&mut self) -> Option<ArtifactReplayPrepared<(), ()>> { None }
    fn cancel(&mut self) { self.payload[0] = 1; }
    fn begin_close(&mut self) { self.closing = true; }
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    fn next_close_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    fn close_step(&mut self, _: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { Ok(RetainedCloneStep::Complete(Default::default())) }
    fn terminal_is_empty(&self) -> bool { self.closing }
}

#[test]
fn cooperative_history_preparation_receipts_keep_each_physical_currency() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for row in law["turnReceipts"]["cases"].as_array().unwrap() {
        let amount = |key: &str| row[key].as_u64().unwrap() as usize;
        let progress = RetainedCloneProgress { copied_items: amount("items"), copied_bytes: amount("copy"), retained_capacity_bytes: amount("capacity"), released_bytes: amount("release") };
        let grant = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: amount("copyGrant"), maximum_capacity_bytes: amount("capacityGrant"), maximum_release_bytes: amount("releaseGrant"), maximum_depth: 1 };
        assert_eq!(admit_replay_preparation_step(ArtifactReplayPreparationStep::Pending(progress), grant).is_ok(), row["accepted"].as_bool().unwrap());
        println!("[DEBUG] operation admitted actual receipt={progress:?} grant={grant:?} expected={}", row["accepted"]);
    }
    let context = ArtifactReplayPreparationContext { mode: ArtifactReplayPreparationMode::Report, operation_index: 0, generation: 7, base_revision: [9; 32] };
    let calls = Arc::new(AtomicUsize::new(0));
    let cursor = Box::new(TextPreparation::new(Arc::new("original 😀".into()), context, calls.clone()));
    let mut task = ArtifactReplayPreparationTask::new(context, cursor);
    let input = TextInput { text: "replacement é".into(), inverse_refused: false, apply_refused: false };
    let (grant, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| task.next_grant(&input, None, 7, [9; 32]).unwrap());
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    assert_eq!((grant.maximum_copy_bytes, grant.maximum_capacity_bytes, grant.maximum_release_bytes), (0, "original 😀".len(), 0));
    assert!(task.next_grant(&input, None, 8, [9; 32]).is_err());
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let step = task.step(&input, None, 7, [9; 32], grant, &mut || false).unwrap();
    assert_eq!(step.progress().retained_capacity_bytes, grant.maximum_capacity_bytes);
    assert_eq!(step.progress().copied_bytes, 0);
    task.cancel();
    assert_eq!(task.next_grant(&input, None, 7, [9; 32]).unwrap().maximum_items, 0);
    close(task.into_retirement());
}

#[test]
fn cooperative_history_preparation_retirement_releases_the_actual_cursor_frame() {
    use semio_framework_trace::observe_heap_allocations_on_this_thread;
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
        let (step, heap) = observe_heap_allocations_on_this_thread(|| owner.close_step(short).unwrap());
        assert_eq!(step.progress(), Default::default());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert!(!owner.terminal_is_empty());
    }
    let (step, heap) = observe_heap_allocations_on_this_thread(|| owner.close_step(full).unwrap());
    assert_eq!(heap.requested_bytes, 0);
    assert_eq!(heap.released_bytes, bytes);
    assert_eq!(step.progress().released_bytes, bytes);
    assert!(step.progress().fits(full) && owner.terminal_is_empty());
    let (_, heap) = observe_heap_allocations_on_this_thread(|| drop(owner));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    println!("[DEBUG] preparation original erased frame exact release={bytes} zero/one-below/depth pause retained");
}
