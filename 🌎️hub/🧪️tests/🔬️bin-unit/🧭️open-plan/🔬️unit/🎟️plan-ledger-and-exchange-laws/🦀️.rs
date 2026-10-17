/// 🎟️ Laws of the document-open plan ledger and the receipt exchange of a bounded socket grant.

#[test]
fn document_open_plan_ledger_is_digest_only_bounded_single_use_revalidated_and_restart_scoped() {
    let fixture: DocumentOpenPlanLedgerFixture = semio_framework_pack_json::from_json_str(semio_hub_testing::hub_source("../🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("document open plan fixture");
    let authority = document_open_plan_test_authority(&fixture);
    for (field, value) in [("artifactKind", String::new()), ("artifactKind", " s.foreign.document".to_owned()), ("standard", String::new()), ("subset", String::new()), ("standard", "\u{85}".to_owned()), ("subset", " * ".to_owned()), ("standard", "🌊".repeat(65))] {
        let mut hostile = authority.clone();
        match field {
            "artifactKind" => hostile.parent_dialect.artifact_kind = value,
            "standard" => hostile.parent_dialect.standard = value,
            "subset" => hostile.parent_dialect.subset = value,
            _ => unreachable!(),
        }
        assert_eq!(hostile.validate(), Err(DocumentOpenPlanErrorCodeV1::Stale), "invalid private parent {field}");
    }
    let ledger = Arc::new(DocumentOpenPlanLedgerV1::default());
    let secret = std::array::from_fn(|index| u8::try_from(index + 1).expect("fixture secret byte"));
    let public = ledger.issue_with_capability(authority.clone(), fixture.now_ms, fixture.now_ms + DOCUMENT_OPEN_PLAN_MAX_TTL_MS, DocumentOpenPlanCapabilityV1::from_secret(secret)).expect("issue fixture plan");
    assert_eq!(public.receipt, "open.v1.AQIDBAUGBwgJCgsMDQ4PEBESExQVFhcYGRobHB0eHyA");
    assert_eq!(public.parent_dialect, fixture.valid_plan.parent_dialect);
    let mut public_parent_kind = public.clone();
    public_parent_kind.parent_dialect.artifact_kind.insert(0, ' ');
    assert_eq!(public_parent_kind.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
    let mut public_parent_two_space = public.clone();
    public_parent_two_space.parent_dialect.artifact_kind = "s.note.note".into();
    public_parent_two_space.artifact.kind = "2d.note".into();
    assert_eq!(public_parent_two_space.validate(fixture.now_ms), Ok(()));
    let mut public_parent_control = public.clone();
    public_parent_control.parent_dialect.standard.push('\u{85}');
    assert_eq!(public_parent_control.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
    let expected_digest = [0x5d, 0x05, 0xc0, 0xd4, 0x09, 0x43, 0xe9, 0xab, 0xd9, 0x66, 0x33, 0xca, 0x62, 0xfe, 0x36, 0x2b, 0x58, 0xc6, 0x6e, 0xa8, 0x5a, 0xe0, 0xa0, 0xba, 0x1f, 0x02, 0x48, 0x8e, 0x38, 0xc0, 0xd3, 0x24];
    {
        let inner = ledger.inner.lock().expect("open plan ledger");
        let record = inner.records.get(&expected_digest).expect("digest-only record");
        assert_eq!(record.receipt_digest, expected_digest);
        assert_eq!(record.issued_at_ms, fixture.now_ms);
        assert_eq!(record.socket_grant_selector, None);
    }

    let barrier = Arc::new(std::sync::Barrier::new(9));
    let exchange_at = fixture.now_ms + 1;
    let attempts = (0..8)
        .map(|_| {
            let ledger = ledger.clone();
            let barrier = barrier.clone();
            let receipt = public.receipt.clone();
            let authority = authority.clone();
            std::thread::spawn(move || {
                barrier.wait();
                ledger.exchange(&receipt, &authority, exchange_at, "socket-selector").is_ok()
            })
        })
        .collect::<Vec<_>>();
    barrier.wait();
    assert_eq!(attempts.into_iter().map(|attempt| attempt.join().expect("exchange race")).filter(|won| *won).count(), 1);
    assert_eq!(ledger.exchange(&public.receipt, &authority, fixture.now_ms + 2, "socket-selector-2"), Err(DocumentOpenPlanErrorCodeV1::AlreadyConsumed));
    assert_eq!(DocumentOpenPlanLedgerV1::default().exchange(&public.receipt, &authority, fixture.now_ms + 2, "socket-selector"), Err(DocumentOpenPlanErrorCodeV1::Denied));

    let mismatch_ledger = DocumentOpenPlanLedgerV1::default();
    let mismatch = mismatch_ledger.issue_with_capability(authority.clone(), fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(10))).expect("issue mismatch plan");
    let mut foreign = authority.clone();
    foreign.catalog.generation_id = "77".repeat(32);
    assert_eq!(mismatch_ledger.exchange(&mismatch.receipt, &foreign, fixture.now_ms + 1, "socket-selector"), Err(DocumentOpenPlanErrorCodeV1::Stale));
    let mut foreign_parent = authority.clone();
    foreign_parent.parent_dialect.standard = "2".into();
    assert_eq!(mismatch_ledger.exchange(&mismatch.receipt, &foreign_parent, fixture.now_ms + 2, "socket-selector"), Err(DocumentOpenPlanErrorCodeV1::Stale));
    assert_eq!(mismatch_ledger.exchange(&mismatch.receipt, &authority, fixture.now_ms + 100, "socket-selector"), Err(DocumentOpenPlanErrorCodeV1::Expired));

    let replacement_ledger = DocumentOpenPlanLedgerV1::default();
    let first = replacement_ledger.issue_with_capability(authority.clone(), fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(11))).expect("first outstanding plan");
    let second = replacement_ledger.issue_with_capability(authority.clone(), fixture.now_ms + 1, fixture.now_ms + 101, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(12))).expect("replacement outstanding plan");
    assert_eq!(replacement_ledger.exchange(&first.receipt, &authority, fixture.now_ms + 2, "socket-selector"), Err(DocumentOpenPlanErrorCodeV1::Stale));
    replacement_ledger.invalidate_receipt(&second.receipt).expect("cancel after publication");
    assert_eq!(replacement_ledger.exchange(&second.receipt, &authority, fixture.now_ms + 2, "socket-selector"), Err(DocumentOpenPlanErrorCodeV1::Stale));

    let replacement_expiry_ledger = DocumentOpenPlanLedgerV1::default();
    let replaced_a = replacement_expiry_ledger.issue_with_capability(authority.clone(), fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(14))).expect("replacement A");
    let replaced_b = replacement_expiry_ledger.issue_with_capability(authority.clone(), fixture.now_ms + 1, fixture.now_ms + 200, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(15))).expect("replacement B");
    let replaced_c =
        replacement_expiry_ledger.issue_with_capability(authority.clone(), fixture.now_ms + 101, fixture.now_ms + 201, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(16))).expect("replacement C after A expiry sweep");
    assert_eq!(replacement_expiry_ledger.exchange(&replaced_a.receipt, &authority, fixture.now_ms + 102, "socket-a"), Err(DocumentOpenPlanErrorCodeV1::Denied));
    assert_eq!(replacement_expiry_ledger.exchange(&replaced_b.receipt, &authority, fixture.now_ms + 102, "socket-b"), Err(DocumentOpenPlanErrorCodeV1::Stale));
    assert!(replacement_expiry_ledger.exchange(&replaced_c.receipt, &authority, fixture.now_ms + 102, "socket-c").is_ok());

    let revoke_ledger = DocumentOpenPlanLedgerV1::default();
    let revoked = revoke_ledger.issue_with_capability(authority.clone(), fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(13))).expect("revocable plan");
    assert_eq!(revoke_ledger.invalidate_binding(&authority.subject.binding()), 1);
    assert_eq!(revoke_ledger.exchange(&revoked.receipt, &authority, fixture.now_ms + 1, "socket-selector"), Err(DocumentOpenPlanErrorCodeV1::Stale));

    let share_scope = authority.scope.clone();
    let mut share = authority.clone();
    share.subject = SocketSubjectV1::Share { share_id: "share-plan".into(), selector: "share-selector".into(), scope: share_scope, expires_at_ms: i64::MAX };
    share.revalidation.session_generation = None;
    share.revalidation.share_generation = Some(1);
    share.surface.role = directory::os_directory::DocumentOpenSurfaceRoleV1::Viewer;
    share.grant.write = false;
    assert!(DocumentOpenPlanLedgerV1::default().issue(share.clone(), fixture.now_ms, fixture.now_ms + 100).is_ok());
    share.grant.write = true;
    assert_eq!(DocumentOpenPlanLedgerV1::default().issue(share, fixture.now_ms, fixture.now_ms + 100), Err(DocumentOpenPlanErrorCodeV1::Stale));

    let mut beyond_binding = authority.clone();
    if let SocketSubjectV1::Session { expires_at_ms, .. } = &mut beyond_binding.subject {
        *expires_at_ms = i64::try_from(fixture.now_ms + 50).expect("binding expiry");
    }
    assert_eq!(DocumentOpenPlanLedgerV1::default().issue_with_capability(beyond_binding, fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(22)),), Err(DocumentOpenPlanErrorCodeV1::Denied));

    let binding_bounded = DocumentOpenPlanLedgerV1::default();
    for document in 0..DOCUMENT_OPEN_PLAN_BINDING_CAPACITY {
        let scoped = document_open_plan_authority_for_scope(&authority, 1, document as u32);
        binding_bounded.issue_with_capability(scoped, fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(100 + document as u32))).expect("within binding capacity");
    }
    let overflow_scope = document_open_plan_authority_for_scope(&authority, 1, DOCUMENT_OPEN_PLAN_BINDING_CAPACITY as u32);
    assert_eq!(binding_bounded.issue_with_capability(overflow_scope, fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(500))), Err(DocumentOpenPlanErrorCodeV1::DeadlineExceeded));

    let globally_bounded = DocumentOpenPlanLedgerV1::default();
    for index in 0..DOCUMENT_OPEN_PLAN_LEDGER_CAPACITY {
        let scoped = document_open_plan_authority_for_scope(&authority, (index / DOCUMENT_OPEN_PLAN_BINDING_CAPACITY) as u32, index as u32);
        globally_bounded.issue_with_capability(scoped, fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(1_000 + index as u32))).expect("within global capacity");
    }
    let overflow = document_open_plan_authority_for_scope(&authority, 99, DOCUMENT_OPEN_PLAN_LEDGER_CAPACITY as u32);
    assert_eq!(globally_bounded.issue_with_capability(overflow, fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(10_000))), Err(DocumentOpenPlanErrorCodeV1::DeadlineExceeded));
    assert!(matches!(DocumentOpenPlanCapabilityV1::parse("open.v1.AQIDBAUGBwgJCgsMDQ4PEBESExQVFhcYGRobHB0eHyB"), Err(DocumentOpenPlanErrorCodeV1::Denied)));
}

#[test]
fn document_open_plan_receipt_exchange_admits_one_exact_bounded_secret_free_socket_grant() {
    let fixture: DocumentOpenPlanLedgerFixture = semio_framework_pack_json::from_json_str(semio_hub_testing::hub_source("../🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("document open plan fixture");
    let authority = document_open_plan_test_authority(&fixture);
    let plans = Arc::new(DocumentOpenPlanLedgerV1::default());
    let sockets = Arc::new(SocketGrantLedgerV1::default());
    let public = plans.issue_with_capability(authority.clone(), fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(30))).expect("route-inaccessible plan fixture");

    let barrier = Arc::new(std::sync::Barrier::new(9));
    let attempts = (0..8)
        .map(|_| {
            let plans = plans.clone();
            let sockets = sockets.clone();
            let barrier = barrier.clone();
            let authority = authority.clone();
            let receipt = public.receipt.clone();
            std::thread::spawn(move || {
                barrier.wait();
                plans.exchange_to_socket_grant(&receipt, &authority, fixture.now_ms + 1, sockets.as_ref())
            })
        })
        .collect::<Vec<_>>();
    barrier.wait();
    let outcomes = attempts.into_iter().map(|attempt| attempt.join().expect("plan exchange race")).collect::<Vec<_>>();
    assert_eq!(outcomes.iter().filter(|outcome| outcome.is_ok()).count(), 1);
    assert_eq!(outcomes.iter().filter(|outcome| matches!(outcome, Err(DocumentOpenPlanErrorCodeV1::AlreadyConsumed))).count(), 7);
    let response = outcomes.into_iter().find_map(Result::ok).expect("one socket grant response");
    assert_eq!(response.schema, "semio.hub.document-socket-grant/v1");
    assert_eq!(response.protocol, SESSION_PROTOCOL_V1);
    assert_eq!(response.actor_id, authority.server_actor_id);
    assert_eq!(response.expires_at_ms, i64::try_from(fixture.now_ms + 100).expect("fixture expiry"));
    let encoded_response = serde_json::to_string(&response).expect("socket grant response encodes");
    assert_eq!(serde_json::from_str::<serde_json::Value>(&encoded_response).expect("receipt json").as_object().expect("receipt object").keys().cloned().collect::<Vec<_>>(), ["actorId", "expiresAtMs", "protocol", "schema"]);
    assert!(!encoded_response.contains(&public.receipt));
    assert!(!encoded_response.contains(&authority.descriptor_digest_v1));
    assert!(!encoded_response.contains(&authority.package.component_sha256));
    assert!(!encoded_response.contains(&authority.scope.document_id));
    assert!(!encoded_response.contains("receipt"));
    assert!(!encoded_response.contains("socket.v1."), "a document socket grant carries no capability");
    let audience = SocketAudienceV1::Document(authority.scope.clone());
    let pending = sockets.pending_document_binding(&audience, &authority.subject.binding(), i64::try_from(fixture.now_ms + 2).expect("fixture time")).expect("exact pending document grant");
    assert_eq!(pending.key, SocketGrantKeyV1::CredentialBinding);
    assert_eq!(pending.actor_id, authority.server_actor_id);
    assert_eq!(pending.subject, authority.subject);
    assert_eq!(pending.document_plan.as_deref(), Some(&authority));
    assert_eq!(pending.expires_at_ms, response.expires_at_ms);
    assert_eq!(sockets.inner.lock().expect("socket ledger").records.len(), 1);
    let plan_digest = DocumentOpenPlanCapabilityV1::parse(&public.receipt).expect("plan receipt parses").digest();
    let plan_record = plans.inner.lock().expect("plan ledger").records.get(&plan_digest).expect("plan record").clone();
    assert_eq!(plan_record.state, DocumentOpenPlanStateV1::Consumed);
    assert_eq!(plan_record.socket_grant_selector.as_deref(), Some(pending.selector.as_str()));

    let capacity_plans = DocumentOpenPlanLedgerV1::default();
    let capacity_sockets = SocketGrantLedgerV1::default();
    let capacity_plan = capacity_plans.issue_with_capability(authority.clone(), fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(31))).expect("capacity plan");
    for _ in 0..SOCKET_GRANT_BINDING_PENDING_CAPACITY {
        capacity_sockets
            .admit_document(
                SocketAudienceV1::Document(authority.scope.clone()),
                authority.server_actor_id.clone(),
                authority.subject.clone(),
                i64::try_from(fixture.now_ms).expect("fixture time"),
                i64::try_from(fixture.now_ms + 1_000).expect("fixture expiry"),
                Arc::new(authority.clone()),
            )
            .expect("fill per-binding socket grant capacity");
    }
    let refused_capability = SocketGrantCapability::mint().expect("refused document capability");
    assert_eq!(
        SocketGrantLedgerV1::default().issue(&refused_capability, SocketAudienceV1::Document(authority.scope.clone()), authority.server_actor_id.clone(), authority.subject.clone(), 1, 1_000),
        Err(SocketGrantLedgerErrorV1::Rejected),
        "a document audience never answers to a capability"
    );
    assert!(matches!(capacity_plans.exchange_to_socket_grant(&capacity_plan.receipt, &authority, fixture.now_ms + 1, &capacity_sockets), Err(DocumentOpenPlanErrorCodeV1::DeadlineExceeded)));
    let capacity_digest = DocumentOpenPlanCapabilityV1::parse(&capacity_plan.receipt).expect("capacity receipt parses").digest();
    let capacity_record = capacity_plans.inner.lock().expect("capacity plan ledger").records.get(&capacity_digest).expect("capacity plan remains").clone();
    assert_eq!(capacity_record.state, DocumentOpenPlanStateV1::Issued);
    assert_eq!(capacity_record.socket_grant_selector, None);
    capacity_sockets.invalidate_binding(authority.subject.binding());
    assert!(capacity_plans.exchange_to_socket_grant(&capacity_plan.receipt, &authority, fixture.now_ms + 2, &capacity_sockets).is_ok());
}
