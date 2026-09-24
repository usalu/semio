use super::*;

/// 🧵️ The whole point of this module, as a law: work handed to [`detach_unit`] runs on a thread
/// this runtime owns — never the caller's — and sees a Tokio context there, while the caller
/// learns the outcome through the repo-owned rendezvous without ever entering one itself.
#[test]
fn detached_work_runs_on_a_driver_thread_inside_a_tokio_context() {
    let caller = std::thread::current().id();
    let observed: std::sync::Arc<std::sync::Mutex<Option<(std::thread::ThreadId, Option<String>, bool)>>> = std::sync::Arc::new(std::sync::Mutex::new(None));
    let sink = observed.clone();
    let mut receiver = detach_unit(Box::pin(async move {
        let current = std::thread::current();
        let name = current.name().map(str::to_owned);
        *sink.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((current.id(), name, tokio::runtime::Handle::try_current().is_ok()));
    }));
    assert!(tokio::runtime::Handle::try_current().is_err(), "the calling thread must have no runtime context of its own");
    loop {
        match receiver.try_recv() {
            Ok(()) => break,
            Err(oneshot::TryRecvError::Empty) => std::thread::yield_now(),
            Err(oneshot::TryRecvError::Closed) => panic!("the driver runtime dropped the rendezvous without completing"),
        }
    }
    let (thread, name, in_context) = observed.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().expect("the detached body must have run");
    assert_ne!(thread, caller, "a driver future must never be polled on the thread that submitted it");
    assert!(name.is_some_and(|name| name.starts_with("semio-db-io-driver")), "the detached body must run on a named driver thread");
    assert!(in_context, "the detached body must see the runtime context its driver requires");
}

/// 📐️ The thread budget is bounded on purpose: a typo in the environment cannot spawn a fleet,
/// and it can never fall to zero.
#[test]
fn driver_thread_count_stays_inside_its_band() {
    assert_eq!(driver_threads().clamp(1, 8), driver_threads());
    assert!(driver_threads() >= 1);
}
