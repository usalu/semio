//! 🧊️ A prepared Core actor opens twice under independent original caller authority and retires every retained extent.

use semio_framework_actor::retained_context::OriginalActorContextSlot;
use semio_framework_actor::retained_turn::OriginalActorTurnWork;
use semio_framework_actor::{OriginalGuestExecutionBudget, RetainedTurnInput};
use semio_framework_job::{OriginalTurnFailure, StepBudget, StepContext};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
use semio_plugin_interpreter_runtime::{OwnedActorStartupStatus, OwnedCompiledGuest, OwnedGuestInstance, OwnedRuntime};
use serde_json::Value;

#[path="actor-clock-host.rs"]
mod actor_clock_host;

fn clock() -> Option<u64> { Some(1000) }

fn unsigned(mut value: usize, destination: &mut Vec<u8>) {
    while value >= 128 {
        destination.push((value & 127) as u8 | 128);
        value >>= 7;
    }
    destination.push(value as u8);
}

fn component(fixture: &Value) -> Vec<u8> {
    let hex = fixture["coreHex"].as_str().unwrap();
    let mut bytes = b"\0asm\x0d\0\x01\0\x01".to_vec();
    unsigned(hex.len() / 2, &mut bytes);
    for index in (0..hex.len()).step_by(2) {
        bytes.push(u8::from_str_radix(&hex[index..index + 2], 16).unwrap());
    }
    bytes
}

struct Source {
    owner: OriginalActorContextSlot,
    input: RetainedTurnInput,
    last: RetainedCloneProgress,
    born: usize,
    released: usize,
}

impl Source {
    fn admit() -> Self {
        let mut source = Self {
            owner: OriginalActorContextSlot::empty(),
            input: RetainedTurnInput {
                operation: 71, generation: 3, epoch: 1,
                work: OriginalActorTurnWork { clock_binding: 195, absolute_deadline_us: 100_000, fuel: 100_000 },
                grant: RetainedCloneGrant { maximum_items: 20_000, maximum_copy_bytes: 4_000_000, maximum_capacity_bytes: 4_000_000, maximum_release_bytes: 4_000_000, maximum_depth: 64 },
            },
            last: Default::default(),
            born: 0,
            released: 0,
        };
        for _ in 0..30 {
            let call = source.owner.admit_step(source.input, source.budget(65_536), clock);
            source.settle(call.receipt_spent, call.work_units_spent);
            if call.result.unwrap().ready { return source }
        }
        panic!("original caller did not admit its Source");
    }

    fn budget(&self, copy: usize) -> StepBudget {
        StepBudget::new(2000, self.input.work.absolute_deadline_us, RetainedCloneGrant {
            maximum_copy_bytes: copy, maximum_items: 128, maximum_capacity_bytes: 200_000, maximum_release_bytes: 200_000, maximum_depth: 64,
        })
    }

    fn settle(&mut self, progress: RetainedCloneProgress, work: u64) {
        let receipt = self.input.return_original(progress, work).unwrap();
        self.input.epoch += 1;
        self.input.work.fuel = receipt.remaining_fuel;
        self.input.grant = receipt.remaining;
    }

    fn turn<T>(&mut self, body: impl FnOnce(&mut StepContext<'_>) -> Result<T, OriginalTurnFailure>) -> Result<T, OriginalTurnFailure> {
        let budget = self.budget(4096);
        let mut received = RetainedCloneProgress::default();
        let mut physical = (0, 0);
        let call = self.owner.dispatch_step(self.input, budget, &mut received, clock, |cx| {
            let before = cx.retained_progress();
            let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| body(cx));
            let after = cx.retained_progress();
            self.last = RetainedCloneProgress {
                copied_items: after.copied_items - before.copied_items,
                copied_bytes: after.copied_bytes - before.copied_bytes,
                retained_capacity_bytes: after.retained_capacity_bytes - before.retained_capacity_bytes,
                released_bytes: after.released_bytes - before.released_bytes,
            };
            physical = (heap.requested_bytes, heap.released_bytes);
            assert_eq!((self.last.retained_capacity_bytes, self.last.released_bytes), physical);
            Ok::<_, OriginalTurnFailure>(result)
        });
        self.settle(call.receipt_spent, call.work_units_spent);
        self.born += physical.0;
        self.released += physical.1;
        call.result.unwrap()
    }

    fn open(&mut self, runtime: &OwnedRuntime, compiled: &OwnedCompiledGuest, channel: u32, instructions: u64) -> OwnedGuestInstance {
        let mut owner = self.turn(|cx| runtime.begin_actor_startup(compiled, cx)).unwrap();
        let mut spent = 0;
        for _ in 0..250 {
            let startup = owner.as_mut().unwrap();
            if startup.ready() { break }
            let call = self.turn(|cx| {
                let input = RetainedTurnInput::borrow_original(cx)?;
                let budget = startup.requires_execution().then(|| OriginalGuestExecutionBudget { input, fuel: 1, deadline_us: cx.deadline_us() });
                let call = startup.advance_step(compiled, cx, budget)?;
                assert_eq!(call.execution.is_some(), budget.is_some());
                if let (Some(receipt), Some(issued)) = (call.execution, budget) {
                    receipt.budget.validate_narrowing(issued)?;
                    receipt.validate_for(receipt.budget)?;
                    spent += receipt.spent_fuel.unwrap();
                }
                call.result
            }).unwrap();
            assert!(matches!(call, OwnedActorStartupStatus::Pending | OwnedActorStartupStatus::Ready));
        }
        assert!(owner.as_ref().unwrap().ready());
        assert_eq!(owner.as_ref().unwrap().observed_channel(), Some(channel));
        assert_eq!(spent, instructions);
        for _ in 0..4 {
            if let Some(instance) = self.turn(|cx| runtime.take_actor_startup_step(&mut owner, compiled, RetainedTurnInput::borrow_original(cx)?, cx)).unwrap() {
                assert!(owner.is_none());
                assert_eq!(instance.original_startup_input().unwrap().epoch, self.input.epoch - 1);
                assert!(self.born >= 65_536);
                return instance;
            }
        }
        panic!("prepared actor could not transfer its retained Source");
    }

    fn close(mut self, runtime: &OwnedRuntime, compiled: &OwnedCompiledGuest, instance: &mut Option<OwnedGuestInstance>) {
        for _ in 0..100 {
            if self.turn(|cx| runtime.close_original_actor_step(instance, compiled, cx)).unwrap() { break }
        }
        assert!(instance.is_none());
        assert_eq!(self.born, self.released);
        for _ in 0..100 {
            if self.owner.terminal_is_empty() { return }
            let call = self.owner.close_step(self.input, self.budget(65_536));
            self.settle(call.receipt_spent, call.work_units_spent);
            call.result.unwrap();
        }
        panic!("prepared actor caller Source did not close");
    }
}

fn marker(runtime: &OwnedRuntime, source: &mut Source, instance: &OwnedGuestInstance, offset: u32) -> u8 {
    let mut value = None;
    source.turn(|cx| runtime.with_original_memory_bytes(instance, offset, 1, cx, |bytes, _| {
        value = Some(bytes[0]);
        Ok(())
    })).unwrap();
    value.unwrap()
}

#[test]
fn prepared_actor_startup_has_fresh_sources_and_isolated_paid_memory() {
    let fixture: Value = serde_json::from_str(include_str!("🧫️fixtures/prepared.json")).unwrap();
    let runtime = OwnedRuntime::new(clock);
    let mut compiled = runtime.compile_component(&component(&fixture)).unwrap();
    let mut progress = 0;
    assert!(compiled.prepare_actor(128, |spent| {
        assert!(spent >= progress);
        progress = spent;
        true
    }).unwrap());
    assert!(progress > 0);
    let channel = fixture["channelVersion"].as_u64().unwrap() as u32;
    assert_eq!(channel, semio_os_kernel::CHANNEL_VERSION);
    let instructions = fixture["channelInstructions"].as_u64().unwrap();
    let offset = fixture["marker"]["offset"].as_u64().unwrap() as u32;
    let prepared = fixture["marker"]["prepared"].as_u64().unwrap() as u8;
    let mutated = fixture["marker"]["mutated"].as_u64().unwrap() as u8;

    let mut first = Source::admit();
    let mut first_actor = Some(first.open(&runtime, &compiled, channel, instructions));
    assert_eq!(marker(&runtime, &mut first, first_actor.as_ref().unwrap(), offset), prepared);
    first.turn(|cx| {
        let (module, core) = first_actor.as_mut().unwrap().original_core_mut()?;
        assert!(core.write_original_memory_step(module, 0, offset as usize, &[mutated], &mut 0, cx)?);
        Ok(())
    }).unwrap();
    assert!(first.last.copied_bytes > 0);

    let mut second = Source::admit();
    let mut second_actor = Some(second.open(&runtime, &compiled, channel, instructions));
    assert!(second.turn(|cx| runtime.close_original_actor_step(&mut first_actor, &compiled, cx)).is_err());
    assert_eq!(marker(&runtime, &mut first, first_actor.as_ref().unwrap(), offset), mutated);
    assert_eq!(marker(&runtime, &mut second, second_actor.as_ref().unwrap(), offset), prepared);
    first.close(&runtime, &compiled, &mut first_actor);
    assert_eq!(marker(&runtime, &mut second, second_actor.as_ref().unwrap(), offset), prepared);
    second.close(&runtime, &compiled, &mut second_actor);
}
