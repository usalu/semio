//! 🖍️ Actual Draw component opening under the independent compiled host runtime.
use semio_framework::kernel::{Budget, Event, RetainedTurnInput, TurnStatus};
use semio_framework_actor::ActorId as RuntimeActorId;
use semio_framework_job::{CancelToken, Generation, JobPayloadAuthority, OperationId, OriginalCallerSourceParent, StepBudget, StepContext};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
use semio_plugin_host::{GuestInstance, GuestRuntime, PackageHash, PackageId, PackageRef, SharedEngineConfig, WasmtimeRuntime, retryable_lifecycle_turn};
use std::path::PathBuf;
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct OpenScenario {
    app_id: String,
    actor: String,
    expected_status: String,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct OpenScenarios {
    version: u32,
    caller: NativeCallerPolicy,
    scenarios: Vec<OpenScenario>,
}

#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct NativeCallerPolicy {
    clock: String,
    lifetime_us: u64,
    fuel: u64,
    retirement_fuel: u64,
    execution_fuel: u64,
    execution_fuel_per_call: u64,
    maximum_turns: usize,
    grant: RetainedCloneGrant,
}

fn open_scenarios() -> OpenScenarios {
    let fixtures: OpenScenarios = serde_json::from_str(include_str!("scenarios.json")).expect("valid language-neutral opening scenarios");
    assert_eq!(fixtures.version, 2);
    assert_eq!(fixtures.caller.clock, "native-monotonic");
    assert!((1..=120_000_000).contains(&fixtures.caller.lifetime_us));
    assert!((1..=100_000_000).contains(&fixtures.caller.fuel));
    assert!((1..fixtures.caller.fuel).contains(&fixtures.caller.retirement_fuel));
    assert!((1..=4_096_000_000_000).contains(&fixtures.caller.execution_fuel));
    assert!((1..=1_000_000_000).contains(&fixtures.caller.execution_fuel_per_call));
    assert!(fixtures.caller.execution_fuel_per_call <= fixtures.caller.execution_fuel);
    assert!((1..=4096).contains(&fixtures.caller.maximum_turns));
    assert_eq!(fixtures.scenarios.len(), 2);
    let mut roles: Vec<_> = fixtures.scenarios.iter().map(|scenario| scenario.actor.as_str()).collect();
    roles.sort_unstable();
    assert_eq!(roles, ["editor", "viewer"]);
    let descriptor_path = std::env::var_os("SEMIO_DRAW_GUEST_DESCRIPTOR").expect("declared guest check supplies its retained model descriptor");
    let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(descriptor_path).expect("read actual component descriptor")).expect("valid actual component descriptor");
    for scenario in &fixtures.scenarios {
        assert_eq!(scenario.expected_status, "idle");
        assert!(descriptor["manifest"]["apps"].as_array().expect("descriptor apps").iter().any(|app| app["id"] == scenario.app_id && app["role"] == scenario.actor), "opening scenario must match a shipped surface");
    }
    fixtures
}
fn plugin_wasm() -> PathBuf {
    let path = PathBuf::from(std::env::var_os("SEMIO_DRAW_GUEST_COMPONENT").expect("declared guest check supplies its retained model component"));
    assert!(path.is_file(), "Draw component producer prerequisite is required");
    path
}
fn package_ref(package_id: &str, bytes: &[u8]) -> PackageRef {
    PackageRef { package: PackageId(package_id.to_string()), hash: PackageHash(*semio_framework_hash::hash(bytes).as_bytes()) }
}

fn native_now() -> u64 {
    semio_framework_job::default_now_us().expect("native monotonic clock is available")
}

struct NativeCaller {
    policy: NativeCallerPolicy,
    source: std::mem::ManuallyDrop<Option<OriginalCallerSourceParent>>,
    spent: RetainedCloneProgress,
    fuel: u64,
    execution_fuel: u64,
    deadline: u64,
    binding: u64,
    epoch: u64,
    sequence: u64,
    turns: usize,
}

impl NativeCaller {
    fn admit(policy: &NativeCallerPolicy, identity: u64) -> Self {
        let mut caller = Self {
            policy: policy.clone(),
            source: std::mem::ManuallyDrop::new(None),
            spent: Default::default(),
            fuel: policy.fuel,
            execution_fuel: policy.execution_fuel,
            deadline: native_now().checked_add(policy.lifetime_us).expect("finite native deadline"),
            binding: identity,
            epoch: 1,
            sequence: 0,
            turns: 0,
        };
        let turn = CancelToken::admit_root(caller.remaining());
        let (cancel, spent) = turn.result.expect("native cancellation admission").expect("authored cancellation stock");
        caller.pay(spent, turn.work);
        let turn = JobPayloadAuthority::admit(OperationId(identity), Generation(1), caller.remaining());
        let (authority, spent) = turn.result.expect("native payload admission").expect("authored payload stock");
        caller.pay(spent, turn.work);
        let turn = OriginalCallerSourceParent::admit(authority, cancel, caller.remaining());
        let (source, spent) = turn.result.unwrap_or_else(|_| panic!("authored original parent admission"));
        *caller.source = Some(source);
        caller.pay(spent, turn.work);
        caller
    }

    fn remaining(&self) -> RetainedCloneGrant {
        assert!(self.spent.fits(self.policy.grant));
        RetainedCloneGrant {
            maximum_items: self.policy.grant.maximum_items - self.spent.copied_items,
            maximum_copy_bytes: self.policy.grant.maximum_copy_bytes - self.spent.copied_bytes,
            maximum_capacity_bytes: self.policy.grant.maximum_capacity_bytes - self.spent.retained_capacity_bytes,
            maximum_release_bytes: self.policy.grant.maximum_release_bytes - self.spent.released_bytes,
            ..self.policy.grant
        }
    }

    fn pay(&mut self, spent: RetainedCloneProgress, work: u64) {
        self.spent = self.spent.checked_add(spent).expect("native original cumulative spending");
        self.fuel = self.fuel.checked_sub(work).expect("native original fuel is conserved");
        assert!(self.spent.fits(self.policy.grant));
    }

    fn reserve_execution(&mut self) {
        assert!(native_now() < self.deadline, "native guest exceeded its absolute deadline");
        self.turns += 1;
        assert!(self.turns <= self.policy.maximum_turns, "native guest exceeded its authored call bound");
        self.execution_fuel = self.execution_fuel.checked_sub(self.policy.execution_fuel_per_call).expect("finite native execution allowance exhausted");
    }

    fn context(&mut self) -> StepContext<'_> {
        let mut cx = self
            .source
            .as_ref()
            .expect("original native source remains retained")
            .borrow_context(StepBudget::new(self.fuel, self.deadline, self.policy.grant), semio_framework_job::default_now_us, &mut self.sequence, &mut self.spent)
            .expect("original native context");
        cx.bind_original_clock_binding(self.binding).expect("bind actual native source clock");
        cx.bind_original_source_epoch(self.epoch).expect("bind actual issued epoch");
        cx
    }

    fn returned(&mut self, fuel: u64) {
        assert!(fuel <= self.fuel, "native original work cannot be renewed");
        self.fuel = fuel;
        self.epoch = self.epoch.checked_add(1).expect("native epoch remains finite");
        assert!(native_now() < self.deadline, "native guest completed outside its absolute deadline");
    }

    fn retire(&mut self) {
        for _ in 0..self.policy.maximum_turns {
            assert!(native_now() < self.deadline, "original native source retirement retains its finite deadline");
            if self.source.as_ref().is_none_or(OriginalCallerSourceParent::terminal_is_empty) {
                self.source.take();
                return;
            }
            assert!(self.fuel > 0, "original native source retirement is funded");
            let grant = self.remaining();
            let turn = self.source.as_mut().unwrap().close_step(grant);
            let step = turn.result.expect("original native source paid retirement");
            self.pay(step.progress(), turn.work);
        }
        panic!("original native source retirement exceeded its authored bound")
    }
}

impl Drop for NativeCaller {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.source.is_none(), "original native source requires paid retirement")
    }
}

fn call_budget(cx: &StepContext<'_>, execution_fuel: u64, reserve: u64) -> Budget {
    let input = RetainedTurnInput::borrow_original(cx).expect("guest input comes from the actual original native context");
    let work_fuel = cx.fuel_remaining().checked_sub(reserve).expect("native semantic work preserves terminal retirement fuel");
    assert!(work_fuel > 0, "native semantic work remains funded");
    Budget { retained: input, retained_policy: cx.retained_grant(), work_fuel, fuel: execution_fuel, deadline_us: cx.deadline_us(), max_effects: 64, max_patch_bytes: 1 << 20, max_frames: 64 }
}

fn receive(cx: &mut StepContext<'_>, budget: Budget, returned: RetainedTurnInput, spent: RetainedCloneProgress, work: u64) {
    cx.consume_fuel(work);
    cx.consume_retained(spent).expect("actual guest physical spending is received before validation");
    assert_eq!(returned, budget.retained, "guest must return its exact issued original input");
    assert!(work <= budget.work_fuel && spent.fits(budget.retained_policy), "guest receipt must fit its actual held semantic loan");
    budget.retained.return_original(spent, work).expect("actual guest work and retained receipt conserve the issued source");
}

fn require_call<T>(cx: &mut StepContext<'_>, budget: Budget, result: Result<T, semio_plugin_host::TurnFault>) -> T {
    match result {
        Ok(value) => value,
        Err(semio_plugin_host::TurnFault::ActorCall(call)) => {
            receive(cx, budget, call.input, call.spent, call.work_units_spent);
            panic!("native guest refused its original call: {call:?}")
        }
        Err(fault) => panic!("native guest call faulted without a returned original receipt: {fault:?}"),
    }
}

fn retire_owned<T: semio_framework_value::retirement::RetireOwned>(cx: &mut StepContext<'_>, value: T, maximum_turns: usize) {
    let mut owner = semio_framework_value::retirement::controlled::ControlledRetirement::new(value).unwrap_or_else(|_| panic!("actual guest test source supports paid retirement"));
    for _ in 0..maximum_turns {
        if owner.terminal_is_empty() {
            return;
        }
        cx.with_original_retirement(|cx| {
            let demand = owner.selected_demand(cx.retained_grant().maximum_copy_bytes)?;
            semio_framework_job::close_original_retained_step(cx, demand, |grant| {
                let result = owner.step(grant);
                semio_framework_value::RetainedWorkOutcome { result, work: owner.last_turn_work() }
            })?;
            Ok::<_, semio_framework_job::OriginalTurnFailure>(())
        })
        .expect("actual guest test source paid retirement");
    }
    panic!("actual guest test source retirement exceeded its authored bound")
}

fn instance_open_event(app_id: &str, actor: &str, config: Vec<u8>) -> Event {
    Event::InstanceOpen {
        request: semio_framework::kernel::ActorInstanceOpenRequest { activation_generation: 1, instance_id: 1, request_sequence: 1 },
        app_id: semio_framework::kernel::AppInstanceId(app_id.to_string()),
        actor: actor.to_string(),
        config,
        assets: Vec::new(),
        capabilities: Vec::new(),
        quotas: semio_framework::kernel::QuotaSchema::default(),
    }
}

async fn open_to_settle(runtime: &impl GuestRuntime, instance: &mut GuestInstance, scenario: &OpenScenario, caller: &mut NativeCaller) {
    let mut owed = vec![instance_open_event(&scenario.app_id, &scenario.actor, Vec::new())];
    let started = std::time::Instant::now();
    let mut opened = None;
    let mut closing = false;
    let mut retired = false;
    let mut close_generation = None;
    let mut identity = semio_plugin::artifact_app_laws::fixture_identity();
    loop {
        caller.reserve_execution();
        let (execution, reserve) = (caller.policy.execution_fuel_per_call, caller.policy.retirement_fuel);
        let mut cx = caller.context();
        instance.bind_original_clock(&cx).expect("guest clock binds to the genuine native source");
        let budget = call_budget(&cx, execution, reserve);
        let result = runtime.admit_actor_context(instance, budget.retained, budget, &mut identity).await;
        let reply = require_call(&mut cx, budget, result);
        receive(&mut cx, budget, reply.input, reply.spent, reply.work_units_spent);
        assert!(reply.refusal.is_none(), "native guest admission refused: {:?}", reply.refusal);
        let fuel = cx.fuel_remaining();
        drop(cx);
        caller.returned(fuel);
        if reply.ready {
            break;
        }
    }
    loop {
        caller.reserve_execution();
        let (execution, reserve, maximum_turns) = (caller.policy.execution_fuel_per_call, caller.policy.retirement_fuel, caller.policy.maximum_turns);
        let mut cx = caller.context();
        let budget = call_budget(&cx, execution, reserve);
        let mut events = std::mem::take(&mut owed);
        let mut settled = false;
        match runtime.execute_turn(instance, &mut events, budget, &mut identity).await {
            Ok(turn) => {
                let receipt = turn.retained_receipt;
                receive(&mut cx, budget, receipt.input, receipt.spent, receipt.work_units_spent);
                receipt.validate_for(budget.retained).expect("native poll exact conserved remaining currencies");
                assert!(!matches!(turn.status, TurnStatus::Faulted(_)), "{} opening faulted: {:?}", scenario.app_id, turn.status);
                if let Some(receipt) = turn.lifecycle_receipt {
                    assert!(receipt.is_valid(), "actual lifecycle receipt must be valid");
                    let lifetime = receipt.lifetime();
                    assert_eq!((lifetime.activation_generation, lifetime.instance_id), (1, 1));
                    if closing {
                        assert_eq!(Some(lifetime), opened, "close must retire the captured original instance");
                        assert_eq!(receipt.request_sequence(), 2);
                        let generation = receipt.close_generation().expect("close receipt owns its generation");
                        if let Some(original) = close_generation {
                            assert_eq!(generation, original)
                        } else {
                            close_generation = Some(generation)
                        }
                        retired |= matches!(receipt, semio_framework::kernel::ActorInstanceLifecycleReceipt::Retired { .. });
                    } else {
                        assert!(matches!(receipt, semio_framework::kernel::ActorInstanceLifecycleReceipt::Captured { .. }));
                        assert_eq!(receipt.request_sequence(), 1);
                        if let Some(original) = opened { assert_eq!(lifetime, original) } else { opened = Some(lifetime) }
                    }
                    owed.push(Event::InstanceLifecycleAck(semio_framework::kernel::ActorInstanceLifecycleAck { receipt }));
                }
                if matches!(turn.status, TurnStatus::Idle) && owed.is_empty() {
                    assert!(opened.is_some(), "{} became idle without acknowledging InstanceOpen", scenario.app_id);
                    settled = true;
                }
                retire_owned(&mut cx, turn, maximum_turns);
                retire_owned(&mut cx, events, maximum_turns);
            }
            Err(fault) if retryable_lifecycle_turn(&fault, &events) => {
                panic!("retryable lifecycle fault did not carry its actual original receipt: {fault:?}")
            }
            Err(semio_plugin_host::TurnFault::ActorCall(call)) => {
                receive(&mut cx, budget, call.input, call.spent, call.work_units_spent);
                panic!("InstanceOpen refused its original call after {:?}: {call:?}", started.elapsed())
            }
            Err(fault) => panic!("InstanceOpen faulted after {:?}: {fault:?}", started.elapsed()),
        }
        let fuel = cx.fuel_remaining();
        drop(cx);
        caller.returned(fuel);
        if settled {
            if closing {
                assert!(retired, "actual instance must acknowledge Retired before actor context closure");
                break;
            }
            closing = true;
            owed.push(Event::InstanceClose(semio_framework::kernel::ActorInstanceCloseRequest { lifetime: opened.expect("settled original instance lifetime"), request_sequence: 2 }));
        }
    }
    loop {
        caller.reserve_execution();
        let (execution, reserve) = (caller.policy.execution_fuel_per_call, caller.policy.retirement_fuel);
        let mut cx = caller.context();
        let budget = call_budget(&cx, execution, reserve);
        let result = runtime.close_actor_context(instance, budget.retained, budget, &mut identity).await;
        let reply = require_call(&mut cx, budget, result);
        receive(&mut cx, budget, reply.input, reply.spent, reply.work_units_spent);
        assert!(reply.refusal.is_none() && !reply.blocked, "native guest closure refused or blocked: {reply:?}");
        let fuel = cx.fuel_remaining();
        drop(cx);
        caller.returned(fuel);
        if reply.complete {
            break;
        }
    }
}

mod quick {
    use super::*;
    #[semio_framework_async_macros::async_test]
    async fn wasmtime_runtime_instance_open_settles_against_the_draw_component() {
        let path = plugin_wasm();
        let bytes = std::fs::read(&path).expect("read plugin component");
        let runtime = WasmtimeRuntime::new(SharedEngineConfig::default(), semio_framework_job::default_now_us).await.expect("engine builds");
        let compiled = runtime.compile(&package_ref("semio:draw", &bytes), &bytes).await.expect("compile plugin component");
        let fixtures = open_scenarios();
        for (index, scenario) in fixtures.scenarios.iter().enumerate() {
            let mut caller = NativeCaller::admit(&fixtures.caller, index as u64 + 1);
            caller.reserve_execution();
            let (execution, reserve) = (caller.policy.execution_fuel_per_call, caller.policy.retirement_fuel);
            let cx = caller.context();
            let budget = call_budget(&cx, execution, reserve);
            let mut instance = runtime.instantiate(&compiled, RuntimeActorId(index as u64 + 1), &[], &budget).await.expect("instantiate plugin actor");
            drop(cx);
            open_to_settle(&runtime, &mut instance, scenario, &mut caller).await;
            runtime.drop_instance(instance).await;
            caller.retire();
        }
    }
}
