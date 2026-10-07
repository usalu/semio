//! 🧪️ Neutral cooperative operation laws with UTF-8 work and exact cancellation ownership.

use super::*;
use semio_framework_value::retirement::{owned_retirement, shared_lease_retirement};
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
    active: Option<Box<dyn ErasedSnapshotRetirement>>,
    phase: u8,
    context: ArtifactReplayPreparationContext,
    calls: Arc<AtomicUsize>,
    cancelled: bool,
    closing: bool,
}

impl TextPreparation {
    fn new(base: Arc<String>, context: ArtifactReplayPreparationContext, calls: Arc<AtomicUsize>) -> Self {
        Self { base: Some(base), inverse: None, next: None, prepared: None, active: None, phase: 0, context, calls, cancelled: false, closing: false }
    }

    fn copy_scalar(source: &str, destination: &mut String, maximum_bytes: usize) -> ArtifactReplayPreparationProgress {
        let Some(scalar) = source[destination.len()..].chars().next() else { return ArtifactReplayPreparationProgress { items: 1, bytes: 0 } };
        let bytes = scalar.len_utf8();
        if bytes > maximum_bytes { return ArtifactReplayPreparationProgress::default(); }
        destination.push(scalar);
        ArtifactReplayPreparationProgress { items: 1, bytes }
    }
}

impl ArtifactReplayPreparation<String, TextInput> for TextPreparation {
    fn advance(&mut self, original: &TextInput, replacement: Option<&InputReplacement>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactReplayPreparationStep, String> {
        if self.cancelled || self.closing || !grant.permits_one() { return Ok(ArtifactReplayPreparationStep::Blocked); }
        assert!(replacement.is_none());
        self.calls.fetch_add(1, Ordering::SeqCst);
        let base = self.base.as_ref().expect("immutable fixture base");
        let mut progress = ArtifactReplayPreparationProgress { items: 1, bytes: 0 };
        match self.phase {
            0 => {
                if base.len() > grant.maximum_bytes { return Ok(ArtifactReplayPreparationStep::Blocked); }
                let mut inverse = String::new();
                inverse.try_reserve_exact(base.len()).map_err(|error| error.to_string())?;
                progress.bytes = inverse.capacity();
                self.inverse = Some(inverse);
                self.phase = 1;
            }
            1 => {
                let inverse = self.inverse.as_mut().expect("inverse owner");
                progress = Self::copy_scalar(base, inverse, grant.maximum_bytes);
                if inverse.len() == base.len() { self.phase = 2; }
            }
            2 => {
                if original.text.len() > grant.maximum_bytes { return Ok(ArtifactReplayPreparationStep::Blocked); }
                let mut next = String::new();
                next.try_reserve_exact(original.text.len()).map_err(|error| error.to_string())?;
                progress.bytes = next.capacity();
                self.next = Some(next);
                self.phase = 3;
            }
            3 => {
                let next = self.next.as_mut().expect("next owner");
                progress = Self::copy_scalar(&original.text, next, grant.maximum_bytes);
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
            5 => return Ok(ArtifactReplayPreparationStep::Prepared(ArtifactReplayPreparationProgress::default())),
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

    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, ValueError> {
        if !self.closing || !grant.permits_one() { return Ok(SnapshotRetirementStep::Blocked); }
        if let Some(active) = self.active.as_mut() {
            let step = active.close_step(1, grant.maximum_bytes)?;
            if step == SnapshotRetirementStep::Complete {
                assert!(active.terminal_is_empty());
                self.active = None;
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        if let Some(prepared) = self.prepared.as_mut() {
            if let Some(next) = prepared.next.take() { self.active = Some(shared_lease_retirement(next)); }
            else if let Ok(inverse) = &mut prepared.inverse {
                if let Some(inverse) = inverse.pop() { self.active = Some(owned_retirement(inverse)); }
                else { self.prepared = None; }
            } else { self.prepared = None; }
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(inverse) = self.inverse.take().or_else(|| self.next.take()) {
            self.active = Some(owned_retirement(inverse));
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            self.active = Some(shared_lease_retirement(base));
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool { self.closing && self.base.is_none() && self.inverse.is_none() && self.next.is_none() && self.prepared.is_none() && self.active.is_none() }
}

impl Drop for TextPreparation {
    fn drop(&mut self) { assert!(self.terminal_is_empty() || std::thread::panicking(), "fixture replay producer dropped live owners"); }
}

fn close(mut retirement: ArtifactReplayPreparationRetirement<String, TextInput>) {
    for _ in 0..4096 {
        if retirement.close_step(1, 1).expect("one-byte exact close") == SnapshotRetirementStep::Complete {
            assert!(retirement.terminal_is_empty());
            return;
        }
    }
    panic!("fixture replay preparation never closed");
}

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
            let grant = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: bytes };
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
                let mut owner = shared_lease_retirement(next);
                while owner.close_step(1, 1).unwrap() != SnapshotRetirementStep::Complete {}
                assert!(owner.terminal_is_empty());
            }
            if let Ok(inverse) = prepared.inverse {
                for value in inverse {
                    let mut owner = owned_retirement(value.text);
                    while owner.close_step(1, 1).unwrap() != SnapshotRetirementStep::Complete {}
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
    assert!(admit_replay_preparation_step(ArtifactReplayPreparationStep::Pending(ArtifactReplayPreparationProgress { items: 2, bytes: 0 }), ArtifactStoreOneItemGrant { maximum_items: 7, maximum_bytes: 4096 }).is_err());
    println!("[DEBUG] cooperative operation neutral UTF-8, freshness, deadline, inverse/apply refusal and terminal cancellation laws complete");
}
