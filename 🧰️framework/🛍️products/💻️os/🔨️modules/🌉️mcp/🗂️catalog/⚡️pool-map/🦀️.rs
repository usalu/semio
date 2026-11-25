//! ⚡️ Order-preserving parallel map over the process worker pool, for the catalog's start-up work. See [`super`].

use std::sync::Arc;

const POOL_WORKER_PREFIX: &str = "semio-pool-worker";
const JOBS_PER_WORKER: usize = 4;

/// ⚡️ Maps `work` over `items` on the process worker pool, handing every item back beside its result in the original order.
/// Runs inline for a single item, a single core, or when already on a pool worker (a worker blocking on its own pool could starve it).
/// See [`semio_framework_async::WorkerPool`].
pub fn pool_map<T: Send + 'static, R: Send + 'static>(items: Vec<T>, work: impl Fn(&T) -> R + Send + Sync + 'static) -> Vec<(T, R)> {
    let workers = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    if items.len() < 2 || workers < 2 || std::thread::current().name().is_some_and(|name| name.starts_with(POOL_WORKER_PREFIX)) {
        return items.into_iter().map(|item| { let result = work(&item); (item, result) }).collect();
    }
    let pool = semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, workers));
    let batch_len = (items.len() / (workers * JOBS_PER_WORKER)).max(1);
    let work = Arc::new(work);
    let mut remaining = items.into_iter();
    let mut answers = Vec::new();
    loop {
        let batch: Vec<T> = remaining.by_ref().take(batch_len).collect();
        if batch.is_empty() {
            break;
        }
        let (sender, receiver) = std::sync::mpsc::channel();
        let work = Arc::clone(&work);
        pool.submit(semio_framework_async::Lane::UserVisible, Box::new(move || {
            let _ = sender.send(batch.into_iter().map(|item| { let result = (*work)(&item); (item, result) }).collect::<Vec<_>>());
        }));
        answers.push(receiver);
    }
    answers.into_iter().flat_map(|receiver| receiver.recv().expect("a catalog worker job ended without answering")).collect()
}
