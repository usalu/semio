//! 🎫️ The bounded issuance service: serving the pipe, issuing local sessions, validating initialize and frames and the error vocabulary.

use super::*;

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

pub(super) async fn issue_local_session(directory: Arc<HubDirectories>, request: &VerifiedLocalBootstrapRequest, context: &IdentityVerificationContext<'_>) -> DirectoryResult<IssuedAuthSession> {
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
    context.control.report(semio_hub_directory_contract::IdentityVerificationProgress { completed_units: 4, total_units: 4 });
    Ok(session)
}

pub(super) fn validate_initialize(initialize: &InitializeWire) -> DirectoryResult<()> {
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
pub(super) fn validate_frame(schema: &str, kind: &str, expected_kind: &str, run_id: &str, expected_run_id: &str, sequence: u64, expected_sequence: u64, exchange_id: &str) -> DirectoryResult<()> {
    if schema != LOCAL_BOOTSTRAP_SCHEMA || kind != expected_kind || run_id != expected_run_id || sequence != expected_sequence {
        return Err(denied());
    }
    decode_hex::<16>(run_id)?;
    decode_hex::<16>(exchange_id)?;
    Ok(())
}

/// ⏱️ The exchange's own validity window holds `now` and is no longer than one exchange deadline.
pub(super) fn validate_window(issued_at: i64, expires_at: i64, now: i64) -> DirectoryResult<()> {
    if issued_at < 0 || expires_at <= issued_at || expires_at.saturating_sub(issued_at) > LOCAL_BOOTSTRAP_EXCHANGE_DEADLINE_MS || now < issued_at.saturating_sub(1_000) || now > expires_at {
        return Err(DirectoryError::Unauthorized);
    }
    Ok(())
}

pub(super) fn validate_identifier(value: &str) -> DirectoryResult<()> {
    if value.is_empty() || value.len() > 64 || !value.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'-')) {
        return Err(denied());
    }
    Ok(())
}

pub(super) fn denied() -> DirectoryError {
    DirectoryError::Unauthorized
}

pub(super) fn resource_limit() -> DirectoryError {
    DirectoryError::Conflict("local bootstrap resource limit exceeded".into())
}

pub(super) fn unavailable() -> DirectoryError {
    DirectoryError::Backend("local bootstrap transport unavailable".into())
}
