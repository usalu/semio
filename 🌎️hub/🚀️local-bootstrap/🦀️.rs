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
pub const LOCAL_BOOTSTRAP_REPLAY_MAX: usize = 64;
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

pub struct InheritedLocalBootstrapTransport {
    run_id: String,
    channel_key: SecretChannelKey,
    profiles: Box<[LocalBootstrapProfileWire]>,
    reader: tokio::sync::Mutex<LocalBootstrapIo>,
    writer: tokio::sync::Mutex<LocalBootstrapIo>,
    incoming_sequence: AtomicU64,
    outgoing_sequence: AtomicU64,
    consumed: Mutex<ExchangeSlots<LOCAL_BOOTSTRAP_REPLAY_MAX>>,
    pending: Mutex<ExchangeSlots<LOCAL_BOOTSTRAP_OUTSTANDING_MAX>>,
    cancelled: Mutex<ExchangeSlots<LOCAL_BOOTSTRAP_OUTSTANDING_MAX>>,
    ready: AtomicBool,
    shutdown: AtomicBool,
}

impl InheritedLocalBootstrapTransport {
    pub async fn open_inherited(context: &IdentityVerificationContext<'_>) -> DirectoryResult<Arc<Self>> {
        let file = inherited_bootstrap_file()?;
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
        let transport = Arc::new(Self {
            run_id: initialize.run_id,
            channel_key: SecretChannelKey(channel_key),
            profiles: initialize.profiles,
            reader: tokio::sync::Mutex::new(reader),
            writer: tokio::sync::Mutex::new(writer),
            incoming_sequence: AtomicU64::new(0),
            outgoing_sequence: AtomicU64::new(0),
            consumed: Mutex::new(ExchangeSlots::new()),
            pending: Mutex::new(ExchangeSlots::new()),
            cancelled: Mutex::new(ExchangeSlots::new()),
            ready: AtomicBool::new(false),
            shutdown: AtomicBool::new(false),
        });
        context.checkpoint(1, 4)?;
        transport.accept_hello(context).await?;
        context.checkpoint(4, 4)?;
        Ok(transport)
    }

    async fn accept_hello(&self, context: &IdentityVerificationContext<'_>) -> DirectoryResult<()> {
        let bytes = {
            let mut reader = self.reader.lock().await;
            read_frame(&mut *reader, context).await?.ok_or_else(unavailable)?
        };
        let hello: HelloWire = serde_json::from_slice(&bytes).map_err(|_| denied())?;
        let unsigned =
            HelloUnsigned { schema: &hello.schema, kind: &hello.kind, run_id: &hello.run_id, sequence: hello.sequence, exchange_id: &hello.exchange_id, issued_at: hello.issued_at, expires_at: hello.expires_at, launcher_nonce: &hello.launcher_nonce };
        validate_common(&hello.schema, &hello.kind, "hello", &hello.run_id, &self.run_id, hello.sequence, 1, &hello.exchange_id, hello.issued_at, hello.expires_at, context.control.now_ms())?;
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

    fn consume_request(&self, exchange_id: &str) -> DirectoryResult<()> {
        let mut consumed = self.consumed.lock().map_err(|_| unavailable())?;
        if !consumed.insert(exchange_id) {
            return Err(denied());
        }
        let mut pending = self.pending.lock().map_err(|_| unavailable())?;
        if !pending.insert(exchange_id) {
            return Err(resource_limit());
        }
        Ok(())
    }

    fn finish_request(&self, exchange_id: &str) {
        if let Ok(mut pending) = self.pending.lock() {
            pending.remove(exchange_id);
        }
        if let Ok(mut cancelled) = self.cancelled.lock() {
            cancelled.remove(exchange_id);
        }
    }

    async fn accept_next(&self, control: &dyn IdentityVerificationControl) -> DirectoryResult<Option<VerifiedLocalBootstrapRequest>> {
        let transport_control = TransportControl { base: control, shutdown: &self.shutdown };
        loop {
            if self.shutdown.load(Ordering::Acquire) {
                return Ok(None);
            }
            let (bytes, deadline_ms) = {
                let mut reader = self.reader.lock().await;
                match read_admitted_frame(&mut *reader, &transport_control).await? {
                    Some(frame) => frame,
                    None => {
                        self.shutdown.store(true, Ordering::Release);
                        self.ready.store(false, Ordering::Release);
                        return Ok(None);
                    }
                }
            };
            let context = IdentityVerificationContext { deadline_ms, control: &transport_control };
            context.checkpoint(1, 4)?;
            let kind: KindWire = serde_json::from_slice(&bytes).map_err(|_| denied())?;
            if kind.kind == "issue" {
                let wire: IssueWire = serde_json::from_slice(&bytes).map_err(|_| denied())?;
                let expected_sequence = self.incoming_sequence.load(Ordering::Acquire).checked_add(1).ok_or_else(resource_limit)?;
                validate_common(&wire.schema, &wire.kind, "issue", &wire.run_id, &self.run_id, wire.sequence, expected_sequence, &wire.exchange_id, wire.issued_at, wire.expires_at, context.control.now_ms())?;
                validate_bounded_auth_text(&wire.device_instance_id, "device instance", DEVICE_INSTANCE_MAX_BYTES)?;
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
                self.verify_proof(&unsigned, &wire.proof)?;
                let profile = self.profile(&wire.profile_id, wire.client_class)?;
                self.consume_request(&wire.exchange_id)?;
                self.incoming_sequence.store(wire.sequence, Ordering::Release);
                context.checkpoint(4, 4)?;
                return Ok(Some(VerifiedLocalBootstrapRequest {
                    request_id: wire.exchange_id,
                    run_id: self.run_id.clone(),
                    profile_id: profile.profile_id.clone(),
                    identity_provider: LOCAL_BOOTSTRAP_IDENTITY_PROVIDER.to_string(),
                    identity_subject: profile.subject.clone(),
                    display_name: profile.display_name.clone(),
                    device_instance_id: wire.device_instance_id,
                    client_class: wire.client_class,
                }));
            }
            if matches!(kind.kind.as_str(), "cancel" | "shutdown") {
                let wire: TerminalInputWire = serde_json::from_slice(&bytes).map_err(|_| denied())?;
                let expected_sequence = self.incoming_sequence.load(Ordering::Acquire).checked_add(1).ok_or_else(resource_limit)?;
                validate_common(&wire.schema, &wire.kind, &kind.kind, &wire.run_id, &self.run_id, wire.sequence, expected_sequence, &wire.exchange_id, wire.issued_at, wire.expires_at, context.control.now_ms())?;
                let unsigned = TerminalUnsigned { schema: &wire.schema, kind: &wire.kind, run_id: &wire.run_id, sequence: wire.sequence, exchange_id: &wire.exchange_id, issued_at: wire.issued_at, expires_at: wire.expires_at };
                self.verify_proof(&unsigned, &wire.proof)?;
                self.incoming_sequence.store(wire.sequence, Ordering::Release);
                if kind.kind == "shutdown" {
                    self.shutdown.store(true, Ordering::Release);
                    self.ready.store(false, Ordering::Release);
                    return Ok(None);
                }
                let mut cancelled = self.cancelled.lock().map_err(|_| unavailable())?;
                if !cancelled.contains(&wire.exchange_id) && !cancelled.insert(&wire.exchange_id) {
                    return Err(resource_limit());
                }
                continue;
            }
            self.shutdown.store(true, Ordering::Release);
            self.ready.store(false, Ordering::Release);
            return Err(denied());
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
            if let Ok(mut cancelled) = self.cancelled.lock() {
                if !cancelled.contains(request_id) && !cancelled.insert(request_id) {
                    return Err(resource_limit());
                }
            }
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

pub async fn serve_local_bootstrap(transport: Arc<dyn LocalBootstrapTransport>, directory: Arc<HubDirectories>, control: Arc<dyn IdentityVerificationControl>) -> DirectoryResult<()> {
    let mut tasks = tokio::task::JoinSet::new();
    loop {
        while tasks.len() >= LOCAL_BOOTSTRAP_OUTSTANDING_MAX {
            if let Some(result) = tasks.join_next().await {
                result.map_err(|_| unavailable())??;
            }
        }
        let Some(request) = transport.accept(control.as_ref()).await? else { break };
        let request_transport = transport.clone();
        let request_directory = directory.clone();
        let request_control = Arc::new(RequestControl { base: control.clone(), transport: transport.clone(), request_id: request.request_id.clone() });
        tasks.spawn(async move {
            let deadline_ms = checked_deadline(request_control.now_ms())?;
            let context = IdentityVerificationContext { deadline_ms, control: request_control.as_ref() };
            let result = issue_local_session(request_directory.clone(), &request, &context).await;
            match result {
                Ok(session) => match request_transport.issue(&request, &session, &context).await {
                    Ok(()) => Ok(()),
                    Err(delivery_error) => {
                        request_directory.revoke_auth_session(&session.record.id, "local-bootstrap-delivery-failed", None, &request.request_id).await?.ok_or_else(unavailable)?;
                        Err(delivery_error)
                    }
                },
                Err(_) => {
                    let code = if request_control.is_cancelled() { LocalBootstrapRejectCode::Cancelled } else { LocalBootstrapRejectCode::Unavailable };
                    request_transport.reject(&request.request_id, code, &context).await
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

fn validate_common(schema: &str, kind: &str, expected_kind: &str, run_id: &str, expected_run_id: &str, sequence: u64, expected_sequence: u64, exchange_id: &str, issued_at: i64, expires_at: i64, now: i64) -> DirectoryResult<()> {
    if schema != LOCAL_BOOTSTRAP_SCHEMA || kind != expected_kind || run_id != expected_run_id || sequence != expected_sequence {
        return Err(denied());
    }
    decode_hex::<16>(run_id)?;
    decode_hex::<16>(exchange_id)?;
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
mod tests {
    use super::*;
    use serde_json::Value;
    use std::sync::atomic::{AtomicI64, AtomicU8};

    struct CancelAtCommit {
        cancelled: AtomicBool,
        now_ms: i64,
    }

    struct AdmissionClock {
        now_ms: AtomicI64,
        admitted_units: AtomicU8,
        cancelled: AtomicBool,
    }

    impl IdentityVerificationControl for AdmissionClock {
        fn now_ms(&self) -> i64 {
            self.now_ms.load(Ordering::Acquire)
        }

        fn is_cancelled(&self) -> bool {
            self.cancelled.load(Ordering::Acquire)
        }

        fn report(&self, progress: crate::directory::IdentityVerificationProgress) {
            self.admitted_units.store(progress.completed_units, Ordering::Release);
        }
    }

    impl IdentityVerificationControl for CancelAtCommit {
        fn now_ms(&self) -> i64 {
            self.now_ms
        }

        fn is_cancelled(&self) -> bool {
            self.cancelled.load(Ordering::Acquire)
        }

        fn report(&self, progress: crate::directory::IdentityVerificationProgress) {
            if progress.completed_units == progress.total_units {
                self.cancelled.store(true, Ordering::Release);
            }
        }
    }

    fn fixture() -> Value {
        serde_json::from_str(include_str!("🧪️fixtures/🚇️pipe-v1/🔣️.json")).expect("fixture")
    }

    fn admission_fixture() -> Value {
        serde_json::from_str(include_str!("🧪️fixtures/⏳️idle-admission-v1/🔣️.json")).expect("admission fixture")
    }

    #[tokio::test]
    async fn local_bootstrap_idle_listener_survives_until_admission_and_admitted_frame_is_deadline_bounded() {
        let fixture = admission_fixture();
        let exchange_deadline_ms = fixture["exchangeDeadlineMs"].as_i64().expect("exchange deadline");
        let idle_before_admission_ms = fixture["idleBeforeAdmissionMs"].as_i64().expect("idle duration");
        assert_eq!(exchange_deadline_ms, LOCAL_BOOTSTRAP_EXCHANGE_DEADLINE_MS);
        assert!(idle_before_admission_ms > exchange_deadline_ms);
        let frame = decode_hex_bytes(fixture["frameHex"].as_str().expect("frame"));
        let payload = decode_hex_bytes(fixture["payloadHex"].as_str().expect("payload"));

        let clock = Arc::new(AdmissionClock { now_ms: AtomicI64::new(0), admitted_units: AtomicU8::new(0), cancelled: AtomicBool::new(false) });
        let (mut reader, mut writer) = tokio::io::duplex(LOCAL_BOOTSTRAP_FRAME_BYTES_MAX);
        let read_clock = clock.clone();
        let read = tokio::spawn(async move { read_admitted_frame(&mut reader, read_clock.as_ref()).await });
        tokio::task::yield_now().await;
        clock.now_ms.store(idle_before_admission_ms, Ordering::Release);
        writer.write_all(&frame).await.expect("write admitted frame");
        let (actual, deadline_ms) = read.await.expect("idle reader task").expect("idle reader result").expect("admitted frame");
        assert_eq!(&*actual, payload.as_ref());
        assert_eq!(deadline_ms, idle_before_admission_ms + exchange_deadline_ms);
        assert_eq!(clock.admitted_units.load(Ordering::Acquire), 2);

        let clock = Arc::new(AdmissionClock { now_ms: AtomicI64::new(0), admitted_units: AtomicU8::new(0), cancelled: AtomicBool::new(false) });
        let (mut reader, mut writer) = tokio::io::duplex(LOCAL_BOOTSTRAP_FRAME_BYTES_MAX);
        let read_clock = clock.clone();
        let read = tokio::spawn(async move { read_admitted_frame(&mut reader, read_clock.as_ref()).await });
        writer.write_all(&frame[..1]).await.expect("write admission byte");
        tokio::time::timeout(std::time::Duration::from_secs(1), async {
            while clock.admitted_units.load(Ordering::Acquire) == 0 {
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("admission progress");
        clock.now_ms.store(exchange_deadline_ms + 1, Ordering::Release);
        writer.write_all(&frame[1..4]).await.expect("write late prefix");
        let error = read.await.expect("bounded reader task").expect_err("late admitted frame");
        assert!(matches!(error, DirectoryError::Conflict(message) if message.contains("deadline")));

        let clock = Arc::new(AdmissionClock { now_ms: AtomicI64::new(0), admitted_units: AtomicU8::new(0), cancelled: AtomicBool::new(false) });
        let (mut reader, _writer) = tokio::io::duplex(LOCAL_BOOTSTRAP_FRAME_BYTES_MAX);
        let read_clock = clock.clone();
        let read = tokio::spawn(async move { read_admitted_frame(&mut reader, read_clock.as_ref()).await });
        clock.cancelled.store(true, Ordering::Release);
        let error = tokio::time::timeout(std::time::Duration::from_secs(1), read)
            .await
            .expect("idle cancellation deadline")
            .expect("idle cancellation task")
            .expect_err("idle cancellation result");
        assert!(matches!(error, DirectoryError::Conflict(message) if message.contains("cancellation")));
    }

    fn decode_hex_bytes(encoded: &str) -> Box<[u8]> {
        assert_eq!(encoded.len() % 2, 0);
        encoded
            .as_bytes()
            .chunks_exact(2)
            .map(|pair| hex_nibble(pair[0]) << 4 | hex_nibble(pair[1]))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    #[test]
    fn local_bootstrap_hmac_matches_neutral_node_oracle_and_rejects_boundaries() {
        let fixture = fixture();
        let key = decode_hex::<32>(fixture["channelKey"].as_str().expect("key")).expect("key bytes");
        let hello: HelloWire = serde_json::from_value(fixture["hello"].clone()).expect("hello fixture");
        let hello_unsigned =
            HelloUnsigned { schema: &hello.schema, kind: &hello.kind, run_id: &hello.run_id, sequence: hello.sequence, exchange_id: &hello.exchange_id, issued_at: hello.issued_at, expires_at: hello.expires_at, launcher_nonce: &hello.launcher_nonce };
        assert_eq!(hex_lower(&hmac_sha256(&key, &serde_json::to_vec(&hello_unsigned).expect("hello canonical"))), hello.proof);
        let issue: IssueWire = serde_json::from_value(fixture["issue"].clone()).expect("issue fixture");
        let issue_unsigned = IssueUnsigned {
            schema: &issue.schema,
            kind: &issue.kind,
            run_id: &issue.run_id,
            sequence: issue.sequence,
            exchange_id: &issue.exchange_id,
            issued_at: issue.issued_at,
            expires_at: issue.expires_at,
            profile_id: &issue.profile_id,
            device_instance_id: &issue.device_instance_id,
            client_class: issue.client_class,
        };
        assert_eq!(hex_lower(&hmac_sha256(&key, &serde_json::to_vec(&issue_unsigned).expect("issue canonical"))), issue.proof);
        let credential = &fixture["credential"];
        let credential_unsigned = CredentialUnsigned {
            schema: credential["schema"].as_str().expect("credential schema"),
            run_id: credential["runId"].as_str().expect("credential run"),
            sequence: credential["sequence"].as_u64().expect("credential sequence"),
            exchange_id: credential["exchangeId"].as_str().expect("credential exchange"),
            issued_at: credential["issuedAt"].as_i64().expect("credential issued"),
            expires_at: credential["expiresAt"].as_i64().expect("credential expires"),
            profile_id: credential["profileId"].as_str().expect("credential profile"),
            client_class: serde_json::from_value(credential["clientClass"].clone()).expect("credential class"),
            session_id: credential["sessionId"].as_str().expect("credential session"),
            session_kind: serde_json::from_value(credential["sessionKind"].clone()).expect("credential session kind"),
            authorization_generation: credential["authorizationGeneration"].as_u64().expect("credential generation"),
            capability: credential["capability"].as_str().expect("credential capability"),
        };
        assert_eq!(hex_lower(&hmac_sha256(&key, &serde_json::to_vec(&credential_unsigned).expect("credential canonical"))), credential["proof"]);
        assert_ne!(fixture["hostile"]["wrongProof"], fixture["hello"]["proof"]);
        assert_eq!(LOCAL_BOOTSTRAP_FRAME_BYTES_MAX, fixture["limits"]["frameBytesMax"].as_u64().expect("frame cap") as usize);
        assert_eq!(LOCAL_BOOTSTRAP_OUTSTANDING_MAX, fixture["limits"]["outstandingRequestsMax"].as_u64().expect("request cap") as usize);
        assert_eq!(LOCAL_BOOTSTRAP_PROFILES_MAX, fixture["limits"]["profilesMax"].as_u64().expect("profile cap") as usize);
        assert_eq!(LOCAL_BOOTSTRAP_SESSION_TTL_SECS, fixture["limits"]["clientTtlSeconds"].as_i64().expect("ttl"));
        assert!(decode_hex::<32>(fixture["hostile"]["wrongProof"].as_str().expect("wrong proof")).is_ok());
        assert!(decode_hex::<16>("AA112233445566778899aabbccddeeff").is_err());
        assert!(validate_identifier(&"a".repeat(64)).is_ok());
        assert!(validate_identifier(&"a".repeat(65)).is_err());
        assert!(validate_bounded_auth_text(&"d".repeat(DEVICE_INSTANCE_MAX_BYTES), "device instance", DEVICE_INSTANCE_MAX_BYTES).is_ok());
        assert!(validate_bounded_auth_text(&"d".repeat(DEVICE_INSTANCE_MAX_BYTES + 1), "device instance", DEVICE_INSTANCE_MAX_BYTES).is_err());
        let run_id = fixture["hello"]["runId"].as_str().expect("run");
        let exchange_id = fixture["hello"]["exchangeId"].as_str().expect("exchange");
        assert!(validate_common(LOCAL_BOOTSTRAP_SCHEMA, "issue", "issue", run_id, run_id, 2, 2, exchange_id, 1_000, 16_000, 16_000).is_ok());
        assert!(validate_common(LOCAL_BOOTSTRAP_SCHEMA, "issue", "issue", run_id, run_id, 2, 2, exchange_id, 1_000, 16_001, 16_000).is_err());
        assert!(validate_common(LOCAL_BOOTSTRAP_SCHEMA, "issue", "issue", run_id, run_id, 1, 2, exchange_id, 1_000, 16_000, 2_000).is_err());
        assert!(validate_common(LOCAL_BOOTSTRAP_SCHEMA, "issue", "issue", run_id, run_id, 2, 2, exchange_id, 1_000, 16_000, 16_001).is_err());
        let mut slots = ExchangeSlots::<LOCAL_BOOTSTRAP_OUTSTANDING_MAX>::new();
        for index in 0..LOCAL_BOOTSTRAP_OUTSTANDING_MAX {
            assert!(slots.insert(&format!("{index:032x}")));
        }
        assert!(!slots.insert("ffffffffffffffffffffffffffffffff"));
        assert!(!slots.insert("00000000000000000000000000000000"));
        assert!(slots.remove("00000000000000000000000000000000"));
        assert!(slots.insert("ffffffffffffffffffffffffffffffff"));
    }

    #[cfg(feature = "sqlite")]
    #[tokio::test]
    async fn local_session_commit_survives_cancellation_observed_by_final_progress() {
        let sqlite = crate::directory::sqlite::SqliteDirectory::connect(":memory:").await.expect("sqlite");
        let directory = Arc::new(HubDirectories::from(sqlite));
        directory.create_user("local-commit@bootstrap.invalid", "Local Commit", None, Some("commit-subject"), Some(LOCAL_BOOTSTRAP_IDENTITY_PROVIDER)).await.expect("profile user");
        let control = CancelAtCommit { cancelled: AtomicBool::new(false), now_ms: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("clock").as_millis() as i64 };
        let context = IdentityVerificationContext { deadline_ms: checked_deadline(control.now_ms()).expect("deadline"), control: &control };
        let request = VerifiedLocalBootstrapRequest {
            request_id: "00112233445566778899aabbccddeeff".into(),
            run_id: "ffeeddccbbaa99887766554433221100".into(),
            profile_id: "developer".into(),
            identity_provider: LOCAL_BOOTSTRAP_IDENTITY_PROVIDER.into(),
            identity_subject: "commit-subject".into(),
            display_name: "Local Commit".into(),
            device_instance_id: "native-commit-device".into(),
            client_class: LocalBootstrapClientClass::Native,
        };

        let issued = issue_local_session(directory.clone(), &request, &context).await.expect("committed session");

        assert!(control.is_cancelled());
        assert!(directory.authenticate_session(&issued.capability).await.expect("authenticate").is_some());
    }
}
