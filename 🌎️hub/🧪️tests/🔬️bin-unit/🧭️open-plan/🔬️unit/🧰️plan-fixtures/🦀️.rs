/// 🧰️ Plan fixtures: openable document publication, the plan secret and the plan authorities for a scope and a session.

async fn publish_openable_document_for_test(state: &HubState, token: &str, space_id: &str, document_id: &str) -> (DocumentDescriptor, os_directory::ArtifactCheckpoint) {
    let pack = b"document-open-genesis-pack";
    let spr = b"document-open-genesis-spr";
    let mut descriptor = document_descriptor_for_test(space_id, document_id);
    descriptor.bootstrap_snapshot_hash = os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(pack));
    let session = state.directory.authenticate_session(&SessionCapability::parse(token).expect("document-open author capability")).await.expect("document-open author session read").expect("document-open author session");
    let actor = ArtifactCreationActorV1 { user_id: session.user_id, session_id: session.id, authorization_generation: session.authorization_generation };
    let parent_dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: TEST_ARTIFACT_PARENT_DIALECT_KIND.into(), standard: "1".into(), subset: "*".into() };
    let checkpoint = publish_genesis_checkpoint_for_test(state, actor, "66".repeat(32), parent_dialect, descriptor.clone(), pack, spr).await;
    (descriptor, checkpoint)
}

fn document_open_plan_secret(index: u32) -> [u8; 32] {
    let mut secret = [0u8; 32];
    secret[28..].copy_from_slice(&index.to_be_bytes());
    secret
}

fn document_open_plan_authority_for_scope(base: &DocumentOpenPlanAuthorityV1, binding: u32, document: u32) -> DocumentOpenPlanAuthorityV1 {
    let mut authority = base.clone();
    authority.scope.document_id = format!("document-{document}");
    authority.descriptor.document_id = authority.scope.document_id.clone();
    authority.descriptor_digest_v1 = os_directory::io::binary::artifact_hash::hex_lower(&os_directory::io::binary::descriptor_digest::descriptor_digest_v1(&authority.descriptor).expect("scoped descriptor digest").0);
    authority.checkpoint.descriptor_digest_v1 = authority.descriptor_digest_v1.clone();
    authority.checkpoint.baseline_frontier.document_id = authority.scope.document_id.clone();
    authority.subject = SocketSubjectV1::Session {
        session_id: format!("open-plan-session-{binding}"),
        user_id: format!("open-plan-user-{binding}"),
        authorization_generation: authority.revalidation.session_generation.expect("session generation"),
        role: Some(SpaceRole::Author),
        expires_at_ms: i64::MAX,
        session_kind: AuthSessionKind::External,
        device_instance_id: format!("open-plan-device-{binding}"),
    };
    authority
}

async fn document_open_plan_authority_for_session(state: &HubState, fixture: &DocumentOpenPlanLedgerFixture, token: &str, scope: DocumentScope) -> DocumentOpenPlanAuthorityV1 {
    let capability = SessionCapability::parse(token).expect("session capability");
    let session = state.directory.authenticate_session(&capability).await.expect("session lookup").expect("active session");
    let role = state.directory.get_role(&scope.space_id, &session.user_id).await.expect("role lookup").expect("space member");
    let descriptor = state.directory.get_document_descriptor(&scope).await.expect("descriptor lookup").expect("announced descriptor");
    let mut authority = document_open_plan_test_authority(fixture);
    authority.scope = scope;
    authority.descriptor = descriptor;
    authority.descriptor_digest_v1 = os_directory::io::binary::artifact_hash::hex_lower(&os_directory::io::binary::descriptor_digest::descriptor_digest_v1(&authority.descriptor).expect("descriptor digest").0);
    authority.package.plugin_id = authority.descriptor.owner.plugin_id.clone();
    authority.package.package_id = authority.descriptor.owner.package_id.clone();
    authority.package.version = authority.descriptor.owner.version.clone();
    authority.package.component_sha256 = authority.descriptor.owner.package_hash.clone();
    authority.artifact.kind = authority.descriptor.artifact_kind.clone();
    authority.artifact.schema = authority.descriptor.artifact_schema.clone();
    authority.artifact.pack_schema_hash = authority.descriptor.pack_schema_hash.clone();
    if let Some(catalog) = &state.openable_catalog {
        let selected = catalog.resolve_document_open(&authority.descriptor, None, matches!(role, SpaceRole::Author)).expect("test catalog selection");
        authority.catalog.generation_id = catalog.generation_id().into();
        authority.package = selected.package;
        authority.artifact = selected.artifact;
        authority.parent_dialect = selected.parent_dialect;
        authority.surface = selected.surface;
        authority.browser_actor = selected.browser_actor;
        authority.grant = selected.grant;
    }
    authority.checkpoint = document_open_checkpoint(state.directory.get_active_artifact_checkpoint(&authority.scope).await.expect("checkpoint lookup").expect("committed document checkpoint"));
    authority.grant.write = matches!(role, SpaceRole::Author);
    let directory_revision = state.directory.head_seq().await.expect("directory revision");
    authority.revalidation.directory_revision = directory_revision;
    authority.revalidation.membership_generation = directory_revision;
    authority.revalidation.session_generation = Some(session.authorization_generation);
    authority.revalidation.share_generation = None;
    authority.subject = SocketSubjectV1::Session { session_id: session.id, user_id: session.user_id, authorization_generation: session.authorization_generation, role: Some(role), expires_at_ms: session.expires_at, session_kind: session.session_kind, device_instance_id: session.device_instance_id };
    authority.server_actor_id = socket_actor_id(&session.secret_digest, true);
    authority.validate().expect("authenticated route authority");
    authority
}
