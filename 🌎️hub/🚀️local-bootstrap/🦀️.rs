//! 🔐️ First-party, inherited-handle local bootstrap transport and bounded issuance service.

use crate::directory::error::{DirectoryError, DirectoryResult};
use crate::directory::model::{AuthSessionIssue, AuthSessionKind, IssuedAuthSession};
use crate::directory::{
    constant_time_digest_eq, identity_subject_digest, validate_bounded_auth_text, HubDirectories, HubDirectory, IdentityVerificationContext, IdentityVerificationControl, LocalBootstrapAcceptFuture, LocalBootstrapClientClass,
    LocalBootstrapIssueFuture, LocalBootstrapRejectCode, LocalBootstrapTerminalFuture, LocalBootstrapTransport, VerifiedLocalBootstrapRequest, AUTH_TEXT_MAX_BYTES, DEVICE_INSTANCE_MAX_BYTES,
};
use semio_framework_hash::{hex_lower, Sha256};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub const LOCAL_BOOTSTRAP_SCHEMA: &str = "semio.hub.local-bootstrap/v1";
pub const LOCAL_CREDENTIAL_SCHEMA: &str = "semio.hub.local-credential-envelope/v1";
pub const LOCAL_BOOTSTRAP_IDENTITY_PROVIDER: &str = "semio.local.bootstrap/v1";
pub const LOCAL_BOOTSTRAP_FRAME_BYTES_MAX: usize = 16 * 1024;
pub const LOCAL_BOOTSTRAP_OUTSTANDING_MAX: usize = 8;
pub const LOCAL_BOOTSTRAP_PROFILES_MAX: usize = 8;
/// ⏳️ Exchange ids one validity window (`LOCAL_BOOTSTRAP_EXCHANGE_DEADLINE_MS`) admits. An id leaves
/// the window once its own `expiresAt` has passed, so a long-lived launcher may exchange for ever at
/// up to this many exchanges per 15 s; a burst above it is answered `resource-limit`, never an exit.
pub const LOCAL_BOOTSTRAP_REPLAY_WINDOW_MAX: usize = 64;
pub const LOCAL_BOOTSTRAP_EXCHANGE_DEADLINE_MS: i64 = 15_000;
pub const LOCAL_BOOTSTRAP_SESSION_TTL_SECS: i64 = 15 * 60;
const LOCAL_BOOTSTRAP_HMAC_DOMAIN: &[u8] = b"semio/hub/local-bootstrap/v1\0";
const LOCAL_BOOTSTRAP_IDLE_POLL_MS: u64 = 100;
const INHERITED_BOOTSTRAP_DESCRIPTOR: i32 = 3;

#[cfg(unix)]
type LocalBootstrapIo = tokio::net::UnixStream;
#[cfg(windows)]
type LocalBootstrapIo = tokio::fs::File;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct LocalBootstrapProfileWire {
    profile_id: String,
    subject: String,
    display_name: String,
    allowed_client_classes: Box<[LocalBootstrapClientClass]>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct InitializeWire {
    schema: String,
    kind: String,
    run_id: String,
    channel_key: String,
    profiles: Box<[LocalBootstrapProfileWire]>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct KindWire {
    kind: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct HelloWire {
    schema: String,
    kind: String,
    run_id: String,
    sequence: u64,
    exchange_id: String,
    issued_at: i64,
    expires_at: i64,
    launcher_nonce: String,
    proof: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HelloUnsigned<'a> {
    schema: &'a str,
    kind: &'a str,
    run_id: &'a str,
    sequence: u64,
    exchange_id: &'a str,
    issued_at: i64,
    expires_at: i64,
    launcher_nonce: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HelloAcceptedUnsigned<'a> {
    schema: &'a str,
    kind: &'a str,
    run_id: &'a str,
    sequence: u64,
    exchange_id: &'a str,
    issued_at: i64,
    expires_at: i64,
    hub_nonce: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HelloAcceptedWire<'a> {
    schema: &'a str,
    kind: &'a str,
    run_id: &'a str,
    sequence: u64,
    exchange_id: &'a str,
    issued_at: i64,
    expires_at: i64,
    hub_nonce: &'a str,
    proof: &'a str,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct IssueWire {
    schema: String,
    kind: String,
    run_id: String,
    sequence: u64,
    exchange_id: String,
    issued_at: i64,
    expires_at: i64,
    profile_id: String,
    device_instance_id: String,
    client_class: LocalBootstrapClientClass,
    proof: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct IssueUnsigned<'a> {
    schema: &'a str,
    kind: &'a str,
    run_id: &'a str,
    sequence: u64,
    exchange_id: &'a str,
    issued_at: i64,
    expires_at: i64,
    profile_id: &'a str,
    device_instance_id: &'a str,
    client_class: LocalBootstrapClientClass,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct TerminalInputWire {
    schema: String,
    kind: String,
    run_id: String,
    sequence: u64,
    exchange_id: String,
    issued_at: i64,
    expires_at: i64,
    proof: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TerminalUnsigned<'a> {
    schema: &'a str,
    kind: &'a str,
    run_id: &'a str,
    sequence: u64,
    exchange_id: &'a str,
    issued_at: i64,
    expires_at: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RejectUnsigned<'a> {
    schema: &'a str,
    kind: &'a str,
    run_id: &'a str,
    sequence: u64,
    exchange_id: &'a str,
    issued_at: i64,
    expires_at: i64,
    code: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RejectWire<'a> {
    schema: &'a str,
    kind: &'a str,
    run_id: &'a str,
    sequence: u64,
    exchange_id: &'a str,
    issued_at: i64,
    expires_at: i64,
    code: &'a str,
    proof: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CredentialUnsigned<'a> {
    schema: &'a str,
    run_id: &'a str,
    sequence: u64,
    exchange_id: &'a str,
    issued_at: i64,
    expires_at: i64,
    profile_id: &'a str,
    client_class: LocalBootstrapClientClass,
    session_id: &'a str,
    session_kind: AuthSessionKind,
    authorization_generation: u64,
    capability: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CredentialWire<'a> {
    schema: &'a str,
    run_id: &'a str,
    sequence: u64,
    exchange_id: &'a str,
    issued_at: i64,
    expires_at: i64,
    profile_id: &'a str,
    client_class: LocalBootstrapClientClass,
    session_id: &'a str,
    session_kind: AuthSessionKind,
    authorization_generation: u64,
    capability: &'a str,
    proof: &'a str,
}

struct SecretChannelKey([u8; 32]);

impl Drop for SecretChannelKey {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}

/// ⏳️ Exchange ids admitted inside their own validity window. An id whose `expiresAt` is past can
/// never validate again — every frame carrying it is refused by its own window — so it is evicted
/// instead of pinning a slot for the rest of the run.
struct ExchangeWindow<const N: usize> {
    values: [Option<(String, i64)>; N],
}

impl<const N: usize> ExchangeWindow<N> {
    fn new() -> Self {
        Self { values: std::array::from_fn(|_| None) }
    }

    fn admit(&mut self, value: &str, expires_at: i64, now: i64) -> Result<(), LocalBootstrapRejectCode> {
        for slot in &mut self.values {
            if slot.as_ref().is_some_and(|(_, expiry)| *expiry < now) {
                *slot = None;
            }
        }
        if self.values.iter().flatten().any(|(existing, _)| existing == value) {
            return Err(LocalBootstrapRejectCode::Denied);
        }
        let slot = self.values.iter_mut().find(|slot| slot.is_none()).ok_or(LocalBootstrapRejectCode::ResourceLimit)?;
        *slot = Some((value.to_owned(), expires_at));
        Ok(())
    }
}

struct ExchangeSlots<const N: usize> {
    values: [Option<String>; N],
}

impl<const N: usize> ExchangeSlots<N> {
    fn new() -> Self {
        Self { values: std::array::from_fn(|_| None) }
    }

    fn contains(&self, value: &str) -> bool {
        self.values.iter().flatten().any(|existing| existing == value)
    }

    fn insert(&mut self, value: &str) -> bool {
        if self.contains(value) {
            return false;
        }
        let Some(slot) = self.values.iter_mut().find(|slot| slot.is_none()) else { return false };
        *slot = Some(value.to_string());
        true
    }

    fn remove(&mut self, value: &str) -> bool {
        let Some(slot) = self.values.iter_mut().find(|slot| slot.as_deref() == Some(value)) else { return false };
        *slot = None;
        true
    }
}

/// 📥️ One frame the pump read off the pipe, or the failure that ended the pipe.
type LocalBootstrapInbound = DirectoryResult<Box<[u8]>>;

pub struct InheritedLocalBootstrapTransport {
    run_id: String,
    channel_key: SecretChannelKey,
    profiles: Box<[LocalBootstrapProfileWire]>,
    inbound: tokio::sync::Mutex<tokio::sync::mpsc::Receiver<LocalBootstrapInbound>>,
    writer: tokio::sync::Mutex<LocalBootstrapIo>,
    incoming_sequence: AtomicU64,
    outgoing_sequence: AtomicU64,
    consumed: Mutex<ExchangeWindow<LOCAL_BOOTSTRAP_REPLAY_WINDOW_MAX>>,
    pending: Mutex<ExchangeSlots<LOCAL_BOOTSTRAP_OUTSTANDING_MAX>>,
    cancelled: Mutex<ExchangeSlots<LOCAL_BOOTSTRAP_OUTSTANDING_MAX>>,
    ready: AtomicBool,
    shutdown: Arc<AtomicBool>,
    closed: Arc<LocalBootstrapClosed>,
}

/// 🚪️ Raised once the pipe ended — the launcher closed it, or a frame broke it — whatever phase the
/// hub is in, so a hub still loading its catalog stops with its launcher instead of outliving it.
#[derive(Default)]
struct LocalBootstrapClosed {
    raised: AtomicBool,
    notify: tokio::sync::Notify,
}

impl LocalBootstrapClosed {
    fn raise(&self) {
        self.raised.store(true, Ordering::Release);
        self.notify.notify_waiters();
    }

    async fn wait(&self) {
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
struct PumpControl {
    base: Arc<dyn IdentityVerificationControl>,
    shutdown: Arc<AtomicBool>,
}

impl IdentityVerificationControl for PumpControl {
    fn now_ms(&self) -> i64 {
        self.base.now_ms()
    }

    fn is_cancelled(&self) -> bool {
        self.base.is_cancelled() || self.shutdown.load(Ordering::Acquire)
    }

    fn report(&self, _progress: crate::directory::IdentityVerificationProgress) {}
}

/// 📥️ Owns the pipe's read half for the whole run: every admitted frame goes to `accept`, bounded by
/// the outstanding-request limit, and the end of the pipe — EOF or a broken frame — raises `closed`.
async fn pump_local_bootstrap_frames(mut reader: LocalBootstrapIo, frames: tokio::sync::mpsc::Sender<LocalBootstrapInbound>, control: PumpControl, closed: Arc<LocalBootstrapClosed>) {
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

impl InheritedLocalBootstrapTransport {
    /// 🔐️ Opens the launcher's inherited pipe (descriptor 3), completes its hello, and starts the pump.
    pub async fn open_inherited(context: &IdentityVerificationContext<'_>, control: Arc<dyn IdentityVerificationControl>) -> DirectoryResult<Arc<Self>> {
        Self::open_over(inherited_bootstrap_file()?, context, control).await
    }

    async fn open_over(file: std::fs::File, context: &IdentityVerificationContext<'_>, control: Arc<dyn IdentityVerificationControl>) -> DirectoryResult<Arc<Self>> {
        let writer = file.try_clone().map_err(|_| unavailable())?;
        let mut reader = into_async_bootstrap_io(file)?;
        let writer = into_async_bootstrap_io(writer)?;
        context.checkpoint(0, 4)?;
        let mut initialize_bytes = read_frame(&mut reader, context).await?.ok_or_else(unavailable)?;
        let mut initialize: InitializeWire = serde_json::from_slice(&initialize_bytes).map_err(|_| denied())?;
        initialize_bytes.fill(0);
        if let Err(error) = validate_initialize(&initialize) {
            std::mem::take(&mut initialize.channel_key).into_bytes().fill(0);
            return Err(error);
        }
        let channel_key = decode_hex::<32>(&initialize.channel_key);
        std::mem::take(&mut initialize.channel_key).into_bytes().fill(0);
        let channel_key = channel_key?;
        let (frames, inbound) = tokio::sync::mpsc::channel(LOCAL_BOOTSTRAP_OUTSTANDING_MAX);
        let transport = Arc::new(Self {
            run_id: initialize.run_id,
            channel_key: SecretChannelKey(channel_key),
            profiles: initialize.profiles,
            inbound: tokio::sync::Mutex::new(inbound),
            writer: tokio::sync::Mutex::new(writer),
            incoming_sequence: AtomicU64::new(0),
            outgoing_sequence: AtomicU64::new(0),
            consumed: Mutex::new(ExchangeWindow::new()),
            pending: Mutex::new(ExchangeSlots::new()),
            cancelled: Mutex::new(ExchangeSlots::new()),
            ready: AtomicBool::new(false),
            shutdown: Arc::new(AtomicBool::new(false)),
            closed: Arc::new(LocalBootstrapClosed::default()),
        });
        context.checkpoint(1, 4)?;
        transport.accept_hello(&mut reader, context).await?;
        tokio::spawn(pump_local_bootstrap_frames(reader, frames, PumpControl { base: control, shutdown: transport.shutdown.clone() }, transport.closed.clone()));
        context.checkpoint(4, 4)?;
        Ok(transport)
    }

    async fn accept_hello(&self, reader: &mut LocalBootstrapIo, context: &IdentityVerificationContext<'_>) -> DirectoryResult<()> {
        let bytes = read_frame(reader, context).await?.ok_or_else(unavailable)?;
        let hello: HelloWire = serde_json::from_slice(&bytes).map_err(|_| denied())?;
        let unsigned =
            HelloUnsigned { schema: &hello.schema, kind: &hello.kind, run_id: &hello.run_id, sequence: hello.sequence, exchange_id: &hello.exchange_id, issued_at: hello.issued_at, expires_at: hello.expires_at, launcher_nonce: &hello.launcher_nonce };
        validate_frame(&hello.schema, &hello.kind, "hello", &hello.run_id, &self.run_id, hello.sequence, 1, &hello.exchange_id)?;
        validate_window(hello.issued_at, hello.expires_at, context.control.now_ms())?;
        decode_hex::<32>(&hello.launcher_nonce)?;
        self.verify_proof(&unsigned, &hello.proof)?;
        self.incoming_sequence.store(hello.sequence, Ordering::Release);
        context.checkpoint(2, 4)?;
        let mut nonce = [0u8; 32];
        directory::os_identity::fill_entropy(&mut nonce).map_err(|_| unavailable())?;
        let hub_nonce = hex_lower(&nonce);
        nonce.fill(0);
        let now = context.control.now_ms();
        let expires_at = checked_deadline(now)?;
        let sequence = self.next_outgoing_sequence()?;
        let unsigned = HelloAcceptedUnsigned { schema: LOCAL_BOOTSTRAP_SCHEMA, kind: "hello-accepted", run_id: &self.run_id, sequence, exchange_id: &hello.exchange_id, issued_at: now, expires_at, hub_nonce: &hub_nonce };
        let proof = self.sign(&unsigned)?;
        let wire = HelloAcceptedWire { schema: unsigned.schema, kind: unsigned.kind, run_id: unsigned.run_id, sequence, exchange_id: unsigned.exchange_id, issued_at: now, expires_at, hub_nonce: &hub_nonce, proof: &proof };
        self.write(&wire, context).await?;
        self.ready.store(true, Ordering::Release);
        context.checkpoint(3, 4)?;
        Ok(())
    }

    fn sign<T: Serialize>(&self, unsigned: &T) -> DirectoryResult<String> {
        let mut canonical = serde_json::to_vec(unsigned).map_err(|_| unavailable())?;
        if canonical.len().checked_add(4).is_none_or(|length| length > LOCAL_BOOTSTRAP_FRAME_BYTES_MAX) {
            canonical.fill(0);
            return Err(resource_limit());
        }
        let proof = hex_lower(&hmac_sha256(&self.channel_key.0, &canonical));
        canonical.fill(0);
        Ok(proof)
    }

    fn verify_proof<T: Serialize>(&self, unsigned: &T, encoded: &str) -> DirectoryResult<()> {
        let expected = hmac_sha256(&self.channel_key.0, &serde_json::to_vec(unsigned).map_err(|_| unavailable())?);
        let actual = decode_hex::<32>(encoded)?;
        if !constant_time_digest_eq(&expected, &actual) {
            return Err(denied());
        }
        Ok(())
    }

    fn next_outgoing_sequence(&self) -> DirectoryResult<u64> {
        self.outgoing_sequence.try_update(Ordering::AcqRel, Ordering::Acquire, |value| value.checked_add(1)).map(|previous| previous + 1).map_err(|_| resource_limit())
    }

    async fn write<T: Serialize>(&self, value: &T, context: &IdentityVerificationContext<'_>) -> DirectoryResult<()> {
        let mut bytes = serde_json::to_vec(value).map_err(|_| unavailable())?;
        let mut writer = self.writer.lock().await;
        let result = write_frame(&mut *writer, &bytes, context).await;
        bytes.fill(0);
        result
    }

    fn profile(&self, profile_id: &str, client_class: LocalBootstrapClientClass) -> DirectoryResult<&LocalBootstrapProfileWire> {
        let profile = self.profiles.iter().find(|profile| profile.profile_id == profile_id).ok_or_else(denied)?;
        if !profile.allowed_client_classes.contains(&client_class) {
            return Err(denied());
        }
        Ok(profile)
    }

    /// 🧾️ The per-request admission of one authentic, in-sequence issue frame. Every refusal here is
    /// the launcher's answer (`reject` with its code); none of them ends the pipe or the hub.
    fn admit_issue(&self, wire: &IssueWire, now: i64) -> Result<&LocalBootstrapProfileWire, LocalBootstrapRejectCode> {
        validate_window(wire.issued_at, wire.expires_at, now).map_err(|_| LocalBootstrapRejectCode::Expired)?;
        validate_bounded_auth_text(&wire.device_instance_id, "device instance", DEVICE_INSTANCE_MAX_BYTES).map_err(|_| LocalBootstrapRejectCode::Denied)?;
        let profile = self.profile(&wire.profile_id, wire.client_class).map_err(|_| LocalBootstrapRejectCode::Denied)?;
        self.consumed.lock().map_err(|_| LocalBootstrapRejectCode::Unavailable)?.admit(&wire.exchange_id, wire.expires_at, now)?;
        if !self.pending.lock().map_err(|_| LocalBootstrapRejectCode::Unavailable)?.insert(&wire.exchange_id) {
            return Err(LocalBootstrapRejectCode::ResourceLimit);
        }
        Ok(profile)
    }

    fn finish_request(&self, exchange_id: &str) {
        if let Ok(mut pending) = self.pending.lock() {
            pending.remove(exchange_id);
        }
        if let Ok(mut cancelled) = self.cancelled.lock() {
            cancelled.remove(exchange_id);
        }
    }

    /// 🚫️ Marks a still-pending exchange cancelled; an exchange that already finished has nothing to
    /// cancel, so the cancelled set never outgrows the pending one.
    fn cancel_pending(&self, exchange_id: &str) {
        let Ok(pending) = self.pending.lock() else { return };
        if !pending.contains(exchange_id) {
            return;
        }
        if let Ok(mut cancelled) = self.cancelled.lock() {
            cancelled.insert(exchange_id);
        }
    }

    /// 💥️ A frame that is not an authentic, in-sequence frame of this run: the pipe itself is broken,
    /// so it stops — the only way a pipe refusal ends the service.
    fn broken(&self) -> DirectoryError {
        self.shutdown.store(true, Ordering::Release);
        self.ready.store(false, Ordering::Release);
        denied()
    }

    async fn accept_next(&self, control: &dyn IdentityVerificationControl) -> DirectoryResult<Option<VerifiedLocalBootstrapRequest>> {
        let transport_control = TransportControl { base: control, shutdown: &self.shutdown };
        let mut inbound = self.inbound.lock().await;
        loop {
            if transport_control.is_cancelled() {
                return Ok(None);
            }
            let bytes = match tokio::time::timeout(std::time::Duration::from_millis(LOCAL_BOOTSTRAP_IDLE_POLL_MS), inbound.recv()).await {
                Err(_) => continue,
                Ok(None) => {
                    self.shutdown.store(true, Ordering::Release);
                    self.ready.store(false, Ordering::Release);
                    return Ok(None);
                }
                Ok(Some(Err(error))) => {
                    self.broken();
                    return Err(error);
                }
                Ok(Some(Ok(bytes))) => bytes,
            };
            let now = control.now_ms();
            let context = IdentityVerificationContext { deadline_ms: checked_deadline(now)?, control: &transport_control };
            let kind: KindWire = serde_json::from_slice(&bytes).map_err(|_| self.broken())?;
            let expected_sequence = self.incoming_sequence.load(Ordering::Acquire).checked_add(1).ok_or_else(resource_limit)?;
            if kind.kind == "issue" {
                let wire: IssueWire = serde_json::from_slice(&bytes).map_err(|_| self.broken())?;
                let unsigned = IssueUnsigned {
                    schema: &wire.schema,
                    kind: &wire.kind,
                    run_id: &wire.run_id,
                    sequence: wire.sequence,
                    exchange_id: &wire.exchange_id,
                    issued_at: wire.issued_at,
                    expires_at: wire.expires_at,
                    profile_id: &wire.profile_id,
                    device_instance_id: &wire.device_instance_id,
                    client_class: wire.client_class,
                };
                validate_frame(&wire.schema, &wire.kind, "issue", &wire.run_id, &self.run_id, wire.sequence, expected_sequence, &wire.exchange_id).map_err(|_| self.broken())?;
                self.verify_proof(&unsigned, &wire.proof).map_err(|_| self.broken())?;
                self.incoming_sequence.store(wire.sequence, Ordering::Release);
                match self.admit_issue(&wire, now) {
                    Ok(profile) => {
                        context.checkpoint(4, 4)?;
                        return Ok(Some(VerifiedLocalBootstrapRequest {
                            request_id: wire.exchange_id.clone(),
                            run_id: self.run_id.clone(),
                            profile_id: profile.profile_id.clone(),
                            identity_provider: LOCAL_BOOTSTRAP_IDENTITY_PROVIDER.to_string(),
                            identity_subject: profile.subject.clone(),
                            display_name: profile.display_name.clone(),
                            device_instance_id: wire.device_instance_id,
                            client_class: wire.client_class,
                        }));
                    }
                    Err(code) => {
                        self.reject_response(&wire.exchange_id, code, &context).await?;
                        continue;
                    }
                }
            }
            if matches!(kind.kind.as_str(), "cancel" | "shutdown") {
                let wire: TerminalInputWire = serde_json::from_slice(&bytes).map_err(|_| self.broken())?;
                let unsigned = TerminalUnsigned { schema: &wire.schema, kind: &wire.kind, run_id: &wire.run_id, sequence: wire.sequence, exchange_id: &wire.exchange_id, issued_at: wire.issued_at, expires_at: wire.expires_at };
                validate_frame(&wire.schema, &wire.kind, &kind.kind, &wire.run_id, &self.run_id, wire.sequence, expected_sequence, &wire.exchange_id).map_err(|_| self.broken())?;
                self.verify_proof(&unsigned, &wire.proof).map_err(|_| self.broken())?;
                self.incoming_sequence.store(wire.sequence, Ordering::Release);
                if validate_window(wire.issued_at, wire.expires_at, now).is_err() {
                    continue;
                }
                if kind.kind == "shutdown" {
                    self.shutdown.store(true, Ordering::Release);
                    self.ready.store(false, Ordering::Release);
                    return Ok(None);
                }
                self.cancel_pending(&wire.exchange_id);
                continue;
            }
            return Err(self.broken());
        }
    }

    async fn issue_response(&self, request: &VerifiedLocalBootstrapRequest, session: &IssuedAuthSession, context: &IdentityVerificationContext<'_>) -> DirectoryResult<()> {
        context.checkpoint(0, 2)?;
        if self.request_cancelled(&request.request_id) {
            self.finish_request(&request.request_id);
            return Err(DirectoryError::Conflict("local bootstrap request cancelled".into()));
        }
        let sequence = self.next_outgoing_sequence()?;
        let capability = session.capability.expose_once();
        let unsigned = CredentialUnsigned {
            schema: LOCAL_CREDENTIAL_SCHEMA,
            run_id: &self.run_id,
            sequence,
            exchange_id: &request.request_id,
            issued_at: session.record.issued_at,
            expires_at: session.record.expires_at,
            profile_id: &request.profile_id,
            client_class: request.client_class,
            session_id: &session.record.id,
            session_kind: session.record.session_kind,
            authorization_generation: session.record.authorization_generation,
            capability: &capability,
        };
        let proof = match self.sign(&unsigned) {
            Ok(proof) => proof,
            Err(error) => {
                drop(unsigned);
                capability.into_bytes().fill(0);
                return Err(error);
            }
        };
        let wire = CredentialWire {
            schema: unsigned.schema,
            run_id: unsigned.run_id,
            sequence,
            exchange_id: unsigned.exchange_id,
            issued_at: unsigned.issued_at,
            expires_at: unsigned.expires_at,
            profile_id: unsigned.profile_id,
            client_class: unsigned.client_class,
            session_id: unsigned.session_id,
            session_kind: unsigned.session_kind,
            authorization_generation: unsigned.authorization_generation,
            capability: &capability,
            proof: &proof,
        };
        let result = self.write(&wire, context).await;
        self.finish_request(&request.request_id);
        capability.into_bytes().fill(0);
        context.control.report(crate::directory::IdentityVerificationProgress { completed_units: 2, total_units: 2 });
        result
    }

    async fn reject_response(&self, request_id: &str, code: LocalBootstrapRejectCode, context: &IdentityVerificationContext<'_>) -> DirectoryResult<()> {
        if decode_hex::<16>(request_id).is_err() {
            return Err(denied());
        }
        let now = context.control.now_ms();
        let expires_at = checked_deadline(now)?;
        let sequence = self.next_outgoing_sequence()?;
        let unsigned = RejectUnsigned { schema: LOCAL_BOOTSTRAP_SCHEMA, kind: "reject", run_id: &self.run_id, sequence, exchange_id: request_id, issued_at: now, expires_at, code: code.as_str() };
        let proof = self.sign(&unsigned)?;
        let wire = RejectWire { schema: unsigned.schema, kind: unsigned.kind, run_id: unsigned.run_id, sequence, exchange_id: unsigned.exchange_id, issued_at: unsigned.issued_at, expires_at: unsigned.expires_at, code: unsigned.code, proof: &proof };
        let result = self.write(&wire, context).await;
        self.finish_request(request_id);
        result
    }
}

impl LocalBootstrapTransport for InheritedLocalBootstrapTransport {
    fn run_id(&self) -> &str {
        &self.run_id
    }

    fn is_ready(&self) -> bool {
        self.ready.load(Ordering::Acquire) && !self.shutdown.load(Ordering::Acquire)
    }

    fn request_cancelled(&self, request_id: &str) -> bool {
        self.shutdown.load(Ordering::Acquire) || self.cancelled.lock().map_or(true, |cancelled| cancelled.contains(request_id))
    }

    fn accept<'a>(&'a self, control: &'a dyn IdentityVerificationControl) -> LocalBootstrapAcceptFuture<'a> {
        Box::pin(async move { self.accept_next(control).await })
    }

    fn issue<'a>(&'a self, request: &'a VerifiedLocalBootstrapRequest, session: &'a IssuedAuthSession, context: &'a IdentityVerificationContext<'a>) -> LocalBootstrapIssueFuture<'a> {
        Box::pin(async move { self.issue_response(request, session, context).await })
    }

    fn reject<'a>(&'a self, request_id: &'a str, code: LocalBootstrapRejectCode, context: &'a IdentityVerificationContext<'a>) -> LocalBootstrapTerminalFuture<'a> {
        Box::pin(async move { self.reject_response(request_id, code, context).await })
    }

    fn cancel<'a>(&'a self, request_id: &'a str) -> LocalBootstrapTerminalFuture<'a> {
        Box::pin(async move {
            self.cancel_pending(request_id);
            Ok(())
        })
    }

    fn closed<'a>(&'a self) -> LocalBootstrapTerminalFuture<'a> {
        Box::pin(async move {
            self.closed.wait().await;
            Ok(())
        })
    }

    fn shutdown<'a>(&'a self) -> LocalBootstrapTerminalFuture<'a> {
        Box::pin(async move {
            self.shutdown.store(true, Ordering::Release);
            self.ready.store(false, Ordering::Release);
            Ok(())
        })
    }
}

struct RequestControl {
    base: Arc<dyn IdentityVerificationControl>,
    transport: Arc<dyn LocalBootstrapTransport>,
    request_id: String,
}

struct TransportControl<'a> {
    base: &'a dyn IdentityVerificationControl,
    shutdown: &'a AtomicBool,
}

impl IdentityVerificationControl for TransportControl<'_> {
    fn now_ms(&self) -> i64 {
        self.base.now_ms()
    }

    fn is_cancelled(&self) -> bool {
        self.base.is_cancelled() || self.shutdown.load(Ordering::Acquire)
    }

    fn report(&self, progress: crate::directory::IdentityVerificationProgress) {
        self.base.report(progress);
    }
}

impl IdentityVerificationControl for RequestControl {
    fn now_ms(&self) -> i64 {
        self.base.now_ms()
    }

    fn is_cancelled(&self) -> bool {
        self.base.is_cancelled() || self.transport.request_cancelled(&self.request_id)
    }

    fn report(&self, progress: crate::directory::IdentityVerificationProgress) {
        self.base.report(progress);
    }
}

/// 🎫️ Serves the pipe until the launcher ends it. A request's own failure is answered on the pipe
/// (`reject`) or retired (an undelivered session is revoked); it never ends the service. Only the end
/// of the pipe, a broken frame, or a panicking request task does.
pub async fn serve_local_bootstrap(transport: Arc<dyn LocalBootstrapTransport>, directory: Arc<HubDirectories>, control: Arc<dyn IdentityVerificationControl>) -> DirectoryResult<()> {
    let mut tasks = tokio::task::JoinSet::new();
    loop {
        while tasks.len() >= LOCAL_BOOTSTRAP_OUTSTANDING_MAX {
            if let Some(Err(join_error)) = tasks.join_next().await {
                if join_error.is_panic() {
                    return Err(unavailable());
                }
            }
        }
        let Some(request) = transport.accept(control.as_ref()).await? else { break };
        let request_transport = transport.clone();
        let request_directory = directory.clone();
        let request_control = Arc::new(RequestControl { base: control.clone(), transport: transport.clone(), request_id: request.request_id.clone() });
        tasks.spawn(async move {
            let Ok(deadline_ms) = checked_deadline(request_control.now_ms()) else { return };
            let context = IdentityVerificationContext { deadline_ms, control: request_control.as_ref() };
            match issue_local_session(request_directory.clone(), &request, &context).await {
                Ok(session) => {
                    if request_transport.issue(&request, &session, &context).await.is_err() {
                        let _ = request_directory.revoke_auth_session(&session.record.id, "local-bootstrap-delivery-failed", None, &request.request_id).await;
                    }
                }
                Err(_) => {
                    let code = if request_control.is_cancelled() { LocalBootstrapRejectCode::Cancelled } else { LocalBootstrapRejectCode::Unavailable };
                    let _ = request_transport.reject(&request.request_id, code, &context).await;
                }
            }
        });
    }
    transport.shutdown().await?;
    while let Some(result) = tasks.join_next().await {
        if let Err(join_error) = result {
            if !join_error.is_cancelled() {
                return Err(unavailable());
            }
        }
    }
    Ok(())
}

async fn issue_local_session(directory: Arc<HubDirectories>, request: &VerifiedLocalBootstrapRequest, context: &IdentityVerificationContext<'_>) -> DirectoryResult<IssuedAuthSession> {
    context.checkpoint(0, 4)?;
    let subject_digest = identity_subject_digest(&request.identity_provider, &request.identity_subject)?;
    let user = match directory.get_user_by_sso_subject(&request.identity_provider, &request.identity_subject).await? {
        Some(user) => user,
        None => {
            context.checkpoint(1, 4)?;
            let email = format!("local-{}@bootstrap.invalid", &hex_lower(&subject_digest)[..16]);
            let user = directory.create_user(&email, &request.display_name, None, Some(&request.identity_subject), Some(&request.identity_provider)).await?;
            user
        }
    };
    context.checkpoint(2, 4)?;
    let issue = AuthSessionIssue {
        user_id: user.id,
        identity_provider: request.identity_provider.clone(),
        identity_subject_digest: subject_digest,
        ttl_secs: LOCAL_BOOTSTRAP_SESSION_TTL_SECS,
        device_instance_id: request.device_instance_id.clone(),
        session_kind: AuthSessionKind::DevelopmentLocal,
        correlation_id: request.request_id.clone(),
        peer_class: request.client_class.as_str().to_string(),
    };
    let session = directory.issue_auth_session(&issue).await?;
    context.control.report(crate::directory::IdentityVerificationProgress { completed_units: 4, total_units: 4 });
    Ok(session)
}

fn validate_initialize(initialize: &InitializeWire) -> DirectoryResult<()> {
    if initialize.schema != LOCAL_BOOTSTRAP_SCHEMA || initialize.kind != "initialize" {
        return Err(denied());
    }
    decode_hex::<16>(&initialize.run_id)?;
    decode_hex::<32>(&initialize.channel_key)?;
    if initialize.profiles.is_empty() || initialize.profiles.len() > LOCAL_BOOTSTRAP_PROFILES_MAX {
        return Err(resource_limit());
    }
    for (profile_index, profile) in initialize.profiles.iter().enumerate() {
        validate_identifier(&profile.profile_id)?;
        validate_bounded_auth_text(&profile.subject, "local bootstrap subject", AUTH_TEXT_MAX_BYTES)?;
        validate_bounded_auth_text(&profile.display_name, "local bootstrap display name", AUTH_TEXT_MAX_BYTES)?;
        if profile.allowed_client_classes.is_empty() || profile.allowed_client_classes.len() > 4 {
            return Err(resource_limit());
        }
        if initialize.profiles[..profile_index].iter().any(|existing| existing.profile_id == profile.profile_id)
            || profile.allowed_client_classes.iter().enumerate().any(|(class_index, class)| profile.allowed_client_classes[..class_index].contains(class))
        {
            return Err(denied());
        }
    }
    Ok(())
}

/// 🔏️ The frame belongs to this run, at the next sequence, with well-formed ids — or the pipe is broken.
fn validate_frame(schema: &str, kind: &str, expected_kind: &str, run_id: &str, expected_run_id: &str, sequence: u64, expected_sequence: u64, exchange_id: &str) -> DirectoryResult<()> {
    if schema != LOCAL_BOOTSTRAP_SCHEMA || kind != expected_kind || run_id != expected_run_id || sequence != expected_sequence {
        return Err(denied());
    }
    decode_hex::<16>(run_id)?;
    decode_hex::<16>(exchange_id)?;
    Ok(())
}

/// ⏱️ The exchange's own validity window holds `now` and is no longer than one exchange deadline.
fn validate_window(issued_at: i64, expires_at: i64, now: i64) -> DirectoryResult<()> {
    if issued_at < 0 || expires_at <= issued_at || expires_at.saturating_sub(issued_at) > LOCAL_BOOTSTRAP_EXCHANGE_DEADLINE_MS || now < issued_at.saturating_sub(1_000) || now > expires_at {
        return Err(DirectoryError::Unauthorized);
    }
    Ok(())
}

fn validate_identifier(value: &str) -> DirectoryResult<()> {
    if value.is_empty() || value.len() > 64 || !value.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'-')) {
        return Err(denied());
    }
    Ok(())
}

fn hmac_sha256(key: &[u8; 32], canonical: &[u8]) -> [u8; 32] {
    let mut inner_pad = [0x36; 64];
    let mut outer_pad = [0x5c; 64];
    for index in 0..32 {
        inner_pad[index] ^= key[index];
        outer_pad[index] ^= key[index];
    }
    let mut inner = Sha256::new();
    inner.update(&inner_pad);
    inner.update(LOCAL_BOOTSTRAP_HMAC_DOMAIN);
    inner.update(&(canonical.len() as u32).to_be_bytes());
    inner.update(canonical);
    let inner_digest = inner.finalize();
    let mut outer = Sha256::new();
    outer.update(&outer_pad);
    outer.update(&inner_digest);
    inner_pad.fill(0);
    outer_pad.fill(0);
    outer.finalize()
}

fn decode_hex<const N: usize>(encoded: &str) -> DirectoryResult<[u8; N]> {
    if encoded.len() != N * 2 || !encoded.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) {
        return Err(denied());
    }
    let mut bytes = [0; N];
    for (index, output) in bytes.iter_mut().enumerate() {
        let high = hex_nibble(encoded.as_bytes()[index * 2]);
        let low = hex_nibble(encoded.as_bytes()[index * 2 + 1]);
        *output = high << 4 | low;
    }
    Ok(bytes)
}

fn hex_nibble(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'a'..=b'f' => byte - b'a' + 10,
        _ => 0,
    }
}

fn checked_deadline(now: i64) -> DirectoryResult<i64> {
    now.checked_add(LOCAL_BOOTSTRAP_EXCHANGE_DEADLINE_MS).ok_or_else(resource_limit)
}

fn remaining(context: &IdentityVerificationContext<'_>) -> DirectoryResult<std::time::Duration> {
    let remaining = context.deadline_ms.checked_sub(context.control.now_ms()).ok_or_else(|| DirectoryError::Conflict("local bootstrap deadline exceeded".into()))?;
    if remaining <= 0 || context.control.is_cancelled() {
        return Err(DirectoryError::Conflict("local bootstrap deadline or cancellation reached".into()));
    }
    Ok(std::time::Duration::from_millis(remaining as u64))
}

async fn read_frame(reader: &mut (impl tokio::io::AsyncRead + Unpin), context: &IdentityVerificationContext<'_>) -> DirectoryResult<Option<Box<[u8]>>> {
    let mut prefix = [0u8; 4];
    match tokio::time::timeout(remaining(context)?, reader.read_exact(&mut prefix)).await {
        Ok(Ok(_)) => {}
        Ok(Err(error)) if error.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
        Ok(Err(_)) => return Err(unavailable()),
        Err(_) => return Err(DirectoryError::Conflict("local bootstrap read deadline exceeded".into())),
    }
    let length = u32::from_be_bytes(prefix) as usize;
    if length == 0 || length.checked_add(4).is_none_or(|complete| complete > LOCAL_BOOTSTRAP_FRAME_BYTES_MAX) {
        return Err(resource_limit());
    }
    let mut bytes = vec![0; length].into_boxed_slice();
    match tokio::time::timeout(remaining(context)?, reader.read_exact(&mut bytes)).await {
        Ok(Ok(_)) => Ok(Some(bytes)),
        Ok(Err(_)) => Err(denied()),
        Err(_) => Err(DirectoryError::Conflict("local bootstrap frame deadline exceeded".into())),
    }
}

async fn read_admitted_frame(reader: &mut (impl tokio::io::AsyncRead + Unpin), control: &dyn IdentityVerificationControl) -> DirectoryResult<Option<(Box<[u8]>, i64)>> {
    let mut prefix = [0u8; 4];
    loop {
        if control.is_cancelled() {
            return Err(DirectoryError::Conflict("local bootstrap cancellation reached while idle".into()));
        }
        match tokio::time::timeout(std::time::Duration::from_millis(LOCAL_BOOTSTRAP_IDLE_POLL_MS), reader.read_exact(&mut prefix[..1])).await {
            Ok(Ok(_)) => break,
            Ok(Err(error)) if error.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
            Ok(Err(_)) => return Err(unavailable()),
            Err(_) => continue,
        }
    }
    let deadline_ms = checked_deadline(control.now_ms())?;
    let context = IdentityVerificationContext { deadline_ms, control };
    control.report(crate::directory::IdentityVerificationProgress { completed_units: 1, total_units: 2 });
    match tokio::time::timeout(remaining(&context)?, reader.read_exact(&mut prefix[1..])).await {
        Ok(Ok(_)) => {}
        Ok(Err(_)) => return Err(denied()),
        Err(_) => return Err(DirectoryError::Conflict("local bootstrap frame deadline exceeded".into())),
    }
    let length = u32::from_be_bytes(prefix) as usize;
    if length == 0 || length.checked_add(4).is_none_or(|complete| complete > LOCAL_BOOTSTRAP_FRAME_BYTES_MAX) {
        return Err(resource_limit());
    }
    let mut bytes = vec![0; length].into_boxed_slice();
    match tokio::time::timeout(remaining(&context)?, reader.read_exact(&mut bytes)).await {
        Ok(Ok(_)) => {
            control.report(crate::directory::IdentityVerificationProgress { completed_units: 2, total_units: 2 });
            Ok(Some((bytes, deadline_ms)))
        }
        Ok(Err(_)) => Err(denied()),
        Err(_) => Err(DirectoryError::Conflict("local bootstrap frame deadline exceeded".into())),
    }
}

async fn write_frame(writer: &mut (impl tokio::io::AsyncWrite + Unpin), bytes: &[u8], context: &IdentityVerificationContext<'_>) -> DirectoryResult<()> {
    if bytes.is_empty() || bytes.len().checked_add(4).is_none_or(|complete| complete > LOCAL_BOOTSTRAP_FRAME_BYTES_MAX) {
        return Err(resource_limit());
    }
    let length = u32::try_from(bytes.len()).map_err(|_| resource_limit())?.to_be_bytes();
    tokio::time::timeout(remaining(context)?, async {
        writer.write_all(&length).await?;
        writer.write_all(bytes).await?;
        writer.flush().await
    })
    .await
    .map_err(|_| DirectoryError::Conflict("local bootstrap write deadline exceeded".into()))?
    .map_err(|_| unavailable())
}

#[cfg(unix)]
fn inherited_bootstrap_file() -> DirectoryResult<std::fs::File> {
    use std::os::fd::FromRawFd;
    unsafe extern "C" {
        fn fcntl(fd: i32, command: i32, ...) -> i32;
    }
    if unsafe { fcntl(INHERITED_BOOTSTRAP_DESCRIPTOR, 1) } < 0 {
        return Err(unavailable());
    }
    Ok(unsafe { std::fs::File::from_raw_fd(INHERITED_BOOTSTRAP_DESCRIPTOR) })
}

#[cfg(unix)]
fn into_async_bootstrap_io(file: std::fs::File) -> DirectoryResult<LocalBootstrapIo> {
    use std::os::fd::{FromRawFd, IntoRawFd};
    let stream = unsafe { std::os::unix::net::UnixStream::from_raw_fd(file.into_raw_fd()) };
    stream.set_nonblocking(true).map_err(|_| unavailable())?;
    tokio::net::UnixStream::from_std(stream).map_err(|_| unavailable())
}

#[cfg(windows)]
fn inherited_bootstrap_file() -> DirectoryResult<std::fs::File> {
    use std::os::windows::io::FromRawHandle;
    unsafe extern "C" {
        fn _get_osfhandle(fd: i32) -> isize;
    }
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetCurrentProcess() -> *mut std::ffi::c_void;
        fn DuplicateHandle(source_process: *mut std::ffi::c_void, source_handle: *mut std::ffi::c_void, target_process: *mut std::ffi::c_void, target_handle: *mut *mut std::ffi::c_void, desired_access: u32, inherit_handle: i32, options: u32) -> i32;
    }
    let handle = unsafe { _get_osfhandle(INHERITED_BOOTSTRAP_DESCRIPTOR) };
    if handle == -1 {
        return Err(unavailable());
    }
    let process = unsafe { GetCurrentProcess() };
    let mut duplicate = std::ptr::null_mut();
    let duplicated = unsafe { DuplicateHandle(process, handle as *mut std::ffi::c_void, process, &mut duplicate, 0, 0, 0x0000_0002) };
    if duplicated == 0 || duplicate.is_null() {
        return Err(unavailable());
    }
    Ok(unsafe { std::fs::File::from_raw_handle(duplicate) })
}

#[cfg(windows)]
fn into_async_bootstrap_io(file: std::fs::File) -> DirectoryResult<LocalBootstrapIo> {
    Ok(tokio::fs::File::from_std(file))
}

fn denied() -> DirectoryError {
    DirectoryError::Unauthorized
}

fn resource_limit() -> DirectoryError {
    DirectoryError::Conflict("local bootstrap resource limit exceeded".into())
}

fn unavailable() -> DirectoryError {
    DirectoryError::Backend("local bootstrap transport unavailable".into())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
