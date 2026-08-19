//! 🧪️ terra-async-harness-spike (Q1-Q6). Host half of a REDUCED, host-controlled copy of real
//! `world actor`'s `reactor`/`jobs`/`checkpoint` exports + `pure`/`hostasync` imports — see
//! `../👽️guest-turn/🧬️schema/📜️world.wit`'s own doc. Answers the six mission questions against real
//! wasmtime 47.0.3, reusing proven idioms from `terra-async-runtime-harness-host-main.rs.txt`
//! (tests A-F: named-interface accessor calls, grant-gated stream park/refill, epoch Yield-then-
//! Interrupt, root-task-drop cancellation, tokio abort, accessor.spawn+oneshot for a second export
//! call while the root call is in flight) and `terra-probe-spikes-report.md`'s S1c (pure-CPU
//! preemption, `black_box`'d to survive LLVM's strength-reduction) — never reimplementing either
//! from scratch.

use anyhow::Result;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::future::Future;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use wasmtime::component::{Accessor, AccessorTask, Component, HasSelf, Linker, ResourceTable};
use wasmtime::{Config, Engine, Store, UpdateDeadline};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

wasmtime::component::bindgen!({
    path: "../👽️guest-turn/🧬️schema/📜️world.wit",
    world: "turnharness",
});

use exports::semio::turnharness::jobs::{JobBudget, JobStep};
use exports::semio::turnharness::reactor::{Budget, Event, TurnResult};

struct HostState {
    wasi: WasiCtx,
    table: ResourceTable,
    logs: Arc<Mutex<Vec<String>>>,
    // #region Q2/Q4/Q6 — hang() start/drop flags
    hang_started: Arc<AtomicBool>,
    hang_dropped: Arc<AtomicBool>,
    // #endregion
    // #region Q1 — pre-registered oneshot receivers, keyed by the `id` the guest passes to wait-signal
    signal_rx: Arc<Mutex<HashMap<u32, tokio::sync::oneshot::Receiver<u32>>>>,
    // #endregion
}

impl WasiView for HostState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView { ctx: &mut self.wasi, table: &mut self.table }
    }
}

fn new_state() -> HostState {
    HostState {
        wasi: WasiCtxBuilder::new().inherit_stdio().build(),
        table: ResourceTable::new(),
        logs: Arc::new(Mutex::new(Vec::new())),
        hang_started: Arc::new(AtomicBool::new(false)),
        hang_dropped: Arc::new(AtomicBool::new(false)),
        signal_rx: Arc::new(Mutex::new(HashMap::new())),
    }
}

//#region 🔌️pure::Host / pure::HostWithStore
impl semio::turnharness::pure::Host for HostState {}

impl semio::turnharness::pure::HostWithStore<HostState> for HasSelf<HostState> {
    async fn log(accessor: &Accessor<HostState, Self>, level: String, message: String) {
        let logs = accessor.with(|mut access| access.get().logs.clone());
        let line = format!("[guest:{level}] {message}");
        println!("{line}");
        logs.lock().expect("logs poisoned").push(line);
    }

    async fn now_ms(_accessor: &Accessor<HostState, Self>) -> i64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as i64).unwrap_or(0)
    }
}
//#endregion

//#region 🔌️hostasync::Host / hostasync::HostWithStore — the two host-controlled imports Q1/Q2 need
impl semio::turnharness::hostasync::Host for HostState {}

impl semio::turnharness::hostasync::HostWithStore<HostState> for HasSelf<HostState> {
    // 🧪️ Q1: resolves once the HOST-side driver sends on the pre-registered oneshot channel for
    // this `id` — genuinely concurrent (no wall-clock sleep, no busy-poll): whichever side reaches
    // its half of the channel first, the other's `.await`/`.send()` still completes correctly.
    async fn wait_signal(accessor: &Accessor<HostState, Self>, id: u32) -> u32 {
        let rx = accessor.with(|mut access| access.get().signal_rx.lock().expect("signal_rx poisoned").remove(&id));
        match rx {
            Some(rx) => rx.await.unwrap_or(0),
            None => {
                println!("[host] wait-signal({id}) called with no receiver pre-registered — resolving with 0");
                0
            }
        }
    }

    // 🧪️ Q2: never resolves on its own — S2's own `DropSignal`/`CancelOnDrop` idiom
    // (`terra-probe-spikes-report.md` S2, `⏳️imports.rs`'s `CancelOnDrop`), reused verbatim: `started`
    // flips the instant this future is entered, `dropped` flips only when THIS future itself is
    // dropped without ever resolving.
    async fn hang(accessor: &Accessor<HostState, Self>, id: u32) -> u32 {
        struct DropSignal(Arc<AtomicBool>);
        impl Drop for DropSignal {
            fn drop(&mut self) {
                self.0.store(true, Ordering::SeqCst);
            }
        }
        let (started, dropped) = accessor.with(|mut access| (access.get().hang_started.clone(), access.get().hang_dropped.clone()));
        started.store(true, Ordering::SeqCst);
        let _guard = DropSignal(dropped);
        println!("[host] hang({id}) started, awaiting pending forever until dropped");
        std::future::pending::<()>().await;
        unreachable!("hang() must never resolve — it exists to be cancelled");
    }
}
//#endregion

//#region ⏱️DeadlineCell / install_epoch_budget — VERBATIM shape from ⏳️runtime.rs's own doc (see
// module doc there), re-run here against the real reactor::poll shape rather than assumed correct.
struct DeadlineCell(Mutex<Instant>);

impl DeadlineCell {
    fn new(initial: Duration) -> Arc<Self> {
        Arc::new(Self(Mutex::new(Instant::now() + initial)))
    }

    fn passed(&self) -> bool {
        Instant::now() >= *self.0.lock().expect("DeadlineCell poisoned")
    }
}

fn install_epoch_budget(store: &mut Store<HostState>, deadline: Arc<DeadlineCell>) {
    store.epoch_deadline_callback(move |_ctx| if deadline.passed() { Ok(UpdateDeadline::Interrupt) } else { Ok(UpdateDeadline::Yield(1)) });
    store.set_epoch_deadline(1);
}
//#endregion

fn build_engine() -> Result<Engine> {
    let mut config = Config::new();
    config.wasm_component_model_async(true);
    config.concurrency_support(true);
    config.consume_fuel(true);
    config.epoch_interruption(true);
    Ok(Engine::new(&config)?)
}

/// 🧪️ Q3 asymmetric/fuel isolation (S1c precedent): a SEPARATE Engine with `epoch_interruption`
/// never enabled at all, so fuel is the ONLY interruption lever in play — no epoch confound.
fn build_engine_fuel_only() -> Result<Engine> {
    let mut config = Config::new();
    config.wasm_component_model_async(true);
    config.concurrency_support(true);
    config.consume_fuel(true);
    Ok(Engine::new(&config)?)
}

fn start_ticker(engine: &Engine) -> (Arc<AtomicBool>, std::thread::JoinHandle<()>) {
    let stop = Arc::new(AtomicBool::new(false));
    let stop_thread = stop.clone();
    let ticker_engine = engine.clone();
    let handle = std::thread::spawn(move || {
        while !stop_thread.load(Ordering::Relaxed) {
            ticker_engine.increment_epoch();
            std::thread::sleep(Duration::from_millis(1));
        }
    });
    (stop, handle)
}

//#region 🧪️Q1 — turn shape works
async fn test_q1_turn_shape(engine: &Engine, component: &Component, linker: &Linker<HostState>) -> Result<String> {
    let state = new_state();
    let (tx, rx) = tokio::sync::oneshot::channel::<u32>();
    state.signal_rx.lock().expect("signal_rx poisoned").insert(7, rx);
    let logs = state.logs.clone();
    let mut store = Store::new(engine, state);
    store.set_fuel(u64::MAX)?;
    install_epoch_budget(&mut store, DeadlineCell::new(Duration::from_secs(10)));
    let instance = Turnharness::instantiate_async(&mut store, component, linker).await?;

    let events = vec![Event::AwaitSignal(7)];
    let budget = Budget { fuel: 1_000_000, deadline_ms: 10_000 };
    let poll_task = tokio::spawn(async move {
        store
            .run_concurrent(async move |accessor: &Accessor<HostState>| -> Result<TurnResult> {
                instance.semio_turnharness_reactor().call_poll(accessor, events, budget).await?.map_err(|fault| anyhow::anyhow!(fault))
            })
            .await
    });

    // 🎯️ Q1's actual mechanism under test: the host resolves the import from OUTSIDE the guest's
    // own call, while `poll_task` is genuinely still in flight — `oneshot::send` needs no
    // wall-clock coordination with the receiver side.
    tx.send(70).map_err(|_| anyhow::anyhow!("wait-signal(7) receiver already dropped"))?;
    let turn = poll_task.await???;
    let logged = logs.lock().expect("logs poisoned").iter().any(|line| line.contains("wait-signal(7) resolved = 70"));
    Ok(format!("turn-result = {turn:?}, guest observed resolved value via pure::log = {logged}"))
}
//#endregion

//#region 🧪️Q2 — cancellation requires dropping the Store (poll shape)
async fn test_q2_cancellation(engine: &Engine, component: &Component, linker: &Linker<HostState>) -> Result<String> {
    let state = new_state();
    let hang_started = state.hang_started.clone();
    let hang_dropped = state.hang_dropped.clone();
    let mut store = Store::new(engine, state);
    store.set_fuel(u64::MAX)?;
    install_epoch_budget(&mut store, DeadlineCell::new(Duration::from_secs(30)));
    let instance = Turnharness::instantiate_async(&mut store, component, linker).await?;

    let events = vec![Event::AwaitHang(99)];
    let budget = Budget { fuel: u64::MAX, deadline_ms: 30_000 };
    let fut = store.run_concurrent(async move |accessor: &Accessor<HostState>| -> Result<TurnResult> {
        instance.semio_turnharness_reactor().call_poll(accessor, events, budget).await?.map_err(|fault| anyhow::anyhow!(fault))
    });
    let mut fut = Box::pin(fut);
    let waker = std::task::Waker::noop();
    let mut cx = Context::from_waker(waker);
    match fut.as_mut().poll(&mut cx) {
        Poll::Pending => {}
        Poll::Ready(_) => anyhow::bail!("await-hang poll() must not resolve on the very first manual poll"),
    }
    anyhow::ensure!(hang_started.load(Ordering::SeqCst), "hang() must have been entered before poll() returns Pending");

    // (a) drop ONLY the run_concurrent future — the Store still owns everything.
    drop(fut);
    let dropped_future_only = hang_dropped.load(Ordering::SeqCst);
    // (b) now also drop the Store.
    drop(store);
    let dropped_after_store = hang_dropped.load(Ordering::SeqCst);

    Ok(format!("dropped_future_only={dropped_future_only} dropped_after_store={dropped_after_store}"))
}
//#endregion

//#region 🧪️Q3 — preemption across actors (epoch AND fuel-only levers)
async fn run_burn_pair(engine: &Engine, component: &Component, linker: &Linker<HostState>, iters_a: u64, iters_b: u64, use_epoch: bool) -> Result<(Duration, Duration, u64, u64)> {
    let mut store_a = Store::new(engine, new_state());
    let mut store_b = Store::new(engine, new_state());
    store_a.set_fuel(u64::MAX)?;
    store_b.set_fuel(u64::MAX)?;
    let hits_a = Arc::new(AtomicU64::new(0));
    let hits_b = Arc::new(AtomicU64::new(0));
    if use_epoch {
        let ha = hits_a.clone();
        store_a.epoch_deadline_callback(move |_ctx| {
            ha.fetch_add(1, Ordering::Relaxed);
            Ok(UpdateDeadline::Yield(1))
        });
        store_a.set_epoch_deadline(1);
        let hb = hits_b.clone();
        store_b.epoch_deadline_callback(move |_ctx| {
            hb.fetch_add(1, Ordering::Relaxed);
            Ok(UpdateDeadline::Yield(1))
        });
        store_b.set_epoch_deadline(1);
    } else {
        store_a.fuel_async_yield_interval(Some(500_000))?;
        store_b.fuel_async_yield_interval(Some(500_000))?;
    }
    let instance_a = Turnharness::instantiate_async(&mut store_a, component, linker).await?;
    let instance_b = Turnharness::instantiate_async(&mut store_b, component, linker).await?;

    let start = Instant::now();
    let done_a: Arc<Mutex<Option<Duration>>> = Arc::new(Mutex::new(None));
    let done_b: Arc<Mutex<Option<Duration>>> = Arc::new(Mutex::new(None));
    let da = done_a.clone();
    let db = done_b.clone();

    let fut_a = async move {
        let events = vec![Event::Burn(iters_a)];
        let budget = Budget { fuel: u64::MAX, deadline_ms: 600_000 };
        let r = store_a
            .run_concurrent(async move |accessor: &Accessor<HostState>| -> Result<TurnResult> {
                instance_a.semio_turnharness_reactor().call_poll(accessor, events, budget).await?.map_err(|fault| anyhow::anyhow!(fault))
            })
            .await??;
        *da.lock().expect("done_a poisoned") = Some(start.elapsed());
        Ok::<TurnResult, anyhow::Error>(r)
    };
    let fut_b = async move {
        let events = vec![Event::Burn(iters_b)];
        let budget = Budget { fuel: u64::MAX, deadline_ms: 600_000 };
        let r = store_b
            .run_concurrent(async move |accessor: &Accessor<HostState>| -> Result<TurnResult> {
                instance_b.semio_turnharness_reactor().call_poll(accessor, events, budget).await?.map_err(|fault| anyhow::anyhow!(fault))
            })
            .await??;
        *db.lock().expect("done_b poisoned") = Some(start.elapsed());
        Ok::<TurnResult, anyhow::Error>(r)
    };
    // 🎯️ HOST-LEVEL join, NOT Accessor::spawn — S1b/S1c's own proven multiplexing shape.
    let (ra, rb) = tokio::join!(fut_a, fut_b);
    ra?;
    rb?;
    let ta = done_a.lock().expect("done_a poisoned").expect("done_a not set");
    let tb = done_b.lock().expect("done_b poisoned").expect("done_b not set");
    Ok((ta, tb, hits_a.load(Ordering::Relaxed), hits_b.load(Ordering::Relaxed)))
}
//#endregion

//#region 🧪️Q4 — jobs while a turn is live
async fn test_q4_jobs_while_turn_live(engine: &Engine, component: &Component, linker: &Linker<HostState>) -> Result<String> {
    // --- SAME instance: poll() suspended on AwaitHang; step-job dispatched concurrently via
    // accessor.spawn (S1's own AccessorTask+oneshot idiom, production runtime.rs's own mechanism
    // for Checkpoint/StepJob dispatch against the SAME live instance). ---
    let mut state = new_state();
    let hang_started = state.hang_started.clone();
    state.hang_started = hang_started.clone();
    let mut store = Store::new(engine, state);
    store.set_fuel(u64::MAX)?;
    install_epoch_budget(&mut store, DeadlineCell::new(Duration::from_secs(30)));
    let instance = Turnharness::instantiate_async(&mut store, component, linker).await?;
    let instance = Arc::new(instance);
    let instance_for_step = instance.clone();

    struct StepJobTask {
        instance: Arc<Turnharness>,
        job: u64,
        reply: tokio::sync::oneshot::Sender<wasmtime::Result<Result<JobStep, String>>>,
    }
    impl AccessorTask<HostState> for StepJobTask {
        async fn run(self, accessor: &Accessor<HostState>) -> wasmtime::Result<()> {
            let budget = JobBudget { fuel: 10_000, deadline_ms: 1_000 };
            let r = self.instance.semio_turnharness_jobs().call_step_job(accessor, self.job, budget).await;
            let _ = self.reply.send(r);
            Ok(())
        }
    }
    let (tx, rx) = tokio::sync::oneshot::channel();

    let events = vec![Event::AwaitHang(42)];
    let budget = Budget { fuel: u64::MAX, deadline_ms: 30_000 };
    let poll_task = tokio::spawn(async move {
        store
            .run_concurrent(async move |accessor: &Accessor<HostState>| -> Result<TurnResult> {
                accessor.spawn(StepJobTask { instance: instance_for_step, job: 5, reply: tx })?;
                instance.semio_turnharness_reactor().call_poll(accessor, events, budget).await?.map_err(|fault| anyhow::anyhow!(fault))
            })
            .await
    });

    for _ in 0..500 {
        if hang_started.load(Ordering::SeqCst) {
            break;
        }
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
    anyhow::ensure!(hang_started.load(Ordering::SeqCst), "hang() must have started before step-job's reply is trusted");

    let same_instance_summary = match rx.await {
        Ok(Ok(Ok(step))) => format!("GO — step-job on the SAME instance succeeded while poll() was suspended awaiting hostasync::hang: {step:?}"),
        Ok(Ok(Err(fault))) => format!("guest-level error: {fault}"),
        Ok(Err(trap)) => format!("wasmtime trap: {trap:#}"),
        Err(_) => "oneshot sender dropped without replying".to_string(),
    };
    // poll() never resolves on its own (hang never resolves) — abort to clean up (ties to Q2/Q6).
    poll_task.abort();
    let _ = poll_task.await;

    // --- DIFFERENT actor: a separate, idle instance/Store — trivial control. ---
    let mut store2 = Store::new(engine, new_state());
    store2.set_fuel(u64::MAX)?;
    install_epoch_budget(&mut store2, DeadlineCell::new(Duration::from_secs(5)));
    let instance2 = Turnharness::instantiate_async(&mut store2, component, linker).await?;
    let different_actor_result = store2
        .run_concurrent(async move |accessor: &Accessor<HostState>| -> Result<Result<JobStep, String>> {
            let budget = JobBudget { fuel: 10_000, deadline_ms: 1_000 };
            Ok(instance2.semio_turnharness_jobs().call_step_job(accessor, 6, budget).await?)
        })
        .await??;
    let different_actor_summary = format!("different-actor (idle instance) step-job: {different_actor_result:?}");

    Ok(format!("{same_instance_summary}; {different_actor_summary}"))
}
//#endregion

//#region 🧪️Q5 — budgets: delta semantics, epoch cutoff, fuel cutoff
async fn test_q5_budgets(engine: &Engine, component: &Component, linker: &Linker<HostState>) -> Result<String> {
    let mut lines = Vec::new();

    // (a) epoch cutoff — a real 20ms wall-clock DeadlineCell against a huge burn.
    {
        let mut store = Store::new(engine, new_state());
        store.set_fuel(u64::MAX)?;
        let hits = Arc::new(AtomicU64::new(0));
        let hits_cb = hits.clone();
        let deadline = DeadlineCell::new(Duration::from_millis(20));
        let deadline_cb = deadline.clone();
        store.epoch_deadline_callback(move |_ctx| {
            if deadline_cb.passed() {
                Ok(UpdateDeadline::Interrupt)
            } else {
                hits_cb.fetch_add(1, Ordering::Relaxed);
                Ok(UpdateDeadline::Yield(1))
            }
        });
        store.set_epoch_deadline(1);
        let instance = Turnharness::instantiate_async(&mut store, component, linker).await?;
        let events = vec![Event::Burn(2_000_000_000u64)];
        let budget = Budget { fuel: u64::MAX, deadline_ms: 20 };
        let result = store
            .run_concurrent(async move |accessor: &Accessor<HostState>| -> Result<TurnResult> {
                instance.semio_turnharness_reactor().call_poll(accessor, events, budget).await?.map_err(|fault| anyhow::anyhow!(fault))
            })
            .await;
        let message = match result {
            Ok(Ok(v)) => format!("UNEXPECTED — poll() returned Ok({v:?}) instead of trapping"),
            Ok(Err(e)) => format!("{e:#}"),
            Err(e) => format!("{e:#}"),
        };
        lines.push(format!("Q5a epoch-deadline cutoff: yields-before-cutoff={} error={message}", hits.load(Ordering::Relaxed)));
    }

    // (b) fuel exhaustion — a small hard fuel cap (no fuel_async_yield_interval), generous epoch so
    // epoch cannot be the thing that actually cuts it off. 🐛️ own-harness bug found and fixed here
    // (kept as evidence, see report): `store.set_fuel(5_000)` BEFORE `instantiate_async` made
    // instantiation itself (WASI setup + component instantiation, which also consumes fuel once
    // `consume_fuel` is enabled) exhaust the cap and trap — not the `poll()` call under test at all.
    // Fixed the honest way: instantiate with a generous cap, THEN arm the small per-grant cap via
    // `access.as_context_mut().set_fuel(...)` from INSIDE the accessor closure, immediately before
    // the call under test — this is the EXACT mechanism `⏳️runtime.rs`'s own control loop already
    // uses to apply a fresh grant's fuel (`GrantHandle::refill` → `access.as_context_mut().set_fuel`),
    // so this fix doubles as independent confirmation that shape is necessary, not optional.
    match (|| async {
        let mut store = Store::new(engine, new_state());
        store.set_fuel(u64::MAX)?;
        install_epoch_budget(&mut store, DeadlineCell::new(Duration::from_secs(30)));
        let instance = Turnharness::instantiate_async(&mut store, component, linker).await?;
        let events = vec![Event::Burn(2_000_000_000u64)];
        let budget = Budget { fuel: 5_000, deadline_ms: 30_000 };
        let result: std::result::Result<Result<TurnResult>, anyhow::Error> = store
            .run_concurrent(async move |accessor: &Accessor<HostState>| -> Result<TurnResult> {
                accessor.with(|mut access| {
                    use wasmtime::AsContextMut;
                    let _ = access.as_context_mut().set_fuel(5_000);
                });
                instance.semio_turnharness_reactor().call_poll(accessor, events, budget).await?.map_err(|fault| anyhow::anyhow!(fault))
            })
            .await
            .map_err(anyhow::Error::from);
        anyhow::Ok(result)
    })()
    .await
    {
        Ok(Ok(Ok(v))) => lines.push(format!("Q5b fuel exhaustion (fuel=5000 armed inside the accessor, no yield interval): UNEXPECTED — poll() returned Ok({v:?}) instead of trapping")),
        Ok(Ok(Err(e))) => lines.push(format!("Q5b fuel exhaustion (fuel=5000 armed inside the accessor, no yield interval): error={e:#}")),
        Ok(Err(e)) => lines.push(format!("Q5b fuel exhaustion (fuel=5000 armed inside the accessor, no yield interval): error={e:#}")),
        Err(e) => lines.push(format!("Q5b setup FAILED before the fuel-exhaustion call under test even ran: {e:#}")),
    }

    // (c) delta-not-absolute — reproduce the S9 self-inflicted bug deliberately, DECISIVELY this
    // time: NO `epoch_deadline_callback` registered at all, so wasmtime's DEFAULT behaviour applies
    // — trap immediately the instant the current epoch reaches/passes the armed deadline (S9's own
    // finding: "with no epoch_deadline_callback registered, the default behavior on an
    // already-past deadline is an immediate trap"). On a store whose ENGINE has had its epoch
    // ticker running for real wall-clock time already (current_epoch >> 0 — this test runs LAST,
    // after several real seconds of Q1-Q5b), `set_epoch_deadline(u64::MAX)` must overflow
    // `current_epoch + delta` and wrap to a deadline already in the past, trapping on the very
    // first epoch check — proof the argument is a DELTA, not an absolute epoch (an absolute-epoch
    // semantics would make u64::MAX read as "essentially never", not "already elapsed").
    {
        let mut store = Store::new(engine, new_state());
        store.set_fuel(u64::MAX)?;
        store.set_epoch_deadline(u64::MAX);
        let instantiate_result = Turnharness::instantiate_async(&mut store, component, linker).await;
        match instantiate_result {
            Err(e) => lines.push(format!("Q5c delta-not-absolute: set_epoch_deadline(u64::MAX) trapped during instantiate_async itself (no callback registered, default = trap on deadline reached): {e:#}")),
            Ok(instance) => {
                let events = vec![Event::Tick];
                let budget = Budget { fuel: u64::MAX, deadline_ms: 1 };
                let result = store
                    .run_concurrent(async move |accessor: &Accessor<HostState>| -> Result<TurnResult> {
                        instance.semio_turnharness_reactor().call_poll(accessor, events, budget).await?.map_err(|fault| anyhow::anyhow!(fault))
                    })
                    .await;
                let message = match result {
                    Ok(Ok(v)) => format!("UNEXPECTED — poll() returned Ok({v:?}) instead of trapping (would suggest u64::MAX did NOT wrap to an already-past deadline)"),
                    Ok(Err(e)) => format!("{e:#}"),
                    Err(e) => format!("{e:#}"),
                };
                lines.push(format!("Q5c delta-not-absolute: set_epoch_deadline(u64::MAX) with no callback registered, called after real wall-clock ticking, trapped on the first poll(): {message}"));
            }
        }
    }

    Ok(lines.join(" | "))
}
//#endregion

//#region 🧪️Q6 — tokio Handle-driven spawn + abort tears down the Store
async fn test_q6_tokio_handle_spawn(handle: &tokio::runtime::Handle, engine: &Engine, component: &Component, linker: &Linker<HostState>) -> Result<String> {
    let state = new_state();
    let hang_started = state.hang_started.clone();
    let hang_dropped = state.hang_dropped.clone();
    let mut store = Store::new(engine, state);
    store.set_fuel(u64::MAX)?;
    install_epoch_budget(&mut store, DeadlineCell::new(Duration::from_secs(30)));
    let instance = Turnharness::instantiate_async(&mut store, component, linker).await?;

    // 🎯️ Q6: spawned via a `Handle` the caller received (not a `Runtime` it constructed itself) —
    // mirrors the "tokio owned by semio-framework-os-services" policy. The Store is moved INTO the
    // spawned async block and constructed/owned there (mirrors AsyncActorTask::spawn's own design),
    // so `JoinHandle::abort()` tears down the future AND the Store together in one shot (ties to Q2).
    let join = handle.spawn(async move {
        let events = vec![Event::AwaitHang(11)];
        let budget = Budget { fuel: u64::MAX, deadline_ms: 30_000 };
        store
            .run_concurrent(async move |accessor: &Accessor<HostState>| -> Result<TurnResult> {
                instance.semio_turnharness_reactor().call_poll(accessor, events, budget).await?.map_err(|fault| anyhow::anyhow!(fault))
            })
            .await
    });

    for _ in 0..500 {
        if hang_started.load(Ordering::SeqCst) {
            break;
        }
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
    let started = hang_started.load(Ordering::SeqCst);
    let dropped_before_abort = hang_dropped.load(Ordering::SeqCst);
    join.abort();
    let _ = join.await;
    let dropped_after_abort = hang_dropped.load(Ordering::SeqCst);

    Ok(format!("started={started} dropped_before_abort={dropped_before_abort} dropped_after_abort={dropped_after_abort}"))
}
//#endregion

fn main() -> Result<()> {
    // 🎯️ Q6 policy shape: build the Runtime ONCE at the top (standing in for
    // `semio-framework-os-services` owning it), then only ever hand a cloned `Handle` downward —
    // every test function below receives `&Engine`/`&Component`/`&Linker` plus, for Q6 specifically,
    // a `&tokio::runtime::Handle`, never the `Runtime` itself.
    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;
    let handle = rt.handle().clone();
    rt.block_on(async_main(handle))
}

async fn async_main(handle: tokio::runtime::Handle) -> Result<()> {
    let engine = build_engine()?;
    let component_path = std::env::var("TURNHARNESS_WASM").expect("TURNHARNESS_WASM must point at the built guest .wasm");
    let component = Component::from_file(&engine, &component_path)?;
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::p2::add_to_linker_async(&mut linker)?;
    Turnharness::add_to_linker::<HostState, HasSelf<HostState>>(&mut linker, |state| state)?;

    let (ticker_stop, ticker) = start_ticker(&engine);

    let mut verdicts: Vec<(&'static str, String)> = Vec::new();

    let q1 = test_q1_turn_shape(&engine, &component, &linker).await;
    let q1_line = match q1 {
        Ok(v) => format!("PASS — {v}"),
        Err(e) => format!("FAIL — {e:#}"),
    };
    println!("[host] Q1: {q1_line}");
    verdicts.push(("Q1-turn-shape", q1_line));

    let q2 = test_q2_cancellation(&engine, &component, &linker).await;
    let q2_line = match q2 {
        Ok(v) => format!("done — {v}"),
        Err(e) => format!("FAIL — {e:#}"),
    };
    println!("[host] Q2: {q2_line}");
    verdicts.push(("Q2-cancellation", q2_line));

    // Q3 — epoch lever, symmetric + asymmetric.
    match run_burn_pair(&engine, &component, &linker, 40_000_000, 40_000_000, true).await {
        Ok((ta, tb, ha, hb)) => {
            let (later, earlier) = if ta > tb { (ta, tb) } else { (tb, ta) };
            let ratio = later.as_secs_f64() / earlier.as_secs_f64().max(1e-9);
            let v = format!("t_a={ta:?} t_b={tb:?} ratio={ratio:.2} hits_a={ha} hits_b={hb}");
            println!("[host] Q3-epoch-sym: {v}");
            verdicts.push(("Q3-epoch-sym", v));
        }
        Err(e) => {
            println!("[host] Q3-epoch-sym: FAIL — {e:#}");
            verdicts.push(("Q3-epoch-sym", format!("FAIL — {e:#}")));
        }
    }
    match run_burn_pair(&engine, &component, &linker, 300_000_000, 5_000_000, true).await {
        Ok((ta, tb, ha, hb)) => {
            let v = format!("huge(a) t_a={ta:?}, tiny(b) t_b={tb:?}, tiny-before-huge={} hits_a={ha} hits_b={hb}", tb < ta);
            println!("[host] Q3-epoch-asym: {v}");
            verdicts.push(("Q3-epoch-asym", v));
        }
        Err(e) => {
            println!("[host] Q3-epoch-asym: FAIL — {e:#}");
            verdicts.push(("Q3-epoch-asym", format!("FAIL — {e:#}")));
        }
    }

    // Q3 — fuel-only lever (separate Engine, no epoch_interruption at all).
    let engine_fuel = build_engine_fuel_only()?;
    let component_fuel = Component::from_file(&engine_fuel, &component_path)?;
    let mut linker_fuel = Linker::new(&engine_fuel);
    wasmtime_wasi::p2::add_to_linker_async(&mut linker_fuel)?;
    Turnharness::add_to_linker::<HostState, HasSelf<HostState>>(&mut linker_fuel, |state| state)?;
    match run_burn_pair(&engine_fuel, &component_fuel, &linker_fuel, 40_000_000, 40_000_000, false).await {
        Ok((ta, tb, _, _)) => {
            let (later, earlier) = if ta > tb { (ta, tb) } else { (tb, ta) };
            let ratio = later.as_secs_f64() / earlier.as_secs_f64().max(1e-9);
            let v = format!("t_a={ta:?} t_b={tb:?} ratio={ratio:.2}");
            println!("[host] Q3-fuel-sym: {v}");
            verdicts.push(("Q3-fuel-sym", v));
        }
        Err(e) => {
            println!("[host] Q3-fuel-sym: FAIL — {e:#}");
            verdicts.push(("Q3-fuel-sym", format!("FAIL — {e:#}")));
        }
    }
    match run_burn_pair(&engine_fuel, &component_fuel, &linker_fuel, 300_000_000, 5_000_000, false).await {
        Ok((ta, tb, _, _)) => {
            let v = format!("huge(a) t_a={ta:?}, tiny(b) t_b={tb:?}, tiny-before-huge={}", tb < ta);
            println!("[host] Q3-fuel-asym: {v}");
            verdicts.push(("Q3-fuel-asym", v));
        }
        Err(e) => {
            println!("[host] Q3-fuel-asym: FAIL — {e:#}");
            verdicts.push(("Q3-fuel-asym", format!("FAIL — {e:#}")));
        }
    }

    let q4 = test_q4_jobs_while_turn_live(&engine, &component, &linker).await;
    let q4_line = match q4 {
        Ok(v) => v,
        Err(e) => format!("FAIL — {e:#}"),
    };
    println!("[host] Q4: {q4_line}");
    verdicts.push(("Q4-jobs-while-live", q4_line));

    let q5 = test_q5_budgets(&engine, &component, &linker).await;
    let q5_line = match q5 {
        Ok(v) => v,
        Err(e) => format!("FAIL — {e:#}"),
    };
    println!("[host] Q5: {q5_line}");
    verdicts.push(("Q5-budgets", q5_line));

    let q6 = test_q6_tokio_handle_spawn(&handle, &engine, &component, &linker).await;
    let q6_line = match q6 {
        Ok(v) => v,
        Err(e) => format!("FAIL — {e:#}"),
    };
    println!("[host] Q6: {q6_line}");
    verdicts.push(("Q6-tokio-handle", q6_line));

    ticker_stop.store(true, Ordering::Relaxed);
    ticker.join().expect("epoch ticker thread panicked");

    println!("[host] ==== VERDICTS ====");
    for (name, v) in &verdicts {
        println!("[host] {name}: {v}");
    }

    Ok(())
}
