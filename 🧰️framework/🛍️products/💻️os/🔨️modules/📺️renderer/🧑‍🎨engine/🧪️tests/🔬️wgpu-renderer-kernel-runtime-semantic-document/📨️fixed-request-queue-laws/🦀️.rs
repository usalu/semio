/// 📨️ Fixed request queue laws: capacity plus one ownership, FIFO order, the aggregate page credit, contention and the admission that never parks without a wake.

fn command_request(instance: u32, generation: u64, page_count: usize) -> KernelRequest {
    let mut pages = semio_framework::kernel::CommandPageSet::try_new(page_count).unwrap();
    for index in 0..page_count {
        let page = if index + 1 == page_count {
            semio_framework::kernel::FixedCommandPage::try_copy_from(b"tail").unwrap()
        } else {
            semio_framework::kernel::FixedCommandPage::try_copy_from(&[3; semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES]).unwrap()
        };
        pages.try_push(page).unwrap();
    }
    let command = semio_framework::kernel::PagedCommand::try_from_pages(pages).unwrap();
    let mut commands = semio_framework::kernel::CommandEnvelopeSet::try_new().unwrap();
    commands.try_push(semio_framework::kernel::CommandEnvelope { instance, seq: generation, command }).unwrap();
    let batch = semio_framework::kernel::CommandBatch::try_new(generation, commands).unwrap();
    KernelRequest::ExchangeCommands { instance, driver: semio_framework::kernel::CommandBatchDriver::new(generation, batch) }
}

#[test]
fn fixed_kernel_request_queue_returns_capacity_plus_one_owner_and_preserves_fifo() {
    let queue = KernelRequestQueue::default();
    for instance in 0..KERNEL_REQUEST_QUEUE_CAPACITY as u32 {
        queue.try_push(destroy_request(instance), Arc::new(ResponseSlot::default()), None).unwrap_or_else(|_| panic!("fixture request queue admission"));
    }
    let (rejected, _) = queue.try_push(destroy_request(999), Arc::new(ResponseSlot::default()), None).unwrap_err();
    assert!(matches!(rejected, KernelRequest::DestroyApp { ref owner } if owner.instance == 999));
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    for expected in 0..KERNEL_REQUEST_QUEUE_CAPACITY as u32 {
        let Poll::Ready((request, _)) = queue.poll(&mut context) else { panic!("fixed request was ready") };
        assert!(matches!(request, KernelRequest::DestroyApp { ref owner } if owner.instance == expected));
    }
}

#[test]
fn fixed_kernel_request_queue_rejects_aggregate_page_credit_plus_one_exactly() {
    let queue = KernelRequestQueue::default();
    queue.try_push(command_request(1, 7, semio_framework::kernel::COMMAND_MAXIMUM_PAGES), Arc::new(ResponseSlot::default()), None).unwrap_or_else(|_| panic!("fixture request queue admission"));
    let (rejected, _) = queue.try_push(command_request(2, 8, 1), Arc::new(ResponseSlot::default()), None).unwrap_err();
    assert!(matches!(rejected, KernelRequest::ExchangeCommands { instance: 2, ref driver } if driver.remaining_pages() == 1));
}

#[test]
fn fixed_kernel_request_queue_contention_returns_the_untouched_owner() {
    let queue = KernelRequestQueue::default();
    let guard = queue.state.lock().unwrap();
    let (rejected, _) = queue.try_push(command_request(7, 11, 1), Arc::new(ResponseSlot::default()), None).unwrap_err();
    assert!(matches!(rejected, KernelRequest::ExchangeCommands { instance: 7, ref driver } if driver.generation() == 11 && driver.remaining_pages() == 1));
    drop(guard);
}

/// 📥️ A producer whose kernel request meets a blocked lane — the queue's lock held for a moment, or
/// the queue full — is never parked without a wake: driven only by the wakes the lane itself
/// arranges, exactly the fixture's count of producers is admitted after each release. tokio's own
/// blocked lanes (a held `sync::Mutex`, a full bounded `mpsc` channel) are the oracle, under the same
/// wake-only executor. A detached render parked on one contended push and hung a whole 180 s law
/// (ticket 26/09/23 slice WG8, session 12).
#[test]
fn kernel_request_admission_never_parks_without_a_wake() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct CountingWake(AtomicUsize);

    impl std::task::Wake for CountingWake {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }

        fn wake_by_ref(self: &Arc<Self>) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    struct Producer<'a> {
        future: Pin<Box<dyn Future<Output = ()> + 'a>>,
        wake: Arc<CountingWake>,
        seen: usize,
        admitted: bool,
    }

    impl<'a> Producer<'a> {
        fn new(future: impl Future<Output = ()> + 'a) -> Self {
            Self { future: Box::pin(future), wake: Arc::new(CountingWake(AtomicUsize::new(0))), seen: 0, admitted: false }
        }

        fn poll(&mut self) {
            self.seen = self.wake.0.load(Ordering::SeqCst);
            let waker = Waker::from(self.wake.clone());
            self.admitted = self.future.as_mut().poll(&mut Context::from_waker(&waker)).is_ready();
        }
    }

    fn admitted_per_release(producers: &mut [Producer<'_>], releases: usize, mut release: impl FnMut()) -> Vec<usize> {
        producers.iter_mut().for_each(Producer::poll);
        assert!(producers.iter().all(|producer| !producer.admitted), "a blocked lane admits nothing");
        (0..releases)
            .map(|_| {
                release();
                while let Some(producer) = producers.iter_mut().find(|producer| !producer.admitted && producer.wake.0.load(Ordering::SeqCst) != producer.seen) {
                    producer.poll();
                }
                producers.iter().filter(|producer| producer.admitted).count()
            })
            .collect()
    }

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧵️kernel-pool-future/🔣️.json")).expect("neutral kernel admission traces");
    for case in fixture["admissions"].as_array().expect("admission cases") {
        let id = case["id"].as_str().unwrap();
        let producers = case["producers"].as_u64().unwrap() as u32;
        let expected: Vec<usize> = case["admittedAfterRelease"].as_array().unwrap().iter().map(|count| count.as_u64().unwrap() as usize).collect();
        let (oracle, actual) = match case["blockedBy"].as_str().unwrap() {
            "contended" => {
                let mutex = tokio::sync::Mutex::new(());
                let mut held = Some(mutex.try_lock().expect("oracle lock"));
                let mut lane: Vec<Producer<'_>> = (0..producers).map(|_| Producer::new(async { drop(mutex.lock().await) })).collect();
                let oracle = admitted_per_release(&mut lane, expected.len(), || drop(held.take()));
                let queue = KernelRequestQueue::default();
                let mut held = Some(queue.state.lock().unwrap());
                let mut lane: Vec<Producer<'_>> = (0..producers).map(|instance| Producer::new(queue.enqueue_retained(destroy_request(instance), Arc::new(ResponseSlot::default())))).collect();
                let actual = admitted_per_release(&mut lane, expected.len(), || drop(held.take()));
                (oracle, actual)
            }
            "full" => {
                let (sender, mut receiver) = tokio::sync::mpsc::channel::<u32>(1);
                sender.try_send(u32::MAX).expect("oracle lane fills");
                let mut lane: Vec<Producer<'_>> = (0..producers)
                    .map(|value| {
                        let sender = sender.clone();
                        Producer::new(async move { sender.send(value).await.expect("oracle receiver remains live") })
                    })
                    .collect();
                let oracle = admitted_per_release(&mut lane, expected.len(), || drop(receiver.try_recv().expect("oracle lane holds a value")));
                let queue = KernelRequestQueue::default();
                for instance in 0..KERNEL_REQUEST_QUEUE_CAPACITY as u32 {
                    queue.try_push(destroy_request(1000 + instance), Arc::new(ResponseSlot::default()), None).unwrap_or_else(|_| panic!("fixture request queue admission"));
                }
                let mut lane: Vec<Producer<'_>> = (0..producers).map(|instance| Producer::new(queue.enqueue_retained(destroy_request(instance), Arc::new(ResponseSlot::default())))).collect();
                let actual = admitted_per_release(&mut lane, expected.len(), || drop(queue.try_next().expect("the full lane holds a request")));
                (oracle, actual)
            }
            other => panic!("unknown neutral blocked lane {other}"),
        };
        assert_eq!(oracle, expected, "{id}: the tokio oracle admits the fixture's counts");
        assert_eq!(actual, expected, "{id}: the kernel request queue admits the fixture's counts");
    }
}
