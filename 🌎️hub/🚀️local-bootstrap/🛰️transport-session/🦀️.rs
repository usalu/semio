//! 🛰️ The inherited transport: opening over the descriptor, hello, signing, request admission, issue and reject responses.

use super::*;

impl InheritedLocalBootstrapTransport {
    /// 🔐️ Opens the launcher's inherited pipe (descriptor 3), completes its hello, and starts the pump.
    pub async fn open_inherited(context: &IdentityVerificationContext<'_>, control: Arc<dyn IdentityVerificationControl>) -> DirectoryResult<Arc<Self>> {
        Self::open_over(inherited_bootstrap_file()?, context, control).await
    }

    pub(super) async fn open_over(file: std::fs::File, context: &IdentityVerificationContext<'_>, control: Arc<dyn IdentityVerificationControl>) -> DirectoryResult<Arc<Self>> {
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
    pub(super) fn cancel_pending(&self, exchange_id: &str) {
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

    pub(super) async fn accept_next(&self, control: &dyn IdentityVerificationControl) -> DirectoryResult<Option<VerifiedLocalBootstrapRequest>> {
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

    pub(super) async fn issue_response(&self, request: &VerifiedLocalBootstrapRequest, session: &IssuedAuthSession, context: &IdentityVerificationContext<'_>) -> DirectoryResult<()> {
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
        context.control.report(semio_hub_directory_contract::IdentityVerificationProgress { completed_units: 2, total_units: 2 });
        result
    }

    pub(super) async fn reject_response(&self, request_id: &str, code: LocalBootstrapRejectCode, context: &IdentityVerificationContext<'_>) -> DirectoryResult<()> {
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
