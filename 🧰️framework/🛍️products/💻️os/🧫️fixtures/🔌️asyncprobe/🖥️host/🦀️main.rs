//! 🧪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-S1 + terra-probe-spikes W6). Host half: instantiates
//! the guest component built from `👽️guest/`, drives its async exports, answers its async imports, and
//! runs the W6 S1-S6 runtime-behaviour spikes for the async-first pooled-actor plan.

use anyhow::Result;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::Duration;
use wasmtime::component::{
    Accessor, AccessorTask, Component, Destination, HasSelf, Linker, ResourceTable,
    StreamProducer, StreamReader, StreamResult, VecBuffer,
};
use wasmtime::{Config, Engine, Store, StoreContextMut, UpdateDeadline};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

wasmtime::component::bindgen!({
    path: "../👽️guest/🧬️schema/📜️world.wit",
    world: "asyncprobe",
});

struct HostState {
    wasi: WasiCtx,
    table: ResourceTable,
    // #region S1 / S1b — progress interleaving log (guest_id, step, elapsed-since-log-start).
    // 🎯️ S1b: wrapped in Arc<Mutex<..>> so TWO SEPARATE `Store`s/HostStates can share ONE log —
    // S1's original single-store case just gets a log with exactly one owner, unchanged behavior.
    progress_log: Arc<Mutex<Vec<(u32, u32, Duration)>>>,
    progress_start: std::time::Instant,
    // #endregion
    // #region S2 — host-import drop-on-cancel flags
    hang_started: Arc<AtomicBool>,
    hang_dropped: Arc<AtomicBool>,
    // #endregion
}

impl WasiView for HostState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi,
            table: &mut self.table,
        }
    }
}

// #region S5 — custom StreamProducer with real Pending/wake choreography
struct WakeyShared {
    queue: VecDeque<u32>,
    done: bool,
    waker: Option<Waker>,
    poll_count: u32,
}

struct WakeyProducer {
    shared: Arc<Mutex<WakeyShared>>,
}

impl StreamProducer<HostState> for WakeyProducer {
    type Item = u32;
    type Buffer = VecBuffer<u32>;

    fn poll_produce<'a>(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        _store: StoreContextMut<'a, HostState>,
        mut destination: Destination<'a, u32, VecBuffer<u32>>,
        _finish: bool,
    ) -> Poll<wasmtime::Result<StreamResult>> {
        let mut shared = self.shared.lock().unwrap();
        shared.poll_count += 1;
        if shared.queue.is_empty() {
            if shared.done {
                return Poll::Ready(Ok(StreamResult::Dropped));
            }
            // 🎯️ S5 core assertion: store the waker instead of spinning, exactly as the trait's
            // docs require, and rely on a foreign OS thread to invoke it later.
            shared.waker = Some(cx.waker().clone());
            return Poll::Pending;
        }
        let items: Vec<u32> = shared.queue.drain(..).collect();
        destination.set_buffer(items.into());
        Poll::Ready(Ok(StreamResult::Completed))
    }
}
// #endregion

// 🎯️ G3a wiring: the async `echo` import is implemented via the `HasSelf<T>`/`Accessor` pattern
// wasmtime 47.0.3's concurrent-call model requires for host imports on an async world.
impl AsyncprobeImportsWithStore<HostState> for HasSelf<HostState> {
    async fn echo(_accessor: &Accessor<HostState, Self>, s: String) -> String {
        println!("[host] echo import called from inside guest await: {s}");
        format!("echo:{s}")
    }

    // #region S6 — deliberately pends once so waker propagation through the guest's nested
    // executor is genuinely exercised, not trivially satisfied by an import that never pends.
    async fn delayed_echo(_accessor: &Accessor<HostState, Self>, s: String) -> String {
        struct YieldOnce(bool);
        impl Future for YieldOnce {
            type Output = ();
            fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
                if self.0 {
                    Poll::Ready(())
                } else {
                    self.0 = true;
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
            }
        }
        YieldOnce(false).await;
        format!("delayed:{s}")
    }
    // #endregion

    // #region S2 — an import that never completes; only useful for observing its future's Drop.
    async fn hang(accessor: &Accessor<HostState, Self>, id: u32) -> u32 {
        struct DropSignal(Arc<AtomicBool>);
        impl Drop for DropSignal {
            fn drop(&mut self) {
                self.0.store(true, Ordering::SeqCst);
            }
        }
        let (started, dropped) = accessor.with(|mut access| {
            (
                access.get().hang_started.clone(),
                access.get().hang_dropped.clone(),
            )
        });
        started.store(true, Ordering::SeqCst);
        let _guard = DropSignal(dropped);
        println!("[host] hang({id}) started, awaiting pending forever until dropped");
        std::future::pending::<()>().await;
        unreachable!("hang() must never resolve — it exists to be cancelled");
    }
    // #endregion

    // #region S7 — deterministic Pending-then-Ready async import (no wall-clock timing needed):
    // requires exactly 5 real `poll()` calls before resolving, so the test can distinguish "the
    // guest's manual poll loop genuinely drove this subtask forward" from "nothing happened at
    // all" without any background thread/timer confound.
    async fn s7_slow_op(_accessor: &Accessor<HostState, Self>, id: u32) -> u32 {
        struct PendingNTimes {
            remaining: u32,
            value: u32,
        }
        impl Future for PendingNTimes {
            type Output = u32;
            fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32> {
                if self.remaining == 0 {
                    Poll::Ready(self.value)
                } else {
                    self.remaining -= 1;
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
            }
        }
        PendingNTimes {
            remaining: 5,
            value: id.wrapping_add(100),
        }
        .await
    }
    // #endregion
}

impl AsyncprobeImports for HostState {
    fn was_hang_dropped(&mut self) -> bool {
        self.hang_dropped.load(Ordering::SeqCst)
    }

    fn was_hang_started(&mut self) -> bool {
        self.hang_started.load(Ordering::SeqCst)
    }

    fn progress(&mut self, guest_id: u32, step: u32) {
        let elapsed = self.progress_start.elapsed();
        self.progress_log.lock().unwrap().push((guest_id, step, elapsed));
    }
}

fn main() -> Result<()> {
    futures::executor::block_on(async_main())
}

async fn async_main() -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model_async(true);
    config.concurrency_support(true);
    // 🧪️ S1 finding under test: the production engine also enables fuel + epoch interruption
    // alongside the two async knobs above. That four-way combination has never been exercised
    // together before this probe.
    config.consume_fuel(true);
    config.epoch_interruption(true);
    let engine = Engine::new(&config)?;

    let component_path = std::env::var("ASYNCPROBE_WASM").unwrap_or_else(|_| {
        "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/🎯️target-probe/wasm32-wasip2/release/semio_asyncprobe_guest.wasm".to_string()
    });
    let component = Component::from_file(&engine, &component_path)?;

    let mut linker = Linker::new(&engine);
    wasmtime_wasi::p2::add_to_linker_async(&mut linker)?;
    Asyncprobe::add_to_linker::<HostState, HasSelf<HostState>>(&mut linker, |state| state)?;

    let wasi = WasiCtxBuilder::new().inherit_stdio().build();
    let table = ResourceTable::new();
    let mut store = Store::new(
        &engine,
        HostState {
            wasi,
            table,
            progress_log: Arc::new(Mutex::new(Vec::new())),
            progress_start: std::time::Instant::now(),
            hang_started: Arc::new(AtomicBool::new(false)),
            hang_dropped: Arc::new(AtomicBool::new(false)),
        },
    );

    // 🧪️ S1: required once fuel+epoch are enabled on the engine, else the store traps immediately
    // (0 fuel, 0 epoch deadline are both "already elapsed" by default).
    store.set_fuel(u64::MAX)?;
    let epoch_callback_hits = Arc::new(std::sync::atomic::AtomicU64::new(0));
    let epoch_callback_hits_cb = epoch_callback_hits.clone();
    store.epoch_deadline_callback(move |_ctx| {
        epoch_callback_hits_cb.fetch_add(1, Ordering::Relaxed);
        Ok(UpdateDeadline::Yield(1))
    });
    store.set_epoch_deadline(1);

    // 🧪️ S1: 1ms epoch ticker on its own OS thread, running for the whole probe lifetime.
    let ticker_engine = engine.clone();
    let ticker_stop = Arc::new(AtomicBool::new(false));
    let ticker_stop_thread = ticker_stop.clone();
    let ticker = std::thread::spawn(move || {
        while !ticker_stop_thread.load(Ordering::Relaxed) {
            ticker_engine.increment_epoch();
            std::thread::sleep(Duration::from_millis(1));
        }
    });

    let instance = Asyncprobe::instantiate_async(&mut store, &component, &linker).await?;
    // 🧪️ S1: two more independently-instantiated guests ("actors") sharing the store/engine —
    // dedicated instances (not shared with `instance`, which `Asyncprobe` doesn't implement
    // `Clone`/`Copy` for) because `Accessor::spawn` needs to move an owned instance per task.
    let instance_s1a = Asyncprobe::instantiate_async(&mut store, &component, &linker).await?;
    let instance_s1b = Asyncprobe::instantiate_async(&mut store, &component, &linker).await?;

    let mut verdicts: Vec<(&'static str, String)> = Vec::new();

    store
        .run_concurrent(async move |accessor: &Accessor<HostState>| -> Result<()> {
            // #region G3 baseline
            let ping_result = instance.call_ping(accessor, 41).await?;
            println!("[host] ping(41) = {ping_result}");
            assert_eq!(ping_result, 42, "ping(41) should equal 42");

            let events: StreamReader<u32> =
                accessor.with(|access| StreamReader::new(access, vec![1u32, 2, 3, 4, 5, 6]))?;
            let sum = instance.call_run(accessor, events).await?;
            println!("[host] run(stream) summed = {sum}");
            assert_eq!(sum, 21, "1+2+3+4+5+6 should equal 21");
            println!("[host] G3 PASS");
            // #endregion

            // #region S1 — epoch-Yield fairness under two concurrent CPU-bound guest instances
            {
                // Large enough that a single call spans many 1ms epoch ticks (each burn call must
                // run for multiple milliseconds of wall-clock time, or the ticker thread never
                // gets a chance to increment the epoch even once during it).
                let iters = 40_000_000u32;
                let start = std::time::Instant::now();
                // 🎯️ S1 finding: a plain `futures::join!` over two `call_burn(...)` futures does
                // NOT interleave (see terra-probe-spikes-run2.txt) — both futures live on the same
                // wasmtime "guest thread" and are driven sequentially by the outer Rust executor.
                // True concurrency requires enrolling each call as its own wasmtime-level task via
                // `Accessor::spawn`, which is what the pooled-actor design would need to use anyway
                // (each pooled actor = its own spawned task, not a `join!`'d future).
                struct BurnTask {
                    instance: Asyncprobe,
                    guest_id: u32,
                    iters: u32,
                    result_tx: futures::channel::oneshot::Sender<wasmtime::Result<u32>>,
                }
                impl AccessorTask<HostState> for BurnTask {
                    async fn run(self, accessor: &Accessor<HostState>) -> wasmtime::Result<()> {
                        let r = self
                            .instance
                            .call_burn(accessor, self.guest_id, self.iters)
                            .await;
                        let _ = self.result_tx.send(r);
                        Ok(())
                    }
                }
                let (tx_a, rx_a) = futures::channel::oneshot::channel();
                let (tx_b, rx_b) = futures::channel::oneshot::channel();
                accessor.spawn(BurnTask {
                    instance: instance_s1a,
                    guest_id: 0,
                    iters,
                    result_tx: tx_a,
                })?;
                accessor.spawn(BurnTask {
                    instance: instance_s1b,
                    guest_id: 1,
                    iters,
                    result_tx: tx_b,
                })?;
                let a = rx_a.await.expect("burn(0) task dropped without sending")?;
                let b = rx_b.await.expect("burn(1) task dropped without sending")?;
                let elapsed = start.elapsed();
                let hits = epoch_callback_hits.load(Ordering::Relaxed);
                println!(
                    "[host] S1: burn(0) = {a}, burn(1) = {b}, elapsed = {elapsed:?}, epoch_deadline_callback hits = {hits}"
                );
                let log = accessor.with(|mut access| access.get().progress_log.lock().unwrap().clone());
                println!("[host] S1: progress log has {} entries", log.len());
                // Interleaved iff guest-id 1 shows up before guest-id 0's log is fully finished,
                // and vice versa — i.e. neither instance's entries form one unbroken prefix.
                let last_zero_idx = log.iter().rposition(|(id, _, _)| *id == 0);
                let first_one_idx = log.iter().position(|(id, _, _)| *id == 1);
                let interleaved = match (last_zero_idx, first_one_idx) {
                    (Some(lz), Some(fo)) => fo < lz,
                    _ => false,
                };
                let verdict = if interleaved {
                    "PASS — burn(0)/burn(1) progress entries interleave under epoch-Yield".to_string()
                } else {
                    format!(
                        "FAIL — no interleaving observed (last id=0 at {last_zero_idx:?}, first id=1 at {first_one_idx:?})"
                    )
                };
                println!("[host] S1: {verdict}");
                verdicts.push(("S1", verdict));
            }
            // #endregion

            // #region S3 + S5 — cross-export concurrency while parked + custom StreamProducer wake
            {
                let shared = Arc::new(Mutex::new(WakeyShared {
                    queue: VecDeque::new(),
                    done: false,
                    waker: None,
                    poll_count: 0,
                }));
                let reader: StreamReader<u32> = accessor.with(|access| {
                    StreamReader::new(
                        access,
                        WakeyProducer {
                            shared: shared.clone(),
                        },
                    )
                })?;

                let mut run_fut = Box::pin(instance.call_run(accessor, reader));
                let noop_waker = Waker::noop();
                let mut noop_cx = Context::from_waker(noop_waker);
                let parked = matches!(run_fut.as_mut().poll(&mut noop_cx), Poll::Pending);
                println!("[host] S3/S5: run() parked on manual first poll = {parked}");

                // S3: call a *different* export while `run_fut` is still alive and parked.
                let checkpoint_result = instance.call_checkpoint(accessor).await;
                let s3_verdict = match &checkpoint_result {
                    Ok(v) => format!("PASS — checkpoint() = {v} succeeded while run() was parked"),
                    Err(e) => format!("FAIL — checkpoint() while parked errored: {e:#}"),
                };
                println!("[host] S3: {s3_verdict}");
                verdicts.push(("S3", s3_verdict));

                // S5: background OS thread delivers items in two waves with real delay + explicit
                // wake, proving the waker captured inside `poll_produce` is the one that matters.
                let shared_bg = shared.clone();
                let pusher = std::thread::spawn(move || {
                    std::thread::sleep(Duration::from_millis(30));
                    {
                        let mut sh = shared_bg.lock().unwrap();
                        sh.queue.extend([10u32, 20, 30]);
                    }
                    if let Some(w) = shared_bg.lock().unwrap().waker.take() {
                        w.wake();
                    }
                    std::thread::sleep(Duration::from_millis(30));
                    {
                        let mut sh = shared_bg.lock().unwrap();
                        sh.queue.extend([40u32, 50]);
                        sh.done = true;
                    }
                    if let Some(w) = shared_bg.lock().unwrap().waker.take() {
                        w.wake();
                    }
                });

                let run_result = run_fut.await;
                pusher.join().expect("pusher thread panicked");
                let poll_count = shared.lock().unwrap().poll_count;
                let s5_verdict = match run_result {
                    Ok(sum) if sum == 150 => format!(
                        "PASS — run() resumed after delayed wake, summed = {sum} ({poll_count} poll_produce calls)"
                    ),
                    Ok(sum) => format!("FAIL — resumed but wrong sum {sum} (expected 150)"),
                    Err(e) => format!("FAIL — run() errored: {e:#}"),
                };
                println!("[host] S5: {s5_verdict}");
                verdicts.push(("S5", s5_verdict));
            }
            // #endregion

            // #region S2 — host-import future drop on guest-side cancel
            {
                let cancel_result = instance.call_cancel_probe(accessor).await;
                let started = accessor.with(|mut access| access.get().hang_started.load(Ordering::SeqCst));
                let dropped = accessor.with(|mut access| access.get().hang_dropped.load(Ordering::SeqCst));
                let verdict = match cancel_result {
                    Ok(_) if started && dropped => {
                        "PASS — hang()'s host future was dropped after guest-side cancel".to_string()
                    }
                    Ok(_) if started && !dropped => {
                        "FAIL — hang() started but its host future was never dropped → fallback: cancel-at-completion-sink".to_string()
                    }
                    Ok(_) => format!(
                        "UNRESOLVED — hang() reported started={started} dropped={dropped}, ambiguous"
                    ),
                    Err(e) => format!("UNRESOLVED — cancel_probe() errored: {e:#}"),
                };
                println!("[host] S2: {verdict}");
                verdicts.push(("S2", verdict));
            }
            // #endregion

            // #region S6 — nested guest-side local executor
            {
                let nested_result = instance.call_nested_exec_probe(accessor).await;
                let verdict = match nested_result {
                    Ok(s) if s == "delayed:nested" => {
                        format!("PASS — nested Rc<RefCell> executor resumed correctly, got {s:?}")
                    }
                    Ok(s) => format!("FAIL — wrong value from nested executor: {s:?}"),
                    Err(e) => format!(
                        "FAIL — nested_exec_probe() errored: {e:#} → fallback: cfg spawn onto wit_bindgen::rt::async_support::spawn"
                    ),
                };
                println!("[host] S6: {verdict}");
                verdicts.push(("S6", verdict));
            }
            // #endregion

            println!("[host] ==== VERDICTS ====");
            for (name, v) in &verdicts {
                println!("[host] {name}: {v}");
            }

            Ok(())
        })
        .await??;

    // #region S1b — inter-store fairness (coordinator follow-up on S1)
    // 🎯️ S1 measured INTRA-store concurrency: two guest tasks inside ONE `Store`, scheduled by
    // wasmtime's own `Accessor::spawn` machinery. The design never asks for that — its rule is one
    // root task per `Store`, never concurrent reentrant calls into a single instance. Fairness is
    // supposed to come from a level up: actor A's `Store` and actor B's `Store` are SEPARATE
    // `run_concurrent` futures, multiplexed by OUR host-level executor on one thread via
    // `futures::join!` (current-thread — no `Accessor::spawn`, no extra OS threads for guest work).
    // S1b tests exactly that shape, then a fuel-only variant for sub-question 3.
    // `verdicts` above was moved into the S1-S6 closure (`async move`) and is gone from this
    // scope — S1b gets its own Vec rather than trying to claw the moved one back.
    let mut verdicts_s1b: Vec<(&'static str, String)> = Vec::new();
    {
        let iters = 40_000_000u32;

        fn make_store_epoch(
            engine: &Engine,
            shared_log: Arc<Mutex<Vec<(u32, u32, Duration)>>>,
            progress_start: std::time::Instant,
        ) -> (Store<HostState>, Arc<std::sync::atomic::AtomicU64>) {
            let wasi = WasiCtxBuilder::new().inherit_stdio().build();
            let mut store = Store::new(
                engine,
                HostState {
                    wasi,
                    table: ResourceTable::new(),
                    progress_log: shared_log,
                    progress_start,
                    hang_started: Arc::new(AtomicBool::new(false)),
                    hang_dropped: Arc::new(AtomicBool::new(false)),
                },
            );
            store.set_fuel(u64::MAX).expect("set_fuel");
            let hits = Arc::new(std::sync::atomic::AtomicU64::new(0));
            let hits_cb = hits.clone();
            store.epoch_deadline_callback(move |_ctx| {
                hits_cb.fetch_add(1, Ordering::Relaxed);
                Ok(UpdateDeadline::Yield(1))
            });
            store.set_epoch_deadline(1);
            (store, hits)
        }

        let shared_log: Arc<Mutex<Vec<(u32, u32, Duration)>>> = Arc::new(Mutex::new(Vec::new()));
        let progress_start = std::time::Instant::now();
        // 🎯️ same shared `engine`/`component`/`linker` as S1 — only the Store, and therefore the
        // instance, differ. The epoch ticker thread started before S1 is still running here.
        let (mut store_a, hits_a) = make_store_epoch(&engine, shared_log.clone(), progress_start);
        let (mut store_b, hits_b) = make_store_epoch(&engine, shared_log.clone(), progress_start);
        let instance_a = Asyncprobe::instantiate_async(&mut store_a, &component, &linker).await?;
        let instance_b = Asyncprobe::instantiate_async(&mut store_b, &component, &linker).await?;

        let start = std::time::Instant::now();
        let fut_a = store_a.run_concurrent(async move |accessor: &Accessor<HostState>| -> Result<u32> {
            Ok(instance_a.call_burn(accessor, 20, iters).await?)
        });
        let fut_b = store_b.run_concurrent(async move |accessor: &Accessor<HostState>| -> Result<u32> {
            Ok(instance_b.call_burn(accessor, 21, iters).await?)
        });
        // 🎯️ S1b core mechanism under test: a HOST-LEVEL `futures::join!` of two independent
        // `Store::run_concurrent` futures on the current thread — NOT `Accessor::spawn` inside a
        // single store. This is `futures::executor::block_on`'s single-threaded poll loop, exactly
        // the shape our own reactor/executor would use to multiplex actor A and actor B.
        let (ra, rb) = futures::join!(fut_a, fut_b);
        let a = ra??;
        let b = rb??;
        let elapsed = start.elapsed();

        let log = shared_log.lock().unwrap().clone();
        let switches = log.windows(2).filter(|w| w[0].0 != w[1].0).count();
        let interleaved = switches > 0;

        println!(
            "[host] S1b: burn(20)={a}, burn(21)={b}, elapsed={elapsed:?}, hits_a={}, hits_b={}, log entries={}, switches={switches}",
            hits_a.load(Ordering::Relaxed),
            hits_b.load(Ordering::Relaxed),
            log.len(),
        );
        if let Some((id0, step0, t0)) = log.first() {
            println!("[host] S1b: first entry id={id0} step={step0} t={t0:?}");
        }
        if let Some((idn, stepn, tn)) = log.last() {
            println!("[host] S1b: last entry id={idn} step={stepn} t={tn:?}");
        }
        // 🎯️ sub-question 1 (granularity): print up to 10 switch points with timestamps so a
        // human can read off how many progress markers (each = 4096 loop iterations) elapse
        // between one store yielding and the other being polled.
        let mut printed = 0u32;
        for w in log.windows(2) {
            if w[0].0 != w[1].0 {
                println!(
                    "[host] S1b: switch #{printed} — (id={}, step={}, t={:?}) -> (id={}, step={}, t={:?})",
                    w[0].0, w[0].1, w[0].2, w[1].0, w[1].1, w[1].2
                );
                printed += 1;
                if printed >= 10 {
                    break;
                }
            }
        }

        // 🎯️ sub-question 2: if `interleaved` is true, epoch-Yield(1) demonstrably returned
        // control all the way out to OUR `futures::join!` (not just wasmtime's internal
        // intra-store scheduler) — that is the only way the sibling `run_concurrent` future could
        // ever get polled. If false, the same failure as S1 is reproduced one level up: whichever
        // `run_concurrent` future is polled first internally loops itself back to "runnable" on
        // every Yield and is never observed as `Poll::Pending` by the outer `join!`.
        let verdict_1b = if interleaved {
            format!(
                "GO — {switches} switches across {} entries; inter-store host-level `futures::join!` DOES interleave epoch-Yield(1)'d CPU-bound guests. Answers Q2: Yield(1) does return control to our executor, not just wasmtime's internal scheduler.",
                log.len()
            )
        } else {
            format!(
                "NO-GO — no interleaving across separate Stores either (burn(20) ran to completion before burn(21)'s first entry). Answers Q2: `run_concurrent`'s future never yielded `Poll::Pending` to our `futures::join!` while its own guest kept being re-runnable after Yield(1) — the same failure as S1, one level up. hits_a={} hits_b={} prove the epoch callback DID fire repeatedly during this run, ruling out 'loop finished before first tick'.",
                hits_a.load(Ordering::Relaxed),
                hits_b.load(Ordering::Relaxed)
            )
        };
        println!("[host] S1b: {verdict_1b}");
        verdicts_s1b.push(("S1b", verdict_1b));

        // #region S1b sub-question 3 — fuel-interval-only yield (no epoch interruption at all)
        // 🎯️ A fully separate `Engine`/`Config` with `epoch_interruption` never enabled, so there
        // is no epoch-deadline requirement on these stores at all — isolates fuel as the ONLY
        // interruption lever, using the same two-Store/host-level-`join!` shape as above.
        {
            let mut config_fuel = Config::new();
            config_fuel.wasm_component_model_async(true);
            config_fuel.concurrency_support(true);
            config_fuel.consume_fuel(true);
            let engine_fuel = Engine::new(&config_fuel)?;
            let component_fuel = Component::from_file(&engine_fuel, &component_path)?;
            let mut linker_fuel = Linker::new(&engine_fuel);
            wasmtime_wasi::p2::add_to_linker_async(&mut linker_fuel)?;
            Asyncprobe::add_to_linker::<HostState, HasSelf<HostState>>(&mut linker_fuel, |state| state)?;

            fn make_store_fuel(
                engine: &Engine,
                shared_log: Arc<Mutex<Vec<(u32, u32, Duration)>>>,
                progress_start: std::time::Instant,
            ) -> Store<HostState> {
                let wasi = WasiCtxBuilder::new().inherit_stdio().build();
                let mut store = Store::new(
                    engine,
                    HostState {
                        wasi,
                        table: ResourceTable::new(),
                        progress_log: shared_log,
                        progress_start,
                        hang_started: Arc::new(AtomicBool::new(false)),
                        hang_dropped: Arc::new(AtomicBool::new(false)),
                    },
                );
                // 🎯️ total fuel budget is effectively unlimited (this is a slicing lever, not a
                // trap-when-exhausted lever); the yield interval is the actual slice size under test.
                store.set_fuel(u64::MAX).expect("set_fuel");
                store
                    .fuel_async_yield_interval(Some(500_000))
                    .expect("fuel_async_yield_interval");
                store
            }

            let shared_log_fuel: Arc<Mutex<Vec<(u32, u32, Duration)>>> = Arc::new(Mutex::new(Vec::new()));
            let progress_start_fuel = std::time::Instant::now();
            let mut store_c = make_store_fuel(&engine_fuel, shared_log_fuel.clone(), progress_start_fuel);
            let mut store_d = make_store_fuel(&engine_fuel, shared_log_fuel.clone(), progress_start_fuel);
            let instance_c =
                Asyncprobe::instantiate_async(&mut store_c, &component_fuel, &linker_fuel).await?;
            let instance_d =
                Asyncprobe::instantiate_async(&mut store_d, &component_fuel, &linker_fuel).await?;

            let fut_c = store_c.run_concurrent(async move |accessor: &Accessor<HostState>| -> Result<u32> {
                Ok(instance_c.call_burn(accessor, 30, iters).await?)
            });
            let fut_d = store_d.run_concurrent(async move |accessor: &Accessor<HostState>| -> Result<u32> {
                Ok(instance_d.call_burn(accessor, 31, iters).await?)
            });
            let (rc, rd) = futures::join!(fut_c, fut_d);
            let c = rc??;
            let d = rd??;
            let log_fuel = shared_log_fuel.lock().unwrap().clone();
            let switches_fuel = log_fuel.windows(2).filter(|w| w[0].0 != w[1].0).count();
            let verdict_fuel = if switches_fuel > 0 {
                format!(
                    "GO — fuel_async_yield_interval(Some(500_000)) ALSO interleaves with NO epoch interruption enabled at all ({switches_fuel} switches / {} entries). Answers Q3: fuel is an independent, sufficient lever, not merely epoch's helper.",
                    log_fuel.len()
                )
            } else {
                "NO-GO — fuel_async_yield_interval alone does not interleave either; same failure as epoch-Yield. Answers Q3: fuel is not a working alternative lever here.".to_string()
            };
            println!(
                "[host] S1b-Q3 (fuel-only, no epoch): burn(30)={c}, burn(31)={d}, log entries={}, {verdict_fuel}",
                log_fuel.len()
            );
            verdicts_s1b.push(("S1b-Q3-fuel", verdict_fuel));
        }
        // #endregion
    }
    // #endregion

    // #region S1c — pure-CPU preemption, ZERO host imports (coordinator follow-up on S1b)
    // 🎯️ `burn` (S1/S1b) calls the `progress` host import every 4096 iterations — a guest->host
    // call boundary that could itself create an implicit checkpoint independent of epoch/fuel-
    // Yield, confounding S1b's interleaving result. `burn_pure` has ZERO host imports anywhere in
    // its loop. This re-runs S1b's exact shape (separate Stores, host-level `futures::join!`, no
    // `Accessor::spawn`) against `burn_pure`, for both the epoch lever and the fuel-only lever,
    // with two independent, import-free interleaving signals:
    //   (a) SYMMETRIC equal workload: if truly interleaved, both calls finish at roughly the SAME
    //       wall-clock time (~T); if sequential/blocking, the second finishes at ~2T.
    //   (b) ASYMMETRIC tiny-vs-huge workload: if truly interleaved, the tiny call finishes quickly
    //       regardless of the huge call's duration; if sequential/blocking and the huge call is
    //       polled to completion first, the tiny call cannot even start until the huge one resolves,
    //       so its completion timestamp lands at ~(huge's duration + tiny's own small duration).
    let mut verdicts_s1c: Vec<(&'static str, String)> = Vec::new();
    {
        async fn run_pure_pair(
            engine: &Engine,
            component: &Component,
            linker: &Linker<HostState>,
            iters_a: u32,
            iters_b: u32,
            use_epoch: bool,
        ) -> Result<(u32, u32, Duration, Duration, u64, u64)> {
            fn make_state() -> HostState {
                HostState {
                    wasi: WasiCtxBuilder::new().inherit_stdio().build(),
                    table: ResourceTable::new(),
                    progress_log: Arc::new(Mutex::new(Vec::new())),
                    progress_start: std::time::Instant::now(),
                    hang_started: Arc::new(AtomicBool::new(false)),
                    hang_dropped: Arc::new(AtomicBool::new(false)),
                }
            }
            let mut store_a = Store::new(engine, make_state());
            let mut store_b = Store::new(engine, make_state());
            store_a.set_fuel(u64::MAX)?;
            store_b.set_fuel(u64::MAX)?;
            let hits_a = Arc::new(std::sync::atomic::AtomicU64::new(0));
            let hits_b = Arc::new(std::sync::atomic::AtomicU64::new(0));
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
            let instance_a = Asyncprobe::instantiate_async(&mut store_a, component, linker).await?;
            let instance_b = Asyncprobe::instantiate_async(&mut store_b, component, linker).await?;

            let start = std::time::Instant::now();
            let done_a: Arc<Mutex<Option<Duration>>> = Arc::new(Mutex::new(None));
            let done_b: Arc<Mutex<Option<Duration>>> = Arc::new(Mutex::new(None));
            let done_a2 = done_a.clone();
            let done_b2 = done_b.clone();

            // 🎯️ each future timestamps ITS OWN completion the instant its own `.await` resolves —
            // independent of when `futures::join!` (which waits for BOTH) itself returns. This is
            // what lets us tell "B finished quickly, in parallel with A still running" apart from
            // "B only started, and finished, after A fully completed".
            let fut_a = async move {
                let r = store_a
                    .run_concurrent(async move |accessor: &Accessor<HostState>| -> Result<u32> {
                        Ok(instance_a.call_burn_pure(accessor, 40, iters_a).await?)
                    })
                    .await??;
                *done_a2.lock().unwrap() = Some(start.elapsed());
                Ok::<u32, anyhow::Error>(r)
            };
            let fut_b = async move {
                let r = store_b
                    .run_concurrent(async move |accessor: &Accessor<HostState>| -> Result<u32> {
                        Ok(instance_b.call_burn_pure(accessor, 41, iters_b).await?)
                    })
                    .await??;
                *done_b2.lock().unwrap() = Some(start.elapsed());
                Ok::<u32, anyhow::Error>(r)
            };

            // 🎯️ S1c core mechanism under test — identical shape to S1b: HOST-LEVEL futures::join!
            // on the current thread, NOT Accessor::spawn, NOT any import call inside the guest loop.
            let (ra, rb) = futures::join!(fut_a, fut_b);
            let a = ra?;
            let b = rb?;
            let ta = done_a.lock().unwrap().expect("done_a not set");
            let tb = done_b.lock().unwrap().expect("done_b not set");
            Ok((a, b, ta, tb, hits_a.load(Ordering::Relaxed), hits_b.load(Ordering::Relaxed)))
        }

        const SYM_ITERS: u32 = 40_000_000;
        const ASYM_HUGE: u32 = 300_000_000;
        const ASYM_TINY: u32 = 5_000_000;

        // --- epoch lever: reuse the main `engine`/`component`/`linker` (already epoch-configured) ---
        let (a, b, ta, tb, ha, hb) =
            run_pure_pair(&engine, &component, &linker, SYM_ITERS, SYM_ITERS, true).await?;
        let (later, earlier) = if ta > tb { (ta, tb) } else { (tb, ta) };
        let ratio = later.as_secs_f64() / earlier.as_secs_f64().max(1e-9);
        let v = if ratio < 1.4 {
            format!("GO — epoch-Yield, symmetric pure-CPU (no imports): a={a} t_a={ta:?}, b={b} t_b={tb:?}, ratio={ratio:.2} (~1.0 = interleaved). hits_a={ha} hits_b={hb}")
        } else {
            format!("NO-GO — epoch-Yield, symmetric pure-CPU (no imports): a={a} t_a={ta:?}, b={b} t_b={tb:?}, ratio={ratio:.2} (~2.0 = sequential, second call only ran after first). hits_a={ha} hits_b={hb}")
        };
        println!("[host] S1c-sym-epoch: {v}");
        verdicts_s1c.push(("S1c-sym-epoch", v));

        let (a, b, ta, tb, ha, hb) =
            run_pure_pair(&engine, &component, &linker, ASYM_HUGE, ASYM_TINY, true).await?;
        let v = if tb < ta {
            format!("GO — epoch-Yield, asymmetric pure-CPU (no imports): huge(a)={a} t_a={ta:?}, tiny(b)={b} t_b={tb:?} — tiny finished BEFORE huge, proving it was not blocked behind it. hits_a={ha} hits_b={hb}")
        } else {
            format!("NO-GO — epoch-Yield, asymmetric pure-CPU (no imports): huge(a)={a} t_a={ta:?}, tiny(b)={b} t_b={tb:?} — tiny only finished AFTER huge (t_b >= t_a), i.e. it never got to run until the huge call fully resolved. hits_a={ha} hits_b={hb}")
        };
        println!("[host] S1c-asym-epoch: {v}");
        verdicts_s1c.push(("S1c-asym-epoch", v));

        // --- fuel-only lever: fresh Engine with epoch_interruption never enabled at all ---
        let mut config_fuel_c = Config::new();
        config_fuel_c.wasm_component_model_async(true);
        config_fuel_c.concurrency_support(true);
        config_fuel_c.consume_fuel(true);
        let engine_fuel_c = Engine::new(&config_fuel_c)?;
        let component_fuel_c = Component::from_file(&engine_fuel_c, &component_path)?;
        let mut linker_fuel_c = Linker::new(&engine_fuel_c);
        wasmtime_wasi::p2::add_to_linker_async(&mut linker_fuel_c)?;
        Asyncprobe::add_to_linker::<HostState, HasSelf<HostState>>(&mut linker_fuel_c, |state| state)?;

        let (a, b, ta, tb, _ha, _hb) = run_pure_pair(
            &engine_fuel_c,
            &component_fuel_c,
            &linker_fuel_c,
            SYM_ITERS,
            SYM_ITERS,
            false,
        )
        .await?;
        let (later, earlier) = if ta > tb { (ta, tb) } else { (tb, ta) };
        let ratio = later.as_secs_f64() / earlier.as_secs_f64().max(1e-9);
        let v = if ratio < 1.4 {
            format!("GO — fuel-only (no epoch at all), symmetric pure-CPU (no imports): a={a} t_a={ta:?}, b={b} t_b={tb:?}, ratio={ratio:.2}")
        } else {
            format!("NO-GO — fuel-only (no epoch at all), symmetric pure-CPU (no imports): a={a} t_a={ta:?}, b={b} t_b={tb:?}, ratio={ratio:.2} (~2.0 = sequential)")
        };
        println!("[host] S1c-sym-fuel: {v}");
        verdicts_s1c.push(("S1c-sym-fuel", v));

        let (a, b, ta, tb, _ha, _hb) = run_pure_pair(
            &engine_fuel_c,
            &component_fuel_c,
            &linker_fuel_c,
            ASYM_HUGE,
            ASYM_TINY,
            false,
        )
        .await?;
        let v = if tb < ta {
            format!("GO — fuel-only (no epoch at all), asymmetric pure-CPU (no imports): huge(a)={a} t_a={ta:?}, tiny(b)={b} t_b={tb:?} — tiny finished BEFORE huge")
        } else {
            format!("NO-GO — fuel-only (no epoch at all), asymmetric pure-CPU (no imports): huge(a)={a} t_a={ta:?}, tiny(b)={b} t_b={tb:?} — tiny only finished AFTER huge")
        };
        println!("[host] S1c-asym-fuel: {v}");
        verdicts_s1c.push(("S1c-asym-fuel", v));
    }
    // #endregion

    // #region S7 — can a SYNC-lifted export drive an ASYNC-lowered import? (coordinator follow-up
    // on the real `jobs`/`checkpoint` schema — `start-job`/`step-job`/`cancel-job`/`checkpoint`/
    // `restore` are all plain `func`, not `async func`, in `🔌️plugin/🧬️schema/📜️component.wit`)
    // 🎯️ deliberately its OWN fresh `Store` with epoch deadline pinned far in the future and fuel
    // maxed out — this test is about the sync/async ABI boundary itself, not epoch/fuel preemption,
    // so both are taken out of the picture as confounds (S1c's lesson: isolate the one variable
    // actually under test).
    let mut verdicts_s7: Vec<(&'static str, String)> = Vec::new();
    {
        let wasi7 = WasiCtxBuilder::new().inherit_stdio().build();
        let mut store7 = Store::new(
            &engine,
            HostState {
                wasi: wasi7,
                table: ResourceTable::new(),
                progress_log: Arc::new(Mutex::new(Vec::new())),
                progress_start: std::time::Instant::now(),
                hang_started: Arc::new(AtomicBool::new(false)),
                hang_dropped: Arc::new(AtomicBool::new(false)),
            },
        );
        store7.set_fuel(u64::MAX)?;
        // 🧪️ self-inflicted bug found and fixed here: `set_epoch_deadline(delta)` sets the
        // deadline as `current_epoch + delta`, NOT an absolute value — passing `u64::MAX` as delta
        // overflows (`current_epoch` is already in the thousands after S1/S1b/S1c's ticker ran),
        // wrapping to a deadline already in the past, which trapped with `wasm trap: interrupt` on
        // the very first guest call with no callback registered (default = trap on reach). Fixed
        // with a large-but-safe delta plus an explicit `Continue` callback — `UpdateDeadline::Yield`
        // is documented (`wasmtime::UpdateDeadline`) to itself trap on a *synchronous* invocation,
        // which every S7 call here is, so `Continue` is the only safe choice regardless.
        store7.set_epoch_deadline(1_000_000);
        store7.epoch_deadline_callback(|_ctx| Ok(UpdateDeadline::Continue(1_000_000)));
        let instance7 = Asyncprobe::instantiate_async(&mut store7, &component, &linker).await?;

        // 🧪️ S7 Q1a-outside: sharpen the mechanism BEFORE entering run_concurrent at all — is a
        // plain sync `func` export categorically uncallable on ANY component-model-async store, or
        // only while reentrant inside an active `run_concurrent` session? The actor's real lifecycle
        // keeps `run_concurrent` continuously active for the instance's whole life either way, so
        // this doesn't change what to DO about it, but it does change WHY, which the coordinator
        // asked to have determined empirically rather than guessed.
        let v0 = match instance7.call_s7_sync_noop(&mut store7, 41) {
            Ok(42) => "GO — plain sync `func` export IS callable via the classic AsContextMut API BEFORE any run_concurrent session is active on this store; returned 42 as expected.".to_string(),
            Ok(v) => format!("PARTIAL — classic sync call returned but wrong value {v} (expected 42)"),
            Err(e) => format!("NO-GO — classic sync call (outside run_concurrent, store otherwise idle) ALSO FAILED: {e:#}"),
        };
        println!("[host] S7-Q1a-outside-run_concurrent: {v0}");
        verdicts_s7.push(("S7-Q1a-outside-run_concurrent", v0));

        let (v1, v2, v3) = store7
            .run_concurrent(
                async move |accessor: &Accessor<HostState>| -> Result<(String, String, String)> {
                    // --- Q1a: trivial sync export, zero imports, called reentrant via accessor.with —
                    // the plain no-argument-await baseline, isolating "callable at all" from "can await".
                    let r = accessor.with(|access| instance7.call_s7_sync_noop(access, 41));
                    let v1 = match r {
                        Ok(42) => "GO — plain sync `func` export IS callable from inside a run_concurrent session on a component-model-async store (via accessor.with(..).call_x(access,..)); returned 42 as expected.".to_string(),
                        Ok(v) => format!("PARTIAL — sync export call returned but wrong value {v} (expected 42)"),
                        Err(e) => format!("NO-GO — plain sync `func` export call FAILED: {e:#}"),
                    };
                    println!("[host] S7-Q1a-sync-noop: {v1}");

                    // --- Q1b/Q2: sync export that internally busy-spin-polls an async import ---
                    let r = accessor.with(|access| instance7.call_s7_sync_awaits_import(access, 7));
                    let v2 = match r {
                        Ok(107) => "GO — sync export's manual busy-spin-poll of the async import's future DID observe it resolve (107 = 7+100 after 5 real Pending polls); the ABI boundary does not forbid a sync export from driving an async import to completion.".to_string(),
                        Ok(v) if v == u32::MAX => "NO-GO (deadlock) — sync export's spin loop hit its 2,000,000-poll cap without ever observing the import resolve; a sync-export-context poll never actually drives the subtask forward.".to_string(),
                        Ok(v) => format!("PARTIAL — unexpected value {v} (expected 107 or the u32::MAX give-up sentinel)"),
                        Err(e) => format!("NO-GO (trap) — sync export awaiting an async import FAILED at the ABI level: {e:#}"),
                    };
                    println!("[host] S7-Q1b-sync-awaits-import: {v2}");

                    // --- Q3 control group: the already-async shape, for direct comparison ---
                    let r = instance7.call_s7_async_awaits_import(accessor, 7).await;
                    let v3 = match r {
                        Ok(107) => "GO (control) — the ASYNC-declared twin resolves normally (107), confirming `s7-slow-op` itself and the harness are correct; any Q1b failure is specifically about the sync/async ABI boundary, not about the import.".to_string(),
                        Ok(v) => format!("PARTIAL — control export returned unexpected value {v}"),
                        Err(e) => format!("UNEXPECTED — control (async) export errored: {e:#}"),
                    };
                    println!("[host] S7-Q3-async-control: {v3}");

                    Ok((v1, v2, v3))
                },
            )
            .await??;
        verdicts_s7.push(("S7-Q1a-sync-noop", v1));
        verdicts_s7.push(("S7-Q1b-sync-awaits-import", v2));
        verdicts_s7.push(("S7-Q3-async-control", v3));

        // --- Q2: the case that actually matters — `run()` concurrently parked on a stream in the
        // SAME instance/Store while the sync-awaits-import export is called reentrant. Only run
        // this if Q1b came back GO — if the ABI already refuses the basic case, adding concurrency
        // cannot make it work, and attempting it risks a genuine, un-diagnosable hang.
        if verdicts_s7[1].1.starts_with("GO") {
            let mut store7b = Store::new(
                &engine,
                HostState {
                    wasi: WasiCtxBuilder::new().inherit_stdio().build(),
                    table: ResourceTable::new(),
                    progress_log: Arc::new(Mutex::new(Vec::new())),
                    progress_start: std::time::Instant::now(),
                    hang_started: Arc::new(AtomicBool::new(false)),
                    hang_dropped: Arc::new(AtomicBool::new(false)),
                },
            );
            store7b.set_fuel(u64::MAX)?;
            store7b.set_epoch_deadline(1_000_000);
            store7b.epoch_deadline_callback(|_ctx| Ok(UpdateDeadline::Continue(1_000_000)));
            let instance7b = Asyncprobe::instantiate_async(&mut store7b, &component, &linker).await?;

            let v4 = store7b
                .run_concurrent(async move |accessor: &Accessor<HostState>| -> Result<String> {
                    // `run()` parked mid-stream, exactly like S3/S5's setup — a WakeyProducer that
                    // never has anything ready keeps it parked indefinitely for this test's duration.
                    let shared = Arc::new(Mutex::new(WakeyShared {
                        queue: VecDeque::new(),
                        done: false,
                        waker: None,
                        poll_count: 0,
                    }));
                    let reader: StreamReader<u32> = accessor.with(|access| {
                        StreamReader::new(
                            access,
                            WakeyProducer {
                                shared: shared.clone(),
                            },
                        )
                    })?;
                    let mut run_fut = Box::pin(instance7b.call_run(accessor, reader));
                    let noop_waker = Waker::noop();
                    let mut noop_cx = Context::from_waker(noop_waker);
                    let parked = matches!(run_fut.as_mut().poll(&mut noop_cx), Poll::Pending);
                    println!("[host] S7-Q2: run() parked = {parked}");

                    // Reentrant call into the SAME instance while `run` is parked — the real shape.
                    let r = accessor.with(|access| instance7b.call_s7_sync_awaits_import(access, 9));
                    let v = match r {
                        Ok(109) => "GO — sync export awaiting an async import completed successfully WHILE `run()` was concurrently parked on a stream in the same instance; no deadlock against the turn loop.".to_string(),
                        Ok(v) if v == u32::MAX => "NO-GO (deadlock) — hit the spin cap specifically when `run()` was also parked in the same instance, even though the standalone Q1b case (no concurrent `run`) succeeded — concurrency with the turn loop is what breaks it.".to_string(),
                        Ok(v) => format!("PARTIAL — unexpected value {v} while run() was parked"),
                        Err(e) => format!("NO-GO (trap) — sync export call FAILED while run() was parked: {e:#}"),
                    };
                    println!("[host] S7-Q2-sync-awaits-import-while-run-parked: {v}");

                    // Drop the still-parked `run_fut` deliberately rather than letting it dangle —
                    // this test only needs to observe the reentrant call's outcome.
                    drop(run_fut);
                    Ok(v)
                })
                .await??;
            verdicts_s7.push(("S7-Q2-sync-awaits-import-while-run-parked", v4));
        } else {
            let skip = "SKIPPED — Q1b (the standalone case, no concurrent run()) already failed, so Q2 cannot succeed either; running it would only risk an undiagnosable hang for no new information.".to_string();
            println!("[host] S7-Q2-sync-awaits-import-while-run-parked: {skip}");
            verdicts_s7.push(("S7-Q2-sync-awaits-import-while-run-parked", skip));
        }

        println!("[host] ==== S7 VERDICTS ====");
        for (name, v) in &verdicts_s7 {
            println!("[host] {name}: {v}");
        }
    }
    // #endregion

    ticker_stop.store(true, Ordering::Relaxed);
    ticker.join().expect("epoch ticker thread panicked");

    println!("[host] ==== S1b VERDICTS ====");
    for (name, v) in &verdicts_s1b {
        println!("[host] {name}: {v}");
    }
    println!("[host] ==== S1c VERDICTS ====");
    for (name, v) in &verdicts_s1c {
        println!("[host] {name}: {v}");
    }

    Ok(())
}

// #region S4 — compile-only Send probes (never executed, only type-checked)
#[allow(dead_code)]
fn _s4_assert_send<T: Send>(_value: T) {}

#[allow(dead_code)]
fn _s4_probe_run_concurrent_is_send(store: &mut Store<HostState>) {
    let fut = store.run_concurrent(async move |accessor: &Accessor<HostState>| {
        let _ = accessor;
    });
    _s4_assert_send(fut);
}

#[allow(dead_code)]
fn _s4_probe_call_ping_is_send(instance: &Asyncprobe, accessor: &Accessor<HostState>) {
    _s4_assert_send(instance.call_ping(accessor, 0));
}
// #endregion
