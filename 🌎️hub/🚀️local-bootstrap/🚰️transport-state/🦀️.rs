//! 🚰️ The inherited transport state: the secret channel key, the closed signal, the pump control and the frame pump.

use super::*;

pub(super) struct SecretChannelKey(pub(super) [u8; 32]);

impl Drop for SecretChannelKey {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}

/// 📥️ One frame the pump read off the pipe, or the failure that ended the pipe.
type LocalBootstrapInbound = DirectoryResult<Box<[u8]>>;

pub struct InheritedLocalBootstrapTransport {
    pub run_id: String,
    pub(super) channel_key: SecretChannelKey,
    pub(super) profiles: Box<[LocalBootstrapProfileWire]>,
    pub(super) inbound: tokio::sync::Mutex<tokio::sync::mpsc::Receiver<LocalBootstrapInbound>>,
    pub(super) writer: tokio::sync::Mutex<LocalBootstrapIo>,
    pub(super) incoming_sequence: AtomicU64,
    pub(super) outgoing_sequence: AtomicU64,
    pub(super) consumed: Mutex<ExchangeWindow<LOCAL_BOOTSTRAP_REPLAY_WINDOW_MAX>>,
    pub(super) pending: Mutex<ExchangeSlots<LOCAL_BOOTSTRAP_OUTSTANDING_MAX>>,
    pub(super) cancelled: Mutex<ExchangeSlots<LOCAL_BOOTSTRAP_OUTSTANDING_MAX>>,
    pub(super) ready: AtomicBool,
    pub shutdown: Arc<AtomicBool>,
    pub closed: Arc<LocalBootstrapClosed>,
}

/// 🚪️ Raised once the pipe ended — the launcher closed it, or a frame broke it — whatever phase the
/// hub is in, so a hub still loading its catalog stops with its launcher instead of outliving it.
#[derive(Default)]
pub(super) struct LocalBootstrapClosed {
    raised: AtomicBool,
    notify: tokio::sync::Notify,
}

impl LocalBootstrapClosed {
    fn raise(&self) {
        self.raised.store(true, Ordering::Release);
        self.notify.notify_waiters();
    }

    pub(super) async fn wait(&self) {
        loop {
            let notified = self.notify.notified();
            tokio::pin!(notified);
            notified.as_mut().enable();
            if self.raised.load(Ordering::Acquire) {
                return;
            }
            notified.await;
        }
    }
}

/// 🧭️ The pump's clock and cancellation: the hub's own bootstrap control, or the transport's shutdown.
pub(super) struct PumpControl {
    pub(super) base: Arc<dyn IdentityVerificationControl>,
    pub(super) shutdown: Arc<AtomicBool>,
}

impl IdentityVerificationControl for PumpControl {
    fn now_ms(&self) -> i64 {
        self.base.now_ms()
    }

    fn is_cancelled(&self) -> bool {
        self.base.is_cancelled() || self.shutdown.load(Ordering::Acquire)
    }

    fn report(&self, _progress: semio_hub_directory_contract::IdentityVerificationProgress) {}
}

/// 📥️ Owns the pipe's read half for the whole run: every admitted frame goes to `accept`, bounded by
/// the outstanding-request limit, and the end of the pipe — EOF or a broken frame — raises `closed`.
pub(super) async fn pump_local_bootstrap_frames(mut reader: LocalBootstrapIo, frames: tokio::sync::mpsc::Sender<LocalBootstrapInbound>, control: PumpControl, closed: Arc<LocalBootstrapClosed>) {
    loop {
        match read_admitted_frame(&mut reader, &control).await {
            Ok(Some((frame, _))) => {
                if frames.send(Ok(frame)).await.is_err() {
                    break;
                }
            }
            Ok(None) => break,
            Err(error) => {
                if !control.is_cancelled() {
                    let _ = frames.send(Err(error)).await;
                }
                break;
            }
        }
    }
    closed.raise();
}
