/// 📣️ Replay refusals and agent offer: the shared replay-refusal vocabulary, the unserved guest refusal, the space artifact creation replay and the agent bridge offer scope.

/// 📣️ The shared replay-refusal vocabulary (`🏛️ShellHost/📣️replay-refusal/🔣️.json`) names exactly the reasons this shell can
/// raise, and its vectors — the same ones React's engine contract answers — pin each reason's text per locale and its code.
#[test]
fn the_shared_replay_refusal_vocabulary_names_every_reason_with_its_text_and_code() {
    let vocabulary = &*REPLAY_REFUSAL_V1;
    let reasons: std::collections::BTreeSet<String> = ReplayRefusalReasonV1::ALL.iter().map(|reason| reason.key()).collect();
    assert_eq!(vocabulary.reasons.keys().cloned().collect::<std::collections::BTreeSet<_>>(), reasons, "vocabulary keys == the shell's reasons");
    assert!(vocabulary.reasons.values().all(|label| !label.en.is_empty() && !label.de.is_empty() && label.en != label.de), "every reason speaks both tongues");
    assert!(!vocabulary.vectors.is_empty());
    for vector in &vocabulary.vectors {
        assert_eq!((vector.reason.notice_text(&vector.locale), vector.reason.code()), (vector.text.clone(), vector.code.clone()), "{vector:?}");
    }
}

/// 📣️ A guest replay this shell cannot serve is refused OUT LOUD — the vocabulary's warning notice in the shell's tongue with
/// its fault code — never dropped: the local studio catalog (route B has no native half yet), an unknown `os.*` command or
/// directory verb, and a space artifact creation nobody signed in for, or from no mounted Space index.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_unserved_guest_replay_is_refused_out_loud_in_the_shells_tongue() {
    use ReplayRefusalReasonV1::{InvalidRequest, LocalCatalogUnavailable, SignInRequired, SpaceIndexRequired, UnroutedCommand};
    let identity = || Identity { user_id: "user-a".into(), email: "a@example.test".into(), display_name: "Ada".into(), hub_base_url: "https://hub.example".into(), issued_at_ms: 1 };
    let cases: [(&str, &str, bool, &[(&str, &str)], ReplayRefusalReasonV1); 7] = [
        ("en", "os.local-catalog.admit", false, &[("documentId", "studio-1")], LocalCatalogUnavailable),
        ("de", "os.local-catalog.open", true, &[("documentId", "studio-1")], LocalCatalogUnavailable),
        ("de", "os.local-catalog.retire", true, &[("documentId", "studio-1")], LocalCatalogUnavailable),
        ("en", "os.unknown-verb", true, &[], UnroutedCommand),
        ("de", "os.directory.unknown-verb", true, &[], InvalidRequest),
        ("de", "os.create-space-artifact", false, &[("kindChoice", "{}"), ("name", "Plan")], SignInRequired),
        ("en", "os.create-space-artifact", true, &[("kindChoice", "{}"), ("name", "Plan")], SpaceIndexRequired),
    ];
    for (locale, action_id, signed_in, args, reason) in cases {
        let mut shell = shell();
        shell.prefs.locale_id = locale.into();
        if signed_in {
            shell.directory.identity = Some(identity());
        }
        shell_command(&mut shell, action_id, args);
        let notice = shell.chrome.transient_notice().unwrap_or_else(|| panic!("{action_id} ({locale}) is refused out loud"));
        let label = &REPLAY_REFUSAL_V1.reasons[&reason.key()];
        let spoken = if locale == "de" { label.de.as_str() } else { label.en.as_str() };
        assert_eq!((notice.message.as_str(), notice.code.as_deref()), (spoken, Some(reason.code().as_str())), "{locale} {action_id}");
        assert!(matches!(notice.severity, semio_framework::Severity::Warning), "{action_id} warns");
    }
}

/// 🌱️ Only a canonical choice of one kind the space's ready catalog offers becomes a creation — ShellHost's
/// `spaceArtifactCreationRequestFromAction` gate: exact `{kindChoice, name}`, the choice byte-equal to the catalog kind's
/// encoding, the catalog of the index's own space.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_space_artifact_creation_replay_names_exactly_one_offered_kind() {
    let catalog: SpaceArtifactCreationCatalogV1 = serde_json::from_value(serde_json::json!({
        "schema": "semio.hub.space-artifact-creation-catalog/v1",
        "spaceId": "space-a",
        "catalogGenerationId": "d1099ba98c89995f462d00c109bc8fcdb5594260e9bdb584e3da628dabbe94ad",
        "kinds": [
            { "kindId": "2d.block", "schema": "block.2d", "dialect": { "artifactKind": "s.block.block2d", "standard": "1", "subset": "*" }, "label": { "en": "Block board", "de": "Blockbrett" } },
            { "kindId": "2d.note", "schema": "note.document", "dialect": { "artifactKind": "s.note.note", "standard": "1", "subset": "*" }, "label": { "en": "Note", "de": "Notiz" } }
        ]
    }))
    .expect("catalog fixture");
    let choice = |kind: &semio_framework_os_kernel::os_directory::schema::space_artifact_creation::SpaceArtifactCreationKindV1| {
        drive(semio_framework::manifest::encode_artifact_kind_choice(&semio_framework::manifest::ArtifactKindChoice {
            kind_id: kind.kind_id.clone(),
            schema: kind.schema.clone(),
            dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: kind.dialect.artifact_kind.clone(), standard: kind.dialect.standard.clone(), subset: kind.dialect.subset.clone() },
            label: LocalizedLabel::native(&kind.label.en, &kind.label.de),
        }))
    };
    let note = choice(&catalog.kinds[1]);
    let judged = |args: Value, space: &str| drive(space_artifact_creation_replay_choice(Some(&args), Some(&catalog), space));
    assert_eq!(judged(serde_json::json!({ "kindChoice": note, "name": "Minutes" }), "space-a"), Ok(("2d.note".to_string(), "Minutes".to_string())));
    let mut forged = serde_json::from_str::<Value>(&note).expect("choice is JSON");
    forged["label"]["en"] = Value::from("Notes");
    let refused = [
        (serde_json::json!({ "kindChoice": note, "name": "Minutes", "spaceId": "space-b" }), "space-a", ReplayRefusalReasonV1::InvalidRequest),
        (serde_json::json!({ "kindChoice": note }), "space-a", ReplayRefusalReasonV1::InvalidRequest),
        (serde_json::json!({ "kindChoice": format!(" {note}"), "name": "Minutes" }), "space-a", ReplayRefusalReasonV1::InvalidRequest),
        (serde_json::json!({ "kindChoice": forged.to_string(), "name": "Minutes" }), "space-a", ReplayRefusalReasonV1::InvalidRequest),
        (serde_json::json!({ "kindChoice": note, "name": 7 }), "space-a", ReplayRefusalReasonV1::InvalidRequest),
        (serde_json::json!({ "kindChoice": note, "name": "Minutes" }), "space-b", ReplayRefusalReasonV1::RouterNotReady),
    ];
    for (args, space, reason) in refused {
        assert_eq!(judged(args.clone(), space), Err(reason), "{args} in {space}");
    }
    assert_eq!(drive(space_artifact_creation_replay_choice(Some(&serde_json::json!({ "kindChoice": note, "name": "Minutes" })), None, "space-a")), Err(ReplayRefusalReasonV1::RouterNotReady));
}

/// 🛰️ The offer scope stays true to the shell (G12 × WG11): a signed-in shell in a space names its human's live agents once the
/// listing answers; a space switch or a sign-out retires the scope AT ONCE; a listing answered for a target the shell left is
/// dropped; the listing is re-read on the discovery max interval, so a revoked delegation leaves the scope; only the three
/// values reach the page.
#[test]
fn the_agent_bridge_offer_scope_follows_sign_in_space_and_revocation() {
    let hub = "http://127.0.0.1:7800".to_string();
    let (space_a, space_b) = ((hub.clone(), "space-a".to_string()), (hub.clone(), "space-b".to_string()));
    let mut owner = AgentBridgeOfferScopeOwnerV1::default();
    assert_eq!((owner.observe(None, 0.0), owner.page_json()), (None, "null".to_string()), "no scope before sign-in");
    let due = owner.observe(Some(space_a.clone()), 10.0).expect("a new target lists at once");
    assert_eq!(owner.observe(Some(space_a.clone()), 11.0), None, "one listing in flight at a time");
    owner.settle(&due, Some(vec!["agent:d-1".into(), "agent:d-2".into()]), 20.0);
    assert_eq!(owner.page_json(), r#"{"hubOrigin":"http://127.0.0.1:7800","spaceId":"space-a","agentPrincipalIds":["agent:d-1","agent:d-2"]}"#);
    assert_eq!(owner.observe(Some(space_a.clone()), 20.0 + AGENT_BRIDGE_OFFER_SCOPE_REFRESH_MS - 1.0), None, "a standing scope is not re-listed early");
    let refresh = owner.observe(Some(space_a.clone()), 20.0 + AGENT_BRIDGE_OFFER_SCOPE_REFRESH_MS).expect("the listing is re-read on schedule");
    owner.settle(&refresh, Some(vec!["agent:d-2".into()]), 30_100.0);
    assert_eq!(owner.scope().map(|scope| scope.agent_principal_ids.clone()), Some(vec!["agent:d-2".to_string()]), "a revoked delegation leaves the scope");
    let switched = owner.observe(Some(space_b.clone()), 30_200.0).expect("a space switch lists the new space at once");
    assert_eq!(owner.page_json(), "null", "a space switch retires the old space's agents at once");
    owner.settle(&space_a, Some(vec!["agent:d-9".into()]), 30_300.0);
    assert_eq!(owner.page_json(), "null", "a listing for a space the shell left is dropped");
    owner.settle(&switched, None, 30_400.0);
    assert_eq!(owner.page_json(), "null", "a failed listing leaves local offers only");
    let retry = owner.observe(Some(space_b.clone()), 30_400.0 + AGENT_BRIDGE_OFFER_SCOPE_REFRESH_MS).expect("a failed listing is retried on schedule");
    owner.settle(&retry, Some(Vec::new()), 60_500.0);
    assert_eq!(owner.page_json(), r#"{"hubOrigin":"http://127.0.0.1:7800","spaceId":"space-b","agentPrincipalIds":[]}"#);
    assert_eq!(owner.observe(None, 60_600.0), None);
    assert_eq!(owner.page_json(), "null", "a sign-out retires the scope at once");
}
