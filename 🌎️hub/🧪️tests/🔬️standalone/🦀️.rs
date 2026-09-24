use semio_hub::directory::CommandResult;

/// 🧪️ A document socket grant without the open-plan route: the credential's subject and actor, a
/// descriptor-only plan-free admission for socket laws that are not about plan revalidation.
#[cfg(test)]
async fn issue_document_socket_grant_fixture(Path((space_id, document_id)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>) -> Result<Json<DocumentSocketGrantReceiptV1>, StatusCode> {
    if !socket_text_bounded(&space_id) || !socket_text_bounded(&document_id) {
        return Err(StatusCode::BAD_REQUEST);
    }
    let scope = DocumentScope::new(space_id, document_id);
    let (subject, actor_id) = authenticate_document_socket_subject(&state, &scope, &headers).await.map_err(|error| match error {
        DocumentOpenPlanErrorCodeV1::DeadlineExceeded => StatusCode::SERVICE_UNAVAILABLE,
        _ => StatusCode::UNAUTHORIZED,
    })?;
    let descriptor = state.directory.get_document_descriptor(&scope).await.map_err(directory_error_status)?;
    if descriptor.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }
    let audience = SocketAudienceV1::Document(scope);
    let _admission = state.socket_binding_gates.acquire_record(&subject, &audience).await;
    if socket_binding_validity(&state, &subject, &audience).await != SocketBindingValidityV1::Active {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let now = now_ms();
    let binding_expiry = match &subject {
        SocketSubjectV1::Session { expires_at_ms, .. } | SocketSubjectV1::Share { expires_at_ms, .. } => *expires_at_ms,
    };
    let expires_at_ms = now.checked_add(SOCKET_GRANT_TTL_MS).ok_or(StatusCode::SERVICE_UNAVAILABLE)?.min(binding_expiry);
    state.socket_grants.admit_document_without_plan_for_test(audience, actor_id.clone(), subject, now, expires_at_ms).map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(Json(DocumentSocketGrantReceiptV1 { schema: DOCUMENT_SOCKET_GRANT_SCHEMA_V1, protocol: SESSION_PROTOCOL_V1, actor_id, expires_at_ms }))
}

#[cfg(test)]
async fn pause_directory_command_authority(state: &HubState, user_id: &str, fenced: bool) {
    if let Some(gate) = &state.live_gate {
        if !fenced {
            gate.directory_command_attempted.add_permits(1);
        }
        let pause = gate.directory_command_pause_user.lock().unwrap().as_ref().is_some_and(|(user, phase)| user == user_id && *phase == fenced);
        if pause {
            gate.directory_command_admitted.add_permits(1);
            gate.directory_command_release.acquire().await.expect("directory command test release").forget();
        }
    }
}

#[cfg(test)]
async fn pause_directory_command_membership_fence(state: &HubState) {
    if let Some(test_gate) = &state.live_gate {
        if test_gate.socket_membership_remove_enabled.load(std::sync::atomic::Ordering::Acquire) {
            test_gate.socket_membership_remove_admitted.add_permits(1);
            let _ = test_gate.socket_membership_remove_release.acquire().await;
        }
    }
}

#[cfg(test)]
async fn pause_admin_effect_started(state: &HubState) {
    if let Some(gate) = &state.live_gate {
        if gate.admin_effect_pause_enabled.load(std::sync::atomic::Ordering::Acquire) {
            gate.admin_effect_admitted.add_permits(1);
            gate.admin_effect_release.acquire().await.expect("administrator effect test release").forget();
        }
    }
}

#[cfg(test)]
async fn execute_directory_command_fenced(state: &HubState, actor: DirectoryActor, command: DirectoryCommand) -> Result<(Vec<DirectoryEvent>, Option<CommandResult>), FencedDirectoryCommandErrorV1> {
    let _authority = acquire_directory_command_fence(state, Vec::new(), &command).await?;
    if matches!(&command, DirectoryCommand::RemoveMember { .. }) {
        pause_directory_command_membership_fence(state).await;
    }
    let result = state.directory_service.execute(actor, command).await.map_err(FencedDirectoryCommandErrorV1::Directory)?;
    invalidate_directory_event_authority(state, &result.0);
    Ok(result)
}

#[cfg(test)]
async fn pause_global_directory_send_for_test(state: &HubState, record: &SocketGrantRecordV1, mode: u8) {
    let Some(gate) = &state.live_gate else { return };
    let SocketSubjectV1::Session { user_id, .. } = &record.subject else { return };
    let pause = gate.socket_global_send_pause.lock().expect("global send pause").clone();
    if matches!(&record.audience, SocketAudienceV1::Directory { .. }) && pause.as_ref().is_some_and(|(recipient, point)| recipient == user_id && *point == mode) {
        gate.socket_global_send_admitted.add_permits(1);
        gate.socket_global_send_release.acquire().await.expect("global send release").forget();
    }
}

#[test]
fn document_scope_key_v1_is_length_prefixed_and_never_colon_ambiguous() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚧️hub-boundaries/🔣️.json")).expect("valid hub boundary fixture");
    let vectors = fixture["documentScopeKeyV1"].as_array().expect("scope key vectors");
    let mut encoded = std::collections::HashSet::new();
    for vector in vectors {
        let scope = DocumentScope::new(vector["scope"]["spaceId"].as_str().unwrap(), vector["scope"]["documentId"].as_str().unwrap());
        let actual = document_scope_key_v1(&scope);
        assert_eq!(actual, vector["encoded"].as_str().unwrap());
        assert!(encoded.insert(actual), "scope vector aliased");
    }
    assert_ne!(db_artifact_id(&DocumentScope::new("space-a", "shared")), db_artifact_id(&DocumentScope::new("space-b", "shared")));
}
