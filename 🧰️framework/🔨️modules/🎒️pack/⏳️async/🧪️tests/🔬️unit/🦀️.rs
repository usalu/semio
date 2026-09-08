
use super::*;
use std::sync::atomic::AtomicUsize;

//#region 🔖️AsyncSource
/// 🧪️ An in-memory `AsyncPackSource` test double that counts how many physical reads it
/// actually served.
struct RecordingSource {
    data: Vec<u8>,
    read_count: AtomicUsize,
}

impl RecordingSource {
    fn new(data: Vec<u8>) -> Self {
        Self { data, read_count: AtomicUsize::new(0) }
    }
}

impl AsyncPackSource for RecordingSource {
    fn len(&self) -> u64 {
        self.data.len() as u64
    }

    async fn read_at(&self, offset: u64, len: usize) -> Result<Vec<u8>, PackError> {
        self.read_count.fetch_add(1, Ordering::SeqCst);
        let start = offset as usize;
        let end = start.checked_add(len).ok_or(PackError::LimitExceeded("test read overflow"))?;
        if end > self.data.len() {
            return Err(PackError::Truncated(offset + len as u64));
        }
        Ok(self.data[start..end].to_vec())
    }
}

/// 🧪️ A test double whose `read_at` never resolves, used to prove `CancellationToken`
/// actually short-circuits an in-flight read rather than waiting for it to finish.
struct HangingSource;

impl AsyncPackSource for HangingSource {
    fn len(&self) -> u64 {
        0
    }

    async fn read_at(&self, _offset: u64, _len: usize) -> Result<Vec<u8>, PackError> {
        std::future::pending::<()>().await;
        unreachable!("HangingSource::read_at never resolves")
    }
}

#[test]
fn cancellation_token_starts_uncancelled_and_latches_once_cancelled() {
    let token = CancellationToken::new();
    assert!(!token.is_cancelled());
    let clone = token.clone();
    clone.cancel();
    assert!(token.is_cancelled(), "cancelling a clone must be visible through the original");
}

/// 🧪️ Wraps a future and counts how many times it is actually polled — used below to prove
/// `CancelWatch` is driven by real waker wakeups rather than a hot poll loop.
struct CountingPoll<F> {
    inner: F,
    polls: Arc<AtomicUsize>,
}

impl<F: Future + Unpin> Future for CountingPoll<F> {
    type Output = F::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.polls.fetch_add(1, Ordering::SeqCst);
        Pin::new(&mut self.inner).poll(cx)
    }
}

/// 🧪️ Regression test for the poll-sleep defect this packet fixes: `CancelWatch` must stay
/// `Pending` until a genuinely separate OS thread transitions `CancellationToken`'s state
/// (calls `cancel()`), then resolve — driven purely by `Waker::wake`, never by sleeping or
/// busy-repolling. A busy-poll implementation (the original 200µs-sleep version) would rack
/// up roughly `20ms / 200µs` ≈ 100 polls waiting out the delay below; a correct waker-based
/// implementation polls a small constant number of times (park, wake, resolve) regardless of
/// how long the other thread takes.
#[test]
fn cancel_watch_resolves_from_another_thread_via_waker_without_spinning_or_sleeping() {
    let token = CancellationToken::new();
    let canceller_token = token.clone();
    let polls = Arc::new(AtomicUsize::new(0));

    let canceller = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(20));
        canceller_token.cancel();
    });

    let counted = CountingPoll { inner: CancelWatch { token: &token }, polls: polls.clone() };
    let result = semio_framework_async::block_on(counted);

    canceller.join().unwrap();
    assert!(result.is_err(), "CancelWatch must resolve once the other thread cancels");
    let observed = polls.load(Ordering::SeqCst);
    assert!(observed <= 4, "CancelWatch::poll must be driven by Waker::wake, not busy-spinning; observed {observed} polls waiting out a 20ms cancellation");
}
//#endregion 🔖️AsyncSource

//#region 🔖️Scheduler
#[test]
fn read_scheduler_coalesces_two_overlapping_ranges_into_one_physical_read() {
    let data: Vec<u8> = (0..100u16).map(|value| value as u8).collect();
    let source = RecordingSource::new(data.clone());
    let scheduler = ReadScheduler::new(source);
    let cancel = CancellationToken::new();

    let request_a = ReadRequest { range: ByteRange { offset: 0, len: 50 }, priority: LoadPriority::Requested };
    let request_b = ReadRequest { range: ByteRange { offset: 30, len: 50 }, priority: LoadPriority::Requested };

    let (result_a, result_b) = semio_framework_async::block_on(semio_framework_async::join2(scheduler.read(request_a, &cancel), scheduler.read(request_b, &cancel)));

    assert_eq!(result_a.unwrap(), data[0..50].to_vec());
    assert_eq!(result_b.unwrap(), data[30..80].to_vec());
    assert_eq!(scheduler.source.read_count.load(Ordering::SeqCst), 1, "two overlapping ranges must coalesce into a single physical read");
}

#[test]
fn read_scheduler_dedups_identical_in_flight_requests() {
    let data: Vec<u8> = (0..40u16).map(|value| value as u8).collect();
    let source = RecordingSource::new(data.clone());
    let scheduler = ReadScheduler::new(source);
    let cancel = CancellationToken::new();
    let request = ReadRequest { range: ByteRange { offset: 5, len: 10 }, priority: LoadPriority::Visible };

    let (result_a, result_b) = semio_framework_async::block_on(semio_framework_async::join2(scheduler.read(request, &cancel), scheduler.read(request, &cancel)));

    assert_eq!(result_a.unwrap(), data[5..15].to_vec());
    assert_eq!(result_b.unwrap(), data[5..15].to_vec());
    assert_eq!(scheduler.source.read_count.load(Ordering::SeqCst), 1, "identical requests must dedup to one physical read");
}

#[test]
fn read_scheduler_non_overlapping_requests_stay_separate_physical_reads() {
    let data: Vec<u8> = (0..40u16).map(|value| value as u8).collect();
    let source = RecordingSource::new(data);
    let scheduler = ReadScheduler::new(source);
    let cancel = CancellationToken::new();

    let far_apart_a = ReadRequest { range: ByteRange { offset: 0, len: 4 }, priority: LoadPriority::Visible };
    let far_apart_b = ReadRequest { range: ByteRange { offset: 30, len: 4 }, priority: LoadPriority::Visible };

    let (result_a, result_b) = semio_framework_async::block_on(semio_framework_async::join2(scheduler.read(far_apart_a, &cancel), scheduler.read(far_apart_b, &cancel)));

    assert!(result_a.is_ok());
    assert!(result_b.is_ok());
    assert_eq!(scheduler.source.read_count.load(Ordering::SeqCst), 2, "disjoint ranges must not be coalesced");
}

#[test]
fn cancellation_short_circuits_an_in_flight_read_instead_of_hanging_forever() {
    let scheduler = ReadScheduler::new(HangingSource);
    let cancel = CancellationToken::new();
    let cancel_from_other_thread = cancel.clone();

    let canceller = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(20));
        cancel_from_other_thread.cancel();
    });

    let request = ReadRequest { range: ByteRange { offset: 0, len: 1 }, priority: LoadPriority::Critical };
    let result = semio_framework_async::block_on(scheduler.read(request, &cancel));

    canceller.join().unwrap();
    assert!(result.is_err(), "a cancelled read must short-circuit rather than hang until the source resolves");
}

#[test]
fn cancellation_already_flagged_before_read_returns_immediately() {
    let scheduler = ReadScheduler::new(HangingSource);
    let cancel = CancellationToken::new();
    cancel.cancel();

    let request = ReadRequest { range: ByteRange { offset: 0, len: 1 }, priority: LoadPriority::Background };
    let result = semio_framework_async::block_on(scheduler.read(request, &cancel));
    assert!(result.is_err());
}
//#endregion 🔖️Scheduler

//#region 🔖️Backpressure
/// 🧵️ Polls `future` exactly once with a no-op waker, without consuming it if it is not
/// ready — used to hand-drive several `BoundedDemand::acquire` futures deterministically to
/// prove priority (not arrival order) decides who is granted next.
fn poll_once<F: Future + Unpin>(future: &mut F) -> Poll<F::Output> {
    let waker = Waker::noop();
    let mut cx = Context::from_waker(waker);
    Pin::new(future).poll(&mut cx)
}

#[test]
fn bounded_demand_priority_ordering_under_contention() {
    let demand = BoundedDemand::new(1);

    // Fill the single slot so every subsequent acquire() has to queue.
    let mut held = Box::pin(demand.acquire(LoadPriority::Critical));
    let permit0 = match poll_once(&mut held) {
        Poll::Ready(permit) => permit,
        Poll::Pending => panic!("first acquire on an empty BoundedDemand must be immediate"),
    };

    // Arrival order deliberately reversed from priority order.
    let mut background = Box::pin(demand.acquire(LoadPriority::Background));
    let mut prefetch = Box::pin(demand.acquire(LoadPriority::Prefetch));
    let mut requested = Box::pin(demand.acquire(LoadPriority::Requested));
    let mut visible = Box::pin(demand.acquire(LoadPriority::Visible));

    assert!(matches!(poll_once(&mut background), Poll::Pending));
    assert!(matches!(poll_once(&mut prefetch), Poll::Pending));
    assert!(matches!(poll_once(&mut requested), Poll::Pending));
    assert!(matches!(poll_once(&mut visible), Poll::Pending));

    drop(permit0);
    assert!(matches!(poll_once(&mut background), Poll::Pending));
    assert!(matches!(poll_once(&mut prefetch), Poll::Pending));
    assert!(matches!(poll_once(&mut requested), Poll::Pending));
    let visible_permit = match poll_once(&mut visible) {
        Poll::Ready(permit) => permit,
        Poll::Pending => panic!("Visible must be granted next: it is the highest-priority queued waiter"),
    };

    drop(visible_permit);
    assert!(matches!(poll_once(&mut background), Poll::Pending));
    assert!(matches!(poll_once(&mut prefetch), Poll::Pending));
    let requested_permit = match poll_once(&mut requested) {
        Poll::Ready(permit) => permit,
        Poll::Pending => panic!("Requested must be granted next"),
    };

    drop(requested_permit);
    assert!(matches!(poll_once(&mut background), Poll::Pending));
    let prefetch_permit = match poll_once(&mut prefetch) {
        Poll::Ready(permit) => permit,
        Poll::Pending => panic!("Prefetch must be granted next"),
    };

    drop(prefetch_permit);
    let background_permit = match poll_once(&mut background) {
        Poll::Ready(permit) => permit,
        Poll::Pending => panic!("Background must finally be granted once everything else has drained"),
    };
    drop(background_permit);
}

#[test]
fn bounded_demand_reports_capacity_and_in_flight() {
    let demand = BoundedDemand::new(2);
    assert_eq!(demand.capacity(), 2);
    assert_eq!(demand.in_flight(), 0);
    let permit_a = semio_framework_async::block_on(demand.acquire(LoadPriority::Requested));
    assert_eq!(demand.in_flight(), 1);
    let permit_b = semio_framework_async::block_on(demand.acquire(LoadPriority::Requested));
    assert_eq!(demand.in_flight(), 2);
    drop(permit_a);
    assert_eq!(demand.in_flight(), 1);
    drop(permit_b);
    assert_eq!(demand.in_flight(), 0);
}
//#endregion 🔖️Backpressure
