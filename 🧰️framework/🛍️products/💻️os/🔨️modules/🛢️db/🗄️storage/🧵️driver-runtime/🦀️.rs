//! 🧵 `db_storage_driver_runtime` — the ONE bounded runtime this crate owns, for the external
//! database drivers that are bound to one.
//!
//! 🪢️ Why it exists. `db_storage`'s async-native backends (`db_storage_postgres` over `sqlx`,
//! `db_storage_neo4j` over `neo4rs`) hand their work to the process `WorkerPool` on `Lane::Io`, whose
//! workers are plain `std::thread`s with no foreign runtime context — `semio_framework_async` says so
//! out loud ("No `tokio` in this crate"). `sqlx` needs its runtime's thread-local context at **poll**
//! time, not only when a pool is constructed, so polling a `sqlx` future on a pool worker aborts the
//! whole hub with `this functionality requires a Tokio context` (measured: ticket `26/09/18`,
//! `📓️db2-postgres-neo4j-live-lanes.md` §5b). Pointing the hub at `OS_HUB_STORAGE_BACKEND=postgres`
//! was therefore fatal on first use, while the directory half — whose `sqlx` calls are awaited on the
//! server's own `#[tokio::main]` runtime — worked.
//!
//! 🚪️ The seam, not the library, is what the rest of the crate sees: `db_storage` declares
//! [`db_storage::DbIoAsyncDriverRuntime`] and nothing else there names a concrete runtime. This module
//! is the single implementation, compiled only when a feature that links such a driver is on
//! (`postgres`/`neo4j`), and it is the only place in the `db` crate family that names `tokio`.
//!
//! 🧮️ Bounded on purpose. The runtime exists to drive sockets and timers for a handful of pooled
//! database connections, not to be a second CPU substrate — that is the `WorkerPool`'s job and it
//! stays the `WorkerPool`'s job. Two worker threads by default (`SEMIO_DB_IO_DRIVER_THREADS` raises
//! or lowers it, clamped to 1..=8), 1 MiB stacks, named `semio-db-io-driver-N` so a crash report or a
//! `sample` names them. It is a process singleton created on first use and never shut down: a
//! detached driver holds the backend's executor for the length of its turn, so tearing the runtime
//! down under it would destroy state the backend registry owns.
//!
//! 🌉️ The bridge handed back to `Lane::Io` is a `semio_framework_async::oneshot` receiver — a repo
//! primitive whose `poll` reads a value and registers a waker, which is safe on any thread.

use crate::db_storage::{DbIoAsyncDriverFuture, DbIoAsyncDriverOutput, DbIoAsyncDriverRuntime};
use semio_framework_async::oneshot;
use std::future::Future;
use std::pin::Pin;
use std::sync::OnceLock;
use std::task::{Context, Poll};

/// 🧮️ Worker threads the driver runtime is allowed, before the environment override.
const DEFAULT_DRIVER_THREADS: usize = 2;

/// 🧮️ Stack each driver thread gets — these frames are driver I/O, not the deep artifact-engine
/// turns that made the pool workers' 2 MiB default too small.
const DRIVER_THREAD_STACK_BYTES: usize = 1 << 20;

/// 🎛️ Overrides [`DEFAULT_DRIVER_THREADS`], clamped to a sane band so a typo cannot spawn a fleet.
const DRIVER_THREADS_ENV: &str = "SEMIO_DB_IO_DRIVER_THREADS";

/// @emoji 🧵️ The bounded runtime the external drivers are polled on.
struct DbIoDriverRuntime {
    runtime: tokio::runtime::Runtime,
}

impl DbIoAsyncDriverRuntime for DbIoDriverRuntime {
    fn detach(&self, future: DbIoAsyncDriverFuture) -> DbIoAsyncDriverFuture {
        let (sender, receiver) = oneshot::channel::<DbIoAsyncDriverOutput>();
        self.runtime.spawn(async move {
            let output = future.await;
            drop(sender.send(output));
        });
        Box::pin(DetachedDriver { receiver })
    }
}

/// @emoji 🌉️ What `Lane::Io` polls in place of the driver's own future.
struct DetachedDriver {
    receiver: oneshot::Receiver<DbIoAsyncDriverOutput>,
}

impl Future for DetachedDriver {
    type Output = DbIoAsyncDriverOutput;

    /// 🌉️ Resolving means the detached turn finished and handed its executor, task and terminal
    /// back. A closed rendezvous means the detached turn itself panicked (the runtime is a never-shut-
    /// down singleton, so nothing else can drop the sender), and the executor and task went down with
    /// it — there is nothing truthful to return, so this re-raises. `db_io_poll_async_driver` already
    /// polls inside `catch_unwind` and turns that into the task's `Panic` fault.
    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let receiver = Pin::new(&mut self.get_mut().receiver);
        match receiver.poll(context) {
            Poll::Ready(Ok(output)) => Poll::Ready(output),
            Poll::Ready(Err(_)) => panic!("DB I/O detached driver turn was lost before returning its executor"),
            Poll::Pending => Poll::Pending,
        }
    }
}

static DB_IO_DRIVER_RUNTIME: OnceLock<DbIoDriverRuntime> = OnceLock::new();

/// 📐️ How many driver threads this process runs, after the environment override and the clamp.
fn driver_threads() -> usize {
    std::env::var(DRIVER_THREADS_ENV).ok().and_then(|value| value.trim().parse::<usize>().ok()).unwrap_or(DEFAULT_DRIVER_THREADS).clamp(1, 8)
}

/// @emoji 🏗️ The process's one external-driver runtime, built on first use and never shut down.
///
/// Panics only if the OS refuses to spawn it, which is the same class of failure as
/// `WorkerPool::new`'s own `expect` on a thread it cannot start: there is no storage backend to
/// return from, and continuing would mean silently polling a driver where it aborts the process.
fn runtime() -> &'static DbIoDriverRuntime {
    DB_IO_DRIVER_RUNTIME.get_or_init(|| {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(driver_threads())
            .thread_name("semio-db-io-driver")
            .thread_stack_size(DRIVER_THREAD_STACK_BYTES)
            .enable_io()
            .enable_time()
            .build()
            .expect("db_storage_driver_runtime: failed to build the external-driver runtime");
        DbIoDriverRuntime { runtime }
    })
}

/// @emoji 🧵️ The process's one external-driver runtime, as the interface every caller sees.
pub fn shared() -> &'static dyn DbIoAsyncDriverRuntime {
    runtime()
}

/// @emoji 🚚 Runs one already-boxed future on the driver runtime and hands its completion back
/// through the same repo-owned rendezvous [`DbIoAsyncDriverRuntime::detach`] uses.
///
/// The backend-close stepper needs this: `sqlx`'s `PgPool::close()` is driver I/O like any other and
/// is polled from `Lane::Io`, so it cannot be awaited there either.
pub fn detach_unit(future: Pin<Box<dyn Future<Output = ()> + Send + 'static>>) -> oneshot::Receiver<()> {
    let (sender, receiver) = oneshot::channel::<()>();
    runtime().runtime.spawn(async move {
        future.await;
        let _ = sender.send(());
    });
    receiver
}

/// @emoji 🚪 Runs `build` inside the driver runtime's context — for driver constructors that spawn
/// their own maintenance tasks (`sqlx`'s pool reaper) and would otherwise abort a caller that has no
/// runtime of its own.
pub fn within<T>(build: impl FnOnce() -> T) -> T {
    let _context = runtime().runtime.enter();
    build()
}

/// @emoji 💓 A periodic driver turn owned by its handle — a remote lease renews itself on the
/// driver runtime while its owner holds the handle, and dropping the handle stops it at once.
pub struct DbIoDriverPeriodic {
    task: tokio::task::JoinHandle<()>,
}

impl Drop for DbIoDriverPeriodic {
    fn drop(&mut self) {
        self.task.abort();
    }
}

/// @emoji 💓 Runs `step` every `period` on the driver runtime until it answers `false` or the
/// returned handle is dropped.
pub fn detach_periodic(period: std::time::Duration, mut step: impl FnMut() -> Pin<Box<dyn Future<Output = bool> + Send>> + Send + 'static) -> DbIoDriverPeriodic {
    let task = runtime().runtime.spawn(async move {
        loop {
            tokio::time::sleep(period).await;
            if !step().await {
                return;
            }
        }
    });
    DbIoDriverPeriodic { task }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
