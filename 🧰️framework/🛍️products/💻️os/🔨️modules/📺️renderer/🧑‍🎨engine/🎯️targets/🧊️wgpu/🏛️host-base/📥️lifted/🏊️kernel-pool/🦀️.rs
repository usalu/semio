//! 🏊️ One kernel-pool task: a boxed future driven turn by turn on a worker-pool lane, rescheduled by its waker.
//!
//! 🧭️ Lifted out of the kernel pool into host-base: the renderer root spawns application tasks with it before any kernel crate exists.
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Waker};

pub struct KernelPoolFuture {
    pub pool: semio_framework_async::WorkerPool,
    pub lane: semio_framework_async::Lane,
    future: Mutex<Option<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>>,
    scheduled: std::sync::atomic::AtomicBool,
    notified: std::sync::atomic::AtomicBool,
}

impl KernelPoolFuture {
    pub fn spawn(pool: semio_framework_async::WorkerPool, lane: semio_framework_async::Lane, future: impl Future<Output = ()> + Send + 'static) -> Arc<Self> {
        let task = Arc::new(Self { pool, lane, future: Mutex::new(Some(Box::pin(crate::drive_renderer_worker_task(future)))), scheduled: std::sync::atomic::AtomicBool::new(false), notified: std::sync::atomic::AtomicBool::new(true) });
        task.schedule();
        task
    }

    fn schedule(self: &Arc<Self>) {
        self.notified.store(true, std::sync::atomic::Ordering::Release);
        if self.scheduled.swap(true, std::sync::atomic::Ordering::AcqRel) {
            return;
        }
        let task = self.clone();
        self.pool.submit(self.lane, Box::new(move || task.run_turn()));
    }

    fn run_turn(self: Arc<Self>) {
        self.notified.store(false, std::sync::atomic::Ordering::Release);
        let future = self.future.lock().expect("kernel pool future lock").take();
        if let Some(mut future) = future {
            let waker = Waker::from(self.clone());
            let mut context = Context::from_waker(&waker);
            if future.as_mut().poll(&mut context).is_pending() {
                *self.future.lock().expect("kernel pool future lock") = Some(future);
            }
        }
        self.scheduled.store(false, std::sync::atomic::Ordering::Release);
        if self.notified.load(std::sync::atomic::Ordering::Acquire) && self.future.lock().expect("kernel pool future lock").is_some() {
            self.schedule();
        }
    }
}

impl std::task::Wake for KernelPoolFuture {
    fn wake(self: Arc<Self>) {
        self.schedule();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.schedule();
    }
}
