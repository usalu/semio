/// 🧰️ Shared vocabulary of the companion test targets: the public pack-json surface, fixture readers and the per-thread original effect bank.
pub use semio_framework_pack_json::testkit::JsonLexeme;
pub use semio_framework_pack_json::*;
pub use semio_framework_pack_json_source_sink::write_json_source_into;
pub use semio_framework_trace_memory_testing::observe_backing;
pub use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
pub use semio_framework_value::{DslValue, RetainedAllocationEffect, RetainedWorkOutcome, ValueError, ValueRefusalKind};
pub use semio_framework_value_testing_admission as admission;

use semio_framework_test_fixtures::fixture_path;
use std::{
    cell::RefCell,
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Mutex, OnceLock},
};

pub fn product_directory() -> &'static Path {
    static DIRECTORY: OnceLock<PathBuf> = OnceLock::new();
    DIRECTORY.get_or_init(|| fixture_path!("../../../.."))
}

fn leaked(relative: &str) -> &'static [u8] {
    static READ: OnceLock<Mutex<HashMap<String, &'static [u8]>>> = OnceLock::new();
    let mut read = READ.get_or_init(Default::default).lock().unwrap();
    read.entry(relative.to_owned()).or_insert_with(|| {
        let path = product_directory().join(relative);
        Box::leak(std::fs::read(&path).unwrap_or_else(|error| panic!("test input {}: {error}", path.display())).into_boxed_slice())
    })
}

pub fn text(relative: &str) -> &'static str {
    std::str::from_utf8(leaked(relative)).unwrap_or_else(|error| panic!("test input {relative} is not UTF-8: {error}"))
}

pub fn bytes(relative: &str) -> &'static [u8] {
    leaked(relative)
}

thread_local! {
    static BANK: RefCell<Option<admission::EffectBank>> = const { RefCell::new(None) };
}

fn new_bank() -> admission::EffectBank {
    admission::EffectBank::new(admission::BankWallet::from_profile(&serde_json::json!({
        "items": 1u64 << 30,
        "copyBytes": 1u64 << 40,
        "capacityBytes": 1u64 << 40,
        "releaseBytes": 1u64 << 40,
        "depth": 1u64 << 20,
        "fuel": 1u64 << 60,
    })))
}

/// 🔥️ Builds this thread's original effect bank before a test observes allocations.
pub fn warm() {
    BANK.with(|slot| {
        let mut slot = slot.borrow_mut();
        if slot.is_none() {
            *slot = Some(new_bank())
        }
    })
}

/// 🎟️ Lends one real original effect of `grant`, nested in a driven producer action so sealed retirement finds its invocation; the bank is warmed so observed regions see no scaffold.
pub fn fx<R>(grant: RetainedCloneGrant, run: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> R) -> R {
    BANK.with(|slot| {
        let mut bank = slot.borrow_mut().take().unwrap_or_else(new_bank);
        let (mut run, mut result) = (Some(run), None);
        let (poll, _) = admission::driven::<ValueError>(&mut bank, grant, 1 << 20, |effect, reply, _| {
            result = Some((run.take().expect("single driven action"))(effect));
            Ok(reply.complete(Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default()))))
        });
        match poll {
            admission::RetainedWorkPoll::Complete(turn) => turn.result.expect("driven action"),
            admission::RetainedWorkPoll::Pending { .. } => panic!("driven action stayed pending"),
        };
        bank.take_admitted();
        *slot.borrow_mut() = Some(bank);
        result.expect("effect call ran")
    })
}

/// 🎟️ Lends one real original effect to a call that polls: a synchronous turn completes, so its outcome is returned.
pub fn fp<T, E>(grant: RetainedCloneGrant, run: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> admission::RetainedWorkPoll<T, E>) -> RetainedWorkOutcome<T, E> {
    match fx(grant, run) {
        admission::RetainedWorkPoll::Complete(turn) => turn,
        admission::RetainedWorkPoll::Pending { .. } => panic!("synchronous original turn stayed pending"),
    }
}

/// 🗑️ Drives one controlled-retirement step under a genuine producer authority with the caller's own grant and body.
pub fn retire_step<T: semio_framework_value::retirement::RetireOwned>(owner: &mut semio_framework_value::retirement::controlled::ControlledRetirement<T>, grant: RetainedCloneGrant, body: usize) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::OriginalRetirementFailure> {
    BANK.with(|slot| {
        let mut bank = slot.borrow_mut().take().unwrap_or_else(new_bank);
        let work = owner.next_computational_work(body).unwrap().max(1);
        let ((poll, _), heap) = semio_framework_value_testing_allocator::observe_retirement_allocations(|| admission::driven::<semio_framework_value::OriginalRetirementFailure>(&mut bank, grant, work, |effect, reply, recover| if recover { owner.recover_step_into(body, effect, reply) } else { owner.step_into(body, effect, reply) }));
        let result = match poll {
            admission::RetainedWorkPoll::Complete(turn) => turn.result,
            admission::RetainedWorkPoll::Pending { .. } => panic!("driven retirement stayed pending"),
        };
        if let Ok(step) = &result {
            bank.post(step.progress(), grant, heap);
        }
        *slot.borrow_mut() = Some(bank);
        result
    })
}
